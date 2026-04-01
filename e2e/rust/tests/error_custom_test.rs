// Hand-written tests for error scenarios that cannot be generated from fixtures.
// These tests require special setup (non-mock URLs, raw TCP servers, wiremock delays)
// that the E2E generator cannot express.
//
// Corresponding fixtures have `skip` directives to prevent generated duplicates.

use e2e_rust::helpers;

#[tokio::test]
async fn test_error_connection_refused() {
    // Handles connection refused error gracefully
    // Uses port 1 which has no listener, triggering a real connection refused error.
    let config = kreuzcrawl::CrawlConfig {
        respect_robots_txt: false,
        request_timeout: std::time::Duration::from_millis(5000),
        ..Default::default()
    };

    let engine = kreuzcrawl::CrawlEngine::builder()
        .config(config.clone())
        .build();
    let result = engine.scrape("http://127.0.0.1:1/").await;
    let err = result.expect_err("request should fail");
    assert!(
        err.to_string().contains("connection"),
        "expected connection error, got: {err}"
    );
}

#[tokio::test]
async fn test_error_dns_resolution() {
    // Handles DNS resolution failure gracefully
    // Uses a guaranteed-nonexistent domain per RFC 6761.
    let config = kreuzcrawl::CrawlConfig {
        respect_robots_txt: false,
        request_timeout: std::time::Duration::from_millis(5000),
        ..Default::default()
    };

    let engine = kreuzcrawl::CrawlEngine::builder()
        .config(config.clone())
        .build();
    let result = engine
        .scrape("http://this-domain-does-not-exist.invalid/")
        .await;
    let err = result.expect_err("request should fail");
    assert!(
        err.to_string().contains("dns"),
        "expected dns error, got: {err}"
    );
}

#[tokio::test]
async fn test_error_ssl_invalid_cert() {
    // Handles SSL certificate validation error
    // Connects via HTTPS to an HTTP-only mock server, triggering an SSL/TLS error.
    let mock = helpers::setup_mock_server().await;

    let config = kreuzcrawl::CrawlConfig {
        respect_robots_txt: false,
        request_timeout: std::time::Duration::from_millis(5000),
        ..Default::default()
    };

    // Replace http:// with https:// to force TLS handshake against plain HTTP
    let https_url = mock.uri().replace("http://", "https://");
    let engine = kreuzcrawl::CrawlEngine::builder()
        .config(config.clone())
        .build();
    let result = engine.scrape(&https_url).await;
    let err = result.expect_err("request should fail");
    assert!(
        err.to_string().contains("ssl")
            || err.to_string().contains("tls")
            || err.to_string().contains("connection"),
        "expected ssl/tls/connection error, got: {err}"
    );
}

#[tokio::test]
async fn test_error_timeout() {
    // Handles request timeout
    // Uses wiremock delay to ensure the response takes longer than the configured timeout.
    use std::time::Duration;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    let mock = MockServer::start().await;
    let response = ResponseTemplate::new(200)
        .set_body_string("<html><body>slow</body></html>")
        .append_header("content-type", "text/html")
        .set_delay(Duration::from_secs(5));

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(response)
        .mount(&mock)
        .await;

    let config = kreuzcrawl::CrawlConfig {
        request_timeout: Duration::from_millis(100),
        ..Default::default()
    };

    let engine = kreuzcrawl::CrawlEngine::builder()
        .config(config.clone())
        .build();
    let result = engine.scrape(&mock.uri()).await;
    let err = result.expect_err("request should fail");
    assert!(
        err.to_string().contains("timeout"),
        "expected timeout error, got: {err}"
    );
}

#[tokio::test]
async fn test_error_partial_response() {
    // Handles incomplete or truncated HTTP response
    // Wiremock overrides content-length, so we use register_mock which appends headers.
    // The appended content-length (99999) differs from actual body size, triggering data_loss.
    let mock = helpers::setup_mock_server().await;
    let body = "<html><body><h1>Truncated".to_owned();
    helpers::register_mock(
        &mock,
        "GET",
        "/",
        200,
        &[
            ("content-type", "text/html; charset=utf-8"),
            ("content-length", "99999"),
        ],
        &body,
    )
    .await;

    let config = kreuzcrawl::CrawlConfig {
        respect_robots_txt: false,
        ..Default::default()
    };

    let engine = kreuzcrawl::CrawlEngine::builder()
        .config(config.clone())
        .build();
    let result = engine.scrape(&mock.uri()).await;
    let err = result.expect_err("request should fail");
    let msg = err.to_string();
    // Wiremock/hyper may reject the mismatched content-length at the server side,
    // causing a connection error instead of data_loss. Both are valid detections.
    assert!(
        msg.contains("data_loss") || msg.contains("connection"),
        "expected data_loss or connection error, got: {err}"
    );
}
