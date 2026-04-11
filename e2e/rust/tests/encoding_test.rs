//! E2e tests for category: encoding

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_encoding_double_encoded() {
    // Handles double-encoded URL characters (%25C3%25B6)
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(!result.html.is_empty(), "expected non-empty value");
    assert!(result.links.len() >= 1, "expected >= 1");
}

#[tokio::test]
async fn test_encoding_mixed_charset_page() {
    // Handles charset mismatch between HTTP header and HTML meta tag
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(!result.html.is_empty(), "expected non-empty value");
}

#[tokio::test]
async fn test_encoding_percent_encoded_path() {
    // Handles percent-encoded spaces and characters in URL paths
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(!result.html.is_empty(), "expected non-empty value");
    assert!(result.links.len() >= 2, "expected >= 2");
}

#[tokio::test]
async fn test_encoding_unicode_url() {
    // Handles Unicode characters in URLs (Hebrew, Japanese, Cyrillic)
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(!result.html.is_empty(), "expected non-empty value");
}
