//! Public types used across the kreuzcrawl crate.

use std::collections::{HashMap, HashSet};
use std::time::Duration;

use serde::{Deserialize, Serialize};

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

mod duration_ms {
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

mod option_duration_ms {
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
            #[cfg(feature = "browser")]
            browser_pool: None,
        }
    }
}

impl CrawlConfig {
    /// Validate the configuration, returning an error if any values are invalid.
    pub fn validate(&self) -> Result<(), crate::error::CrawlError> {
        use crate::error::CrawlError;

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

/// Article metadata extracted from `article:*` Open Graph tags.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArticleMetadata {
    /// The article publication time.
    pub published_time: Option<String>,
    /// The article modification time.
    pub modified_time: Option<String>,
    /// The article author.
    pub author: Option<String>,
    /// The article section.
    pub section: Option<String>,
    /// The article tags.
    pub tags: Vec<String>,
}

/// An hreflang alternate link entry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HreflangEntry {
    /// The language code (e.g., "en", "fr", "x-default").
    pub lang: String,
    /// The URL for this language variant.
    pub url: String,
}

/// Information about a favicon or icon link.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FaviconInfo {
    /// The icon URL.
    pub url: String,
    /// The `rel` attribute (e.g., "icon", "apple-touch-icon").
    pub rel: String,
    /// The `sizes` attribute, if present.
    pub sizes: Option<String>,
    /// The MIME type, if present.
    pub mime_type: Option<String>,
}

/// A heading element extracted from the page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeadingInfo {
    /// The heading level (1-6).
    pub level: u8,
    /// The heading text content.
    pub text: String,
}

/// Response metadata extracted from HTTP headers.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseMeta {
    /// The ETag header value.
    pub etag: Option<String>,
    /// The Last-Modified header value.
    pub last_modified: Option<String>,
    /// The Cache-Control header value.
    pub cache_control: Option<String>,
    /// The Server header value.
    pub server: Option<String>,
    /// The X-Powered-By header value.
    pub x_powered_by: Option<String>,
    /// The Content-Language header value.
    pub content_language: Option<String>,
    /// The Content-Encoding header value.
    pub content_encoding: Option<String>,
}

/// The category of a downloaded asset.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssetCategory {
    /// A document file (PDF, DOC, etc.).
    Document,
    /// An image file.
    #[default]
    Image,
    /// An audio file.
    Audio,
    /// A video file.
    Video,
    /// A font file.
    Font,
    /// A CSS stylesheet.
    Stylesheet,
    /// A JavaScript file.
    Script,
    /// An archive file (ZIP, TAR, etc.).
    Archive,
    /// A data file (JSON, XML, CSV, etc.).
    Data,
    /// An unrecognized asset type.
    Other,
}

/// Information about a downloaded asset.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DownloadedAsset {
    /// The original URL of the asset.
    pub url: String,
    /// The SHA-256 content hash of the asset.
    pub content_hash: String,
    /// The MIME type from the Content-Type header.
    pub mime_type: Option<String>,
    /// The size of the asset in bytes.
    pub size: usize,
    /// The category of the asset.
    pub asset_category: AssetCategory,
    /// The HTML tag that referenced this asset (e.g., "link", "script", "img").
    pub html_tag: Option<String>,
}

/// Metadata extracted from an HTML page's `<meta>` tags and `<title>` element.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PageMetadata {
    /// The page title from the `<title>` element.
    pub title: Option<String>,
    /// The meta description.
    pub description: Option<String>,
    /// The canonical URL from `<link rel="canonical">`.
    pub canonical_url: Option<String>,
    /// Keywords from `<meta name="keywords">`.
    pub keywords: Option<String>,
    /// Author from `<meta name="author">`.
    pub author: Option<String>,
    /// Viewport content from `<meta name="viewport">`.
    pub viewport: Option<String>,
    /// Theme color from `<meta name="theme-color">`.
    pub theme_color: Option<String>,
    /// Generator from `<meta name="generator">`.
    pub generator: Option<String>,
    /// Robots content from `<meta name="robots">`.
    pub robots: Option<String>,
    /// The `lang` attribute from the `<html>` element.
    pub html_lang: Option<String>,
    /// The `dir` attribute from the `<html>` element.
    pub html_dir: Option<String>,
    /// Open Graph title.
    pub og_title: Option<String>,
    /// Open Graph type.
    pub og_type: Option<String>,
    /// Open Graph image URL.
    pub og_image: Option<String>,
    /// Open Graph description.
    pub og_description: Option<String>,
    /// Open Graph URL.
    pub og_url: Option<String>,
    /// Open Graph site name.
    pub og_site_name: Option<String>,
    /// Open Graph locale.
    pub og_locale: Option<String>,
    /// Open Graph video URL.
    pub og_video: Option<String>,
    /// Open Graph audio URL.
    pub og_audio: Option<String>,
    /// Open Graph locale alternates.
    pub og_locale_alternates: Option<Vec<String>>,
    /// Twitter card type.
    pub twitter_card: Option<String>,
    /// Twitter title.
    pub twitter_title: Option<String>,
    /// Twitter description.
    pub twitter_description: Option<String>,
    /// Twitter image URL.
    pub twitter_image: Option<String>,
    /// Twitter site handle.
    pub twitter_site: Option<String>,
    /// Twitter creator handle.
    pub twitter_creator: Option<String>,
    /// Dublin Core title.
    pub dc_title: Option<String>,
    /// Dublin Core creator.
    pub dc_creator: Option<String>,
    /// Dublin Core subject.
    pub dc_subject: Option<String>,
    /// Dublin Core description.
    pub dc_description: Option<String>,
    /// Dublin Core publisher.
    pub dc_publisher: Option<String>,
    /// Dublin Core date.
    pub dc_date: Option<String>,
    /// Dublin Core type.
    pub dc_type: Option<String>,
    /// Dublin Core format.
    pub dc_format: Option<String>,
    /// Dublin Core identifier.
    pub dc_identifier: Option<String>,
    /// Dublin Core language.
    pub dc_language: Option<String>,
    /// Dublin Core rights.
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
    pub word_count: Option<usize>,
}

/// The classification of a link.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LinkType {
    /// A link to the same domain.
    #[default]
    Internal,
    /// A link to a different domain.
    External,
    /// A fragment-only link (e.g., `#section`).
    Anchor,
    /// A link to a downloadable document (PDF, DOC, etc.).
    Document,
}

impl std::fmt::Display for LinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Internal => write!(f, "internal"),
            Self::External => write!(f, "external"),
            Self::Anchor => write!(f, "anchor"),
            Self::Document => write!(f, "document"),
        }
    }
}

/// Information about a link found on a page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LinkInfo {
    /// The resolved URL of the link.
    pub url: String,
    /// The visible text of the link.
    pub text: String,
    /// The classification of the link.
    pub link_type: LinkType,
    /// The `rel` attribute value, if present.
    pub rel: Option<String>,
    /// Whether the link has `rel="nofollow"`.
    pub nofollow: bool,
}

/// The source of an image reference.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ImageSource {
    /// An `<img>` tag.
    #[default]
    Img,
    /// A `<source>` tag inside `<picture>`.
    PictureSource,
    /// An `og:image` meta tag.
    #[serde(rename = "og:image")]
    OgImage,
    /// A `twitter:image` meta tag.
    #[serde(rename = "twitter:image")]
    TwitterImage,
}

impl std::fmt::Display for ImageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Img => write!(f, "img"),
            Self::PictureSource => write!(f, "picture_source"),
            Self::OgImage => write!(f, "og:image"),
            Self::TwitterImage => write!(f, "twitter:image"),
        }
    }
}

/// Information about an image found on a page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImageInfo {
    /// The image URL.
    pub url: String,
    /// The alt text, if present.
    pub alt: Option<String>,
    /// The width attribute, if present and parseable.
    pub width: Option<u32>,
    /// The height attribute, if present and parseable.
    pub height: Option<u32>,
    /// The source of the image reference.
    pub source: ImageSource,
}

/// The type of a feed (RSS, Atom, or JSON Feed).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum FeedType {
    /// RSS feed.
    #[default]
    Rss,
    /// Atom feed.
    Atom,
    /// JSON Feed.
    JsonFeed,
}

/// Information about a feed link found on a page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeedInfo {
    /// The feed URL.
    pub url: String,
    /// The feed title, if present.
    pub title: Option<String>,
    /// The type of feed.
    pub feed_type: FeedType,
}

/// A JSON-LD structured data entry found on a page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonLdEntry {
    /// The `@type` value from the JSON-LD object.
    pub schema_type: String,
    /// The `name` value, if present.
    pub name: Option<String>,
    /// The raw JSON-LD string.
    pub raw: String,
}

/// Information about an HTTP cookie received from a response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CookieInfo {
    /// The cookie name.
    pub name: String,
    /// The cookie value.
    pub value: String,
    /// The cookie domain, if specified.
    pub domain: Option<String>,
    /// The cookie path, if specified.
    pub path: Option<String>,
}

/// The result of a single-page scrape operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScrapeResult {
    /// The HTTP status code of the response.
    pub status_code: u16,
    /// The Content-Type header value.
    pub content_type: String,
    /// The HTML body of the response.
    pub html: String,
    /// The size of the response body in bytes.
    pub body_size: usize,
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
    pub is_allowed: bool,
    /// The crawl delay from robots.txt, in seconds.
    pub crawl_delay: Option<u64>,
    /// Whether a noindex directive was detected.
    pub noindex_detected: bool,
    /// Whether a nofollow directive was detected.
    pub nofollow_detected: bool,
    /// The X-Robots-Tag header value, if present.
    pub x_robots_tag: Option<String>,
    /// Whether the content is a PDF.
    pub is_pdf: bool,
    /// Whether the page was skipped (binary or PDF content).
    pub was_skipped: bool,
    /// The detected character set encoding.
    pub detected_charset: Option<String>,
    /// Whether main_content_only was active during extraction.
    pub main_content_only: bool,
    /// Whether an authentication header was sent with the request.
    pub auth_header_sent: bool,
    /// Response metadata extracted from HTTP headers.
    pub response_meta: Option<ResponseMeta>,
    /// Downloaded assets from the page.
    pub assets: Vec<DownloadedAsset>,
    /// Whether the page content suggests JavaScript rendering is needed.
    pub js_render_hint: bool,
    /// Whether the browser fallback was used to fetch this page.
    pub browser_used: bool,
}

/// The result of crawling a single page during a crawl operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrawlPageResult {
    /// The original URL of the page.
    pub url: String,
    /// The normalized URL of the page.
    pub normalized_url: String,
    /// The HTTP status code of the response.
    pub status_code: u16,
    /// The Content-Type header value.
    pub content_type: String,
    /// The HTML body of the response.
    pub html: String,
    /// The size of the response body in bytes.
    pub body_size: usize,
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
    pub depth: usize,
    /// Whether this page is on the same domain as the start URL.
    pub stayed_on_domain: bool,
    /// Whether this page was skipped (binary or PDF content).
    pub was_skipped: bool,
    /// Whether the content is a PDF.
    pub is_pdf: bool,
    /// The detected character set encoding.
    pub detected_charset: Option<String>,
}

/// An event emitted during a streaming crawl operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrawlEvent {
    /// A single page has been crawled.
    Page(Box<CrawlPageResult>),
    /// An error occurred while crawling a URL.
    Error {
        /// The URL that failed.
        url: String,
        /// The error message.
        error: String,
    },
    /// The crawl has completed.
    Complete {
        /// Total number of pages crawled.
        pages_crawled: usize,
    },
}

/// The result of a multi-page crawl operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrawlResult {
    /// The list of crawled pages.
    pub pages: Vec<CrawlPageResult>,
    /// The final URL after following redirects.
    pub final_url: String,
    /// The number of redirects followed.
    pub redirect_count: usize,
    /// Whether any page was skipped during crawling.
    pub was_skipped: bool,
    /// An error message, if the crawl encountered an issue.
    pub error: Option<String>,
    /// Cookies collected during the crawl.
    pub cookies: Vec<CookieInfo>,
    /// Internal list of normalized URLs for deduplication counting.
    normalized_urls: Vec<String>,
}

impl CrawlResult {
    /// Create a new `CrawlResult` with the given fields.
    pub(crate) fn new(
        pages: Vec<CrawlPageResult>,
        final_url: String,
        redirect_count: usize,
        was_skipped: bool,
        error: Option<String>,
        cookies: Vec<CookieInfo>,
        normalized_urls: Vec<String>,
    ) -> Self {
        Self {
            pages,
            final_url,
            redirect_count,
            was_skipped,
            error,
            cookies,
            normalized_urls,
        }
    }

    /// Returns the count of unique normalized URLs encountered during crawling.
    pub fn unique_normalized_urls(&self) -> usize {
        let mut unique: HashSet<&str> = HashSet::new();
        for n in &self.normalized_urls {
            unique.insert(n.as_str());
        }
        unique.len()
    }
}

/// A URL entry from a sitemap.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SitemapUrl {
    /// The URL.
    pub url: String,
    /// The last modification date, if present.
    pub lastmod: Option<String>,
    /// The change frequency, if present.
    pub changefreq: Option<String>,
    /// The priority, if present.
    pub priority: Option<String>,
}

/// The result of a map operation, containing discovered URLs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapResult {
    /// The list of discovered URLs.
    pub urls: Vec<SitemapUrl>,
}
