//! E2e tests for category: robots

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_robots_allow_all() {
    // Permissive robots.txt allows all paths
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.is_allowed, true, "equals assertion failed");
}

#[tokio::test]
async fn test_robots_allow_override() {
    // Allow directive overrides Disallow for specific paths
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.is_allowed, true, "equals assertion failed");
}

#[tokio::test]
async fn test_robots_comments_handling() {
    // Correctly parses robots.txt with inline and line comments
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.is_allowed, true, "equals assertion failed");
}

#[tokio::test]
async fn test_robots_crawl_delay() {
    // Respects crawl-delay directive from robots.txt
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    let crawl_delay = result.crawl_delay.as_deref().unwrap_or("");
    assert_eq!(crawl_delay, 2, "equals assertion failed");
}

#[tokio::test]
async fn test_robots_disallow_path() {
    // Robots.txt disallows specific paths
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.is_allowed, false, "equals assertion failed");
}

#[tokio::test]
async fn test_robots_meta_nofollow() {
    // Detects nofollow meta robots tag and skips link extraction
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.nofollow_detected, true, "equals assertion failed");
}

#[tokio::test]
async fn test_robots_meta_noindex() {
    // Detects noindex meta robots tag in HTML page
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.noindex_detected, true, "equals assertion failed");
}

#[tokio::test]
async fn test_robots_missing_404() {
    // Missing robots.txt (404) allows all crawling
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.is_allowed, true, "equals assertion failed");
}

#[tokio::test]
async fn test_robots_multiple_user_agents() {
    // Picks the most specific user-agent block from robots.txt
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.is_allowed, true, "equals assertion failed");
}

#[tokio::test]
async fn test_robots_request_rate() {
    // Parses request-rate directive from robots.txt
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    let crawl_delay = result.crawl_delay.as_deref().unwrap_or("");
    assert_eq!(crawl_delay, 5, "equals assertion failed");
    assert_eq!(result.is_allowed, true, "equals assertion failed");
}

#[tokio::test]
async fn test_robots_sitemap_directive() {
    // Discovers sitemap URL from Sitemap directive in robots.txt
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.is_allowed, true, "equals assertion failed");
}

#[tokio::test]
async fn test_robots_user_agent_specific() {
    // Matches user-agent specific rules in robots.txt
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.is_allowed, false, "equals assertion failed");
}

#[tokio::test]
async fn test_robots_wildcard_paths() {
    // Handles wildcard Disallow patterns in robots.txt
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    assert_eq!(result.is_allowed, false, "equals assertion failed");
}

#[tokio::test]
async fn test_robots_x_robots_tag() {
    // Respects X-Robots-Tag HTTP header directives
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = String::new();
    let result = scrape(&engine, &url).await.expect("should succeed");
    let x_robots_tag = result.x_robots_tag.as_deref().unwrap_or("");
    assert_eq!(x_robots_tag.trim(), r#"noindex, nofollow"#, "equals assertion failed");
    assert_eq!(result.noindex_detected, true, "equals assertion failed");
    assert_eq!(result.nofollow_detected, true, "equals assertion failed");
}
