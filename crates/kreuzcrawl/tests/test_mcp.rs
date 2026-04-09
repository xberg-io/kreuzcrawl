#![cfg(feature = "mcp")]

//! MCP parameter and formatting tests.
//!
//! These tests verify serialization/deserialization of MCP parameter types
//! and formatting helpers without requiring network access.

use kreuzcrawl::mcp::{
    BatchScrapeParams, CrawlParams, CrawlStatusParams, DownloadParams, InteractParams, MapParams,
    ResearchParams, ScrapeParams, ScreenshotParams,
};

#[test]
fn test_scrape_params_url_required() {
    let json = r#"{}"#;
    let result = serde_json::from_str::<ScrapeParams>(json);
    assert!(result.is_err(), "missing url should fail deserialization");
}

#[test]
fn test_scrape_params_with_format() {
    let json = r#"{"url": "https://example.com", "format": "markdown"}"#;
    let params: ScrapeParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.url, "https://example.com");
    assert_eq!(params.format.as_deref(), Some("markdown"));
    assert_eq!(params.use_browser, None);
}

#[test]
fn test_crawl_params_defaults() {
    let json = r#"{"url": "https://example.com"}"#;
    let params: CrawlParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.url, "https://example.com");
    assert_eq!(params.max_depth, None);
    assert_eq!(params.max_pages, None);
    assert_eq!(params.format, None);
    assert_eq!(params.stay_on_domain, None);
}

#[test]
fn test_interact_params_with_actions() {
    let json = r##"{
        "url": "https://example.com",
        "actions": [
            {"type": "click", "selector": "#submit"},
            {"type": "wait", "duration_ms": 1000}
        ]
    }"##;
    let params: InteractParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.url, "https://example.com");
    assert_eq!(params.actions.len(), 2);
    assert_eq!(params.actions[0]["type"], "click");
    assert_eq!(params.actions[1]["type"], "wait");
}

#[test]
fn test_download_params_with_max_size() {
    let json = r#"{"url": "https://example.com/doc.pdf", "max_size": 10485760}"#;
    let params: DownloadParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.url, "https://example.com/doc.pdf");
    assert_eq!(params.max_size, Some(10_485_760));
}

#[test]
fn test_all_params_have_json_schema() {
    use rmcp::schemars;

    // Verify that schema generation compiles and produces valid schemas for each type
    let _ = schemars::schema_for!(ScrapeParams);
    let _ = schemars::schema_for!(CrawlParams);
    let _ = schemars::schema_for!(MapParams);
    let _ = schemars::schema_for!(BatchScrapeParams);
    let _ = schemars::schema_for!(ScreenshotParams);
    let _ = schemars::schema_for!(DownloadParams);
    let _ = schemars::schema_for!(InteractParams);
    let _ = schemars::schema_for!(ResearchParams);
}

#[test]
fn test_research_params_with_seed_urls() {
    let json = r#"{
        "query": "rust async patterns",
        "max_depth": 3,
        "max_pages": 20,
        "seed_urls": ["https://docs.rs", "https://blog.rust-lang.org"]
    }"#;
    let params: ResearchParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.query, "rust async patterns");
    assert_eq!(params.max_depth, Some(3));
    assert_eq!(params.max_pages, Some(20));
    let seeds = params.seed_urls.unwrap();
    assert_eq!(seeds.len(), 2);
    assert_eq!(seeds[0], "https://docs.rs");
}

#[test]
fn test_map_params_with_search() {
    let json = r#"{"url": "https://example.com", "search": "blog", "limit": 100}"#;
    let params: MapParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.url, "https://example.com");
    assert_eq!(params.search.as_deref(), Some("blog"));
    assert_eq!(params.limit, Some(100));
}

#[test]
fn test_batch_scrape_params() {
    let json = r#"{
        "urls": ["https://a.com", "https://b.com", "https://c.com"],
        "format": "json",
        "concurrency": 5
    }"#;
    let params: BatchScrapeParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.urls.len(), 3);
    assert_eq!(params.format.as_deref(), Some("json"));
    assert_eq!(params.concurrency, Some(5));
}

#[test]
fn test_scrape_params_deny_unknown_fields() {
    let json = serde_json::json!({ "url": "https://example.com", "unknown": true });
    assert!(serde_json::from_value::<ScrapeParams>(json).is_err());
}

#[test]
fn test_crawl_params_deny_unknown_fields() {
    let json = serde_json::json!({ "url": "https://example.com", "extra": 42 });
    assert!(serde_json::from_value::<CrawlParams>(json).is_err());
}

#[test]
fn test_empty_url_in_scrape_params() {
    let json = serde_json::json!({ "url": "" });
    let params: ScrapeParams = serde_json::from_value(json).unwrap();
    assert!(params.url.is_empty()); // Deserialization succeeds, validation catches it
}

#[test]
fn test_format_as_json_valid_output() {
    use kreuzcrawl::ScrapeResult;
    let result = ScrapeResult::default();
    let json_str = kreuzcrawl::mcp::format::format_as_json(&result);
    assert!(serde_json::from_str::<serde_json::Value>(&json_str).is_ok());
}

#[test]
fn test_crawl_status_params() {
    let json = serde_json::json!({});
    let params: CrawlStatusParams = serde_json::from_value(json).unwrap();
    assert_eq!(params.job_id, None);
}

#[test]
fn test_crawl_status_params_deny_unknown_fields() {
    let json = serde_json::json!({ "job_id": "abc", "extra": true });
    assert!(serde_json::from_value::<CrawlStatusParams>(json).is_err());
}

#[test]
fn test_map_params_deny_unknown_fields() {
    let json = serde_json::json!({ "url": "https://example.com", "bogus": 1 });
    assert!(serde_json::from_value::<MapParams>(json).is_err());
}

#[test]
fn test_batch_scrape_params_deny_unknown_fields() {
    let json = serde_json::json!({ "urls": ["https://a.com"], "foo": "bar" });
    assert!(serde_json::from_value::<BatchScrapeParams>(json).is_err());
}

#[test]
fn test_screenshot_params_deny_unknown_fields() {
    let json = serde_json::json!({ "url": "https://example.com", "width": 1024 });
    assert!(serde_json::from_value::<ScreenshotParams>(json).is_err());
}

#[test]
fn test_download_params_deny_unknown_fields() {
    let json = serde_json::json!({ "url": "https://example.com/f.pdf", "nope": true });
    assert!(serde_json::from_value::<DownloadParams>(json).is_err());
}

#[test]
fn test_interact_params_deny_unknown_fields() {
    let json = serde_json::json!({ "url": "https://example.com", "actions": [], "extra": 1 });
    assert!(serde_json::from_value::<InteractParams>(json).is_err());
}

#[test]
fn test_research_params_deny_unknown_fields() {
    let json = serde_json::json!({ "query": "test", "bogus": true });
    assert!(serde_json::from_value::<ResearchParams>(json).is_err());
}
