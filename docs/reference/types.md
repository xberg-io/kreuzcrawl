---
title: "Types Reference"
---

## Types Reference

All types defined by the library, grouped by category. Types are shown using Rust as the canonical representation.

### Result Types

#### ScrapeResult

The result of a single-page scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `status_code` | `u16` | — | The HTTP status code of the response. |
| `content_type` | `String` | — | The Content-Type header value. |
| `html` | `String` | — | The HTML body of the response. |
| `body_size` | `usize` | — | The size of the response body in bytes. |
| `metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `links` | `Vec<LinkInfo>` | `vec![]` | Links found on the page. |
| `images` | `Vec<ImageInfo>` | `vec![]` | Images found on the page. |
| `feeds` | `Vec<FeedInfo>` | `vec![]` | Feed links found on the page. |
| `json_ld` | `Vec<JsonLdEntry>` | `vec![]` | JSON-LD entries found on the page. |
| `is_allowed` | `bool` | — | Whether the URL is allowed by robots.txt. |
| `crawl_delay` | `Option<u64>` | `Default::default()` | The crawl delay from robots.txt, in seconds. |
| `noindex_detected` | `bool` | — | Whether a noindex directive was detected. |
| `nofollow_detected` | `bool` | — | Whether a nofollow directive was detected. |
| `x_robots_tag` | `Option<String>` | `Default::default()` | The X-Robots-Tag header value, if present. |
| `is_pdf` | `bool` | — | Whether the content is a PDF. |
| `was_skipped` | `bool` | — | Whether the page was skipped (binary or PDF content). |
| `detected_charset` | `Option<String>` | `Default::default()` | The detected character set encoding. |
| `auth_header_sent` | `bool` | — | Whether an authentication header was sent with the request. |
| `response_meta` | `Option<ResponseMeta>` | `Default::default()` | Response metadata extracted from HTTP headers. |
| `assets` | `Vec<DownloadedAsset>` | `vec![]` | Downloaded assets from the page. |
| `js_render_hint` | `bool` | — | Whether the page content suggests JavaScript rendering is needed. |
| `browser_used` | `bool` | — | Whether the browser fallback was used to fetch this page. |
| `markdown` | `Option<MarkdownResult>` | `Default::default()` | Markdown conversion of the page content. |
| `extracted_data` | `Option<serde_json::Value>` | `Default::default()` | Structured data extracted by LLM. Populated when extraction is configured. |
| `extraction_meta` | `Option<ExtractionMeta>` | `Default::default()` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `screenshot` | `Option<Vec<u8>>` | `Default::default()` | Screenshot of the page as PNG bytes. Populated when browser is used and capture_screenshot is enabled. |
| `downloaded_document` | `Option<DownloadedDocument>` | `Default::default()` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |

---

#### CrawlPageResult

The result of crawling a single page during a crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The original URL of the page. |
| `normalized_url` | `String` | — | The normalized URL of the page. |
| `status_code` | `u16` | — | The HTTP status code of the response. |
| `content_type` | `String` | — | The Content-Type header value. |
| `html` | `String` | — | The HTML body of the response. |
| `body_size` | `usize` | — | The size of the response body in bytes. |
| `metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `links` | `Vec<LinkInfo>` | `vec![]` | Links found on the page. |
| `images` | `Vec<ImageInfo>` | `vec![]` | Images found on the page. |
| `feeds` | `Vec<FeedInfo>` | `vec![]` | Feed links found on the page. |
| `json_ld` | `Vec<JsonLdEntry>` | `vec![]` | JSON-LD entries found on the page. |
| `depth` | `usize` | — | The depth of this page from the start URL. |
| `stayed_on_domain` | `bool` | — | Whether this page is on the same domain as the start URL. |
| `was_skipped` | `bool` | — | Whether this page was skipped (binary or PDF content). |
| `is_pdf` | `bool` | — | Whether the content is a PDF. |
| `detected_charset` | `Option<String>` | `Default::default()` | The detected character set encoding. |
| `markdown` | `Option<MarkdownResult>` | `Default::default()` | Markdown conversion of the page content. |
| `extracted_data` | `Option<serde_json::Value>` | `Default::default()` | Structured data extracted by LLM. Populated when extraction is configured. |
| `extraction_meta` | `Option<ExtractionMeta>` | `Default::default()` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `downloaded_document` | `Option<DownloadedDocument>` | `Default::default()` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |

---

#### CrawlResult

The result of a multi-page crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pages` | `Vec<CrawlPageResult>` | `vec![]` | The list of crawled pages. |
| `final_url` | `String` | — | The final URL after following redirects. |
| `redirect_count` | `usize` | — | The number of redirects followed. |
| `was_skipped` | `bool` | — | Whether any page was skipped during crawling. |
| `error` | `Option<String>` | `Default::default()` | An error message, if the crawl encountered an issue. |
| `cookies` | `Vec<CookieInfo>` | `vec![]` | Cookies collected during the crawl. |
| `normalized_urls` | `Vec<String>` | `vec![]` | Normalized URLs encountered during crawling (for deduplication counting). |

---

#### MapResult

The result of a map operation, containing discovered URLs.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `urls` | `Vec<SitemapUrl>` | `vec![]` | The list of discovered URLs. |

---

#### MarkdownResult

Rich markdown conversion result from HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | Converted markdown text. |
| `document_structure` | `Option<serde_json::Value>` | `Default::default()` | Structured document tree with semantic nodes. |
| `tables` | `Vec<serde_json::Value>` | `vec![]` | Extracted tables with structured cell data. |
| `warnings` | `Vec<String>` | `vec![]` | Non-fatal processing warnings. |
| `citations` | `Option<CitationResult>` | `Default::default()` | Content with links replaced by numbered citations. |
| `fit_content` | `Option<String>` | `Default::default()` | Content-filtered markdown optimized for LLM consumption. |

---

#### CitationResult

Result of citation conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | Markdown with links replaced by numbered citations. |
| `references` | `Vec<CitationReference>` | `vec![]` | Numbered reference list: (index, url, text). |

---

#### BatchScrapeResult

Result from a single URL in a batch scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The URL that was scraped. |
| `result` | `Option<ScrapeResult>` | `Default::default()` | The scrape result, if successful. |
| `error` | `Option<String>` | `Default::default()` | The error message, if the scrape failed. |

---

#### BatchCrawlResult

Result from a single URL in a batch crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The seed URL that was crawled. |
| `result` | `Option<CrawlResult>` | `Default::default()` | The crawl result, if successful. |
| `error` | `Option<String>` | `Default::default()` | The error message, if the crawl failed. |

---

### Configuration Types

See [Configuration Reference](configuration.md) for detailed defaults and language-specific representations.

#### ProxyConfig

Proxy configuration for HTTP requests.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Proxy URL (e.g. "<http://proxy:8080",> "socks5://proxy:1080"). |
| `username` | `Option<String>` | `Default::default()` | Optional username for proxy authentication. |
| `password` | `Option<String>` | `Default::default()` | Optional password for proxy authentication. |

---

#### ContentConfig

Content extraction and conversion configuration.

Controls how HTML is converted to the output format. Uses
html-to-markdown-rs as the conversion engine for all formats
(markdown, plain text, djot).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `output_format` | `String` | `"markdown"` | Output format: `"markdown"` (default), `"plain"`, `"djot"`. |
| `preprocessing_preset` | `String` | `"standard"` | Preprocessing aggressiveness: `"minimal"`, `"standard"` (default), `"aggressive"`. - Minimal: only scripts/styles removed. - Standard: also removes nav, nav-hinted headers/footers/asides, forms. - Aggressive: removes all footers/asides unconditionally. |
| `remove_navigation` | `bool` | `true` | Remove navigation elements (nav, breadcrumbs, menus). Default: `true`. |
| `remove_forms` | `bool` | `true` | Remove form elements. Default: `true`. |
| `strip_tags` | `Vec<String>` | `vec![]` | HTML tag names to strip (render children only, remove the tag wrapper). Default: `["noscript"]`. |
| `preserve_tags` | `Vec<String>` | `vec![]` | HTML tag names to preserve as raw HTML in output. |
| `exclude_selectors` | `Vec<String>` | `vec![]` | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the wrapper but keeps children), excluded elements and all descendants are dropped. Supports CSS selectors: `.class`, `#id`, `[attribute]`, compound selectors. Example: `[".cookie-banner", "#ad-container", "[role='complementary']"]` |
| `skip_images` | `bool` | `false` | Skip image elements in output. Default: `false`. |
| `max_depth` | `Option<usize>` | `None` | Max DOM traversal depth. Prevents stack overflow on deeply nested HTML. |
| `wrap` | `bool` | `false` | Enable line wrapping. Default: `false`. |
| `wrap_width` | `usize` | `80` | Wrap width when `wrap` is enabled. Default: `80`. |
| `include_document_structure` | `bool` | `true` | Include document structure tree in output. Default: `true`. |

---

#### BrowserConfig

Browser fallback configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `mode` | `BrowserMode` | `BrowserMode::Auto` | When to use the headless browser fallback. |
| `endpoint` | `Option<String>` | `None` | CDP WebSocket endpoint for connecting to an external browser instance. |
| `timeout` | `Duration` | `30000ms` | Timeout for browser page load and rendering (in milliseconds when serialized). |
| `wait` | `BrowserWait` | `BrowserWait::NetworkIdle` | Wait strategy after browser navigation. |
| `wait_selector` | `Option<String>` | `None` | CSS selector to wait for when `wait` is `Selector`. |
| `extra_wait` | `Option<Duration>` | `None` | Extra time to wait after the wait condition is met. |

---

#### CrawlConfig

Configuration for crawl, scrape, and map operations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_depth` | `Option<usize>` | `None` | Maximum crawl depth (number of link hops from the start URL). |
| `max_pages` | `Option<usize>` | `None` | Maximum number of pages to crawl. |
| `max_concurrent` | `Option<usize>` | `None` | Maximum number of concurrent requests. |
| `respect_robots_txt` | `bool` | `false` | Whether to respect robots.txt directives. |
| `soft_http_errors` | `bool` | `false` | When true, HTTP-level error responses (404 NotFound, 403 Forbidden, WAF blocks) are surfaced as `ScrapeResult` records with the matching `status_code` rather than raised as `CrawlError`. Default `false` preserves the historical throw-on-error contract for direct fetches. Independently of this flag, 404s reached at the end of a redirect chain are *always* surfaced softly — the user opted into redirect-following, so receiving a 404 there is part of the normal flow rather than an unexpected error. |
| `user_agent` | `Option<String>` | `None` | Custom user-agent string. |
| `stay_on_domain` | `bool` | `false` | Whether to restrict crawling to the same domain. |
| `allow_subdomains` | `bool` | `false` | Whether to allow subdomains when `stay_on_domain` is true. |
| `include_paths` | `Vec<String>` | `vec![]` | Regex patterns for paths to include during crawling. |
| `exclude_paths` | `Vec<String>` | `vec![]` | Regex patterns for paths to exclude during crawling. |
| `custom_headers` | `HashMap<String, String>` | `HashMap::new()` | Custom HTTP headers to send with each request. |
| `request_timeout` | `Duration` | `30000ms` | Timeout for individual HTTP requests (in milliseconds when serialized). |
| `rate_limit_ms` | `Option<u64>` | `None` | Per-domain rate limit in milliseconds. When set, enforces a minimum delay between requests to the same domain. Defaults to 200ms when `None`. |
| `max_redirects` | `usize` | `10` | Maximum number of redirects to follow. |
| `retry_count` | `usize` | `0` | Number of retry attempts for failed requests. |
| `retry_codes` | `Vec<u16>` | `vec![]` | HTTP status codes that should trigger a retry. |
| `cookies_enabled` | `bool` | `false` | Whether to enable cookie handling. |
| `auth` | `Option<AuthConfig>` | `None` | Authentication configuration. |
| `max_body_size` | `Option<usize>` | `None` | Maximum response body size in bytes. |
| `remove_tags` | `Vec<String>` | `vec![]` | CSS selectors for tags to remove from HTML before processing. |
| `content` | `ContentConfig` | — | Content extraction and conversion configuration. |
| `map_limit` | `Option<usize>` | `None` | Maximum number of URLs to return from a map operation. |
| `map_search` | `Option<String>` | `None` | Search filter for map results (case-insensitive substring match on URLs). |
| `download_assets` | `bool` | `false` | Whether to download assets (CSS, JS, images, etc.) from the page. |
| `asset_types` | `Vec<AssetCategory>` | `vec![]` | Filter for asset categories to download. |
| `max_asset_size` | `Option<usize>` | `None` | Maximum size in bytes for individual asset downloads. |
| `browser` | `BrowserConfig` | — | Browser configuration. |
| `proxy` | `Option<ProxyConfig>` | `None` | Proxy configuration for HTTP requests. |
| `user_agents` | `Vec<String>` | `vec![]` | List of user-agent strings for rotation. If non-empty, overrides `user_agent`. |
| `capture_screenshot` | `bool` | `false` | Whether to capture a screenshot when using the browser. |
| `download_documents` | `bool` | `true` | Whether to download non-HTML documents (PDF, DOCX, images, code, etc.) instead of skipping them. |
| `document_max_size` | `Option<usize>` | `Default::default()` | Maximum size in bytes for document downloads. Defaults to 50 MB. |
| `document_mime_types` | `Vec<String>` | `vec![]` | Allowlist of MIME types to download. If empty, uses built-in defaults. |
| `warc_output` | `Option<PathBuf>` | `None` | Path to write WARC output. If `None`, WARC output is disabled. |
| `browser_profile` | `Option<String>` | `None` | Named browser profile for persistent sessions (cookies, localStorage). |
| `save_browser_profile` | `bool` | `false` | Whether to save changes back to the browser profile on exit. |

---

#### DownloadedDocument

A downloaded non-HTML document (PDF, DOCX, image, code file, etc.).

When the crawler encounters non-HTML content and `download_documents` is
enabled, it downloads the raw bytes and populates this struct instead of
skipping the resource.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The URL the document was fetched from. |
| `mime_type` | `String` | — | The MIME type from the Content-Type header. |
| `content` | `Vec<u8>` | — | Raw document bytes. Skipped during JSON serialization. |
| `size` | `usize` | — | Size of the document in bytes. |
| `filename` | `Option<String>` | `Default::default()` | Filename extracted from Content-Disposition or URL path. |
| `content_hash` | `String` | — | SHA-256 hex digest of the content. |
| `headers` | `HashMap<String, String>` | `HashMap::new()` | Selected response headers. |

---

#### SitemapUrl

A URL entry from a sitemap.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The URL. |
| `lastmod` | `Option<String>` | `Default::default()` | The last modification date, if present. |
| `changefreq` | `Option<String>` | `Default::default()` | The change frequency, if present. |
| `priority` | `Option<String>` | `Default::default()` | The priority, if present. |

---

#### LinkInfo

Information about a link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The resolved URL of the link. |
| `text` | `String` | — | The visible text of the link. |
| `link_type` | `LinkType` | `LinkType::Internal` | The classification of the link. |
| `rel` | `Option<String>` | `Default::default()` | The `rel` attribute value, if present. |
| `nofollow` | `bool` | — | Whether the link has `rel="nofollow"`. |

---

#### ImageInfo

Information about an image found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The image URL. |
| `alt` | `Option<String>` | `Default::default()` | The alt text, if present. |
| `width` | `Option<u32>` | `Default::default()` | The width attribute, if present and parseable. |
| `height` | `Option<u32>` | `Default::default()` | The height attribute, if present and parseable. |
| `source` | `ImageSource` | `ImageSource::Img` | The source of the image reference. |

---

#### FeedInfo

Information about a feed link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The feed URL. |
| `title` | `Option<String>` | `Default::default()` | The feed title, if present. |
| `feed_type` | `FeedType` | `FeedType::Rss` | The type of feed. |

---

#### JsonLdEntry

A JSON-LD structured data entry found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `schema_type` | `String` | — | The `@type` value from the JSON-LD object. |
| `name` | `Option<String>` | `Default::default()` | The `name` value, if present. |
| `raw` | `String` | — | The raw JSON-LD string. |

---

#### CookieInfo

Information about an HTTP cookie received from a response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The cookie name. |
| `value` | `String` | — | The cookie value. |
| `domain` | `Option<String>` | `Default::default()` | The cookie domain, if specified. |
| `path` | `Option<String>` | `Default::default()` | The cookie path, if specified. |

---

#### DownloadedAsset

A downloaded asset from a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The original URL of the asset. |
| `content_hash` | `String` | — | The SHA-256 content hash of the asset. |
| `mime_type` | `Option<String>` | `Default::default()` | The MIME type from the Content-Type header. |
| `size` | `usize` | — | The size of the asset in bytes. |
| `asset_category` | `AssetCategory` | `AssetCategory::Image` | The category of the asset. |
| `html_tag` | `Option<String>` | `Default::default()` | The HTML tag that referenced this asset (e.g., "link", "script", "img"). |

---

#### HreflangEntry

An hreflang alternate link entry.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `lang` | `String` | — | The language code (e.g., "en", "fr", "x-default"). |
| `url` | `String` | — | The URL for this language variant. |

---

#### FaviconInfo

Information about a favicon or icon link.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The icon URL. |
| `rel` | `String` | — | The `rel` attribute (e.g., "icon", "apple-touch-icon"). |
| `sizes` | `Option<String>` | `Default::default()` | The `sizes` attribute, if present. |
| `mime_type` | `Option<String>` | `Default::default()` | The MIME type, if present. |

---

#### HeadingInfo

A heading element extracted from the page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `u8` | — | The heading level (1-6). |
| `text` | `String` | — | The heading text content. |

---

#### CitationReference

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `usize` | — | Index |
| `url` | `String` | — | Url |
| `text` | `String` | — | Text |

---

### Metadata Types

#### ExtractionMeta

Metadata about an LLM extraction pass.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cost` | `Option<f64>` | `Default::default()` | Estimated cost of the LLM call in USD. |
| `prompt_tokens` | `Option<u64>` | `Default::default()` | Number of prompt (input) tokens consumed. |
| `completion_tokens` | `Option<u64>` | `Default::default()` | Number of completion (output) tokens generated. |
| `model` | `Option<String>` | `Default::default()` | The model identifier used for extraction. |
| `chunks_processed` | `usize` | — | Number of content chunks sent to the LLM. |

---

#### ArticleMetadata

Article metadata extracted from `article:*` Open Graph tags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `published_time` | `Option<String>` | `Default::default()` | The article publication time. |
| `modified_time` | `Option<String>` | `Default::default()` | The article modification time. |
| `author` | `Option<String>` | `Default::default()` | The article author. |
| `section` | `Option<String>` | `Default::default()` | The article section. |
| `tags` | `Vec<String>` | `vec![]` | The article tags. |

---

#### ResponseMeta

Response metadata extracted from HTTP headers.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `etag` | `Option<String>` | `Default::default()` | The ETag header value. |
| `last_modified` | `Option<String>` | `Default::default()` | The Last-Modified header value. |
| `cache_control` | `Option<String>` | `Default::default()` | The Cache-Control header value. |
| `server` | `Option<String>` | `Default::default()` | The Server header value. |
| `x_powered_by` | `Option<String>` | `Default::default()` | The X-Powered-By header value. |
| `content_language` | `Option<String>` | `Default::default()` | The Content-Language header value. |
| `content_encoding` | `Option<String>` | `Default::default()` | The Content-Encoding header value. |

---

#### PageMetadata

Metadata extracted from an HTML page's `<meta>` tags and `<title>` element.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `Option<String>` | `Default::default()` | The page title from the `<title>` element. |
| `description` | `Option<String>` | `Default::default()` | The meta description. |
| `canonical_url` | `Option<String>` | `Default::default()` | The canonical URL from `<link rel="canonical">`. |
| `keywords` | `Option<String>` | `Default::default()` | Keywords from `<meta name="keywords">`. |
| `author` | `Option<String>` | `Default::default()` | Author from `<meta name="author">`. |
| `viewport` | `Option<String>` | `Default::default()` | Viewport content from `<meta name="viewport">`. |
| `theme_color` | `Option<String>` | `Default::default()` | Theme color from `<meta name="theme-color">`. |
| `generator` | `Option<String>` | `Default::default()` | Generator from `<meta name="generator">`. |
| `robots` | `Option<String>` | `Default::default()` | Robots content from `<meta name="robots">`. |
| `html_lang` | `Option<String>` | `Default::default()` | The `lang` attribute from the `<html>` element. |
| `html_dir` | `Option<String>` | `Default::default()` | The `dir` attribute from the `<html>` element. |
| `og_title` | `Option<String>` | `Default::default()` | Open Graph title. |
| `og_type` | `Option<String>` | `Default::default()` | Open Graph type. |
| `og_image` | `Option<String>` | `Default::default()` | Open Graph image URL. |
| `og_description` | `Option<String>` | `Default::default()` | Open Graph description. |
| `og_url` | `Option<String>` | `Default::default()` | Open Graph URL. |
| `og_site_name` | `Option<String>` | `Default::default()` | Open Graph site name. |
| `og_locale` | `Option<String>` | `Default::default()` | Open Graph locale. |
| `og_video` | `Option<String>` | `Default::default()` | Open Graph video URL. |
| `og_audio` | `Option<String>` | `Default::default()` | Open Graph audio URL. |
| `og_locale_alternates` | `Vec<String>` | `vec![]` | Open Graph locale alternates. |
| `twitter_card` | `Option<String>` | `Default::default()` | Twitter card type. |
| `twitter_title` | `Option<String>` | `Default::default()` | Twitter title. |
| `twitter_description` | `Option<String>` | `Default::default()` | Twitter description. |
| `twitter_image` | `Option<String>` | `Default::default()` | Twitter image URL. |
| `twitter_site` | `Option<String>` | `Default::default()` | Twitter site handle. |
| `twitter_creator` | `Option<String>` | `Default::default()` | Twitter creator handle. |
| `dc_title` | `Option<String>` | `Default::default()` | Dublin Core title. |
| `dc_creator` | `Option<String>` | `Default::default()` | Dublin Core creator. |
| `dc_subject` | `Option<String>` | `Default::default()` | Dublin Core subject. |
| `dc_description` | `Option<String>` | `Default::default()` | Dublin Core description. |
| `dc_publisher` | `Option<String>` | `Default::default()` | Dublin Core publisher. |
| `dc_date` | `Option<String>` | `Default::default()` | Dublin Core date. |
| `dc_type` | `Option<String>` | `Default::default()` | Dublin Core type. |
| `dc_format` | `Option<String>` | `Default::default()` | Dublin Core format. |
| `dc_identifier` | `Option<String>` | `Default::default()` | Dublin Core identifier. |
| `dc_language` | `Option<String>` | `Default::default()` | Dublin Core language. |
| `dc_rights` | `Option<String>` | `Default::default()` | Dublin Core rights. |
| `article` | `Option<ArticleMetadata>` | `Default::default()` | Article metadata from `article:*` Open Graph tags. |
| `hreflangs` | `Vec<HreflangEntry>` | `vec![]` | Hreflang alternate links. |
| `favicons` | `Vec<FaviconInfo>` | `vec![]` | Favicon and icon links. |
| `headings` | `Vec<HeadingInfo>` | `vec![]` | Heading elements (h1-h6). |
| `word_count` | `Option<usize>` | `Default::default()` | Computed word count of the page body text. |

---

### Other Types

#### CrawlEngineHandle

Opaque handle to a configured crawl engine.

Constructed via `create_engine` with an optional `CrawlConfig`.
Default implementations for all pluggable components are used internally.

*Opaque type — fields are not directly accessible.*

---

### Enums

#### AssetCategory

The category of a downloaded asset.

| Variant | Wire value | Description |
|---------|------------|-------------|
| `Document` | `document` | A document file (PDF, DOC, etc.). |
| `Image` | `image` | An image file. |
| `Audio` | `audio` | An audio file. |
| `Video` | `video` | A video file. |
| `Font` | `font` | A font file. |
| `Stylesheet` | `stylesheet` | A CSS stylesheet. |
| `Script` | `script` | A JavaScript file. |
| `Archive` | `archive` | An archive file (ZIP, TAR, etc.). |
| `Data` | `data` | A data file (JSON, XML, CSV, etc.). |
| `Other` | `other` | An unrecognized asset type. |

---

#### AuthConfig

Authentication configuration.

| Variant | Wire value | Description |
|---------|------------|-------------|
| `Basic` | `basic` | HTTP Basic authentication. — Fields: `username`: `String`, `password`: `String` |
| `Bearer` | `bearer` | Bearer token authentication. — Fields: `token`: `String` |
| `Header` | `header` | Custom authentication header. — Fields: `name`: `String`, `value`: `String` |

---

#### BrowserMode

When to use the headless browser fallback.

| Variant | Wire value | Description |
|---------|------------|-------------|
| `Auto` | `auto` | Automatically detect when JS rendering is needed and fall back to browser. |
| `Always` | `always` | Always use the browser for every request. |
| `Never` | `never` | Never use the browser fallback. |

---

#### BrowserWait

Wait strategy for browser page rendering.

| Variant | Wire value | Description |
|---------|------------|-------------|
| `NetworkIdle` | `network_idle` | Wait until network activity is idle. |
| `Selector` | `selector` | Wait for a specific CSS selector to appear in the DOM. |
| `Fixed` | `fixed` | Wait for a fixed duration after navigation. |

---

#### FeedType

The type of a feed (RSS, Atom, or JSON Feed).

| Variant | Wire value | Description |
|---------|------------|-------------|
| `Rss` | `rss` | RSS feed. |
| `Atom` | `atom` | Atom feed. |
| `JsonFeed` | `json_feed` | JSON Feed. |

---

#### ImageSource

The source of an image reference.

| Variant | Wire value | Description |
|---------|------------|-------------|
| `Img` | `img` | An `<img>` tag. |
| `PictureSource` | `picture_source` | A `<source>` tag inside `<picture>`. |
| `OgImage` | `og:image` | An `og:image` meta tag. |
| `TwitterImage` | `twitter:image` | A `twitter:image` meta tag. |

---

#### LinkType

The classification of a link.

| Variant | Wire value | Description |
|---------|------------|-------------|
| `Internal` | `internal` | A link to the same domain. |
| `External` | `external` | A link to a different domain. |
| `Anchor` | `anchor` | A fragment-only link (e.g., `#section`). |
| `Document` | `document` | A link to a downloadable document (PDF, DOC, etc.). |

---
