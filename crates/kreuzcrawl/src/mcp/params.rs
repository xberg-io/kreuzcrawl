//! MCP request parameter types.
//!
//! This module defines the parameter structures for all MCP tool calls.

use rmcp::schemars;

/// Request parameters for single-page scraping.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ScrapeParams {
    /// URL to scrape
    pub url: String,
    /// Output format: "markdown" (default) or "json"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Force browser rendering instead of HTTP fetch
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_browser: Option<bool>,
}

/// Request parameters for multi-page crawling.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CrawlParams {
    /// Starting URL for the crawl
    pub url: String,
    /// Maximum link depth from the start URL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<usize>,
    /// Maximum number of pages to crawl
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_pages: Option<usize>,
    /// Output format: "markdown" (default) or "json"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Whether to restrict crawling to the same domain
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stay_on_domain: Option<bool>,
}

/// Request parameters for site mapping (URL discovery).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MapParams {
    /// URL of the website to map
    pub url: String,
    /// Maximum number of URLs to return
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Case-insensitive substring filter for discovered URLs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Whether to respect robots.txt directives
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub respect_robots_txt: Option<bool>,
}

/// Request parameters for batch scraping multiple URLs.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BatchScrapeParams {
    /// List of URLs to scrape
    pub urls: Vec<String>,
    /// Output format: "markdown" (default) or "json"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Maximum number of concurrent requests
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<usize>,
}

/// Request parameters for taking a screenshot of a URL.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ScreenshotParams {
    /// URL to capture
    pub url: String,
    /// Whether to capture the full page or just the viewport
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_page: Option<bool>,
}

/// Request parameters for downloading a document from a URL.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DownloadParams {
    /// URL to download
    pub url: String,
    /// Maximum document size in bytes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_size: Option<usize>,
}

/// Request parameters for browser interaction on a page.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct InteractParams {
    /// URL to navigate to before executing actions
    pub url: String,
    /// Sequence of browser actions to execute (click, type, scroll, etc.)
    pub actions: Vec<serde_json::Value>,
}

/// Request parameters for AI-driven research.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ResearchParams {
    /// Research query or question
    pub query: String,
    /// Maximum crawl depth per seed URL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<usize>,
    /// Maximum total pages to visit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_pages: Option<usize>,
    /// Optional seed URLs to start research from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed_urls: Option<Vec<String>>,
}

/// Request parameters for checking crawl job status.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CrawlStatusParams {
    /// Optional job ID to check status for
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

/// Empty parameters for tools that take no arguments.
///
/// This generates `{"type": "object", "properties": {}}` which is required by
/// the MCP specification, unlike `()` which generates `{"const": null}`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EmptyParams {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrape_params_defaults() {
        let json = r#"{"url": "https://example.com"}"#;
        let params: ScrapeParams = serde_json::from_str(json).unwrap();

        assert_eq!(params.url, "https://example.com");
        assert_eq!(params.format, None);
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
    fn test_map_params_defaults() {
        let json = r#"{"url": "https://example.com"}"#;
        let params: MapParams = serde_json::from_str(json).unwrap();

        assert_eq!(params.url, "https://example.com");
        assert_eq!(params.limit, None);
        assert_eq!(params.search, None);
        assert_eq!(params.respect_robots_txt, None);
    }

    #[test]
    fn test_batch_scrape_params_defaults() {
        let json = r#"{"urls": ["https://a.com", "https://b.com"]}"#;
        let params: BatchScrapeParams = serde_json::from_str(json).unwrap();

        assert_eq!(params.urls.len(), 2);
        assert_eq!(params.format, None);
        assert_eq!(params.concurrency, None);
    }

    #[test]
    fn test_interact_params() {
        let json = r##"{"url": "https://example.com", "actions": [{"type": "click", "selector": "#btn"}]}"##;
        let params: InteractParams = serde_json::from_str(json).unwrap();

        assert_eq!(params.url, "https://example.com");
        assert_eq!(params.actions.len(), 1);
    }

    #[test]
    fn test_research_params_defaults() {
        let json = r#"{"query": "rust async patterns"}"#;
        let params: ResearchParams = serde_json::from_str(json).unwrap();

        assert_eq!(params.query, "rust async patterns");
        assert_eq!(params.max_depth, None);
        assert_eq!(params.seed_urls, None);
    }
}
