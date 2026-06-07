//! Integration tests for frontier deduplication: verifying duplicate URLs are not re-fetched.

use kreuzcrawl::{CrawlConfig, crawl, create_engine};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_duplicate_links_deduplicated() {
    let mock = MockServer::start().await;

    // Root links to /b and /c; /b also links to /c.
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body><a href=\"/b\">B</a><a href=\"/c\">C</a></body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/b"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body><a href=\"/c\">C again</a></body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/c"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>Page C</body></html>")
                .append_header("content-type", "text/html"),
        )
        .expect(1) // C should only be fetched ONCE.
        .mount(&mock)
        .await;

    let config = CrawlConfig {
        max_depth: Some(2),
        max_concurrent: Some(1),
        ..Default::default()
    };
    let handle = create_engine(Some(config)).unwrap();

    let result = crawl(&handle, &mock.uri()).await.unwrap();
    assert_eq!(
        result.pages.len(),
        3,
        "should crawl exactly 3 unique pages, got: {:?}",
        result.pages.iter().map(|p| &p.url).collect::<Vec<_>>()
    );
}
