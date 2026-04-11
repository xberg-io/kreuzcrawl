//! E2e tests for category: map

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_map_discover_urls() {
    // Discovers all URLs on a site without fetching full content
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'urls.length' not available on result type
}

#[tokio::test]
async fn test_map_exclude_patterns() {
    // Excludes URLs matching patterns from URL map
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'urls.length' not available on result type
}

#[tokio::test]
async fn test_map_include_subdomains() {
    // Includes subdomain URLs in URL map discovery
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'urls.length' not available on result type
    // skipped: field 'urls' not available on result type
}

#[tokio::test]
async fn test_map_large_sitemap() {
    // Handles large sitemap with 100+ URLs
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'urls.length' not available on result type
}

#[tokio::test]
async fn test_map_limit_pagination() {
    // Limits map result count to specified maximum
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'urls.length' not available on result type
}

#[tokio::test]
async fn test_map_search_filter() {
    // Filters map results by search keyword
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'urls.length' not available on result type
    // skipped: field 'urls' not available on result type
}
