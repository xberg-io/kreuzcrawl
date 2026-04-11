//! E2e tests for category: sitemap

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_sitemap_basic() {
    // Parses a standard urlset sitemap
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'urls.length' not available on result type
    // skipped: field 'has_lastmod' not available on result type
}

#[tokio::test]
async fn test_sitemap_compressed_gzip() {
    // Parses a gzip-compressed sitemap file
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'urls.length' not available on result type
}

#[tokio::test]
async fn test_sitemap_empty() {
    // Handles empty sitemap gracefully
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'urls.length' not available on result type
}

#[tokio::test]
async fn test_sitemap_from_robots_txt() {
    // Discovers sitemap via robots.txt Sitemap directive
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'urls.length' not available on result type
}

#[tokio::test]
async fn test_sitemap_index() {
    // Follows sitemap index to discover child sitemaps
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'urls.length' not available on result type
}

#[tokio::test]
async fn test_sitemap_lastmod_filter() {
    // Filters sitemap URLs by lastmod date
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'urls.length' not available on result type
    // skipped: field 'has_lastmod' not available on result type
}

#[tokio::test]
async fn test_sitemap_only_mode() {
    // Uses sitemap URLs exclusively without following page links
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'urls.length' not available on result type
}

#[tokio::test]
async fn test_sitemap_xhtml_links() {
    // Parses sitemap with XHTML namespace alternate links
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'urls.length' not available on result type
    // skipped: field 'has_lastmod' not available on result type
}
