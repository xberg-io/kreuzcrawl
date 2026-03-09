//! Public types used across the kreuzcrawl crate.

use std::collections::{HashMap, HashSet};
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Configuration for crawl, scrape, and map operations.
#[derive(Debug, Clone)]
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
    pub include_paths: Option<Vec<String>>,
    /// Regex patterns for paths to exclude during crawling.
    pub exclude_paths: Option<Vec<String>>,
    /// Custom HTTP headers to send with each request.
    pub custom_headers: Option<HashMap<String, String>>,
    /// Timeout for individual HTTP requests.
    pub request_timeout: Duration,
    /// Maximum number of redirects to follow.
    pub max_redirects: Option<usize>,
    /// Number of retry attempts for failed requests.
    pub retry_count: Option<usize>,
    /// HTTP status codes that should trigger a retry.
    pub retry_codes: Option<Vec<u16>>,
    /// Whether to enable cookie handling.
    pub cookies_enabled: bool,
    /// HTTP Basic authentication credentials (username, password).
    pub auth_basic: Option<(String, String)>,
    /// Bearer token for authentication.
    pub auth_bearer: Option<String>,
    /// Custom authentication header (name, value).
    pub auth_header: Option<(String, String)>,
    /// Maximum response body size in bytes.
    pub max_body_size: Option<usize>,
    /// Whether to extract only the main content from HTML pages.
    pub main_content_only: bool,
    /// CSS selectors for tags to remove from HTML before processing.
    pub remove_tags: Option<Vec<String>>,
    /// Maximum number of URLs to return from a map operation.
    pub map_limit: Option<usize>,
    /// Search filter for map results (case-insensitive substring match on URLs).
    pub map_search: Option<String>,
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
            include_paths: None,
            exclude_paths: None,
            custom_headers: None,
            request_timeout: Duration::from_secs(30),
            max_redirects: None,
            retry_count: None,
            retry_codes: None,
            cookies_enabled: false,
            auth_basic: None,
            auth_bearer: None,
            auth_header: None,
            max_body_size: None,
            main_content_only: false,
            remove_tags: None,
            map_limit: None,
            map_search: None,
        }
    }
}

/// Metadata extracted from an HTML page's `<meta>` tags and `<title>` element.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageMetadata {
    /// The page title from the `<title>` element.
    pub title: Option<String>,
    /// The meta description.
    pub description: Option<String>,
    /// The canonical URL from `<link rel="canonical">`.
    pub canonical_url: Option<String>,
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
}

/// Information about a link found on a page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkInfo {
    /// The resolved URL of the link.
    pub url: String,
    /// The visible text of the link.
    pub text: String,
    /// The classification of the link: "internal", "external", "anchor", or "document".
    pub link_type: String,
    /// The `rel` attribute value, if present.
    pub rel: Option<String>,
    /// Whether the link has `rel="nofollow"`.
    pub nofollow: bool,
}

/// Information about an image found on a page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    /// The image URL.
    pub url: String,
    /// The alt text, if present.
    pub alt: Option<String>,
    /// The width attribute, if present and parseable.
    pub width: Option<u32>,
    /// The height attribute, if present and parseable.
    pub height: Option<u32>,
    /// The source of the image reference: "img", "picture_source", "og:image", or "twitter:image".
    pub source: String,
}

/// The type of a feed (RSS, Atom, or JSON Feed).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FeedType {
    /// RSS feed.
    Rss,
    /// Atom feed.
    Atom,
    /// JSON Feed.
    JsonFeed,
}

/// Information about a feed link found on a page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedInfo {
    /// The feed URL.
    pub url: String,
    /// The feed title, if present.
    pub title: Option<String>,
    /// The type of feed.
    pub feed_type: FeedType,
}

/// A JSON-LD structured data entry found on a page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonLdEntry {
    /// The `@type` value from the JSON-LD object.
    pub schema_type: String,
    /// The `name` value, if present.
    pub name: Option<String>,
    /// The raw JSON-LD string.
    pub raw: String,
}

/// Information about an HTTP cookie received from a response.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone)]
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
}

/// The result of crawling a single page during a crawl operation.
#[derive(Debug, Clone)]
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

/// The result of a multi-page crawl operation.
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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

impl std::ops::Deref for SitemapUrl {
    type Target = str;
    fn deref(&self) -> &str {
        &self.url
    }
}

/// The result of a map operation, containing discovered URLs.
#[derive(Debug, Clone)]
pub struct MapResult {
    /// The list of discovered URLs.
    pub urls: Vec<SitemapUrl>,
}
