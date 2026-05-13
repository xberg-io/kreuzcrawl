---
title: "Kotlin API Reference"
---

## Kotlin API Reference <span class="version-badge">v0.3.0-rc.19</span>

### Functions

#### createEngine()

Create a new crawl engine with the given configuration.

If `config` is `null`, uses `CrawlConfig.default()`.
Returns an error if the configuration is invalid.

**Signature:**

```kotlin
// Phase 1: kotlin backend signature generation
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

```kotlin
// Phase 1: kotlin backend signature generation
```

**Parameters:**

| Name     | Type                | Required | Description             |
| -------- | ------------------- | -------- | ----------------------- |
| `engine` | `CrawlEngineHandle` | Yes      | The crawl engine handle |
| `url`    | `String`            | Yes      | The URL to fetch        |

**Returns:** `ScrapeResult`
**Errors:** Throws `CrawlError`.

---

#### crawl()

Crawl a website starting from `url`, following links up to the configured depth.

**Signature:**

```kotlin
// Phase 1: kotlin backend signature generation
```

**Parameters:**

| Name     | Type                | Required | Description             |
| -------- | ------------------- | -------- | ----------------------- |
| `engine` | `CrawlEngineHandle` | Yes      | The crawl engine handle |
| `url`    | `String`            | Yes      | The URL to fetch        |

**Returns:** `CrawlResult`
**Errors:** Throws `CrawlError`.

---

#### mapUrls()

Discover all pages on a website by following links and sitemaps.

**Signature:**

```kotlin
// Phase 1: kotlin backend signature generation
```

**Parameters:**

| Name     | Type                | Required | Description             |
| -------- | ------------------- | -------- | ----------------------- |
| `engine` | `CrawlEngineHandle` | Yes      | The crawl engine handle |
| `url`    | `String`            | Yes      | The URL to fetch        |

**Returns:** `MapResult`
**Errors:** Throws `CrawlError`.

---

#### batchScrape()

Scrape multiple URLs concurrently.

**Signature:**

```kotlin
// Phase 1: kotlin backend signature generation
```

**Parameters:**

| Name     | Type                | Required | Description             |
| -------- | ------------------- | -------- | ----------------------- |
| `engine` | `CrawlEngineHandle` | Yes      | The crawl engine handle |
| `urls`   | `List<String>`      | Yes      | The urls                |

**Returns:** `List<BatchScrapeResult>`
**Errors:** Throws `CrawlError`.

---

#### batchCrawl()

Crawl multiple seed URLs concurrently, each following links to configured depth.

**Signature:**

```kotlin
// Phase 1: kotlin backend signature generation
```

**Parameters:**

| Name     | Type                | Required | Description             |
| -------- | ------------------- | -------- | ----------------------- |
| `engine` | `CrawlEngineHandle` | Yes      | The crawl engine handle |
| `urls`   | `List<String>`      | Yes      | The urls                |

**Returns:** `List<BatchCrawlResult>`
**Errors:** Throws `CrawlError`.

---

### Types

#### ArticleMetadata

Article metadata extracted from `article:*` Open Graph tags.

| Field           | Type           | Default | Description                    |
| --------------- | -------------- | ------- | ------------------------------ |
| `publishedTime` | `String?`      | `null`  | The article publication time.  |
| `modifiedTime`  | `String?`      | `null`  | The article modification time. |
| `author`        | `String?`      | `null`  | The article author.            |
| `section`       | `String?`      | `null`  | The article section.           |
| `tags`          | `List<String>` | `[]`    | The article tags.              |

---

#### BatchCrawlResult

Result from a single URL in a batch crawl operation.

| Field    | Type           | Default | Description                             |
| -------- | -------------- | ------- | --------------------------------------- |
| `url`    | `String`       | —       | The seed URL that was crawled.          |
| `result` | `CrawlResult?` | `null`  | The crawl result, if successful.        |
| `error`  | `String?`      | `null`  | The error message, if the crawl failed. |

---

#### BatchScrapeResult

Result from a single URL in a batch scrape operation.

| Field    | Type            | Default | Description                              |
| -------- | --------------- | ------- | ---------------------------------------- |
| `url`    | `String`        | —       | The URL that was scraped.                |
| `result` | `ScrapeResult?` | `null`  | The scrape result, if successful.        |
| `error`  | `String?`       | `null`  | The error message, if the scrape failed. |

---

#### BrowserConfig

Browser fallback configuration.

| Field          | Type          | Default                   | Description                                                                    |
| -------------- | ------------- | ------------------------- | ------------------------------------------------------------------------------ |
| `mode`         | `BrowserMode` | `BrowserMode.Auto`        | When to use the headless browser fallback.                                     |
| `endpoint`     | `String?`     | `null`                    | CDP WebSocket endpoint for connecting to an external browser instance.         |
| `timeout`      | `Duration`    | `30000ms`                 | Timeout for browser page load and rendering (in milliseconds when serialized). |
| `wait`         | `BrowserWait` | `BrowserWait.NetworkIdle` | Wait strategy after browser navigation.                                        |
| `waitSelector` | `String?`     | `null`                    | CSS selector to wait for when `wait` is `Selector`.                            |
| `extraWait`    | `Duration?`   | `null`                    | Extra time to wait after the wait condition is met.                            |

##### Methods

###### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

#### CitationReference

| Field   | Type     | Default | Description |
| ------- | -------- | ------- | ----------- |
| `index` | `Long`   | —       | Index       |
| `url`   | `String` | —       | Url         |
| `text`  | `String` | —       | Text        |

---

#### CitationResult

Result of citation conversion.

| Field        | Type                      | Default | Description                                         |
| ------------ | ------------------------- | ------- | --------------------------------------------------- |
| `content`    | `String`                  | —       | Markdown with links replaced by numbered citations. |
| `references` | `List<CitationReference>` | `[]`    | Numbered reference list: (index, url, text).        |

---

#### ContentConfig

Content extraction and conversion configuration.

Controls how HTML is converted to the output format. Uses
html-to-markdown-rs as the conversion engine for all formats
(markdown, plain text, djot).

| Field                      | Type           | Default      | Description                                                                                                                                                                                                                                                                                                                                         |
| -------------------------- | -------------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `outputFormat`             | `String`       | `"markdown"` | Output format: `"markdown"` (default), `"plain"`, `"djot"`.                                                                                                                                                                                                                                                                                         |
| `preprocessingPreset`      | `String`       | `"standard"` | Preprocessing aggressiveness: `"minimal"`, `"standard"` (default), `"aggressive"`. - Minimal: only scripts/styles removed. - Standard: also removes nav, nav-hinted headers/footers/asides, forms. - Aggressive: removes all footers/asides unconditionally.                                                                                        |
| `removeNavigation`         | `Boolean`      | `true`       | Remove navigation elements (nav, breadcrumbs, menus). Default: `true`.                                                                                                                                                                                                                                                                              |
| `removeForms`              | `Boolean`      | `true`       | Remove form elements. Default: `true`.                                                                                                                                                                                                                                                                                                              |
| `stripTags`                | `List<String>` | `[]`         | HTML tag names to strip (render children only, remove the tag wrapper). Default: `["noscript"]`.                                                                                                                                                                                                                                                    |
| `preserveTags`             | `List<String>` | `[]`         | HTML tag names to preserve as raw HTML in output.                                                                                                                                                                                                                                                                                                   |
| `excludeSelectors`         | `List<String>` | `[]`         | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the wrapper but keeps children), excluded elements and all descendants are dropped. Supports CSS selectors: `.class`, `#id`, `[attribute]`, compound selectors. Example: `[".cookie-banner", "#ad-container", "[role='complementary']"]` |
| `skipImages`               | `Boolean`      | `false`      | Skip image elements in output. Default: `false`.                                                                                                                                                                                                                                                                                                    |
| `maxDepth`                 | `Long?`        | `null`       | Max DOM traversal depth. Prevents stack overflow on deeply nested HTML.                                                                                                                                                                                                                                                                             |
| `wrap`                     | `Boolean`      | `false`      | Enable line wrapping. Default: `false`.                                                                                                                                                                                                                                                                                                             |
| `wrapWidth`                | `Long`         | `80`         | Wrap width when `wrap` is enabled. Default: `80`.                                                                                                                                                                                                                                                                                                   |
| `includeDocumentStructure` | `Boolean`      | `true`       | Include document structure tree in output. Default: `true`.                                                                                                                                                                                                                                                                                         |

##### Methods

###### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

#### CookieInfo

Information about an HTTP cookie received from a response.

| Field    | Type      | Default | Description                      |
| -------- | --------- | ------- | -------------------------------- |
| `name`   | `String`  | —       | The cookie name.                 |
| `value`  | `String`  | —       | The cookie value.                |
| `domain` | `String?` | `null`  | The cookie domain, if specified. |
| `path`   | `String?` | `null`  | The cookie path, if specified.   |

---

#### CrawlConfig

Configuration for crawl, scrape, and map operations.

| Field                | Type                  | Default   | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| -------------------- | --------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `maxDepth`           | `Long?`               | `null`    | Maximum crawl depth (number of link hops from the start URL).                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `maxPages`           | `Long?`               | `null`    | Maximum number of pages to crawl.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `maxConcurrent`      | `Long?`               | `null`    | Maximum number of concurrent requests.                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `respectRobotsTxt`   | `Boolean`             | `false`   | Whether to respect robots.txt directives.                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `softHttpErrors`     | `Boolean`             | `false`   | When true, HTTP-level error responses (404 NotFound, 403 Forbidden, WAF blocks) are surfaced as `ScrapeResult` records with the matching `status_code` rather than raised as `CrawlError`. Default `false` preserves the historical throw-on-error contract for direct fetches. Independently of this flag, 404s reached at the end of a redirect chain are _always_ surfaced softly — the user opted into redirect-following, so receiving a 404 there is part of the normal flow rather than an unexpected error. |
| `userAgent`          | `String?`             | `null`    | Custom user-agent string.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `stayOnDomain`       | `Boolean`             | `false`   | Whether to restrict crawling to the same domain.                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `allowSubdomains`    | `Boolean`             | `false`   | Whether to allow subdomains when `stay_on_domain` is true.                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `includePaths`       | `List<String>`        | `[]`      | Regex patterns for paths to include during crawling.                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `excludePaths`       | `List<String>`        | `[]`      | Regex patterns for paths to exclude during crawling.                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `customHeaders`      | `Map<String, String>` | `{}`      | Custom HTTP headers to send with each request.                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `requestTimeout`     | `Duration`            | `30000ms` | Timeout for individual HTTP requests (in milliseconds when serialized).                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `rateLimitMs`        | `Long?`               | `null`    | Per-domain rate limit in milliseconds. When set, enforces a minimum delay between requests to the same domain. Defaults to 200ms when `null`.                                                                                                                                                                                                                                                                                                                                                                       |
| `maxRedirects`       | `Long`                | `10`      | Maximum number of redirects to follow.                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `retryCount`         | `Long`                | `0`       | Number of retry attempts for failed requests.                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `retryCodes`         | `List<Short>`         | `[]`      | HTTP status codes that should trigger a retry.                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `cookiesEnabled`     | `Boolean`             | `false`   | Whether to enable cookie handling.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `auth`               | `AuthConfig?`         | `null`    | Authentication configuration.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `maxBodySize`        | `Long?`               | `null`    | Maximum response body size in bytes.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `removeTags`         | `List<String>`        | `[]`      | CSS selectors for tags to remove from HTML before processing.                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `content`            | `ContentConfig`       | —         | Content extraction and conversion configuration.                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `mapLimit`           | `Long?`               | `null`    | Maximum number of URLs to return from a map operation.                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `mapSearch`          | `String?`             | `null`    | Search filter for map results (case-insensitive substring match on URLs).                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `downloadAssets`     | `Boolean`             | `false`   | Whether to download assets (CSS, JS, images, etc.) from the page.                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `assetTypes`         | `List<AssetCategory>` | `[]`      | Filter for asset categories to download.                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `maxAssetSize`       | `Long?`               | `null`    | Maximum size in bytes for individual asset downloads.                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `browser`            | `BrowserConfig`       | —         | Browser configuration.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `proxy`              | `ProxyConfig?`        | `null`    | Proxy configuration for HTTP requests.                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `userAgents`         | `List<String>`        | `[]`      | List of user-agent strings for rotation. If non-empty, overrides `user_agent`.                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `captureScreenshot`  | `Boolean`             | `false`   | Whether to capture a screenshot when using the browser.                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `downloadDocuments`  | `Boolean`             | `true`    | Whether to download non-HTML documents (PDF, DOCX, images, code, etc.) instead of skipping them.                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `documentMaxSize`    | `Long?`               | `null`    | Maximum size in bytes for document downloads. Defaults to 50 MB.                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `documentMimeTypes`  | `List<String>`        | `[]`      | Allowlist of MIME types to download. If empty, uses built-in defaults.                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `warcOutput`         | `Path?`               | `null`    | Path to write WARC output. If `null`, WARC output is disabled.                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `browserProfile`     | `String?`             | `null`    | Named browser profile for persistent sessions (cookies, localStorage).                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `saveBrowserProfile` | `Boolean`             | `false`   | Whether to save changes back to the browser profile on exit.                                                                                                                                                                                                                                                                                                                                                                                                                                                        |

##### Methods

###### default()

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

###### validate()

Validate the configuration, returning an error if any values are invalid.

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
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
| `url`                | `String`              | —       | The original URL of the page.                                              |
| `normalizedUrl`      | `String`              | —       | The normalized URL of the page.                                            |
| `statusCode`         | `Short`               | —       | The HTTP status code of the response.                                      |
| `contentType`        | `String`              | —       | The Content-Type header value.                                             |
| `html`               | `String`              | —       | The HTML body of the response.                                             |
| `bodySize`           | `Long`                | —       | The size of the response body in bytes.                                    |
| `metadata`           | `PageMetadata`        | —       | Extracted metadata from the page.                                          |
| `links`              | `List<LinkInfo>`      | `[]`    | Links found on the page.                                                   |
| `images`             | `List<ImageInfo>`     | `[]`    | Images found on the page.                                                  |
| `feeds`              | `List<FeedInfo>`      | `[]`    | Feed links found on the page.                                              |
| `jsonLd`             | `List<JsonLdEntry>`   | `[]`    | JSON-LD entries found on the page.                                         |
| `depth`              | `Long`                | —       | The depth of this page from the start URL.                                 |
| `stayedOnDomain`     | `Boolean`             | —       | Whether this page is on the same domain as the start URL.                  |
| `wasSkipped`         | `Boolean`             | —       | Whether this page was skipped (binary or PDF content).                     |
| `isPdf`              | `Boolean`             | —       | Whether the content is a PDF.                                              |
| `detectedCharset`    | `String?`             | `null`  | The detected character set encoding.                                       |
| `markdown`           | `MarkdownResult?`     | `null`  | Markdown conversion of the page content.                                   |
| `extractedData`      | `Any?`                | `null`  | Structured data extracted by LLM. Populated when extraction is configured. |
| `extractionMeta`     | `ExtractionMeta?`     | `null`  | Metadata about the LLM extraction pass (cost, tokens, model).              |
| `downloadedDocument` | `DownloadedDocument?` | `null`  | Downloaded non-HTML document (PDF, DOCX, image, code, etc.).               |

---

#### CrawlResult

The result of a multi-page crawl operation.

| Field            | Type                    | Default | Description                                                               |
| ---------------- | ----------------------- | ------- | ------------------------------------------------------------------------- |
| `pages`          | `List<CrawlPageResult>` | `[]`    | The list of crawled pages.                                                |
| `finalUrl`       | `String`                | —       | The final URL after following redirects.                                  |
| `redirectCount`  | `Long`                  | —       | The number of redirects followed.                                         |
| `wasSkipped`     | `Boolean`               | —       | Whether any page was skipped during crawling.                             |
| `error`          | `String?`               | `null`  | An error message, if the crawl encountered an issue.                      |
| `cookies`        | `List<CookieInfo>`      | `[]`    | Cookies collected during the crawl.                                       |
| `normalizedUrls` | `List<String>`          | `[]`    | Normalized URLs encountered during crawling (for deduplication counting). |

##### Methods

###### uniqueNormalizedUrls()

Returns the count of unique normalized URLs encountered during crawling.

**Signature:**

```kotlin
// Phase 1: kotlin backend method signature generation
```

---

#### DownloadedAsset

A downloaded asset from a page.

| Field           | Type            | Default               | Description                                                              |
| --------------- | --------------- | --------------------- | ------------------------------------------------------------------------ |
| `url`           | `String`        | —                     | The original URL of the asset.                                           |
| `contentHash`   | `String`        | —                     | The SHA-256 content hash of the asset.                                   |
| `mimeType`      | `String?`       | `null`                | The MIME type from the Content-Type header.                              |
| `size`          | `Long`          | —                     | The size of the asset in bytes.                                          |
| `assetCategory` | `AssetCategory` | `AssetCategory.Image` | The category of the asset.                                               |
| `htmlTag`       | `String?`       | `null`                | The HTML tag that referenced this asset (e.g., "link", "script", "img"). |

---

#### DownloadedDocument

A downloaded non-HTML document (PDF, DOCX, image, code file, etc.).

When the crawler encounters non-HTML content and `download_documents` is
enabled, it downloads the raw bytes and populates this struct instead of
skipping the resource.

| Field         | Type                  | Default | Description                                              |
| ------------- | --------------------- | ------- | -------------------------------------------------------- |
| `url`         | `String`              | —       | The URL the document was fetched from.                   |
| `mimeType`    | `String`              | —       | The MIME type from the Content-Type header.              |
| `content`     | `ByteArray`           | —       | Raw document bytes. Skipped during JSON serialization.   |
| `size`        | `Long`                | —       | Size of the document in bytes.                           |
| `filename`    | `String?`             | `null`  | Filename extracted from Content-Disposition or URL path. |
| `contentHash` | `String`              | —       | SHA-256 hex digest of the content.                       |
| `headers`     | `Map<String, String>` | `{}`    | Selected response headers.                               |

---

#### ExtractionMeta

Metadata about an LLM extraction pass.

| Field              | Type      | Default | Description                                     |
| ------------------ | --------- | ------- | ----------------------------------------------- |
| `cost`             | `Double?` | `null`  | Estimated cost of the LLM call in USD.          |
| `promptTokens`     | `Long?`   | `null`  | Number of prompt (input) tokens consumed.       |
| `completionTokens` | `Long?`   | `null`  | Number of completion (output) tokens generated. |
| `model`            | `String?` | `null`  | The model identifier used for extraction.       |
| `chunksProcessed`  | `Long`    | —       | Number of content chunks sent to the LLM.       |

---

#### FaviconInfo

Information about a favicon or icon link.

| Field      | Type      | Default | Description                                             |
| ---------- | --------- | ------- | ------------------------------------------------------- |
| `url`      | `String`  | —       | The icon URL.                                           |
| `rel`      | `String`  | —       | The `rel` attribute (e.g., "icon", "apple-touch-icon"). |
| `sizes`    | `String?` | `null`  | The `sizes` attribute, if present.                      |
| `mimeType` | `String?` | `null`  | The MIME type, if present.                              |

---

#### FeedInfo

Information about a feed link found on a page.

| Field      | Type       | Default        | Description                 |
| ---------- | ---------- | -------------- | --------------------------- |
| `url`      | `String`   | —              | The feed URL.               |
| `title`    | `String?`  | `null`         | The feed title, if present. |
| `feedType` | `FeedType` | `FeedType.Rss` | The type of feed.           |

---

#### HeadingInfo

A heading element extracted from the page.

| Field   | Type     | Default | Description               |
| ------- | -------- | ------- | ------------------------- |
| `level` | `Byte`   | —       | The heading level (1-6).  |
| `text`  | `String` | —       | The heading text content. |

---

#### HreflangEntry

An hreflang alternate link entry.

| Field  | Type     | Default | Description                                        |
| ------ | -------- | ------- | -------------------------------------------------- |
| `lang` | `String` | —       | The language code (e.g., "en", "fr", "x-default"). |
| `url`  | `String` | —       | The URL for this language variant.                 |

---

#### ImageInfo

Information about an image found on a page.

| Field    | Type          | Default           | Description                                     |
| -------- | ------------- | ----------------- | ----------------------------------------------- |
| `url`    | `String`      | —                 | The image URL.                                  |
| `alt`    | `String?`     | `null`            | The alt text, if present.                       |
| `width`  | `Int?`        | `null`            | The width attribute, if present and parseable.  |
| `height` | `Int?`        | `null`            | The height attribute, if present and parseable. |
| `source` | `ImageSource` | `ImageSource.Img` | The source of the image reference.              |

---

#### JsonLdEntry

A JSON-LD structured data entry found on a page.

| Field        | Type      | Default | Description                                |
| ------------ | --------- | ------- | ------------------------------------------ |
| `schemaType` | `String`  | —       | The `@type` value from the JSON-LD object. |
| `name`       | `String?` | `null`  | The `name` value, if present.              |
| `raw`        | `String`  | —       | The raw JSON-LD string.                    |

---

#### LinkInfo

Information about a link found on a page.

| Field      | Type       | Default             | Description                            |
| ---------- | ---------- | ------------------- | -------------------------------------- |
| `url`      | `String`   | —                   | The resolved URL of the link.          |
| `text`     | `String`   | —                   | The visible text of the link.          |
| `linkType` | `LinkType` | `LinkType.Internal` | The classification of the link.        |
| `rel`      | `String?`  | `null`              | The `rel` attribute value, if present. |
| `nofollow` | `Boolean`  | —                   | Whether the link has `rel="nofollow"`. |

---

#### MapResult

The result of a map operation, containing discovered URLs.

| Field  | Type               | Default | Description                  |
| ------ | ------------------ | ------- | ---------------------------- |
| `urls` | `List<SitemapUrl>` | `[]`    | The list of discovered URLs. |

---

#### MarkdownResult

Rich markdown conversion result from HTML processing.

| Field               | Type              | Default | Description                                              |
| ------------------- | ----------------- | ------- | -------------------------------------------------------- |
| `content`           | `String`          | —       | Converted markdown text.                                 |
| `documentStructure` | `Any?`            | `null`  | Structured document tree with semantic nodes.            |
| `tables`            | `List<Any>`       | `[]`    | Extracted tables with structured cell data.              |
| `warnings`          | `List<String>`    | `[]`    | Non-fatal processing warnings.                           |
| `citations`         | `CitationResult?` | `null`  | Content with links replaced by numbered citations.       |
| `fitContent`        | `String?`         | `null`  | Content-filtered markdown optimized for LLM consumption. |

---

#### PageMetadata

Metadata extracted from an HTML page's `<meta>` tags and `<title>` element.

| Field                | Type                   | Default | Description                                        |
| -------------------- | ---------------------- | ------- | -------------------------------------------------- |
| `title`              | `String?`              | `null`  | The page title from the `<title>` element.         |
| `description`        | `String?`              | `null`  | The meta description.                              |
| `canonicalUrl`       | `String?`              | `null`  | The canonical URL from `<link rel="canonical">`.   |
| `keywords`           | `String?`              | `null`  | Keywords from `<meta name="keywords">`.            |
| `author`             | `String?`              | `null`  | Author from `<meta name="author">`.                |
| `viewport`           | `String?`              | `null`  | Viewport content from `<meta name="viewport">`.    |
| `themeColor`         | `String?`              | `null`  | Theme color from `<meta name="theme-color">`.      |
| `generator`          | `String?`              | `null`  | Generator from `<meta name="generator">`.          |
| `robots`             | `String?`              | `null`  | Robots content from `<meta name="robots">`.        |
| `htmlLang`           | `String?`              | `null`  | The `lang` attribute from the `<html>` element.    |
| `htmlDir`            | `String?`              | `null`  | The `dir` attribute from the `<html>` element.     |
| `ogTitle`            | `String?`              | `null`  | Open Graph title.                                  |
| `ogType`             | `String?`              | `null`  | Open Graph type.                                   |
| `ogImage`            | `String?`              | `null`  | Open Graph image URL.                              |
| `ogDescription`      | `String?`              | `null`  | Open Graph description.                            |
| `ogUrl`              | `String?`              | `null`  | Open Graph URL.                                    |
| `ogSiteName`         | `String?`              | `null`  | Open Graph site name.                              |
| `ogLocale`           | `String?`              | `null`  | Open Graph locale.                                 |
| `ogVideo`            | `String?`              | `null`  | Open Graph video URL.                              |
| `ogAudio`            | `String?`              | `null`  | Open Graph audio URL.                              |
| `ogLocaleAlternates` | `List<String>?`        | `[]`    | Open Graph locale alternates.                      |
| `twitterCard`        | `String?`              | `null`  | Twitter card type.                                 |
| `twitterTitle`       | `String?`              | `null`  | Twitter title.                                     |
| `twitterDescription` | `String?`              | `null`  | Twitter description.                               |
| `twitterImage`       | `String?`              | `null`  | Twitter image URL.                                 |
| `twitterSite`        | `String?`              | `null`  | Twitter site handle.                               |
| `twitterCreator`     | `String?`              | `null`  | Twitter creator handle.                            |
| `dcTitle`            | `String?`              | `null`  | Dublin Core title.                                 |
| `dcCreator`          | `String?`              | `null`  | Dublin Core creator.                               |
| `dcSubject`          | `String?`              | `null`  | Dublin Core subject.                               |
| `dcDescription`      | `String?`              | `null`  | Dublin Core description.                           |
| `dcPublisher`        | `String?`              | `null`  | Dublin Core publisher.                             |
| `dcDate`             | `String?`              | `null`  | Dublin Core date.                                  |
| `dcType`             | `String?`              | `null`  | Dublin Core type.                                  |
| `dcFormat`           | `String?`              | `null`  | Dublin Core format.                                |
| `dcIdentifier`       | `String?`              | `null`  | Dublin Core identifier.                            |
| `dcLanguage`         | `String?`              | `null`  | Dublin Core language.                              |
| `dcRights`           | `String?`              | `null`  | Dublin Core rights.                                |
| `article`            | `ArticleMetadata?`     | `null`  | Article metadata from `article:*` Open Graph tags. |
| `hreflangs`          | `List<HreflangEntry>?` | `[]`    | Hreflang alternate links.                          |
| `favicons`           | `List<FaviconInfo>?`   | `[]`    | Favicon and icon links.                            |
| `headings`           | `List<HeadingInfo>?`   | `[]`    | Heading elements (h1-h6).                          |
| `wordCount`          | `Long?`                | `null`  | Computed word count of the page body text.         |

---

#### ProxyConfig

Proxy configuration for HTTP requests.

| Field      | Type      | Default | Description                                                    |
| ---------- | --------- | ------- | -------------------------------------------------------------- |
| `url`      | `String`  | —       | Proxy URL (e.g. "<http://proxy:8080",> "socks5://proxy:1080"). |
| `username` | `String?` | `null`  | Optional username for proxy authentication.                    |
| `password` | `String?` | `null`  | Optional password for proxy authentication.                    |

---

#### ResponseMeta

Response metadata extracted from HTTP headers.

| Field             | Type      | Default | Description                        |
| ----------------- | --------- | ------- | ---------------------------------- |
| `etag`            | `String?` | `null`  | The ETag header value.             |
| `lastModified`    | `String?` | `null`  | The Last-Modified header value.    |
| `cacheControl`    | `String?` | `null`  | The Cache-Control header value.    |
| `server`          | `String?` | `null`  | The Server header value.           |
| `xPoweredBy`      | `String?` | `null`  | The X-Powered-By header value.     |
| `contentLanguage` | `String?` | `null`  | The Content-Language header value. |
| `contentEncoding` | `String?` | `null`  | The Content-Encoding header value. |

---

#### ScrapeResult

The result of a single-page scrape operation.

| Field                | Type                    | Default | Description                                                                                            |
| -------------------- | ----------------------- | ------- | ------------------------------------------------------------------------------------------------------ |
| `statusCode`         | `Short`                 | —       | The HTTP status code of the response.                                                                  |
| `contentType`        | `String`                | —       | The Content-Type header value.                                                                         |
| `html`               | `String`                | —       | The HTML body of the response.                                                                         |
| `bodySize`           | `Long`                  | —       | The size of the response body in bytes.                                                                |
| `metadata`           | `PageMetadata`          | —       | Extracted metadata from the page.                                                                      |
| `links`              | `List<LinkInfo>`        | `[]`    | Links found on the page.                                                                               |
| `images`             | `List<ImageInfo>`       | `[]`    | Images found on the page.                                                                              |
| `feeds`              | `List<FeedInfo>`        | `[]`    | Feed links found on the page.                                                                          |
| `jsonLd`             | `List<JsonLdEntry>`     | `[]`    | JSON-LD entries found on the page.                                                                     |
| `isAllowed`          | `Boolean`               | —       | Whether the URL is allowed by robots.txt.                                                              |
| `crawlDelay`         | `Long?`                 | `null`  | The crawl delay from robots.txt, in seconds.                                                           |
| `noindexDetected`    | `Boolean`               | —       | Whether a noindex directive was detected.                                                              |
| `nofollowDetected`   | `Boolean`               | —       | Whether a nofollow directive was detected.                                                             |
| `xRobotsTag`         | `String?`               | `null`  | The X-Robots-Tag header value, if present.                                                             |
| `isPdf`              | `Boolean`               | —       | Whether the content is a PDF.                                                                          |
| `wasSkipped`         | `Boolean`               | —       | Whether the page was skipped (binary or PDF content).                                                  |
| `detectedCharset`    | `String?`               | `null`  | The detected character set encoding.                                                                   |
| `authHeaderSent`     | `Boolean`               | —       | Whether an authentication header was sent with the request.                                            |
| `responseMeta`       | `ResponseMeta?`         | `null`  | Response metadata extracted from HTTP headers.                                                         |
| `assets`             | `List<DownloadedAsset>` | `[]`    | Downloaded assets from the page.                                                                       |
| `jsRenderHint`       | `Boolean`               | —       | Whether the page content suggests JavaScript rendering is needed.                                      |
| `browserUsed`        | `Boolean`               | —       | Whether the browser fallback was used to fetch this page.                                              |
| `markdown`           | `MarkdownResult?`       | `null`  | Markdown conversion of the page content.                                                               |
| `extractedData`      | `Any?`                  | `null`  | Structured data extracted by LLM. Populated when extraction is configured.                             |
| `extractionMeta`     | `ExtractionMeta?`       | `null`  | Metadata about the LLM extraction pass (cost, tokens, model).                                          |
| `screenshot`         | `ByteArray?`            | `null`  | Screenshot of the page as PNG bytes. Populated when browser is used and capture_screenshot is enabled. |
| `downloadedDocument` | `DownloadedDocument?`   | `null`  | Downloaded non-HTML document (PDF, DOCX, image, code, etc.).                                           |

---

#### SitemapUrl

A URL entry from a sitemap.

| Field        | Type      | Default | Description                             |
| ------------ | --------- | ------- | --------------------------------------- |
| `url`        | `String`  | —       | The URL.                                |
| `lastmod`    | `String?` | `null`  | The last modification date, if present. |
| `changefreq` | `String?` | `null`  | The change frequency, if present.       |
| `priority`   | `String?` | `null`  | The priority, if present.               |

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

| Value    | Description                                                                     |
| -------- | ------------------------------------------------------------------------------- |
| `Basic`  | HTTP Basic authentication. — Fields: `username`: `String`, `password`: `String` |
| `Bearer` | Bearer token authentication. — Fields: `token`: `String`                        |
| `Header` | Custom authentication header. — Fields: `name`: `String`, `value`: `String`     |

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
