---
title: "C# API Reference"
---

## C# API Reference <span class="version-badge">v0.1.1</span>

### Functions

#### CreateEngine()

Create a new crawl engine with the given configuration.

If `config` is `null`, uses `CrawlConfig.default()`.
Returns an error if the configuration is invalid.

**Signature:**

```csharp
public static CrawlEngineHandle CreateEngine(CrawlConfig? config = null)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Config` | `CrawlConfig?` | No | The configuration options |

**Returns:** `CrawlEngineHandle`

**Errors:** Throws `CrawlError`.


---

#### Scrape()

Scrape a single URL, returning extracted page data.

**Signature:**

```csharp
public static async Task<ScrapeResult> ScrapeAsync(CrawlEngineHandle engine, string url)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `Url` | `string` | Yes | The URL to fetch |

**Returns:** `ScrapeResult`

**Errors:** Throws `CrawlError`.


---

#### Crawl()

Crawl a website starting from `url`, following links up to the configured depth.

**Signature:**

```csharp
public static async Task<CrawlResult> CrawlAsync(CrawlEngineHandle engine, string url)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `Url` | `string` | Yes | The URL to fetch |

**Returns:** `CrawlResult`

**Errors:** Throws `CrawlError`.


---

#### MapUrls()

Discover all pages on a website by following links and sitemaps.

**Signature:**

```csharp
public static async Task<MapResult> MapUrlsAsync(CrawlEngineHandle engine, string url)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `Url` | `string` | Yes | The URL to fetch |

**Returns:** `MapResult`

**Errors:** Throws `CrawlError`.


---

#### BatchScrape()

Scrape multiple URLs concurrently.

**Signature:**

```csharp
public static async Task<List<BatchScrapeResult>> BatchScrapeAsync(CrawlEngineHandle engine, List<string> urls)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `Urls` | `List<string>` | Yes | The urls |

**Returns:** `List<BatchScrapeResult>`


---

#### BatchCrawl()

Crawl multiple seed URLs concurrently, each following links to configured depth.

**Signature:**

```csharp
public static async Task<List<BatchCrawlResult>> BatchCrawlAsync(CrawlEngineHandle engine, List<string> urls)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `Urls` | `List<string>` | Yes | The urls |

**Returns:** `List<BatchCrawlResult>`


---

### Types

#### ArticleMetadata

Article metadata extracted from `article:*` Open Graph tags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `PublishedTime` | `string?` | `null` | The article publication time. |
| `ModifiedTime` | `string?` | `null` | The article modification time. |
| `Author` | `string?` | `null` | The article author. |
| `Section` | `string?` | `null` | The article section. |
| `Tags` | `List<string>` | `new List<string>()` | The article tags. |


---

#### BatchCrawlResult

Result from a single URL in a batch crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | The seed URL that was crawled. |
| `Result` | `CrawlResult?` | `null` | The crawl result, if successful. |
| `Error` | `string?` | `null` | The error message, if the crawl failed. |


---

#### BatchScrapeResult

Result from a single URL in a batch scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | The URL that was scraped. |
| `Result` | `ScrapeResult?` | `null` | The scrape result, if successful. |
| `Error` | `string?` | `null` | The error message, if the scrape failed. |


---

#### BrowserConfig

Browser fallback configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Mode` | `BrowserMode` | `BrowserMode.Auto` | When to use the headless browser fallback. |
| `Endpoint` | `string?` | `null` | CDP WebSocket endpoint for connecting to an external browser instance. |
| `Timeout` | `TimeSpan` | `30000ms` | Timeout for browser page load and rendering (in milliseconds when serialized). |
| `Wait` | `BrowserWait` | `BrowserWait.NetworkIdle` | Wait strategy after browser navigation. |
| `WaitSelector` | `string?` | `null` | CSS selector to wait for when `wait` is `Selector`. |
| `ExtraWait` | `TimeSpan?` | `null` | Extra time to wait after the wait condition is met. |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public BrowserConfig CreateDefault()
```


---

#### CitationReference

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Index` | `nuint` | — | Index |
| `Url` | `string` | — | Url |
| `Text` | `string` | — | Text |


---

#### CitationResult

Result of citation conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | Markdown with links replaced by numbered citations. |
| `References` | `List<CitationReference>` | `new List<CitationReference>()` | Numbered reference list: (index, url, text). |


---

#### CookieInfo

Information about an HTTP cookie received from a response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The cookie name. |
| `Value` | `string` | — | The cookie value. |
| `Domain` | `string?` | `null` | The cookie domain, if specified. |
| `Path` | `string?` | `null` | The cookie path, if specified. |


---

#### CrawlConfig

Configuration for crawl, scrape, and map operations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `MaxDepth` | `nuint?` | `null` | Maximum crawl depth (number of link hops from the start URL). |
| `MaxPages` | `nuint?` | `null` | Maximum number of pages to crawl. |
| `MaxConcurrent` | `nuint?` | `null` | Maximum number of concurrent requests. |
| `RespectRobotsTxt` | `bool` | `false` | Whether to respect robots.txt directives. |
| `UserAgent` | `string?` | `null` | Custom user-agent string. |
| `StayOnDomain` | `bool` | `false` | Whether to restrict crawling to the same domain. |
| `AllowSubdomains` | `bool` | `false` | Whether to allow subdomains when `stay_on_domain` is true. |
| `IncludePaths` | `List<string>` | `new List<string>()` | Regex patterns for paths to include during crawling. |
| `ExcludePaths` | `List<string>` | `new List<string>()` | Regex patterns for paths to exclude during crawling. |
| `CustomHeaders` | `Dictionary<string, string>` | `new Dictionary<string, string>()` | Custom HTTP headers to send with each request. |
| `RequestTimeout` | `TimeSpan` | `30000ms` | Timeout for individual HTTP requests (in milliseconds when serialized). |
| `MaxRedirects` | `nuint` | `10` | Maximum number of redirects to follow. |
| `RetryCount` | `nuint` | `0` | Number of retry attempts for failed requests. |
| `RetryCodes` | `List<ushort>` | `new List<ushort>()` | HTTP status codes that should trigger a retry. |
| `CookiesEnabled` | `bool` | `false` | Whether to enable cookie handling. |
| `Auth` | `AuthConfig?` | `null` | Authentication configuration. |
| `MaxBodySize` | `nuint?` | `null` | Maximum response body size in bytes. |
| `MainContentOnly` | `bool` | `false` | Whether to extract only the main content from HTML pages. |
| `RemoveTags` | `List<string>` | `new List<string>()` | CSS selectors for tags to remove from HTML before processing. |
| `MapLimit` | `nuint?` | `null` | Maximum number of URLs to return from a map operation. |
| `MapSearch` | `string?` | `null` | Search filter for map results (case-insensitive substring match on URLs). |
| `DownloadAssets` | `bool` | `false` | Whether to download assets (CSS, JS, images, etc.) from the page. |
| `AssetTypes` | `List<AssetCategory>` | `new List<AssetCategory>()` | Filter for asset categories to download. |
| `MaxAssetSize` | `nuint?` | `null` | Maximum size in bytes for individual asset downloads. |
| `Browser` | `BrowserConfig` | — | Browser configuration. |
| `Proxy` | `ProxyConfig?` | `null` | Proxy configuration for HTTP requests. |
| `UserAgents` | `List<string>` | `new List<string>()` | List of user-agent strings for rotation. If non-empty, overrides `user_agent`. |
| `CaptureScreenshot` | `bool` | `false` | Whether to capture a screenshot when using the browser. |
| `DownloadDocuments` | `bool` | `true` | Whether to download non-HTML documents (PDF, DOCX, images, code, etc.) instead of skipping them. |
| `DocumentMaxSize` | `nuint?` | `null` | Maximum size in bytes for document downloads. Defaults to 50 MB. |
| `DocumentMimeTypes` | `List<string>` | `new List<string>()` | Allowlist of MIME types to download. If empty, uses built-in defaults. |
| `WarcOutput` | `string?` | `null` | Path to write WARC output. If `null`, WARC output is disabled. |
| `BrowserProfile` | `string?` | `null` | Named browser profile for persistent sessions (cookies, localStorage). |
| `SaveBrowserProfile` | `bool` | `false` | Whether to save changes back to the browser profile on exit. |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public CrawlConfig CreateDefault()
```

###### Validate()

Validate the configuration, returning an error if any values are invalid.

**Signature:**

```csharp
public void Validate()
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
| `Url` | `string` | — | The original URL of the page. |
| `NormalizedUrl` | `string` | — | The normalized URL of the page. |
| `StatusCode` | `ushort` | — | The HTTP status code of the response. |
| `ContentType` | `string` | — | The Content-Type header value. |
| `Html` | `string` | — | The HTML body of the response. |
| `BodySize` | `nuint` | — | The size of the response body in bytes. |
| `Metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `Links` | `List<LinkInfo>` | `new List<LinkInfo>()` | Links found on the page. |
| `Images` | `List<ImageInfo>` | `new List<ImageInfo>()` | Images found on the page. |
| `Feeds` | `List<FeedInfo>` | `new List<FeedInfo>()` | Feed links found on the page. |
| `JsonLd` | `List<JsonLdEntry>` | `new List<JsonLdEntry>()` | JSON-LD entries found on the page. |
| `Depth` | `nuint` | — | The depth of this page from the start URL. |
| `StayedOnDomain` | `bool` | — | Whether this page is on the same domain as the start URL. |
| `WasSkipped` | `bool` | — | Whether this page was skipped (binary or PDF content). |
| `IsPdf` | `bool` | — | Whether the content is a PDF. |
| `DetectedCharset` | `string?` | `null` | The detected character set encoding. |
| `Markdown` | `MarkdownResult?` | `null` | Markdown conversion of the page content. |
| `ExtractedData` | `object?` | `null` | Structured data extracted by LLM. Populated when extraction is configured. |
| `ExtractionMeta` | `ExtractionMeta?` | `null` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `DownloadedDocument` | `DownloadedDocument?` | `null` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |


---

#### CrawlResult

The result of a multi-page crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Pages` | `List<CrawlPageResult>` | `new List<CrawlPageResult>()` | The list of crawled pages. |
| `FinalUrl` | `string` | — | The final URL after following redirects. |
| `RedirectCount` | `nuint` | — | The number of redirects followed. |
| `WasSkipped` | `bool` | — | Whether any page was skipped during crawling. |
| `Error` | `string?` | `null` | An error message, if the crawl encountered an issue. |
| `Cookies` | `List<CookieInfo>` | `new List<CookieInfo>()` | Cookies collected during the crawl. |
| `NormalizedUrls` | `List<string>` | `new List<string>()` | Normalized URLs encountered during crawling (for deduplication counting). |

##### Methods

###### UniqueNormalizedUrls()

Returns the count of unique normalized URLs encountered during crawling.

**Signature:**

```csharp
public nuint UniqueNormalizedUrls()
```


---

#### DownloadedAsset

A downloaded asset from a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | The original URL of the asset. |
| `ContentHash` | `string` | — | The SHA-256 content hash of the asset. |
| `MimeType` | `string?` | `null` | The MIME type from the Content-Type header. |
| `Size` | `nuint` | — | The size of the asset in bytes. |
| `AssetCategory` | `AssetCategory` | `AssetCategory.Image` | The category of the asset. |
| `HtmlTag` | `string?` | `null` | The HTML tag that referenced this asset (e.g., "link", "script", "img"). |


---

#### DownloadedDocument

A downloaded non-HTML document (PDF, DOCX, image, code file, etc.).

When the crawler encounters non-HTML content and `download_documents` is
enabled, it downloads the raw bytes and populates this struct instead of
skipping the resource.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | The URL the document was fetched from. |
| `MimeType` | `string` | — | The MIME type from the Content-Type header. |
| `Content` | `byte[]` | — | Raw document bytes. Skipped during JSON serialization. |
| `Size` | `nuint` | — | Size of the document in bytes. |
| `Filename` | `string?` | `null` | Filename extracted from Content-Disposition or URL path. |
| `ContentHash` | `string` | — | SHA-256 hex digest of the content. |
| `Headers` | `Dictionary<string, string>` | `new Dictionary<string, string>()` | Selected response headers. |


---

#### ExtractionMeta

Metadata about an LLM extraction pass.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Cost` | `double?` | `null` | Estimated cost of the LLM call in USD. |
| `PromptTokens` | `ulong?` | `null` | Number of prompt (input) tokens consumed. |
| `CompletionTokens` | `ulong?` | `null` | Number of completion (output) tokens generated. |
| `Model` | `string?` | `null` | The model identifier used for extraction. |
| `ChunksProcessed` | `nuint` | — | Number of content chunks sent to the LLM. |


---

#### FaviconInfo

Information about a favicon or icon link.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | The icon URL. |
| `Rel` | `string` | — | The `rel` attribute (e.g., "icon", "apple-touch-icon"). |
| `Sizes` | `string?` | `null` | The `sizes` attribute, if present. |
| `MimeType` | `string?` | `null` | The MIME type, if present. |


---

#### FeedInfo

Information about a feed link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | The feed URL. |
| `Title` | `string?` | `null` | The feed title, if present. |
| `FeedType` | `FeedType` | `FeedType.Rss` | The type of feed. |


---

#### HeadingInfo

A heading element extracted from the page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Level` | `byte` | — | The heading level (1-6). |
| `Text` | `string` | — | The heading text content. |


---

#### HreflangEntry

An hreflang alternate link entry.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Lang` | `string` | — | The language code (e.g., "en", "fr", "x-default"). |
| `Url` | `string` | — | The URL for this language variant. |


---

#### ImageInfo

Information about an image found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | The image URL. |
| `Alt` | `string?` | `null` | The alt text, if present. |
| `Width` | `uint?` | `null` | The width attribute, if present and parseable. |
| `Height` | `uint?` | `null` | The height attribute, if present and parseable. |
| `Source` | `ImageSource` | `ImageSource.Img` | The source of the image reference. |


---

#### JsonLdEntry

A JSON-LD structured data entry found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `SchemaType` | `string` | — | The `@type` value from the JSON-LD object. |
| `Name` | `string?` | `null` | The `name` value, if present. |
| `Raw` | `string` | — | The raw JSON-LD string. |


---

#### LinkInfo

Information about a link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | The resolved URL of the link. |
| `Text` | `string` | — | The visible text of the link. |
| `LinkType` | `LinkType` | `LinkType.Internal` | The classification of the link. |
| `Rel` | `string?` | `null` | The `rel` attribute value, if present. |
| `Nofollow` | `bool` | — | Whether the link has `rel="nofollow"`. |


---

#### MapResult

The result of a map operation, containing discovered URLs.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Urls` | `List<SitemapUrl>` | `new List<SitemapUrl>()` | The list of discovered URLs. |


---

#### MarkdownResult

Rich markdown conversion result from HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | Converted markdown text. |
| `DocumentStructure` | `object?` | `null` | Structured document tree with semantic nodes. |
| `Tables` | `List<object>` | `new List<object>()` | Extracted tables with structured cell data. |
| `Warnings` | `List<string>` | `new List<string>()` | Non-fatal processing warnings. |
| `Citations` | `CitationResult?` | `null` | Content with links replaced by numbered citations. |
| `FitContent` | `string?` | `null` | Content-filtered markdown optimized for LLM consumption. |


---

#### PageMetadata

Metadata extracted from an HTML page's `<meta>` tags and `<title>` element.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Title` | `string?` | `null` | The page title from the `<title>` element. |
| `Description` | `string?` | `null` | The meta description. |
| `CanonicalUrl` | `string?` | `null` | The canonical URL from `<link rel="canonical">`. |
| `Keywords` | `string?` | `null` | Keywords from `<meta name="keywords">`. |
| `Author` | `string?` | `null` | Author from `<meta name="author">`. |
| `Viewport` | `string?` | `null` | Viewport content from `<meta name="viewport">`. |
| `ThemeColor` | `string?` | `null` | Theme color from `<meta name="theme-color">`. |
| `Generator` | `string?` | `null` | Generator from `<meta name="generator">`. |
| `Robots` | `string?` | `null` | Robots content from `<meta name="robots">`. |
| `HtmlLang` | `string?` | `null` | The `lang` attribute from the `<html>` element. |
| `HtmlDir` | `string?` | `null` | The `dir` attribute from the `<html>` element. |
| `OgTitle` | `string?` | `null` | Open Graph title. |
| `OgType` | `string?` | `null` | Open Graph type. |
| `OgImage` | `string?` | `null` | Open Graph image URL. |
| `OgDescription` | `string?` | `null` | Open Graph description. |
| `OgUrl` | `string?` | `null` | Open Graph URL. |
| `OgSiteName` | `string?` | `null` | Open Graph site name. |
| `OgLocale` | `string?` | `null` | Open Graph locale. |
| `OgVideo` | `string?` | `null` | Open Graph video URL. |
| `OgAudio` | `string?` | `null` | Open Graph audio URL. |
| `OgLocaleAlternates` | `List<string>?` | `new List<string>()` | Open Graph locale alternates. |
| `TwitterCard` | `string?` | `null` | Twitter card type. |
| `TwitterTitle` | `string?` | `null` | Twitter title. |
| `TwitterDescription` | `string?` | `null` | Twitter description. |
| `TwitterImage` | `string?` | `null` | Twitter image URL. |
| `TwitterSite` | `string?` | `null` | Twitter site handle. |
| `TwitterCreator` | `string?` | `null` | Twitter creator handle. |
| `DcTitle` | `string?` | `null` | Dublin Core title. |
| `DcCreator` | `string?` | `null` | Dublin Core creator. |
| `DcSubject` | `string?` | `null` | Dublin Core subject. |
| `DcDescription` | `string?` | `null` | Dublin Core description. |
| `DcPublisher` | `string?` | `null` | Dublin Core publisher. |
| `DcDate` | `string?` | `null` | Dublin Core date. |
| `DcType` | `string?` | `null` | Dublin Core type. |
| `DcFormat` | `string?` | `null` | Dublin Core format. |
| `DcIdentifier` | `string?` | `null` | Dublin Core identifier. |
| `DcLanguage` | `string?` | `null` | Dublin Core language. |
| `DcRights` | `string?` | `null` | Dublin Core rights. |
| `Article` | `ArticleMetadata?` | `null` | Article metadata from `article:*` Open Graph tags. |
| `Hreflangs` | `List<HreflangEntry>?` | `new List<HreflangEntry>()` | Hreflang alternate links. |
| `Favicons` | `List<FaviconInfo>?` | `new List<FaviconInfo>()` | Favicon and icon links. |
| `Headings` | `List<HeadingInfo>?` | `new List<HeadingInfo>()` | Heading elements (h1-h6). |
| `WordCount` | `nuint?` | `null` | Computed word count of the page body text. |


---

#### ProxyConfig

Proxy configuration for HTTP requests.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | Proxy URL (e.g. "<http://proxy:8080",> "socks5://proxy:1080"). |
| `Username` | `string?` | `null` | Optional username for proxy authentication. |
| `Password` | `string?` | `null` | Optional password for proxy authentication. |


---

#### ResponseMeta

Response metadata extracted from HTTP headers.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Etag` | `string?` | `null` | The ETag header value. |
| `LastModified` | `string?` | `null` | The Last-Modified header value. |
| `CacheControl` | `string?` | `null` | The Cache-Control header value. |
| `Server` | `string?` | `null` | The Server header value. |
| `XPoweredBy` | `string?` | `null` | The X-Powered-By header value. |
| `ContentLanguage` | `string?` | `null` | The Content-Language header value. |
| `ContentEncoding` | `string?` | `null` | The Content-Encoding header value. |


---

#### ScrapeResult

The result of a single-page scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `StatusCode` | `ushort` | — | The HTTP status code of the response. |
| `ContentType` | `string` | — | The Content-Type header value. |
| `Html` | `string` | — | The HTML body of the response. |
| `BodySize` | `nuint` | — | The size of the response body in bytes. |
| `Metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `Links` | `List<LinkInfo>` | `new List<LinkInfo>()` | Links found on the page. |
| `Images` | `List<ImageInfo>` | `new List<ImageInfo>()` | Images found on the page. |
| `Feeds` | `List<FeedInfo>` | `new List<FeedInfo>()` | Feed links found on the page. |
| `JsonLd` | `List<JsonLdEntry>` | `new List<JsonLdEntry>()` | JSON-LD entries found on the page. |
| `IsAllowed` | `bool` | — | Whether the URL is allowed by robots.txt. |
| `CrawlDelay` | `ulong?` | `null` | The crawl delay from robots.txt, in seconds. |
| `NoindexDetected` | `bool` | — | Whether a noindex directive was detected. |
| `NofollowDetected` | `bool` | — | Whether a nofollow directive was detected. |
| `XRobotsTag` | `string?` | `null` | The X-Robots-Tag header value, if present. |
| `IsPdf` | `bool` | — | Whether the content is a PDF. |
| `WasSkipped` | `bool` | — | Whether the page was skipped (binary or PDF content). |
| `DetectedCharset` | `string?` | `null` | The detected character set encoding. |
| `MainContentOnly` | `bool` | — | Whether main_content_only was active during extraction. |
| `AuthHeaderSent` | `bool` | — | Whether an authentication header was sent with the request. |
| `ResponseMeta` | `ResponseMeta?` | `null` | Response metadata extracted from HTTP headers. |
| `Assets` | `List<DownloadedAsset>` | `new List<DownloadedAsset>()` | Downloaded assets from the page. |
| `JsRenderHint` | `bool` | — | Whether the page content suggests JavaScript rendering is needed. |
| `BrowserUsed` | `bool` | — | Whether the browser fallback was used to fetch this page. |
| `Markdown` | `MarkdownResult?` | `null` | Markdown conversion of the page content. |
| `ExtractedData` | `object?` | `null` | Structured data extracted by LLM. Populated when extraction is configured. |
| `ExtractionMeta` | `ExtractionMeta?` | `null` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `Screenshot` | `byte[]?` | `null` | Screenshot of the page as PNG bytes. Populated when browser is used and capture_screenshot is enabled. |
| `DownloadedDocument` | `DownloadedDocument?` | `null` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |


---

#### SitemapUrl

A URL entry from a sitemap.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | The URL. |
| `Lastmod` | `string?` | `null` | The last modification date, if present. |
| `Changefreq` | `string?` | `null` | The change frequency, if present. |
| `Priority` | `string?` | `null` | The priority, if present. |


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
| `Basic` | HTTP Basic authentication. — Fields: `Username`: `string`, `Password`: `string` |
| `Bearer` | Bearer token authentication. — Fields: `Token`: `string` |
| `Header` | Custom authentication header. — Fields: `Name`: `string`, `Value`: `string` |


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
