---
title: "WebAssembly API Reference"
---

## WebAssembly API Reference <span class="version-badge">v0.3.0-rc.1</span>

### Functions

#### createEngine()

Create a new crawl engine with the given configuration.

If `config` is `null`, uses `CrawlConfig.default()`.
Returns an error if the configuration is invalid.

**Signature:**

```typescript
function createEngine(config?: CrawlConfig): CrawlEngineHandle
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `CrawlConfig | null` | No | The configuration options |

**Returns:** `CrawlEngineHandle`

**Errors:** Throws `Error` with a descriptive message.


---

#### scrape()

Scrape a single URL, returning extracted page data.

**Signature:**

```typescript
function scrape(engine: CrawlEngineHandle, url: string): Promise<ScrapeResult>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `url` | `string` | Yes | The URL to fetch |

**Returns:** `ScrapeResult`

**Errors:** Throws `Error` with a descriptive message.


---

#### crawl()

Crawl a website starting from `url`, following links up to the configured depth.

**Signature:**

```typescript
function crawl(engine: CrawlEngineHandle, url: string): Promise<CrawlResult>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `url` | `string` | Yes | The URL to fetch |

**Returns:** `CrawlResult`

**Errors:** Throws `Error` with a descriptive message.


---

#### mapUrls()

Discover all pages on a website by following links and sitemaps.

**Signature:**

```typescript
function mapUrls(engine: CrawlEngineHandle, url: string): Promise<MapResult>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `url` | `string` | Yes | The URL to fetch |

**Returns:** `MapResult`

**Errors:** Throws `Error` with a descriptive message.


---

#### batchScrape()

Scrape multiple URLs concurrently.

**Signature:**

```typescript
function batchScrape(engine: CrawlEngineHandle, urls: Array<string>): Promise<Array<BatchScrapeResult>>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `urls` | `Array<string>` | Yes | The urls |

**Returns:** `Array<BatchScrapeResult>`


---

#### batchCrawl()

Crawl multiple seed URLs concurrently, each following links to configured depth.

**Signature:**

```typescript
function batchCrawl(engine: CrawlEngineHandle, urls: Array<string>): Promise<Array<BatchCrawlResult>>
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `urls` | `Array<string>` | Yes | The urls |

**Returns:** `Array<BatchCrawlResult>`


---

### Types

#### ArticleMetadata

Article metadata extracted from `article:*` Open Graph tags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `publishedTime` | `string | null` | `null` | The article publication time. |
| `modifiedTime` | `string | null` | `null` | The article modification time. |
| `author` | `string | null` | `null` | The article author. |
| `section` | `string | null` | `null` | The article section. |
| `tags` | `Array<string>` | `[]` | The article tags. |


---

#### BatchCrawlResult

Result from a single URL in a batch crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `string` | — | The seed URL that was crawled. |
| `result` | `CrawlResult | null` | `null` | The crawl result, if successful. |
| `error` | `string | null` | `null` | The error message, if the crawl failed. |


---

#### BatchScrapeResult

Result from a single URL in a batch scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `string` | — | The URL that was scraped. |
| `result` | `ScrapeResult | null` | `null` | The scrape result, if successful. |
| `error` | `string | null` | `null` | The error message, if the scrape failed. |


---

#### BrowserConfig

Browser fallback configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `mode` | `BrowserMode` | `BrowserMode.Auto` | When to use the headless browser fallback. |
| `endpoint` | `string | null` | `null` | CDP WebSocket endpoint for connecting to an external browser instance. |
| `timeout` | `number` | `30000ms` | Timeout for browser page load and rendering (in milliseconds when serialized). |
| `wait` | `BrowserWait` | `BrowserWait.NetworkIdle` | Wait strategy after browser navigation. |
| `waitSelector` | `string | null` | `null` | CSS selector to wait for when `wait` is `Selector`. |
| `extraWait` | `number | null` | `null` | Extra time to wait after the wait condition is met. |

##### Methods

###### default()

**Signature:**

```typescript
static default(): BrowserConfig
```


---

#### CitationReference

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `number` | — | Index |
| `url` | `string` | — | Url |
| `text` | `string` | — | Text |


---

#### CitationResult

Result of citation conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `string` | — | Markdown with links replaced by numbered citations. |
| `references` | `Array<CitationReference>` | `[]` | Numbered reference list: (index, url, text). |


---

#### ContentConfig

Content extraction and conversion configuration.

Controls how HTML is converted to the output format. Uses
html-to-markdown-rs as the conversion engine for all formats
(markdown, plain text, djot).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `outputFormat` | `string` | `"markdown"` | Output format: `"markdown"` (default), `"plain"`, `"djot"`. |
| `preprocessingPreset` | `string` | `"standard"` | Preprocessing aggressiveness: `"minimal"`, `"standard"` (default), `"aggressive"`. - Minimal: only scripts/styles removed. - Standard: also removes nav, nav-hinted headers/footers/asides, forms. - Aggressive: removes all footers/asides unconditionally. |
| `removeNavigation` | `boolean` | `true` | Remove navigation elements (nav, breadcrumbs, menus). Default: `true`. |
| `removeForms` | `boolean` | `true` | Remove form elements. Default: `true`. |
| `stripTags` | `Array<string>` | `[]` | HTML tag names to strip (render children only, remove the tag wrapper). Default: `["noscript"]`. |
| `preserveTags` | `Array<string>` | `[]` | HTML tag names to preserve as raw HTML in output. |
| `excludeSelectors` | `Array<string>` | `[]` | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the wrapper but keeps children), excluded elements and all descendants are dropped. Supports CSS selectors: `.class`, `#id`, `[attribute]`, compound selectors. Example: `[".cookie-banner", "#ad-container", "[role='complementary']"]` |
| `skipImages` | `boolean` | `false` | Skip image elements in output. Default: `false`. |
| `maxDepth` | `number | null` | `null` | Max DOM traversal depth. Prevents stack overflow on deeply nested HTML. |
| `wrap` | `boolean` | `false` | Enable line wrapping. Default: `false`. |
| `wrapWidth` | `number` | `80` | Wrap width when `wrap` is enabled. Default: `80`. |
| `includeDocumentStructure` | `boolean` | `true` | Include document structure tree in output. Default: `true`. |

##### Methods

###### default()

**Signature:**

```typescript
static default(): ContentConfig
```


---

#### CookieInfo

Information about an HTTP cookie received from a response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `string` | — | The cookie name. |
| `value` | `string` | — | The cookie value. |
| `domain` | `string | null` | `null` | The cookie domain, if specified. |
| `path` | `string | null` | `null` | The cookie path, if specified. |


---

#### CrawlConfig

Configuration for crawl, scrape, and map operations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `maxDepth` | `number | null` | `null` | Maximum crawl depth (number of link hops from the start URL). |
| `maxPages` | `number | null` | `null` | Maximum number of pages to crawl. |
| `maxConcurrent` | `number | null` | `null` | Maximum number of concurrent requests. |
| `respectRobotsTxt` | `boolean` | `false` | Whether to respect robots.txt directives. |
| `userAgent` | `string | null` | `null` | Custom user-agent string. |
| `stayOnDomain` | `boolean` | `false` | Whether to restrict crawling to the same domain. |
| `allowSubdomains` | `boolean` | `false` | Whether to allow subdomains when `stay_on_domain` is true. |
| `includePaths` | `Array<string>` | `[]` | Regex patterns for paths to include during crawling. |
| `excludePaths` | `Array<string>` | `[]` | Regex patterns for paths to exclude during crawling. |
| `customHeaders` | `Record<string, string>` | `{}` | Custom HTTP headers to send with each request. |
| `requestTimeout` | `number` | `30000ms` | Timeout for individual HTTP requests (in milliseconds when serialized). |
| `rateLimitMs` | `number | null` | `null` | Per-domain rate limit in milliseconds. When set, enforces a minimum delay between requests to the same domain. Defaults to 200ms when `null`. |
| `maxRedirects` | `number` | `10` | Maximum number of redirects to follow. |
| `retryCount` | `number` | `0` | Number of retry attempts for failed requests. |
| `retryCodes` | `Array<number>` | `[]` | HTTP status codes that should trigger a retry. |
| `cookiesEnabled` | `boolean` | `false` | Whether to enable cookie handling. |
| `auth` | `AuthConfig | null` | `null` | Authentication configuration. |
| `maxBodySize` | `number | null` | `null` | Maximum response body size in bytes. |
| `removeTags` | `Array<string>` | `[]` | CSS selectors for tags to remove from HTML before processing. |
| `content` | `ContentConfig` | — | Content extraction and conversion configuration. |
| `mapLimit` | `number | null` | `null` | Maximum number of URLs to return from a map operation. |
| `mapSearch` | `string | null` | `null` | Search filter for map results (case-insensitive substring match on URLs). |
| `downloadAssets` | `boolean` | `false` | Whether to download assets (CSS, JS, images, etc.) from the page. |
| `assetTypes` | `Array<AssetCategory>` | `[]` | Filter for asset categories to download. |
| `maxAssetSize` | `number | null` | `null` | Maximum size in bytes for individual asset downloads. |
| `browser` | `BrowserConfig` | — | Browser configuration. |
| `proxy` | `ProxyConfig | null` | `null` | Proxy configuration for HTTP requests. |
| `userAgents` | `Array<string>` | `[]` | List of user-agent strings for rotation. If non-empty, overrides `user_agent`. |
| `captureScreenshot` | `boolean` | `false` | Whether to capture a screenshot when using the browser. |
| `downloadDocuments` | `boolean` | `true` | Whether to download non-HTML documents (PDF, DOCX, images, code, etc.) instead of skipping them. |
| `documentMaxSize` | `number | null` | `null` | Maximum size in bytes for document downloads. Defaults to 50 MB. |
| `documentMimeTypes` | `Array<string>` | `[]` | Allowlist of MIME types to download. If empty, uses built-in defaults. |
| `warcOutput` | `string | null` | `null` | Path to write WARC output. If `null`, WARC output is disabled. |
| `browserProfile` | `string | null` | `null` | Named browser profile for persistent sessions (cookies, localStorage). |
| `saveBrowserProfile` | `boolean` | `false` | Whether to save changes back to the browser profile on exit. |

##### Methods

###### default()

**Signature:**

```typescript
static default(): CrawlConfig
```

###### validate()

Validate the configuration, returning an error if any values are invalid.

**Signature:**

```typescript
validate(): void
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
| `url` | `string` | — | The original URL of the page. |
| `normalizedUrl` | `string` | — | The normalized URL of the page. |
| `statusCode` | `number` | — | The HTTP status code of the response. |
| `contentType` | `string` | — | The Content-Type header value. |
| `html` | `string` | — | The HTML body of the response. |
| `bodySize` | `number` | — | The size of the response body in bytes. |
| `metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `links` | `Array<LinkInfo>` | `[]` | Links found on the page. |
| `images` | `Array<ImageInfo>` | `[]` | Images found on the page. |
| `feeds` | `Array<FeedInfo>` | `[]` | Feed links found on the page. |
| `jsonLd` | `Array<JsonLdEntry>` | `[]` | JSON-LD entries found on the page. |
| `depth` | `number` | — | The depth of this page from the start URL. |
| `stayedOnDomain` | `boolean` | — | Whether this page is on the same domain as the start URL. |
| `wasSkipped` | `boolean` | — | Whether this page was skipped (binary or PDF content). |
| `isPdf` | `boolean` | — | Whether the content is a PDF. |
| `detectedCharset` | `string | null` | `null` | The detected character set encoding. |
| `markdown` | `MarkdownResult | null` | `null` | Markdown conversion of the page content. |
| `extractedData` | `unknown | null` | `null` | Structured data extracted by LLM. Populated when extraction is configured. |
| `extractionMeta` | `ExtractionMeta | null` | `null` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `downloadedDocument` | `DownloadedDocument | null` | `null` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |


---

#### CrawlResult

The result of a multi-page crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pages` | `Array<CrawlPageResult>` | `[]` | The list of crawled pages. |
| `finalUrl` | `string` | — | The final URL after following redirects. |
| `redirectCount` | `number` | — | The number of redirects followed. |
| `wasSkipped` | `boolean` | — | Whether any page was skipped during crawling. |
| `error` | `string | null` | `null` | An error message, if the crawl encountered an issue. |
| `cookies` | `Array<CookieInfo>` | `[]` | Cookies collected during the crawl. |
| `normalizedUrls` | `Array<string>` | `[]` | Normalized URLs encountered during crawling (for deduplication counting). |

##### Methods

###### uniqueNormalizedUrls()

Returns the count of unique normalized URLs encountered during crawling.

**Signature:**

```typescript
uniqueNormalizedUrls(): number
```


---

#### DownloadedAsset

A downloaded asset from a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `string` | — | The original URL of the asset. |
| `contentHash` | `string` | — | The SHA-256 content hash of the asset. |
| `mimeType` | `string | null` | `null` | The MIME type from the Content-Type header. |
| `size` | `number` | — | The size of the asset in bytes. |
| `assetCategory` | `AssetCategory` | `AssetCategory.Image` | The category of the asset. |
| `htmlTag` | `string | null` | `null` | The HTML tag that referenced this asset (e.g., "link", "script", "img"). |


---

#### DownloadedDocument

A downloaded non-HTML document (PDF, DOCX, image, code file, etc.).

When the crawler encounters non-HTML content and `download_documents` is
enabled, it downloads the raw bytes and populates this struct instead of
skipping the resource.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `string` | — | The URL the document was fetched from. |
| `mimeType` | `string` | — | The MIME type from the Content-Type header. |
| `content` | `Buffer` | — | Raw document bytes. Skipped during JSON serialization. |
| `size` | `number` | — | Size of the document in bytes. |
| `filename` | `string | null` | `null` | Filename extracted from Content-Disposition or URL path. |
| `contentHash` | `string` | — | SHA-256 hex digest of the content. |
| `headers` | `Record<string, string>` | `{}` | Selected response headers. |


---

#### ExtractionMeta

Metadata about an LLM extraction pass.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cost` | `number | null` | `null` | Estimated cost of the LLM call in USD. |
| `promptTokens` | `number | null` | `null` | Number of prompt (input) tokens consumed. |
| `completionTokens` | `number | null` | `null` | Number of completion (output) tokens generated. |
| `model` | `string | null` | `null` | The model identifier used for extraction. |
| `chunksProcessed` | `number` | — | Number of content chunks sent to the LLM. |


---

#### FaviconInfo

Information about a favicon or icon link.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `string` | — | The icon URL. |
| `rel` | `string` | — | The `rel` attribute (e.g., "icon", "apple-touch-icon"). |
| `sizes` | `string | null` | `null` | The `sizes` attribute, if present. |
| `mimeType` | `string | null` | `null` | The MIME type, if present. |


---

#### FeedInfo

Information about a feed link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `string` | — | The feed URL. |
| `title` | `string | null` | `null` | The feed title, if present. |
| `feedType` | `FeedType` | `FeedType.Rss` | The type of feed. |


---

#### HeadingInfo

A heading element extracted from the page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `number` | — | The heading level (1-6). |
| `text` | `string` | — | The heading text content. |


---

#### HreflangEntry

An hreflang alternate link entry.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `lang` | `string` | — | The language code (e.g., "en", "fr", "x-default"). |
| `url` | `string` | — | The URL for this language variant. |


---

#### ImageInfo

Information about an image found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `string` | — | The image URL. |
| `alt` | `string | null` | `null` | The alt text, if present. |
| `width` | `number | null` | `null` | The width attribute, if present and parseable. |
| `height` | `number | null` | `null` | The height attribute, if present and parseable. |
| `source` | `ImageSource` | `ImageSource.Img` | The source of the image reference. |


---

#### JsonLdEntry

A JSON-LD structured data entry found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `schemaType` | `string` | — | The `@type` value from the JSON-LD object. |
| `name` | `string | null` | `null` | The `name` value, if present. |
| `raw` | `string` | — | The raw JSON-LD string. |


---

#### LinkInfo

Information about a link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `string` | — | The resolved URL of the link. |
| `text` | `string` | — | The visible text of the link. |
| `linkType` | `LinkType` | `LinkType.Internal` | The classification of the link. |
| `rel` | `string | null` | `null` | The `rel` attribute value, if present. |
| `nofollow` | `boolean` | — | Whether the link has `rel="nofollow"`. |


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
| `content` | `string` | — | Converted markdown text. |
| `documentStructure` | `unknown | null` | `null` | Structured document tree with semantic nodes. |
| `tables` | `Array<unknown>` | `[]` | Extracted tables with structured cell data. |
| `warnings` | `Array<string>` | `[]` | Non-fatal processing warnings. |
| `citations` | `CitationResult | null` | `null` | Content with links replaced by numbered citations. |
| `fitContent` | `string | null` | `null` | Content-filtered markdown optimized for LLM consumption. |


---

#### PageMetadata

Metadata extracted from an HTML page's `<meta>` tags and `<title>` element.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `string | null` | `null` | The page title from the `<title>` element. |
| `description` | `string | null` | `null` | The meta description. |
| `canonicalUrl` | `string | null` | `null` | The canonical URL from `<link rel="canonical">`. |
| `keywords` | `string | null` | `null` | Keywords from `<meta name="keywords">`. |
| `author` | `string | null` | `null` | Author from `<meta name="author">`. |
| `viewport` | `string | null` | `null` | Viewport content from `<meta name="viewport">`. |
| `themeColor` | `string | null` | `null` | Theme color from `<meta name="theme-color">`. |
| `generator` | `string | null` | `null` | Generator from `<meta name="generator">`. |
| `robots` | `string | null` | `null` | Robots content from `<meta name="robots">`. |
| `htmlLang` | `string | null` | `null` | The `lang` attribute from the `<html>` element. |
| `htmlDir` | `string | null` | `null` | The `dir` attribute from the `<html>` element. |
| `ogTitle` | `string | null` | `null` | Open Graph title. |
| `ogType` | `string | null` | `null` | Open Graph type. |
| `ogImage` | `string | null` | `null` | Open Graph image URL. |
| `ogDescription` | `string | null` | `null` | Open Graph description. |
| `ogUrl` | `string | null` | `null` | Open Graph URL. |
| `ogSiteName` | `string | null` | `null` | Open Graph site name. |
| `ogLocale` | `string | null` | `null` | Open Graph locale. |
| `ogVideo` | `string | null` | `null` | Open Graph video URL. |
| `ogAudio` | `string | null` | `null` | Open Graph audio URL. |
| `ogLocaleAlternates` | `Array<string> | null` | `[]` | Open Graph locale alternates. |
| `twitterCard` | `string | null` | `null` | Twitter card type. |
| `twitterTitle` | `string | null` | `null` | Twitter title. |
| `twitterDescription` | `string | null` | `null` | Twitter description. |
| `twitterImage` | `string | null` | `null` | Twitter image URL. |
| `twitterSite` | `string | null` | `null` | Twitter site handle. |
| `twitterCreator` | `string | null` | `null` | Twitter creator handle. |
| `dcTitle` | `string | null` | `null` | Dublin Core title. |
| `dcCreator` | `string | null` | `null` | Dublin Core creator. |
| `dcSubject` | `string | null` | `null` | Dublin Core subject. |
| `dcDescription` | `string | null` | `null` | Dublin Core description. |
| `dcPublisher` | `string | null` | `null` | Dublin Core publisher. |
| `dcDate` | `string | null` | `null` | Dublin Core date. |
| `dcType` | `string | null` | `null` | Dublin Core type. |
| `dcFormat` | `string | null` | `null` | Dublin Core format. |
| `dcIdentifier` | `string | null` | `null` | Dublin Core identifier. |
| `dcLanguage` | `string | null` | `null` | Dublin Core language. |
| `dcRights` | `string | null` | `null` | Dublin Core rights. |
| `article` | `ArticleMetadata | null` | `null` | Article metadata from `article:*` Open Graph tags. |
| `hreflangs` | `Array<HreflangEntry> | null` | `[]` | Hreflang alternate links. |
| `favicons` | `Array<FaviconInfo> | null` | `[]` | Favicon and icon links. |
| `headings` | `Array<HeadingInfo> | null` | `[]` | Heading elements (h1-h6). |
| `wordCount` | `number | null` | `null` | Computed word count of the page body text. |


---

#### ProxyConfig

Proxy configuration for HTTP requests.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `string` | — | Proxy URL (e.g. "<http://proxy:8080",> "socks5://proxy:1080"). |
| `username` | `string | null` | `null` | Optional username for proxy authentication. |
| `password` | `string | null` | `null` | Optional password for proxy authentication. |


---

#### ResponseMeta

Response metadata extracted from HTTP headers.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `etag` | `string | null` | `null` | The ETag header value. |
| `lastModified` | `string | null` | `null` | The Last-Modified header value. |
| `cacheControl` | `string | null` | `null` | The Cache-Control header value. |
| `server` | `string | null` | `null` | The Server header value. |
| `xPoweredBy` | `string | null` | `null` | The X-Powered-By header value. |
| `contentLanguage` | `string | null` | `null` | The Content-Language header value. |
| `contentEncoding` | `string | null` | `null` | The Content-Encoding header value. |


---

#### ScrapeResult

The result of a single-page scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `statusCode` | `number` | — | The HTTP status code of the response. |
| `contentType` | `string` | — | The Content-Type header value. |
| `html` | `string` | — | The HTML body of the response. |
| `bodySize` | `number` | — | The size of the response body in bytes. |
| `metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `links` | `Array<LinkInfo>` | `[]` | Links found on the page. |
| `images` | `Array<ImageInfo>` | `[]` | Images found on the page. |
| `feeds` | `Array<FeedInfo>` | `[]` | Feed links found on the page. |
| `jsonLd` | `Array<JsonLdEntry>` | `[]` | JSON-LD entries found on the page. |
| `isAllowed` | `boolean` | — | Whether the URL is allowed by robots.txt. |
| `crawlDelay` | `number | null` | `null` | The crawl delay from robots.txt, in seconds. |
| `noindexDetected` | `boolean` | — | Whether a noindex directive was detected. |
| `nofollowDetected` | `boolean` | — | Whether a nofollow directive was detected. |
| `xRobotsTag` | `string | null` | `null` | The X-Robots-Tag header value, if present. |
| `isPdf` | `boolean` | — | Whether the content is a PDF. |
| `wasSkipped` | `boolean` | — | Whether the page was skipped (binary or PDF content). |
| `detectedCharset` | `string | null` | `null` | The detected character set encoding. |
| `authHeaderSent` | `boolean` | — | Whether an authentication header was sent with the request. |
| `responseMeta` | `ResponseMeta | null` | `null` | Response metadata extracted from HTTP headers. |
| `assets` | `Array<DownloadedAsset>` | `[]` | Downloaded assets from the page. |
| `jsRenderHint` | `boolean` | — | Whether the page content suggests JavaScript rendering is needed. |
| `browserUsed` | `boolean` | — | Whether the browser fallback was used to fetch this page. |
| `markdown` | `MarkdownResult | null` | `null` | Markdown conversion of the page content. |
| `extractedData` | `unknown | null` | `null` | Structured data extracted by LLM. Populated when extraction is configured. |
| `extractionMeta` | `ExtractionMeta | null` | `null` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `screenshot` | `Buffer | null` | `null` | Screenshot of the page as PNG bytes. Populated when browser is used and capture_screenshot is enabled. |
| `downloadedDocument` | `DownloadedDocument | null` | `null` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |


---

#### SitemapUrl

A URL entry from a sitemap.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `string` | — | The URL. |
| `lastmod` | `string | null` | `null` | The last modification date, if present. |
| `changefreq` | `string | null` | `null` | The change frequency, if present. |
| `priority` | `string | null` | `null` | The priority, if present. |


---

### Enums

#### BrowserMode

When to use the headless browser fallback.

| Value | Description |
|-------|-------------|
| `Auto` | Automatically detect when JS rendering is needed and fall back to browser. |
| `Always` | Always use the browser for every request. |
| `Never` | Never use the browser fallback. |


---

#### BrowserWait

Wait strategy for browser page rendering.

| Value | Description |
|-------|-------------|
| `NetworkIdle` | Wait until network activity is idle. |
| `Selector` | Wait for a specific CSS selector to appear in the DOM. |
| `Fixed` | Wait for a fixed duration after navigation. |


---

#### AuthConfig

Authentication configuration.

| Value | Description |
|-------|-------------|
| `Basic` | HTTP Basic authentication. — Fields: `username`: `string`, `password`: `string` |
| `Bearer` | Bearer token authentication. — Fields: `token`: `string` |
| `Header` | Custom authentication header. — Fields: `name`: `string`, `value`: `string` |


---

#### LinkType

The classification of a link.

| Value | Description |
|-------|-------------|
| `Internal` | A link to the same domain. |
| `External` | A link to a different domain. |
| `Anchor` | A fragment-only link (e.g., `#section`). |
| `Document` | A link to a downloadable document (PDF, DOC, etc.). |


---

#### ImageSource

The source of an image reference.

| Value | Description |
|-------|-------------|
| `Img` | An `<img>` tag. |
| `PictureSource` | A `<source>` tag inside `<picture>`. |
| `OgImage` | An `og:image` meta tag. |
| `TwitterImage` | A `twitter:image` meta tag. |


---

#### FeedType

The type of a feed (RSS, Atom, or JSON Feed).

| Value | Description |
|-------|-------------|
| `Rss` | RSS feed. |
| `Atom` | Atom feed. |
| `JsonFeed` | JSON Feed. |


---

#### AssetCategory

The category of a downloaded asset.

| Value | Description |
|-------|-------------|
| `Document` | A document file (PDF, DOC, etc.). |
| `Image` | An image file. |
| `Audio` | An audio file. |
| `Video` | A video file. |
| `Font` | A font file. |
| `Stylesheet` | A CSS stylesheet. |
| `Script` | A JavaScript file. |
| `Archive` | An archive file (ZIP, TAR, etc.). |
| `Data` | A data file (JSON, XML, CSV, etc.). |
| `Other` | An unrecognized asset type. |


---

### Errors

#### CrawlError

Errors that can occur during crawling, scraping, or mapping operations.

Errors are thrown as plain `Error` objects with descriptive messages.

| Variant | Description |
|---------|-------------|
| `NotFound` | The requested page was not found (HTTP 404). |
| `Unauthorized` | The request was unauthorized (HTTP 401). |
| `Forbidden` | The request was forbidden (HTTP 403). |
| `WafBlocked` | The request was blocked by a WAF or bot protection (HTTP 403 with WAF indicators). |
| `Timeout` | The request timed out. |
| `RateLimited` | The request was rate-limited (HTTP 429). |
| `ServerError` | A server error occurred (HTTP 5xx). |
| `BadGateway` | A bad gateway error occurred (HTTP 502). |
| `Gone` | The resource is permanently gone (HTTP 410). |
| `Connection` | A connection error occurred. |
| `Dns` | A DNS resolution error occurred. |
| `Ssl` | An SSL/TLS error occurred. |
| `DataLoss` | Data was lost or truncated during transfer. |
| `BrowserError` | The browser failed to launch, connect, or navigate. |
| `BrowserTimeout` | The browser page load or rendering timed out. |
| `InvalidConfig` | The provided configuration is invalid. |
| `Other` | An unclassified error occurred. |


---
