//! E2e tests for category: auth

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_auth_basic_http() {
    // Sends HTTP Basic authentication header
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.auth_header_sent, true, "equals assertion failed");
    assert_eq!(result.status_code, 200, "equals assertion failed");
}

#[tokio::test]
async fn test_auth_bearer_token() {
    // Sends Bearer token in Authorization header
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.auth_header_sent, true, "equals assertion failed");
    assert_eq!(result.status_code, 200, "equals assertion failed");
}

#[tokio::test]
async fn test_auth_custom_header() {
    // Sends authentication via custom header (X-API-Key)
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.auth_header_sent, true, "equals assertion failed");
    assert_eq!(result.status_code, 200, "equals assertion failed");
}
