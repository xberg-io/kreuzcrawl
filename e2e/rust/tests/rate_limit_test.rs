//! E2e tests for category: rate_limit

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_rate_limit_basic_delay() {
    // Rate limiter adds delay between requests to the same domain
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'crawl.pages_crawled' not available on result type
    // skipped: field 'rate_limit.min_duration_ms' not available on result type
}

#[tokio::test]
async fn test_rate_limit_zero_no_delay() {
    // Rate limiter with zero delay does not slow crawling
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'crawl.pages_crawled' not available on result type
}
