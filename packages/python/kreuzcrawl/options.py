"""Configuration options for the conversion API."""

from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum
from typing import Any


class BrowserMode(str, Enum):
    """When to use the headless browser fallback."""

    AUTO = "auto"
    ALWAYS = "always"
    NEVER = "never"


class BrowserWait(str, Enum):
    """Wait strategy for browser page rendering."""

    NETWORKIDLE = "network_idle"
    SELECTOR = "selector"
    FIXED = "fixed"


class AuthConfig(str, Enum):
    """Authentication configuration."""

    BASIC = "basic"
    BEARER = "bearer"
    HEADER = "header"


class LinkType(str, Enum):
    """The classification of a link."""

    INTERNAL = "internal"
    EXTERNAL = "external"
    ANCHOR = "anchor"
    DOCUMENT = "document"


class ImageSource(str, Enum):
    """The source of an image reference."""

    IMG = "img"
    PICTURESOURCE = "picture_source"
    OGIMAGE = "og_image"
    TWITTERIMAGE = "twitter_image"


class FeedType(str, Enum):
    """The type of a feed (RSS, Atom, or JSON Feed)."""

    RSS = "rss"
    ATOM = "atom"
    JSONFEED = "json_feed"


class AssetCategory(str, Enum):
    """The category of a downloaded asset."""

    DOCUMENT = "document"
    IMAGE = "image"
    AUDIO = "audio"
    VIDEO = "video"
    FONT = "font"
    STYLESHEET = "stylesheet"
    SCRIPT = "script"
    ARCHIVE = "archive"
    DATA = "data"
    OTHER = "other"


@dataclass
class ExtractionMeta:
    """Metadata about an LLM extraction pass."""

    cost: float | None = 0.0
    """Estimated cost of the LLM call in USD."""

    prompt_tokens: int | None = 0
    """Number of prompt (input) tokens consumed."""

    completion_tokens: int | None = 0
    """Number of completion (output) tokens generated."""

    model: str | None = ""
    """The model identifier used for extraction."""

    chunks_processed: int = 0
    """Number of content chunks sent to the LLM."""


@dataclass
class ProxyConfig:
    """Proxy configuration for HTTP requests."""

    url: str = ""
    """Proxy URL (e.g. "http://proxy:8080", "socks5://proxy:1080")."""

    username: str | None = ""
    """Optional username for proxy authentication."""

    password: str | None = ""
    """Optional password for proxy authentication."""


@dataclass
class BrowserConfig:
    """Browser fallback configuration."""

    mode: str = "auto"
    """When to use the headless browser fallback."""

    endpoint: str | None = ""
    """CDP WebSocket endpoint for connecting to an external browser instance."""

    timeout: int = 0
    """Timeout for browser page load and rendering (in milliseconds when serialized)."""

    wait: str = "network_idle"
    """Wait strategy after browser navigation."""

    wait_selector: str | None = ""
    """CSS selector to wait for when `wait` is `Selector`."""

    extra_wait: int | None = 0
    """Extra time to wait after the wait condition is met."""


@dataclass
class CrawlConfig:
    """Configuration for crawl, scrape, and map operations."""

    max_depth: int | None = 0
    """Maximum crawl depth (number of link hops from the start URL)."""

    max_pages: int | None = 0
    """Maximum number of pages to crawl."""

    max_concurrent: int | None = 0
    """Maximum number of concurrent requests."""

    respect_robots_txt: bool = False
    """Whether to respect robots.txt directives."""

    user_agent: str | None = ""
    """Custom user-agent string."""

    stay_on_domain: bool = False
    """Whether to restrict crawling to the same domain."""

    allow_subdomains: bool = False
    """Whether to allow subdomains when `stay_on_domain` is true."""

    include_paths: list[str] = field(default_factory=list)
    """Regex patterns for paths to include during crawling."""

    exclude_paths: list[str] = field(default_factory=list)
    """Regex patterns for paths to exclude during crawling."""

    custom_headers: dict[str, str] = field(default_factory=dict)
    """Custom HTTP headers to send with each request."""

    request_timeout: int = 0
    """Timeout for individual HTTP requests (in milliseconds when serialized)."""

    max_redirects: int = 10
    """Maximum number of redirects to follow."""

    retry_count: int = 0
    """Number of retry attempts for failed requests."""

    retry_codes: list[int] = field(default_factory=list)
    """HTTP status codes that should trigger a retry."""

    cookies_enabled: bool = False
    """Whether to enable cookie handling."""

    auth: str | None = "basic"
    """Authentication configuration."""

    max_body_size: int | None = 0
    """Maximum response body size in bytes."""

    main_content_only: bool = False
    """Whether to extract only the main content from HTML pages."""

    remove_tags: list[str] = field(default_factory=list)
    """CSS selectors for tags to remove from HTML before processing."""

    map_limit: int | None = 0
    """Maximum number of URLs to return from a map operation."""

    map_search: str | None = ""
    """Search filter for map results (case-insensitive substring match on URLs)."""

    download_assets: bool = False
    """Whether to download assets (CSS, JS, images, etc.) from the page."""

    asset_types: list[str] = field(default_factory=list)
    """Filter for asset categories to download."""

    max_asset_size: int | None = 0
    """Maximum size in bytes for individual asset downloads."""

    browser: Any = None
    """Browser configuration."""

    proxy: Any | None = None
    """Proxy configuration for HTTP requests."""

    user_agents: list[str] = field(default_factory=list)
    """List of user-agent strings for rotation. If non-empty, overrides `user_agent`."""

    capture_screenshot: bool = False
    """Whether to capture a screenshot when using the browser."""

    download_documents: bool = True
    """Whether to download non-HTML documents (PDF, DOCX, images, code, etc.) instead of skipping them."""

    document_max_size: int | None = 0
    """Maximum size in bytes for document downloads. Defaults to 50 MB."""

    document_mime_types: list[str] = field(default_factory=list)
    """Allowlist of MIME types to download. If empty, uses built-in defaults."""

    warc_output: str | None = ""
    """Path to write WARC output. If `None`, WARC output is disabled."""

    browser_profile: str | None = ""
    """Named browser profile for persistent sessions (cookies, localStorage)."""

    save_browser_profile: bool = False
    """Whether to save changes back to the browser profile on exit."""


@dataclass
class DownloadedDocument:
    """A downloaded non-HTML document (PDF, DOCX, image, code file, etc.)."""

    url: str = ""
    """The URL the document was fetched from."""

    mime_type: str = ""
    """The MIME type from the Content-Type header."""

    content: bytes = b""
    """Raw document bytes. Skipped during JSON serialization."""

    size: int = 0
    """Size of the document in bytes."""

    filename: str | None = ""
    """Filename extracted from Content-Disposition or URL path."""

    content_hash: str = ""
    """SHA-256 hex digest of the content."""

    headers: dict[str, str] = field(default_factory=dict)
    """Selected response headers."""


@dataclass
class InteractionResult:
    """Result of executing a sequence of page interaction actions."""

    action_results: list[Any] = field(default_factory=list)
    """Results from each executed action."""

    final_html: str = ""
    """Final page HTML after all actions completed."""

    final_url: str = ""
    """Final page URL (may have changed due to navigation)."""

    screenshot: bytes | None = b""
    """Screenshot taken after all actions, if requested."""


@dataclass
class ActionResult:
    """Result from a single page action execution."""

    action_index: int = 0
    """Zero-based index of the action in the sequence."""

    action_type: str = ""
    """The type of action that was executed."""

    success: bool = False
    """Whether the action completed successfully."""

    data: str | None = ""
    """Action-specific return data (screenshot bytes, JS return value, scraped HTML)."""

    error: str | None = ""
    """Error message if the action failed."""


@dataclass
class ScrapeResult:
    """The result of a single-page scrape operation."""

    status_code: int = 0
    """The HTTP status code of the response."""

    content_type: str = ""
    """The Content-Type header value."""

    html: str = ""
    """The HTML body of the response."""

    body_size: int = 0
    """The size of the response body in bytes."""

    metadata: Any = None
    """Extracted metadata from the page."""

    links: list[Any] = field(default_factory=list)
    """Links found on the page."""

    images: list[Any] = field(default_factory=list)
    """Images found on the page."""

    feeds: list[Any] = field(default_factory=list)
    """Feed links found on the page."""

    json_ld: list[Any] = field(default_factory=list)
    """JSON-LD entries found on the page."""

    is_allowed: bool = False
    """Whether the URL is allowed by robots.txt."""

    crawl_delay: int | None = 0
    """The crawl delay from robots.txt, in seconds."""

    noindex_detected: bool = False
    """Whether a noindex directive was detected."""

    nofollow_detected: bool = False
    """Whether a nofollow directive was detected."""

    x_robots_tag: str | None = ""
    """The X-Robots-Tag header value, if present."""

    is_pdf: bool = False
    """Whether the content is a PDF."""

    was_skipped: bool = False
    """Whether the page was skipped (binary or PDF content)."""

    detected_charset: str | None = ""
    """The detected character set encoding."""

    main_content_only: bool = False
    """Whether main_content_only was active during extraction."""

    auth_header_sent: bool = False
    """Whether an authentication header was sent with the request."""

    response_meta: Any | None = None
    """Response metadata extracted from HTTP headers."""

    assets: list[Any] = field(default_factory=list)
    """Downloaded assets from the page."""

    js_render_hint: bool = False
    """Whether the page content suggests JavaScript rendering is needed."""

    browser_used: bool = False
    """Whether the browser fallback was used to fetch this page."""

    markdown: Any | None = None
    """Markdown conversion of the page content."""

    extracted_data: str | None = ""
    """Structured data extracted by LLM. Populated when using LlmExtractor."""

    extraction_meta: Any | None = None
    """Metadata about the LLM extraction pass (cost, tokens, model)."""

    screenshot: bytes | None = b""
    """Screenshot of the page as PNG bytes. Populated when browser is used and capture_screenshot is enabled."""

    downloaded_document: Any | None = None
    """Downloaded non-HTML document (PDF, DOCX, image, code, etc.)."""


@dataclass
class CrawlPageResult:
    """The result of crawling a single page during a crawl operation."""

    url: str = ""
    """The original URL of the page."""

    normalized_url: str = ""
    """The normalized URL of the page."""

    status_code: int = 0
    """The HTTP status code of the response."""

    content_type: str = ""
    """The Content-Type header value."""

    html: str = ""
    """The HTML body of the response."""

    body_size: int = 0
    """The size of the response body in bytes."""

    metadata: Any = None
    """Extracted metadata from the page."""

    links: list[Any] = field(default_factory=list)
    """Links found on the page."""

    images: list[Any] = field(default_factory=list)
    """Images found on the page."""

    feeds: list[Any] = field(default_factory=list)
    """Feed links found on the page."""

    json_ld: list[Any] = field(default_factory=list)
    """JSON-LD entries found on the page."""

    depth: int = 0
    """The depth of this page from the start URL."""

    stayed_on_domain: bool = False
    """Whether this page is on the same domain as the start URL."""

    was_skipped: bool = False
    """Whether this page was skipped (binary or PDF content)."""

    is_pdf: bool = False
    """Whether the content is a PDF."""

    detected_charset: str | None = ""
    """The detected character set encoding."""

    markdown: Any | None = None
    """Markdown conversion of the page content."""

    extracted_data: str | None = ""
    """Structured data extracted by LLM. Populated when using LlmExtractor."""

    extraction_meta: Any | None = None
    """Metadata about the LLM extraction pass (cost, tokens, model)."""

    downloaded_document: Any | None = None
    """Downloaded non-HTML document (PDF, DOCX, image, code, etc.)."""


@dataclass
class CrawlResult:
    """The result of a multi-page crawl operation."""

    pages: list[Any] = field(default_factory=list)
    """The list of crawled pages."""

    final_url: str = ""
    """The final URL after following redirects."""

    redirect_count: int = 0
    """The number of redirects followed."""

    was_skipped: bool = False
    """Whether any page was skipped during crawling."""

    error: str | None = ""
    """An error message, if the crawl encountered an issue."""

    cookies: list[Any] = field(default_factory=list)
    """Cookies collected during the crawl."""

    normalized_urls: list[str] = field(default_factory=list)
    """Normalized URLs encountered during crawling (for deduplication counting)."""


@dataclass
class SitemapUrl:
    """A URL entry from a sitemap."""

    url: str = ""
    """The URL."""

    lastmod: str | None = ""
    """The last modification date, if present."""

    changefreq: str | None = ""
    """The change frequency, if present."""

    priority: str | None = ""
    """The priority, if present."""


@dataclass
class MapResult:
    """The result of a map operation, containing discovered URLs."""

    urls: list[Any] = field(default_factory=list)
    """The list of discovered URLs."""


@dataclass
class MarkdownResult:
    """Rich markdown conversion result from HTML processing."""

    content: str = ""
    """Converted markdown text."""

    document_structure: str | None = ""
    """Structured document tree with semantic nodes."""

    tables: list[str] = field(default_factory=list)
    """Extracted tables with structured cell data."""

    warnings: list[str] = field(default_factory=list)
    """Non-fatal processing warnings."""

    citations: Any | None = None
    """Content with links replaced by numbered citations."""

    fit_content: str | None = ""
    """Content-filtered markdown optimized for LLM consumption."""


@dataclass
class CachedPage:
    """Cached page data for HTTP response caching."""

    url: str = ""
    status_code: int = 0
    content_type: str = ""
    body: str = ""
    etag: str | None = ""
    last_modified: str | None = ""
    cached_at: int = 0

@dataclass
class LinkInfo:
    """Information about a link found on a page."""

    url: str = ""
    """The resolved URL of the link."""

    text: str = ""
    """The visible text of the link."""

    link_type: str = "internal"
    """The classification of the link."""

    rel: str | None = ""
    """The `rel` attribute value, if present."""

    nofollow: bool = False
    """Whether the link has `rel="nofollow"`."""


@dataclass
class ImageInfo:
    """Information about an image found on a page."""

    url: str = ""
    """The image URL."""

    alt: str | None = ""
    """The alt text, if present."""

    width: int | None = 0
    """The width attribute, if present and parseable."""

    height: int | None = 0
    """The height attribute, if present and parseable."""

    source: str = "img"
    """The source of the image reference."""


@dataclass
class FeedInfo:
    """Information about a feed link found on a page."""

    url: str = ""
    """The feed URL."""

    title: str | None = ""
    """The feed title, if present."""

    feed_type: str = "rss"
    """The type of feed."""


@dataclass
class JsonLdEntry:
    """A JSON-LD structured data entry found on a page."""

    schema_type: str = ""
    """The `@type` value from the JSON-LD object."""

    name: str | None = ""
    """The `name` value, if present."""

    raw: str = ""
    """The raw JSON-LD string."""


@dataclass
class CookieInfo:
    """Information about an HTTP cookie received from a response."""

    name: str = ""
    """The cookie name."""

    value: str = ""
    """The cookie value."""

    domain: str | None = ""
    """The cookie domain, if specified."""

    path: str | None = ""
    """The cookie path, if specified."""


@dataclass
class DownloadedAsset:
    """A downloaded asset from a page."""

    url: str = ""
    """The original URL of the asset."""

    content_hash: str = ""
    """The SHA-256 content hash of the asset."""

    mime_type: str | None = ""
    """The MIME type from the Content-Type header."""

    size: int = 0
    """The size of the asset in bytes."""

    asset_category: str = "image"
    """The category of the asset."""

    html_tag: str | None = ""
    """The HTML tag that referenced this asset (e.g., "link", "script", "img")."""


@dataclass
class ArticleMetadata:
    """Article metadata extracted from `article:*` Open Graph tags."""

    published_time: str | None = ""
    """The article publication time."""

    modified_time: str | None = ""
    """The article modification time."""

    author: str | None = ""
    """The article author."""

    section: str | None = ""
    """The article section."""

    tags: list[str] = field(default_factory=list)
    """The article tags."""


@dataclass
class HreflangEntry:
    """An hreflang alternate link entry."""

    lang: str = ""
    """The language code (e.g., "en", "fr", "x-default")."""

    url: str = ""
    """The URL for this language variant."""


@dataclass
class FaviconInfo:
    """Information about a favicon or icon link."""

    url: str = ""
    """The icon URL."""

    rel: str = ""
    """The `rel` attribute (e.g., "icon", "apple-touch-icon")."""

    sizes: str | None = ""
    """The `sizes` attribute, if present."""

    mime_type: str | None = ""
    """The MIME type, if present."""


@dataclass
class HeadingInfo:
    """A heading element extracted from the page."""

    level: int = 0
    """The heading level (1-6)."""

    text: str = ""
    """The heading text content."""


@dataclass
class ResponseMeta:
    """Response metadata extracted from HTTP headers."""

    etag: str | None = ""
    """The ETag header value."""

    last_modified: str | None = ""
    """The Last-Modified header value."""

    cache_control: str | None = ""
    """The Cache-Control header value."""

    server: str | None = ""
    """The Server header value."""

    x_powered_by: str | None = ""
    """The X-Powered-By header value."""

    content_language: str | None = ""
    """The Content-Language header value."""

    content_encoding: str | None = ""
    """The Content-Encoding header value."""


@dataclass
class PageMetadata:
    """Metadata extracted from an HTML page's `<meta>` tags and `<title>` element."""

    title: str | None = ""
    """The page title from the `<title>` element."""

    description: str | None = ""
    """The meta description."""

    canonical_url: str | None = ""
    """The canonical URL from `<link rel="canonical">`."""

    keywords: str | None = ""
    """Keywords from `<meta name="keywords">`."""

    author: str | None = ""
    """Author from `<meta name="author">`."""

    viewport: str | None = ""
    """Viewport content from `<meta name="viewport">`."""

    theme_color: str | None = ""
    """Theme color from `<meta name="theme-color">`."""

    generator: str | None = ""
    """Generator from `<meta name="generator">`."""

    robots: str | None = ""
    """Robots content from `<meta name="robots">`."""

    html_lang: str | None = ""
    """The `lang` attribute from the `<html>` element."""

    html_dir: str | None = ""
    """The `dir` attribute from the `<html>` element."""

    og_title: str | None = ""
    """Open Graph title."""

    og_type: str | None = ""
    """Open Graph type."""

    og_image: str | None = ""
    """Open Graph image URL."""

    og_description: str | None = ""
    """Open Graph description."""

    og_url: str | None = ""
    """Open Graph URL."""

    og_site_name: str | None = ""
    """Open Graph site name."""

    og_locale: str | None = ""
    """Open Graph locale."""

    og_video: str | None = ""
    """Open Graph video URL."""

    og_audio: str | None = ""
    """Open Graph audio URL."""

    og_locale_alternates: list[str] | None = field(default_factory=list)
    """Open Graph locale alternates."""

    twitter_card: str | None = ""
    """Twitter card type."""

    twitter_title: str | None = ""
    """Twitter title."""

    twitter_description: str | None = ""
    """Twitter description."""

    twitter_image: str | None = ""
    """Twitter image URL."""

    twitter_site: str | None = ""
    """Twitter site handle."""

    twitter_creator: str | None = ""
    """Twitter creator handle."""

    dc_title: str | None = ""
    """Dublin Core title."""

    dc_creator: str | None = ""
    """Dublin Core creator."""

    dc_subject: str | None = ""
    """Dublin Core subject."""

    dc_description: str | None = ""
    """Dublin Core description."""

    dc_publisher: str | None = ""
    """Dublin Core publisher."""

    dc_date: str | None = ""
    """Dublin Core date."""

    dc_type: str | None = ""
    """Dublin Core type."""

    dc_format: str | None = ""
    """Dublin Core format."""

    dc_identifier: str | None = ""
    """Dublin Core identifier."""

    dc_language: str | None = ""
    """Dublin Core language."""

    dc_rights: str | None = ""
    """Dublin Core rights."""

    article: Any | None = None
    """Article metadata from `article:*` Open Graph tags."""

    hreflangs: list[Any] | None = field(default_factory=list)
    """Hreflang alternate links."""

    favicons: list[Any] | None = field(default_factory=list)
    """Favicon and icon links."""

    headings: list[Any] | None = field(default_factory=list)
    """Heading elements (h1-h6)."""

    word_count: int | None = 0
    """Computed word count of the page body text."""


@dataclass
class CitationResult:
    """Result of citation conversion."""

    content: str = ""
    """Markdown with links replaced by numbered citations."""

    references: list[Any] = field(default_factory=list)
    """Numbered reference list: (index, url, text)."""


@dataclass
class CitationReference:
    index: int = 0
    url: str = ""
    text: str = ""

@dataclass
class BatchScrapeResult:
    """Result from a single URL in a batch scrape operation."""

    url: str = ""
    """The URL that was scraped."""

    result: Any | None = None
    """The scrape result, if successful."""

    error: str | None = ""
    """The error message, if the scrape failed."""


@dataclass
class BatchCrawlResult:
    """Result from a single URL in a batch crawl operation."""

    url: str = ""
    """The seed URL that was crawled."""

    result: Any | None = None
    """The crawl result, if successful."""

    error: str | None = ""
    """The error message, if the crawl failed."""


