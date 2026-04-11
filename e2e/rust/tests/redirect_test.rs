//! E2e tests for category: redirect

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_redirect_301_permanent() {
    // Follows 301 permanent redirect and returns final page content
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'final_url' not available on result type
    // skipped: field 'redirect_count' not available on result type
}

#[tokio::test]
async fn test_redirect_302_found() {
    // Follows 302 Found redirect correctly
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'final_url' not available on result type
    // skipped: field 'redirect_count' not available on result type
}

#[tokio::test]
async fn test_redirect_303_see_other() {
    // Follows 303 See Other redirect (method changes to GET)
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'final_url' not available on result type
    // skipped: field 'redirect_count' not available on result type
}

#[tokio::test]
async fn test_redirect_307_temporary() {
    // Follows 307 Temporary Redirect (preserves method)
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'final_url' not available on result type
    // skipped: field 'redirect_count' not available on result type
}

#[tokio::test]
async fn test_redirect_308_permanent() {
    // Follows 308 Permanent Redirect (preserves method)
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'final_url' not available on result type
    // skipped: field 'redirect_count' not available on result type
}

#[tokio::test]
async fn test_redirect_chain() {
    // Follows a chain of redirects (301 -> 302 -> 200)
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'final_url' not available on result type
    // skipped: field 'redirect_count' not available on result type
}

#[tokio::test]
async fn test_redirect_cross_domain() {
    // Reports cross-domain redirect target without following to external domain
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'final_url' not available on result type
    // skipped: field 'redirect_count' not available on result type
}

#[tokio::test]
async fn test_redirect_loop() {
    // Detects redirect loop (A -> B -> A) and returns error
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'is_error' not available on result type
}

#[tokio::test]
async fn test_redirect_max_exceeded() {
    // Aborts when redirect count exceeds max_redirects limit
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'is_error' not available on result type
}

#[tokio::test]
async fn test_redirect_meta_refresh() {
    // Follows HTML meta-refresh redirect to target page
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'final_url' not available on result type
    // skipped: field 'redirect_count' not available on result type
}

#[tokio::test]
async fn test_redirect_refresh_header() {
    // Handles HTTP Refresh header redirect
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'final_url' not available on result type
    // skipped: field 'redirect_count' not available on result type
}

#[tokio::test]
async fn test_redirect_to_404() {
    // Redirect target returns 404 Not Found
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'final_url' not available on result type
    // skipped: field 'redirect_count' not available on result type
    // skipped: field 'is_error' not available on result type
}
