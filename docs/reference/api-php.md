---
title: "PHP API Reference"
---

## PHP API Reference <span class="version-badge">v0.3.0-rc.21</span>

### Functions

#### generateCitations()

Convert markdown links to numbered citations.

`[Example](https://example.com)` becomes `Example[1]`
with `[1]: <https://example.com`> in the reference list.
Images `![alt](url)` are preserved unchanged.

**Signature:**

```php
public static function generateCitations(string $markdown): CitationResult
```

**Parameters:**

| Name       | Type     | Required | Description  |
| ---------- | -------- | -------- | ------------ |
| `markdown` | `string` | Yes      | The markdown |

**Returns:** `CitationResult`

---

#### createEngine()

Create a new crawl engine with the given configuration.

If `config` is `null`, uses `CrawlConfig::default()`.
Returns an error if the configuration is invalid.

**Signature:**

```php
public static function createEngine(?CrawlConfig $config = null): CrawlEngineHandle
```

**Parameters:**

| Name     | Type           | Required | Description               |
| -------- | -------------- | -------- | ------------------------- |
| `config` | `?CrawlConfig` | No       | The configuration options |

**Returns:** `CrawlEngineHandle`
**Errors:** Throws `CrawlError`.

---

#### scrape()

Scrape a single URL, returning extracted page data.

**Signature:**

```php
public static function scrape(CrawlEngineHandle $engine, string $url): ScrapeResult
```

**Parameters:**

| Name     | Type                | Required | Description             |
| -------- | ------------------- | -------- | ----------------------- |
| `engine` | `CrawlEngineHandle` | Yes      | The crawl engine handle |
| `url`    | `string`            | Yes      | The URL to fetch        |

**Returns:** `ScrapeResult`
**Errors:** Throws `CrawlError`.

---

#### crawl()

Crawl a website starting from `url`, following links up to the configured depth.

**Signature:**

```php
public static function crawl(CrawlEngineHandle $engine, string $url): CrawlResult
```

**Parameters:**

| Name     | Type                | Required | Description             |
| -------- | ------------------- | -------- | ----------------------- |
| `engine` | `CrawlEngineHandle` | Yes      | The crawl engine handle |
| `url`    | `string`            | Yes      | The URL to fetch        |

**Returns:** `CrawlResult`
**Errors:** Throws `CrawlError`.

---

#### mapUrls()

Discover all pages on a website by following links and sitemaps.

**Signature:**

```php
public static function mapUrls(CrawlEngineHandle $engine, string $url): MapResult
```

**Parameters:**

| Name     | Type                | Required | Description             |
| -------- | ------------------- | -------- | ----------------------- |
| `engine` | `CrawlEngineHandle` | Yes      | The crawl engine handle |
| `url`    | `string`            | Yes      | The URL to fetch        |

**Returns:** `MapResult`
**Errors:** Throws `CrawlError`.

---

#### interact()

Execute browser actions on a single page.

**Signature:**

```php
public static function interact(CrawlEngineHandle $engine, string $url, array<PageAction> $actions): InteractionResult
```

**Parameters:**

| Name      | Type                | Required | Description             |
| --------- | ------------------- | -------- | ----------------------- |
| `engine`  | `CrawlEngineHandle` | Yes      | The crawl engine handle |
| `url`     | `string`            | Yes      | The URL to fetch        |
| `actions` | `array<PageAction>` | Yes      | The actions             |

**Returns:** `InteractionResult`
**Errors:** Throws `CrawlError`.

---

#### batchScrape()

Scrape multiple URLs concurrently.

**Signature:**

```php
public static function batchScrape(CrawlEngineHandle $engine, array<string> $urls): BatchScrapeResults
```

**Parameters:**

| Name     | Type                | Required | Description             |
| -------- | ------------------- | -------- | ----------------------- |
| `engine` | `CrawlEngineHandle` | Yes      | The crawl engine handle |
| `urls`   | `array<string>`     | Yes      | The urls                |

**Returns:** `BatchScrapeResults`
**Errors:** Throws `CrawlError`.

---

#### batchCrawl()

Crawl multiple seed URLs concurrently, each following links to configured depth.

**Signature:**

```php
public static function batchCrawl(CrawlEngineHandle $engine, array<string> $urls): BatchCrawlResults
```

**Parameters:**

| Name     | Type                | Required | Description             |
| -------- | ------------------- | -------- | ----------------------- |
| `engine` | `CrawlEngineHandle` | Yes      | The crawl engine handle |
| `urls`   | `array<string>`     | Yes      | The urls                |

**Returns:** `BatchCrawlResults`
**Errors:** Throws `CrawlError`.

---

### Types

#### ActionResult

Result from a single page action execution.

| Field         | Type      | Default | Description                                                                    |
| ------------- | --------- | ------- | ------------------------------------------------------------------------------ |
| `actionIndex` | `int`     | —       | Zero-based index of the action in the sequence.                                |
| `actionType`  | `string`  | —       | The type of action that was executed.                                          |
| `success`     | `bool`    | —       | Whether the action completed successfully.                                     |
| `data`        | `?mixed`  | `null`  | Action-specific return data (screenshot bytes, JS return value, scraped HTML). |
| `error`       | `?string` | `null`  | Error message if the action failed.                                            |

---

#### ArticleMetadata

Article metadata extracted from `article:*` Open Graph tags.

| Field           | Type            | Default | Description                    |
| --------------- | --------------- | ------- | ------------------------------ |
| `publishedTime` | `?string`       | `null`  | The article publication time.  |
| `modifiedTime`  | `?string`       | `null`  | The article modification time. |
| `author`        | `?string`       | `null`  | The article author.            |
| `section`       | `?string`       | `null`  | The article section.           |
| `tags`          | `array<string>` | `[]`    | The article tags.              |

---

#### BatchCrawlResult

Result from a single URL in a batch crawl operation.

| Field    | Type           | Default | Description                             |
| -------- | -------------- | ------- | --------------------------------------- |
| `url`    | `string`       | —       | The seed URL that was crawled.          |
| `result` | `?CrawlResult` | `null`  | The crawl result, if successful.        |
| `error`  | `?string`      | `null`  | The error message, if the crawl failed. |

---

#### BatchCrawlResults

Aggregate result of a batch crawl, exposing per-URL results plus precomputed counts.

The counts are derived once at construction so every binding language can read them
as plain integer fields without re-iterating the `results` vector.

| Field            | Type                      | Default | Description                                                        |
| ---------------- | ------------------------- | ------- | ------------------------------------------------------------------ |
| `results`        | `array<BatchCrawlResult>` | `[]`    | Per-URL crawl results, in the order seed URLs were submitted.      |
| `totalCount`     | `int`                     | —       | Total number of seed URLs in the batch (equal to `results.len()`). |
| `completedCount` | `int`                     | —       | Number of seed URLs whose crawl succeeded (`error` is `null`).     |
| `failedCount`    | `int`                     | —       | Number of seed URLs whose crawl failed (`error` is `Some`).        |

---

#### BatchCrawlStreamRequest

Request to begin a multi-URL streaming crawl.

Wraps a set of seed URLs for delivery through the streaming-adapter binding
surface. Required as a struct because alef's streaming adapter requires a
named request type — primitives are not supported.

| Field  | Type            | Default | Description                                                                                     |
| ------ | --------------- | ------- | ----------------------------------------------------------------------------------------------- |
| `urls` | `array<string>` | `[]`    | The seed URLs to crawl. Each URL is followed independently up to the engine's configured depth. |

---

#### BatchScrapeResult

Result from a single URL in a batch scrape operation.

| Field    | Type            | Default | Description                              |
| -------- | --------------- | ------- | ---------------------------------------- |
| `url`    | `string`        | —       | The URL that was scraped.                |
| `result` | `?ScrapeResult` | `null`  | The scrape result, if successful.        |
| `error`  | `?string`       | `null`  | The error message, if the scrape failed. |

---

#### BatchScrapeResults

Aggregate result of a batch scrape, exposing per-URL results plus precomputed counts.

The counts are derived once at construction so every binding language can read them
as plain integer fields without re-iterating the `results` vector.

| Field            | Type                       | Default | Description                                                   |
| ---------------- | -------------------------- | ------- | ------------------------------------------------------------- |
| `results`        | `array<BatchScrapeResult>` | `[]`    | Per-URL scrape results, in the order URLs were submitted.     |
| `totalCount`     | `int`                      | —       | Total number of URLs in the batch (equal to `results.len()`). |
| `completedCount` | `int`                      | —       | Number of URLs whose scrape succeeded (`error` is `null`).    |
| `failedCount`    | `int`                      | —       | Number of URLs whose scrape failed (`error` is `Some`).       |

---

#### BrowserConfig

Browser fallback configuration.

| Field                  | Type             | Default                         | Description                                                                                                                                                                                                                                                                        |
| ---------------------- | ---------------- | ------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mode`                 | `BrowserMode`    | `BrowserMode::Auto`             | When to use the headless browser fallback.                                                                                                                                                                                                                                         |
| `backend`              | `BrowserBackend` | `BrowserBackend::Chromiumoxide` | Browser backend used to render JavaScript-heavy pages.                                                                                                                                                                                                                             |
| `endpoint`             | `?string`        | `null`                          | CDP WebSocket endpoint for connecting to an external browser instance.                                                                                                                                                                                                             |
| `timeout`              | `float`          | `30000ms`                       | Timeout for browser page load and rendering (in milliseconds when serialized).                                                                                                                                                                                                     |
| `wait`                 | `BrowserWait`    | `BrowserWait::NetworkIdle`      | Wait strategy after browser navigation.                                                                                                                                                                                                                                            |
| `waitSelector`         | `?string`        | `null`                          | CSS selector to wait for when `wait` is `Selector`.                                                                                                                                                                                                                                |
| `extraWait`            | `?float`         | `null`                          | Extra time to wait after the wait condition is met.                                                                                                                                                                                                                                |
| `stealth`              | `bool`           | `false`                         | Enable browser-realistic TLS fingerprint via the stealth HTTP client. Only honored by `BrowserBackend::Native` — chromiumoxide is already full-stealth via Chrome's TLS stack.                                                                                                     |
| `proxy`                | `?ProxyConfig`   | `null`                          | Proxy for browser fetches. Overrides `CrawlConfig.proxy` when set. Native backend supports http/https only (no SOCKS5).                                                                                                                                                            |
| `blockUrlPatterns`     | `array<string>`  | `[]`                            | URL patterns to block before the network request fires. Supports `*` wildcards. Useful for skipping ads/analytics/large images. Honored by `BrowserBackend::Native`; chromiumoxide ignores this field today.                                                                       |
| `evalScript`           | `?string`        | `null`                          | JavaScript snippet evaluated after navigation completes. Scraping captures the native backend result in `ScrapeResult.browser.eval_result`. Interactions run this script before page actions on both browser backends but do not include the script result in `InteractionResult`. |
| `robotsUserAgent`      | `?string`        | `null`                          | User-agent used when fetching robots.txt. Defaults to `BrowserConfig.user_agent` (or kreuzcrawl's default) if unset. Native only.                                                                                                                                                  |
| `captureNetworkEvents` | `bool`           | `false`                         | Capture the full network event stream into the result. Default false (only the document event is captured). Native only.                                                                                                                                                           |

### Methods

#### default()

**Signature:**

```php
public static function default(): BrowserConfig
```

---

#### BrowserExtras

Browser-specific extras populated when the native browser backend was used.

Available on `ScrapeResult.browser` when `BrowserBackend::Native` handled the request.

| Field           | Type                  | Default | Description                                                                                                                                 |
| --------------- | --------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| `evalResult`    | `?mixed`              | `null`  | Return value of `BrowserConfig.eval_script`, if provided.                                                                                   |
| `networkEvents` | `array<ResponseMeta>` | `[]`    | Network events captured during page navigation (only populated when `BrowserConfig.capture_network_events` is true).                        |
| `cookies`       | `array<CookieInfo>`   | `[]`    | All non-expired cookies present in the browser's cookie jar after navigation completes (includes both prior cookies and server Set-Cookie). |

---

#### CitationReference

A single numbered reference in a citation list — produced by the citation
extractor when content uses inline `[N]`-style markers.

| Field   | Type     | Default | Description                                                |
| ------- | -------- | ------- | ---------------------------------------------------------- |
| `index` | `int`    | —       | 1-based reference number as it appears in the source text. |
| `url`   | `string` | —       | Resolved absolute URL for this reference.                  |
| `text`  | `string` | —       | Human-readable anchor text or title for the reference.     |

---

#### CitationResult

Result of citation conversion.

| Field        | Type                       | Default | Description                                         |
| ------------ | -------------------------- | ------- | --------------------------------------------------- |
| `content`    | `string`                   | —       | Markdown with links replaced by numbered citations. |
| `references` | `array<CitationReference>` | `[]`    | Numbered reference list: (index, url, text).        |

---

#### ContentConfig

Content extraction and conversion configuration.

Controls how HTML is converted to the output format. Uses
html-to-markdown-rs as the conversion engine for all formats
(markdown, plain text, djot).

| Field                      | Type            | Default      | Description                                                                                                                                                                                                                                                                                                                                         |
| -------------------------- | --------------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `outputFormat`             | `string`        | `"markdown"` | Output format: `"markdown"` (default), `"plain"`, `"djot"`.                                                                                                                                                                                                                                                                                         |
| `preprocessingPreset`      | `string`        | `"standard"` | Preprocessing aggressiveness: `"minimal"`, `"standard"` (default), `"aggressive"`. - Minimal: only scripts/styles removed. - Standard: also removes nav, nav-hinted headers/footers/asides, forms. - Aggressive: removes all footers/asides unconditionally.                                                                                        |
| `removeNavigation`         | `bool`          | `true`       | Remove navigation elements (nav, breadcrumbs, menus). Default: `true`.                                                                                                                                                                                                                                                                              |
| `removeForms`              | `bool`          | `true`       | Remove form elements. Default: `true`.                                                                                                                                                                                                                                                                                                              |
| `stripTags`                | `array<string>` | `[]`         | HTML tag names to strip (render children only, remove the tag wrapper). Default: `["noscript"]`.                                                                                                                                                                                                                                                    |
| `preserveTags`             | `array<string>` | `[]`         | HTML tag names to preserve as raw HTML in output.                                                                                                                                                                                                                                                                                                   |
| `excludeSelectors`         | `array<string>` | `[]`         | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the wrapper but keeps children), excluded elements and all descendants are dropped. Supports CSS selectors: `.class`, `#id`, `[attribute]`, compound selectors. Example: `[".cookie-banner", "#ad-container", "[role='complementary']"]` |
| `skipImages`               | `bool`          | `false`      | Skip image elements in output. Default: `false`.                                                                                                                                                                                                                                                                                                    |
| `maxDepth`                 | `?int`          | `null`       | Max DOM traversal depth. Prevents stack overflow on deeply nested HTML.                                                                                                                                                                                                                                                                             |
| `wrap`                     | `bool`          | `false`      | Enable line wrapping. Default: `false`.                                                                                                                                                                                                                                                                                                             |
| `wrapWidth`                | `int`           | `80`         | Wrap width when `wrap` is enabled. Default: `80`.                                                                                                                                                                                                                                                                                                   |
| `includeDocumentStructure` | `bool`          | `true`       | Include document structure tree in output. Default: `true`.                                                                                                                                                                                                                                                                                         |

### Methods

#### default()

**Signature:**

```php
public static function default(): ContentConfig
```

---

#### CookieInfo

Information about an HTTP cookie received from a response.

| Field    | Type      | Default | Description                      |
| -------- | --------- | ------- | -------------------------------- |
| `name`   | `string`  | —       | The cookie name.                 |
| `value`  | `string`  | —       | The cookie value.                |
| `domain` | `?string` | `null`  | The cookie domain, if specified. |
| `path`   | `?string` | `null`  | The cookie path, if specified.   |

---

#### CrawlConfig

Configuration for crawl, scrape, and map operations.

| Field                | Type                    | Default   | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| -------------------- | ----------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `maxDepth`           | `?int`                  | `null`    | Maximum crawl depth (number of link hops from the start URL).                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `maxPages`           | `?int`                  | `null`    | Maximum number of pages to crawl.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `maxConcurrent`      | `?int`                  | `null`    | Maximum number of concurrent requests.                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `respectRobotsTxt`   | `bool`                  | `false`   | Whether to respect robots.txt directives.                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `softHttpErrors`     | `bool`                  | `false`   | When true, HTTP-level error responses (404 NotFound, 403 Forbidden, WAF blocks) are surfaced as `ScrapeResult` records with the matching `status_code` rather than raised as `CrawlError`. Default `false` preserves the historical throw-on-error contract for direct fetches. Independently of this flag, 404s reached at the end of a redirect chain are _always_ surfaced softly — the user opted into redirect-following, so receiving a 404 there is part of the normal flow rather than an unexpected error. |
| `userAgent`          | `?string`               | `null`    | Custom user-agent string.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `stayOnDomain`       | `bool`                  | `false`   | Whether to restrict crawling to the same domain.                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `allowSubdomains`    | `bool`                  | `false`   | Whether to allow subdomains when `stay_on_domain` is true.                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `includePaths`       | `array<string>`         | `[]`      | Regex patterns for paths to include during crawling.                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `excludePaths`       | `array<string>`         | `[]`      | Regex patterns for paths to exclude during crawling.                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `customHeaders`      | `array<string, string>` | `{}`      | Custom HTTP headers to send with each request.                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `requestTimeout`     | `float`                 | `30000ms` | Timeout for individual HTTP requests (in milliseconds when serialized).                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `rateLimitMs`        | `?int`                  | `null`    | Per-domain rate limit in milliseconds. When set, enforces a minimum delay between requests to the same domain. Defaults to 200ms when `null`.                                                                                                                                                                                                                                                                                                                                                                       |
| `maxRedirects`       | `int`                   | `10`      | Maximum number of redirects to follow.                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `retryCount`         | `int`                   | `0`       | Number of retry attempts for failed requests.                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `retryCodes`         | `array<int>`            | `[]`      | HTTP status codes that should trigger a retry.                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `cookiesEnabled`     | `bool`                  | `false`   | Whether to enable cookie handling.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `auth`               | `?AuthConfig`           | `null`    | Authentication configuration.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `maxBodySize`        | `?int`                  | `null`    | Maximum response body size in bytes.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `removeTags`         | `array<string>`         | `[]`      | CSS selectors for tags to remove from HTML before processing.                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `content`            | `ContentConfig`         | —         | Content extraction and conversion configuration.                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `mapLimit`           | `?int`                  | `null`    | Maximum number of URLs to return from a map operation.                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `mapSearch`          | `?string`               | `null`    | Search filter for map results (case-insensitive substring match on URLs).                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `downloadAssets`     | `bool`                  | `false`   | Whether to download assets (CSS, JS, images, etc.) from the page.                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `assetTypes`         | `array<AssetCategory>`  | `[]`      | Filter for asset categories to download.                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `maxAssetSize`       | `?int`                  | `null`    | Maximum size in bytes for individual asset downloads.                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `browser`            | `BrowserConfig`         | —         | Browser configuration.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `proxy`              | `?ProxyConfig`          | `null`    | Proxy configuration for HTTP requests.                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `userAgents`         | `array<string>`         | `[]`      | List of user-agent strings for rotation. If non-empty, overrides `user_agent`.                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `captureScreenshot`  | `bool`                  | `false`   | Whether to capture a screenshot when using the browser.                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `downloadDocuments`  | `bool`                  | `true`    | Whether to download non-HTML documents (PDF, DOCX, images, code, etc.) instead of skipping them.                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `documentMaxSize`    | `?int`                  | `null`    | Maximum size in bytes for document downloads. Defaults to 50 MB.                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `documentMimeTypes`  | `array<string>`         | `[]`      | Allowlist of MIME types to download. If empty, uses built-in defaults.                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `warcOutput`         | `?string`               | `null`    | Path to write WARC output. If `null`, WARC output is disabled.                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `browserProfile`     | `?string`               | `null`    | Named browser profile for persistent sessions (cookies, localStorage).                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `saveBrowserProfile` | `bool`                  | `false`   | Whether to save changes back to the browser profile on exit.                                                                                                                                                                                                                                                                                                                                                                                                                                                        |

### Methods

#### default()

**Signature:**

```php
public static function default(): CrawlConfig
```

#### validate()

Validate the configuration, returning an error if any values are invalid.

**Signature:**

```php
public function validate(): void
```

---

#### CrawlEngineHandle

Opaque handle to a configured crawl engine.

Constructed via `create_engine` with an optional `CrawlConfig`.
Default implementations for all pluggable components are used internally.

### Methods

#### crawlStream()

Stream a single-URL crawl, yielding `CrawlEvent`s as pages are processed.

Returns an async stream that emits one event per crawled page, plus a
terminal `Complete` event. On per-URL failure during the crawl, emits an
`Error` event followed by `Complete`. The stream item type is wrapped in
a `Result` to surface transport-level errors; today every emit is `Ok`.

**Signature:**

```php
public function crawlStream(CrawlStreamRequest $req): string
```

#### batchCrawlStream()

Stream a multi-URL crawl, yielding `CrawlEvent`s across all seeds.

Returns an async stream that emits one event per crawled page across all
seeds, plus terminal `Complete` and `Error` events as appropriate. The
stream item type is wrapped in a `Result` to surface transport-level
errors; today every emit is `Ok`.

**Signature:**

```php
public function batchCrawlStream(BatchCrawlStreamRequest $req): string
```

---

#### CrawlPageResult

The result of crawling a single page during a crawl operation.

| Field                | Type                  | Default | Description                                                                |
| -------------------- | --------------------- | ------- | -------------------------------------------------------------------------- |
| `url`                | `string`              | —       | The original URL of the page.                                              |
| `normalizedUrl`      | `string`              | —       | The normalized URL of the page.                                            |
| `statusCode`         | `int`                 | —       | The HTTP status code of the response.                                      |
| `contentType`        | `string`              | —       | The Content-Type header value.                                             |
| `html`               | `string`              | —       | The HTML body of the response.                                             |
| `bodySize`           | `int`                 | —       | The size of the response body in bytes.                                    |
| `metadata`           | `PageMetadata`        | —       | Extracted metadata from the page.                                          |
| `links`              | `array<LinkInfo>`     | `[]`    | Links found on the page.                                                   |
| `images`             | `array<ImageInfo>`    | `[]`    | Images found on the page.                                                  |
| `feeds`              | `array<FeedInfo>`     | `[]`    | Feed links found on the page.                                              |
| `jsonLd`             | `array<JsonLdEntry>`  | `[]`    | JSON-LD entries found on the page.                                         |
| `depth`              | `int`                 | —       | The depth of this page from the start URL.                                 |
| `stayedOnDomain`     | `bool`                | —       | Whether this page is on the same domain as the start URL.                  |
| `wasSkipped`         | `bool`                | —       | Whether this page was skipped (binary or PDF content).                     |
| `isPdf`              | `bool`                | —       | Whether the content is a PDF.                                              |
| `detectedCharset`    | `?string`             | `null`  | The detected character set encoding.                                       |
| `markdown`           | `?MarkdownResult`     | `null`  | Markdown conversion of the page content.                                   |
| `extractedData`      | `?mixed`              | `null`  | Structured data extracted by LLM. Populated when extraction is configured. |
| `extractionMeta`     | `?ExtractionMeta`     | `null`  | Metadata about the LLM extraction pass (cost, tokens, model).              |
| `downloadedDocument` | `?DownloadedDocument` | `null`  | Downloaded non-HTML document (PDF, DOCX, image, code, etc.).               |

---

#### CrawlResult

The result of a multi-page crawl operation.

| Field            | Type                     | Default | Description                                                               |
| ---------------- | ------------------------ | ------- | ------------------------------------------------------------------------- |
| `pages`          | `array<CrawlPageResult>` | `[]`    | The list of crawled pages.                                                |
| `finalUrl`       | `string`                 | —       | The final URL after following redirects.                                  |
| `redirectCount`  | `int`                    | —       | The number of redirects followed.                                         |
| `wasSkipped`     | `bool`                   | —       | Whether any page was skipped during crawling.                             |
| `error`          | `?string`                | `null`  | An error message, if the crawl encountered an issue.                      |
| `cookies`        | `array<CookieInfo>`      | `[]`    | Cookies collected during the crawl.                                       |
| `normalizedUrls` | `array<string>`          | `[]`    | Normalized URLs encountered during crawling (for deduplication counting). |

### Methods

#### uniqueNormalizedUrls()

Returns the count of unique normalized URLs encountered during crawling.

**Signature:**

```php
public function uniqueNormalizedUrls(): int
```

---

#### CrawlStreamRequest

Request to begin a single-URL streaming crawl.

Wraps a single seed URL for delivery through the streaming-adapter binding
surface. Required as a struct because alef's streaming adapter requires a
named request type — primitives are not supported.

| Field | Type     | Default | Description            |
| ----- | -------- | ------- | ---------------------- |
| `url` | `string` | —       | The seed URL to crawl. |

---

#### DownloadedAsset

A downloaded asset from a page.

| Field           | Type            | Default                | Description                                                              |
| --------------- | --------------- | ---------------------- | ------------------------------------------------------------------------ |
| `url`           | `string`        | —                      | The original URL of the asset.                                           |
| `contentHash`   | `string`        | —                      | The SHA-256 content hash of the asset.                                   |
| `mimeType`      | `?string`       | `null`                 | The MIME type from the Content-Type header.                              |
| `size`          | `int`           | —                      | The size of the asset in bytes.                                          |
| `assetCategory` | `AssetCategory` | `AssetCategory::Image` | The category of the asset.                                               |
| `htmlTag`       | `?string`       | `null`                 | The HTML tag that referenced this asset (e.g., "link", "script", "img"). |

---

#### DownloadedDocument

A downloaded non-HTML document (PDF, DOCX, image, code file, etc.).

When the crawler encounters non-HTML content and `download_documents` is
enabled, it downloads the raw bytes and populates this struct instead of
skipping the resource.

| Field         | Type                    | Default | Description                                              |
| ------------- | ----------------------- | ------- | -------------------------------------------------------- |
| `url`         | `string`                | —       | The URL the document was fetched from.                   |
| `mimeType`    | `string`                | —       | The MIME type from the Content-Type header.              |
| `content`     | `string`                | —       | Raw document bytes. Skipped during JSON serialization.   |
| `size`        | `int`                   | —       | Size of the document in bytes.                           |
| `filename`    | `?string`               | `null`  | Filename extracted from Content-Disposition or URL path. |
| `contentHash` | `string`                | —       | SHA-256 hex digest of the content.                       |
| `headers`     | `array<string, string>` | `{}`    | Selected response headers.                               |

---

#### ExtractionMeta

Metadata about an LLM extraction pass.

| Field              | Type      | Default | Description                                     |
| ------------------ | --------- | ------- | ----------------------------------------------- |
| `cost`             | `?float`  | `null`  | Estimated cost of the LLM call in USD.          |
| `promptTokens`     | `?int`    | `null`  | Number of prompt (input) tokens consumed.       |
| `completionTokens` | `?int`    | `null`  | Number of completion (output) tokens generated. |
| `model`            | `?string` | `null`  | The model identifier used for extraction.       |
| `chunksProcessed`  | `int`     | —       | Number of content chunks sent to the LLM.       |

---

#### FaviconInfo

Information about a favicon or icon link.

| Field      | Type      | Default | Description                                             |
| ---------- | --------- | ------- | ------------------------------------------------------- |
| `url`      | `string`  | —       | The icon URL.                                           |
| `rel`      | `string`  | —       | The `rel` attribute (e.g., "icon", "apple-touch-icon"). |
| `sizes`    | `?string` | `null`  | The `sizes` attribute, if present.                      |
| `mimeType` | `?string` | `null`  | The MIME type, if present.                              |

---

#### FeedInfo

Information about a feed link found on a page.

| Field      | Type       | Default         | Description                 |
| ---------- | ---------- | --------------- | --------------------------- |
| `url`      | `string`   | —               | The feed URL.               |
| `title`    | `?string`  | `null`          | The feed title, if present. |
| `feedType` | `FeedType` | `FeedType::Rss` | The type of feed.           |

---

#### HeadingInfo

A heading element extracted from the page.

| Field   | Type     | Default | Description               |
| ------- | -------- | ------- | ------------------------- |
| `level` | `int`    | —       | The heading level (1-6).  |
| `text`  | `string` | —       | The heading text content. |

---

#### HreflangEntry

An hreflang alternate link entry.

| Field  | Type     | Default | Description                                        |
| ------ | -------- | ------- | -------------------------------------------------- |
| `lang` | `string` | —       | The language code (e.g., "en", "fr", "x-default"). |
| `url`  | `string` | —       | The URL for this language variant.                 |

---

#### ImageInfo

Information about an image found on a page.

| Field    | Type          | Default            | Description                                     |
| -------- | ------------- | ------------------ | ----------------------------------------------- |
| `url`    | `string`      | —                  | The image URL.                                  |
| `alt`    | `?string`     | `null`             | The alt text, if present.                       |
| `width`  | `?int`        | `null`             | The width attribute, if present and parseable.  |
| `height` | `?int`        | `null`             | The height attribute, if present and parseable. |
| `source` | `ImageSource` | `ImageSource::Img` | The source of the image reference.              |

---

#### InteractionResult

Result of executing a sequence of page interaction actions.

| Field           | Type                  | Default | Description                                          |
| --------------- | --------------------- | ------- | ---------------------------------------------------- |
| `actionResults` | `array<ActionResult>` | `[]`    | Results from each executed action.                   |
| `finalHtml`     | `string`              | —       | Final page HTML after all actions completed.         |
| `finalUrl`      | `string`              | —       | Final page URL (may have changed due to navigation). |
| `screenshot`    | `?string`             | `null`  | Screenshot taken after all actions, if requested.    |

---

#### JsonLdEntry

A JSON-LD structured data entry found on a page.

| Field        | Type      | Default | Description                                |
| ------------ | --------- | ------- | ------------------------------------------ |
| `schemaType` | `string`  | —       | The `@type` value from the JSON-LD object. |
| `name`       | `?string` | `null`  | The `name` value, if present.              |
| `raw`        | `string`  | —       | The raw JSON-LD string.                    |

---

#### LinkInfo

Information about a link found on a page.

| Field      | Type       | Default              | Description                            |
| ---------- | ---------- | -------------------- | -------------------------------------- |
| `url`      | `string`   | —                    | The resolved URL of the link.          |
| `text`     | `string`   | —                    | The visible text of the link.          |
| `linkType` | `LinkType` | `LinkType::Internal` | The classification of the link.        |
| `rel`      | `?string`  | `null`               | The `rel` attribute value, if present. |
| `nofollow` | `bool`     | —                    | Whether the link has `rel="nofollow"`. |

---

#### MapResult

The result of a map operation, containing discovered URLs.

| Field  | Type                | Default | Description                  |
| ------ | ------------------- | ------- | ---------------------------- |
| `urls` | `array<SitemapUrl>` | `[]`    | The list of discovered URLs. |

---

#### MarkdownResult

Rich markdown conversion result from HTML processing.

| Field               | Type              | Default | Description                                              |
| ------------------- | ----------------- | ------- | -------------------------------------------------------- |
| `content`           | `string`          | —       | Converted markdown text.                                 |
| `documentStructure` | `?mixed`          | `null`  | Structured document tree with semantic nodes.            |
| `tables`            | `array<mixed>`    | `[]`    | Extracted tables with structured cell data.              |
| `warnings`          | `array<string>`   | `[]`    | Non-fatal processing warnings.                           |
| `citations`         | `?CitationResult` | `null`  | Content with links replaced by numbered citations.       |
| `fitContent`        | `?string`         | `null`  | Content-filtered markdown optimized for LLM consumption. |

---

#### PageMetadata

Metadata extracted from an HTML page's `<meta>` tags and `<title>` element.

| Field                | Type                    | Default | Description                                        |
| -------------------- | ----------------------- | ------- | -------------------------------------------------- |
| `title`              | `?string`               | `null`  | The page title from the `<title>` element.         |
| `description`        | `?string`               | `null`  | The meta description.                              |
| `canonicalUrl`       | `?string`               | `null`  | The canonical URL from `<link rel="canonical">`.   |
| `keywords`           | `?string`               | `null`  | Keywords from `<meta name="keywords">`.            |
| `author`             | `?string`               | `null`  | Author from `<meta name="author">`.                |
| `viewport`           | `?string`               | `null`  | Viewport content from `<meta name="viewport">`.    |
| `themeColor`         | `?string`               | `null`  | Theme color from `<meta name="theme-color">`.      |
| `generator`          | `?string`               | `null`  | Generator from `<meta name="generator">`.          |
| `robots`             | `?string`               | `null`  | Robots content from `<meta name="robots">`.        |
| `htmlLang`           | `?string`               | `null`  | The `lang` attribute from the `<html>` element.    |
| `htmlDir`            | `?string`               | `null`  | The `dir` attribute from the `<html>` element.     |
| `ogTitle`            | `?string`               | `null`  | Open Graph title.                                  |
| `ogType`             | `?string`               | `null`  | Open Graph type.                                   |
| `ogImage`            | `?string`               | `null`  | Open Graph image URL.                              |
| `ogDescription`      | `?string`               | `null`  | Open Graph description.                            |
| `ogUrl`              | `?string`               | `null`  | Open Graph URL.                                    |
| `ogSiteName`         | `?string`               | `null`  | Open Graph site name.                              |
| `ogLocale`           | `?string`               | `null`  | Open Graph locale.                                 |
| `ogVideo`            | `?string`               | `null`  | Open Graph video URL.                              |
| `ogAudio`            | `?string`               | `null`  | Open Graph audio URL.                              |
| `ogLocaleAlternates` | `?array<string>`        | `[]`    | Open Graph locale alternates.                      |
| `twitterCard`        | `?string`               | `null`  | Twitter card type.                                 |
| `twitterTitle`       | `?string`               | `null`  | Twitter title.                                     |
| `twitterDescription` | `?string`               | `null`  | Twitter description.                               |
| `twitterImage`       | `?string`               | `null`  | Twitter image URL.                                 |
| `twitterSite`        | `?string`               | `null`  | Twitter site handle.                               |
| `twitterCreator`     | `?string`               | `null`  | Twitter creator handle.                            |
| `dcTitle`            | `?string`               | `null`  | Dublin Core title.                                 |
| `dcCreator`          | `?string`               | `null`  | Dublin Core creator.                               |
| `dcSubject`          | `?string`               | `null`  | Dublin Core subject.                               |
| `dcDescription`      | `?string`               | `null`  | Dublin Core description.                           |
| `dcPublisher`        | `?string`               | `null`  | Dublin Core publisher.                             |
| `dcDate`             | `?string`               | `null`  | Dublin Core date.                                  |
| `dcType`             | `?string`               | `null`  | Dublin Core type.                                  |
| `dcFormat`           | `?string`               | `null`  | Dublin Core format.                                |
| `dcIdentifier`       | `?string`               | `null`  | Dublin Core identifier.                            |
| `dcLanguage`         | `?string`               | `null`  | Dublin Core language.                              |
| `dcRights`           | `?string`               | `null`  | Dublin Core rights.                                |
| `article`            | `?ArticleMetadata`      | `null`  | Article metadata from `article:*` Open Graph tags. |
| `hreflangs`          | `?array<HreflangEntry>` | `[]`    | Hreflang alternate links.                          |
| `favicons`           | `?array<FaviconInfo>`   | `[]`    | Favicon and icon links.                            |
| `headings`           | `?array<HeadingInfo>`   | `[]`    | Heading elements (h1-h6).                          |
| `wordCount`          | `?int`                  | `null`  | Computed word count of the page body text.         |

---

#### ProxyConfig

Proxy configuration for HTTP requests.

| Field      | Type      | Default | Description                                                    |
| ---------- | --------- | ------- | -------------------------------------------------------------- |
| `url`      | `string`  | —       | Proxy URL (e.g. "<http://proxy:8080",> "socks5://proxy:1080"). |
| `username` | `?string` | `null`  | Optional username for proxy authentication.                    |
| `password` | `?string` | `null`  | Optional password for proxy authentication.                    |

---

#### ResponseMeta

Response metadata extracted from HTTP headers.

| Field             | Type      | Default | Description                        |
| ----------------- | --------- | ------- | ---------------------------------- |
| `etag`            | `?string` | `null`  | The ETag header value.             |
| `lastModified`    | `?string` | `null`  | The Last-Modified header value.    |
| `cacheControl`    | `?string` | `null`  | The Cache-Control header value.    |
| `server`          | `?string` | `null`  | The Server header value.           |
| `xPoweredBy`      | `?string` | `null`  | The X-Powered-By header value.     |
| `contentLanguage` | `?string` | `null`  | The Content-Language header value. |
| `contentEncoding` | `?string` | `null`  | The Content-Encoding header value. |

---

#### ScrapeResult

The result of a single-page scrape operation.

| Field                | Type                     | Default | Description                                                                                                                             |
| -------------------- | ------------------------ | ------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `statusCode`         | `int`                    | —       | The HTTP status code of the response.                                                                                                   |
| `contentType`        | `string`                 | —       | The Content-Type header value.                                                                                                          |
| `html`               | `string`                 | —       | The HTML body of the response.                                                                                                          |
| `bodySize`           | `int`                    | —       | The size of the response body in bytes.                                                                                                 |
| `metadata`           | `PageMetadata`           | —       | Extracted metadata from the page.                                                                                                       |
| `links`              | `array<LinkInfo>`        | `[]`    | Links found on the page.                                                                                                                |
| `images`             | `array<ImageInfo>`       | `[]`    | Images found on the page.                                                                                                               |
| `feeds`              | `array<FeedInfo>`        | `[]`    | Feed links found on the page.                                                                                                           |
| `jsonLd`             | `array<JsonLdEntry>`     | `[]`    | JSON-LD entries found on the page.                                                                                                      |
| `isAllowed`          | `bool`                   | —       | Whether the URL is allowed by robots.txt.                                                                                               |
| `crawlDelay`         | `?int`                   | `null`  | The crawl delay from robots.txt, in seconds.                                                                                            |
| `noindexDetected`    | `bool`                   | —       | Whether a noindex directive was detected.                                                                                               |
| `nofollowDetected`   | `bool`                   | —       | Whether a nofollow directive was detected.                                                                                              |
| `xRobotsTag`         | `?string`                | `null`  | The X-Robots-Tag header value, if present.                                                                                              |
| `isPdf`              | `bool`                   | —       | Whether the content is a PDF.                                                                                                           |
| `wasSkipped`         | `bool`                   | —       | Whether the page was skipped (binary or PDF content).                                                                                   |
| `detectedCharset`    | `?string`                | `null`  | The detected character set encoding.                                                                                                    |
| `authHeaderSent`     | `bool`                   | —       | Whether an authentication header was sent with the request.                                                                             |
| `responseMeta`       | `?ResponseMeta`          | `null`  | Response metadata extracted from HTTP headers.                                                                                          |
| `assets`             | `array<DownloadedAsset>` | `[]`    | Downloaded assets from the page.                                                                                                        |
| `jsRenderHint`       | `bool`                   | —       | Whether the page content suggests JavaScript rendering is needed.                                                                       |
| `browserUsed`        | `bool`                   | —       | Whether the browser fallback was used to fetch this page.                                                                               |
| `markdown`           | `?MarkdownResult`        | `null`  | Markdown conversion of the page content.                                                                                                |
| `extractedData`      | `?mixed`                 | `null`  | Structured data extracted by LLM. Populated when extraction is configured.                                                              |
| `extractionMeta`     | `?ExtractionMeta`        | `null`  | Metadata about the LLM extraction pass (cost, tokens, model).                                                                           |
| `screenshot`         | `?string`                | `null`  | Screenshot of the page as PNG bytes. Populated when browser is used and capture_screenshot is enabled.                                  |
| `downloadedDocument` | `?DownloadedDocument`    | `null`  | Downloaded non-HTML document (PDF, DOCX, image, code, etc.).                                                                            |
| `browser`            | `?BrowserExtras`         | `null`  | Browser-specific extras (eval result, network events, cookies). Only populated when `BrowserBackend::Native` was used for this request. |

---

#### SitemapUrl

A URL entry from a sitemap.

| Field        | Type      | Default | Description                             |
| ------------ | --------- | ------- | --------------------------------------- |
| `url`        | `string`  | —       | The URL.                                |
| `lastmod`    | `?string` | `null`  | The last modification date, if present. |
| `changefreq` | `?string` | `null`  | The change frequency, if present.       |
| `priority`   | `?string` | `null`  | The priority, if present.               |

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

#### BrowserBackend

Browser backend used for JavaScript rendering.

| Value           | Description                                                   |
| --------------- | ------------------------------------------------------------- |
| `Chromiumoxide` | Existing Chromium/CDP backend powered by chromiumoxide.       |
| `Native`        | Kreuzcrawl-owned native browser backend derived from Obscura. |

---

#### AuthConfig

Authentication configuration.

| Value    | Description                                                                     |
| -------- | ------------------------------------------------------------------------------- |
| `Basic`  | HTTP Basic authentication. — Fields: `username`: `string`, `password`: `string` |
| `Bearer` | Bearer token authentication. — Fields: `token`: `string`                        |
| `Header` | Custom authentication header. — Fields: `name`: `string`, `value`: `string`     |

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

#### CrawlEvent

An event emitted during a streaming crawl operation.

Not available on `wasm32` targets — streaming requires native concurrency
primitives (tokio channels, `JoinSet`) that are not supported on wasm32.

Delivered to bindings via alef's streaming-adapter pattern. The
`crawl_stream` / `batch_crawl_stream` binding wrappers in `bindings.rs`
expose this as the per-language streaming idiom (Python `AsyncIterator`,
Ruby `Enumerator`, PHP `Generator`, Elixir `Stream.unfold`, etc.).

| Value      | Description                                                                          |
| ---------- | ------------------------------------------------------------------------------------ |
| `Page`     | A single page has been crawled. — Fields: `result`: `CrawlPageResult`                |
| `Error`    | An error occurred while crawling a URL. — Fields: `url`: `string`, `error`: `string` |
| `Complete` | The crawl has completed. — Fields: `pagesCrawled`: `int`                             |

---

#### PageAction

A single page interaction action.

Actions are serialized with a `type` tag using camelCase naming,
except `ExecuteJs` which is explicitly renamed to `"executeJs"`.

| Value        | Description                                                                                                                                                                                             |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Click`      | Click on an element matching the given CSS selector. — Fields: `selector`: `string`                                                                                                                     |
| `TypeText`   | Type text into an element matching the given CSS selector. — Fields: `selector`: `string`, `text`: `string`                                                                                             |
| `Press`      | Press a keyboard key (e.g. "Enter", "Tab", "Escape"). — Fields: `key`: `string`                                                                                                                         |
| `Scroll`     | Scroll the page or a specific element. — Fields: `direction`: `ScrollDirection`, `selector`: `string`, `amount`: `int`                                                                                  |
| `Wait`       | Wait for a duration or for an element to appear. — Fields: `milliseconds`: `int`, `selector`: `string`                                                                                                  |
| `Screenshot` | Take a screenshot of the current page. — Fields: `fullPage`: `bool`                                                                                                                                     |
| `ExecuteJs`  | Execute arbitrary JavaScript in the page context. **Safety:** The script runs with full page privileges in the browser context. Only execute scripts from trusted sources. — Fields: `script`: `string` |
| `Scrape`     | Scrape the current page HTML.                                                                                                                                                                           |

---

#### ScrollDirection

Direction for a scroll action.

| Value  | Description      |
| ------ | ---------------- |
| `Up`   | Scroll upward.   |
| `Down` | Scroll downward. |

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
| `Unsupported`    | The requested capability is not supported by the active backend or build.          |
| `Other`          | An unclassified error occurred.                                                    |

---
