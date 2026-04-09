//! Integration tests for the Firecrawl v1-compatible REST API.

#![cfg(feature = "api")]

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use std::sync::Arc;
use tower::ServiceExt;

use kreuzcrawl::{CrawlConfig, CrawlEngine, NoopRateLimiter};

/// Build a test router backed by a default engine.
fn test_router() -> axum::Router {
    let engine = CrawlEngine::builder()
        .config(CrawlConfig::default())
        .rate_limiter(NoopRateLimiter)
        .build()
        .expect("default engine should build");
    kreuzcrawl::api::create_router(Arc::new(engine))
}

/// Convenience helper: send a JSON POST to the given path and return the status + body bytes.
async fn post_json(router: &axum::Router, path: &str, body: &str) -> (StatusCode, Vec<u8>) {
    let req = Request::builder()
        .method(Method::POST)
        .uri(path)
        .header("content-type", "application/json")
        .body(Body::from(body.to_owned()))
        .expect("build request");

    let resp = router.clone().oneshot(req).await.expect("oneshot");
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .expect("read body");
    (status, bytes.to_vec())
}

#[tokio::test]
async fn test_health_endpoint() {
    let router = test_router();
    let req = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .expect("build request");

    let resp = router.oneshot(req).await.expect("oneshot");
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .expect("read body");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("parse json");
    assert_eq!(json["status"], "ok");
}

#[tokio::test]
async fn test_version_endpoint() {
    let router = test_router();
    let req = Request::builder()
        .uri("/version")
        .body(Body::empty())
        .expect("build request");

    let resp = router.oneshot(req).await.expect("oneshot");
    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .expect("read body");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("parse json");
    assert!(json["version"].is_string());
    assert!(!json["version"].as_str().unwrap_or("").is_empty());
}

#[tokio::test]
async fn test_scrape_requires_url() {
    let router = test_router();
    let (status, body) = post_json(&router, "/v1/scrape", "{}").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let json: serde_json::Value = serde_json::from_slice(&body).expect("parse json");
    assert_eq!(json["success"], false);
}

#[tokio::test]
async fn test_crawl_returns_202_with_job_id() {
    let router = test_router();
    let (status, body) = post_json(&router, "/v1/crawl", r#"{"url":"https://example.com"}"#).await;
    assert_eq!(status, StatusCode::ACCEPTED);

    let json: serde_json::Value = serde_json::from_slice(&body).expect("parse json");
    assert_eq!(json["success"], true);
    assert!(json["id"].is_string());
    // Verify it looks like a UUID.
    let id = json["id"].as_str().unwrap();
    assert!(
        uuid::Uuid::parse_str(id).is_ok(),
        "id should be a valid UUID"
    );
}

#[tokio::test]
async fn test_crawl_status_not_found() {
    let router = test_router();
    let random_id = uuid::Uuid::new_v4().to_string();
    let req = Request::builder()
        .uri(format!("/v1/crawl/{random_id}"))
        .body(Body::empty())
        .expect("build request");

    let resp = router.oneshot(req).await.expect("oneshot");
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_map_returns_200() {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body><a href=\"/about\">About</a></body></html>")
                .insert_header("content-type", "text/html"),
        )
        .mount(&mock_server)
        .await;

    // Also mock robots.txt to avoid 404 noise.
    Mock::given(method("GET"))
        .and(path("/robots.txt"))
        .respond_with(ResponseTemplate::new(200).set_body_string("User-agent: *\nAllow: /"))
        .mount(&mock_server)
        .await;

    // Mock sitemap since map may try to fetch it.
    Mock::given(method("GET"))
        .and(path("/sitemap.xml"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let router = test_router();
    let body = serde_json::json!({ "url": mock_server.uri() }).to_string();
    let (status, _body) = post_json(&router, "/v1/map", &body).await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn test_request_body_size_limit() {
    let router = test_router();
    // 11 MB of data should exceed the 10 MB limit.
    let big_body = "x".repeat(11 * 1024 * 1024);
    let req = Request::builder()
        .method(Method::POST)
        .uri("/v1/scrape")
        .header("content-type", "application/json")
        .body(Body::from(big_body))
        .expect("build request");

    let resp = router.oneshot(req).await.expect("oneshot");
    assert_eq!(resp.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_cors_headers_present() {
    let router = test_router();
    let req = Request::builder()
        .method(Method::OPTIONS)
        .uri("/health")
        .header("origin", "https://example.com")
        .header("access-control-request-method", "GET")
        .body(Body::empty())
        .expect("build request");

    let resp = router.oneshot(req).await.expect("oneshot");
    assert!(
        resp.headers().contains_key("access-control-allow-origin"),
        "CORS header should be present"
    );
}

#[tokio::test]
async fn test_download_requires_url() {
    let router = test_router();
    let (status, body) = post_json(&router, "/v1/download", "{}").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let json: serde_json::Value = serde_json::from_slice(&body).expect("parse json");
    assert_eq!(json["success"], false);
}

#[tokio::test]
async fn test_scrape_invalid_url_scheme() {
    let router = test_router();
    let (status, body) = post_json(&router, "/v1/scrape", r#"{"url":"ftp://example.com"}"#).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let json: serde_json::Value = serde_json::from_slice(&body).expect("parse json");
    assert_eq!(json["success"], false);
    assert!(
        json["error"]["message"]
            .as_str()
            .unwrap_or("")
            .contains("http"),
        "error should mention http scheme requirement"
    );
}

#[tokio::test]
async fn test_scrape_empty_url() {
    let router = test_router();
    let (status, body) = post_json(&router, "/v1/scrape", r#"{"url":""}"#).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let json: serde_json::Value = serde_json::from_slice(&body).expect("parse json");
    assert_eq!(json["success"], false);
    assert!(
        json["error"]["message"]
            .as_str()
            .unwrap_or("")
            .contains("required"),
        "error should mention url is required"
    );
}

#[tokio::test]
async fn test_crawl_job_lifecycle() {
    let router = test_router();

    // Start a crawl job.
    let (status, body) = post_json(&router, "/v1/crawl", r#"{"url":"https://example.com"}"#).await;
    assert_eq!(status, StatusCode::ACCEPTED);

    let json: serde_json::Value = serde_json::from_slice(&body).expect("parse json");
    let id = json["id"].as_str().expect("id should be a string");

    // Poll the job status -- it should exist.
    let req = Request::builder()
        .uri(format!("/v1/crawl/{id}"))
        .body(Body::empty())
        .expect("build request");

    let resp = router.clone().oneshot(req).await.expect("oneshot");
    let poll_status = resp.status();
    assert!(
        poll_status == StatusCode::OK,
        "polling an existing job should return 200, got {poll_status}"
    );

    let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .expect("read body");
    let poll_json: serde_json::Value = serde_json::from_slice(&body).expect("parse json");
    let job_status = poll_json["status"].as_str().unwrap_or("");
    assert!(
        job_status == "pending"
            || job_status == "in_progress"
            || job_status == "completed"
            || job_status == "failed",
        "job status should be a valid state, got {job_status}"
    );
}

#[tokio::test]
async fn test_crawl_cancel_nonexistent() {
    let router = test_router();
    let random_id = uuid::Uuid::new_v4().to_string();
    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/v1/crawl/{random_id}"))
        .body(Body::empty())
        .expect("build request");

    let resp = router.oneshot(req).await.expect("oneshot");
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_request_id_header_present() {
    let router = test_router();
    let req = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .expect("build request");

    let resp = router.oneshot(req).await.expect("oneshot");
    assert_eq!(resp.status(), StatusCode::OK);
    assert!(
        resp.headers().contains_key("x-request-id"),
        "response should have x-request-id header"
    );
}

#[tokio::test]
async fn test_error_response_has_code() {
    let router = test_router();
    let (status, body) = post_json(&router, "/v1/scrape", "{}").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let json: serde_json::Value = serde_json::from_slice(&body).expect("parse json");
    assert_eq!(json["success"], false);
    assert!(
        json["error"]["code"].is_string(),
        "error response should have a code field"
    );
    assert!(
        json["error"]["message"].is_string(),
        "error response should have a message field"
    );
}

#[tokio::test]
async fn test_batch_scrape_requires_urls() {
    let router = test_router();

    // Send with an empty urls array -- should get 400 from handler validation.
    let (status, body) = post_json(&router, "/v1/batch/scrape", r#"{"urls":[]}"#).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let json: serde_json::Value = serde_json::from_slice(&body).expect("parse json");
    assert_eq!(json["success"], false);
}

#[tokio::test]
async fn test_batch_scrape_missing_urls_field() {
    let router = test_router();

    // Missing required field `urls` should fail deserialization (422).
    let (status, _body) = post_json(&router, "/v1/batch/scrape", "{}").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}
