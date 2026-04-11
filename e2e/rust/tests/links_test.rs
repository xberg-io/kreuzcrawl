//! E2e tests for category: links

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_links_anchor_fragment() {
    // Identifies fragment-only links as anchor type
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(result.links[0].link_type.to_string().contains(r#"anchor"#), "expected to contain: {}", r#"anchor"#);
}

#[tokio::test]
async fn test_links_base_tag() {
    // Resolves relative URLs using base tag href
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(result.links.len() > 2, "expected > 2");
    assert!(result.links[0].url.to_string().contains(r#"example.com"#), "expected to contain: {}", r#"example.com"#);
}

#[tokio::test]
async fn test_links_document_types() {
    // Detects PDF, DOCX, XLSX links as document type
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(result.links[0].link_type.to_string().contains(r#"document"#), "expected to contain: {}", r#"document"#);
}

#[tokio::test]
async fn test_links_empty_href() {
    // Handles empty href attributes without errors
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(result.links.len() > 0, "expected > 0");
    assert!(result.links[0].url.to_string().contains(r#"/valid"#), "expected to contain: {}", r#"/valid"#);
}

#[tokio::test]
async fn test_links_internal_external_classification() {
    // Correctly classifies internal vs external links by domain
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(result.links.len() > 4, "expected > 4");
    assert!(result.links[0].link_type.to_string().contains(r#"internal"#), "expected to contain: {}", r#"internal"#);
    assert!(result.links[0].link_type.to_string().contains(r#"external"#), "expected to contain: {}", r#"external"#);
}

#[tokio::test]
async fn test_links_mailto_javascript_skip() {
    // Skips mailto:, javascript:, and tel: scheme links
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(result.links.len() > 0, "expected > 0");
    assert!(!result.links[0].url.to_string().contains(r#"mailto:"#), "expected NOT to contain: {}", r#"mailto:"#);
}

#[tokio::test]
async fn test_links_protocol_relative() {
    // Handles protocol-relative URLs (//example.com) correctly
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(result.links.len() > 1, "expected > 1");
    assert!(result.links[0].url.to_string().contains(r#"//"#), "expected to contain: {}", r#"//"#);
}

#[tokio::test]
async fn test_links_rel_attributes() {
    // Preserves rel=nofollow and rel=canonical attributes
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(result.links.len() > 0, "expected > 0");
}

#[tokio::test]
async fn test_links_relative_parent() {
    // Resolves ../ and ./ relative parent path links correctly
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(result.links.len() > 3, "expected > 3");
}
