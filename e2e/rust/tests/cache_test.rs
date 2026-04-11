//! E2e tests for category: cache

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_cache_basic() {
    // Crawling with disk cache enabled succeeds without errors
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.status_code, 200, "equals assertion failed");
}
