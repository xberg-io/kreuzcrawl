//! E2e tests for category: stream

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_crawl_stream_events() {
    // Crawl stream produces page and complete events
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'stream.event_count_min' not available on result type
    // skipped: field 'stream.has_page_event' not available on result type
    // skipped: field 'stream.has_complete_event' not available on result type
}

#[tokio::test]
async fn test_stream_depth_crawl() {
    // Stream produces events for multi-depth crawl with link following
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'stream.event_count_min' not available on result type
    // skipped: field 'stream.has_page_event' not available on result type
    // skipped: field 'stream.has_complete_event' not available on result type
}

#[tokio::test]
async fn test_stream_with_error_event() {
    // Stream emits page and complete events even when some pages fail
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'stream.has_page_event' not available on result type
    // skipped: field 'stream.has_complete_event' not available on result type
    // skipped: field 'stream.event_count_min' not available on result type
}
