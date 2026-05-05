//! E2E mock HTTP server for kreuzcrawl fixture-driven tests.
//!
//! Reads fixture JSON files, extracts `mock_responses`, and serves them
//! namespaced by fixture ID: `GET /fixtures/{fixture_id}{path}`.
//!
//! Usage:
//!   mock-server --fixtures ./fixtures [--port 0]
//!
//! Prints `MOCK_SERVER_URL=http://127.0.0.1:{port}` to stdout on startup.

use axum::Router;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{HeaderName, HeaderValue, Response, StatusCode};
use axum::routing::any;
use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Parser)]
#[command(name = "mock-server", about = "E2E mock HTTP server for fixture-driven tests")]
struct Cli {
    /// Path to the fixtures directory.
    #[arg(long, default_value = "./fixtures")]
    fixtures: PathBuf,

    /// Port to listen on (0 = random available port).
    #[arg(long, default_value_t = 0)]
    port: u16,
}

/// A single mock route extracted from a fixture.
#[derive(Clone)]
struct MockRoute {
    status_code: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
    /// Optional delay before sending the response (milliseconds).
    delay_ms: Option<u64>,
}

/// All routes keyed by `"{fixture_id}{path}"`.
type RouteMap = HashMap<String, MockRoute>;

/// Minimal fixture structure — only fields we need.
#[derive(Deserialize)]
struct FixtureFile {
    id: String,
    #[serde(default)]
    input: serde_json::Value,
    /// New single-response format: `mock_response: { status, body, headers }`.
    #[serde(default)]
    mock_response: Option<serde_json::Value>,
}

/// A mock response spec from the fixture.
#[derive(Deserialize)]
struct MockResponseSpec {
    #[serde(default = "default_path")]
    path: String,
    #[serde(default = "default_status")]
    status_code: u16,
    #[serde(default)]
    headers: HashMap<String, String>,
    #[serde(default)]
    body_file: Option<String>,
    #[serde(default)]
    body_inline: Option<String>,
    /// Delay before sending the response (milliseconds).
    #[serde(default)]
    delay_ms: Option<u64>,
}

fn default_path() -> String {
    "/".to_string()
}

fn default_status() -> u16 {
    200
}

fn load_fixtures(fixtures_dir: &std::path::Path) -> RouteMap {
    let mut routes = RouteMap::new();
    let responses_dir = fixtures_dir.join("responses");

    // Recursively find all .json files (skip schema.json and _prefixed files).
    let json_files = walk_json_files(fixtures_dir);

    for path in json_files {
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("warn: failed to read {}: {e}", path.display());
                continue;
            }
        };

        let fixture: FixtureFile = match serde_json::from_str(&content) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("warn: failed to parse {}: {e}", path.display());
                continue;
            }
        };

        // Support two fixture formats:
        // 1. New: top-level `mock_response: { status, body, headers }` → single route at `/`
        // 2. Old: `input.mock_responses: [{ path, status_code, headers, body_inline, ... }]`
        let mock_responses: Vec<MockResponseSpec> = if let Some(ref mr) = fixture.mock_response {
            // New format: convert to a single MockResponseSpec at default path "/"
            let status_code = mr.get("status").and_then(|v| v.as_u64()).unwrap_or(200) as u16;
            let body_inline = mr.get("body").and_then(|v| v.as_str()).map(String::from);
            let headers: HashMap<String, String> = mr
                .get("headers")
                .and_then(|v| v.as_object())
                .map(|obj| {
                    obj.iter()
                        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                        .collect()
                })
                .unwrap_or_default();
            vec![MockResponseSpec {
                path: "/".to_string(),
                status_code,
                headers,
                body_file: None,
                body_inline,
                delay_ms: None,
            }]
        } else {
            match fixture
                .input
                .get("mock_responses")
                .and_then(|v| serde_json::from_value::<Vec<MockResponseSpec>>(v.clone()).ok())
            {
                Some(v) => v,
                None => continue,
            }
        };

        for spec in &mock_responses {
            let route_path = normalize_path(&spec.path);
            let key = format!("/fixtures/{}{}", fixture.id, route_path);

            let body = if let Some(ref body_file) = spec.body_file {
                let file_path = responses_dir.join(body_file);
                match std::fs::read(&file_path) {
                    Ok(b) => b,
                    Err(e) => {
                        eprintln!(
                            "warn: failed to load body_file {} for fixture {}: {e}",
                            file_path.display(),
                            fixture.id
                        );
                        Vec::new()
                    }
                }
            } else if let Some(ref body_inline) = spec.body_inline {
                body_inline.as_bytes().to_vec()
            } else {
                Vec::new()
            };

            let mock_route = MockRoute {
                status_code: spec.status_code,
                headers: spec.headers.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
                body,
                delay_ms: spec.delay_ms,
            };

            routes.insert(key, mock_route);
        }
    }

    routes
}

/// Ensure path starts with `/`.
fn normalize_path(p: &str) -> String {
    if p.starts_with('/') {
        p.to_string()
    } else {
        format!("/{p}")
    }
}

/// Recursively find all `.json` files, skipping schema.json and _-prefixed files.
fn walk_json_files(dir: &std::path::Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return files;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            // Skip the `responses` subdirectory (body files, not fixtures).
            if path.file_name().is_some_and(|n| n == "responses") {
                continue;
            }
            files.extend(walk_json_files(&path));
        } else if path.extension().is_some_and(|e| e == "json") {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            if name == "schema.json" || name.starts_with('_') {
                continue;
            }
            files.push(path);
        }
    }
    files
}

async fn serve_fixture(Path(rest): Path<String>, State(routes): State<Arc<RouteMap>>) -> Response<Body> {
    let key = format!("/fixtures/{rest}");

    // Try exact match first.
    if let Some(route) = routes.get(&key) {
        return build_response(route).await;
    }

    // Try with trailing slash.
    if let Some(route) = routes.get(&format!("{key}/")) {
        return build_response(route).await;
    }

    // Try without trailing slash.
    if key.ends_with('/')
        && let Some(route) = routes.get(&key[..key.len() - 1])
    {
        return build_response(route).await;
    }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(format!("no mock route for: {key}")))
        .expect("NOT_FOUND response builder is infallible")
}

async fn build_response(route: &MockRoute) -> Response<Body> {
    // Apply optional delay before responding.
    if let Some(delay_ms) = route.delay_ms {
        tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
    }
    let mut builder = Response::builder().status(route.status_code);
    for (k, v) in &route.headers {
        if let (Ok(name), Ok(val)) = (k.parse::<HeaderName>(), v.parse::<HeaderValue>()) {
            builder = builder.header(name, val);
        }
    }
    builder
        .body(Body::from(route.body.clone()))
        .expect("response body builder is infallible")
}

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let routes = load_fixtures(&cli.fixtures);
    let route_count = routes.len();
    let state = Arc::new(routes);

    let app = Router::new()
        .route("/health", any(health))
        .route("/fixtures/{*rest}", any(serve_fixture))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], cli.port));
    let listener = TcpListener::bind(addr).await.expect("failed to bind");
    let local_addr = listener.local_addr().expect("failed to get local addr");

    // Print the URL for Taskfile to capture.
    println!("MOCK_SERVER_URL=http://{local_addr}");
    eprintln!("mock-server: listening on http://{local_addr} with {route_count} routes");

    axum::serve(listener, app).await.expect("server error");
}
