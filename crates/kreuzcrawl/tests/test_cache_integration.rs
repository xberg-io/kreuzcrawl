//! Integration tests for DiskCache: verifies that cached responses reduce server requests.

use kreuzcrawl::{CrawlConfig, CrawlEngine, DiskCache, NoopRateLimiter};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_disk_cache_reduces_requests() {
    let mock = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>Cached Page</body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let cache = DiskCache::new(dir.path(), 3600, 1000).unwrap();

    let config = CrawlConfig {
        max_depth: Some(0),
        ..Default::default()
    };
    let engine = CrawlEngine::builder()
        .config(config)
        .cache(cache)
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    // First crawl -- hits the server (redirect-resolution phase + crawl loop).
    let r1 = engine.crawl(&mock.uri()).await.unwrap();
    assert!(!r1.pages.is_empty(), "should have crawled 1 page");
    assert!(r1.pages[0].html.contains("Cached Page"));

    let requests_after_first = mock.received_requests().await.unwrap().len();

    // Second crawl -- the crawl loop should serve from cache for the page fetch,
    // but the redirect-resolution phase still makes an HTTP request.
    let r2 = engine.crawl(&mock.uri()).await.unwrap();
    assert!(!r2.pages.is_empty(), "should still return a page");
    assert!(r2.pages[0].html.contains("Cached Page"));

    let requests_after_second = mock.received_requests().await.unwrap().len();

    // The second crawl should make fewer new requests than the first crawl
    // because the crawl-loop fetch is served from cache.
    let first_crawl_requests = requests_after_first;
    let second_crawl_requests = requests_after_second - requests_after_first;

    assert!(
        second_crawl_requests < first_crawl_requests,
        "second crawl should make fewer requests ({}) than first ({}), \
         proving the cache avoided a re-fetch",
        second_crawl_requests,
        first_crawl_requests
    );
}
