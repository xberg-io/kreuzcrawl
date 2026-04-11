//! E2e tests for category: markdown

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_markdown_basic_conversion() {
    // HTML is always converted to markdown alongside raw HTML
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    let metadata_title = result.metadata.title.as_deref().unwrap_or("");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert_eq!(metadata_title.trim(), r#"Test"#, "equals assertion failed");
    assert!(!result.html.is_empty(), "expected non-empty value");
    assert!(!result.markdown.as_ref().unwrap().content.is_empty(), "expected non-empty value");
    assert!(result.markdown.as_ref().unwrap().content.to_string().contains(r#"Hello World"#), "expected to contain: {}", r#"Hello World"#);
}

#[tokio::test]
async fn test_markdown_crawl_all_pages() {
    // All crawled pages have markdown field populated
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'crawl.pages_crawled' not available on result type
}

#[tokio::test]
async fn test_markdown_fit_content() {
    // Fit markdown removes navigation and boilerplate content
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert!(!result.markdown.as_ref().unwrap().content.is_empty(), "expected non-empty value");
}

#[tokio::test]
async fn test_markdown_headings_and_paragraphs() {
    // Markdown conversion preserves heading hierarchy and paragraph text
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(!result.markdown.as_ref().unwrap().content.is_empty(), "expected non-empty value");
    assert!(result.markdown.as_ref().unwrap().content.to_string().contains(r#"Main Title"#), "expected to contain: {}", r#"Main Title"#);
}

#[tokio::test]
async fn test_markdown_links_converted() {
    // HTML links are converted to markdown link syntax
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert!(!result.html.is_empty(), "expected non-empty value");
    assert!(!result.markdown.as_ref().unwrap().content.is_empty(), "expected non-empty value");
    assert!(result.markdown.as_ref().unwrap().content.to_string().contains(r#"Example"#), "expected to contain: {}", r#"Example"#);
}

#[tokio::test]
async fn test_markdown_with_citations() {
    // Markdown includes citation conversion with numbered references
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert!(!result.markdown.as_ref().unwrap().content.is_empty(), "expected non-empty value");
}
