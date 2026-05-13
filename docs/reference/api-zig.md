---
title: "Zig API Reference"
---

## Zig API Reference <span class="version-badge">v0.3.0-rc.19</span>

### Functions

#### createEngine()

Create a new crawl engine with the given configuration.

If `config` is `null`, uses `CrawlConfig.default()`.
Returns an error if the configuration is invalid.

**Signature:**

```zig
// Phase 1: zig backend signature generation
```

**Parameters:**

| Name     | Type           | Required | Description               |
| -------- | -------------- | -------- | ------------------------- |
| `config` | `CrawlConfig?` | No       | The configuration options |

**Returns:** `CrawlEngineHandle`
**Errors:** Throws `CrawlError`.

---

#### scrape()

Scrape a single URL, returning extracted page data.

**Signature:**

```zig
// Phase 1: zig backend signature generation
```

**Parameters:**

| Name     | Type                | Required | Description             |
| -------- | ------------------- | -------- | ----------------------- |
| `engine` | `CrawlEngineHandle` | Yes      | The crawl engine handle |
| `url`    | `[:0]const u8`      | Yes      | The URL to fetch        |

**Returns:** `ScrapeResult`
**Errors:** Throws `CrawlError`.

---

#### crawl()

Crawl a website starting from `url`, following links up to the configured depth.

**Signature:**

```zig
// Phase 1: zig backend signature generation
```

**Parameters:**

| Name     | Type                | Required | Description             |
| -------- | ------------------- | -------- | ----------------------- |
| `engine` | `CrawlEngineHandle` | Yes      | The crawl engine handle |
| `url`    | `[:0]const u8`      | Yes      | The URL to fetch        |

**Returns:** `CrawlResult`
**Errors:** Throws `CrawlError`.

---

#### mapUrls()

Discover all pages on a website by following links and sitemaps.

**Signature:**

```zig
// Phase 1: zig backend signature generation
```

**Parameters:**

| Name     | Type                | Required | Description             |
| -------- | ------------------- | -------- | ----------------------- |
| `engine` | `CrawlEngineHandle` | Yes      | The crawl engine handle |
| `url`    | `[:0]const u8`      | Yes      | The URL to fetch        |

**Returns:** `MapResult`
**Errors:** Throws `CrawlError`.

---

#### batchScrape()

Scrape multiple URLs concurrently.

**Signature:**

```zig
// Phase 1: zig backend signature generation
```

**Parameters:**

| Name     | Type                   | Required | Description             |
| -------- | ---------------------- | -------- | ----------------------- |
| `engine` | `CrawlEngineHandle`    | Yes      | The crawl engine handle |
| `urls`   | `[]const [:0]const u8` | Yes      | The urls                |

**Returns:** `[]const BatchScrapeResult`
**Errors:** Throws `CrawlError`.

---

#### batchCrawl()

Crawl multiple seed URLs concurrently, each following links to configured depth.

**Signature:**

```zig
// Phase 1: zig backend signature generation
```

**Parameters:**

| Name     | Type                   | Required | Description             |
| -------- | ---------------------- | -------- | ----------------------- |
| `engine` | `CrawlEngineHandle`    | Yes      | The crawl engine handle |
| `urls`   | `[]const [:0]const u8` | Yes      | The urls                |

**Returns:** `[]const BatchCrawlResult`
**Errors:** Throws `CrawlError`.

---

### Types

#### ArticleMetadata

Article metadata extracted from `article:*` Open Graph tags.

| Field           | Type                   | Default | Description                    |
| --------------- | ---------------------- | ------- | ------------------------------ |
| `publishedTime` | `[:0]const u8?`        | `null`  | The article publication time.  |
| `modifiedTime`  | `[:0]const u8?`        | `null`  | The article modification time. |
| `author`        | `[:0]const u8?`        | `null`  | The article author.            |
| `section`       | `[:0]const u8?`        | `null`  | The article section.           |
| `tags`          | `[]const [:0]const u8` | `[]`    | The article tags.              |

---

#### BatchCrawlResult

Result from a single URL in a batch crawl operation.

| Field    | Type            | Default | Description                             |
| -------- | --------------- | ------- | --------------------------------------- |
| `url`    | `[:0]const u8`  | —       | The seed URL that was crawled.          |
| `result` | `CrawlResult?`  | `null`  | The crawl result, if successful.        |
| `error`  | `[:0]const u8?` | `null`  | The error message, if the crawl failed. |

---

#### BatchScrapeResult

Result from a single URL in a batch scrape operation.

| Field    | Type            | Default | Description                              |
| -------- | --------------- | ------- | ---------------------------------------- |
| `url`    | `[:0]const u8`  | —       | The URL that was scraped.                |
| `result` | `ScrapeResult?` | `null`  | The scrape result, if successful.        |
| `error`  | `[:0]const u8?` | `null`  | The error message, if the scrape failed. |

---

#### BrowserConfig

Browser fallback configuration.

| Field          | Type            | Default                   | Description                                                                    |
| -------------- | --------------- | ------------------------- | ------------------------------------------------------------------------------ |
| `mode`         | `BrowserMode`   | `BrowserMode.Auto`        | When to use the headless browser fallback.                                     |
| `endpoint`     | `[:0]const u8?` | `null`                    | CDP WebSocket endpoint for connecting to an external browser instance.         |
| `timeout`      | `i64`           | `30000ms`                 | Timeout for browser page load and rendering (in milliseconds when serialized). |
| `wait`         | `BrowserWait`   | `BrowserWait.NetworkIdle` | Wait strategy after browser navigation.                                        |
| `waitSelector` | `[:0]const u8?` | `null`                    | CSS selector to wait for when `wait` is `Selector`.                            |
| `extraWait`    | `i64?`          | `null`                    | Extra time to wait after the wait condition is met.                            |

##### Methods

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### CitationReference

| Field   | Type           | Default | Description |
| ------- | -------------- | ------- | ----------- |
| `index` | `u64`          | —       | Index       |
| `url`   | `[:0]const u8` | —       | Url         |
| `text`  | `[:0]const u8` | —       | Text        |

---

#### CitationResult

Result of citation conversion.

| Field        | Type                        | Default | Description                                         |
| ------------ | --------------------------- | ------- | --------------------------------------------------- |
| `content`    | `[:0]const u8`              | —       | Markdown with links replaced by numbered citations. |
| `references` | `[]const CitationReference` | `[]`    | Numbered reference list: (index, url, text).        |

---

#### ContentConfig

Content extraction and conversion configuration.

Controls how HTML is converted to the output format. Uses
html-to-markdown-rs as the conversion engine for all formats
(markdown, plain text, djot).

| Field                      | Type                   | Default      | Description                                                                                                                                                                                                                                                                                                                                         |
| -------------------------- | ---------------------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `outputFormat`             | `[:0]const u8`         | `"markdown"` | Output format: `"markdown"` (default), `"plain"`, `"djot"`.                                                                                                                                                                                                                                                                                         |
| `preprocessingPreset`      | `[:0]const u8`         | `"standard"` | Preprocessing aggressiveness: `"minimal"`, `"standard"` (default), `"aggressive"`. - Minimal: only scripts/styles removed. - Standard: also removes nav, nav-hinted headers/footers/asides, forms. - Aggressive: removes all footers/asides unconditionally.                                                                                        |
| `removeNavigation`         | `bool`                 | `true`       | Remove navigation elements (nav, breadcrumbs, menus). Default: `true`.                                                                                                                                                                                                                                                                              |
| `removeForms`              | `bool`                 | `true`       | Remove form elements. Default: `true`.                                                                                                                                                                                                                                                                                                              |
| `stripTags`                | `[]const [:0]const u8` | `[]`         | HTML tag names to strip (render children only, remove the tag wrapper). Default: `["noscript"]`.                                                                                                                                                                                                                                                    |
| `preserveTags`             | `[]const [:0]const u8` | `[]`         | HTML tag names to preserve as raw HTML in output.                                                                                                                                                                                                                                                                                                   |
| `excludeSelectors`         | `[]const [:0]const u8` | `[]`         | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the wrapper but keeps children), excluded elements and all descendants are dropped. Supports CSS selectors: `.class`, `#id`, `[attribute]`, compound selectors. Example: `[".cookie-banner", "#ad-container", "[role='complementary']"]` |
| `skipImages`               | `bool`                 | `false`      | Skip image elements in output. Default: `false`.                                                                                                                                                                                                                                                                                                    |
| `maxDepth`                 | `u64?`                 | `null`       | Max DOM traversal depth. Prevents stack overflow on deeply nested HTML.                                                                                                                                                                                                                                                                             |
| `wrap`                     | `bool`                 | `false`      | Enable line wrapping. Default: `false`.                                                                                                                                                                                                                                                                                                             |
| `wrapWidth`                | `u64`                  | `80`         | Wrap width when `wrap` is enabled. Default: `80`.                                                                                                                                                                                                                                                                                                   |
| `includeDocumentStructure` | `bool`                 | `true`       | Include document structure tree in output. Default: `true`.                                                                                                                                                                                                                                                                                         |

##### Methods

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### CookieInfo

Information about an HTTP cookie received from a response.

| Field    | Type            | Default | Description                      |
| -------- | --------------- | ------- | -------------------------------- |
| `name`   | `[:0]const u8`  | —       | The cookie name.                 |
| `value`  | `[:0]const u8`  | —       | The cookie value.                |
| `domain` | `[:0]const u8?` | `null`  | The cookie domain, if specified. |
| `path`   | `[:0]const u8?` | `null`  | The cookie path, if specified.   |

---

#### CrawlConfig

Configuration for crawl, scrape, and map operations.

| Field                | Type                              | Default   | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| -------------------- | --------------------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `maxDepth`           | `u64?`                            | `null`    | Maximum crawl depth (number of link hops from the start URL).                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `maxPages`           | `u64?`                            | `null`    | Maximum number of pages to crawl.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `maxConcurrent`      | `u64?`                            | `null`    | Maximum number of concurrent requests.                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `respectRobotsTxt`   | `bool`                            | `false`   | Whether to respect robots.txt directives.                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `softHttpErrors`     | `bool`                            | `false`   | When true, HTTP-level error responses (404 NotFound, 403 Forbidden, WAF blocks) are surfaced as `ScrapeResult` records with the matching `status_code` rather than raised as `CrawlError`. Default `false` preserves the historical throw-on-error contract for direct fetches. Independently of this flag, 404s reached at the end of a redirect chain are _always_ surfaced softly — the user opted into redirect-following, so receiving a 404 there is part of the normal flow rather than an unexpected error. |
| `userAgent`          | `[:0]const u8?`                   | `null`    | Custom user-agent string.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `stayOnDomain`       | `bool`                            | `false`   | Whether to restrict crawling to the same domain.                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `allowSubdomains`    | `bool`                            | `false`   | Whether to allow subdomains when `stay_on_domain` is true.                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `includePaths`       | `[]const [:0]const u8`            | `[]`      | Regex patterns for paths to include during crawling.                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `excludePaths`       | `[]const [:0]const u8`            | `[]`      | Regex patterns for paths to exclude during crawling.                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `customHeaders`      | `std.StringHashMap([:0]const u8)` | `{}`      | Custom HTTP headers to send with each request.                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `requestTimeout`     | `i64`                             | `30000ms` | Timeout for individual HTTP requests (in milliseconds when serialized).                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `rateLimitMs`        | `u64?`                            | `null`    | Per-domain rate limit in milliseconds. When set, enforces a minimum delay between requests to the same domain. Defaults to 200ms when `null`.                                                                                                                                                                                                                                                                                                                                                                       |
| `maxRedirects`       | `u64`                             | `10`      | Maximum number of redirects to follow.                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `retryCount`         | `u64`                             | `0`       | Number of retry attempts for failed requests.                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `retryCodes`         | `[]const u16`                     | `[]`      | HTTP status codes that should trigger a retry.                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `cookiesEnabled`     | `bool`                            | `false`   | Whether to enable cookie handling.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `auth`               | `AuthConfig?`                     | `null`    | Authentication configuration.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `maxBodySize`        | `u64?`                            | `null`    | Maximum response body size in bytes.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `removeTags`         | `[]const [:0]const u8`            | `[]`      | CSS selectors for tags to remove from HTML before processing.                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `content`            | `ContentConfig`                   | —         | Content extraction and conversion configuration.                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `mapLimit`           | `u64?`                            | `null`    | Maximum number of URLs to return from a map operation.                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `mapSearch`          | `[:0]const u8?`                   | `null`    | Search filter for map results (case-insensitive substring match on URLs).                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `downloadAssets`     | `bool`                            | `false`   | Whether to download assets (CSS, JS, images, etc.) from the page.                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `assetTypes`         | `[]const AssetCategory`           | `[]`      | Filter for asset categories to download.                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `maxAssetSize`       | `u64?`                            | `null`    | Maximum size in bytes for individual asset downloads.                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `browser`            | `BrowserConfig`                   | —         | Browser configuration.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `proxy`              | `ProxyConfig?`                    | `null`    | Proxy configuration for HTTP requests.                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `userAgents`         | `[]const [:0]const u8`            | `[]`      | List of user-agent strings for rotation. If non-empty, overrides `user_agent`.                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `captureScreenshot`  | `bool`                            | `false`   | Whether to capture a screenshot when using the browser.                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `downloadDocuments`  | `bool`                            | `true`    | Whether to download non-HTML documents (PDF, DOCX, images, code, etc.) instead of skipping them.                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `documentMaxSize`    | `u64?`                            | `null`    | Maximum size in bytes for document downloads. Defaults to 50 MB.                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `documentMimeTypes`  | `[]const [:0]const u8`            | `[]`      | Allowlist of MIME types to download. If empty, uses built-in defaults.                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `warcOutput`         | `[:0]const u8?`                   | `null`    | Path to write WARC output. If `null`, WARC output is disabled.                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `browserProfile`     | `[:0]const u8?`                   | `null`    | Named browser profile for persistent sessions (cookies, localStorage).                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `saveBrowserProfile` | `bool`                            | `false`   | Whether to save changes back to the browser profile on exit.                                                                                                                                                                                                                                                                                                                                                                                                                                                        |

##### Methods

###### default()

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

###### validate()

Validate the configuration, returning an error if any values are invalid.

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### CrawlEngineHandle

Opaque handle to a configured crawl engine.

Constructed via `create_engine` with an optional `CrawlConfig`.
Default implementations for all pluggable components are used internally.

---

#### CrawlPageResult

The result of crawling a single page during a crawl operation.

| Field                | Type                  | Default | Description                                                                |
| -------------------- | --------------------- | ------- | -------------------------------------------------------------------------- |
| `url`                | `[:0]const u8`        | —       | The original URL of the page.                                              |
| `normalizedUrl`      | `[:0]const u8`        | —       | The normalized URL of the page.                                            |
| `statusCode`         | `u16`                 | —       | The HTTP status code of the response.                                      |
| `contentType`        | `[:0]const u8`        | —       | The Content-Type header value.                                             |
| `html`               | `[:0]const u8`        | —       | The HTML body of the response.                                             |
| `bodySize`           | `u64`                 | —       | The size of the response body in bytes.                                    |
| `metadata`           | `PageMetadata`        | —       | Extracted metadata from the page.                                          |
| `links`              | `[]const LinkInfo`    | `[]`    | Links found on the page.                                                   |
| `images`             | `[]const ImageInfo`   | `[]`    | Images found on the page.                                                  |
| `feeds`              | `[]const FeedInfo`    | `[]`    | Feed links found on the page.                                              |
| `jsonLd`             | `[]const JsonLdEntry` | `[]`    | JSON-LD entries found on the page.                                         |
| `depth`              | `u64`                 | —       | The depth of this page from the start URL.                                 |
| `stayedOnDomain`     | `bool`                | —       | Whether this page is on the same domain as the start URL.                  |
| `wasSkipped`         | `bool`                | —       | Whether this page was skipped (binary or PDF content).                     |
| `isPdf`              | `bool`                | —       | Whether the content is a PDF.                                              |
| `detectedCharset`    | `[:0]const u8?`       | `null`  | The detected character set encoding.                                       |
| `markdown`           | `MarkdownResult?`     | `null`  | Markdown conversion of the page content.                                   |
| `extractedData`      | `[:0]const u8?`       | `null`  | Structured data extracted by LLM. Populated when extraction is configured. |
| `extractionMeta`     | `ExtractionMeta?`     | `null`  | Metadata about the LLM extraction pass (cost, tokens, model).              |
| `downloadedDocument` | `DownloadedDocument?` | `null`  | Downloaded non-HTML document (PDF, DOCX, image, code, etc.).               |

---

#### CrawlResult

The result of a multi-page crawl operation.

| Field            | Type                      | Default | Description                                                               |
| ---------------- | ------------------------- | ------- | ------------------------------------------------------------------------- |
| `pages`          | `[]const CrawlPageResult` | `[]`    | The list of crawled pages.                                                |
| `finalUrl`       | `[:0]const u8`            | —       | The final URL after following redirects.                                  |
| `redirectCount`  | `u64`                     | —       | The number of redirects followed.                                         |
| `wasSkipped`     | `bool`                    | —       | Whether any page was skipped during crawling.                             |
| `error`          | `[:0]const u8?`           | `null`  | An error message, if the crawl encountered an issue.                      |
| `cookies`        | `[]const CookieInfo`      | `[]`    | Cookies collected during the crawl.                                       |
| `normalizedUrls` | `[]const [:0]const u8`    | `[]`    | Normalized URLs encountered during crawling (for deduplication counting). |

##### Methods

###### uniqueNormalizedUrls()

Returns the count of unique normalized URLs encountered during crawling.

**Signature:**

```zig
// Phase 1: zig backend method signature generation
```

---

#### DownloadedAsset

A downloaded asset from a page.

| Field           | Type            | Default               | Description                                                              |
| --------------- | --------------- | --------------------- | ------------------------------------------------------------------------ |
| `url`           | `[:0]const u8`  | —                     | The original URL of the asset.                                           |
| `contentHash`   | `[:0]const u8`  | —                     | The SHA-256 content hash of the asset.                                   |
| `mimeType`      | `[:0]const u8?` | `null`                | The MIME type from the Content-Type header.                              |
| `size`          | `u64`           | —                     | The size of the asset in bytes.                                          |
| `assetCategory` | `AssetCategory` | `AssetCategory.Image` | The category of the asset.                                               |
| `htmlTag`       | `[:0]const u8?` | `null`                | The HTML tag that referenced this asset (e.g., "link", "script", "img"). |

---

#### DownloadedDocument

A downloaded non-HTML document (PDF, DOCX, image, code file, etc.).

When the crawler encounters non-HTML content and `download_documents` is
enabled, it downloads the raw bytes and populates this struct instead of
skipping the resource.

| Field         | Type                              | Default | Description                                              |
| ------------- | --------------------------------- | ------- | -------------------------------------------------------- |
| `url`         | `[:0]const u8`                    | —       | The URL the document was fetched from.                   |
| `mimeType`    | `[:0]const u8`                    | —       | The MIME type from the Content-Type header.              |
| `content`     | `[]const u8`                      | —       | Raw document bytes. Skipped during JSON serialization.   |
| `size`        | `u64`                             | —       | Size of the document in bytes.                           |
| `filename`    | `[:0]const u8?`                   | `null`  | Filename extracted from Content-Disposition or URL path. |
| `contentHash` | `[:0]const u8`                    | —       | SHA-256 hex digest of the content.                       |
| `headers`     | `std.StringHashMap([:0]const u8)` | `{}`    | Selected response headers.                               |

---

#### ExtractionMeta

Metadata about an LLM extraction pass.

| Field              | Type            | Default | Description                                     |
| ------------------ | --------------- | ------- | ----------------------------------------------- |
| `cost`             | `f64?`          | `null`  | Estimated cost of the LLM call in USD.          |
| `promptTokens`     | `u64?`          | `null`  | Number of prompt (input) tokens consumed.       |
| `completionTokens` | `u64?`          | `null`  | Number of completion (output) tokens generated. |
| `model`            | `[:0]const u8?` | `null`  | The model identifier used for extraction.       |
| `chunksProcessed`  | `u64`           | —       | Number of content chunks sent to the LLM.       |

---

#### FaviconInfo

Information about a favicon or icon link.

| Field      | Type            | Default | Description                                             |
| ---------- | --------------- | ------- | ------------------------------------------------------- |
| `url`      | `[:0]const u8`  | —       | The icon URL.                                           |
| `rel`      | `[:0]const u8`  | —       | The `rel` attribute (e.g., "icon", "apple-touch-icon"). |
| `sizes`    | `[:0]const u8?` | `null`  | The `sizes` attribute, if present.                      |
| `mimeType` | `[:0]const u8?` | `null`  | The MIME type, if present.                              |

---

#### FeedInfo

Information about a feed link found on a page.

| Field      | Type            | Default        | Description                 |
| ---------- | --------------- | -------------- | --------------------------- |
| `url`      | `[:0]const u8`  | —              | The feed URL.               |
| `title`    | `[:0]const u8?` | `null`         | The feed title, if present. |
| `feedType` | `FeedType`      | `FeedType.Rss` | The type of feed.           |

---

#### HeadingInfo

A heading element extracted from the page.

| Field   | Type           | Default | Description               |
| ------- | -------------- | ------- | ------------------------- |
| `level` | `u8`           | —       | The heading level (1-6).  |
| `text`  | `[:0]const u8` | —       | The heading text content. |

---

#### HreflangEntry

An hreflang alternate link entry.

| Field  | Type           | Default | Description                                        |
| ------ | -------------- | ------- | -------------------------------------------------- |
| `lang` | `[:0]const u8` | —       | The language code (e.g., "en", "fr", "x-default"). |
| `url`  | `[:0]const u8` | —       | The URL for this language variant.                 |

---

#### ImageInfo

Information about an image found on a page.

| Field    | Type            | Default           | Description                                     |
| -------- | --------------- | ----------------- | ----------------------------------------------- |
| `url`    | `[:0]const u8`  | —                 | The image URL.                                  |
| `alt`    | `[:0]const u8?` | `null`            | The alt text, if present.                       |
| `width`  | `u32?`          | `null`            | The width attribute, if present and parseable.  |
| `height` | `u32?`          | `null`            | The height attribute, if present and parseable. |
| `source` | `ImageSource`   | `ImageSource.Img` | The source of the image reference.              |

---

#### JsonLdEntry

A JSON-LD structured data entry found on a page.

| Field        | Type            | Default | Description                                |
| ------------ | --------------- | ------- | ------------------------------------------ |
| `schemaType` | `[:0]const u8`  | —       | The `@type` value from the JSON-LD object. |
| `name`       | `[:0]const u8?` | `null`  | The `name` value, if present.              |
| `raw`        | `[:0]const u8`  | —       | The raw JSON-LD string.                    |

---

#### LinkInfo

Information about a link found on a page.

| Field      | Type            | Default             | Description                            |
| ---------- | --------------- | ------------------- | -------------------------------------- |
| `url`      | `[:0]const u8`  | —                   | The resolved URL of the link.          |
| `text`     | `[:0]const u8`  | —                   | The visible text of the link.          |
| `linkType` | `LinkType`      | `LinkType.Internal` | The classification of the link.        |
| `rel`      | `[:0]const u8?` | `null`              | The `rel` attribute value, if present. |
| `nofollow` | `bool`          | —                   | Whether the link has `rel="nofollow"`. |

---

#### MapResult

The result of a map operation, containing discovered URLs.

| Field  | Type                 | Default | Description                  |
| ------ | -------------------- | ------- | ---------------------------- |
| `urls` | `[]const SitemapUrl` | `[]`    | The list of discovered URLs. |

---

#### MarkdownResult

Rich markdown conversion result from HTML processing.

| Field               | Type                   | Default | Description                                              |
| ------------------- | ---------------------- | ------- | -------------------------------------------------------- |
| `content`           | `[:0]const u8`         | —       | Converted markdown text.                                 |
| `documentStructure` | `[:0]const u8?`        | `null`  | Structured document tree with semantic nodes.            |
| `tables`            | `[]const [:0]const u8` | `[]`    | Extracted tables with structured cell data.              |
| `warnings`          | `[]const [:0]const u8` | `[]`    | Non-fatal processing warnings.                           |
| `citations`         | `CitationResult?`      | `null`  | Content with links replaced by numbered citations.       |
| `fitContent`        | `[:0]const u8?`        | `null`  | Content-filtered markdown optimized for LLM consumption. |

---

#### PageMetadata

Metadata extracted from an HTML page's `<meta>` tags and `<title>` element.

| Field                | Type                     | Default | Description                                        |
| -------------------- | ------------------------ | ------- | -------------------------------------------------- |
| `title`              | `[:0]const u8?`          | `null`  | The page title from the `<title>` element.         |
| `description`        | `[:0]const u8?`          | `null`  | The meta description.                              |
| `canonicalUrl`       | `[:0]const u8?`          | `null`  | The canonical URL from `<link rel="canonical">`.   |
| `keywords`           | `[:0]const u8?`          | `null`  | Keywords from `<meta name="keywords">`.            |
| `author`             | `[:0]const u8?`          | `null`  | Author from `<meta name="author">`.                |
| `viewport`           | `[:0]const u8?`          | `null`  | Viewport content from `<meta name="viewport">`.    |
| `themeColor`         | `[:0]const u8?`          | `null`  | Theme color from `<meta name="theme-color">`.      |
| `generator`          | `[:0]const u8?`          | `null`  | Generator from `<meta name="generator">`.          |
| `robots`             | `[:0]const u8?`          | `null`  | Robots content from `<meta name="robots">`.        |
| `htmlLang`           | `[:0]const u8?`          | `null`  | The `lang` attribute from the `<html>` element.    |
| `htmlDir`            | `[:0]const u8?`          | `null`  | The `dir` attribute from the `<html>` element.     |
| `ogTitle`            | `[:0]const u8?`          | `null`  | Open Graph title.                                  |
| `ogType`             | `[:0]const u8?`          | `null`  | Open Graph type.                                   |
| `ogImage`            | `[:0]const u8?`          | `null`  | Open Graph image URL.                              |
| `ogDescription`      | `[:0]const u8?`          | `null`  | Open Graph description.                            |
| `ogUrl`              | `[:0]const u8?`          | `null`  | Open Graph URL.                                    |
| `ogSiteName`         | `[:0]const u8?`          | `null`  | Open Graph site name.                              |
| `ogLocale`           | `[:0]const u8?`          | `null`  | Open Graph locale.                                 |
| `ogVideo`            | `[:0]const u8?`          | `null`  | Open Graph video URL.                              |
| `ogAudio`            | `[:0]const u8?`          | `null`  | Open Graph audio URL.                              |
| `ogLocaleAlternates` | `[]const [:0]const u8?`  | `[]`    | Open Graph locale alternates.                      |
| `twitterCard`        | `[:0]const u8?`          | `null`  | Twitter card type.                                 |
| `twitterTitle`       | `[:0]const u8?`          | `null`  | Twitter title.                                     |
| `twitterDescription` | `[:0]const u8?`          | `null`  | Twitter description.                               |
| `twitterImage`       | `[:0]const u8?`          | `null`  | Twitter image URL.                                 |
| `twitterSite`        | `[:0]const u8?`          | `null`  | Twitter site handle.                               |
| `twitterCreator`     | `[:0]const u8?`          | `null`  | Twitter creator handle.                            |
| `dcTitle`            | `[:0]const u8?`          | `null`  | Dublin Core title.                                 |
| `dcCreator`          | `[:0]const u8?`          | `null`  | Dublin Core creator.                               |
| `dcSubject`          | `[:0]const u8?`          | `null`  | Dublin Core subject.                               |
| `dcDescription`      | `[:0]const u8?`          | `null`  | Dublin Core description.                           |
| `dcPublisher`        | `[:0]const u8?`          | `null`  | Dublin Core publisher.                             |
| `dcDate`             | `[:0]const u8?`          | `null`  | Dublin Core date.                                  |
| `dcType`             | `[:0]const u8?`          | `null`  | Dublin Core type.                                  |
| `dcFormat`           | `[:0]const u8?`          | `null`  | Dublin Core format.                                |
| `dcIdentifier`       | `[:0]const u8?`          | `null`  | Dublin Core identifier.                            |
| `dcLanguage`         | `[:0]const u8?`          | `null`  | Dublin Core language.                              |
| `dcRights`           | `[:0]const u8?`          | `null`  | Dublin Core rights.                                |
| `article`            | `ArticleMetadata?`       | `null`  | Article metadata from `article:*` Open Graph tags. |
| `hreflangs`          | `[]const HreflangEntry?` | `[]`    | Hreflang alternate links.                          |
| `favicons`           | `[]const FaviconInfo?`   | `[]`    | Favicon and icon links.                            |
| `headings`           | `[]const HeadingInfo?`   | `[]`    | Heading elements (h1-h6).                          |
| `wordCount`          | `u64?`                   | `null`  | Computed word count of the page body text.         |

---

#### ProxyConfig

Proxy configuration for HTTP requests.

| Field      | Type            | Default | Description                                                    |
| ---------- | --------------- | ------- | -------------------------------------------------------------- |
| `url`      | `[:0]const u8`  | —       | Proxy URL (e.g. "<http://proxy:8080",> "socks5://proxy:1080"). |
| `username` | `[:0]const u8?` | `null`  | Optional username for proxy authentication.                    |
| `password` | `[:0]const u8?` | `null`  | Optional password for proxy authentication.                    |

---

#### ResponseMeta

Response metadata extracted from HTTP headers.

| Field             | Type            | Default | Description                        |
| ----------------- | --------------- | ------- | ---------------------------------- |
| `etag`            | `[:0]const u8?` | `null`  | The ETag header value.             |
| `lastModified`    | `[:0]const u8?` | `null`  | The Last-Modified header value.    |
| `cacheControl`    | `[:0]const u8?` | `null`  | The Cache-Control header value.    |
| `server`          | `[:0]const u8?` | `null`  | The Server header value.           |
| `xPoweredBy`      | `[:0]const u8?` | `null`  | The X-Powered-By header value.     |
| `contentLanguage` | `[:0]const u8?` | `null`  | The Content-Language header value. |
| `contentEncoding` | `[:0]const u8?` | `null`  | The Content-Encoding header value. |

---

#### ScrapeResult

The result of a single-page scrape operation.

| Field                | Type                      | Default | Description                                                                                            |
| -------------------- | ------------------------- | ------- | ------------------------------------------------------------------------------------------------------ |
| `statusCode`         | `u16`                     | —       | The HTTP status code of the response.                                                                  |
| `contentType`        | `[:0]const u8`            | —       | The Content-Type header value.                                                                         |
| `html`               | `[:0]const u8`            | —       | The HTML body of the response.                                                                         |
| `bodySize`           | `u64`                     | —       | The size of the response body in bytes.                                                                |
| `metadata`           | `PageMetadata`            | —       | Extracted metadata from the page.                                                                      |
| `links`              | `[]const LinkInfo`        | `[]`    | Links found on the page.                                                                               |
| `images`             | `[]const ImageInfo`       | `[]`    | Images found on the page.                                                                              |
| `feeds`              | `[]const FeedInfo`        | `[]`    | Feed links found on the page.                                                                          |
| `jsonLd`             | `[]const JsonLdEntry`     | `[]`    | JSON-LD entries found on the page.                                                                     |
| `isAllowed`          | `bool`                    | —       | Whether the URL is allowed by robots.txt.                                                              |
| `crawlDelay`         | `u64?`                    | `null`  | The crawl delay from robots.txt, in seconds.                                                           |
| `noindexDetected`    | `bool`                    | —       | Whether a noindex directive was detected.                                                              |
| `nofollowDetected`   | `bool`                    | —       | Whether a nofollow directive was detected.                                                             |
| `xRobotsTag`         | `[:0]const u8?`           | `null`  | The X-Robots-Tag header value, if present.                                                             |
| `isPdf`              | `bool`                    | —       | Whether the content is a PDF.                                                                          |
| `wasSkipped`         | `bool`                    | —       | Whether the page was skipped (binary or PDF content).                                                  |
| `detectedCharset`    | `[:0]const u8?`           | `null`  | The detected character set encoding.                                                                   |
| `authHeaderSent`     | `bool`                    | —       | Whether an authentication header was sent with the request.                                            |
| `responseMeta`       | `ResponseMeta?`           | `null`  | Response metadata extracted from HTTP headers.                                                         |
| `assets`             | `[]const DownloadedAsset` | `[]`    | Downloaded assets from the page.                                                                       |
| `jsRenderHint`       | `bool`                    | —       | Whether the page content suggests JavaScript rendering is needed.                                      |
| `browserUsed`        | `bool`                    | —       | Whether the browser fallback was used to fetch this page.                                              |
| `markdown`           | `MarkdownResult?`         | `null`  | Markdown conversion of the page content.                                                               |
| `extractedData`      | `[:0]const u8?`           | `null`  | Structured data extracted by LLM. Populated when extraction is configured.                             |
| `extractionMeta`     | `ExtractionMeta?`         | `null`  | Metadata about the LLM extraction pass (cost, tokens, model).                                          |
| `screenshot`         | `[]const u8?`             | `null`  | Screenshot of the page as PNG bytes. Populated when browser is used and capture_screenshot is enabled. |
| `downloadedDocument` | `DownloadedDocument?`     | `null`  | Downloaded non-HTML document (PDF, DOCX, image, code, etc.).                                           |

---

#### SitemapUrl

A URL entry from a sitemap.

| Field        | Type            | Default | Description                             |
| ------------ | --------------- | ------- | --------------------------------------- |
| `url`        | `[:0]const u8`  | —       | The URL.                                |
| `lastmod`    | `[:0]const u8?` | `null`  | The last modification date, if present. |
| `changefreq` | `[:0]const u8?` | `null`  | The change frequency, if present.       |
| `priority`   | `[:0]const u8?` | `null`  | The priority, if present.               |

---

### Enums

#### BrowserMode

When to use the headless browser fallback.

| Value    | Description                                                                |
| -------- | -------------------------------------------------------------------------- |
| `Auto`   | Automatically detect when JS rendering is needed and fall back to browser. |
| `Always` | Always use the browser for every request.                                  |
| `Never`  | Never use the browser fallback.                                            |

---

#### BrowserWait

Wait strategy for browser page rendering.

| Value         | Description                                            |
| ------------- | ------------------------------------------------------ |
| `NetworkIdle` | Wait until network activity is idle.                   |
| `Selector`    | Wait for a specific CSS selector to appear in the DOM. |
| `Fixed`       | Wait for a fixed duration after navigation.            |

---

#### AuthConfig

Authentication configuration.

| Value    | Description                                                                                 |
| -------- | ------------------------------------------------------------------------------------------- |
| `Basic`  | HTTP Basic authentication. — Fields: `username`: `[:0]const u8`, `password`: `[:0]const u8` |
| `Bearer` | Bearer token authentication. — Fields: `token`: `[:0]const u8`                              |
| `Header` | Custom authentication header. — Fields: `name`: `[:0]const u8`, `value`: `[:0]const u8`     |

---

#### LinkType

The classification of a link.

| Value      | Description                                         |
| ---------- | --------------------------------------------------- |
| `Internal` | A link to the same domain.                          |
| `External` | A link to a different domain.                       |
| `Anchor`   | A fragment-only link (e.g., `#section`).            |
| `Document` | A link to a downloadable document (PDF, DOC, etc.). |

---

#### ImageSource

The source of an image reference.

| Value           | Description                          |
| --------------- | ------------------------------------ |
| `Img`           | An `<img>` tag.                      |
| `PictureSource` | A `<source>` tag inside `<picture>`. |
| `OgImage`       | An `og:image` meta tag.              |
| `TwitterImage`  | A `twitter:image` meta tag.          |

---

#### FeedType

The type of a feed (RSS, Atom, or JSON Feed).

| Value      | Description |
| ---------- | ----------- |
| `Rss`      | RSS feed.   |
| `Atom`     | Atom feed.  |
| `JsonFeed` | JSON Feed.  |

---

#### AssetCategory

The category of a downloaded asset.

| Value        | Description                         |
| ------------ | ----------------------------------- |
| `Document`   | A document file (PDF, DOC, etc.).   |
| `Image`      | An image file.                      |
| `Audio`      | An audio file.                      |
| `Video`      | A video file.                       |
| `Font`       | A font file.                        |
| `Stylesheet` | A CSS stylesheet.                   |
| `Script`     | A JavaScript file.                  |
| `Archive`    | An archive file (ZIP, TAR, etc.).   |
| `Data`       | A data file (JSON, XML, CSV, etc.). |
| `Other`      | An unrecognized asset type.         |

---

### Errors

#### CrawlError

Errors that can occur during crawling, scraping, or mapping operations.

| Variant          | Description                                                                        |
| ---------------- | ---------------------------------------------------------------------------------- |
| `NotFound`       | The requested page was not found (HTTP 404).                                       |
| `Unauthorized`   | The request was unauthorized (HTTP 401).                                           |
| `Forbidden`      | The request was forbidden (HTTP 403).                                              |
| `WafBlocked`     | The request was blocked by a WAF or bot protection (HTTP 403 with WAF indicators). |
| `Timeout`        | The request timed out.                                                             |
| `RateLimited`    | The request was rate-limited (HTTP 429).                                           |
| `ServerError`    | A server error occurred (HTTP 5xx).                                                |
| `BadGateway`     | A bad gateway error occurred (HTTP 502).                                           |
| `Gone`           | The resource is permanently gone (HTTP 410).                                       |
| `Connection`     | A connection error occurred.                                                       |
| `Dns`            | A DNS resolution error occurred.                                                   |
| `Ssl`            | An SSL/TLS error occurred.                                                         |
| `DataLoss`       | Data was lost or truncated during transfer.                                        |
| `BrowserError`   | The browser failed to launch, connect, or navigate.                                |
| `BrowserTimeout` | The browser page load or rendering timed out.                                      |
| `InvalidConfig`  | The provided configuration is invalid.                                             |
| `Other`          | An unclassified error occurred.                                                    |

---
