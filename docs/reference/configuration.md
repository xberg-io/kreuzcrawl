---
title: "Configuration Reference"
---

## Configuration Reference

This page documents all configuration types and their defaults across all languages.

### ExtractionMeta

Metadata about an LLM extraction pass.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cost` | `float | None` | `None` | Estimated cost of the LLM call in USD. |
| `prompt_tokens` | `int | None` | `None` | Number of prompt (input) tokens consumed. |
| `completion_tokens` | `int | None` | `None` | Number of completion (output) tokens generated. |
| `model` | `str | None` | `None` | The model identifier used for extraction. |
| `chunks_processed` | `int` | — | Number of content chunks sent to the LLM. |

---

### ProxyConfig

Proxy configuration for HTTP requests.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str` | — | Proxy URL (e.g. "<http://proxy:8080",> "socks5://proxy:1080"). |
| `username` | `str | None` | `None` | Optional username for proxy authentication. |
| `password` | `str | None` | `None` | Optional password for proxy authentication. |

---

### ContentConfig

Content extraction and conversion configuration.

Controls how HTML is converted to the output format. Uses
html-to-markdown-rs as the conversion engine for all formats
(markdown, plain text, djot).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `output_format` | `str` | `"markdown"` | Output format: `"markdown"` (default), `"plain"`, `"djot"`. |
| `preprocessing_preset` | `str` | `"standard"` | Preprocessing aggressiveness: `"minimal"`, `"standard"` (default), `"aggressive"`. - Minimal: only scripts/styles removed. - Standard: also removes nav, nav-hinted headers/footers/asides, forms. - Aggressive: removes all footers/asides unconditionally. |
| `remove_navigation` | `bool` | `True` | Remove navigation elements (nav, breadcrumbs, menus). Default: `True`. |
| `remove_forms` | `bool` | `True` | Remove form elements. Default: `True`. |
| `strip_tags` | `list[str]` | `[]` | HTML tag names to strip (render children only, remove the tag wrapper). Default: `["noscript"]`. |
| `preserve_tags` | `list[str]` | `[]` | HTML tag names to preserve as raw HTML in output. |
| `exclude_selectors` | `list[str]` | `[]` | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the wrapper but keeps children), excluded elements and all descendants are dropped. Supports CSS selectors: `.class`, `#id`, `[attribute]`, compound selectors. Example: `[".cookie-banner", "#ad-container", "[role='complementary']"]` |
| `skip_images` | `bool` | `False` | Skip image elements in output. Default: `False`. |
| `max_depth` | `int | None` | `None` | Max DOM traversal depth. Prevents stack overflow on deeply nested HTML. |
| `wrap` | `bool` | `False` | Enable line wrapping. Default: `False`. |
| `wrap_width` | `int` | `80` | Wrap width when `wrap` is enabled. Default: `80`. |
| `include_document_structure` | `bool` | `True` | Include document structure tree in output. Default: `True`. |

---

### BrowserConfig

Browser fallback configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `mode` | `BrowserMode` | `BrowserMode.AUTO` | When to use the headless browser fallback. |
| `endpoint` | `str | None` | `None` | CDP WebSocket endpoint for connecting to an external browser instance. |
| `timeout` | `float` | `30000ms` | Timeout for browser page load and rendering (in milliseconds when serialized). |
| `wait` | `BrowserWait` | `BrowserWait.NETWORK_IDLE` | Wait strategy after browser navigation. |
| `wait_selector` | `str | None` | `None` | CSS selector to wait for when `wait` is `Selector`. |
| `extra_wait` | `float | None` | `None` | Extra time to wait after the wait condition is met. |

---

### CrawlConfig

Configuration for crawl, scrape, and map operations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_depth` | `int | None` | `None` | Maximum crawl depth (number of link hops from the start URL). |
| `max_pages` | `int | None` | `None` | Maximum number of pages to crawl. |
| `max_concurrent` | `int | None` | `None` | Maximum number of concurrent requests. |
| `respect_robots_txt` | `bool` | `False` | Whether to respect robots.txt directives. |
| `user_agent` | `str | None` | `None` | Custom user-agent string. |
| `stay_on_domain` | `bool` | `False` | Whether to restrict crawling to the same domain. |
| `allow_subdomains` | `bool` | `False` | Whether to allow subdomains when `stay_on_domain` is true. |
| `include_paths` | `list[str]` | `[]` | Regex patterns for paths to include during crawling. |
| `exclude_paths` | `list[str]` | `[]` | Regex patterns for paths to exclude during crawling. |
| `custom_headers` | `dict[str, str]` | `{}` | Custom HTTP headers to send with each request. |
| `request_timeout` | `float` | `30000ms` | Timeout for individual HTTP requests (in milliseconds when serialized). |
| `rate_limit_ms` | `int | None` | `None` | Per-domain rate limit in milliseconds. When set, enforces a minimum delay between requests to the same domain. Defaults to 200ms when `None`. |
| `max_redirects` | `int` | `10` | Maximum number of redirects to follow. |
| `retry_count` | `int` | `0` | Number of retry attempts for failed requests. |
| `retry_codes` | `list[int]` | `[]` | HTTP status codes that should trigger a retry. |
| `cookies_enabled` | `bool` | `False` | Whether to enable cookie handling. |
| `auth` | `AuthConfig | None` | `None` | Authentication configuration. |
| `max_body_size` | `int | None` | `None` | Maximum response body size in bytes. |
| `remove_tags` | `list[str]` | `[]` | CSS selectors for tags to remove from HTML before processing. |
| `content` | `ContentConfig` | — | Content extraction and conversion configuration. |
| `map_limit` | `int | None` | `None` | Maximum number of URLs to return from a map operation. |
| `map_search` | `str | None` | `None` | Search filter for map results (case-insensitive substring match on URLs). |
| `download_assets` | `bool` | `False` | Whether to download assets (CSS, JS, images, etc.) from the page. |
| `asset_types` | `list[AssetCategory]` | `[]` | Filter for asset categories to download. |
| `max_asset_size` | `int | None` | `None` | Maximum size in bytes for individual asset downloads. |
| `browser` | `BrowserConfig` | — | Browser configuration. |
| `proxy` | `ProxyConfig | None` | `None` | Proxy configuration for HTTP requests. |
| `user_agents` | `list[str]` | `[]` | List of user-agent strings for rotation. If non-empty, overrides `user_agent`. |
| `capture_screenshot` | `bool` | `False` | Whether to capture a screenshot when using the browser. |
| `download_documents` | `bool` | `True` | Whether to download non-HTML documents (PDF, DOCX, images, code, etc.) instead of skipping them. |
| `document_max_size` | `int | None` | `None` | Maximum size in bytes for document downloads. Defaults to 50 MB. |
| `document_mime_types` | `list[str]` | `[]` | Allowlist of MIME types to download. If empty, uses built-in defaults. |
| `warc_output` | `str | None` | `None` | Path to write WARC output. If `None`, WARC output is disabled. |
| `browser_profile` | `str | None` | `None` | Named browser profile for persistent sessions (cookies, localStorage). |
| `save_browser_profile` | `bool` | `False` | Whether to save changes back to the browser profile on exit. |

---

### DownloadedDocument

A downloaded non-HTML document (PDF, DOCX, image, code file, etc.).

When the crawler encounters non-HTML content and `download_documents` is
enabled, it downloads the raw bytes and populates this struct instead of
skipping the resource.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str` | — | The URL the document was fetched from. |
| `mime_type` | `str` | — | The MIME type from the Content-Type header. |
| `content` | `bytes` | — | Raw document bytes. Skipped during JSON serialization. |
| `size` | `int` | — | Size of the document in bytes. |
| `filename` | `str | None` | `None` | Filename extracted from Content-Disposition or URL path. |
| `content_hash` | `str` | — | SHA-256 hex digest of the content. |
| `headers` | `dict[str, str]` | `{}` | Selected response headers. |

---

### ScrapeResult

The result of a single-page scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `status_code` | `int` | — | The HTTP status code of the response. |
| `content_type` | `str` | — | The Content-Type header value. |
| `html` | `str` | — | The HTML body of the response. |
| `body_size` | `int` | — | The size of the response body in bytes. |
| `metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `links` | `list[LinkInfo]` | `[]` | Links found on the page. |
| `images` | `list[ImageInfo]` | `[]` | Images found on the page. |
| `feeds` | `list[FeedInfo]` | `[]` | Feed links found on the page. |
| `json_ld` | `list[JsonLdEntry]` | `[]` | JSON-LD entries found on the page. |
| `is_allowed` | `bool` | — | Whether the URL is allowed by robots.txt. |
| `crawl_delay` | `int | None` | `None` | The crawl delay from robots.txt, in seconds. |
| `noindex_detected` | `bool` | — | Whether a noindex directive was detected. |
| `nofollow_detected` | `bool` | — | Whether a nofollow directive was detected. |
| `x_robots_tag` | `str | None` | `None` | The X-Robots-Tag header value, if present. |
| `is_pdf` | `bool` | — | Whether the content is a PDF. |
| `was_skipped` | `bool` | — | Whether the page was skipped (binary or PDF content). |
| `detected_charset` | `str | None` | `None` | The detected character set encoding. |
| `auth_header_sent` | `bool` | — | Whether an authentication header was sent with the request. |
| `response_meta` | `ResponseMeta | None` | `None` | Response metadata extracted from HTTP headers. |
| `assets` | `list[DownloadedAsset]` | `[]` | Downloaded assets from the page. |
| `js_render_hint` | `bool` | — | Whether the page content suggests JavaScript rendering is needed. |
| `browser_used` | `bool` | — | Whether the browser fallback was used to fetch this page. |
| `markdown` | `MarkdownResult | None` | `None` | Markdown conversion of the page content. |
| `extracted_data` | `dict[str, Any] | None` | `None` | Structured data extracted by LLM. Populated when extraction is configured. |
| `extraction_meta` | `ExtractionMeta | None` | `None` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `screenshot` | `bytes | None` | `None` | Screenshot of the page as PNG bytes. Populated when browser is used and capture_screenshot is enabled. |
| `downloaded_document` | `DownloadedDocument | None` | `None` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |

---

### CrawlPageResult

The result of crawling a single page during a crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str` | — | The original URL of the page. |
| `normalized_url` | `str` | — | The normalized URL of the page. |
| `status_code` | `int` | — | The HTTP status code of the response. |
| `content_type` | `str` | — | The Content-Type header value. |
| `html` | `str` | — | The HTML body of the response. |
| `body_size` | `int` | — | The size of the response body in bytes. |
| `metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `links` | `list[LinkInfo]` | `[]` | Links found on the page. |
| `images` | `list[ImageInfo]` | `[]` | Images found on the page. |
| `feeds` | `list[FeedInfo]` | `[]` | Feed links found on the page. |
| `json_ld` | `list[JsonLdEntry]` | `[]` | JSON-LD entries found on the page. |
| `depth` | `int` | — | The depth of this page from the start URL. |
| `stayed_on_domain` | `bool` | — | Whether this page is on the same domain as the start URL. |
| `was_skipped` | `bool` | — | Whether this page was skipped (binary or PDF content). |
| `is_pdf` | `bool` | — | Whether the content is a PDF. |
| `detected_charset` | `str | None` | `None` | The detected character set encoding. |
| `markdown` | `MarkdownResult | None` | `None` | Markdown conversion of the page content. |
| `extracted_data` | `dict[str, Any] | None` | `None` | Structured data extracted by LLM. Populated when extraction is configured. |
| `extraction_meta` | `ExtractionMeta | None` | `None` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `downloaded_document` | `DownloadedDocument | None` | `None` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |

---

### CrawlResult

The result of a multi-page crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pages` | `list[CrawlPageResult]` | `[]` | The list of crawled pages. |
| `final_url` | `str` | — | The final URL after following redirects. |
| `redirect_count` | `int` | — | The number of redirects followed. |
| `was_skipped` | `bool` | — | Whether any page was skipped during crawling. |
| `error` | `str | None` | `None` | An error message, if the crawl encountered an issue. |
| `cookies` | `list[CookieInfo]` | `[]` | Cookies collected during the crawl. |
| `normalized_urls` | `list[str]` | `[]` | Normalized URLs encountered during crawling (for deduplication counting). |

---

### SitemapUrl

A URL entry from a sitemap.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str` | — | The URL. |
| `lastmod` | `str | None` | `None` | The last modification date, if present. |
| `changefreq` | `str | None` | `None` | The change frequency, if present. |
| `priority` | `str | None` | `None` | The priority, if present. |

---

### MapResult

The result of a map operation, containing discovered URLs.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `urls` | `list[SitemapUrl]` | `[]` | The list of discovered URLs. |

---

### MarkdownResult

Rich markdown conversion result from HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str` | — | Converted markdown text. |
| `document_structure` | `dict[str, Any] | None` | `None` | Structured document tree with semantic nodes. |
| `tables` | `list[dict[str, Any]]` | `[]` | Extracted tables with structured cell data. |
| `warnings` | `list[str]` | `[]` | Non-fatal processing warnings. |
| `citations` | `CitationResult | None` | `None` | Content with links replaced by numbered citations. |
| `fit_content` | `str | None` | `None` | Content-filtered markdown optimized for LLM consumption. |

---

### LinkInfo

Information about a link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str` | — | The resolved URL of the link. |
| `text` | `str` | — | The visible text of the link. |
| `link_type` | `LinkType` | `LinkType.INTERNAL` | The classification of the link. |
| `rel` | `str | None` | `None` | The `rel` attribute value, if present. |
| `nofollow` | `bool` | — | Whether the link has `rel="nofollow"`. |

---

### ImageInfo

Information about an image found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str` | — | The image URL. |
| `alt` | `str | None` | `None` | The alt text, if present. |
| `width` | `int | None` | `None` | The width attribute, if present and parseable. |
| `height` | `int | None` | `None` | The height attribute, if present and parseable. |
| `source` | `ImageSource` | `ImageSource.IMG` | The source of the image reference. |

---

### FeedInfo

Information about a feed link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str` | — | The feed URL. |
| `title` | `str | None` | `None` | The feed title, if present. |
| `feed_type` | `FeedType` | `FeedType.RSS` | The type of feed. |

---

### JsonLdEntry

A JSON-LD structured data entry found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `schema_type` | `str` | — | The `@type` value from the JSON-LD object. |
| `name` | `str | None` | `None` | The `name` value, if present. |
| `raw` | `str` | — | The raw JSON-LD string. |

---

### CookieInfo

Information about an HTTP cookie received from a response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `str` | — | The cookie name. |
| `value` | `str` | — | The cookie value. |
| `domain` | `str | None` | `None` | The cookie domain, if specified. |
| `path` | `str | None` | `None` | The cookie path, if specified. |

---

### DownloadedAsset

A downloaded asset from a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str` | — | The original URL of the asset. |
| `content_hash` | `str` | — | The SHA-256 content hash of the asset. |
| `mime_type` | `str | None` | `None` | The MIME type from the Content-Type header. |
| `size` | `int` | — | The size of the asset in bytes. |
| `asset_category` | `AssetCategory` | `AssetCategory.IMAGE` | The category of the asset. |
| `html_tag` | `str | None` | `None` | The HTML tag that referenced this asset (e.g., "link", "script", "img"). |

---

### ArticleMetadata

Article metadata extracted from `article:*` Open Graph tags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `published_time` | `str | None` | `None` | The article publication time. |
| `modified_time` | `str | None` | `None` | The article modification time. |
| `author` | `str | None` | `None` | The article author. |
| `section` | `str | None` | `None` | The article section. |
| `tags` | `list[str]` | `[]` | The article tags. |

---

### HreflangEntry

An hreflang alternate link entry.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `lang` | `str` | — | The language code (e.g., "en", "fr", "x-default"). |
| `url` | `str` | — | The URL for this language variant. |

---

### FaviconInfo

Information about a favicon or icon link.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str` | — | The icon URL. |
| `rel` | `str` | — | The `rel` attribute (e.g., "icon", "apple-touch-icon"). |
| `sizes` | `str | None` | `None` | The `sizes` attribute, if present. |
| `mime_type` | `str | None` | `None` | The MIME type, if present. |

---

### HeadingInfo

A heading element extracted from the page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `int` | — | The heading level (1-6). |
| `text` | `str` | — | The heading text content. |

---

### ResponseMeta

Response metadata extracted from HTTP headers.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `etag` | `str | None` | `None` | The ETag header value. |
| `last_modified` | `str | None` | `None` | The Last-Modified header value. |
| `cache_control` | `str | None` | `None` | The Cache-Control header value. |
| `server` | `str | None` | `None` | The Server header value. |
| `x_powered_by` | `str | None` | `None` | The X-Powered-By header value. |
| `content_language` | `str | None` | `None` | The Content-Language header value. |
| `content_encoding` | `str | None` | `None` | The Content-Encoding header value. |

---

### PageMetadata

Metadata extracted from an HTML page's `<meta>` tags and `<title>` element.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `str | None` | `None` | The page title from the `<title>` element. |
| `description` | `str | None` | `None` | The meta description. |
| `canonical_url` | `str | None` | `None` | The canonical URL from `<link rel="canonical">`. |
| `keywords` | `str | None` | `None` | Keywords from `<meta name="keywords">`. |
| `author` | `str | None` | `None` | Author from `<meta name="author">`. |
| `viewport` | `str | None` | `None` | Viewport content from `<meta name="viewport">`. |
| `theme_color` | `str | None` | `None` | Theme color from `<meta name="theme-color">`. |
| `generator` | `str | None` | `None` | Generator from `<meta name="generator">`. |
| `robots` | `str | None` | `None` | Robots content from `<meta name="robots">`. |
| `html_lang` | `str | None` | `None` | The `lang` attribute from the `<html>` element. |
| `html_dir` | `str | None` | `None` | The `dir` attribute from the `<html>` element. |
| `og_title` | `str | None` | `None` | Open Graph title. |
| `og_type` | `str | None` | `None` | Open Graph type. |
| `og_image` | `str | None` | `None` | Open Graph image URL. |
| `og_description` | `str | None` | `None` | Open Graph description. |
| `og_url` | `str | None` | `None` | Open Graph URL. |
| `og_site_name` | `str | None` | `None` | Open Graph site name. |
| `og_locale` | `str | None` | `None` | Open Graph locale. |
| `og_video` | `str | None` | `None` | Open Graph video URL. |
| `og_audio` | `str | None` | `None` | Open Graph audio URL. |
| `og_locale_alternates` | `list[str] | None` | `[]` | Open Graph locale alternates. |
| `twitter_card` | `str | None` | `None` | Twitter card type. |
| `twitter_title` | `str | None` | `None` | Twitter title. |
| `twitter_description` | `str | None` | `None` | Twitter description. |
| `twitter_image` | `str | None` | `None` | Twitter image URL. |
| `twitter_site` | `str | None` | `None` | Twitter site handle. |
| `twitter_creator` | `str | None` | `None` | Twitter creator handle. |
| `dc_title` | `str | None` | `None` | Dublin Core title. |
| `dc_creator` | `str | None` | `None` | Dublin Core creator. |
| `dc_subject` | `str | None` | `None` | Dublin Core subject. |
| `dc_description` | `str | None` | `None` | Dublin Core description. |
| `dc_publisher` | `str | None` | `None` | Dublin Core publisher. |
| `dc_date` | `str | None` | `None` | Dublin Core date. |
| `dc_type` | `str | None` | `None` | Dublin Core type. |
| `dc_format` | `str | None` | `None` | Dublin Core format. |
| `dc_identifier` | `str | None` | `None` | Dublin Core identifier. |
| `dc_language` | `str | None` | `None` | Dublin Core language. |
| `dc_rights` | `str | None` | `None` | Dublin Core rights. |
| `article` | `ArticleMetadata | None` | `None` | Article metadata from `article:*` Open Graph tags. |
| `hreflangs` | `list[HreflangEntry] | None` | `[]` | Hreflang alternate links. |
| `favicons` | `list[FaviconInfo] | None` | `[]` | Favicon and icon links. |
| `headings` | `list[HeadingInfo] | None` | `[]` | Heading elements (h1-h6). |
| `word_count` | `int | None` | `None` | Computed word count of the page body text. |

---

### CitationResult

Result of citation conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str` | — | Markdown with links replaced by numbered citations. |
| `references` | `list[CitationReference]` | `[]` | Numbered reference list: (index, url, text). |

---

### CitationReference

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `int` | — | Index |
| `url` | `str` | — | Url |
| `text` | `str` | — | Text |

---

### BatchScrapeResult

Result from a single URL in a batch scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str` | — | The URL that was scraped. |
| `result` | `ScrapeResult | None` | `None` | The scrape result, if successful. |
| `error` | `str | None` | `None` | The error message, if the scrape failed. |

---

### BatchCrawlResult

Result from a single URL in a batch crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `str` | — | The seed URL that was crawled. |
| `result` | `CrawlResult | None` | `None` | The crawl result, if successful. |
| `error` | `str | None` | `None` | The error message, if the crawl failed. |

---
