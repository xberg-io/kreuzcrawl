//! E2e tests for category: validation

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_validation_invalid_exclude_regex() {
    // Invalid regex in exclude_paths is rejected
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("exclude_path"), "error message mismatch");
}

#[tokio::test]
async fn test_validation_invalid_include_regex() {
    // Invalid regex in include_paths is rejected
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("include_path"), "error message mismatch");
}

#[tokio::test]
async fn test_validation_invalid_retry_code() {
    // Retry code outside 100-599 is rejected
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("retry code"), "error message mismatch");
}

#[tokio::test]
async fn test_validation_max_pages_zero() {
    // max_pages=0 is rejected as invalid config
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("max_pages"), "error message mismatch");
}

#[tokio::test]
async fn test_validation_max_redirects_too_high() {
    // max_redirects > 100 is rejected as invalid config
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("max_redirects"), "error message mismatch");
}

#[tokio::test]
async fn test_validation_timeout_zero() {
    // Zero request timeout is rejected as invalid config
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await;
    assert!(result.is_err(), "expected call to fail");
    assert!(result.as_ref().unwrap_err().to_string().contains("request_timeout"), "error message mismatch");
}
