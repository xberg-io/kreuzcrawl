//! Integration test: verifies that a pre-built `BrowserPool` can be injected into
//! `CrawlEngineBuilder` and is reused across multiple `batch_crawl` calls without
//! being reconstructed.
//!
//! Chrome is never actually launched in this test — the pool remains idle (no URLs
//! use `BrowserMode::Always`) and the test asserts construction-once via
//! `Arc::strong_count`.

use std::sync::Arc;

use kreuzcrawl::{BrowserPool, BrowserPoolConfig, CrawlConfig, batch_crawl, create_engine};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Construct a pool, hand it to `create_engine` via `CrawlConfig.browser_pool`, then verify:
/// 1. The `Arc` reference count reflects the pool being held by the engine's config.
/// 2. Two consecutive `batch_crawl` calls both succeed using the same engine (and
///    therefore the same pool arc), proving no re-construction per call.
#[tokio::test]
async fn pool_injected_once_and_reused_across_batch_crawl_calls() {
    let mock = MockServer::start().await;

    for page in ["page1", "page2"] {
        Mock::given(method("GET"))
            .and(path(format!("/{page}")))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(format!("<html><body>{page}</body></html>"))
                    .append_header("content-type", "text/html"),
            )
            .mount(&mock)
            .await;
    }

    // Construct the pool once — Chrome is NOT launched at this point.
    let pool = BrowserPool::new(BrowserPoolConfig::default());

    // Before injection: only the caller holds the Arc.
    let initial_count = Arc::strong_count(&pool);
    assert_eq!(initial_count, 1, "only the caller should hold the pool initially");

    // Inject the pool via CrawlConfig (the field-level injection path used by
    // downstream callers that don't use the builder directly).
    let config = CrawlConfig {
        max_depth: Some(0),
        browser_pool: Some(Arc::clone(&pool)),
        ..Default::default()
    };
    let engine_handle = create_engine(Some(config)).expect("engine build must not fail");

    // After injection: caller (1) + engine's config (1) = 2.
    let post_injection_count = Arc::strong_count(&pool);
    assert_eq!(
        post_injection_count, 2,
        "pool should be held by both caller and the engine (got {post_injection_count})"
    );

    let urls1: Vec<String> = vec![format!("{}/page1", mock.uri())];
    let urls2: Vec<String> = vec![format!("{}/page2", mock.uri())];

    // First batch_crawl call — BrowserMode defaults to Auto, no JS detected,
    // so Chrome is never launched; all fetches go through the HTTP stack.
    let results1 = batch_crawl(&engine_handle, urls1)
        .await
        .expect("first batch_crawl must succeed");
    assert_eq!(results1.completed_count, 1, "first call: all URLs should succeed");

    // Second batch_crawl call — same engine, same pool arc.
    let results2 = batch_crawl(&engine_handle, urls2)
        .await
        .expect("second batch_crawl must succeed");
    assert_eq!(results2.completed_count, 1, "second call: all URLs should succeed");

    // Pool ref-count must still be 2 (no new Arc was created by either call).
    let after_crawl_count = Arc::strong_count(&pool);
    assert_eq!(
        after_crawl_count, 2,
        "no extra pool clones should have been created during batch_crawl (got {after_crawl_count})"
    );
}

/// Verify that `CrawlEngineBuilder::with_browser_pool` sets the pool correctly,
/// producing the same refcount behavior as the config-field path.
#[tokio::test]
async fn with_browser_pool_builder_method_injects_pool() {
    use kreuzcrawl::CrawlEngine;

    let pool = BrowserPool::new(BrowserPoolConfig::default());
    assert_eq!(Arc::strong_count(&pool), 1, "only caller before builder");

    // Build via the new builder method — should compile and clone the Arc once.
    let _engine = CrawlEngine::builder()
        .with_browser_pool(Arc::clone(&pool))
        .build()
        .expect("builder with pool must not fail");

    // After build: caller (1) + engine's config (1) = 2.
    assert_eq!(
        Arc::strong_count(&pool),
        2,
        "builder should place exactly one clone of the pool in the engine's config"
    );
}
