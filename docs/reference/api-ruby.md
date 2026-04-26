---
title: "Ruby API Reference"
---

## Ruby API Reference <span class="version-badge">v0.3.0-rc.2</span>

### Functions

#### create_engine()

Create a new crawl engine with the given configuration.

If `config` is `nil`, uses `CrawlConfig.default()`.
Returns an error if the configuration is invalid.

**Signature:**

```ruby
def self.create_engine(config: nil)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `CrawlConfig?` | No | The configuration options |

**Returns:** `CrawlEngineHandle`

**Errors:** Raises `CrawlError`.


---

#### scrape()

Scrape a single URL, returning extracted page data.

**Signature:**

```ruby
def self.scrape(engine, url)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `url` | `String` | Yes | The URL to fetch |

**Returns:** `ScrapeResult`

**Errors:** Raises `CrawlError`.


---

#### crawl()

Crawl a website starting from `url`, following links up to the configured depth.

**Signature:**

```ruby
def self.crawl(engine, url)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `url` | `String` | Yes | The URL to fetch |

**Returns:** `CrawlResult`

**Errors:** Raises `CrawlError`.


---

#### map_urls()

Discover all pages on a website by following links and sitemaps.

**Signature:**

```ruby
def self.map_urls(engine, url)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `url` | `String` | Yes | The URL to fetch |

**Returns:** `MapResult`

**Errors:** Raises `CrawlError`.


---

#### batch_scrape()

Scrape multiple URLs concurrently.

**Signature:**

```ruby
def self.batch_scrape(engine, urls)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `urls` | `Array<String>` | Yes | The urls |

**Returns:** `Array<BatchScrapeResult>`


---

#### batch_crawl()

Crawl multiple seed URLs concurrently, each following links to configured depth.

**Signature:**

```ruby
def self.batch_crawl(engine, urls)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `urls` | `Array<String>` | Yes | The urls |

**Returns:** `Array<BatchCrawlResult>`


---

### Types

#### ArticleMetadata

Article metadata extracted from `article:*` Open Graph tags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `published_time` | `String?` | `nil` | The article publication time. |
| `modified_time` | `String?` | `nil` | The article modification time. |
| `author` | `String?` | `nil` | The article author. |
| `section` | `String?` | `nil` | The article section. |
| `tags` | `Array<String>` | `[]` | The article tags. |


---

#### BatchCrawlResult

Result from a single URL in a batch crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The seed URL that was crawled. |
| `result` | `CrawlResult?` | `nil` | The crawl result, if successful. |
| `error` | `String?` | `nil` | The error message, if the crawl failed. |


---

#### BatchScrapeResult

Result from a single URL in a batch scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The URL that was scraped. |
| `result` | `ScrapeResult?` | `nil` | The scrape result, if successful. |
| `error` | `String?` | `nil` | The error message, if the scrape failed. |


---

#### BrowserConfig

Browser fallback configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `mode` | `BrowserMode` | `:auto` | When to use the headless browser fallback. |
| `endpoint` | `String?` | `nil` | CDP WebSocket endpoint for connecting to an external browser instance. |
| `timeout` | `Float` | `30000ms` | Timeout for browser page load and rendering (in milliseconds when serialized). |
| `wait` | `BrowserWait` | `:network_idle` | Wait strategy after browser navigation. |
| `wait_selector` | `String?` | `nil` | CSS selector to wait for when `wait` is `Selector`. |
| `extra_wait` | `Float?` | `nil` | Extra time to wait after the wait condition is met. |

##### Methods

###### default()

**Signature:**

```ruby
def self.default()
```


---

#### CitationReference

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `Integer` | — | Index |
| `url` | `String` | — | Url |
| `text` | `String` | — | Text |


---

#### CitationResult

Result of citation conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | Markdown with links replaced by numbered citations. |
| `references` | `Array<CitationReference>` | `[]` | Numbered reference list: (index, url, text). |


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
| `remove_navigation` | `Boolean` | `true` | Remove navigation elements (nav, breadcrumbs, menus). Default: `true`. |
| `remove_forms` | `Boolean` | `true` | Remove form elements. Default: `true`. |
| `strip_tags` | `Array<String>` | `[]` | HTML tag names to strip (render children only, remove the tag wrapper). Default: `["noscript"]`. |
| `preserve_tags` | `Array<String>` | `[]` | HTML tag names to preserve as raw HTML in output. |
| `exclude_selectors` | `Array<String>` | `[]` | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the wrapper but keeps children), excluded elements and all descendants are dropped. Supports CSS selectors: `.class`, `#id`, `[attribute]`, compound selectors. Example: `[".cookie-banner", "#ad-container", "[role='complementary']"]` |
| `skip_images` | `Boolean` | `false` | Skip image elements in output. Default: `false`. |
| `max_depth` | `Integer?` | `nil` | Max DOM traversal depth. Prevents stack overflow on deeply nested HTML. |
| `wrap` | `Boolean` | `false` | Enable line wrapping. Default: `false`. |
| `wrap_width` | `Integer` | `80` | Wrap width when `wrap` is enabled. Default: `80`. |
| `include_document_structure` | `Boolean` | `true` | Include document structure tree in output. Default: `true`. |

##### Methods

###### default()

**Signature:**

```ruby
def self.default()
```


---

#### CookieInfo

Information about an HTTP cookie received from a response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The cookie name. |
| `value` | `String` | — | The cookie value. |
| `domain` | `String?` | `nil` | The cookie domain, if specified. |
| `path` | `String?` | `nil` | The cookie path, if specified. |


---

#### CrawlConfig

Configuration for crawl, scrape, and map operations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_depth` | `Integer?` | `nil` | Maximum crawl depth (number of link hops from the start URL). |
| `max_pages` | `Integer?` | `nil` | Maximum number of pages to crawl. |
| `max_concurrent` | `Integer?` | `nil` | Maximum number of concurrent requests. |
| `respect_robots_txt` | `Boolean` | `false` | Whether to respect robots.txt directives. |
| `user_agent` | `String?` | `nil` | Custom user-agent string. |
| `stay_on_domain` | `Boolean` | `false` | Whether to restrict crawling to the same domain. |
| `allow_subdomains` | `Boolean` | `false` | Whether to allow subdomains when `stay_on_domain` is true. |
| `include_paths` | `Array<String>` | `[]` | Regex patterns for paths to include during crawling. |
| `exclude_paths` | `Array<String>` | `[]` | Regex patterns for paths to exclude during crawling. |
| `custom_headers` | `Hash{String=>String}` | `{}` | Custom HTTP headers to send with each request. |
| `request_timeout` | `Float` | `30000ms` | Timeout for individual HTTP requests (in milliseconds when serialized). |
| `rate_limit_ms` | `Integer?` | `nil` | Per-domain rate limit in milliseconds. When set, enforces a minimum delay between requests to the same domain. Defaults to 200ms when `nil`. |
| `max_redirects` | `Integer` | `10` | Maximum number of redirects to follow. |
| `retry_count` | `Integer` | `0` | Number of retry attempts for failed requests. |
| `retry_codes` | `Array<Integer>` | `[]` | HTTP status codes that should trigger a retry. |
| `cookies_enabled` | `Boolean` | `false` | Whether to enable cookie handling. |
| `auth` | `AuthConfig?` | `nil` | Authentication configuration. |
| `max_body_size` | `Integer?` | `nil` | Maximum response body size in bytes. |
| `remove_tags` | `Array<String>` | `[]` | CSS selectors for tags to remove from HTML before processing. |
| `content` | `ContentConfig` | — | Content extraction and conversion configuration. |
| `map_limit` | `Integer?` | `nil` | Maximum number of URLs to return from a map operation. |
| `map_search` | `String?` | `nil` | Search filter for map results (case-insensitive substring match on URLs). |
| `download_assets` | `Boolean` | `false` | Whether to download assets (CSS, JS, images, etc.) from the page. |
| `asset_types` | `Array<AssetCategory>` | `[]` | Filter for asset categories to download. |
| `max_asset_size` | `Integer?` | `nil` | Maximum size in bytes for individual asset downloads. |
| `browser` | `BrowserConfig` | — | Browser configuration. |
| `proxy` | `ProxyConfig?` | `nil` | Proxy configuration for HTTP requests. |
| `user_agents` | `Array<String>` | `[]` | List of user-agent strings for rotation. If non-empty, overrides `user_agent`. |
| `capture_screenshot` | `Boolean` | `false` | Whether to capture a screenshot when using the browser. |
| `download_documents` | `Boolean` | `true` | Whether to download non-HTML documents (PDF, DOCX, images, code, etc.) instead of skipping them. |
| `document_max_size` | `Integer?` | `nil` | Maximum size in bytes for document downloads. Defaults to 50 MB. |
| `document_mime_types` | `Array<String>` | `[]` | Allowlist of MIME types to download. If empty, uses built-in defaults. |
| `warc_output` | `String?` | `nil` | Path to write WARC output. If `nil`, WARC output is disabled. |
| `browser_profile` | `String?` | `nil` | Named browser profile for persistent sessions (cookies, localStorage). |
| `save_browser_profile` | `Boolean` | `false` | Whether to save changes back to the browser profile on exit. |

##### Methods

###### default()

**Signature:**

```ruby
def self.default()
```

###### validate()

Validate the configuration, returning an error if any values are invalid.

**Signature:**

```ruby
def validate()
```


---

#### CrawlEngineHandle

Opaque handle to a configured crawl engine.

Constructed via `create_engine` with an optional `CrawlConfig`.
Default implementations for all pluggable components are used internally.


---

#### CrawlPageResult

The result of crawling a single page during a crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The original URL of the page. |
| `normalized_url` | `String` | — | The normalized URL of the page. |
| `status_code` | `Integer` | — | The HTTP status code of the response. |
| `content_type` | `String` | — | The Content-Type header value. |
| `html` | `String` | — | The HTML body of the response. |
| `body_size` | `Integer` | — | The size of the response body in bytes. |
| `metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `links` | `Array<LinkInfo>` | `[]` | Links found on the page. |
| `images` | `Array<ImageInfo>` | `[]` | Images found on the page. |
| `feeds` | `Array<FeedInfo>` | `[]` | Feed links found on the page. |
| `json_ld` | `Array<JsonLdEntry>` | `[]` | JSON-LD entries found on the page. |
| `depth` | `Integer` | — | The depth of this page from the start URL. |
| `stayed_on_domain` | `Boolean` | — | Whether this page is on the same domain as the start URL. |
| `was_skipped` | `Boolean` | — | Whether this page was skipped (binary or PDF content). |
| `is_pdf` | `Boolean` | — | Whether the content is a PDF. |
| `detected_charset` | `String?` | `nil` | The detected character set encoding. |
| `markdown` | `MarkdownResult?` | `nil` | Markdown conversion of the page content. |
| `extracted_data` | `Object?` | `nil` | Structured data extracted by LLM. Populated when extraction is configured. |
| `extraction_meta` | `ExtractionMeta?` | `nil` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `downloaded_document` | `DownloadedDocument?` | `nil` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |


---

#### CrawlResult

The result of a multi-page crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pages` | `Array<CrawlPageResult>` | `[]` | The list of crawled pages. |
| `final_url` | `String` | — | The final URL after following redirects. |
| `redirect_count` | `Integer` | — | The number of redirects followed. |
| `was_skipped` | `Boolean` | — | Whether any page was skipped during crawling. |
| `error` | `String?` | `nil` | An error message, if the crawl encountered an issue. |
| `cookies` | `Array<CookieInfo>` | `[]` | Cookies collected during the crawl. |
| `normalized_urls` | `Array<String>` | `[]` | Normalized URLs encountered during crawling (for deduplication counting). |

##### Methods

###### unique_normalized_urls()

Returns the count of unique normalized URLs encountered during crawling.

**Signature:**

```ruby
def unique_normalized_urls()
```


---

#### DownloadedAsset

A downloaded asset from a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The original URL of the asset. |
| `content_hash` | `String` | — | The SHA-256 content hash of the asset. |
| `mime_type` | `String?` | `nil` | The MIME type from the Content-Type header. |
| `size` | `Integer` | — | The size of the asset in bytes. |
| `asset_category` | `AssetCategory` | `:image` | The category of the asset. |
| `html_tag` | `String?` | `nil` | The HTML tag that referenced this asset (e.g., "link", "script", "img"). |


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
| `content` | `String` | — | Raw document bytes. Skipped during JSON serialization. |
| `size` | `Integer` | — | Size of the document in bytes. |
| `filename` | `String?` | `nil` | Filename extracted from Content-Disposition or URL path. |
| `content_hash` | `String` | — | SHA-256 hex digest of the content. |
| `headers` | `Hash{String=>String}` | `{}` | Selected response headers. |


---

#### ExtractionMeta

Metadata about an LLM extraction pass.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cost` | `Float?` | `nil` | Estimated cost of the LLM call in USD. |
| `prompt_tokens` | `Integer?` | `nil` | Number of prompt (input) tokens consumed. |
| `completion_tokens` | `Integer?` | `nil` | Number of completion (output) tokens generated. |
| `model` | `String?` | `nil` | The model identifier used for extraction. |
| `chunks_processed` | `Integer` | — | Number of content chunks sent to the LLM. |


---

#### FaviconInfo

Information about a favicon or icon link.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The icon URL. |
| `rel` | `String` | — | The `rel` attribute (e.g., "icon", "apple-touch-icon"). |
| `sizes` | `String?` | `nil` | The `sizes` attribute, if present. |
| `mime_type` | `String?` | `nil` | The MIME type, if present. |


---

#### FeedInfo

Information about a feed link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The feed URL. |
| `title` | `String?` | `nil` | The feed title, if present. |
| `feed_type` | `FeedType` | `:rss` | The type of feed. |


---

#### HeadingInfo

A heading element extracted from the page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `Integer` | — | The heading level (1-6). |
| `text` | `String` | — | The heading text content. |


---

#### HreflangEntry

An hreflang alternate link entry.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `lang` | `String` | — | The language code (e.g., "en", "fr", "x-default"). |
| `url` | `String` | — | The URL for this language variant. |


---

#### ImageInfo

Information about an image found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The image URL. |
| `alt` | `String?` | `nil` | The alt text, if present. |
| `width` | `Integer?` | `nil` | The width attribute, if present and parseable. |
| `height` | `Integer?` | `nil` | The height attribute, if present and parseable. |
| `source` | `ImageSource` | `:img` | The source of the image reference. |


---

#### JsonLdEntry

A JSON-LD structured data entry found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `schema_type` | `String` | — | The `@type` value from the JSON-LD object. |
| `name` | `String?` | `nil` | The `name` value, if present. |
| `raw` | `String` | — | The raw JSON-LD string. |


---

#### LinkInfo

Information about a link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The resolved URL of the link. |
| `text` | `String` | — | The visible text of the link. |
| `link_type` | `LinkType` | `:internal` | The classification of the link. |
| `rel` | `String?` | `nil` | The `rel` attribute value, if present. |
| `nofollow` | `Boolean` | — | Whether the link has `rel="nofollow"`. |


---

#### MapResult

The result of a map operation, containing discovered URLs.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `urls` | `Array<SitemapUrl>` | `[]` | The list of discovered URLs. |


---

#### MarkdownResult

Rich markdown conversion result from HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | Converted markdown text. |
| `document_structure` | `Object?` | `nil` | Structured document tree with semantic nodes. |
| `tables` | `Array<Object>` | `[]` | Extracted tables with structured cell data. |
| `warnings` | `Array<String>` | `[]` | Non-fatal processing warnings. |
| `citations` | `CitationResult?` | `nil` | Content with links replaced by numbered citations. |
| `fit_content` | `String?` | `nil` | Content-filtered markdown optimized for LLM consumption. |


---

#### PageMetadata

Metadata extracted from an HTML page's `<meta>` tags and `<title>` element.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `String?` | `nil` | The page title from the `<title>` element. |
| `description` | `String?` | `nil` | The meta description. |
| `canonical_url` | `String?` | `nil` | The canonical URL from `<link rel="canonical">`. |
| `keywords` | `String?` | `nil` | Keywords from `<meta name="keywords">`. |
| `author` | `String?` | `nil` | Author from `<meta name="author">`. |
| `viewport` | `String?` | `nil` | Viewport content from `<meta name="viewport">`. |
| `theme_color` | `String?` | `nil` | Theme color from `<meta name="theme-color">`. |
| `generator` | `String?` | `nil` | Generator from `<meta name="generator">`. |
| `robots` | `String?` | `nil` | Robots content from `<meta name="robots">`. |
| `html_lang` | `String?` | `nil` | The `lang` attribute from the `<html>` element. |
| `html_dir` | `String?` | `nil` | The `dir` attribute from the `<html>` element. |
| `og_title` | `String?` | `nil` | Open Graph title. |
| `og_type` | `String?` | `nil` | Open Graph type. |
| `og_image` | `String?` | `nil` | Open Graph image URL. |
| `og_description` | `String?` | `nil` | Open Graph description. |
| `og_url` | `String?` | `nil` | Open Graph URL. |
| `og_site_name` | `String?` | `nil` | Open Graph site name. |
| `og_locale` | `String?` | `nil` | Open Graph locale. |
| `og_video` | `String?` | `nil` | Open Graph video URL. |
| `og_audio` | `String?` | `nil` | Open Graph audio URL. |
| `og_locale_alternates` | `Array<String>?` | `[]` | Open Graph locale alternates. |
| `twitter_card` | `String?` | `nil` | Twitter card type. |
| `twitter_title` | `String?` | `nil` | Twitter title. |
| `twitter_description` | `String?` | `nil` | Twitter description. |
| `twitter_image` | `String?` | `nil` | Twitter image URL. |
| `twitter_site` | `String?` | `nil` | Twitter site handle. |
| `twitter_creator` | `String?` | `nil` | Twitter creator handle. |
| `dc_title` | `String?` | `nil` | Dublin Core title. |
| `dc_creator` | `String?` | `nil` | Dublin Core creator. |
| `dc_subject` | `String?` | `nil` | Dublin Core subject. |
| `dc_description` | `String?` | `nil` | Dublin Core description. |
| `dc_publisher` | `String?` | `nil` | Dublin Core publisher. |
| `dc_date` | `String?` | `nil` | Dublin Core date. |
| `dc_type` | `String?` | `nil` | Dublin Core type. |
| `dc_format` | `String?` | `nil` | Dublin Core format. |
| `dc_identifier` | `String?` | `nil` | Dublin Core identifier. |
| `dc_language` | `String?` | `nil` | Dublin Core language. |
| `dc_rights` | `String?` | `nil` | Dublin Core rights. |
| `article` | `ArticleMetadata?` | `nil` | Article metadata from `article:*` Open Graph tags. |
| `hreflangs` | `Array<HreflangEntry>?` | `[]` | Hreflang alternate links. |
| `favicons` | `Array<FaviconInfo>?` | `[]` | Favicon and icon links. |
| `headings` | `Array<HeadingInfo>?` | `[]` | Heading elements (h1-h6). |
| `word_count` | `Integer?` | `nil` | Computed word count of the page body text. |


---

#### ProxyConfig

Proxy configuration for HTTP requests.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Proxy URL (e.g. "<http://proxy:8080",> "socks5://proxy:1080"). |
| `username` | `String?` | `nil` | Optional username for proxy authentication. |
| `password` | `String?` | `nil` | Optional password for proxy authentication. |


---

#### ResponseMeta

Response metadata extracted from HTTP headers.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `etag` | `String?` | `nil` | The ETag header value. |
| `last_modified` | `String?` | `nil` | The Last-Modified header value. |
| `cache_control` | `String?` | `nil` | The Cache-Control header value. |
| `server` | `String?` | `nil` | The Server header value. |
| `x_powered_by` | `String?` | `nil` | The X-Powered-By header value. |
| `content_language` | `String?` | `nil` | The Content-Language header value. |
| `content_encoding` | `String?` | `nil` | The Content-Encoding header value. |


---

#### ScrapeResult

The result of a single-page scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `status_code` | `Integer` | — | The HTTP status code of the response. |
| `content_type` | `String` | — | The Content-Type header value. |
| `html` | `String` | — | The HTML body of the response. |
| `body_size` | `Integer` | — | The size of the response body in bytes. |
| `metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `links` | `Array<LinkInfo>` | `[]` | Links found on the page. |
| `images` | `Array<ImageInfo>` | `[]` | Images found on the page. |
| `feeds` | `Array<FeedInfo>` | `[]` | Feed links found on the page. |
| `json_ld` | `Array<JsonLdEntry>` | `[]` | JSON-LD entries found on the page. |
| `is_allowed` | `Boolean` | — | Whether the URL is allowed by robots.txt. |
| `crawl_delay` | `Integer?` | `nil` | The crawl delay from robots.txt, in seconds. |
| `noindex_detected` | `Boolean` | — | Whether a noindex directive was detected. |
| `nofollow_detected` | `Boolean` | — | Whether a nofollow directive was detected. |
| `x_robots_tag` | `String?` | `nil` | The X-Robots-Tag header value, if present. |
| `is_pdf` | `Boolean` | — | Whether the content is a PDF. |
| `was_skipped` | `Boolean` | — | Whether the page was skipped (binary or PDF content). |
| `detected_charset` | `String?` | `nil` | The detected character set encoding. |
| `auth_header_sent` | `Boolean` | — | Whether an authentication header was sent with the request. |
| `response_meta` | `ResponseMeta?` | `nil` | Response metadata extracted from HTTP headers. |
| `assets` | `Array<DownloadedAsset>` | `[]` | Downloaded assets from the page. |
| `js_render_hint` | `Boolean` | — | Whether the page content suggests JavaScript rendering is needed. |
| `browser_used` | `Boolean` | — | Whether the browser fallback was used to fetch this page. |
| `markdown` | `MarkdownResult?` | `nil` | Markdown conversion of the page content. |
| `extracted_data` | `Object?` | `nil` | Structured data extracted by LLM. Populated when extraction is configured. |
| `extraction_meta` | `ExtractionMeta?` | `nil` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `screenshot` | `String?` | `nil` | Screenshot of the page as PNG bytes. Populated when browser is used and capture_screenshot is enabled. |
| `downloaded_document` | `DownloadedDocument?` | `nil` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |


---

#### SitemapUrl

A URL entry from a sitemap.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The URL. |
| `lastmod` | `String?` | `nil` | The last modification date, if present. |
| `changefreq` | `String?` | `nil` | The change frequency, if present. |
| `priority` | `String?` | `nil` | The priority, if present. |


---

### Enums

#### BrowserMode

When to use the headless browser fallback.

| Value | Description |
|-------|-------------|
| `auto` | Automatically detect when JS rendering is needed and fall back to browser. |
| `always` | Always use the browser for every request. |
| `never` | Never use the browser fallback. |


---

#### BrowserWait

Wait strategy for browser page rendering.

| Value | Description |
|-------|-------------|
| `network_idle` | Wait until network activity is idle. |
| `selector` | Wait for a specific CSS selector to appear in the DOM. |
| `fixed` | Wait for a fixed duration after navigation. |


---

#### AuthConfig

Authentication configuration.

| Value | Description |
|-------|-------------|
| `basic` | HTTP Basic authentication. — Fields: `username`: `String`, `password`: `String` |
| `bearer` | Bearer token authentication. — Fields: `token`: `String` |
| `header` | Custom authentication header. — Fields: `name`: `String`, `value`: `String` |


---

#### LinkType

The classification of a link.

| Value | Description |
|-------|-------------|
| `internal` | A link to the same domain. |
| `external` | A link to a different domain. |
| `anchor` | A fragment-only link (e.g., `#section`). |
| `document` | A link to a downloadable document (PDF, DOC, etc.). |


---

#### ImageSource

The source of an image reference.

| Value | Description |
|-------|-------------|
| `img` | An `<img>` tag. |
| `picture_source` | A `<source>` tag inside `<picture>`. |
| `og_image` | An `og:image` meta tag. |
| `twitter_image` | A `twitter:image` meta tag. |


---

#### FeedType

The type of a feed (RSS, Atom, or JSON Feed).

| Value | Description |
|-------|-------------|
| `rss` | RSS feed. |
| `atom` | Atom feed. |
| `json_feed` | JSON Feed. |


---

#### AssetCategory

The category of a downloaded asset.

| Value | Description |
|-------|-------------|
| `document` | A document file (PDF, DOC, etc.). |
| `image` | An image file. |
| `audio` | An audio file. |
| `video` | A video file. |
| `font` | A font file. |
| `stylesheet` | A CSS stylesheet. |
| `script` | A JavaScript file. |
| `archive` | An archive file (ZIP, TAR, etc.). |
| `data` | A data file (JSON, XML, CSV, etc.). |
| `other` | An unrecognized asset type. |


---

### Errors

#### CrawlError

Errors that can occur during crawling, scraping, or mapping operations.

| Variant | Description |
|---------|-------------|
| `not_found` | The requested page was not found (HTTP 404). |
| `unauthorized` | The request was unauthorized (HTTP 401). |
| `forbidden` | The request was forbidden (HTTP 403). |
| `waf_blocked` | The request was blocked by a WAF or bot protection (HTTP 403 with WAF indicators). |
| `timeout` | The request timed out. |
| `rate_limited` | The request was rate-limited (HTTP 429). |
| `server_error` | A server error occurred (HTTP 5xx). |
| `bad_gateway` | A bad gateway error occurred (HTTP 502). |
| `gone` | The resource is permanently gone (HTTP 410). |
| `connection` | A connection error occurred. |
| `dns` | A DNS resolution error occurred. |
| `ssl` | An SSL/TLS error occurred. |
| `data_loss` | Data was lost or truncated during transfer. |
| `browser_error` | The browser failed to launch, connect, or navigate. |
| `browser_timeout` | The browser page load or rendering timed out. |
| `invalid_config` | The provided configuration is invalid. |
| `other` | An unclassified error occurred. |


---
