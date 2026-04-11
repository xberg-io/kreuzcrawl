//! E2e tests for category: engine

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[test]
fn test_engine_batch_basic() {
    // CrawlEngine with defaults batch scrapes like the free function
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    // skipped: field 'batch.completed_count' not available on result type
    // skipped: field 'batch.total_count' not available on result type
}

#[test]
fn test_engine_crawl_basic() {
    // CrawlEngine with defaults crawls multiple pages like the free function
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    // skipped: field 'crawl.pages_crawled' not available on result type
    // skipped: field 'crawl.min_pages' not available on result type
}

#[test]
fn test_engine_map_basic() {
    // CrawlEngine with defaults discovers URLs like the free function
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    // skipped: field 'map.min_urls' not available on result type
}

#[test]
fn test_engine_scrape_basic() {
    // CrawlEngine with defaults scrapes a page identically to the free function
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    let metadata_title = result.metadata.title.as_deref().unwrap_or("");
    let metadata_description = result.metadata.description.as_deref().unwrap_or("");
    assert_eq!(result.status_code, "200", "equals assertion failed");
    assert_eq!(result.content_type, r#"text/html"#, "equals assertion failed");
    assert_eq!(metadata_title, r#"Engine Test"#, "equals assertion failed");
    assert!(metadata_description.contains(r#"Testing the engine"#), "expected to contain: {}", r#"Testing the engine"#);
    assert!(result.links.len() >= 1_f64, "expected >= 1");
    // skipped: field 'headings.h1_text' not available on result type
}

#[test]
fn test_engine_stream_basic() {
    // CrawlEngine with defaults streams events like the free function
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, url).expect("should succeed");
    // skipped: field 'stream.has_page_event' not available on result type
    // skipped: field 'stream.has_complete_event' not available on result type
    // skipped: field 'stream.event_count_min' not available on result type
}

