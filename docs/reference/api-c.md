---
title: "C API Reference"
---

## C API Reference <span class="version-badge">v0.3.0-rc.1</span>

### Functions

#### kcrawl_create_engine()

Create a new crawl engine with the given configuration.

If `config` is `NULL`, uses `CrawlConfig.default()`.
Returns an error if the configuration is invalid.

**Signature:**

```c
KcrawlCrawlEngineHandle* kcrawl_create_engine(KcrawlCrawlConfig config);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `KcrawlCrawlConfig*` | No | The configuration options |

**Returns:** `KcrawlCrawlEngineHandle`

**Errors:** Returns `NULL` on error.


---

#### kcrawl_scrape()

Scrape a single URL, returning extracted page data.

**Signature:**

```c
KcrawlScrapeResult* kcrawl_scrape(KcrawlCrawlEngineHandle engine, const char* url);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `KcrawlCrawlEngineHandle` | Yes | The crawl engine handle |
| `url` | `const char*` | Yes | The URL to fetch |

**Returns:** `KcrawlScrapeResult`

**Errors:** Returns `NULL` on error.


---

#### kcrawl_crawl()

Crawl a website starting from `url`, following links up to the configured depth.

**Signature:**

```c
KcrawlCrawlResult* kcrawl_crawl(KcrawlCrawlEngineHandle engine, const char* url);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `KcrawlCrawlEngineHandle` | Yes | The crawl engine handle |
| `url` | `const char*` | Yes | The URL to fetch |

**Returns:** `KcrawlCrawlResult`

**Errors:** Returns `NULL` on error.


---

#### kcrawl_map_urls()

Discover all pages on a website by following links and sitemaps.

**Signature:**

```c
KcrawlMapResult* kcrawl_map_urls(KcrawlCrawlEngineHandle engine, const char* url);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `KcrawlCrawlEngineHandle` | Yes | The crawl engine handle |
| `url` | `const char*` | Yes | The URL to fetch |

**Returns:** `KcrawlMapResult`

**Errors:** Returns `NULL` on error.


---

#### kcrawl_batch_scrape()

Scrape multiple URLs concurrently.

**Signature:**

```c
KcrawlBatchScrapeResult* kcrawl_batch_scrape(KcrawlCrawlEngineHandle engine, const char** urls);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `KcrawlCrawlEngineHandle` | Yes | The crawl engine handle |
| `urls` | `const char**` | Yes | The urls |

**Returns:** `KcrawlBatchScrapeResult*`


---

#### kcrawl_batch_crawl()

Crawl multiple seed URLs concurrently, each following links to configured depth.

**Signature:**

```c
KcrawlBatchCrawlResult* kcrawl_batch_crawl(KcrawlCrawlEngineHandle engine, const char** urls);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `KcrawlCrawlEngineHandle` | Yes | The crawl engine handle |
| `urls` | `const char**` | Yes | The urls |

**Returns:** `KcrawlBatchCrawlResult*`


---

### Types

#### KcrawlArticleMetadata

Article metadata extracted from `article:*` Open Graph tags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `published_time` | `const char**` | `NULL` | The article publication time. |
| `modified_time` | `const char**` | `NULL` | The article modification time. |
| `author` | `const char**` | `NULL` | The article author. |
| `section` | `const char**` | `NULL` | The article section. |
| `tags` | `const char**` | `NULL` | The article tags. |


---

#### KcrawlBatchCrawlResult

Result from a single URL in a batch crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `const char*` | — | The seed URL that was crawled. |
| `result` | `KcrawlCrawlResult*` | `NULL` | The crawl result, if successful. |
| `error` | `const char**` | `NULL` | The error message, if the crawl failed. |


---

#### KcrawlBatchScrapeResult

Result from a single URL in a batch scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `const char*` | — | The URL that was scraped. |
| `result` | `KcrawlScrapeResult*` | `NULL` | The scrape result, if successful. |
| `error` | `const char**` | `NULL` | The error message, if the scrape failed. |


---

#### KcrawlBrowserConfig

Browser fallback configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `mode` | `KcrawlBrowserMode` | `KCRAWL_KCRAWL_AUTO` | When to use the headless browser fallback. |
| `endpoint` | `const char**` | `NULL` | CDP WebSocket endpoint for connecting to an external browser instance. |
| `timeout` | `uint64_t` | `30000ms` | Timeout for browser page load and rendering (in milliseconds when serialized). |
| `wait` | `KcrawlBrowserWait` | `KCRAWL_KCRAWL_NETWORK_IDLE` | Wait strategy after browser navigation. |
| `wait_selector` | `const char**` | `NULL` | CSS selector to wait for when `wait` is `Selector`. |
| `extra_wait` | `uint64_t*` | `NULL` | Extra time to wait after the wait condition is met. |

##### Methods

###### kcrawl_default()

**Signature:**

```c
KcrawlBrowserConfig kcrawl_default();
```


---

#### KcrawlCitationReference

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `uintptr_t` | — | Index |
| `url` | `const char*` | — | Url |
| `text` | `const char*` | — | Text |


---

#### KcrawlCitationResult

Result of citation conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `const char*` | — | Markdown with links replaced by numbered citations. |
| `references` | `KcrawlCitationReference*` | `NULL` | Numbered reference list: (index, url, text). |


---

#### KcrawlContentConfig

Content extraction and conversion configuration.

Controls how HTML is converted to the output format. Uses
html-to-markdown-rs as the conversion engine for all formats
(markdown, plain text, djot).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `output_format` | `const char*` | `"markdown"` | Output format: `"markdown"` (default), `"plain"`, `"djot"`. |
| `preprocessing_preset` | `const char*` | `"standard"` | Preprocessing aggressiveness: `"minimal"`, `"standard"` (default), `"aggressive"`. - Minimal: only scripts/styles removed. - Standard: also removes nav, nav-hinted headers/footers/asides, forms. - Aggressive: removes all footers/asides unconditionally. |
| `remove_navigation` | `bool` | `true` | Remove navigation elements (nav, breadcrumbs, menus). Default: `true`. |
| `remove_forms` | `bool` | `true` | Remove form elements. Default: `true`. |
| `strip_tags` | `const char**` | `NULL` | HTML tag names to strip (render children only, remove the tag wrapper). Default: `["noscript"]`. |
| `preserve_tags` | `const char**` | `NULL` | HTML tag names to preserve as raw HTML in output. |
| `exclude_selectors` | `const char**` | `NULL` | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the wrapper but keeps children), excluded elements and all descendants are dropped. Supports CSS selectors: `.class`, `#id`, `[attribute]`, compound selectors. Example: `[".cookie-banner", "#ad-container", "[role='complementary']"]` |
| `skip_images` | `bool` | `false` | Skip image elements in output. Default: `false`. |
| `max_depth` | `uintptr_t*` | `NULL` | Max DOM traversal depth. Prevents stack overflow on deeply nested HTML. |
| `wrap` | `bool` | `false` | Enable line wrapping. Default: `false`. |
| `wrap_width` | `uintptr_t` | `80` | Wrap width when `wrap` is enabled. Default: `80`. |
| `include_document_structure` | `bool` | `true` | Include document structure tree in output. Default: `true`. |

##### Methods

###### kcrawl_default()

**Signature:**

```c
KcrawlContentConfig kcrawl_default();
```


---

#### KcrawlCookieInfo

Information about an HTTP cookie received from a response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `const char*` | — | The cookie name. |
| `value` | `const char*` | — | The cookie value. |
| `domain` | `const char**` | `NULL` | The cookie domain, if specified. |
| `path` | `const char**` | `NULL` | The cookie path, if specified. |


---

#### KcrawlCrawlConfig

Configuration for crawl, scrape, and map operations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_depth` | `uintptr_t*` | `NULL` | Maximum crawl depth (number of link hops from the start URL). |
| `max_pages` | `uintptr_t*` | `NULL` | Maximum number of pages to crawl. |
| `max_concurrent` | `uintptr_t*` | `NULL` | Maximum number of concurrent requests. |
| `respect_robots_txt` | `bool` | `false` | Whether to respect robots.txt directives. |
| `user_agent` | `const char**` | `NULL` | Custom user-agent string. |
| `stay_on_domain` | `bool` | `false` | Whether to restrict crawling to the same domain. |
| `allow_subdomains` | `bool` | `false` | Whether to allow subdomains when `stay_on_domain` is true. |
| `include_paths` | `const char**` | `NULL` | Regex patterns for paths to include during crawling. |
| `exclude_paths` | `const char**` | `NULL` | Regex patterns for paths to exclude during crawling. |
| `custom_headers` | `void*` | `NULL` | Custom HTTP headers to send with each request. |
| `request_timeout` | `uint64_t` | `30000ms` | Timeout for individual HTTP requests (in milliseconds when serialized). |
| `rate_limit_ms` | `uint64_t*` | `NULL` | Per-domain rate limit in milliseconds. When set, enforces a minimum delay between requests to the same domain. Defaults to 200ms when `NULL`. |
| `max_redirects` | `uintptr_t` | `10` | Maximum number of redirects to follow. |
| `retry_count` | `uintptr_t` | `0` | Number of retry attempts for failed requests. |
| `retry_codes` | `uint16_t*` | `NULL` | HTTP status codes that should trigger a retry. |
| `cookies_enabled` | `bool` | `false` | Whether to enable cookie handling. |
| `auth` | `KcrawlAuthConfig*` | `NULL` | Authentication configuration. |
| `max_body_size` | `uintptr_t*` | `NULL` | Maximum response body size in bytes. |
| `remove_tags` | `const char**` | `NULL` | CSS selectors for tags to remove from HTML before processing. |
| `content` | `KcrawlContentConfig` | — | Content extraction and conversion configuration. |
| `map_limit` | `uintptr_t*` | `NULL` | Maximum number of URLs to return from a map operation. |
| `map_search` | `const char**` | `NULL` | Search filter for map results (case-insensitive substring match on URLs). |
| `download_assets` | `bool` | `false` | Whether to download assets (CSS, JS, images, etc.) from the page. |
| `asset_types` | `KcrawlAssetCategory*` | `NULL` | Filter for asset categories to download. |
| `max_asset_size` | `uintptr_t*` | `NULL` | Maximum size in bytes for individual asset downloads. |
| `browser` | `KcrawlBrowserConfig` | — | Browser configuration. |
| `proxy` | `KcrawlProxyConfig*` | `NULL` | Proxy configuration for HTTP requests. |
| `user_agents` | `const char**` | `NULL` | List of user-agent strings for rotation. If non-empty, overrides `user_agent`. |
| `capture_screenshot` | `bool` | `false` | Whether to capture a screenshot when using the browser. |
| `download_documents` | `bool` | `true` | Whether to download non-HTML documents (PDF, DOCX, images, code, etc.) instead of skipping them. |
| `document_max_size` | `uintptr_t*` | `NULL` | Maximum size in bytes for document downloads. Defaults to 50 MB. |
| `document_mime_types` | `const char**` | `NULL` | Allowlist of MIME types to download. If empty, uses built-in defaults. |
| `warc_output` | `const char**` | `NULL` | Path to write WARC output. If `NULL`, WARC output is disabled. |
| `browser_profile` | `const char**` | `NULL` | Named browser profile for persistent sessions (cookies, localStorage). |
| `save_browser_profile` | `bool` | `false` | Whether to save changes back to the browser profile on exit. |

##### Methods

###### kcrawl_default()

**Signature:**

```c
KcrawlCrawlConfig kcrawl_default();
```

###### kcrawl_validate()

Validate the configuration, returning an error if any values are invalid.

**Signature:**

```c
void kcrawl_validate();
```


---

#### KcrawlCrawlEngineHandle

Opaque handle to a configured crawl engine.

Constructed via `create_engine` with an optional `CrawlConfig`.
Default implementations for all pluggable components are used internally.


---

#### KcrawlCrawlPageResult

The result of crawling a single page during a crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `const char*` | — | The original URL of the page. |
| `normalized_url` | `const char*` | — | The normalized URL of the page. |
| `status_code` | `uint16_t` | — | The HTTP status code of the response. |
| `content_type` | `const char*` | — | The Content-Type header value. |
| `html` | `const char*` | — | The HTML body of the response. |
| `body_size` | `uintptr_t` | — | The size of the response body in bytes. |
| `metadata` | `KcrawlPageMetadata` | — | Extracted metadata from the page. |
| `links` | `KcrawlLinkInfo*` | `NULL` | Links found on the page. |
| `images` | `KcrawlImageInfo*` | `NULL` | Images found on the page. |
| `feeds` | `KcrawlFeedInfo*` | `NULL` | Feed links found on the page. |
| `json_ld` | `KcrawlJsonLdEntry*` | `NULL` | JSON-LD entries found on the page. |
| `depth` | `uintptr_t` | — | The depth of this page from the start URL. |
| `stayed_on_domain` | `bool` | — | Whether this page is on the same domain as the start URL. |
| `was_skipped` | `bool` | — | Whether this page was skipped (binary or PDF content). |
| `is_pdf` | `bool` | — | Whether the content is a PDF. |
| `detected_charset` | `const char**` | `NULL` | The detected character set encoding. |
| `markdown` | `KcrawlMarkdownResult*` | `NULL` | Markdown conversion of the page content. |
| `extracted_data` | `void**` | `NULL` | Structured data extracted by LLM. Populated when extraction is configured. |
| `extraction_meta` | `KcrawlExtractionMeta*` | `NULL` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `downloaded_document` | `KcrawlDownloadedDocument*` | `NULL` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |


---

#### KcrawlCrawlResult

The result of a multi-page crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pages` | `KcrawlCrawlPageResult*` | `NULL` | The list of crawled pages. |
| `final_url` | `const char*` | — | The final URL after following redirects. |
| `redirect_count` | `uintptr_t` | — | The number of redirects followed. |
| `was_skipped` | `bool` | — | Whether any page was skipped during crawling. |
| `error` | `const char**` | `NULL` | An error message, if the crawl encountered an issue. |
| `cookies` | `KcrawlCookieInfo*` | `NULL` | Cookies collected during the crawl. |
| `normalized_urls` | `const char**` | `NULL` | Normalized URLs encountered during crawling (for deduplication counting). |

##### Methods

###### kcrawl_unique_normalized_urls()

Returns the count of unique normalized URLs encountered during crawling.

**Signature:**

```c
uintptr_t kcrawl_unique_normalized_urls();
```


---

#### KcrawlDownloadedAsset

A downloaded asset from a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `const char*` | — | The original URL of the asset. |
| `content_hash` | `const char*` | — | The SHA-256 content hash of the asset. |
| `mime_type` | `const char**` | `NULL` | The MIME type from the Content-Type header. |
| `size` | `uintptr_t` | — | The size of the asset in bytes. |
| `asset_category` | `KcrawlAssetCategory` | `KCRAWL_KCRAWL_IMAGE` | The category of the asset. |
| `html_tag` | `const char**` | `NULL` | The HTML tag that referenced this asset (e.g., "link", "script", "img"). |


---

#### KcrawlDownloadedDocument

A downloaded non-HTML document (PDF, DOCX, image, code file, etc.).

When the crawler encounters non-HTML content and `download_documents` is
enabled, it downloads the raw bytes and populates this struct instead of
skipping the resource.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `const char*` | — | The URL the document was fetched from. |
| `mime_type` | `const char*` | — | The MIME type from the Content-Type header. |
| `content` | `const uint8_t*` | — | Raw document bytes. Skipped during JSON serialization. |
| `size` | `uintptr_t` | — | Size of the document in bytes. |
| `filename` | `const char**` | `NULL` | Filename extracted from Content-Disposition or URL path. |
| `content_hash` | `const char*` | — | SHA-256 hex digest of the content. |
| `headers` | `void*` | `NULL` | Selected response headers. |


---

#### KcrawlExtractionMeta

Metadata about an LLM extraction pass.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cost` | `double*` | `NULL` | Estimated cost of the LLM call in USD. |
| `prompt_tokens` | `uint64_t*` | `NULL` | Number of prompt (input) tokens consumed. |
| `completion_tokens` | `uint64_t*` | `NULL` | Number of completion (output) tokens generated. |
| `model` | `const char**` | `NULL` | The model identifier used for extraction. |
| `chunks_processed` | `uintptr_t` | — | Number of content chunks sent to the LLM. |


---

#### KcrawlFaviconInfo

Information about a favicon or icon link.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `const char*` | — | The icon URL. |
| `rel` | `const char*` | — | The `rel` attribute (e.g., "icon", "apple-touch-icon"). |
| `sizes` | `const char**` | `NULL` | The `sizes` attribute, if present. |
| `mime_type` | `const char**` | `NULL` | The MIME type, if present. |


---

#### KcrawlFeedInfo

Information about a feed link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `const char*` | — | The feed URL. |
| `title` | `const char**` | `NULL` | The feed title, if present. |
| `feed_type` | `KcrawlFeedType` | `KCRAWL_KCRAWL_RSS` | The type of feed. |


---

#### KcrawlHeadingInfo

A heading element extracted from the page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `uint8_t` | — | The heading level (1-6). |
| `text` | `const char*` | — | The heading text content. |


---

#### KcrawlHreflangEntry

An hreflang alternate link entry.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `lang` | `const char*` | — | The language code (e.g., "en", "fr", "x-default"). |
| `url` | `const char*` | — | The URL for this language variant. |


---

#### KcrawlImageInfo

Information about an image found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `const char*` | — | The image URL. |
| `alt` | `const char**` | `NULL` | The alt text, if present. |
| `width` | `uint32_t*` | `NULL` | The width attribute, if present and parseable. |
| `height` | `uint32_t*` | `NULL` | The height attribute, if present and parseable. |
| `source` | `KcrawlImageSource` | `KCRAWL_KCRAWL_IMG` | The source of the image reference. |


---

#### KcrawlJsonLdEntry

A JSON-LD structured data entry found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `schema_type` | `const char*` | — | The `@type` value from the JSON-LD object. |
| `name` | `const char**` | `NULL` | The `name` value, if present. |
| `raw` | `const char*` | — | The raw JSON-LD string. |


---

#### KcrawlLinkInfo

Information about a link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `const char*` | — | The resolved URL of the link. |
| `text` | `const char*` | — | The visible text of the link. |
| `link_type` | `KcrawlLinkType` | `KCRAWL_KCRAWL_INTERNAL` | The classification of the link. |
| `rel` | `const char**` | `NULL` | The `rel` attribute value, if present. |
| `nofollow` | `bool` | — | Whether the link has `rel="nofollow"`. |


---

#### KcrawlMapResult

The result of a map operation, containing discovered URLs.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `urls` | `KcrawlSitemapUrl*` | `NULL` | The list of discovered URLs. |


---

#### KcrawlMarkdownResult

Rich markdown conversion result from HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `const char*` | — | Converted markdown text. |
| `document_structure` | `void**` | `NULL` | Structured document tree with semantic nodes. |
| `tables` | `void**` | `NULL` | Extracted tables with structured cell data. |
| `warnings` | `const char**` | `NULL` | Non-fatal processing warnings. |
| `citations` | `KcrawlCitationResult*` | `NULL` | Content with links replaced by numbered citations. |
| `fit_content` | `const char**` | `NULL` | Content-filtered markdown optimized for LLM consumption. |


---

#### KcrawlPageMetadata

Metadata extracted from an HTML page's `<meta>` tags and `<title>` element.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `const char**` | `NULL` | The page title from the `<title>` element. |
| `description` | `const char**` | `NULL` | The meta description. |
| `canonical_url` | `const char**` | `NULL` | The canonical URL from `<link rel="canonical">`. |
| `keywords` | `const char**` | `NULL` | Keywords from `<meta name="keywords">`. |
| `author` | `const char**` | `NULL` | Author from `<meta name="author">`. |
| `viewport` | `const char**` | `NULL` | Viewport content from `<meta name="viewport">`. |
| `theme_color` | `const char**` | `NULL` | Theme color from `<meta name="theme-color">`. |
| `generator` | `const char**` | `NULL` | Generator from `<meta name="generator">`. |
| `robots` | `const char**` | `NULL` | Robots content from `<meta name="robots">`. |
| `html_lang` | `const char**` | `NULL` | The `lang` attribute from the `<html>` element. |
| `html_dir` | `const char**` | `NULL` | The `dir` attribute from the `<html>` element. |
| `og_title` | `const char**` | `NULL` | Open Graph title. |
| `og_type` | `const char**` | `NULL` | Open Graph type. |
| `og_image` | `const char**` | `NULL` | Open Graph image URL. |
| `og_description` | `const char**` | `NULL` | Open Graph description. |
| `og_url` | `const char**` | `NULL` | Open Graph URL. |
| `og_site_name` | `const char**` | `NULL` | Open Graph site name. |
| `og_locale` | `const char**` | `NULL` | Open Graph locale. |
| `og_video` | `const char**` | `NULL` | Open Graph video URL. |
| `og_audio` | `const char**` | `NULL` | Open Graph audio URL. |
| `og_locale_alternates` | `const char***` | `NULL` | Open Graph locale alternates. |
| `twitter_card` | `const char**` | `NULL` | Twitter card type. |
| `twitter_title` | `const char**` | `NULL` | Twitter title. |
| `twitter_description` | `const char**` | `NULL` | Twitter description. |
| `twitter_image` | `const char**` | `NULL` | Twitter image URL. |
| `twitter_site` | `const char**` | `NULL` | Twitter site handle. |
| `twitter_creator` | `const char**` | `NULL` | Twitter creator handle. |
| `dc_title` | `const char**` | `NULL` | Dublin Core title. |
| `dc_creator` | `const char**` | `NULL` | Dublin Core creator. |
| `dc_subject` | `const char**` | `NULL` | Dublin Core subject. |
| `dc_description` | `const char**` | `NULL` | Dublin Core description. |
| `dc_publisher` | `const char**` | `NULL` | Dublin Core publisher. |
| `dc_date` | `const char**` | `NULL` | Dublin Core date. |
| `dc_type` | `const char**` | `NULL` | Dublin Core type. |
| `dc_format` | `const char**` | `NULL` | Dublin Core format. |
| `dc_identifier` | `const char**` | `NULL` | Dublin Core identifier. |
| `dc_language` | `const char**` | `NULL` | Dublin Core language. |
| `dc_rights` | `const char**` | `NULL` | Dublin Core rights. |
| `article` | `KcrawlArticleMetadata*` | `NULL` | Article metadata from `article:*` Open Graph tags. |
| `hreflangs` | `KcrawlHreflangEntry**` | `NULL` | Hreflang alternate links. |
| `favicons` | `KcrawlFaviconInfo**` | `NULL` | Favicon and icon links. |
| `headings` | `KcrawlHeadingInfo**` | `NULL` | Heading elements (h1-h6). |
| `word_count` | `uintptr_t*` | `NULL` | Computed word count of the page body text. |


---

#### KcrawlProxyConfig

Proxy configuration for HTTP requests.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `const char*` | — | Proxy URL (e.g. "<http://proxy:8080",> "socks5://proxy:1080"). |
| `username` | `const char**` | `NULL` | Optional username for proxy authentication. |
| `password` | `const char**` | `NULL` | Optional password for proxy authentication. |


---

#### KcrawlResponseMeta

Response metadata extracted from HTTP headers.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `etag` | `const char**` | `NULL` | The ETag header value. |
| `last_modified` | `const char**` | `NULL` | The Last-Modified header value. |
| `cache_control` | `const char**` | `NULL` | The Cache-Control header value. |
| `server` | `const char**` | `NULL` | The Server header value. |
| `x_powered_by` | `const char**` | `NULL` | The X-Powered-By header value. |
| `content_language` | `const char**` | `NULL` | The Content-Language header value. |
| `content_encoding` | `const char**` | `NULL` | The Content-Encoding header value. |


---

#### KcrawlScrapeResult

The result of a single-page scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `status_code` | `uint16_t` | — | The HTTP status code of the response. |
| `content_type` | `const char*` | — | The Content-Type header value. |
| `html` | `const char*` | — | The HTML body of the response. |
| `body_size` | `uintptr_t` | — | The size of the response body in bytes. |
| `metadata` | `KcrawlPageMetadata` | — | Extracted metadata from the page. |
| `links` | `KcrawlLinkInfo*` | `NULL` | Links found on the page. |
| `images` | `KcrawlImageInfo*` | `NULL` | Images found on the page. |
| `feeds` | `KcrawlFeedInfo*` | `NULL` | Feed links found on the page. |
| `json_ld` | `KcrawlJsonLdEntry*` | `NULL` | JSON-LD entries found on the page. |
| `is_allowed` | `bool` | — | Whether the URL is allowed by robots.txt. |
| `crawl_delay` | `uint64_t*` | `NULL` | The crawl delay from robots.txt, in seconds. |
| `noindex_detected` | `bool` | — | Whether a noindex directive was detected. |
| `nofollow_detected` | `bool` | — | Whether a nofollow directive was detected. |
| `x_robots_tag` | `const char**` | `NULL` | The X-Robots-Tag header value, if present. |
| `is_pdf` | `bool` | — | Whether the content is a PDF. |
| `was_skipped` | `bool` | — | Whether the page was skipped (binary or PDF content). |
| `detected_charset` | `const char**` | `NULL` | The detected character set encoding. |
| `auth_header_sent` | `bool` | — | Whether an authentication header was sent with the request. |
| `response_meta` | `KcrawlResponseMeta*` | `NULL` | Response metadata extracted from HTTP headers. |
| `assets` | `KcrawlDownloadedAsset*` | `NULL` | Downloaded assets from the page. |
| `js_render_hint` | `bool` | — | Whether the page content suggests JavaScript rendering is needed. |
| `browser_used` | `bool` | — | Whether the browser fallback was used to fetch this page. |
| `markdown` | `KcrawlMarkdownResult*` | `NULL` | Markdown conversion of the page content. |
| `extracted_data` | `void**` | `NULL` | Structured data extracted by LLM. Populated when extraction is configured. |
| `extraction_meta` | `KcrawlExtractionMeta*` | `NULL` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `screenshot` | `const uint8_t**` | `NULL` | Screenshot of the page as PNG bytes. Populated when browser is used and capture_screenshot is enabled. |
| `downloaded_document` | `KcrawlDownloadedDocument*` | `NULL` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |


---

#### KcrawlSitemapUrl

A URL entry from a sitemap.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `const char*` | — | The URL. |
| `lastmod` | `const char**` | `NULL` | The last modification date, if present. |
| `changefreq` | `const char**` | `NULL` | The change frequency, if present. |
| `priority` | `const char**` | `NULL` | The priority, if present. |


---

### Enums

#### KcrawlBrowserMode

When to use the headless browser fallback.

| Value | Description |
|-------|-------------|
| `KCRAWL_AUTO` | Automatically detect when JS rendering is needed and fall back to browser. |
| `KCRAWL_ALWAYS` | Always use the browser for every request. |
| `KCRAWL_NEVER` | Never use the browser fallback. |


---

#### KcrawlBrowserWait

Wait strategy for browser page rendering.

| Value | Description |
|-------|-------------|
| `KCRAWL_NETWORK_IDLE` | Wait until network activity is idle. |
| `KCRAWL_SELECTOR` | Wait for a specific CSS selector to appear in the DOM. |
| `KCRAWL_FIXED` | Wait for a fixed duration after navigation. |


---

#### KcrawlAuthConfig

Authentication configuration.

| Value | Description |
|-------|-------------|
| `KCRAWL_BASIC` | HTTP Basic authentication. — Fields: `username`: `const char*`, `password`: `const char*` |
| `KCRAWL_BEARER` | Bearer token authentication. — Fields: `token`: `const char*` |
| `KCRAWL_HEADER` | Custom authentication header. — Fields: `name`: `const char*`, `value`: `const char*` |


---

#### KcrawlLinkType

The classification of a link.

| Value | Description |
|-------|-------------|
| `KCRAWL_INTERNAL` | A link to the same domain. |
| `KCRAWL_EXTERNAL` | A link to a different domain. |
| `KCRAWL_ANCHOR` | A fragment-only link (e.g., `#section`). |
| `KCRAWL_DOCUMENT` | A link to a downloadable document (PDF, DOC, etc.). |


---

#### KcrawlImageSource

The source of an image reference.

| Value | Description |
|-------|-------------|
| `KCRAWL_IMG` | An `<img>` tag. |
| `KCRAWL_PICTURE_SOURCE` | A `<source>` tag inside `<picture>`. |
| `KCRAWL_OG_IMAGE` | An `og:image` meta tag. |
| `KCRAWL_TWITTER_IMAGE` | A `twitter:image` meta tag. |


---

#### KcrawlFeedType

The type of a feed (RSS, Atom, or JSON Feed).

| Value | Description |
|-------|-------------|
| `KCRAWL_RSS` | RSS feed. |
| `KCRAWL_ATOM` | Atom feed. |
| `KCRAWL_JSON_FEED` | JSON Feed. |


---

#### KcrawlAssetCategory

The category of a downloaded asset.

| Value | Description |
|-------|-------------|
| `KCRAWL_DOCUMENT` | A document file (PDF, DOC, etc.). |
| `KCRAWL_IMAGE` | An image file. |
| `KCRAWL_AUDIO` | An audio file. |
| `KCRAWL_VIDEO` | A video file. |
| `KCRAWL_FONT` | A font file. |
| `KCRAWL_STYLESHEET` | A CSS stylesheet. |
| `KCRAWL_SCRIPT` | A JavaScript file. |
| `KCRAWL_ARCHIVE` | An archive file (ZIP, TAR, etc.). |
| `KCRAWL_DATA` | A data file (JSON, XML, CSV, etc.). |
| `KCRAWL_OTHER` | An unrecognized asset type. |


---

### Errors

#### KcrawlCrawlError

Errors that can occur during crawling, scraping, or mapping operations.

| Variant | Description |
|---------|-------------|
| `KCRAWL_NOT_FOUND` | The requested page was not found (HTTP 404). |
| `KCRAWL_UNAUTHORIZED` | The request was unauthorized (HTTP 401). |
| `KCRAWL_FORBIDDEN` | The request was forbidden (HTTP 403). |
| `KCRAWL_WAF_BLOCKED` | The request was blocked by a WAF or bot protection (HTTP 403 with WAF indicators). |
| `KCRAWL_TIMEOUT` | The request timed out. |
| `KCRAWL_RATE_LIMITED` | The request was rate-limited (HTTP 429). |
| `KCRAWL_SERVER_ERROR` | A server error occurred (HTTP 5xx). |
| `KCRAWL_BAD_GATEWAY` | A bad gateway error occurred (HTTP 502). |
| `KCRAWL_GONE` | The resource is permanently gone (HTTP 410). |
| `KCRAWL_CONNECTION` | A connection error occurred. |
| `KCRAWL_DNS` | A DNS resolution error occurred. |
| `KCRAWL_SSL` | An SSL/TLS error occurred. |
| `KCRAWL_DATA_LOSS` | Data was lost or truncated during transfer. |
| `KCRAWL_BROWSER_ERROR` | The browser failed to launch, connect, or navigate. |
| `KCRAWL_BROWSER_TIMEOUT` | The browser page load or rendering timed out. |
| `KCRAWL_INVALID_CONFIG` | The provided configuration is invalid. |
| `KCRAWL_OTHER` | An unclassified error occurred. |


---
