#![allow(clippy::unwrap_used, clippy::panic)]
#![cfg(feature = "mcp")]

//! MCP contract tests.
//!
//! These tests verify that types used by the MCP server maintain
//! serialization stability (JSON round-trip).

use kreuzcrawl::CrawlConfig;

#[test]
fn test_crawl_config_json_roundtrip() {
    let config = CrawlConfig {
        max_depth: Some(5),
        max_pages: Some(100),
        max_concurrent: Some(10),
        respect_robots_txt: true,
        user_agent: Some("TestBot/1.0".to_string()),
        stay_on_domain: true,
        allow_subdomains: true,
        cookies_enabled: true,
        retry_count: 3,
        max_redirects: 5,
        ..Default::default()
    };

    let json = serde_json::to_string_pretty(&config).expect("serialize should succeed");
    let deserialized: CrawlConfig = serde_json::from_str(&json).expect("deserialize should succeed");

    assert_eq!(deserialized.max_depth, Some(5));
    assert_eq!(deserialized.max_pages, Some(100));
    assert_eq!(deserialized.max_concurrent, Some(10));
    assert!(deserialized.respect_robots_txt);
    assert_eq!(deserialized.user_agent.as_deref(), Some("TestBot/1.0"));
    assert!(deserialized.stay_on_domain);
    assert!(deserialized.allow_subdomains);
    assert!(deserialized.cookies_enabled);
    assert_eq!(deserialized.retry_count, 3);
    assert_eq!(deserialized.max_redirects, 5);
}
