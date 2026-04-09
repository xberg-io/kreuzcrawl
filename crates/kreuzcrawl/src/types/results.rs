use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{
    CookieInfo, DownloadedAsset, ExtractionMeta, FeedInfo, ImageInfo, JsonLdEntry, LinkInfo,
    PageMetadata, ResponseMeta,
};

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
    /// Markdown conversion of the page content.
    pub markdown: Option<MarkdownResult>,
    /// Structured data extracted by LLM. Populated when using LlmExtractor.
    pub extracted_data: Option<serde_json::Value>,
    /// Metadata about the LLM extraction pass (cost, tokens, model).
    pub extraction_meta: Option<ExtractionMeta>,
    /// Screenshot of the page as PNG bytes. Populated when browser is used and capture_screenshot is enabled.
    #[serde(skip)]
    pub screenshot: Option<Vec<u8>>,
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
    /// Markdown conversion of the page content.
    pub markdown: Option<MarkdownResult>,
    /// Structured data extracted by LLM. Populated when using LlmExtractor.
    pub extracted_data: Option<serde_json::Value>,
    /// Metadata about the LLM extraction pass (cost, tokens, model).
    pub extraction_meta: Option<ExtractionMeta>,
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

/// Rich markdown conversion result from HTML processing.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarkdownResult {
    /// Converted markdown text.
    pub content: String,
    /// Structured document tree with semantic nodes.
    pub document_structure: Option<serde_json::Value>,
    /// Extracted tables with structured cell data.
    pub tables: Vec<serde_json::Value>,
    /// Non-fatal processing warnings.
    pub warnings: Vec<String>,
    /// Content with links replaced by numbered citations.
    pub citations: Option<crate::citations::CitationResult>,
    /// Content-filtered markdown optimized for LLM consumption.
    pub fit_content: Option<String>,
}

/// Cached page data for HTTP response caching.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedPage {
    pub url: String,
    pub status_code: u16,
    pub content_type: String,
    pub body: String,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub cached_at: u64,
}
