//! Tests that streaming APIs emit CrawlEvent::Error when a seed URL fails with a 5xx response.
//!
//! Regression coverage for: batch_crawl_stream and crawl_stream silently swallowing seed-level
//! HTTP 500 errors — they returned Ok+Complete without emitting Error because the 5xx was
//! caught in Phase 1 (resolve_initial_redirects) before reaching process_fetch_result.

use crawlberg::{CrawlConfig, CrawlEvent, batch_crawl_stream, crawl_stream, create_engine};
use futures::StreamExt;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// batch_crawl_stream must emit CrawlEvent::Error for each seed URL that returns 500.
#[tokio::test]
async fn test_batch_crawl_stream_seed_500_emits_error_event() {
    let mock = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/ok"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>OK</body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    Mock::given(method("GET"))
        .and(path("/fail"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&mock)
        .await;

    let config = CrawlConfig {
        max_depth: Some(0),
        respect_robots_txt: false,
        ..Default::default()
    };
    let engine = create_engine(Some(config)).unwrap();

    let urls = vec![format!("{}/ok", mock.uri()), format!("{}/fail", mock.uri())];

    let stream = batch_crawl_stream(&engine, urls)
        .await
        .expect("batch_crawl_stream should not fail to initialise");
    let events: Vec<CrawlEvent> = stream
        .map(|r| r.expect("stream item should not be Err"))
        .collect()
        .await;

    assert!(
        events.iter().any(|e| matches!(e, CrawlEvent::Page { .. })),
        "stream should contain at least one Page event for the 200 seed"
    );
    assert!(
        events.iter().any(|e| matches!(e, CrawlEvent::Error { .. })),
        "stream must emit Error event for the 500 seed; got events: {events:?}"
    );
    assert!(
        events.iter().any(|e| matches!(e, CrawlEvent::Complete { .. })),
        "stream must emit Complete event"
    );
}

/// crawl_stream must emit CrawlEvent::Error when the single seed URL returns 500.
#[tokio::test]
async fn test_crawl_stream_seed_500_emits_error_event() {
    let mock = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fail"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&mock)
        .await;

    let config = CrawlConfig {
        max_depth: Some(0),
        respect_robots_txt: false,
        ..Default::default()
    };
    let engine = create_engine(Some(config)).unwrap();
    let url = format!("{}/fail", mock.uri());

    let stream = crawl_stream(&engine, &url)
        .await
        .expect("crawl_stream should not fail to initialise");
    let events: Vec<CrawlEvent> = stream
        .map(|r| r.expect("stream item should not be Err"))
        .collect()
        .await;

    assert!(
        events.iter().any(|e| matches!(e, CrawlEvent::Error { .. })),
        "crawl_stream must emit Error event for 500 seed; got events: {events:?}"
    );
}
