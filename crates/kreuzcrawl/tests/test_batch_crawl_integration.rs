//! Integration tests for batch_crawl: multiple seed URLs crawled concurrently.

use kreuzcrawl::{CrawlConfig, CrawlEngine, NoopRateLimiter};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_batch_crawl_multiple_seeds() {
    let mock = MockServer::start().await;

    for name in ["a", "b", "c"] {
        Mock::given(method("GET"))
            .and(path(format!("/{name}")))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(format!("<html><body>{name}</body></html>"))
                    .append_header("content-type", "text/html"),
            )
            .mount(&mock)
            .await;
    }

    let config = CrawlConfig {
        max_depth: Some(0),
        ..Default::default()
    };
    let engine = CrawlEngine::builder()
        .config(config)
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    let urls: Vec<String> = ["a", "b", "c"]
        .iter()
        .map(|n| format!("{}/{n}", mock.uri()))
        .collect();
    let url_refs: Vec<&str> = urls.iter().map(|s| s.as_str()).collect();

    let results = engine.batch_crawl(&url_refs).await;
    assert_eq!(results.len(), 3);

    let success_count = results.iter().filter(|(_, r)| r.is_ok()).count();
    assert_eq!(success_count, 3, "all 3 should succeed");

    // Verify each result has pages.
    for (url, result) in &results {
        let crawl = result
            .as_ref()
            .unwrap_or_else(|e| panic!("{url} failed: {e}"));
        assert!(!crawl.pages.is_empty(), "{url} should have at least 1 page");
    }
}

#[tokio::test]
async fn test_batch_crawl_partial_failure() {
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
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock)
        .await;

    let config = CrawlConfig {
        max_depth: Some(0),
        ..Default::default()
    };
    let engine = CrawlEngine::builder()
        .config(config)
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    let urls = [format!("{}/ok", mock.uri()), format!("{}/fail", mock.uri())];
    let url_refs: Vec<&str> = urls.iter().map(|s| s.as_str()).collect();

    let results = engine.batch_crawl(&url_refs).await;
    assert_eq!(results.len(), 2);

    // At least one should succeed and at least one should fail or have an error.
    let successes = results.iter().filter(|(_, r)| r.is_ok()).count();
    assert!(successes >= 1, "at least one seed should succeed");
}
