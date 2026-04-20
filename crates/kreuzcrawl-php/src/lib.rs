#![allow(dead_code, unused_imports, unused_variables)]
#![allow(
    clippy::too_many_arguments,
    clippy::let_unit_value,
    clippy::needless_borrow,
    clippy::map_identity,
    clippy::just_underscores_and_digits
)]

use ext_php_rs::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

static WORKER_RUNTIME: std::sync::LazyLock<tokio::runtime::Runtime> = std::sync::LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime")
});

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\ExtractionMeta")]
pub struct ExtractionMeta {
    /// Estimated cost of the LLM call in USD.
    #[php(prop, name = "cost")]
    pub cost: Option<f64>,
    /// Number of prompt (input) tokens consumed.
    #[php(prop, name = "prompt_tokens")]
    pub prompt_tokens: Option<i64>,
    /// Number of completion (output) tokens generated.
    #[php(prop, name = "completion_tokens")]
    pub completion_tokens: Option<i64>,
    /// The model identifier used for extraction.
    #[php(prop, name = "model")]
    pub model: Option<String>,
    /// Number of content chunks sent to the LLM.
    #[php(prop, name = "chunks_processed")]
    pub chunks_processed: i64,
}

#[php_impl]
impl ExtractionMeta {
    pub fn __construct(
        cost: Option<f64>,
        prompt_tokens: Option<i64>,
        completion_tokens: Option<i64>,
        model: Option<String>,
        chunks_processed: Option<i64>,
    ) -> Self {
        Self {
            cost,
            prompt_tokens,
            completion_tokens,
            model,
            chunks_processed: chunks_processed.unwrap_or_default(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\ProxyConfig")]
pub struct ProxyConfig {
    /// Proxy URL (e.g. "http://proxy:8080", "socks5://proxy:1080").
    #[php(prop, name = "url")]
    pub url: String,
    /// Optional username for proxy authentication.
    #[php(prop, name = "username")]
    pub username: Option<String>,
    /// Optional password for proxy authentication.
    #[php(prop, name = "password")]
    pub password: Option<String>,
}

#[php_impl]
impl ProxyConfig {
    pub fn __construct(url: Option<String>, username: Option<String>, password: Option<String>) -> Self {
        Self {
            url: url.unwrap_or_default(),
            username,
            password,
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\BrowserConfig")]
pub struct BrowserConfig {
    /// When to use the headless browser fallback.
    #[php(prop, name = "mode")]
    pub mode: String,
    /// CDP WebSocket endpoint for connecting to an external browser instance.
    #[php(prop, name = "endpoint")]
    pub endpoint: Option<String>,
    /// Timeout for browser page load and rendering (in milliseconds when serialized).
    #[php(prop, name = "timeout")]
    pub timeout: Option<i64>,
    /// Wait strategy after browser navigation.
    #[php(prop, name = "wait")]
    pub wait: String,
    /// CSS selector to wait for when `wait` is `Selector`.
    #[php(prop, name = "wait_selector")]
    pub wait_selector: Option<String>,
    /// Extra time to wait after the wait condition is met.
    #[php(prop, name = "extra_wait")]
    pub extra_wait: Option<i64>,
}

#[php_impl]
impl BrowserConfig {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> BrowserConfig {
        kreuzcrawl::BrowserConfig::default().into()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\CrawlConfig")]
#[allow(clippy::similar_names)]
pub struct CrawlConfig {
    /// Maximum crawl depth (number of link hops from the start URL).
    #[php(prop, name = "max_depth")]
    pub max_depth: Option<i64>,
    /// Maximum number of pages to crawl.
    #[php(prop, name = "max_pages")]
    pub max_pages: Option<i64>,
    /// Maximum number of concurrent requests.
    #[php(prop, name = "max_concurrent")]
    pub max_concurrent: Option<i64>,
    /// Whether to respect robots.txt directives.
    #[php(prop, name = "respect_robots_txt")]
    pub respect_robots_txt: bool,
    /// Custom user-agent string.
    #[php(prop, name = "user_agent")]
    pub user_agent: Option<String>,
    /// Whether to restrict crawling to the same domain.
    #[php(prop, name = "stay_on_domain")]
    pub stay_on_domain: bool,
    /// Whether to allow subdomains when `stay_on_domain` is true.
    #[php(prop, name = "allow_subdomains")]
    pub allow_subdomains: bool,
    /// Regex patterns for paths to include during crawling.
    #[php(prop, name = "include_paths")]
    pub include_paths: Vec<String>,
    /// Regex patterns for paths to exclude during crawling.
    #[php(prop, name = "exclude_paths")]
    pub exclude_paths: Vec<String>,
    /// Custom HTTP headers to send with each request.
    pub custom_headers: HashMap<String, String>,
    /// Timeout for individual HTTP requests (in milliseconds when serialized).
    #[php(prop, name = "request_timeout")]
    pub request_timeout: Option<i64>,
    /// Maximum number of redirects to follow.
    #[php(prop, name = "max_redirects")]
    pub max_redirects: i64,
    /// Number of retry attempts for failed requests.
    #[php(prop, name = "retry_count")]
    pub retry_count: i64,
    /// HTTP status codes that should trigger a retry.
    #[php(prop, name = "retry_codes")]
    pub retry_codes: Vec<u16>,
    /// Whether to enable cookie handling.
    #[php(prop, name = "cookies_enabled")]
    pub cookies_enabled: bool,
    /// Authentication configuration.
    #[php(prop, name = "auth")]
    pub auth: Option<String>,
    /// Maximum response body size in bytes.
    #[php(prop, name = "max_body_size")]
    pub max_body_size: Option<i64>,
    /// Whether to extract only the main content from HTML pages.
    #[php(prop, name = "main_content_only")]
    pub main_content_only: bool,
    /// CSS selectors for tags to remove from HTML before processing.
    #[php(prop, name = "remove_tags")]
    pub remove_tags: Vec<String>,
    /// Maximum number of URLs to return from a map operation.
    #[php(prop, name = "map_limit")]
    pub map_limit: Option<i64>,
    /// Search filter for map results (case-insensitive substring match on URLs).
    #[php(prop, name = "map_search")]
    pub map_search: Option<String>,
    /// Whether to download assets (CSS, JS, images, etc.) from the page.
    #[php(prop, name = "download_assets")]
    pub download_assets: bool,
    /// Filter for asset categories to download.
    #[php(prop, name = "asset_types")]
    pub asset_types: Vec<String>,
    /// Maximum size in bytes for individual asset downloads.
    #[php(prop, name = "max_asset_size")]
    pub max_asset_size: Option<i64>,
    /// Browser configuration.
    pub browser: BrowserConfig,
    /// Proxy configuration for HTTP requests.
    pub proxy: Option<ProxyConfig>,
    /// List of user-agent strings for rotation. If non-empty, overrides `user_agent`.
    #[php(prop, name = "user_agents")]
    pub user_agents: Vec<String>,
    /// Whether to capture a screenshot when using the browser.
    #[php(prop, name = "capture_screenshot")]
    pub capture_screenshot: bool,
    /// Whether to download non-HTML documents (PDF, DOCX, images, code, etc.) instead of skipping them.
    #[php(prop, name = "download_documents")]
    pub download_documents: bool,
    /// Maximum size in bytes for document downloads. Defaults to 50 MB.
    #[php(prop, name = "document_max_size")]
    pub document_max_size: Option<i64>,
    /// Allowlist of MIME types to download. If empty, uses built-in defaults.
    #[php(prop, name = "document_mime_types")]
    pub document_mime_types: Vec<String>,
    /// Path to write WARC output. If `None`, WARC output is disabled.
    #[php(prop, name = "warc_output")]
    pub warc_output: Option<String>,
    /// Named browser profile for persistent sessions (cookies, localStorage).
    #[php(prop, name = "browser_profile")]
    pub browser_profile: Option<String>,
    /// Whether to save changes back to the browser profile on exit.
    #[php(prop, name = "save_browser_profile")]
    pub save_browser_profile: bool,
}

#[php_impl]
impl CrawlConfig {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_custom_headers(&self) -> HashMap<String, String> {
        self.custom_headers.clone()
    }

    #[php(getter)]
    pub fn get_browser(&self) -> BrowserConfig {
        self.browser.clone()
    }

    #[php(getter)]
    pub fn get_proxy(&self) -> Option<ProxyConfig> {
        self.proxy.clone()
    }

    pub fn validate(&self) -> PhpResult<()> {
        #[allow(clippy::needless_update)]
        let core_self = kreuzcrawl::CrawlConfig {
            max_depth: self.max_depth.map(|v| v as usize),
            max_pages: self.max_pages.map(|v| v as usize),
            max_concurrent: self.max_concurrent.map(|v| v as usize),
            respect_robots_txt: self.respect_robots_txt,
            user_agent: self.user_agent.clone(),
            stay_on_domain: self.stay_on_domain,
            allow_subdomains: self.allow_subdomains,
            include_paths: self.include_paths.clone(),
            exclude_paths: self.exclude_paths.clone(),
            custom_headers: self.custom_headers.clone().into_iter().collect(),
            request_timeout: self
                .request_timeout
                .map(|v| std::time::Duration::from_millis(v as u64))
                .unwrap_or_default(),
            max_redirects: self.max_redirects as usize,
            retry_count: self.retry_count as usize,
            retry_codes: self.retry_codes.clone(),
            cookies_enabled: self.cookies_enabled,
            auth: self.auth.as_deref().map(|s| match s {
                "Basic" => kreuzcrawl::AuthConfig::Basic {
                    username: Default::default(),
                    password: Default::default(),
                },
                "Bearer" => kreuzcrawl::AuthConfig::Bearer {
                    token: Default::default(),
                },
                "Header" => kreuzcrawl::AuthConfig::Header {
                    name: Default::default(),
                    value: Default::default(),
                },
                _ => kreuzcrawl::AuthConfig::Basic {
                    username: Default::default(),
                    password: Default::default(),
                },
            }),
            max_body_size: self.max_body_size.map(|v| v as usize),
            main_content_only: self.main_content_only,
            remove_tags: self.remove_tags.clone(),
            map_limit: self.map_limit.map(|v| v as usize),
            map_search: self.map_search.clone(),
            download_assets: self.download_assets,
            asset_types: self
                .asset_types
                .clone()
                .into_iter()
                .map(|s| match s.as_str() {
                    "Document" => kreuzcrawl::AssetCategory::Document,
                    "Image" => kreuzcrawl::AssetCategory::Image,
                    "Audio" => kreuzcrawl::AssetCategory::Audio,
                    "Video" => kreuzcrawl::AssetCategory::Video,
                    "Font" => kreuzcrawl::AssetCategory::Font,
                    "Stylesheet" => kreuzcrawl::AssetCategory::Stylesheet,
                    "Script" => kreuzcrawl::AssetCategory::Script,
                    "Archive" => kreuzcrawl::AssetCategory::Archive,
                    "Data" => kreuzcrawl::AssetCategory::Data,
                    "Other" => kreuzcrawl::AssetCategory::Other,
                    _ => kreuzcrawl::AssetCategory::Document,
                })
                .collect(),
            max_asset_size: self.max_asset_size.map(|v| v as usize),
            browser: self.browser.clone().into(),
            proxy: self.proxy.clone().map(Into::into),
            user_agents: self.user_agents.clone(),
            capture_screenshot: self.capture_screenshot,
            download_documents: self.download_documents,
            document_max_size: self.document_max_size.map(|v| v as usize),
            document_mime_types: self.document_mime_types.clone(),
            warc_output: self.warc_output.clone().map(Into::into),
            browser_profile: self.browser_profile.clone(),
            save_browser_profile: self.save_browser_profile,
            ..Default::default()
        };
        let result = core_self
            .validate()
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(result)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> CrawlConfig {
        kreuzcrawl::CrawlConfig::default().into()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\DownloadedDocument")]
pub struct DownloadedDocument {
    /// The URL the document was fetched from.
    #[php(prop, name = "url")]
    pub url: String,
    /// The MIME type from the Content-Type header.
    #[php(prop, name = "mime_type")]
    pub mime_type: String,
    /// Raw document bytes. Skipped during JSON serialization.
    pub content: Vec<u8>,
    /// Size of the document in bytes.
    #[php(prop, name = "size")]
    pub size: i64,
    /// Filename extracted from Content-Disposition or URL path.
    #[php(prop, name = "filename")]
    pub filename: Option<String>,
    /// SHA-256 hex digest of the content.
    #[php(prop, name = "content_hash")]
    pub content_hash: String,
    /// Selected response headers.
    pub headers: HashMap<String, String>,
}

#[php_impl]
impl DownloadedDocument {
    pub fn __construct(
        url: Option<String>,
        mime_type: Option<String>,
        content: Option<Vec<u8>>,
        size: Option<i64>,
        filename: Option<String>,
        content_hash: Option<String>,
        headers: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            url: url.unwrap_or_default(),
            mime_type: mime_type.unwrap_or_default(),
            content: content.unwrap_or_default(),
            size: size.unwrap_or_default(),
            filename,
            content_hash: content_hash.unwrap_or_default(),
            headers: headers.unwrap_or_default(),
        }
    }

    #[php(getter)]
    pub fn get_content(&self) -> Vec<u8> {
        self.content.clone()
    }

    #[php(getter)]
    pub fn get_headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\InteractionResult")]
pub struct InteractionResult {
    /// Results from each executed action.
    pub action_results: Vec<ActionResult>,
    /// Final page HTML after all actions completed.
    #[php(prop, name = "final_html")]
    pub final_html: String,
    /// Final page URL (may have changed due to navigation).
    #[php(prop, name = "final_url")]
    pub final_url: String,
    /// Screenshot taken after all actions, if requested.
    pub screenshot: Option<Vec<u8>>,
}

#[php_impl]
impl InteractionResult {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_action_results(&self) -> Vec<ActionResult> {
        self.action_results.clone()
    }

    #[php(getter)]
    pub fn get_screenshot(&self) -> Option<Vec<u8>> {
        self.screenshot.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\ActionResult")]
pub struct ActionResult {
    /// Zero-based index of the action in the sequence.
    #[php(prop, name = "action_index")]
    pub action_index: i64,
    /// The type of action that was executed.
    #[php(prop, name = "action_type")]
    pub action_type: String,
    /// Whether the action completed successfully.
    #[php(prop, name = "success")]
    pub success: bool,
    /// Action-specific return data (screenshot bytes, JS return value, scraped HTML).
    pub data: Option<String>,
    /// Error message if the action failed.
    #[php(prop, name = "error")]
    pub error: Option<String>,
}

#[php_impl]
impl ActionResult {
    pub fn __construct(
        action_index: Option<i64>,
        action_type: Option<String>,
        success: Option<bool>,
        data: Option<String>,
        error: Option<String>,
    ) -> Self {
        Self {
            action_index: action_index.unwrap_or_default(),
            action_type: action_type.unwrap_or_default(),
            success: success.unwrap_or_default(),
            data,
            error,
        }
    }

    #[php(getter)]
    pub fn get_data(&self) -> Option<String> {
        self.data.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\ScrapeResult")]
pub struct ScrapeResult {
    /// The HTTP status code of the response.
    #[php(prop, name = "status_code")]
    pub status_code: u16,
    /// The Content-Type header value.
    #[php(prop, name = "content_type")]
    pub content_type: String,
    /// The HTML body of the response.
    #[php(prop, name = "html")]
    pub html: String,
    /// The size of the response body in bytes.
    #[php(prop, name = "body_size")]
    pub body_size: i64,
    /// Extracted metadata from the page.
    pub metadata: PageMetadata,
    /// Links found on the page.
    pub links: Vec<LinkInfo>,
    /// Images found on the page.
    pub images: Vec<ImageInfo>,
    /// Feed links found on the page.
    pub feeds: Vec<FeedInfo>,
    /// JSON-LD entries found on the page.
    pub json_ld: Vec<JsonLdEntry>,
    /// Whether the URL is allowed by robots.txt.
    #[php(prop, name = "is_allowed")]
    pub is_allowed: bool,
    /// The crawl delay from robots.txt, in seconds.
    #[php(prop, name = "crawl_delay")]
    pub crawl_delay: Option<i64>,
    /// Whether a noindex directive was detected.
    #[php(prop, name = "noindex_detected")]
    pub noindex_detected: bool,
    /// Whether a nofollow directive was detected.
    #[php(prop, name = "nofollow_detected")]
    pub nofollow_detected: bool,
    /// The X-Robots-Tag header value, if present.
    #[php(prop, name = "x_robots_tag")]
    pub x_robots_tag: Option<String>,
    /// Whether the content is a PDF.
    #[php(prop, name = "is_pdf")]
    pub is_pdf: bool,
    /// Whether the page was skipped (binary or PDF content).
    #[php(prop, name = "was_skipped")]
    pub was_skipped: bool,
    /// The detected character set encoding.
    #[php(prop, name = "detected_charset")]
    pub detected_charset: Option<String>,
    /// Whether main_content_only was active during extraction.
    #[php(prop, name = "main_content_only")]
    pub main_content_only: bool,
    /// Whether an authentication header was sent with the request.
    #[php(prop, name = "auth_header_sent")]
    pub auth_header_sent: bool,
    /// Response metadata extracted from HTTP headers.
    pub response_meta: Option<ResponseMeta>,
    /// Downloaded assets from the page.
    pub assets: Vec<DownloadedAsset>,
    /// Whether the page content suggests JavaScript rendering is needed.
    #[php(prop, name = "js_render_hint")]
    pub js_render_hint: bool,
    /// Whether the browser fallback was used to fetch this page.
    #[php(prop, name = "browser_used")]
    pub browser_used: bool,
    /// Markdown conversion of the page content.
    pub markdown: Option<MarkdownResult>,
    /// Structured data extracted by LLM. Populated when using LlmExtractor.
    pub extracted_data: Option<String>,
    /// Metadata about the LLM extraction pass (cost, tokens, model).
    pub extraction_meta: Option<ExtractionMeta>,
    /// Screenshot of the page as PNG bytes. Populated when browser is used and capture_screenshot is enabled.
    pub screenshot: Option<Vec<u8>>,
    /// Downloaded non-HTML document (PDF, DOCX, image, code, etc.).
    pub downloaded_document: Option<DownloadedDocument>,
}

#[php_impl]
impl ScrapeResult {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_metadata(&self) -> PageMetadata {
        self.metadata.clone()
    }

    #[php(getter)]
    pub fn get_links(&self) -> Vec<LinkInfo> {
        self.links.clone()
    }

    #[php(getter)]
    pub fn get_images(&self) -> Vec<ImageInfo> {
        self.images.clone()
    }

    #[php(getter)]
    pub fn get_feeds(&self) -> Vec<FeedInfo> {
        self.feeds.clone()
    }

    #[php(getter)]
    pub fn get_json_ld(&self) -> Vec<JsonLdEntry> {
        self.json_ld.clone()
    }

    #[php(getter)]
    pub fn get_response_meta(&self) -> Option<ResponseMeta> {
        self.response_meta.clone()
    }

    #[php(getter)]
    pub fn get_assets(&self) -> Vec<DownloadedAsset> {
        self.assets.clone()
    }

    #[php(getter)]
    pub fn get_markdown(&self) -> Option<MarkdownResult> {
        self.markdown.clone()
    }

    #[php(getter)]
    pub fn get_extracted_data(&self) -> Option<String> {
        self.extracted_data.clone()
    }

    #[php(getter)]
    pub fn get_extraction_meta(&self) -> Option<ExtractionMeta> {
        self.extraction_meta.clone()
    }

    #[php(getter)]
    pub fn get_screenshot(&self) -> Option<Vec<u8>> {
        self.screenshot.clone()
    }

    #[php(getter)]
    pub fn get_downloaded_document(&self) -> Option<DownloadedDocument> {
        self.downloaded_document.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\CrawlPageResult")]
pub struct CrawlPageResult {
    /// The original URL of the page.
    #[php(prop, name = "url")]
    pub url: String,
    /// The normalized URL of the page.
    #[php(prop, name = "normalized_url")]
    pub normalized_url: String,
    /// The HTTP status code of the response.
    #[php(prop, name = "status_code")]
    pub status_code: u16,
    /// The Content-Type header value.
    #[php(prop, name = "content_type")]
    pub content_type: String,
    /// The HTML body of the response.
    #[php(prop, name = "html")]
    pub html: String,
    /// The size of the response body in bytes.
    #[php(prop, name = "body_size")]
    pub body_size: i64,
    /// Extracted metadata from the page.
    pub metadata: PageMetadata,
    /// Links found on the page.
    pub links: Vec<LinkInfo>,
    /// Images found on the page.
    pub images: Vec<ImageInfo>,
    /// Feed links found on the page.
    pub feeds: Vec<FeedInfo>,
    /// JSON-LD entries found on the page.
    pub json_ld: Vec<JsonLdEntry>,
    /// The depth of this page from the start URL.
    #[php(prop, name = "depth")]
    pub depth: i64,
    /// Whether this page is on the same domain as the start URL.
    #[php(prop, name = "stayed_on_domain")]
    pub stayed_on_domain: bool,
    /// Whether this page was skipped (binary or PDF content).
    #[php(prop, name = "was_skipped")]
    pub was_skipped: bool,
    /// Whether the content is a PDF.
    #[php(prop, name = "is_pdf")]
    pub is_pdf: bool,
    /// The detected character set encoding.
    #[php(prop, name = "detected_charset")]
    pub detected_charset: Option<String>,
    /// Markdown conversion of the page content.
    pub markdown: Option<MarkdownResult>,
    /// Structured data extracted by LLM. Populated when using LlmExtractor.
    pub extracted_data: Option<String>,
    /// Metadata about the LLM extraction pass (cost, tokens, model).
    pub extraction_meta: Option<ExtractionMeta>,
    /// Downloaded non-HTML document (PDF, DOCX, image, code, etc.).
    pub downloaded_document: Option<DownloadedDocument>,
}

#[php_impl]
impl CrawlPageResult {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_metadata(&self) -> PageMetadata {
        self.metadata.clone()
    }

    #[php(getter)]
    pub fn get_links(&self) -> Vec<LinkInfo> {
        self.links.clone()
    }

    #[php(getter)]
    pub fn get_images(&self) -> Vec<ImageInfo> {
        self.images.clone()
    }

    #[php(getter)]
    pub fn get_feeds(&self) -> Vec<FeedInfo> {
        self.feeds.clone()
    }

    #[php(getter)]
    pub fn get_json_ld(&self) -> Vec<JsonLdEntry> {
        self.json_ld.clone()
    }

    #[php(getter)]
    pub fn get_markdown(&self) -> Option<MarkdownResult> {
        self.markdown.clone()
    }

    #[php(getter)]
    pub fn get_extracted_data(&self) -> Option<String> {
        self.extracted_data.clone()
    }

    #[php(getter)]
    pub fn get_extraction_meta(&self) -> Option<ExtractionMeta> {
        self.extraction_meta.clone()
    }

    #[php(getter)]
    pub fn get_downloaded_document(&self) -> Option<DownloadedDocument> {
        self.downloaded_document.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\CrawlResult")]
pub struct CrawlResult {
    /// The list of crawled pages.
    pub pages: Vec<CrawlPageResult>,
    /// The final URL after following redirects.
    #[php(prop, name = "final_url")]
    pub final_url: String,
    /// The number of redirects followed.
    #[php(prop, name = "redirect_count")]
    pub redirect_count: i64,
    /// Whether any page was skipped during crawling.
    #[php(prop, name = "was_skipped")]
    pub was_skipped: bool,
    /// An error message, if the crawl encountered an issue.
    #[php(prop, name = "error")]
    pub error: Option<String>,
    /// Cookies collected during the crawl.
    pub cookies: Vec<CookieInfo>,
    /// Normalized URLs encountered during crawling (for deduplication counting).
    #[php(prop, name = "normalized_urls")]
    pub normalized_urls: Vec<String>,
}

#[php_impl]
impl CrawlResult {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_pages(&self) -> Vec<CrawlPageResult> {
        self.pages.clone()
    }

    #[php(getter)]
    pub fn get_cookies(&self) -> Vec<CookieInfo> {
        self.cookies.clone()
    }

    pub fn unique_normalized_urls(&self) -> i64 {
        let core_self = kreuzcrawl::CrawlResult {
            pages: self.pages.clone().into_iter().map(Into::into).collect(),
            final_url: self.final_url.clone(),
            redirect_count: self.redirect_count as usize,
            was_skipped: self.was_skipped,
            error: self.error.clone(),
            cookies: self.cookies.clone().into_iter().map(Into::into).collect(),
            normalized_urls: self.normalized_urls.clone(),
        };
        core_self.unique_normalized_urls() as i64
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\SitemapUrl")]
pub struct SitemapUrl {
    /// The URL.
    #[php(prop, name = "url")]
    pub url: String,
    /// The last modification date, if present.
    #[php(prop, name = "lastmod")]
    pub lastmod: Option<String>,
    /// The change frequency, if present.
    #[php(prop, name = "changefreq")]
    pub changefreq: Option<String>,
    /// The priority, if present.
    #[php(prop, name = "priority")]
    pub priority: Option<String>,
}

#[php_impl]
impl SitemapUrl {
    pub fn __construct(
        url: Option<String>,
        lastmod: Option<String>,
        changefreq: Option<String>,
        priority: Option<String>,
    ) -> Self {
        Self {
            url: url.unwrap_or_default(),
            lastmod,
            changefreq,
            priority,
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\MapResult")]
pub struct MapResult {
    /// The list of discovered URLs.
    pub urls: Vec<SitemapUrl>,
}

#[php_impl]
impl MapResult {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_urls(&self) -> Vec<SitemapUrl> {
        self.urls.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\MarkdownResult")]
pub struct MarkdownResult {
    /// Converted markdown text.
    #[php(prop, name = "content")]
    pub content: String,
    /// Structured document tree with semantic nodes.
    pub document_structure: Option<String>,
    /// Extracted tables with structured cell data.
    pub tables: Vec<String>,
    /// Non-fatal processing warnings.
    #[php(prop, name = "warnings")]
    pub warnings: Vec<String>,
    /// Content with links replaced by numbered citations.
    pub citations: Option<CitationResult>,
    /// Content-filtered markdown optimized for LLM consumption.
    #[php(prop, name = "fit_content")]
    pub fit_content: Option<String>,
}

#[php_impl]
impl MarkdownResult {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_document_structure(&self) -> Option<String> {
        self.document_structure.clone()
    }

    #[php(getter)]
    pub fn get_tables(&self) -> Vec<String> {
        self.tables.clone()
    }

    #[php(getter)]
    pub fn get_citations(&self) -> Option<CitationResult> {
        self.citations.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\CachedPage")]
pub struct CachedPage {
    #[php(prop, name = "url")]
    pub url: String,
    #[php(prop, name = "status_code")]
    pub status_code: u16,
    #[php(prop, name = "content_type")]
    pub content_type: String,
    #[php(prop, name = "body")]
    pub body: String,
    #[php(prop, name = "etag")]
    pub etag: Option<String>,
    #[php(prop, name = "last_modified")]
    pub last_modified: Option<String>,
    #[php(prop, name = "cached_at")]
    pub cached_at: i64,
}

#[php_impl]
impl CachedPage {
    pub fn __construct(
        url: Option<String>,
        status_code: Option<u16>,
        content_type: Option<String>,
        body: Option<String>,
        etag: Option<String>,
        last_modified: Option<String>,
        cached_at: Option<i64>,
    ) -> Self {
        Self {
            url: url.unwrap_or_default(),
            status_code: status_code.unwrap_or_default(),
            content_type: content_type.unwrap_or_default(),
            body: body.unwrap_or_default(),
            etag,
            last_modified,
            cached_at: cached_at.unwrap_or_default(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\LinkInfo")]
#[allow(clippy::similar_names)]
pub struct LinkInfo {
    /// The resolved URL of the link.
    #[php(prop, name = "url")]
    pub url: String,
    /// The visible text of the link.
    #[php(prop, name = "text")]
    pub text: String,
    /// The classification of the link.
    #[php(prop, name = "link_type")]
    pub link_type: String,
    /// The `rel` attribute value, if present.
    #[php(prop, name = "rel")]
    pub rel: Option<String>,
    /// Whether the link has `rel="nofollow"`.
    #[php(prop, name = "nofollow")]
    pub nofollow: bool,
}

#[php_impl]
impl LinkInfo {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\ImageInfo")]
pub struct ImageInfo {
    /// The image URL.
    #[php(prop, name = "url")]
    pub url: String,
    /// The alt text, if present.
    #[php(prop, name = "alt")]
    pub alt: Option<String>,
    /// The width attribute, if present and parseable.
    #[php(prop, name = "width")]
    pub width: Option<u32>,
    /// The height attribute, if present and parseable.
    #[php(prop, name = "height")]
    pub height: Option<u32>,
    /// The source of the image reference.
    #[php(prop, name = "source")]
    pub source: String,
}

#[php_impl]
impl ImageInfo {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\FeedInfo")]
pub struct FeedInfo {
    /// The feed URL.
    #[php(prop, name = "url")]
    pub url: String,
    /// The feed title, if present.
    #[php(prop, name = "title")]
    pub title: Option<String>,
    /// The type of feed.
    #[php(prop, name = "feed_type")]
    pub feed_type: String,
}

#[php_impl]
impl FeedInfo {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\JsonLdEntry")]
pub struct JsonLdEntry {
    /// The `@type` value from the JSON-LD object.
    #[php(prop, name = "schema_type")]
    pub schema_type: String,
    /// The `name` value, if present.
    #[php(prop, name = "name")]
    pub name: Option<String>,
    /// The raw JSON-LD string.
    #[php(prop, name = "raw")]
    pub raw: String,
}

#[php_impl]
impl JsonLdEntry {
    pub fn __construct(schema_type: Option<String>, name: Option<String>, raw: Option<String>) -> Self {
        Self {
            schema_type: schema_type.unwrap_or_default(),
            name,
            raw: raw.unwrap_or_default(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\CookieInfo")]
pub struct CookieInfo {
    /// The cookie name.
    #[php(prop, name = "name")]
    pub name: String,
    /// The cookie value.
    #[php(prop, name = "value")]
    pub value: String,
    /// The cookie domain, if specified.
    #[php(prop, name = "domain")]
    pub domain: Option<String>,
    /// The cookie path, if specified.
    #[php(prop, name = "path")]
    pub path: Option<String>,
}

#[php_impl]
impl CookieInfo {
    pub fn __construct(
        name: Option<String>,
        value: Option<String>,
        domain: Option<String>,
        path: Option<String>,
    ) -> Self {
        Self {
            name: name.unwrap_or_default(),
            value: value.unwrap_or_default(),
            domain,
            path,
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\DownloadedAsset")]
pub struct DownloadedAsset {
    /// The original URL of the asset.
    #[php(prop, name = "url")]
    pub url: String,
    /// The SHA-256 content hash of the asset.
    #[php(prop, name = "content_hash")]
    pub content_hash: String,
    /// The MIME type from the Content-Type header.
    #[php(prop, name = "mime_type")]
    pub mime_type: Option<String>,
    /// The size of the asset in bytes.
    #[php(prop, name = "size")]
    pub size: i64,
    /// The category of the asset.
    #[php(prop, name = "asset_category")]
    pub asset_category: String,
    /// The HTML tag that referenced this asset (e.g., "link", "script", "img").
    #[php(prop, name = "html_tag")]
    pub html_tag: Option<String>,
}

#[php_impl]
impl DownloadedAsset {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\ArticleMetadata")]
pub struct ArticleMetadata {
    /// The article publication time.
    #[php(prop, name = "published_time")]
    pub published_time: Option<String>,
    /// The article modification time.
    #[php(prop, name = "modified_time")]
    pub modified_time: Option<String>,
    /// The article author.
    #[php(prop, name = "author")]
    pub author: Option<String>,
    /// The article section.
    #[php(prop, name = "section")]
    pub section: Option<String>,
    /// The article tags.
    #[php(prop, name = "tags")]
    pub tags: Vec<String>,
}

#[php_impl]
impl ArticleMetadata {
    pub fn __construct(
        published_time: Option<String>,
        modified_time: Option<String>,
        author: Option<String>,
        section: Option<String>,
        tags: Option<Vec<String>>,
    ) -> Self {
        Self {
            published_time,
            modified_time,
            author,
            section,
            tags: tags.unwrap_or_default(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\HreflangEntry")]
pub struct HreflangEntry {
    /// The language code (e.g., "en", "fr", "x-default").
    #[php(prop, name = "lang")]
    pub lang: String,
    /// The URL for this language variant.
    #[php(prop, name = "url")]
    pub url: String,
}

#[php_impl]
impl HreflangEntry {
    pub fn __construct(lang: Option<String>, url: Option<String>) -> Self {
        Self {
            lang: lang.unwrap_or_default(),
            url: url.unwrap_or_default(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\FaviconInfo")]
#[allow(clippy::similar_names)]
pub struct FaviconInfo {
    /// The icon URL.
    #[php(prop, name = "url")]
    pub url: String,
    /// The `rel` attribute (e.g., "icon", "apple-touch-icon").
    #[php(prop, name = "rel")]
    pub rel: String,
    /// The `sizes` attribute, if present.
    #[php(prop, name = "sizes")]
    pub sizes: Option<String>,
    /// The MIME type, if present.
    #[php(prop, name = "mime_type")]
    pub mime_type: Option<String>,
}

#[php_impl]
impl FaviconInfo {
    pub fn __construct(
        url: Option<String>,
        rel: Option<String>,
        sizes: Option<String>,
        mime_type: Option<String>,
    ) -> Self {
        Self {
            url: url.unwrap_or_default(),
            rel: rel.unwrap_or_default(),
            sizes,
            mime_type,
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\HeadingInfo")]
pub struct HeadingInfo {
    /// The heading level (1-6).
    #[php(prop, name = "level")]
    pub level: u8,
    /// The heading text content.
    #[php(prop, name = "text")]
    pub text: String,
}

#[php_impl]
impl HeadingInfo {
    pub fn __construct(level: Option<u8>, text: Option<String>) -> Self {
        Self {
            level: level.unwrap_or_default(),
            text: text.unwrap_or_default(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\ResponseMeta")]
pub struct ResponseMeta {
    /// The ETag header value.
    #[php(prop, name = "etag")]
    pub etag: Option<String>,
    /// The Last-Modified header value.
    #[php(prop, name = "last_modified")]
    pub last_modified: Option<String>,
    /// The Cache-Control header value.
    #[php(prop, name = "cache_control")]
    pub cache_control: Option<String>,
    /// The Server header value.
    #[php(prop, name = "server")]
    pub server: Option<String>,
    /// The X-Powered-By header value.
    #[php(prop, name = "x_powered_by")]
    pub x_powered_by: Option<String>,
    /// The Content-Language header value.
    #[php(prop, name = "content_language")]
    pub content_language: Option<String>,
    /// The Content-Encoding header value.
    #[php(prop, name = "content_encoding")]
    pub content_encoding: Option<String>,
}

#[php_impl]
impl ResponseMeta {
    pub fn __construct(
        etag: Option<String>,
        last_modified: Option<String>,
        cache_control: Option<String>,
        server: Option<String>,
        x_powered_by: Option<String>,
        content_language: Option<String>,
        content_encoding: Option<String>,
    ) -> Self {
        Self {
            etag,
            last_modified,
            cache_control,
            server,
            x_powered_by,
            content_language,
            content_encoding,
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\PageMetadata")]
#[allow(clippy::similar_names)]
pub struct PageMetadata {
    /// The page title from the `<title>` element.
    #[php(prop, name = "title")]
    pub title: Option<String>,
    /// The meta description.
    #[php(prop, name = "description")]
    pub description: Option<String>,
    /// The canonical URL from `<link rel="canonical">`.
    #[php(prop, name = "canonical_url")]
    pub canonical_url: Option<String>,
    /// Keywords from `<meta name="keywords">`.
    #[php(prop, name = "keywords")]
    pub keywords: Option<String>,
    /// Author from `<meta name="author">`.
    #[php(prop, name = "author")]
    pub author: Option<String>,
    /// Viewport content from `<meta name="viewport">`.
    #[php(prop, name = "viewport")]
    pub viewport: Option<String>,
    /// Theme color from `<meta name="theme-color">`.
    #[php(prop, name = "theme_color")]
    pub theme_color: Option<String>,
    /// Generator from `<meta name="generator">`.
    #[php(prop, name = "generator")]
    pub generator: Option<String>,
    /// Robots content from `<meta name="robots">`.
    #[php(prop, name = "robots")]
    pub robots: Option<String>,
    /// The `lang` attribute from the `<html>` element.
    #[php(prop, name = "html_lang")]
    pub html_lang: Option<String>,
    /// The `dir` attribute from the `<html>` element.
    #[php(prop, name = "html_dir")]
    pub html_dir: Option<String>,
    /// Open Graph title.
    #[php(prop, name = "og_title")]
    pub og_title: Option<String>,
    /// Open Graph type.
    #[php(prop, name = "og_type")]
    pub og_type: Option<String>,
    /// Open Graph image URL.
    #[php(prop, name = "og_image")]
    pub og_image: Option<String>,
    /// Open Graph description.
    #[php(prop, name = "og_description")]
    pub og_description: Option<String>,
    /// Open Graph URL.
    #[php(prop, name = "og_url")]
    pub og_url: Option<String>,
    /// Open Graph site name.
    #[php(prop, name = "og_site_name")]
    pub og_site_name: Option<String>,
    /// Open Graph locale.
    #[php(prop, name = "og_locale")]
    pub og_locale: Option<String>,
    /// Open Graph video URL.
    #[php(prop, name = "og_video")]
    pub og_video: Option<String>,
    /// Open Graph audio URL.
    #[php(prop, name = "og_audio")]
    pub og_audio: Option<String>,
    /// Open Graph locale alternates.
    #[php(prop, name = "og_locale_alternates")]
    pub og_locale_alternates: Option<Vec<String>>,
    /// Twitter card type.
    #[php(prop, name = "twitter_card")]
    pub twitter_card: Option<String>,
    /// Twitter title.
    #[php(prop, name = "twitter_title")]
    pub twitter_title: Option<String>,
    /// Twitter description.
    #[php(prop, name = "twitter_description")]
    pub twitter_description: Option<String>,
    /// Twitter image URL.
    #[php(prop, name = "twitter_image")]
    pub twitter_image: Option<String>,
    /// Twitter site handle.
    #[php(prop, name = "twitter_site")]
    pub twitter_site: Option<String>,
    /// Twitter creator handle.
    #[php(prop, name = "twitter_creator")]
    pub twitter_creator: Option<String>,
    /// Dublin Core title.
    #[php(prop, name = "dc_title")]
    pub dc_title: Option<String>,
    /// Dublin Core creator.
    #[php(prop, name = "dc_creator")]
    pub dc_creator: Option<String>,
    /// Dublin Core subject.
    #[php(prop, name = "dc_subject")]
    pub dc_subject: Option<String>,
    /// Dublin Core description.
    #[php(prop, name = "dc_description")]
    pub dc_description: Option<String>,
    /// Dublin Core publisher.
    #[php(prop, name = "dc_publisher")]
    pub dc_publisher: Option<String>,
    /// Dublin Core date.
    #[php(prop, name = "dc_date")]
    pub dc_date: Option<String>,
    /// Dublin Core type.
    #[php(prop, name = "dc_type")]
    pub dc_type: Option<String>,
    /// Dublin Core format.
    #[php(prop, name = "dc_format")]
    pub dc_format: Option<String>,
    /// Dublin Core identifier.
    #[php(prop, name = "dc_identifier")]
    pub dc_identifier: Option<String>,
    /// Dublin Core language.
    #[php(prop, name = "dc_language")]
    pub dc_language: Option<String>,
    /// Dublin Core rights.
    #[php(prop, name = "dc_rights")]
    pub dc_rights: Option<String>,
    /// Article metadata from `article:*` Open Graph tags.
    pub article: Option<ArticleMetadata>,
    /// Hreflang alternate links.
    pub hreflangs: Option<Vec<HreflangEntry>>,
    /// Favicon and icon links.
    pub favicons: Option<Vec<FaviconInfo>>,
    /// Heading elements (h1-h6).
    pub headings: Option<Vec<HeadingInfo>>,
    /// Computed word count of the page body text.
    #[php(prop, name = "word_count")]
    pub word_count: Option<i64>,
}

#[php_impl]
impl PageMetadata {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_article(&self) -> Option<ArticleMetadata> {
        self.article.clone()
    }

    #[php(getter)]
    pub fn get_hreflangs(&self) -> Option<Vec<HreflangEntry>> {
        self.hreflangs.clone()
    }

    #[php(getter)]
    pub fn get_favicons(&self) -> Option<Vec<FaviconInfo>> {
        self.favicons.clone()
    }

    #[php(getter)]
    pub fn get_headings(&self) -> Option<Vec<HeadingInfo>> {
        self.headings.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\CitationResult")]
pub struct CitationResult {
    /// Markdown with links replaced by numbered citations.
    #[php(prop, name = "content")]
    pub content: String,
    /// Numbered reference list: (index, url, text).
    pub references: Vec<CitationReference>,
}

#[php_impl]
impl CitationResult {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_references(&self) -> Vec<CitationReference> {
        self.references.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\CitationReference")]
pub struct CitationReference {
    #[php(prop, name = "index")]
    pub index: i64,
    #[php(prop, name = "url")]
    pub url: String,
    #[php(prop, name = "text")]
    pub text: String,
}

#[php_impl]
impl CitationReference {
    pub fn __construct(index: Option<i64>, url: Option<String>, text: Option<String>) -> Self {
        Self {
            index: index.unwrap_or_default(),
            url: url.unwrap_or_default(),
            text: text.unwrap_or_default(),
        }
    }
}

#[derive(Clone)]
#[php_class]
#[php(name = "Kreuzcrawl\\CrawlEngineHandle")]
pub struct CrawlEngineHandle {
    inner: Arc<kreuzcrawl::CrawlEngineHandle>,
}

#[php_impl]
impl CrawlEngineHandle {}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\BatchScrapeResult")]
pub struct BatchScrapeResult {
    /// The URL that was scraped.
    #[php(prop, name = "url")]
    pub url: String,
    /// The scrape result, if successful.
    pub result: Option<ScrapeResult>,
    /// The error message, if the scrape failed.
    #[php(prop, name = "error")]
    pub error: Option<String>,
}

#[php_impl]
impl BatchScrapeResult {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_result(&self) -> Option<ScrapeResult> {
        self.result.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Kreuzcrawl\\BatchCrawlResult")]
pub struct BatchCrawlResult {
    /// The seed URL that was crawled.
    #[php(prop, name = "url")]
    pub url: String,
    /// The crawl result, if successful.
    pub result: Option<CrawlResult>,
    /// The error message, if the crawl failed.
    #[php(prop, name = "error")]
    pub error: Option<String>,
}

#[php_impl]
impl BatchCrawlResult {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_result(&self) -> Option<CrawlResult> {
        self.result.clone()
    }
}

// BrowserMode enum values
pub const BROWSERMODE_AUTO: &str = "Auto";
pub const BROWSERMODE_ALWAYS: &str = "Always";
pub const BROWSERMODE_NEVER: &str = "Never";

// BrowserWait enum values
pub const BROWSERWAIT_NETWORKIDLE: &str = "NetworkIdle";
pub const BROWSERWAIT_SELECTOR: &str = "Selector";
pub const BROWSERWAIT_FIXED: &str = "Fixed";

// AuthConfig enum values
pub const AUTHCONFIG_BASIC: &str = "Basic";
pub const AUTHCONFIG_BEARER: &str = "Bearer";
pub const AUTHCONFIG_HEADER: &str = "Header";

// LinkType enum values
pub const LINKTYPE_INTERNAL: &str = "Internal";
pub const LINKTYPE_EXTERNAL: &str = "External";
pub const LINKTYPE_ANCHOR: &str = "Anchor";
pub const LINKTYPE_DOCUMENT: &str = "Document";

// ImageSource enum values
pub const IMAGESOURCE_IMG: &str = "Img";
pub const IMAGESOURCE_PICTURESOURCE: &str = "PictureSource";
pub const IMAGESOURCE_OGIMAGE: &str = "OgImage";
pub const IMAGESOURCE_TWITTERIMAGE: &str = "TwitterImage";

// FeedType enum values
pub const FEEDTYPE_RSS: &str = "Rss";
pub const FEEDTYPE_ATOM: &str = "Atom";
pub const FEEDTYPE_JSONFEED: &str = "JsonFeed";

// AssetCategory enum values
pub const ASSETCATEGORY_DOCUMENT: &str = "Document";
pub const ASSETCATEGORY_IMAGE: &str = "Image";
pub const ASSETCATEGORY_AUDIO: &str = "Audio";
pub const ASSETCATEGORY_VIDEO: &str = "Video";
pub const ASSETCATEGORY_FONT: &str = "Font";
pub const ASSETCATEGORY_STYLESHEET: &str = "Stylesheet";
pub const ASSETCATEGORY_SCRIPT: &str = "Script";
pub const ASSETCATEGORY_ARCHIVE: &str = "Archive";
pub const ASSETCATEGORY_DATA: &str = "Data";
pub const ASSETCATEGORY_OTHER: &str = "Other";

// CrawlEvent enum values
pub const CRAWLEVENT_PAGE: &str = "Page";
pub const CRAWLEVENT_ERROR: &str = "Error";
pub const CRAWLEVENT_COMPLETE: &str = "Complete";

#[php_class]
#[php(name = "Kreuzcrawl\\KreuzcrawlApi")]
pub struct KreuzcrawlApi;

#[php_impl]
impl KreuzcrawlApi {
    pub fn create_engine(config: Option<&CrawlConfig>) -> PhpResult<CrawlEngineHandle> {
        let config_core: Option<kreuzcrawl::CrawlConfig> = config.map(|v| v.clone().into());
        let result = kreuzcrawl::create_engine(config_core)
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(CrawlEngineHandle {
            inner: Arc::new(result),
        })
    }

    pub fn scrape_async(engine: &CrawlEngineHandle, url: String) -> PhpResult<ScrapeResult> {
        WORKER_RUNTIME.block_on(async {
            let result = kreuzcrawl::scrape(&engine.inner, &url)
                .await
                .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
            Ok(result.into())
        })
    }

    pub fn crawl_async(engine: &CrawlEngineHandle, url: String) -> PhpResult<CrawlResult> {
        WORKER_RUNTIME.block_on(async {
            let result = kreuzcrawl::crawl(&engine.inner, &url)
                .await
                .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
            Ok(result.into())
        })
    }

    pub fn map_urls_async(engine: &CrawlEngineHandle, url: String) -> PhpResult<MapResult> {
        WORKER_RUNTIME.block_on(async {
            let result = kreuzcrawl::map_urls(&engine.inner, &url)
                .await
                .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
            Ok(result.into())
        })
    }

    pub fn batch_scrape_async(engine: &CrawlEngineHandle, urls: Vec<String>) -> Vec<BatchScrapeResult> {
        let result = WORKER_RUNTIME.block_on(async { kreuzcrawl::batch_scrape(&engine.inner, urls).await });
        result.into_iter().map(Into::into).collect()
    }

    pub fn batch_crawl_async(engine: &CrawlEngineHandle, urls: Vec<String>) -> Vec<BatchCrawlResult> {
        let result = WORKER_RUNTIME.block_on(async { kreuzcrawl::batch_crawl(&engine.inner, urls).await });
        result.into_iter().map(Into::into).collect()
    }
}

impl From<ExtractionMeta> for kreuzcrawl::ExtractionMeta {
    fn from(val: ExtractionMeta) -> Self {
        Self {
            cost: val.cost,
            prompt_tokens: val.prompt_tokens.map(|v| v as u64),
            completion_tokens: val.completion_tokens.map(|v| v as u64),
            model: val.model,
            chunks_processed: val.chunks_processed as usize,
        }
    }
}

impl From<kreuzcrawl::ExtractionMeta> for ExtractionMeta {
    fn from(val: kreuzcrawl::ExtractionMeta) -> Self {
        Self {
            cost: val.cost,
            prompt_tokens: val.prompt_tokens.map(|v| v as i64),
            completion_tokens: val.completion_tokens.map(|v| v as i64),
            model: val.model,
            chunks_processed: val.chunks_processed as i64,
        }
    }
}

impl From<ProxyConfig> for kreuzcrawl::ProxyConfig {
    fn from(val: ProxyConfig) -> Self {
        Self {
            url: val.url,
            username: val.username,
            password: val.password,
        }
    }
}

impl From<kreuzcrawl::ProxyConfig> for ProxyConfig {
    fn from(val: kreuzcrawl::ProxyConfig) -> Self {
        Self {
            url: val.url,
            username: val.username,
            password: val.password,
        }
    }
}

impl From<BrowserConfig> for kreuzcrawl::BrowserConfig {
    fn from(val: BrowserConfig) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<kreuzcrawl::BrowserConfig> for BrowserConfig {
    fn from(val: kreuzcrawl::BrowserConfig) -> Self {
        Self {
            mode: serde_json::to_value(val.mode)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            endpoint: val.endpoint,
            timeout: Some(val.timeout.as_millis() as u64 as i64),
            wait: serde_json::to_value(val.wait)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            wait_selector: val.wait_selector,
            extra_wait: val.extra_wait.map(|d| d.as_millis() as u64 as i64),
        }
    }
}

impl From<CrawlConfig> for kreuzcrawl::CrawlConfig {
    fn from(val: CrawlConfig) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<kreuzcrawl::CrawlConfig> for CrawlConfig {
    fn from(val: kreuzcrawl::CrawlConfig) -> Self {
        Self {
            max_depth: val.max_depth.map(|v| v as i64),
            max_pages: val.max_pages.map(|v| v as i64),
            max_concurrent: val.max_concurrent.map(|v| v as i64),
            respect_robots_txt: val.respect_robots_txt,
            user_agent: val.user_agent,
            stay_on_domain: val.stay_on_domain,
            allow_subdomains: val.allow_subdomains,
            include_paths: val.include_paths,
            exclude_paths: val.exclude_paths,
            custom_headers: val.custom_headers.into_iter().collect(),
            request_timeout: Some(val.request_timeout.as_millis() as u64 as i64),
            max_redirects: val.max_redirects as i64,
            retry_count: val.retry_count as i64,
            retry_codes: val.retry_codes,
            cookies_enabled: val.cookies_enabled,
            auth: val.auth.as_ref().map(|v| {
                serde_json::to_value(v)
                    .ok()
                    .and_then(|s| s.as_str().map(String::from))
                    .unwrap_or_default()
            }),
            max_body_size: val.max_body_size.map(|v| v as i64),
            main_content_only: val.main_content_only,
            remove_tags: val.remove_tags,
            map_limit: val.map_limit.map(|v| v as i64),
            map_search: val.map_search,
            download_assets: val.download_assets,
            asset_types: val
                .asset_types
                .iter()
                .map(|v| {
                    serde_json::to_value(v)
                        .ok()
                        .and_then(|s| s.as_str().map(String::from))
                        .unwrap_or_default()
                })
                .collect(),
            max_asset_size: val.max_asset_size.map(|v| v as i64),
            browser: val.browser.into(),
            proxy: val.proxy.map(Into::into),
            user_agents: val.user_agents,
            capture_screenshot: val.capture_screenshot,
            download_documents: val.download_documents,
            document_max_size: val.document_max_size.map(|v| v as i64),
            document_mime_types: val.document_mime_types,
            warc_output: val.warc_output.map(|p| p.to_string_lossy().to_string()),
            browser_profile: val.browser_profile,
            save_browser_profile: val.save_browser_profile,
        }
    }
}

impl From<DownloadedDocument> for kreuzcrawl::DownloadedDocument {
    fn from(val: DownloadedDocument) -> Self {
        Self {
            url: val.url,
            mime_type: Default::default(),
            content: val.content,
            size: val.size as usize,
            filename: Default::default(),
            content_hash: Default::default(),
            headers: Default::default(),
        }
    }
}

impl From<kreuzcrawl::DownloadedDocument> for DownloadedDocument {
    fn from(val: kreuzcrawl::DownloadedDocument) -> Self {
        Self {
            url: val.url,
            mime_type: format!("{:?}", val.mime_type),
            content: val.content.to_vec(),
            size: val.size as i64,
            filename: val.filename.as_ref().map(|v| format!("{v:?}")),
            content_hash: format!("{:?}", val.content_hash),
            headers: val
                .headers
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}

impl From<kreuzcrawl::InteractionResult> for InteractionResult {
    fn from(val: kreuzcrawl::InteractionResult) -> Self {
        Self {
            action_results: val.action_results.into_iter().map(Into::into).collect(),
            final_html: val.final_html,
            final_url: val.final_url,
            screenshot: val.screenshot.map(|v| v.to_vec()),
        }
    }
}

impl From<kreuzcrawl::ActionResult> for ActionResult {
    fn from(val: kreuzcrawl::ActionResult) -> Self {
        Self {
            action_index: val.action_index as i64,
            action_type: format!("{:?}", val.action_type),
            success: val.success,
            data: val.data.as_ref().map(ToString::to_string),
            error: val.error,
        }
    }
}

impl From<ScrapeResult> for kreuzcrawl::ScrapeResult {
    fn from(val: ScrapeResult) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<kreuzcrawl::ScrapeResult> for ScrapeResult {
    fn from(val: kreuzcrawl::ScrapeResult) -> Self {
        Self {
            status_code: val.status_code,
            content_type: val.content_type,
            html: val.html,
            body_size: val.body_size as i64,
            metadata: val.metadata.into(),
            links: val.links.into_iter().map(Into::into).collect(),
            images: val.images.into_iter().map(Into::into).collect(),
            feeds: val.feeds.into_iter().map(Into::into).collect(),
            json_ld: val.json_ld.into_iter().map(Into::into).collect(),
            is_allowed: val.is_allowed,
            crawl_delay: val.crawl_delay.map(|v| v as i64),
            noindex_detected: val.noindex_detected,
            nofollow_detected: val.nofollow_detected,
            x_robots_tag: val.x_robots_tag,
            is_pdf: val.is_pdf,
            was_skipped: val.was_skipped,
            detected_charset: val.detected_charset,
            main_content_only: val.main_content_only,
            auth_header_sent: val.auth_header_sent,
            response_meta: val.response_meta.map(Into::into),
            assets: val.assets.into_iter().map(Into::into).collect(),
            js_render_hint: val.js_render_hint,
            browser_used: val.browser_used,
            markdown: val.markdown.map(Into::into),
            extracted_data: val.extracted_data.as_ref().map(ToString::to_string),
            extraction_meta: val.extraction_meta.map(Into::into),
            screenshot: val.screenshot.map(|v| v.to_vec()),
            downloaded_document: val.downloaded_document.map(Into::into),
        }
    }
}

impl From<CrawlPageResult> for kreuzcrawl::CrawlPageResult {
    fn from(val: CrawlPageResult) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<kreuzcrawl::CrawlPageResult> for CrawlPageResult {
    fn from(val: kreuzcrawl::CrawlPageResult) -> Self {
        Self {
            url: val.url,
            normalized_url: val.normalized_url,
            status_code: val.status_code,
            content_type: val.content_type,
            html: val.html,
            body_size: val.body_size as i64,
            metadata: val.metadata.into(),
            links: val.links.into_iter().map(Into::into).collect(),
            images: val.images.into_iter().map(Into::into).collect(),
            feeds: val.feeds.into_iter().map(Into::into).collect(),
            json_ld: val.json_ld.into_iter().map(Into::into).collect(),
            depth: val.depth as i64,
            stayed_on_domain: val.stayed_on_domain,
            was_skipped: val.was_skipped,
            is_pdf: val.is_pdf,
            detected_charset: val.detected_charset,
            markdown: val.markdown.map(Into::into),
            extracted_data: val.extracted_data.as_ref().map(ToString::to_string),
            extraction_meta: val.extraction_meta.map(Into::into),
            downloaded_document: val.downloaded_document.map(Into::into),
        }
    }
}

impl From<CrawlResult> for kreuzcrawl::CrawlResult {
    fn from(val: CrawlResult) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<kreuzcrawl::CrawlResult> for CrawlResult {
    fn from(val: kreuzcrawl::CrawlResult) -> Self {
        Self {
            pages: val.pages.into_iter().map(Into::into).collect(),
            final_url: val.final_url,
            redirect_count: val.redirect_count as i64,
            was_skipped: val.was_skipped,
            error: val.error,
            cookies: val.cookies.into_iter().map(Into::into).collect(),
            normalized_urls: val.normalized_urls,
        }
    }
}

impl From<SitemapUrl> for kreuzcrawl::SitemapUrl {
    fn from(val: SitemapUrl) -> Self {
        Self {
            url: val.url,
            lastmod: val.lastmod,
            changefreq: val.changefreq,
            priority: val.priority,
        }
    }
}

impl From<kreuzcrawl::SitemapUrl> for SitemapUrl {
    fn from(val: kreuzcrawl::SitemapUrl) -> Self {
        Self {
            url: val.url,
            lastmod: val.lastmod,
            changefreq: val.changefreq,
            priority: val.priority,
        }
    }
}

impl From<MapResult> for kreuzcrawl::MapResult {
    fn from(val: MapResult) -> Self {
        Self {
            urls: val.urls.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<kreuzcrawl::MapResult> for MapResult {
    fn from(val: kreuzcrawl::MapResult) -> Self {
        Self {
            urls: val.urls.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<MarkdownResult> for kreuzcrawl::MarkdownResult {
    fn from(val: MarkdownResult) -> Self {
        Self {
            content: val.content,
            document_structure: Default::default(),
            tables: val
                .tables
                .into_iter()
                .filter_map(|s| serde_json::from_str(&s).ok())
                .collect(),
            warnings: val.warnings,
            citations: val.citations.map(Into::into),
            fit_content: val.fit_content,
        }
    }
}

impl From<kreuzcrawl::MarkdownResult> for MarkdownResult {
    fn from(val: kreuzcrawl::MarkdownResult) -> Self {
        Self {
            content: val.content,
            document_structure: val.document_structure.as_ref().map(ToString::to_string),
            tables: val.tables.iter().map(ToString::to_string).collect(),
            warnings: val.warnings,
            citations: val.citations.map(Into::into),
            fit_content: val.fit_content,
        }
    }
}

impl From<kreuzcrawl::CachedPage> for CachedPage {
    fn from(val: kreuzcrawl::CachedPage) -> Self {
        Self {
            url: val.url,
            status_code: val.status_code,
            content_type: val.content_type,
            body: val.body,
            etag: val.etag,
            last_modified: val.last_modified,
            cached_at: val.cached_at as i64,
        }
    }
}

impl From<LinkInfo> for kreuzcrawl::LinkInfo {
    fn from(val: LinkInfo) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<kreuzcrawl::LinkInfo> for LinkInfo {
    fn from(val: kreuzcrawl::LinkInfo) -> Self {
        Self {
            url: val.url,
            text: val.text,
            link_type: serde_json::to_value(val.link_type)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            rel: val.rel,
            nofollow: val.nofollow,
        }
    }
}

impl From<ImageInfo> for kreuzcrawl::ImageInfo {
    fn from(val: ImageInfo) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<kreuzcrawl::ImageInfo> for ImageInfo {
    fn from(val: kreuzcrawl::ImageInfo) -> Self {
        Self {
            url: val.url,
            alt: val.alt,
            width: val.width,
            height: val.height,
            source: serde_json::to_value(val.source)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
        }
    }
}

impl From<FeedInfo> for kreuzcrawl::FeedInfo {
    fn from(val: FeedInfo) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<kreuzcrawl::FeedInfo> for FeedInfo {
    fn from(val: kreuzcrawl::FeedInfo) -> Self {
        Self {
            url: val.url,
            title: val.title,
            feed_type: serde_json::to_value(val.feed_type)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
        }
    }
}

impl From<JsonLdEntry> for kreuzcrawl::JsonLdEntry {
    fn from(val: JsonLdEntry) -> Self {
        Self {
            schema_type: val.schema_type,
            name: val.name,
            raw: val.raw,
        }
    }
}

impl From<kreuzcrawl::JsonLdEntry> for JsonLdEntry {
    fn from(val: kreuzcrawl::JsonLdEntry) -> Self {
        Self {
            schema_type: val.schema_type,
            name: val.name,
            raw: val.raw,
        }
    }
}

impl From<CookieInfo> for kreuzcrawl::CookieInfo {
    fn from(val: CookieInfo) -> Self {
        Self {
            name: val.name,
            value: val.value,
            domain: val.domain,
            path: val.path,
        }
    }
}

impl From<kreuzcrawl::CookieInfo> for CookieInfo {
    fn from(val: kreuzcrawl::CookieInfo) -> Self {
        Self {
            name: val.name,
            value: val.value,
            domain: val.domain,
            path: val.path,
        }
    }
}

impl From<DownloadedAsset> for kreuzcrawl::DownloadedAsset {
    fn from(val: DownloadedAsset) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<kreuzcrawl::DownloadedAsset> for DownloadedAsset {
    fn from(val: kreuzcrawl::DownloadedAsset) -> Self {
        Self {
            url: val.url,
            content_hash: val.content_hash,
            mime_type: val.mime_type,
            size: val.size as i64,
            asset_category: serde_json::to_value(val.asset_category)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            html_tag: val.html_tag,
        }
    }
}

impl From<ArticleMetadata> for kreuzcrawl::ArticleMetadata {
    fn from(val: ArticleMetadata) -> Self {
        Self {
            published_time: val.published_time,
            modified_time: val.modified_time,
            author: val.author,
            section: val.section,
            tags: val.tags,
        }
    }
}

impl From<kreuzcrawl::ArticleMetadata> for ArticleMetadata {
    fn from(val: kreuzcrawl::ArticleMetadata) -> Self {
        Self {
            published_time: val.published_time,
            modified_time: val.modified_time,
            author: val.author,
            section: val.section,
            tags: val.tags,
        }
    }
}

impl From<HreflangEntry> for kreuzcrawl::HreflangEntry {
    fn from(val: HreflangEntry) -> Self {
        Self {
            lang: val.lang,
            url: val.url,
        }
    }
}

impl From<kreuzcrawl::HreflangEntry> for HreflangEntry {
    fn from(val: kreuzcrawl::HreflangEntry) -> Self {
        Self {
            lang: val.lang,
            url: val.url,
        }
    }
}

impl From<FaviconInfo> for kreuzcrawl::FaviconInfo {
    fn from(val: FaviconInfo) -> Self {
        Self {
            url: val.url,
            rel: val.rel,
            sizes: val.sizes,
            mime_type: val.mime_type,
        }
    }
}

impl From<kreuzcrawl::FaviconInfo> for FaviconInfo {
    fn from(val: kreuzcrawl::FaviconInfo) -> Self {
        Self {
            url: val.url,
            rel: val.rel,
            sizes: val.sizes,
            mime_type: val.mime_type,
        }
    }
}

impl From<HeadingInfo> for kreuzcrawl::HeadingInfo {
    fn from(val: HeadingInfo) -> Self {
        Self {
            level: val.level,
            text: val.text,
        }
    }
}

impl From<kreuzcrawl::HeadingInfo> for HeadingInfo {
    fn from(val: kreuzcrawl::HeadingInfo) -> Self {
        Self {
            level: val.level,
            text: val.text,
        }
    }
}

impl From<ResponseMeta> for kreuzcrawl::ResponseMeta {
    fn from(val: ResponseMeta) -> Self {
        Self {
            etag: val.etag,
            last_modified: val.last_modified,
            cache_control: val.cache_control,
            server: val.server,
            x_powered_by: val.x_powered_by,
            content_language: val.content_language,
            content_encoding: val.content_encoding,
        }
    }
}

impl From<kreuzcrawl::ResponseMeta> for ResponseMeta {
    fn from(val: kreuzcrawl::ResponseMeta) -> Self {
        Self {
            etag: val.etag,
            last_modified: val.last_modified,
            cache_control: val.cache_control,
            server: val.server,
            x_powered_by: val.x_powered_by,
            content_language: val.content_language,
            content_encoding: val.content_encoding,
        }
    }
}

impl From<PageMetadata> for kreuzcrawl::PageMetadata {
    fn from(val: PageMetadata) -> Self {
        Self {
            title: val.title,
            description: val.description,
            canonical_url: val.canonical_url,
            keywords: val.keywords,
            author: val.author,
            viewport: val.viewport,
            theme_color: val.theme_color,
            generator: val.generator,
            robots: val.robots,
            html_lang: val.html_lang,
            html_dir: val.html_dir,
            og_title: val.og_title,
            og_type: val.og_type,
            og_image: val.og_image,
            og_description: val.og_description,
            og_url: val.og_url,
            og_site_name: val.og_site_name,
            og_locale: val.og_locale,
            og_video: val.og_video,
            og_audio: val.og_audio,
            og_locale_alternates: val.og_locale_alternates,
            twitter_card: val.twitter_card,
            twitter_title: val.twitter_title,
            twitter_description: val.twitter_description,
            twitter_image: val.twitter_image,
            twitter_site: val.twitter_site,
            twitter_creator: val.twitter_creator,
            dc_title: val.dc_title,
            dc_creator: val.dc_creator,
            dc_subject: val.dc_subject,
            dc_description: val.dc_description,
            dc_publisher: val.dc_publisher,
            dc_date: val.dc_date,
            dc_type: val.dc_type,
            dc_format: val.dc_format,
            dc_identifier: val.dc_identifier,
            dc_language: val.dc_language,
            dc_rights: val.dc_rights,
            article: val.article.map(Into::into),
            hreflangs: val.hreflangs.map(|v| v.into_iter().map(Into::into).collect()),
            favicons: val.favicons.map(|v| v.into_iter().map(Into::into).collect()),
            headings: val.headings.map(|v| v.into_iter().map(Into::into).collect()),
            word_count: val.word_count.map(|v| v as usize),
        }
    }
}

impl From<kreuzcrawl::PageMetadata> for PageMetadata {
    fn from(val: kreuzcrawl::PageMetadata) -> Self {
        Self {
            title: val.title,
            description: val.description,
            canonical_url: val.canonical_url,
            keywords: val.keywords,
            author: val.author,
            viewport: val.viewport,
            theme_color: val.theme_color,
            generator: val.generator,
            robots: val.robots,
            html_lang: val.html_lang,
            html_dir: val.html_dir,
            og_title: val.og_title,
            og_type: val.og_type,
            og_image: val.og_image,
            og_description: val.og_description,
            og_url: val.og_url,
            og_site_name: val.og_site_name,
            og_locale: val.og_locale,
            og_video: val.og_video,
            og_audio: val.og_audio,
            og_locale_alternates: val.og_locale_alternates,
            twitter_card: val.twitter_card,
            twitter_title: val.twitter_title,
            twitter_description: val.twitter_description,
            twitter_image: val.twitter_image,
            twitter_site: val.twitter_site,
            twitter_creator: val.twitter_creator,
            dc_title: val.dc_title,
            dc_creator: val.dc_creator,
            dc_subject: val.dc_subject,
            dc_description: val.dc_description,
            dc_publisher: val.dc_publisher,
            dc_date: val.dc_date,
            dc_type: val.dc_type,
            dc_format: val.dc_format,
            dc_identifier: val.dc_identifier,
            dc_language: val.dc_language,
            dc_rights: val.dc_rights,
            article: val.article.map(Into::into),
            hreflangs: val.hreflangs.map(|v| v.into_iter().map(Into::into).collect()),
            favicons: val.favicons.map(|v| v.into_iter().map(Into::into).collect()),
            headings: val.headings.map(|v| v.into_iter().map(Into::into).collect()),
            word_count: val.word_count.map(|v| v as i64),
        }
    }
}

impl From<CitationResult> for kreuzcrawl::CitationResult {
    fn from(val: CitationResult) -> Self {
        Self {
            content: val.content,
            references: val.references.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<kreuzcrawl::CitationResult> for CitationResult {
    fn from(val: kreuzcrawl::CitationResult) -> Self {
        Self {
            content: val.content,
            references: val.references.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<CitationReference> for kreuzcrawl::CitationReference {
    fn from(val: CitationReference) -> Self {
        Self {
            index: val.index as usize,
            url: val.url,
            text: val.text,
        }
    }
}

impl From<kreuzcrawl::CitationReference> for CitationReference {
    fn from(val: kreuzcrawl::CitationReference) -> Self {
        Self {
            index: val.index as i64,
            url: val.url,
            text: val.text,
        }
    }
}

impl From<BatchScrapeResult> for kreuzcrawl::BatchScrapeResult {
    fn from(val: BatchScrapeResult) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<kreuzcrawl::BatchScrapeResult> for BatchScrapeResult {
    fn from(val: kreuzcrawl::BatchScrapeResult) -> Self {
        Self {
            url: val.url,
            result: val.result.map(Into::into),
            error: val.error,
        }
    }
}

impl From<BatchCrawlResult> for kreuzcrawl::BatchCrawlResult {
    fn from(val: BatchCrawlResult) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<kreuzcrawl::BatchCrawlResult> for BatchCrawlResult {
    fn from(val: kreuzcrawl::BatchCrawlResult) -> Self {
        Self {
            url: val.url,
            result: val.result.map(Into::into),
            error: val.error,
        }
    }
}

/// Convert a `kreuzcrawl::CrawlError` error to a PHP exception.
#[allow(dead_code)]
fn crawl_error_to_php_err(e: kreuzcrawl::CrawlError) -> ext_php_rs::exception::PhpException {
    let msg = e.to_string();
    #[allow(unreachable_patterns)]
    match &e {
        kreuzcrawl::CrawlError::NotFound(..) => {
            ext_php_rs::exception::PhpException::default(format!("[NotFound] {}", msg))
        }
        kreuzcrawl::CrawlError::Unauthorized(..) => {
            ext_php_rs::exception::PhpException::default(format!("[Unauthorized] {}", msg))
        }
        kreuzcrawl::CrawlError::Forbidden(..) => {
            ext_php_rs::exception::PhpException::default(format!("[Forbidden] {}", msg))
        }
        kreuzcrawl::CrawlError::WafBlocked(..) => {
            ext_php_rs::exception::PhpException::default(format!("[WafBlocked] {}", msg))
        }
        kreuzcrawl::CrawlError::Timeout(..) => {
            ext_php_rs::exception::PhpException::default(format!("[Timeout] {}", msg))
        }
        kreuzcrawl::CrawlError::RateLimited(..) => {
            ext_php_rs::exception::PhpException::default(format!("[RateLimited] {}", msg))
        }
        kreuzcrawl::CrawlError::ServerError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[ServerError] {}", msg))
        }
        kreuzcrawl::CrawlError::BadGateway(..) => {
            ext_php_rs::exception::PhpException::default(format!("[BadGateway] {}", msg))
        }
        kreuzcrawl::CrawlError::Gone(..) => ext_php_rs::exception::PhpException::default(format!("[Gone] {}", msg)),
        kreuzcrawl::CrawlError::Connection(..) => {
            ext_php_rs::exception::PhpException::default(format!("[Connection] {}", msg))
        }
        kreuzcrawl::CrawlError::Dns(..) => ext_php_rs::exception::PhpException::default(format!("[Dns] {}", msg)),
        kreuzcrawl::CrawlError::Ssl(..) => ext_php_rs::exception::PhpException::default(format!("[Ssl] {}", msg)),
        kreuzcrawl::CrawlError::DataLoss(..) => {
            ext_php_rs::exception::PhpException::default(format!("[DataLoss] {}", msg))
        }
        kreuzcrawl::CrawlError::BrowserError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[BrowserError] {}", msg))
        }
        kreuzcrawl::CrawlError::BrowserTimeout(..) => {
            ext_php_rs::exception::PhpException::default(format!("[BrowserTimeout] {}", msg))
        }
        kreuzcrawl::CrawlError::InvalidConfig(..) => {
            ext_php_rs::exception::PhpException::default(format!("[InvalidConfig] {}", msg))
        }
        kreuzcrawl::CrawlError::Other(..) => ext_php_rs::exception::PhpException::default(format!("[Other] {}", msg)),
        _ => ext_php_rs::exception::PhpException::default(msg),
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .class::<ExtractionMeta>()
        .class::<ProxyConfig>()
        .class::<BrowserConfig>()
        .class::<CrawlConfig>()
        .class::<DownloadedDocument>()
        .class::<InteractionResult>()
        .class::<ActionResult>()
        .class::<ScrapeResult>()
        .class::<CrawlPageResult>()
        .class::<CrawlResult>()
        .class::<SitemapUrl>()
        .class::<MapResult>()
        .class::<MarkdownResult>()
        .class::<CachedPage>()
        .class::<LinkInfo>()
        .class::<ImageInfo>()
        .class::<FeedInfo>()
        .class::<JsonLdEntry>()
        .class::<CookieInfo>()
        .class::<DownloadedAsset>()
        .class::<ArticleMetadata>()
        .class::<HreflangEntry>()
        .class::<FaviconInfo>()
        .class::<HeadingInfo>()
        .class::<ResponseMeta>()
        .class::<PageMetadata>()
        .class::<CitationResult>()
        .class::<CitationReference>()
        .class::<CrawlEngineHandle>()
        .class::<BatchScrapeResult>()
        .class::<BatchCrawlResult>()
        .class::<KreuzcrawlApi>()
}
