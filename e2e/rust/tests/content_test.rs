//! E2e tests for category: content

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_content_204_no_content() {
    // Handles 204 No Content response gracefully
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.status_code, 204, "equals assertion failed");
    assert!(result.html.is_empty(), "expected empty value");
}

#[tokio::test]
async fn test_content_charset_iso8859() {
    // Handles ISO-8859-1 encoded page correctly
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    let detected_charset = result.detected_charset.as_deref().unwrap_or("");
    assert_eq!(detected_charset.trim(), r#"iso-8859-1"#, "equals assertion failed");
}

#[tokio::test]
async fn test_content_empty_body() {
    // Handles 200 response with empty body gracefully
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.status_code, 200, "equals assertion failed");
}

#[tokio::test]
async fn test_content_gzip_compressed() {
    // Handles response with Accept-Encoding gzip negotiation
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(!result.html.is_empty(), "expected non-empty value");
    assert_eq!(result.status_code, 200, "equals assertion failed");
}

#[tokio::test]
async fn test_content_large_page_limit() {
    // Respects max body size limit and truncates or skips oversized pages
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(result.body_size < 1025, "expected < 1025");
}

#[tokio::test]
async fn test_content_main_only() {
    // Extracts only main content area, excluding nav, sidebar, footer
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.main_content_only, true, "equals assertion failed");
}

#[tokio::test]
async fn test_content_pdf_no_extension() {
    // Detects PDF content by Content-Type header when URL has no .pdf extension
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.is_pdf, true, "equals assertion failed");
}

#[tokio::test]
async fn test_content_remove_tags() {
    // Removes specified HTML elements by CSS selector before processing
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(!result.html.is_empty(), "expected non-empty value");
}

#[tokio::test]
async fn test_content_utf8_bom() {
    // Handles UTF-8 content with BOM marker correctly
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    let detected_charset = result.detected_charset.as_deref().unwrap_or("");
    assert_eq!(detected_charset.trim(), r#"utf-8"#, "equals assertion failed");
    assert!(!result.html.is_empty(), "expected non-empty value");
}
