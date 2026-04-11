//! E2e tests for category: middleware

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_middleware_engine_crawl_with_defaults() {
    // Engine crawl with default middleware chain produces correct multi-page results
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'crawl.pages_crawled' not available on result type
    // skipped: field 'crawl.min_pages' not available on result type
}

#[tokio::test]
async fn test_middleware_noop_no_effect() {
    // Default middleware chain does not affect normal scraping
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    let metadata_title = result.metadata.title.as_deref().unwrap_or("");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert_eq!(metadata_title.trim(), r#"Middleware Test"#, "equals assertion failed");
}
