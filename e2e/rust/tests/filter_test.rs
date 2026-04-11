//! E2e tests for category: filter

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_filter_bm25_crawl_integration() {
    // BM25 filter works during multi-page crawl, keeping relevant pages
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'filter.remaining_contain_keyword' not available on result type
}

#[tokio::test]
async fn test_filter_bm25_empty_query() {
    // BM25 filter with empty query passes all pages through
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'crawl.pages_crawled' not available on result type
}

#[tokio::test]
async fn test_filter_bm25_high_threshold() {
    // BM25 filter with very high threshold filters out all pages
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'filter.pages_after_filter' not available on result type
}

#[tokio::test]
async fn test_filter_bm25_relevant_pages() {
    // BM25 filter keeps only pages relevant to the query
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'filter.remaining_contain_keyword' not available on result type
}

#[tokio::test]
async fn test_filter_bm25_threshold_zero() {
    // BM25 filter with zero threshold passes all pages
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'crawl.pages_crawled' not available on result type
}

#[tokio::test]
async fn test_filter_noop_crawl_all_kept() {
    // NoopFilter keeps all pages during a multi-page crawl
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'filter.pages_after_filter' not available on result type
}

#[tokio::test]
async fn test_filter_noop_passes_all() {
    // No content filter passes all crawled pages through
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'crawl.pages_crawled' not available on result type
}
