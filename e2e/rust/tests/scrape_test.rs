//! E2e tests for category: scrape

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_scrape_asset_dedup() {
    // Same asset linked twice results in one download with one unique hash
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert_eq!(result.assets.len(), 2, "equals assertion failed");
    assert!(!result.assets[0].content_hash.is_empty(), "expected non-empty value");
}

#[tokio::test]
async fn test_scrape_asset_max_size() {
    // Skips assets exceeding max_asset_size limit
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert_eq!(result.assets.len(), 2, "equals assertion failed");
}

#[tokio::test]
async fn test_scrape_asset_type_filter() {
    // Only downloads image assets when asset_types filter is set
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert_eq!(result.assets.len(), 1, "equals assertion failed");
    assert!(result.assets[0].asset_category.to_string().contains(r#"image"#), "expected to contain: {}", r#"image"#);
}

#[tokio::test]
async fn test_scrape_basic_html_page() {
    // Scrapes a simple HTML page and extracts title, description, and links
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    let metadata_title = result.metadata.title.as_deref().unwrap_or("");
    let metadata_description = result.metadata.description.as_deref().unwrap_or("");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert_eq!(result.content_type.trim(), r#"text/html"#, "equals assertion failed");
    assert!(!result.html.is_empty(), "expected non-empty value");
    assert_eq!(metadata_title.trim(), r#"Example Domain"#, "equals assertion failed");
    assert!(metadata_description.to_string().contains(r#"illustrative examples"#), "expected to contain: {}", r#"illustrative examples"#);
    assert!(result.metadata.canonical_url.is_some(), "expected metadata.canonical_url to be present");
    assert!(result.links.len() > 0, "expected > 0");
    assert!(result.links[0].link_type.to_string().contains(r#"external"#), "expected to contain: {}", r#"external"#);
    assert_eq!(result.images.len(), 0, "equals assertion failed");
    assert!(result.metadata.og_title.is_none(), "expected og.title to be absent");
}

#[tokio::test]
async fn test_scrape_complex_links() {
    // Classifies links by type: internal, external, anchor, document, image
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert!(result.links.len() > 9, "expected > 9");
    assert!(result.links[0].link_type.to_string().contains(r#"internal"#), "expected to contain: {}", r#"internal"#);
    assert!(result.links[0].link_type.to_string().contains(r#"external"#), "expected to contain: {}", r#"external"#);
    assert!(result.links[0].link_type.to_string().contains(r#"anchor"#), "expected to contain: {}", r#"anchor"#);
    assert!(result.links[0].link_type.to_string().contains(r#"document"#), "expected to contain: {}", r#"document"#);
}

#[tokio::test]
async fn test_scrape_download_assets() {
    // Downloads CSS, JS, and image assets from page
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert!(result.assets.len() > 2, "expected > 2");
}

#[tokio::test]
async fn test_scrape_dublin_core() {
    // Extracts Dublin Core metadata from a page
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    let metadata_dc_title = result.metadata.dc_title.as_deref().unwrap_or("");
    let metadata_dc_creator = result.metadata.dc_creator.as_deref().unwrap_or("");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert!(!metadata_dc_title.is_empty(), "expected non-empty value");
    assert_eq!(metadata_dc_title.trim(), r#"Effects of Climate Change on Marine Biodiversity"#, "equals assertion failed");
    assert_eq!(metadata_dc_creator.trim(), r#"Dr. Jane Smith"#, "equals assertion failed");
}

#[tokio::test]
async fn test_scrape_empty_page() {
    // Handles an empty HTML document without errors
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert!(result.links.len() > -1, "expected > -1");
    assert_eq!(result.images.len(), 0, "equals assertion failed");
}

#[tokio::test]
async fn test_scrape_feed_discovery() {
    // Discovers RSS, Atom, and JSON feed links
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert!(result.feeds.len() >= 3, "expected >= 3");
}

#[tokio::test]
async fn test_scrape_image_sources() {
    // Extracts images from img, picture, og:image, twitter:image
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    let metadata_og_image = result.metadata.og_image.as_deref().unwrap_or("");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert!(result.images.len() > 4, "expected > 4");
    assert_eq!(metadata_og_image.trim(), r#"https://example.com/images/og-hero.jpg"#, "equals assertion failed");
}

#[tokio::test]
async fn test_scrape_js_heavy_spa() {
    // Handles SPA page with JavaScript-only content (no server-rendered HTML)
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert!(!result.html.is_empty(), "expected non-empty value");
}

#[tokio::test]
async fn test_scrape_json_ld() {
    // Extracts JSON-LD structured data from a page
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    let json_ld_name = result.json_ld[0].name.as_deref().unwrap_or("");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert!(!result.json_ld.is_empty(), "expected non-empty value");
    assert_eq!(result.json_ld[0].schema_type.trim(), r#"Recipe"#, "equals assertion failed");
    assert_eq!(json_ld_name.trim(), r#"Best Chocolate Cake"#, "equals assertion failed");
}

#[tokio::test]
async fn test_scrape_malformed_html() {
    // Gracefully handles broken HTML without crashing
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    let metadata_description = result.metadata.description.as_deref().unwrap_or("");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert!(!result.html.is_empty(), "expected non-empty value");
    assert!(metadata_description.to_string().contains(r#"broken HTML"#), "expected to contain: {}", r#"broken HTML"#);
}

#[tokio::test]
async fn test_scrape_og_metadata() {
    // Extracts full Open Graph metadata from a page
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    let metadata_og_title = result.metadata.og_title.as_deref().unwrap_or("");
    let metadata_og_type = result.metadata.og_type.as_deref().unwrap_or("");
    let metadata_og_image = result.metadata.og_image.as_deref().unwrap_or("");
    let metadata_title = result.metadata.title.as_deref().unwrap_or("");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert!(!metadata_og_title.is_empty(), "expected non-empty value");
    assert_eq!(metadata_og_title.trim(), r#"Article Title"#, "equals assertion failed");
    assert_eq!(metadata_og_type.trim(), r#"article"#, "equals assertion failed");
    assert_eq!(metadata_og_image.trim(), r#"https://example.com/images/article-hero.jpg"#, "equals assertion failed");
    assert!(result.metadata.og_description.is_some(), "expected og.description to be present");
    assert_eq!(metadata_title.trim(), r#"Article Title - Example Blog"#, "equals assertion failed");
}

#[tokio::test]
async fn test_scrape_twitter_card() {
    // Extracts Twitter Card metadata from a page
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    let metadata_twitter_card = result.metadata.twitter_card.as_deref().unwrap_or("");
    let metadata_twitter_title = result.metadata.twitter_title.as_deref().unwrap_or("");
    assert_eq!(result.status_code, 200, "equals assertion failed");
    assert!(result.metadata.twitter_card.is_some(), "expected twitter.card to be present");
    assert_eq!(metadata_twitter_card.trim(), r#"summary_large_image"#, "equals assertion failed");
    assert_eq!(metadata_twitter_title.trim(), r#"New Product Launch"#, "equals assertion failed");
}
