//! Integration tests for markdown output: citations, fit_content, and structure.

use kreuzcrawl::{CrawlConfig, CrawlEngine, NoopRateLimiter};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_markdown_output_is_populated() {
    let mock = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(
                    r#"<html><body>
                <nav><a href="/">Home</a> | <a href="/about">About</a></nav>
                <article>
                    <h1>Title</h1>
                    <p>Visit <a href="https://example.com">Example</a> for more info.</p>
                    <p>Some additional content here to fill the page.</p>
                </article>
                <footer>Copyright 2024. All rights reserved.</footer>
            </body></html>"#,
                )
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let engine = CrawlEngine::builder()
        .config(CrawlConfig::default())
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    let result = engine.scrape(&mock.uri()).await.unwrap();
    let md = result.markdown.expect("markdown should be present");

    // Markdown content should contain the title and text.
    assert!(
        md.content.contains("Title"),
        "markdown content should contain the heading"
    );
    assert!(
        md.content.contains("Example"),
        "markdown content should contain link text"
    );

    // Citations should exist.
    assert!(
        md.citations.is_some(),
        "citations should be populated for pages with links"
    );
    let citations = md.citations.unwrap();
    assert!(
        !citations.references.is_empty(),
        "should have citation references"
    );

    // Fit markdown should exist.
    assert!(md.fit_content.is_some(), "fit content should be populated");
}

#[tokio::test]
async fn test_markdown_heading_extraction() {
    let mock = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(
                    r#"<html><body>
                    <h1>Main Title</h1>
                    <h2>Section One</h2>
                    <p>Content for section one.</p>
                    <h2>Section Two</h2>
                    <p>Content for section two.</p>
                </body></html>"#,
                )
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let engine = CrawlEngine::builder()
        .config(CrawlConfig::default())
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    let result = engine.scrape(&mock.uri()).await.unwrap();
    let md = result.markdown.expect("markdown should be present");

    assert!(
        md.content.contains("# Main Title") || md.content.contains("Main Title"),
        "should contain h1 content in markdown: {}",
        md.content
    );
    assert!(
        md.content.contains("Section One"),
        "should contain h2 content"
    );
}
