use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::AssetCategory;

/// Metadata about an LLM extraction pass.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExtractionMeta {
    /// Estimated cost of the LLM call in USD.
    pub cost: Option<f64>,
    /// Number of prompt (input) tokens consumed.
    pub prompt_tokens: Option<u64>,
    /// Number of completion (output) tokens generated.
    pub completion_tokens: Option<u64>,
    /// The model identifier used for extraction.
    pub model: Option<String>,
    /// Number of content chunks sent to the LLM.
    pub chunks_processed: usize,
}

/// When to use the headless browser fallback.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BrowserMode {
    /// Automatically detect when JS rendering is needed and fall back to browser.
    #[default]
    Auto,
    /// Always use the browser for every request.
    Always,
    /// Never use the browser fallback.
    Never,
}

/// Wait strategy for browser page rendering.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BrowserWait {
    /// Wait until network activity is idle.
    #[default]
    NetworkIdle,
    /// Wait for a specific CSS selector to appear in the DOM.
    Selector,
    /// Wait for a fixed duration after navigation.
    Fixed,
}

pub(crate) mod duration_ms {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S: Serializer>(d: &Duration, s: S) -> Result<S::Ok, S::Error> {
        d.as_millis().serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
        let ms = u64::deserialize(d)?;
        Ok(Duration::from_millis(ms))
    }
}

pub(crate) mod option_duration_ms {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S: Serializer>(d: &Option<Duration>, s: S) -> Result<S::Ok, S::Error> {
        d.map(|d| d.as_millis() as u64).serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Duration>, D::Error> {
        let ms: Option<u64> = Option::deserialize(d)?;
        Ok(ms.map(Duration::from_millis))
    }
}

/// Proxy configuration for HTTP requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProxyConfig {
    /// Proxy URL (e.g. "http://proxy:8080", "socks5://proxy:1080").
    pub url: String,
    /// Optional username for proxy authentication.
    pub username: Option<String>,
    /// Optional password for proxy authentication.
    pub password: Option<String>,
}

/// Authentication configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, tag = "type")]
pub enum AuthConfig {
    /// HTTP Basic authentication.
    #[serde(rename = "basic")]
    Basic { username: String, password: String },
    /// Bearer token authentication.
    #[serde(rename = "bearer")]
    Bearer { token: String },
    /// Custom authentication header.
    #[serde(rename = "header")]
    Header { name: String, value: String },
}

/// Browser fallback configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct BrowserConfig {
    /// When to use the headless browser fallback.
    pub mode: BrowserMode,
    /// CDP WebSocket endpoint for connecting to an external browser instance.
    pub endpoint: Option<String>,
    /// Timeout for browser page load and rendering (in milliseconds when serialized).
    #[serde(with = "duration_ms")]
    pub timeout: Duration,
    /// Wait strategy after browser navigation.
    pub wait: BrowserWait,
    /// CSS selector to wait for when `wait` is `Selector`.
    pub wait_selector: Option<String>,
    /// Extra time to wait after the wait condition is met.
    #[serde(default, with = "option_duration_ms")]
    pub extra_wait: Option<Duration>,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            mode: BrowserMode::Auto,
            endpoint: None,
            timeout: Duration::from_secs(30),
            wait: BrowserWait::default(),
            wait_selector: None,
            extra_wait: None,
        }
    }
}

/// Configuration for crawl, scrape, and map operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct CrawlConfig {
    /// Maximum crawl depth (number of link hops from the start URL).
    pub max_depth: Option<usize>,
    /// Maximum number of pages to crawl.
    pub max_pages: Option<usize>,
    /// Maximum number of concurrent requests.
    pub max_concurrent: Option<usize>,
    /// Whether to respect robots.txt directives.
    pub respect_robots_txt: bool,
    /// Custom user-agent string.
    pub user_agent: Option<String>,
    /// Whether to restrict crawling to the same domain.
    pub stay_on_domain: bool,
    /// Whether to allow subdomains when `stay_on_domain` is true.
    pub allow_subdomains: bool,
    /// Regex patterns for paths to include during crawling.
    #[serde(default)]
    pub include_paths: Vec<String>,
    /// Regex patterns for paths to exclude during crawling.
    #[serde(default)]
    pub exclude_paths: Vec<String>,
    /// Custom HTTP headers to send with each request.
    #[serde(default)]
    pub custom_headers: HashMap<String, String>,
    /// Timeout for individual HTTP requests (in milliseconds when serialized).
    #[serde(with = "duration_ms")]
    pub request_timeout: Duration,
    /// Maximum number of redirects to follow.
    pub max_redirects: usize,
    /// Number of retry attempts for failed requests.
    pub retry_count: usize,
    /// HTTP status codes that should trigger a retry.
    #[serde(default)]
    pub retry_codes: Vec<u16>,
    /// Whether to enable cookie handling.
    pub cookies_enabled: bool,
    /// Authentication configuration.
    pub auth: Option<AuthConfig>,
    /// Maximum response body size in bytes.
    pub max_body_size: Option<usize>,
    /// Whether to extract only the main content from HTML pages.
    pub main_content_only: bool,
    /// CSS selectors for tags to remove from HTML before processing.
    #[serde(default)]
    pub remove_tags: Vec<String>,
    /// Maximum number of URLs to return from a map operation.
    pub map_limit: Option<usize>,
    /// Search filter for map results (case-insensitive substring match on URLs).
    pub map_search: Option<String>,
    /// Whether to download assets (CSS, JS, images, etc.) from the page.
    pub download_assets: bool,
    /// Filter for asset categories to download.
    #[serde(default)]
    pub asset_types: Vec<AssetCategory>,
    /// Maximum size in bytes for individual asset downloads.
    pub max_asset_size: Option<usize>,
    /// Browser configuration.
    #[serde(default)]
    pub browser: BrowserConfig,
    /// Proxy configuration for HTTP requests.
    pub proxy: Option<ProxyConfig>,
    /// List of user-agent strings for rotation. If non-empty, overrides `user_agent`.
    #[serde(default)]
    pub user_agents: Vec<String>,
    /// Whether to capture a screenshot when using the browser.
    pub capture_screenshot: bool,
    /// Whether to download non-HTML documents (PDF, DOCX, images, code, etc.) instead of skipping them.
    pub download_documents: bool,
    /// Maximum size in bytes for document downloads. Defaults to 50 MB.
    pub document_max_size: Option<usize>,
    /// Allowlist of MIME types to download. If empty, uses built-in defaults.
    #[serde(default)]
    pub document_mime_types: Vec<String>,
    /// Path to write WARC output. If `None`, WARC output is disabled.
    pub warc_output: Option<PathBuf>,
    /// Named browser profile for persistent sessions (cookies, localStorage).
    pub browser_profile: Option<String>,
    /// Whether to save changes back to the browser profile on exit.
    pub save_browser_profile: bool,
    /// Shared browser pool for reusing Chrome across requests (not serializable).
    #[cfg(feature = "browser")]
    #[serde(skip)]
    pub browser_pool: Option<std::sync::Arc<crate::browser_pool::BrowserPool>>,
}

impl Default for CrawlConfig {
    fn default() -> Self {
        Self {
            max_depth: None,
            max_pages: None,
            max_concurrent: None,
            respect_robots_txt: false,
            user_agent: None,
            stay_on_domain: false,
            allow_subdomains: false,
            include_paths: Vec::new(),
            exclude_paths: Vec::new(),
            custom_headers: HashMap::new(),
            request_timeout: Duration::from_secs(30),
            max_redirects: 10,
            retry_count: 0,
            retry_codes: Vec::new(),
            cookies_enabled: false,
            auth: None,
            max_body_size: None,
            main_content_only: false,
            remove_tags: Vec::new(),
            map_limit: None,
            map_search: None,
            download_assets: false,
            asset_types: Vec::new(),
            max_asset_size: None,
            browser: BrowserConfig::default(),
            proxy: None,
            user_agents: Vec::new(),
            capture_screenshot: false,
            download_documents: true,
            document_max_size: Some(50 * 1024 * 1024), // 50 MB
            document_mime_types: Vec::new(),
            warc_output: None,
            browser_profile: None,
            save_browser_profile: false,
            #[cfg(feature = "browser")]
            browser_pool: None,
        }
    }
}

impl CrawlConfig {
    /// Validate the configuration, returning an error if any values are invalid.
    pub fn validate(&self) -> Result<(), crate::error::CrawlError> {
        use crate::error::CrawlError;

        if let Some(0) = self.max_concurrent {
            return Err(CrawlError::InvalidConfig(
                "max_concurrent must be > 0".into(),
            ));
        }
        if self.browser.wait == BrowserWait::Selector && self.browser.wait_selector.is_none() {
            return Err(CrawlError::InvalidConfig(
                "browser.wait_selector required when browser.wait is Selector".into(),
            ));
        }
        if let Some(max_pages) = self.max_pages
            && max_pages == 0
        {
            return Err(CrawlError::InvalidConfig("max_pages must be > 0".into()));
        }
        if self.max_redirects > 100 {
            return Err(CrawlError::InvalidConfig(
                "max_redirects must be <= 100".into(),
            ));
        }
        for pattern in &self.include_paths {
            regex::Regex::new(pattern).map_err(|e| {
                CrawlError::InvalidConfig(format!("invalid include_path regex '{pattern}': {e}"))
            })?;
        }
        for pattern in &self.exclude_paths {
            regex::Regex::new(pattern).map_err(|e| {
                CrawlError::InvalidConfig(format!("invalid exclude_path regex '{pattern}': {e}"))
            })?;
        }
        for &code in &self.retry_codes {
            if !(100..=599).contains(&code) {
                return Err(CrawlError::InvalidConfig(format!(
                    "invalid retry code: {code}"
                )));
            }
        }
        if self.request_timeout.is_zero() {
            return Err(CrawlError::InvalidConfig(
                "request_timeout must be > 0".into(),
            ));
        }
        Ok(())
    }
}
