---
title: "Java API Reference"
---

## Java API Reference <span class="version-badge">v0.1.1</span>

### Functions

#### createEngine()

Create a new crawl engine with the given configuration.

If `config` is `null`, uses `CrawlConfig.default()`.
Returns an error if the configuration is invalid.

**Signature:**

```java
public static CrawlEngineHandle createEngine(CrawlConfig config) throws CrawlError
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `config` | `Optional<CrawlConfig>` | No | The configuration options |

**Returns:** `CrawlEngineHandle`

**Errors:** Throws `CrawlErrorException`.


---

#### scrape()

Scrape a single URL, returning extracted page data.

**Signature:**

```java
public static ScrapeResult scrape(CrawlEngineHandle engine, String url) throws CrawlError
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `url` | `String` | Yes | The URL to fetch |

**Returns:** `ScrapeResult`

**Errors:** Throws `CrawlErrorException`.


---

#### crawl()

Crawl a website starting from `url`, following links up to the configured depth.

**Signature:**

```java
public static CrawlResult crawl(CrawlEngineHandle engine, String url) throws CrawlError
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `url` | `String` | Yes | The URL to fetch |

**Returns:** `CrawlResult`

**Errors:** Throws `CrawlErrorException`.


---

#### mapUrls()

Discover all pages on a website by following links and sitemaps.

**Signature:**

```java
public static MapResult mapUrls(CrawlEngineHandle engine, String url) throws CrawlError
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `url` | `String` | Yes | The URL to fetch |

**Returns:** `MapResult`

**Errors:** Throws `CrawlErrorException`.


---

#### batchScrape()

Scrape multiple URLs concurrently.

**Signature:**

```java
public static List<BatchScrapeResult> batchScrape(CrawlEngineHandle engine, List<String> urls)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `urls` | `List<String>` | Yes | The urls |

**Returns:** `List<BatchScrapeResult>`


---

#### batchCrawl()

Crawl multiple seed URLs concurrently, each following links to configured depth.

**Signature:**

```java
public static List<BatchCrawlResult> batchCrawl(CrawlEngineHandle engine, List<String> urls)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `urls` | `List<String>` | Yes | The urls |

**Returns:** `List<BatchCrawlResult>`


---

### Types

#### ArticleMetadata

Article metadata extracted from `article:*` Open Graph tags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `publishedTime` | `Optional<String>` | `null` | The article publication time. |
| `modifiedTime` | `Optional<String>` | `null` | The article modification time. |
| `author` | `Optional<String>` | `null` | The article author. |
| `section` | `Optional<String>` | `null` | The article section. |
| `tags` | `List<String>` | `Collections.emptyList()` | The article tags. |


---

#### BatchCrawlResult

Result from a single URL in a batch crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The seed URL that was crawled. |
| `result` | `Optional<CrawlResult>` | `null` | The crawl result, if successful. |
| `error` | `Optional<String>` | `null` | The error message, if the crawl failed. |


---

#### BatchScrapeResult

Result from a single URL in a batch scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The URL that was scraped. |
| `result` | `Optional<ScrapeResult>` | `null` | The scrape result, if successful. |
| `error` | `Optional<String>` | `null` | The error message, if the scrape failed. |


---

#### BrowserConfig

Browser fallback configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `mode` | `BrowserMode` | `BrowserMode.AUTO` | When to use the headless browser fallback. |
| `endpoint` | `Optional<String>` | `null` | CDP WebSocket endpoint for connecting to an external browser instance. |
| `timeout` | `Duration` | `30000ms` | Timeout for browser page load and rendering (in milliseconds when serialized). |
| `wait` | `BrowserWait` | `BrowserWait.NETWORK_IDLE` | Wait strategy after browser navigation. |
| `waitSelector` | `Optional<String>` | `null` | CSS selector to wait for when `wait` is `Selector`. |
| `extraWait` | `Optional<Duration>` | `null` | Extra time to wait after the wait condition is met. |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static BrowserConfig defaultOptions()
```


---

#### CitationReference

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `long` | — | Index |
| `url` | `String` | — | Url |
| `text` | `String` | — | Text |


---

#### CitationResult

Result of citation conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | Markdown with links replaced by numbered citations. |
| `references` | `List<CitationReference>` | `Collections.emptyList()` | Numbered reference list: (index, url, text). |


---

#### CookieInfo

Information about an HTTP cookie received from a response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The cookie name. |
| `value` | `String` | — | The cookie value. |
| `domain` | `Optional<String>` | `null` | The cookie domain, if specified. |
| `path` | `Optional<String>` | `null` | The cookie path, if specified. |


---

#### CrawlConfig

Configuration for crawl, scrape, and map operations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `maxDepth` | `Optional<Long>` | `null` | Maximum crawl depth (number of link hops from the start URL). |
| `maxPages` | `Optional<Long>` | `null` | Maximum number of pages to crawl. |
| `maxConcurrent` | `Optional<Long>` | `null` | Maximum number of concurrent requests. |
| `respectRobotsTxt` | `boolean` | `false` | Whether to respect robots.txt directives. |
| `userAgent` | `Optional<String>` | `null` | Custom user-agent string. |
| `stayOnDomain` | `boolean` | `false` | Whether to restrict crawling to the same domain. |
| `allowSubdomains` | `boolean` | `false` | Whether to allow subdomains when `stay_on_domain` is true. |
| `includePaths` | `List<String>` | `Collections.emptyList()` | Regex patterns for paths to include during crawling. |
| `excludePaths` | `List<String>` | `Collections.emptyList()` | Regex patterns for paths to exclude during crawling. |
| `customHeaders` | `Map<String, String>` | `Collections.emptyMap()` | Custom HTTP headers to send with each request. |
| `requestTimeout` | `Duration` | `30000ms` | Timeout for individual HTTP requests (in milliseconds when serialized). |
| `maxRedirects` | `long` | `10` | Maximum number of redirects to follow. |
| `retryCount` | `long` | `0` | Number of retry attempts for failed requests. |
| `retryCodes` | `List<Short>` | `Collections.emptyList()` | HTTP status codes that should trigger a retry. |
| `cookiesEnabled` | `boolean` | `false` | Whether to enable cookie handling. |
| `auth` | `Optional<AuthConfig>` | `null` | Authentication configuration. |
| `maxBodySize` | `Optional<Long>` | `null` | Maximum response body size in bytes. |
| `mainContentOnly` | `boolean` | `false` | Whether to extract only the main content from HTML pages. |
| `removeTags` | `List<String>` | `Collections.emptyList()` | CSS selectors for tags to remove from HTML before processing. |
| `mapLimit` | `Optional<Long>` | `null` | Maximum number of URLs to return from a map operation. |
| `mapSearch` | `Optional<String>` | `null` | Search filter for map results (case-insensitive substring match on URLs). |
| `downloadAssets` | `boolean` | `false` | Whether to download assets (CSS, JS, images, etc.) from the page. |
| `assetTypes` | `List<AssetCategory>` | `Collections.emptyList()` | Filter for asset categories to download. |
| `maxAssetSize` | `Optional<Long>` | `null` | Maximum size in bytes for individual asset downloads. |
| `browser` | `BrowserConfig` | — | Browser configuration. |
| `proxy` | `Optional<ProxyConfig>` | `null` | Proxy configuration for HTTP requests. |
| `userAgents` | `List<String>` | `Collections.emptyList()` | List of user-agent strings for rotation. If non-empty, overrides `user_agent`. |
| `captureScreenshot` | `boolean` | `false` | Whether to capture a screenshot when using the browser. |
| `downloadDocuments` | `boolean` | `true` | Whether to download non-HTML documents (PDF, DOCX, images, code, etc.) instead of skipping them. |
| `documentMaxSize` | `Optional<Long>` | `null` | Maximum size in bytes for document downloads. Defaults to 50 MB. |
| `documentMimeTypes` | `List<String>` | `Collections.emptyList()` | Allowlist of MIME types to download. If empty, uses built-in defaults. |
| `warcOutput` | `Optional<String>` | `null` | Path to write WARC output. If `null`, WARC output is disabled. |
| `browserProfile` | `Optional<String>` | `null` | Named browser profile for persistent sessions (cookies, localStorage). |
| `saveBrowserProfile` | `boolean` | `false` | Whether to save changes back to the browser profile on exit. |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static CrawlConfig defaultOptions()
```

###### validate()

Validate the configuration, returning an error if any values are invalid.

**Signature:**

```java
public void validate() throws CrawlError
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
| `normalizedUrl` | `String` | — | The normalized URL of the page. |
| `statusCode` | `short` | — | The HTTP status code of the response. |
| `contentType` | `String` | — | The Content-Type header value. |
| `html` | `String` | — | The HTML body of the response. |
| `bodySize` | `long` | — | The size of the response body in bytes. |
| `metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `links` | `List<LinkInfo>` | `Collections.emptyList()` | Links found on the page. |
| `images` | `List<ImageInfo>` | `Collections.emptyList()` | Images found on the page. |
| `feeds` | `List<FeedInfo>` | `Collections.emptyList()` | Feed links found on the page. |
| `jsonLd` | `List<JsonLdEntry>` | `Collections.emptyList()` | JSON-LD entries found on the page. |
| `depth` | `long` | — | The depth of this page from the start URL. |
| `stayedOnDomain` | `boolean` | — | Whether this page is on the same domain as the start URL. |
| `wasSkipped` | `boolean` | — | Whether this page was skipped (binary or PDF content). |
| `isPdf` | `boolean` | — | Whether the content is a PDF. |
| `detectedCharset` | `Optional<String>` | `null` | The detected character set encoding. |
| `markdown` | `Optional<MarkdownResult>` | `null` | Markdown conversion of the page content. |
| `extractedData` | `Optional<Object>` | `null` | Structured data extracted by LLM. Populated when extraction is configured. |
| `extractionMeta` | `Optional<ExtractionMeta>` | `null` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `downloadedDocument` | `Optional<DownloadedDocument>` | `null` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |


---

#### CrawlResult

The result of a multi-page crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pages` | `List<CrawlPageResult>` | `Collections.emptyList()` | The list of crawled pages. |
| `finalUrl` | `String` | — | The final URL after following redirects. |
| `redirectCount` | `long` | — | The number of redirects followed. |
| `wasSkipped` | `boolean` | — | Whether any page was skipped during crawling. |
| `error` | `Optional<String>` | `null` | An error message, if the crawl encountered an issue. |
| `cookies` | `List<CookieInfo>` | `Collections.emptyList()` | Cookies collected during the crawl. |
| `normalizedUrls` | `List<String>` | `Collections.emptyList()` | Normalized URLs encountered during crawling (for deduplication counting). |

##### Methods

###### uniqueNormalizedUrls()

Returns the count of unique normalized URLs encountered during crawling.

**Signature:**

```java
public long uniqueNormalizedUrls()
```


---

#### DownloadedAsset

A downloaded asset from a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The original URL of the asset. |
| `contentHash` | `String` | — | The SHA-256 content hash of the asset. |
| `mimeType` | `Optional<String>` | `null` | The MIME type from the Content-Type header. |
| `size` | `long` | — | The size of the asset in bytes. |
| `assetCategory` | `AssetCategory` | `AssetCategory.IMAGE` | The category of the asset. |
| `htmlTag` | `Optional<String>` | `null` | The HTML tag that referenced this asset (e.g., "link", "script", "img"). |


---

#### DownloadedDocument

A downloaded non-HTML document (PDF, DOCX, image, code file, etc.).

When the crawler encounters non-HTML content and `download_documents` is
enabled, it downloads the raw bytes and populates this struct instead of
skipping the resource.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The URL the document was fetched from. |
| `mimeType` | `String` | — | The MIME type from the Content-Type header. |
| `content` | `byte[]` | — | Raw document bytes. Skipped during JSON serialization. |
| `size` | `long` | — | Size of the document in bytes. |
| `filename` | `Optional<String>` | `null` | Filename extracted from Content-Disposition or URL path. |
| `contentHash` | `String` | — | SHA-256 hex digest of the content. |
| `headers` | `Map<String, String>` | `Collections.emptyMap()` | Selected response headers. |


---

#### ExtractionMeta

Metadata about an LLM extraction pass.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cost` | `Optional<Double>` | `null` | Estimated cost of the LLM call in USD. |
| `promptTokens` | `Optional<Long>` | `null` | Number of prompt (input) tokens consumed. |
| `completionTokens` | `Optional<Long>` | `null` | Number of completion (output) tokens generated. |
| `model` | `Optional<String>` | `null` | The model identifier used for extraction. |
| `chunksProcessed` | `long` | — | Number of content chunks sent to the LLM. |


---

#### FaviconInfo

Information about a favicon or icon link.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The icon URL. |
| `rel` | `String` | — | The `rel` attribute (e.g., "icon", "apple-touch-icon"). |
| `sizes` | `Optional<String>` | `null` | The `sizes` attribute, if present. |
| `mimeType` | `Optional<String>` | `null` | The MIME type, if present. |


---

#### FeedInfo

Information about a feed link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The feed URL. |
| `title` | `Optional<String>` | `null` | The feed title, if present. |
| `feedType` | `FeedType` | `FeedType.RSS` | The type of feed. |


---

#### HeadingInfo

A heading element extracted from the page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `byte` | — | The heading level (1-6). |
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
| `alt` | `Optional<String>` | `null` | The alt text, if present. |
| `width` | `Optional<Integer>` | `null` | The width attribute, if present and parseable. |
| `height` | `Optional<Integer>` | `null` | The height attribute, if present and parseable. |
| `source` | `ImageSource` | `ImageSource.IMG` | The source of the image reference. |


---

#### JsonLdEntry

A JSON-LD structured data entry found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `schemaType` | `String` | — | The `@type` value from the JSON-LD object. |
| `name` | `Optional<String>` | `null` | The `name` value, if present. |
| `raw` | `String` | — | The raw JSON-LD string. |


---

#### LinkInfo

Information about a link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The resolved URL of the link. |
| `text` | `String` | — | The visible text of the link. |
| `linkType` | `LinkType` | `LinkType.INTERNAL` | The classification of the link. |
| `rel` | `Optional<String>` | `null` | The `rel` attribute value, if present. |
| `nofollow` | `boolean` | — | Whether the link has `rel="nofollow"`. |


---

#### MapResult

The result of a map operation, containing discovered URLs.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `urls` | `List<SitemapUrl>` | `Collections.emptyList()` | The list of discovered URLs. |


---

#### MarkdownResult

Rich markdown conversion result from HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | Converted markdown text. |
| `documentStructure` | `Optional<Object>` | `null` | Structured document tree with semantic nodes. |
| `tables` | `List<Object>` | `Collections.emptyList()` | Extracted tables with structured cell data. |
| `warnings` | `List<String>` | `Collections.emptyList()` | Non-fatal processing warnings. |
| `citations` | `Optional<CitationResult>` | `null` | Content with links replaced by numbered citations. |
| `fitContent` | `Optional<String>` | `null` | Content-filtered markdown optimized for LLM consumption. |


---

#### PageMetadata

Metadata extracted from an HTML page's `<meta>` tags and `<title>` element.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `Optional<String>` | `null` | The page title from the `<title>` element. |
| `description` | `Optional<String>` | `null` | The meta description. |
| `canonicalUrl` | `Optional<String>` | `null` | The canonical URL from `<link rel="canonical">`. |
| `keywords` | `Optional<String>` | `null` | Keywords from `<meta name="keywords">`. |
| `author` | `Optional<String>` | `null` | Author from `<meta name="author">`. |
| `viewport` | `Optional<String>` | `null` | Viewport content from `<meta name="viewport">`. |
| `themeColor` | `Optional<String>` | `null` | Theme color from `<meta name="theme-color">`. |
| `generator` | `Optional<String>` | `null` | Generator from `<meta name="generator">`. |
| `robots` | `Optional<String>` | `null` | Robots content from `<meta name="robots">`. |
| `htmlLang` | `Optional<String>` | `null` | The `lang` attribute from the `<html>` element. |
| `htmlDir` | `Optional<String>` | `null` | The `dir` attribute from the `<html>` element. |
| `ogTitle` | `Optional<String>` | `null` | Open Graph title. |
| `ogType` | `Optional<String>` | `null` | Open Graph type. |
| `ogImage` | `Optional<String>` | `null` | Open Graph image URL. |
| `ogDescription` | `Optional<String>` | `null` | Open Graph description. |
| `ogUrl` | `Optional<String>` | `null` | Open Graph URL. |
| `ogSiteName` | `Optional<String>` | `null` | Open Graph site name. |
| `ogLocale` | `Optional<String>` | `null` | Open Graph locale. |
| `ogVideo` | `Optional<String>` | `null` | Open Graph video URL. |
| `ogAudio` | `Optional<String>` | `null` | Open Graph audio URL. |
| `ogLocaleAlternates` | `Optional<List<String>>` | `Collections.emptyList()` | Open Graph locale alternates. |
| `twitterCard` | `Optional<String>` | `null` | Twitter card type. |
| `twitterTitle` | `Optional<String>` | `null` | Twitter title. |
| `twitterDescription` | `Optional<String>` | `null` | Twitter description. |
| `twitterImage` | `Optional<String>` | `null` | Twitter image URL. |
| `twitterSite` | `Optional<String>` | `null` | Twitter site handle. |
| `twitterCreator` | `Optional<String>` | `null` | Twitter creator handle. |
| `dcTitle` | `Optional<String>` | `null` | Dublin Core title. |
| `dcCreator` | `Optional<String>` | `null` | Dublin Core creator. |
| `dcSubject` | `Optional<String>` | `null` | Dublin Core subject. |
| `dcDescription` | `Optional<String>` | `null` | Dublin Core description. |
| `dcPublisher` | `Optional<String>` | `null` | Dublin Core publisher. |
| `dcDate` | `Optional<String>` | `null` | Dublin Core date. |
| `dcType` | `Optional<String>` | `null` | Dublin Core type. |
| `dcFormat` | `Optional<String>` | `null` | Dublin Core format. |
| `dcIdentifier` | `Optional<String>` | `null` | Dublin Core identifier. |
| `dcLanguage` | `Optional<String>` | `null` | Dublin Core language. |
| `dcRights` | `Optional<String>` | `null` | Dublin Core rights. |
| `article` | `Optional<ArticleMetadata>` | `null` | Article metadata from `article:*` Open Graph tags. |
| `hreflangs` | `Optional<List<HreflangEntry>>` | `Collections.emptyList()` | Hreflang alternate links. |
| `favicons` | `Optional<List<FaviconInfo>>` | `Collections.emptyList()` | Favicon and icon links. |
| `headings` | `Optional<List<HeadingInfo>>` | `Collections.emptyList()` | Heading elements (h1-h6). |
| `wordCount` | `Optional<Long>` | `null` | Computed word count of the page body text. |


---

#### ProxyConfig

Proxy configuration for HTTP requests.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Proxy URL (e.g. "<http://proxy:8080",> "socks5://proxy:1080"). |
| `username` | `Optional<String>` | `null` | Optional username for proxy authentication. |
| `password` | `Optional<String>` | `null` | Optional password for proxy authentication. |


---

#### ResponseMeta

Response metadata extracted from HTTP headers.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `etag` | `Optional<String>` | `null` | The ETag header value. |
| `lastModified` | `Optional<String>` | `null` | The Last-Modified header value. |
| `cacheControl` | `Optional<String>` | `null` | The Cache-Control header value. |
| `server` | `Optional<String>` | `null` | The Server header value. |
| `xPoweredBy` | `Optional<String>` | `null` | The X-Powered-By header value. |
| `contentLanguage` | `Optional<String>` | `null` | The Content-Language header value. |
| `contentEncoding` | `Optional<String>` | `null` | The Content-Encoding header value. |


---

#### ScrapeResult

The result of a single-page scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `statusCode` | `short` | — | The HTTP status code of the response. |
| `contentType` | `String` | — | The Content-Type header value. |
| `html` | `String` | — | The HTML body of the response. |
| `bodySize` | `long` | — | The size of the response body in bytes. |
| `metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `links` | `List<LinkInfo>` | `Collections.emptyList()` | Links found on the page. |
| `images` | `List<ImageInfo>` | `Collections.emptyList()` | Images found on the page. |
| `feeds` | `List<FeedInfo>` | `Collections.emptyList()` | Feed links found on the page. |
| `jsonLd` | `List<JsonLdEntry>` | `Collections.emptyList()` | JSON-LD entries found on the page. |
| `isAllowed` | `boolean` | — | Whether the URL is allowed by robots.txt. |
| `crawlDelay` | `Optional<Long>` | `null` | The crawl delay from robots.txt, in seconds. |
| `noindexDetected` | `boolean` | — | Whether a noindex directive was detected. |
| `nofollowDetected` | `boolean` | — | Whether a nofollow directive was detected. |
| `xRobotsTag` | `Optional<String>` | `null` | The X-Robots-Tag header value, if present. |
| `isPdf` | `boolean` | — | Whether the content is a PDF. |
| `wasSkipped` | `boolean` | — | Whether the page was skipped (binary or PDF content). |
| `detectedCharset` | `Optional<String>` | `null` | The detected character set encoding. |
| `mainContentOnly` | `boolean` | — | Whether main_content_only was active during extraction. |
| `authHeaderSent` | `boolean` | — | Whether an authentication header was sent with the request. |
| `responseMeta` | `Optional<ResponseMeta>` | `null` | Response metadata extracted from HTTP headers. |
| `assets` | `List<DownloadedAsset>` | `Collections.emptyList()` | Downloaded assets from the page. |
| `jsRenderHint` | `boolean` | — | Whether the page content suggests JavaScript rendering is needed. |
| `browserUsed` | `boolean` | — | Whether the browser fallback was used to fetch this page. |
| `markdown` | `Optional<MarkdownResult>` | `null` | Markdown conversion of the page content. |
| `extractedData` | `Optional<Object>` | `null` | Structured data extracted by LLM. Populated when extraction is configured. |
| `extractionMeta` | `Optional<ExtractionMeta>` | `null` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `screenshot` | `Optional<byte[]>` | `null` | Screenshot of the page as PNG bytes. Populated when browser is used and capture_screenshot is enabled. |
| `downloadedDocument` | `Optional<DownloadedDocument>` | `null` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |


---

#### SitemapUrl

A URL entry from a sitemap.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The URL. |
| `lastmod` | `Optional<String>` | `null` | The last modification date, if present. |
| `changefreq` | `Optional<String>` | `null` | The change frequency, if present. |
| `priority` | `Optional<String>` | `null` | The priority, if present. |


---

### Enums

#### BrowserMode

When to use the headless browser fallback.

| Value | Description |
|-------|-------------|
| `AUTO` | Automatically detect when JS rendering is needed and fall back to browser. |
| `ALWAYS` | Always use the browser for every request. |
| `NEVER` | Never use the browser fallback. |


---

#### BrowserWait

Wait strategy for browser page rendering.

| Value | Description |
|-------|-------------|
| `NETWORK_IDLE` | Wait until network activity is idle. |
| `SELECTOR` | Wait for a specific CSS selector to appear in the DOM. |
| `FIXED` | Wait for a fixed duration after navigation. |


---

#### AuthConfig

Authentication configuration.

| Value | Description |
|-------|-------------|
| `BASIC` | HTTP Basic authentication. — Fields: `username`: `String`, `password`: `String` |
| `BEARER` | Bearer token authentication. — Fields: `token`: `String` |
| `HEADER` | Custom authentication header. — Fields: `name`: `String`, `value`: `String` |


---

#### LinkType

The classification of a link.

| Value | Description |
|-------|-------------|
| `INTERNAL` | A link to the same domain. |
| `EXTERNAL` | A link to a different domain. |
| `ANCHOR` | A fragment-only link (e.g., `#section`). |
| `DOCUMENT` | A link to a downloadable document (PDF, DOC, etc.). |


---

#### ImageSource

The source of an image reference.

| Value | Description |
|-------|-------------|
| `IMG` | An `<img>` tag. |
| `PICTURE_SOURCE` | A `<source>` tag inside `<picture>`. |
| `OG_IMAGE` | An `og:image` meta tag. |
| `TWITTER_IMAGE` | A `twitter:image` meta tag. |


---

#### FeedType

The type of a feed (RSS, Atom, or JSON Feed).

| Value | Description |
|-------|-------------|
| `RSS` | RSS feed. |
| `ATOM` | Atom feed. |
| `JSON_FEED` | JSON Feed. |


---

#### AssetCategory

The category of a downloaded asset.

| Value | Description |
|-------|-------------|
| `DOCUMENT` | A document file (PDF, DOC, etc.). |
| `IMAGE` | An image file. |
| `AUDIO` | An audio file. |
| `VIDEO` | A video file. |
| `FONT` | A font file. |
| `STYLESHEET` | A CSS stylesheet. |
| `SCRIPT` | A JavaScript file. |
| `ARCHIVE` | An archive file (ZIP, TAR, etc.). |
| `DATA` | A data file (JSON, XML, CSV, etc.). |
| `OTHER` | An unrecognized asset type. |


---

### Errors

#### CrawlError

Errors that can occur during crawling, scraping, or mapping operations.

| Variant | Description |
|---------|-------------|
| `NOT_FOUND` | The requested page was not found (HTTP 404). |
| `UNAUTHORIZED` | The request was unauthorized (HTTP 401). |
| `FORBIDDEN` | The request was forbidden (HTTP 403). |
| `WAF_BLOCKED` | The request was blocked by a WAF or bot protection (HTTP 403 with WAF indicators). |
| `TIMEOUT` | The request timed out. |
| `RATE_LIMITED` | The request was rate-limited (HTTP 429). |
| `SERVER_ERROR` | A server error occurred (HTTP 5xx). |
| `BAD_GATEWAY` | A bad gateway error occurred (HTTP 502). |
| `GONE` | The resource is permanently gone (HTTP 410). |
| `CONNECTION` | A connection error occurred. |
| `DNS` | A DNS resolution error occurred. |
| `SSL` | An SSL/TLS error occurred. |
| `DATA_LOSS` | Data was lost or truncated during transfer. |
| `BROWSER_ERROR` | The browser failed to launch, connect, or navigate. |
| `BROWSER_TIMEOUT` | The browser page load or rendering timed out. |
| `INVALID_CONFIG` | The provided configuration is invalid. |
| `OTHER` | An unclassified error occurred. |


---
