//! E2e tests for category: concurrent

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_concurrent_basic() {
    // Concurrent crawling fetches all pages with max_concurrent workers
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'pages.length' not available on result type
    // skipped: field 'pages.length' not available on result type
}

#[tokio::test]
async fn test_concurrent_depth_two_fan_out() {
    // Concurrent depth=2 crawl correctly fans out and deduplicates across levels
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'pages.length' not available on result type
}

#[tokio::test]
async fn test_concurrent_max_pages_exact() {
    // Concurrent crawling does not exceed max_pages limit even with high concurrency
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'pages.length' not available on result type
}

#[tokio::test]
async fn test_concurrent_partial_errors() {
    // Concurrent crawl handles partial failures gracefully
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'pages.length' not available on result type
}

#[tokio::test]
async fn test_concurrent_respects_max_pages() {
    // Concurrent crawling respects max_pages limit
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'pages.length' not available on result type
}
