---
title: "Elixir API Reference"
---

## Elixir API Reference <span class="version-badge">v0.3.0-rc.1</span>

### Functions

#### create_engine()

Create a new crawl engine with the given configuration.

If `config` is `nil`, uses `CrawlConfig.default()`.
Returns an error if the configuration is invalid.

**Signature:**

```elixir
@spec create_engine(config) :: {:ok, term()} | {:error, term()}
def create_engine(config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `CrawlConfig | nil` | No | The configuration options |

**Returns:** `CrawlEngineHandle`

**Errors:** Returns `{:error, reason}`


---

#### scrape()

Scrape a single URL, returning extracted page data.

**Signature:**

```elixir
@spec scrape(engine, url) :: {:ok, term()} | {:error, term()}
def scrape(engine, url)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `url` | `String.t()` | Yes | The URL to fetch |

**Returns:** `ScrapeResult`

**Errors:** Returns `{:error, reason}`


---

#### crawl()

Crawl a website starting from `url`, following links up to the configured depth.

**Signature:**

```elixir
@spec crawl(engine, url) :: {:ok, term()} | {:error, term()}
def crawl(engine, url)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `url` | `String.t()` | Yes | The URL to fetch |

**Returns:** `CrawlResult`

**Errors:** Returns `{:error, reason}`


---

#### map_urls()

Discover all pages on a website by following links and sitemaps.

**Signature:**

```elixir
@spec map_urls(engine, url) :: {:ok, term()} | {:error, term()}
def map_urls(engine, url)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `url` | `String.t()` | Yes | The URL to fetch |

**Returns:** `MapResult`

**Errors:** Returns `{:error, reason}`


---

#### batch_scrape()

Scrape multiple URLs concurrently.

**Signature:**

```elixir
@spec batch_scrape(engine, urls) :: {:ok, term()} | {:error, term()}
def batch_scrape(engine, urls)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `urls` | `list(String.t())` | Yes | The urls |

**Returns:** `list(BatchScrapeResult)`


---

#### batch_crawl()

Crawl multiple seed URLs concurrently, each following links to configured depth.

**Signature:**

```elixir
@spec batch_crawl(engine, urls) :: {:ok, term()} | {:error, term()}
def batch_crawl(engine, urls)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `urls` | `list(String.t())` | Yes | The urls |

**Returns:** `list(BatchCrawlResult)`


---

### Types

#### ArticleMetadata

Article metadata extracted from `article:*` Open Graph tags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `published_time` | `String.t() | nil` | `nil` | The article publication time. |
| `modified_time` | `String.t() | nil` | `nil` | The article modification time. |
| `author` | `String.t() | nil` | `nil` | The article author. |
| `section` | `String.t() | nil` | `nil` | The article section. |
| `tags` | `list(String.t())` | `[]` | The article tags. |


---

#### BatchCrawlResult

Result from a single URL in a batch crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String.t()` | — | The seed URL that was crawled. |
| `result` | `CrawlResult | nil` | `nil` | The crawl result, if successful. |
| `error` | `String.t() | nil` | `nil` | The error message, if the crawl failed. |


---

#### BatchScrapeResult

Result from a single URL in a batch scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String.t()` | — | The URL that was scraped. |
| `result` | `ScrapeResult | nil` | `nil` | The scrape result, if successful. |
| `error` | `String.t() | nil` | `nil` | The error message, if the scrape failed. |


---

#### BrowserConfig

Browser fallback configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `mode` | `BrowserMode` | `:auto` | When to use the headless browser fallback. |
| `endpoint` | `String.t() | nil` | `nil` | CDP WebSocket endpoint for connecting to an external browser instance. |
| `timeout` | `integer()` | `30000ms` | Timeout for browser page load and rendering (in milliseconds when serialized). |
| `wait` | `BrowserWait` | `:network_idle` | Wait strategy after browser navigation. |
| `wait_selector` | `String.t() | nil` | `nil` | CSS selector to wait for when `wait` is `Selector`. |
| `extra_wait` | `integer() | nil` | `nil` | Extra time to wait after the wait condition is met. |

##### Functions

###### default()

**Signature:**

```elixir
def default()
```


---

#### CitationReference

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `integer()` | — | Index |
| `url` | `String.t()` | — | Url |
| `text` | `String.t()` | — | Text |


---

#### CitationResult

Result of citation conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String.t()` | — | Markdown with links replaced by numbered citations. |
| `references` | `list(CitationReference)` | `[]` | Numbered reference list: (index, url, text). |


---

#### ContentConfig

Content extraction and conversion configuration.

Controls how HTML is converted to the output format. Uses
html-to-markdown-rs as the conversion engine for all formats
(markdown, plain text, djot).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `output_format` | `String.t()` | `"markdown"` | Output format: `"markdown"` (default), `"plain"`, `"djot"`. |
| `preprocessing_preset` | `String.t()` | `"standard"` | Preprocessing aggressiveness: `"minimal"`, `"standard"` (default), `"aggressive"`. - Minimal: only scripts/styles removed. - Standard: also removes nav, nav-hinted headers/footers/asides, forms. - Aggressive: removes all footers/asides unconditionally. |
| `remove_navigation` | `boolean()` | `true` | Remove navigation elements (nav, breadcrumbs, menus). Default: `true`. |
| `remove_forms` | `boolean()` | `true` | Remove form elements. Default: `true`. |
| `strip_tags` | `list(String.t())` | `[]` | HTML tag names to strip (render children only, remove the tag wrapper). Default: `["noscript"]`. |
| `preserve_tags` | `list(String.t())` | `[]` | HTML tag names to preserve as raw HTML in output. |
| `exclude_selectors` | `list(String.t())` | `[]` | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the wrapper but keeps children), excluded elements and all descendants are dropped. Supports CSS selectors: `.class`, `#id`, `[attribute]`, compound selectors. Example: `[".cookie-banner", "#ad-container", "[role='complementary']"]` |
| `skip_images` | `boolean()` | `false` | Skip image elements in output. Default: `false`. |
| `max_depth` | `integer() | nil` | `nil` | Max DOM traversal depth. Prevents stack overflow on deeply nested HTML. |
| `wrap` | `boolean()` | `false` | Enable line wrapping. Default: `false`. |
| `wrap_width` | `integer()` | `80` | Wrap width when `wrap` is enabled. Default: `80`. |
| `include_document_structure` | `boolean()` | `true` | Include document structure tree in output. Default: `true`. |

##### Functions

###### default()

**Signature:**

```elixir
def default()
```


---

#### CookieInfo

Information about an HTTP cookie received from a response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String.t()` | — | The cookie name. |
| `value` | `String.t()` | — | The cookie value. |
| `domain` | `String.t() | nil` | `nil` | The cookie domain, if specified. |
| `path` | `String.t() | nil` | `nil` | The cookie path, if specified. |


---

#### CrawlConfig

Configuration for crawl, scrape, and map operations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_depth` | `integer() | nil` | `nil` | Maximum crawl depth (number of link hops from the start URL). |
| `max_pages` | `integer() | nil` | `nil` | Maximum number of pages to crawl. |
| `max_concurrent` | `integer() | nil` | `nil` | Maximum number of concurrent requests. |
| `respect_robots_txt` | `boolean()` | `false` | Whether to respect robots.txt directives. |
| `user_agent` | `String.t() | nil` | `nil` | Custom user-agent string. |
| `stay_on_domain` | `boolean()` | `false` | Whether to restrict crawling to the same domain. |
| `allow_subdomains` | `boolean()` | `false` | Whether to allow subdomains when `stay_on_domain` is true. |
| `include_paths` | `list(String.t())` | `[]` | Regex patterns for paths to include during crawling. |
| `exclude_paths` | `list(String.t())` | `[]` | Regex patterns for paths to exclude during crawling. |
| `custom_headers` | `map()` | `%{}` | Custom HTTP headers to send with each request. |
| `request_timeout` | `integer()` | `30000ms` | Timeout for individual HTTP requests (in milliseconds when serialized). |
| `rate_limit_ms` | `integer() | nil` | `nil` | Per-domain rate limit in milliseconds. When set, enforces a minimum delay between requests to the same domain. Defaults to 200ms when `nil`. |
| `max_redirects` | `integer()` | `10` | Maximum number of redirects to follow. |
| `retry_count` | `integer()` | `0` | Number of retry attempts for failed requests. |
| `retry_codes` | `list(integer())` | `[]` | HTTP status codes that should trigger a retry. |
| `cookies_enabled` | `boolean()` | `false` | Whether to enable cookie handling. |
| `auth` | `AuthConfig | nil` | `nil` | Authentication configuration. |
| `max_body_size` | `integer() | nil` | `nil` | Maximum response body size in bytes. |
| `remove_tags` | `list(String.t())` | `[]` | CSS selectors for tags to remove from HTML before processing. |
| `content` | `ContentConfig` | — | Content extraction and conversion configuration. |
| `map_limit` | `integer() | nil` | `nil` | Maximum number of URLs to return from a map operation. |
| `map_search` | `String.t() | nil` | `nil` | Search filter for map results (case-insensitive substring match on URLs). |
| `download_assets` | `boolean()` | `false` | Whether to download assets (CSS, JS, images, etc.) from the page. |
| `asset_types` | `list(AssetCategory)` | `[]` | Filter for asset categories to download. |
| `max_asset_size` | `integer() | nil` | `nil` | Maximum size in bytes for individual asset downloads. |
| `browser` | `BrowserConfig` | — | Browser configuration. |
| `proxy` | `ProxyConfig | nil` | `nil` | Proxy configuration for HTTP requests. |
| `user_agents` | `list(String.t())` | `[]` | List of user-agent strings for rotation. If non-empty, overrides `user_agent`. |
| `capture_screenshot` | `boolean()` | `false` | Whether to capture a screenshot when using the browser. |
| `download_documents` | `boolean()` | `true` | Whether to download non-HTML documents (PDF, DOCX, images, code, etc.) instead of skipping them. |
| `document_max_size` | `integer() | nil` | `nil` | Maximum size in bytes for document downloads. Defaults to 50 MB. |
| `document_mime_types` | `list(String.t())` | `[]` | Allowlist of MIME types to download. If empty, uses built-in defaults. |
| `warc_output` | `String.t() | nil` | `nil` | Path to write WARC output. If `nil`, WARC output is disabled. |
| `browser_profile` | `String.t() | nil` | `nil` | Named browser profile for persistent sessions (cookies, localStorage). |
| `save_browser_profile` | `boolean()` | `false` | Whether to save changes back to the browser profile on exit. |

##### Functions

###### default()

**Signature:**

```elixir
def default()
```

###### validate()

Validate the configuration, returning an error if any values are invalid.

**Signature:**

```elixir
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
| `url` | `String.t()` | — | The original URL of the page. |
| `normalized_url` | `String.t()` | — | The normalized URL of the page. |
| `status_code` | `integer()` | — | The HTTP status code of the response. |
| `content_type` | `String.t()` | — | The Content-Type header value. |
| `html` | `String.t()` | — | The HTML body of the response. |
| `body_size` | `integer()` | — | The size of the response body in bytes. |
| `metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `links` | `list(LinkInfo)` | `[]` | Links found on the page. |
| `images` | `list(ImageInfo)` | `[]` | Images found on the page. |
| `feeds` | `list(FeedInfo)` | `[]` | Feed links found on the page. |
| `json_ld` | `list(JsonLdEntry)` | `[]` | JSON-LD entries found on the page. |
| `depth` | `integer()` | — | The depth of this page from the start URL. |
| `stayed_on_domain` | `boolean()` | — | Whether this page is on the same domain as the start URL. |
| `was_skipped` | `boolean()` | — | Whether this page was skipped (binary or PDF content). |
| `is_pdf` | `boolean()` | — | Whether the content is a PDF. |
| `detected_charset` | `String.t() | nil` | `nil` | The detected character set encoding. |
| `markdown` | `MarkdownResult | nil` | `nil` | Markdown conversion of the page content. |
| `extracted_data` | `term() | nil` | `nil` | Structured data extracted by LLM. Populated when extraction is configured. |
| `extraction_meta` | `ExtractionMeta | nil` | `nil` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `downloaded_document` | `DownloadedDocument | nil` | `nil` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |


---

#### CrawlResult

The result of a multi-page crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pages` | `list(CrawlPageResult)` | `[]` | The list of crawled pages. |
| `final_url` | `String.t()` | — | The final URL after following redirects. |
| `redirect_count` | `integer()` | — | The number of redirects followed. |
| `was_skipped` | `boolean()` | — | Whether any page was skipped during crawling. |
| `error` | `String.t() | nil` | `nil` | An error message, if the crawl encountered an issue. |
| `cookies` | `list(CookieInfo)` | `[]` | Cookies collected during the crawl. |
| `normalized_urls` | `list(String.t())` | `[]` | Normalized URLs encountered during crawling (for deduplication counting). |

##### Functions

###### unique_normalized_urls()

Returns the count of unique normalized URLs encountered during crawling.

**Signature:**

```elixir
def unique_normalized_urls()
```


---

#### DownloadedAsset

A downloaded asset from a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String.t()` | — | The original URL of the asset. |
| `content_hash` | `String.t()` | — | The SHA-256 content hash of the asset. |
| `mime_type` | `String.t() | nil` | `nil` | The MIME type from the Content-Type header. |
| `size` | `integer()` | — | The size of the asset in bytes. |
| `asset_category` | `AssetCategory` | `:image` | The category of the asset. |
| `html_tag` | `String.t() | nil` | `nil` | The HTML tag that referenced this asset (e.g., "link", "script", "img"). |


---

#### DownloadedDocument

A downloaded non-HTML document (PDF, DOCX, image, code file, etc.).

When the crawler encounters non-HTML content and `download_documents` is
enabled, it downloads the raw bytes and populates this struct instead of
skipping the resource.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String.t()` | — | The URL the document was fetched from. |
| `mime_type` | `String.t()` | — | The MIME type from the Content-Type header. |
| `content` | `binary()` | — | Raw document bytes. Skipped during JSON serialization. |
| `size` | `integer()` | — | Size of the document in bytes. |
| `filename` | `String.t() | nil` | `nil` | Filename extracted from Content-Disposition or URL path. |
| `content_hash` | `String.t()` | — | SHA-256 hex digest of the content. |
| `headers` | `map()` | `%{}` | Selected response headers. |


---

#### ExtractionMeta

Metadata about an LLM extraction pass.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cost` | `float() | nil` | `nil` | Estimated cost of the LLM call in USD. |
| `prompt_tokens` | `integer() | nil` | `nil` | Number of prompt (input) tokens consumed. |
| `completion_tokens` | `integer() | nil` | `nil` | Number of completion (output) tokens generated. |
| `model` | `String.t() | nil` | `nil` | The model identifier used for extraction. |
| `chunks_processed` | `integer()` | — | Number of content chunks sent to the LLM. |


---

#### FaviconInfo

Information about a favicon or icon link.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String.t()` | — | The icon URL. |
| `rel` | `String.t()` | — | The `rel` attribute (e.g., "icon", "apple-touch-icon"). |
| `sizes` | `String.t() | nil` | `nil` | The `sizes` attribute, if present. |
| `mime_type` | `String.t() | nil` | `nil` | The MIME type, if present. |


---

#### FeedInfo

Information about a feed link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String.t()` | — | The feed URL. |
| `title` | `String.t() | nil` | `nil` | The feed title, if present. |
| `feed_type` | `FeedType` | `:rss` | The type of feed. |


---

#### HeadingInfo

A heading element extracted from the page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `integer()` | — | The heading level (1-6). |
| `text` | `String.t()` | — | The heading text content. |


---

#### HreflangEntry

An hreflang alternate link entry.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `lang` | `String.t()` | — | The language code (e.g., "en", "fr", "x-default"). |
| `url` | `String.t()` | — | The URL for this language variant. |


---

#### ImageInfo

Information about an image found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String.t()` | — | The image URL. |
| `alt` | `String.t() | nil` | `nil` | The alt text, if present. |
| `width` | `integer() | nil` | `nil` | The width attribute, if present and parseable. |
| `height` | `integer() | nil` | `nil` | The height attribute, if present and parseable. |
| `source` | `ImageSource` | `:img` | The source of the image reference. |


---

#### JsonLdEntry

A JSON-LD structured data entry found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `schema_type` | `String.t()` | — | The `@type` value from the JSON-LD object. |
| `name` | `String.t() | nil` | `nil` | The `name` value, if present. |
| `raw` | `String.t()` | — | The raw JSON-LD string. |


---

#### LinkInfo

Information about a link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String.t()` | — | The resolved URL of the link. |
| `text` | `String.t()` | — | The visible text of the link. |
| `link_type` | `LinkType` | `:internal` | The classification of the link. |
| `rel` | `String.t() | nil` | `nil` | The `rel` attribute value, if present. |
| `nofollow` | `boolean()` | — | Whether the link has `rel="nofollow"`. |


---

#### MapResult

The result of a map operation, containing discovered URLs.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `urls` | `list(SitemapUrl)` | `[]` | The list of discovered URLs. |


---

#### MarkdownResult

Rich markdown conversion result from HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String.t()` | — | Converted markdown text. |
| `document_structure` | `term() | nil` | `nil` | Structured document tree with semantic nodes. |
| `tables` | `list(term())` | `[]` | Extracted tables with structured cell data. |
| `warnings` | `list(String.t())` | `[]` | Non-fatal processing warnings. |
| `citations` | `CitationResult | nil` | `nil` | Content with links replaced by numbered citations. |
| `fit_content` | `String.t() | nil` | `nil` | Content-filtered markdown optimized for LLM consumption. |


---

#### PageMetadata

Metadata extracted from an HTML page's `<meta>` tags and `<title>` element.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `String.t() | nil` | `nil` | The page title from the `<title>` element. |
| `description` | `String.t() | nil` | `nil` | The meta description. |
| `canonical_url` | `String.t() | nil` | `nil` | The canonical URL from `<link rel="canonical">`. |
| `keywords` | `String.t() | nil` | `nil` | Keywords from `<meta name="keywords">`. |
| `author` | `String.t() | nil` | `nil` | Author from `<meta name="author">`. |
| `viewport` | `String.t() | nil` | `nil` | Viewport content from `<meta name="viewport">`. |
| `theme_color` | `String.t() | nil` | `nil` | Theme color from `<meta name="theme-color">`. |
| `generator` | `String.t() | nil` | `nil` | Generator from `<meta name="generator">`. |
| `robots` | `String.t() | nil` | `nil` | Robots content from `<meta name="robots">`. |
| `html_lang` | `String.t() | nil` | `nil` | The `lang` attribute from the `<html>` element. |
| `html_dir` | `String.t() | nil` | `nil` | The `dir` attribute from the `<html>` element. |
| `og_title` | `String.t() | nil` | `nil` | Open Graph title. |
| `og_type` | `String.t() | nil` | `nil` | Open Graph type. |
| `og_image` | `String.t() | nil` | `nil` | Open Graph image URL. |
| `og_description` | `String.t() | nil` | `nil` | Open Graph description. |
| `og_url` | `String.t() | nil` | `nil` | Open Graph URL. |
| `og_site_name` | `String.t() | nil` | `nil` | Open Graph site name. |
| `og_locale` | `String.t() | nil` | `nil` | Open Graph locale. |
| `og_video` | `String.t() | nil` | `nil` | Open Graph video URL. |
| `og_audio` | `String.t() | nil` | `nil` | Open Graph audio URL. |
| `og_locale_alternates` | `list(String.t()) | nil` | `[]` | Open Graph locale alternates. |
| `twitter_card` | `String.t() | nil` | `nil` | Twitter card type. |
| `twitter_title` | `String.t() | nil` | `nil` | Twitter title. |
| `twitter_description` | `String.t() | nil` | `nil` | Twitter description. |
| `twitter_image` | `String.t() | nil` | `nil` | Twitter image URL. |
| `twitter_site` | `String.t() | nil` | `nil` | Twitter site handle. |
| `twitter_creator` | `String.t() | nil` | `nil` | Twitter creator handle. |
| `dc_title` | `String.t() | nil` | `nil` | Dublin Core title. |
| `dc_creator` | `String.t() | nil` | `nil` | Dublin Core creator. |
| `dc_subject` | `String.t() | nil` | `nil` | Dublin Core subject. |
| `dc_description` | `String.t() | nil` | `nil` | Dublin Core description. |
| `dc_publisher` | `String.t() | nil` | `nil` | Dublin Core publisher. |
| `dc_date` | `String.t() | nil` | `nil` | Dublin Core date. |
| `dc_type` | `String.t() | nil` | `nil` | Dublin Core type. |
| `dc_format` | `String.t() | nil` | `nil` | Dublin Core format. |
| `dc_identifier` | `String.t() | nil` | `nil` | Dublin Core identifier. |
| `dc_language` | `String.t() | nil` | `nil` | Dublin Core language. |
| `dc_rights` | `String.t() | nil` | `nil` | Dublin Core rights. |
| `article` | `ArticleMetadata | nil` | `nil` | Article metadata from `article:*` Open Graph tags. |
| `hreflangs` | `list(HreflangEntry) | nil` | `[]` | Hreflang alternate links. |
| `favicons` | `list(FaviconInfo) | nil` | `[]` | Favicon and icon links. |
| `headings` | `list(HeadingInfo) | nil` | `[]` | Heading elements (h1-h6). |
| `word_count` | `integer() | nil` | `nil` | Computed word count of the page body text. |


---

#### ProxyConfig

Proxy configuration for HTTP requests.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String.t()` | — | Proxy URL (e.g. "<http://proxy:8080",> "socks5://proxy:1080"). |
| `username` | `String.t() | nil` | `nil` | Optional username for proxy authentication. |
| `password` | `String.t() | nil` | `nil` | Optional password for proxy authentication. |


---

#### ResponseMeta

Response metadata extracted from HTTP headers.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `etag` | `String.t() | nil` | `nil` | The ETag header value. |
| `last_modified` | `String.t() | nil` | `nil` | The Last-Modified header value. |
| `cache_control` | `String.t() | nil` | `nil` | The Cache-Control header value. |
| `server` | `String.t() | nil` | `nil` | The Server header value. |
| `x_powered_by` | `String.t() | nil` | `nil` | The X-Powered-By header value. |
| `content_language` | `String.t() | nil` | `nil` | The Content-Language header value. |
| `content_encoding` | `String.t() | nil` | `nil` | The Content-Encoding header value. |


---

#### ScrapeResult

The result of a single-page scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `status_code` | `integer()` | — | The HTTP status code of the response. |
| `content_type` | `String.t()` | — | The Content-Type header value. |
| `html` | `String.t()` | — | The HTML body of the response. |
| `body_size` | `integer()` | — | The size of the response body in bytes. |
| `metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `links` | `list(LinkInfo)` | `[]` | Links found on the page. |
| `images` | `list(ImageInfo)` | `[]` | Images found on the page. |
| `feeds` | `list(FeedInfo)` | `[]` | Feed links found on the page. |
| `json_ld` | `list(JsonLdEntry)` | `[]` | JSON-LD entries found on the page. |
| `is_allowed` | `boolean()` | — | Whether the URL is allowed by robots.txt. |
| `crawl_delay` | `integer() | nil` | `nil` | The crawl delay from robots.txt, in seconds. |
| `noindex_detected` | `boolean()` | — | Whether a noindex directive was detected. |
| `nofollow_detected` | `boolean()` | — | Whether a nofollow directive was detected. |
| `x_robots_tag` | `String.t() | nil` | `nil` | The X-Robots-Tag header value, if present. |
| `is_pdf` | `boolean()` | — | Whether the content is a PDF. |
| `was_skipped` | `boolean()` | — | Whether the page was skipped (binary or PDF content). |
| `detected_charset` | `String.t() | nil` | `nil` | The detected character set encoding. |
| `auth_header_sent` | `boolean()` | — | Whether an authentication header was sent with the request. |
| `response_meta` | `ResponseMeta | nil` | `nil` | Response metadata extracted from HTTP headers. |
| `assets` | `list(DownloadedAsset)` | `[]` | Downloaded assets from the page. |
| `js_render_hint` | `boolean()` | — | Whether the page content suggests JavaScript rendering is needed. |
| `browser_used` | `boolean()` | — | Whether the browser fallback was used to fetch this page. |
| `markdown` | `MarkdownResult | nil` | `nil` | Markdown conversion of the page content. |
| `extracted_data` | `term() | nil` | `nil` | Structured data extracted by LLM. Populated when extraction is configured. |
| `extraction_meta` | `ExtractionMeta | nil` | `nil` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `screenshot` | `binary() | nil` | `nil` | Screenshot of the page as PNG bytes. Populated when browser is used and capture_screenshot is enabled. |
| `downloaded_document` | `DownloadedDocument | nil` | `nil` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |


---

#### SitemapUrl

A URL entry from a sitemap.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String.t()` | — | The URL. |
| `lastmod` | `String.t() | nil` | `nil` | The last modification date, if present. |
| `changefreq` | `String.t() | nil` | `nil` | The change frequency, if present. |
| `priority` | `String.t() | nil` | `nil` | The priority, if present. |


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
| `basic` | HTTP Basic authentication. — Fields: `username`: `String.t()`, `password`: `String.t()` |
| `bearer` | Bearer token authentication. — Fields: `token`: `String.t()` |
| `header` | Custom authentication header. — Fields: `name`: `String.t()`, `value`: `String.t()` |


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
