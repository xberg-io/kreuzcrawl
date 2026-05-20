//! Integration tests for BrowserBackend::Native via wiremock.
//!
//! The native HTTP client rejects RFC1918/loopback unless KREUZCRAWL_ALLOW_PRIVATE_NETWORK
//! is set. We set it once via std::sync::OnceLock before any test runs.

#![cfg(feature = "browser-native")]
#![allow(clippy::unwrap_used, clippy::panic)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use kreuzcrawl::{BrowserBackend, BrowserConfig, BrowserWait, CrawlConfig, batch_scrape, create_engine, scrape};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

static ALLOW_PRIVATE: OnceLock<()> = OnceLock::new();

fn allow_private_network() {
    ALLOW_PRIVATE.get_or_init(|| {
        // SAFETY: tests run in a single process; the env var is written once
        // from `OnceLock::get_or_init` before any network call is made.
        #[allow(unsafe_code)]
        unsafe {
            std::env::set_var("KREUZCRAWL_ALLOW_PRIVATE_NETWORK", "1");
        }
    });
}

fn native_config(extra: impl FnOnce(BrowserConfig) -> BrowserConfig) -> CrawlConfig {
    allow_private_network();
    let browser = extra(BrowserConfig {
        backend: BrowserBackend::Native,
        mode: kreuzcrawl::BrowserMode::Always,
        timeout: Duration::from_secs(15),
        ..BrowserConfig::default()
    });
    CrawlConfig {
        browser,
        ..CrawlConfig::default()
    }
}

fn engine_with(config: CrawlConfig) -> kreuzcrawl::CrawlEngineHandle {
    create_engine(Some(config)).expect("engine must build")
}

// ---------------------------------------------------------------------------
// 1. simple HTML render
// ---------------------------------------------------------------------------

#[tokio::test]
async fn native_renders_simple_html() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body><h1>Hello</h1></body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let url = mock.uri();
    let result = scrape(&engine_with(native_config(|c| c)), &url).await;
    assert!(result.is_ok(), "should succeed: {:?}", result.err());
    assert!(result.unwrap().html.contains("Hello"));
}

// ---------------------------------------------------------------------------
// 2. follows HTTP 302 redirect
// ---------------------------------------------------------------------------

#[tokio::test]
async fn native_follows_redirect() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/start"))
        .respond_with(
            ResponseTemplate::new(302)
                .append_header("location", "/final")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/final"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>Redirected</body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let url = format!("{}/start", mock.uri());
    let result = scrape(&engine_with(native_config(|c| c)), &url).await;
    assert!(result.is_ok(), "should succeed after redirect: {:?}", result.err());
    let page = result.unwrap();
    assert!(page.html.contains("Redirected"), "final body expected");
}

// ---------------------------------------------------------------------------
// 3. browser timeout
// ---------------------------------------------------------------------------

#[tokio::test]
async fn native_respects_timeout() {
    // Point at a port that refuses connections — guarantees an error without
    // relying on mock server delays (which don't always interact well with the
    // inner tokio runtime inside spawn_blocking).
    allow_private_network();
    // Use a non-routable IP so the connect times out rather than refusing.
    // RFC 5737 TEST-NET-1: 192.0.2.0/24 — not routable on LAN.
    let url = "http://192.0.2.1:80/timeout-target";
    let config = native_config(|mut c| {
        c.timeout = Duration::from_millis(500);
        c
    });
    let start = std::time::Instant::now();
    let result = scrape(&engine_with(config), url).await;
    let elapsed = start.elapsed();
    // Must finish in under 5s (the timeout is 500ms; give generous headroom).
    assert!(
        elapsed < Duration::from_secs(5),
        "should have timed out well before 5s, took {:?}",
        elapsed
    );
    assert!(result.is_err(), "should return an error on timeout/connection failure");
}

// ---------------------------------------------------------------------------
// 4. forwards extra headers
// ---------------------------------------------------------------------------

#[tokio::test]
async fn native_forwards_extra_headers() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .and(header("x-custom", "value"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>OK</body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let url = mock.uri();
    let config = {
        allow_private_network();
        let mut headers = std::collections::HashMap::new();
        headers.insert("x-custom".to_string(), "value".to_string());
        CrawlConfig {
            browser: BrowserConfig {
                backend: BrowserBackend::Native,
                mode: kreuzcrawl::BrowserMode::Always,
                timeout: Duration::from_secs(15),
                ..BrowserConfig::default()
            },
            custom_headers: headers,
            ..CrawlConfig::default()
        }
    };
    let result = scrape(&engine_with(config), &url).await;
    assert!(result.is_ok(), "should succeed with custom header: {:?}", result.err());
}

// ---------------------------------------------------------------------------
// 5. connection refused
// ---------------------------------------------------------------------------

#[tokio::test]
async fn native_errors_on_connection_refused() {
    allow_private_network();
    let url = "http://127.0.0.1:1/unreachable";
    let result = scrape(&engine_with(native_config(|c| c)), url).await;
    assert!(result.is_err(), "should return error, not panic");
}

// ---------------------------------------------------------------------------
// 6. block URL patterns
// ---------------------------------------------------------------------------

#[tokio::test]
async fn native_block_url_patterns_blocks_match() {
    let mock = MockServer::start().await;

    // Main page that references /track.js
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><head><script src="/track.js"></script></head><body>Page</body></html>"#)
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    // track.js — should be blocked, but register so we can verify
    Mock::given(method("GET"))
        .and(path("/track.js"))
        .respond_with(ResponseTemplate::new(200).set_body_string("// tracker"))
        .expect(0) // assert it is NEVER requested
        .mount(&mock)
        .await;

    let url = mock.uri();
    let config = native_config(|mut c| {
        c.block_url_patterns = vec!["*track*".to_string()];
        c
    });
    let result = scrape(&engine_with(config), &url).await;
    assert!(result.is_ok(), "page should still render: {:?}", result.err());
    // wiremock verifies the 0-times expectation on drop
}

// ---------------------------------------------------------------------------
// 7. eval_script returns value
// ---------------------------------------------------------------------------

#[tokio::test]
async fn native_eval_script_returns_value() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><head><title>Example</title></head><body></body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let url = mock.uri();
    let config = native_config(|mut c| {
        c.eval_script = Some("document.title".to_string());
        c
    });
    let result = scrape(&engine_with(config), &url).await;
    assert!(result.is_ok(), "should succeed: {:?}", result.err());
    let page = result.unwrap();
    let browser = page.browser.expect("browser extras must be present");
    let eval = browser.eval_result.expect("eval_result must be set");
    assert_eq!(eval.as_str(), Some("Example"), "eval result should be page title");
}

// ---------------------------------------------------------------------------
// 8. capture network events includes document
// ---------------------------------------------------------------------------

#[tokio::test]
async fn native_capture_network_events_includes_document() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>Events</body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let url = mock.uri();
    let config = native_config(|mut c| {
        c.capture_network_events = true;
        c
    });
    let result = scrape(&engine_with(config), &url).await;
    assert!(result.is_ok(), "should succeed: {:?}", result.err());
    let page = result.unwrap();
    let browser = page.browser.expect("browser extras must be present");
    assert!(
        !browser.network_events.is_empty(),
        "at least the Document event should be captured"
    );
}

// ---------------------------------------------------------------------------
// 9. prior cookies sent on request
// ---------------------------------------------------------------------------

#[tokio::test]
async fn native_prior_cookies_sent_on_request() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .and(header("cookie", "session=abc"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>Authenticated</body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let url = mock.uri();
    // Pass prior_cookies via custom_headers for this integration test.
    // The engine's public `scrape()` API doesn't expose prior_cookies yet;
    // using custom_headers is the simplest way to validate cookie forwarding.
    let config = {
        allow_private_network();
        let mut headers = std::collections::HashMap::new();
        headers.insert("cookie".to_string(), "session=abc".to_string());
        CrawlConfig {
            browser: BrowserConfig {
                backend: BrowserBackend::Native,
                mode: kreuzcrawl::BrowserMode::Always,
                timeout: Duration::from_secs(15),
                ..BrowserConfig::default()
            },
            custom_headers: headers,
            ..CrawlConfig::default()
        }
    };
    let result = scrape(&engine_with(config), &url).await;
    assert!(result.is_ok(), "should succeed with cookie: {:?}", result.err());
}

// ---------------------------------------------------------------------------
// 10. post-render cookies captured from Set-Cookie
// ---------------------------------------------------------------------------

#[tokio::test]
async fn native_post_render_cookies_capture_set_cookie() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>Cookie test</body></html>")
                .append_header("content-type", "text/html")
                .append_header("set-cookie", "tracker=xyz; Path=/"),
        )
        .mount(&mock)
        .await;

    let url = mock.uri();
    let config = native_config(|mut c| {
        c.capture_network_events = true;
        c
    });
    let result = scrape(&engine_with(config), &url).await;
    assert!(result.is_ok(), "should succeed: {:?}", result.err());
    let page = result.unwrap();
    let browser = page.browser.expect("browser extras must be present");
    assert!(
        browser.cookies.iter().any(|c| c.name == "tracker" && c.value == "xyz"),
        "tracker=xyz cookie should be captured; got: {:?}",
        browser.cookies
    );
}

// ---------------------------------------------------------------------------
// 11. wait_selector succeeds when element is present
// ---------------------------------------------------------------------------

#[tokio::test]
async fn native_wait_selector_succeeds() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><body><div id="ready">Ready</div></body></html>"#)
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let url = mock.uri();
    let config = native_config(|mut c| {
        c.wait = BrowserWait::Selector;
        c.wait_selector = Some("#ready".to_string());
        c
    });
    let result = scrape(&engine_with(config), &url).await;
    assert!(result.is_ok(), "wait_selector should succeed: {:?}", result.err());
}

#[tokio::test]
async fn native_batch_scrape_uses_shared_executor_concurrently() {
    let server = TestServer::start().await;
    let config = native_config(|mut browser| {
        browser.timeout = Duration::from_secs(15);
        browser
    });
    let config = CrawlConfig {
        max_concurrent: Some(4),
        ..config
    };
    let engine = engine_with(config);

    let urls = (0..12)
        .map(|index| format!("{}/page-{index}", server.base_url))
        .collect::<Vec<_>>();
    let results = batch_scrape(&engine, urls).await.expect("batch scrape should run");

    assert_eq!(results.total_count, 12);
    for result in results.results {
        let page = result.result.expect("native scrape should succeed");
        assert!(page.html.contains("Native executor"));
        assert!(page.html.contains("data-rendered=\"true\""));
    }
    assert!(
        server.max_in_flight.load(Ordering::SeqCst) >= 2,
        "server should observe parallel native requests"
    );
}

struct TestServer {
    base_url: String,
    max_in_flight: Arc<AtomicUsize>,
}

impl TestServer {
    async fn start() -> Self {
        allow_private_network();
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("test server should bind");
        let addr = listener.local_addr().expect("test server should have local addr");
        let current = Arc::new(AtomicUsize::new(0));
        let max_in_flight = Arc::new(AtomicUsize::new(0));
        let current_for_task = current.clone();
        let max_for_task = max_in_flight.clone();

        tokio::spawn(async move {
            loop {
                let Ok((mut stream, _)) = listener.accept().await else {
                    return;
                };
                let current = current_for_task.clone();
                let max_in_flight = max_for_task.clone();
                tokio::spawn(async move {
                    let active = current.fetch_add(1, Ordering::SeqCst) + 1;
                    max_in_flight.fetch_max(active, Ordering::SeqCst);

                    let mut buffer = [0_u8; 1024];
                    let _ = stream.read(&mut buffer).await;
                    tokio::time::sleep(Duration::from_millis(150)).await;
                    let body = r#"
                        <html>
                          <body>
                            <div id="status">Native executor</div>
                            <script>
                              document.body.setAttribute('data-rendered', 'true');
                            </script>
                          </body>
                        </html>
                    "#;
                    let response = format!(
                        "HTTP/1.1 200 OK\r\ncontent-type: text/html\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                        body.len(),
                        body
                    );
                    let _ = stream.write_all(response.as_bytes()).await;
                    let _ = stream.shutdown().await;
                    current.fetch_sub(1, Ordering::SeqCst);
                });
            }
        });

        Self {
            base_url: format!("http://{addr}"),
            max_in_flight,
        }
    }
}
