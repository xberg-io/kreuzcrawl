//! Integration tests for rate limiting: verifies per-domain throttle enforces delays.

use std::time::{Duration, Instant};

use kreuzcrawl::{CrawlConfig, CrawlEngine, PerDomainThrottle};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_rate_limiting_enforces_delay() {
    let mock = MockServer::start().await;

    for i in 0..3 {
        Mock::given(method("GET"))
            .and(path(format!("/page{i}")))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(format!("<html><body>Page {i}</body></html>"))
                    .append_header("content-type", "text/html"),
            )
            .mount(&mock)
            .await;
    }
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(
                    "<html><body>\
                     <a href=\"/page0\">0</a>\
                     <a href=\"/page1\">1</a>\
                     <a href=\"/page2\">2</a>\
                     </body></html>",
                )
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let config = CrawlConfig {
        max_depth: Some(1),
        max_concurrent: Some(1), // Force serial for deterministic timing
        ..Default::default()
    };

    let engine = CrawlEngine::builder()
        .config(config)
        .rate_limiter(PerDomainThrottle::new(Duration::from_millis(200)))
        .build()
        .unwrap();

    let start = Instant::now();
    let result = engine.crawl(&mock.uri()).await.unwrap();
    let elapsed = start.elapsed();

    assert!(
        result.pages.len() >= 3,
        "should crawl at least 3 pages, got {}",
        result.pages.len()
    );
    // 4 pages total (root + 3 children), 3 inter-request delays of 200ms each = 600ms minimum.
    assert!(
        elapsed.as_millis() >= 400,
        "should take at least 400ms (delays between requests), took {}ms",
        elapsed.as_millis()
    );
}
