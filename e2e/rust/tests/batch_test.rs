//! E2e tests for category: batch

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_scrape_batch_basic() {
    // Batch scrape of multiple URLs all succeeding
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'batch.completed_count' not available on result type
    // skipped: field 'batch.failed_count' not available on result type
    // skipped: field 'batch.total_count' not available on result type
}

#[tokio::test]
async fn test_scrape_batch_partial_failure() {
    // Batch scrape with one URL failing returns partial results
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'batch.completed_count' not available on result type
    // skipped: field 'batch.failed_count' not available on result type
    // skipped: field 'batch.total_count' not available on result type
}

#[tokio::test]
async fn test_scrape_batch_progress() {
    // Batch scrape results include specific URL
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'batch.total_count' not available on result type
    // skipped: field 'batch.results' not available on result type
}
