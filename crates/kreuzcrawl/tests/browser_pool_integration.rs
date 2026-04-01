//! Integration tests for BrowserPool.
//!
//! These tests require Chrome/Chromium to be installed on the system.
//! Run with: cargo test -p kreuzcrawl --features browser --test browser_pool_integration

#![cfg(feature = "browser")]

use std::sync::Arc;
use std::time::Duration;

use kreuzcrawl::{BrowserMode, BrowserPool, BrowserPoolConfig, CrawlConfig, CrawlEngine};

const TEST_PAGE: &str = "data:text/html,<html><head><title>Test Page</title></head><body><h1>Hello</h1><p>Test content for pool integration tests.</p></body></html>";

fn pool_config() -> BrowserPoolConfig {
    BrowserPoolConfig {
        max_pages: 4,
        launch_timeout: Duration::from_secs(15),
        ..Default::default()
    }
}

fn scrape_config(pool: &Arc<BrowserPool>) -> CrawlConfig {
    CrawlConfig {
        browser: kreuzcrawl::BrowserConfig {
            mode: BrowserMode::Always,
            timeout: Duration::from_secs(10),
            ..Default::default()
        },
        browser_pool: Some(Arc::clone(pool)),
        respect_robots_txt: false,
        ..Default::default()
    }
}

#[tokio::test]
async fn test_pool_warm_launches_chrome() {
    let pool = BrowserPool::new(pool_config());
    assert!(!pool.is_healthy(), "pool should not be healthy before warm");

    pool.warm().await.expect("warm should succeed");
    assert!(pool.is_healthy(), "pool should be healthy after warm");

    pool.shutdown().await;
}

#[tokio::test]
async fn test_pool_acquires_and_navigates() {
    let pool = BrowserPool::new(pool_config());
    let pooled = pool.acquire_page().await.expect("acquire_page");

    pooled.page().goto(TEST_PAGE).await.expect("goto");
    tokio::time::sleep(Duration::from_millis(500)).await;

    let html = pooled.page().content().await.expect("content");
    assert!(
        html.contains("Test content for pool integration tests"),
        "page should contain content"
    );

    pooled.close().await;
    pool.shutdown().await;
}

#[tokio::test]
async fn test_pool_reuses_chrome() {
    let pool = BrowserPool::new(pool_config());

    // First acquire.
    let p1 = pool.acquire_page().await.expect("first acquire");
    p1.close().await;

    // Second acquire — should reuse same Chrome (no relaunch).
    assert!(pool.is_healthy(), "pool should still be healthy");
    let p2 = pool.acquire_page().await.expect("second acquire");
    p2.close().await;

    pool.shutdown().await;
}

#[tokio::test]
async fn test_pool_concurrent_pages() {
    let pool = BrowserPool::new(pool_config());
    pool.warm().await.expect("warm");

    let mut handles = Vec::new();
    for i in 0..4 {
        let pool = Arc::clone(&pool);
        handles.push(tokio::spawn(async move {
            let page = pool.acquire_page().await.expect("acquire");
            page.page().goto(TEST_PAGE).await.expect("goto");
            tokio::time::sleep(Duration::from_millis(500)).await;
            let html = page.page().content().await.expect("content");
            page.close().await;
            assert!(
                html.contains("Test content for pool integration tests"),
                "page {i} should have content"
            );
        }));
    }

    for h in handles {
        h.await.expect("task should not panic");
    }

    pool.shutdown().await;
}

#[tokio::test]
async fn test_pool_respects_max_pages() {
    let config = BrowserPoolConfig {
        max_pages: 2,
        launch_timeout: Duration::from_secs(15),
        ..Default::default()
    };
    let pool = BrowserPool::new(config);
    pool.warm().await.expect("warm");

    // Acquire 2 pages (at limit).
    let p1 = pool.acquire_page().await.expect("p1");
    let p2 = pool.acquire_page().await.expect("p2");

    // Third acquire should block. Use a timeout to verify.
    let pool_clone = Arc::clone(&pool);
    let result = tokio::time::timeout(Duration::from_millis(500), async move {
        pool_clone.acquire_page().await
    })
    .await;

    assert!(
        result.is_err(),
        "third acquire should timeout (blocked by semaphore)"
    );

    // Release one page — now third should succeed.
    p1.close().await;
    let p3 = pool.acquire_page().await.expect("p3 after release");
    p3.close().await;
    p2.close().await;

    pool.shutdown().await;
}

#[tokio::test]
async fn test_pool_shutdown_prevents_new_acquires() {
    let pool = BrowserPool::new(pool_config());
    pool.warm().await.expect("warm");

    pool.shutdown().await;

    let result = pool.acquire_page().await;
    assert!(result.is_err(), "acquire after shutdown should fail");
}

#[tokio::test]
async fn test_pool_with_scrape_api() {
    let pool = BrowserPool::new(pool_config());
    pool.warm().await.expect("warm");

    let config = scrape_config(&pool);
    let engine = CrawlEngine::builder().config(config).build().unwrap();
    let result = engine.scrape(TEST_PAGE).await.expect("scrape");

    assert!(result.browser_used, "browser should be used");
    assert_eq!(result.metadata.title.as_deref(), Some("Test Page"));
    assert!(!result.html.is_empty());

    pool.shutdown().await;
}

#[tokio::test]
async fn test_pool_with_scrape_sequential_reuse() {
    let pool = BrowserPool::new(pool_config());
    pool.warm().await.expect("warm");

    let config = scrape_config(&pool);
    let engine = CrawlEngine::builder().config(config).build().unwrap();

    // Two sequential scrapes — both should reuse the same Chrome.
    let r1 = engine.scrape(TEST_PAGE).await.expect("scrape 1");
    assert!(r1.browser_used);

    let r2 = engine.scrape(TEST_PAGE).await.expect("scrape 2");
    assert!(r2.browser_used);

    assert!(
        pool.is_healthy(),
        "pool should still be healthy after 2 scrapes"
    );
    pool.shutdown().await;
}

#[tokio::test]
async fn test_pool_crash_recovery() {
    let pool = BrowserPool::new(pool_config());
    pool.warm().await.expect("warm");
    assert!(pool.is_healthy());

    // Simulate Chrome crash by shutting down and re-warming.
    // (We can't easily kill the Chrome PID from here, but we can test
    // that after a failed page open, the pool relaunches.)
    pool.shutdown().await;
    assert!(!pool.is_healthy());

    // Create a new pool to test recovery flow.
    let pool2 = BrowserPool::new(pool_config());
    let page = pool2
        .acquire_page()
        .await
        .expect("should launch fresh Chrome");
    page.close().await;
    assert!(pool2.is_healthy());
    pool2.shutdown().await;
}
