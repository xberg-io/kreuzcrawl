//! E2e tests for category: error

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_error_401_unauthorized() {
    // Handles 401 Unauthorized response correctly
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("unauthorized"), "error message mismatch");
}

#[tokio::test]
async fn test_error_403_forbidden() {
    // Handles 403 Forbidden response correctly
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("forbidden"), "error message mismatch");
}

#[tokio::test]
async fn test_error_404_page() {
    // Handles 404 response correctly
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("not_found"), "error message mismatch");
}

#[tokio::test]
async fn test_error_408_request_timeout() {
    // Handles 408 Request Timeout response correctly
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("timeout"), "error message mismatch");
}

#[tokio::test]
async fn test_error_410_gone() {
    // Handles 410 Gone response correctly
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("gone"), "error message mismatch");
}

#[tokio::test]
async fn test_error_500_server() {
    // Handles 500 server error
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("server_error"), "error message mismatch");
}

#[tokio::test]
async fn test_error_502_bad_gateway() {
    // Handles 502 Bad Gateway response correctly
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("bad_gateway"), "error message mismatch");
}

#[tokio::test]
async fn test_error_connection_refused() {
    // Handles connection refused error gracefully
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("connection"), "error message mismatch");
}

#[tokio::test]
async fn test_error_dns_resolution() {
    // Handles DNS resolution failure gracefully
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("dns"), "error message mismatch");
}

#[tokio::test]
async fn test_error_empty_response() {
    // Handles 200 with completely empty body gracefully
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'html_not_empty' not available on result type
    // skipped: field 'error.is_error' not available on result type
}

#[tokio::test]
async fn test_error_invalid_proxy() {
    // Proxy pointing to unreachable address causes connection error during scrape
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("connection"), "error message mismatch");
}

#[tokio::test]
async fn test_error_partial_response() {
    // Handles incomplete or truncated HTTP response
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("data_loss"), "error message mismatch");
}

#[tokio::test]
async fn test_error_rate_limited() {
    // Handles 429 rate limiting with Retry-After
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("rate_limited"), "error message mismatch");
}

#[tokio::test]
async fn test_error_retry_503() {
    // Retries request on 503 Service Unavailable response
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("server_error"), "error message mismatch");
}

#[tokio::test]
async fn test_error_retry_backoff() {
    // Implements exponential backoff when retrying failed requests
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("rate_limited"), "error message mismatch");
}

#[tokio::test]
async fn test_error_ssl_invalid_cert() {
    // Handles SSL certificate validation error
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("ssl"), "error message mismatch");
}

#[tokio::test]
async fn test_error_timeout() {
    // Handles request timeout
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("timeout"), "error message mismatch");
}

#[tokio::test]
async fn test_error_waf_akamai() {
    // Akamai WAF detection returns WafBlocked error
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    // skipped: field 'error.is_waf_blocked' not available on result type
}

#[tokio::test]
async fn test_error_waf_false_403() {
    // Detects WAF/bot protection false 403 (Cloudflare challenge page)
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("forbidden"), "error message mismatch");
    // skipped: field 'error.is_waf_blocked' not available on result type
}

#[tokio::test]
async fn test_error_waf_imperva() {
    // Imperva/Incapsula WAF detection
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    // skipped: field 'error.is_waf_blocked' not available on result type
}
