---
title: "Go API Reference"
---

## Go API Reference <span class="version-badge">v0.3.0-rc.1</span>

### Functions

#### CreateEngine()

Create a new crawl engine with the given configuration.

If `config` is `nil`, uses `CrawlConfig.default()`.
Returns an error if the configuration is invalid.

**Signature:**

```go
func CreateEngine(config CrawlConfig) (CrawlEngineHandle, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Config` | `*CrawlConfig` | No | The configuration options |

**Returns:** `CrawlEngineHandle`

**Errors:** Returns `error`.


---

#### Scrape()

Scrape a single URL, returning extracted page data.

**Signature:**

```go
func Scrape(engine CrawlEngineHandle, url string) (ScrapeResult, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `Url` | `string` | Yes | The URL to fetch |

**Returns:** `ScrapeResult`

**Errors:** Returns `error`.


---

#### Crawl()

Crawl a website starting from `url`, following links up to the configured depth.

**Signature:**

```go
func Crawl(engine CrawlEngineHandle, url string) (CrawlResult, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `Url` | `string` | Yes | The URL to fetch |

**Returns:** `CrawlResult`

**Errors:** Returns `error`.


---

#### MapUrls()

Discover all pages on a website by following links and sitemaps.

**Signature:**

```go
func MapUrls(engine CrawlEngineHandle, url string) (MapResult, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `Url` | `string` | Yes | The URL to fetch |

**Returns:** `MapResult`

**Errors:** Returns `error`.


---

#### BatchScrape()

Scrape multiple URLs concurrently.

**Signature:**

```go
func BatchScrape(engine CrawlEngineHandle, urls []string) []BatchScrapeResult
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `Urls` | `[]string` | Yes | The urls |

**Returns:** `[]BatchScrapeResult`


---

#### BatchCrawl()

Crawl multiple seed URLs concurrently, each following links to configured depth.

**Signature:**

```go
func BatchCrawl(engine CrawlEngineHandle, urls []string) []BatchCrawlResult
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Engine` | `CrawlEngineHandle` | Yes | The crawl engine handle |
| `Urls` | `[]string` | Yes | The urls |

**Returns:** `[]BatchCrawlResult`


---

### Types

#### ArticleMetadata

Article metadata extracted from `article:*` Open Graph tags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `PublishedTime` | `*string` | `nil` | The article publication time. |
| `ModifiedTime` | `*string` | `nil` | The article modification time. |
| `Author` | `*string` | `nil` | The article author. |
| `Section` | `*string` | `nil` | The article section. |
| `Tags` | `[]string` | `nil` | The article tags. |


---

#### BatchCrawlResult

Result from a single URL in a batch crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | The seed URL that was crawled. |
| `Result` | `*CrawlResult` | `nil` | The crawl result, if successful. |
| `Error` | `*string` | `nil` | The error message, if the crawl failed. |


---

#### BatchScrapeResult

Result from a single URL in a batch scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | The URL that was scraped. |
| `Result` | `*ScrapeResult` | `nil` | The scrape result, if successful. |
| `Error` | `*string` | `nil` | The error message, if the scrape failed. |


---

#### BrowserConfig

Browser fallback configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Mode` | `BrowserMode` | `BrowserMode.Auto` | When to use the headless browser fallback. |
| `Endpoint` | `*string` | `nil` | CDP WebSocket endpoint for connecting to an external browser instance. |
| `Timeout` | `time.Duration` | `30000ms` | Timeout for browser page load and rendering (in milliseconds when serialized). |
| `Wait` | `BrowserWait` | `BrowserWait.NetworkIdle` | Wait strategy after browser navigation. |
| `WaitSelector` | `*string` | `nil` | CSS selector to wait for when `wait` is `Selector`. |
| `ExtraWait` | `*time.Duration` | `nil` | Extra time to wait after the wait condition is met. |

##### Methods

###### Default()

**Signature:**

```go
func (o *BrowserConfig) Default() BrowserConfig
```


---

#### CitationReference

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Index` | `int` | — | Index |
| `Url` | `string` | — | Url |
| `Text` | `string` | — | Text |


---

#### CitationResult

Result of citation conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | Markdown with links replaced by numbered citations. |
| `References` | `[]CitationReference` | `nil` | Numbered reference list: (index, url, text). |


---

#### ContentConfig

Content extraction and conversion configuration.

Controls how HTML is converted to the output format. Uses
html-to-markdown-rs as the conversion engine for all formats
(markdown, plain text, djot).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `OutputFormat` | `string` | `"markdown"` | Output format: `"markdown"` (default), `"plain"`, `"djot"`. |
| `PreprocessingPreset` | `string` | `"standard"` | Preprocessing aggressiveness: `"minimal"`, `"standard"` (default), `"aggressive"`. - Minimal: only scripts/styles removed. - Standard: also removes nav, nav-hinted headers/footers/asides, forms. - Aggressive: removes all footers/asides unconditionally. |
| `RemoveNavigation` | `bool` | `true` | Remove navigation elements (nav, breadcrumbs, menus). Default: `true`. |
| `RemoveForms` | `bool` | `true` | Remove form elements. Default: `true`. |
| `StripTags` | `[]string` | `nil` | HTML tag names to strip (render children only, remove the tag wrapper). Default: `["noscript"]`. |
| `PreserveTags` | `[]string` | `nil` | HTML tag names to preserve as raw HTML in output. |
| `ExcludeSelectors` | `[]string` | `nil` | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the wrapper but keeps children), excluded elements and all descendants are dropped. Supports CSS selectors: `.class`, `#id`, `[attribute]`, compound selectors. Example: `[".cookie-banner", "#ad-container", "[role='complementary']"]` |
| `SkipImages` | `bool` | `false` | Skip image elements in output. Default: `false`. |
| `MaxDepth` | `*int` | `nil` | Max DOM traversal depth. Prevents stack overflow on deeply nested HTML. |
| `Wrap` | `bool` | `false` | Enable line wrapping. Default: `false`. |
| `WrapWidth` | `int` | `80` | Wrap width when `wrap` is enabled. Default: `80`. |
| `IncludeDocumentStructure` | `bool` | `true` | Include document structure tree in output. Default: `true`. |

##### Methods

###### Default()

**Signature:**

```go
func (o *ContentConfig) Default() ContentConfig
```


---

#### CookieInfo

Information about an HTTP cookie received from a response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The cookie name. |
| `Value` | `string` | — | The cookie value. |
| `Domain` | `*string` | `nil` | The cookie domain, if specified. |
| `Path` | `*string` | `nil` | The cookie path, if specified. |


---

#### CrawlConfig

Configuration for crawl, scrape, and map operations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `MaxDepth` | `*int` | `nil` | Maximum crawl depth (number of link hops from the start URL). |
| `MaxPages` | `*int` | `nil` | Maximum number of pages to crawl. |
| `MaxConcurrent` | `*int` | `nil` | Maximum number of concurrent requests. |
| `RespectRobotsTxt` | `bool` | `false` | Whether to respect robots.txt directives. |
| `UserAgent` | `*string` | `nil` | Custom user-agent string. |
| `StayOnDomain` | `bool` | `false` | Whether to restrict crawling to the same domain. |
| `AllowSubdomains` | `bool` | `false` | Whether to allow subdomains when `stay_on_domain` is true. |
| `IncludePaths` | `[]string` | `nil` | Regex patterns for paths to include during crawling. |
| `ExcludePaths` | `[]string` | `nil` | Regex patterns for paths to exclude during crawling. |
| `CustomHeaders` | `map[string]string` | `nil` | Custom HTTP headers to send with each request. |
| `RequestTimeout` | `time.Duration` | `30000ms` | Timeout for individual HTTP requests (in milliseconds when serialized). |
| `RateLimitMs` | `*uint64` | `nil` | Per-domain rate limit in milliseconds. When set, enforces a minimum delay between requests to the same domain. Defaults to 200ms when `nil`. |
| `MaxRedirects` | `int` | `10` | Maximum number of redirects to follow. |
| `RetryCount` | `int` | `0` | Number of retry attempts for failed requests. |
| `RetryCodes` | `[]uint16` | `nil` | HTTP status codes that should trigger a retry. |
| `CookiesEnabled` | `bool` | `false` | Whether to enable cookie handling. |
| `Auth` | `*AuthConfig` | `nil` | Authentication configuration. |
| `MaxBodySize` | `*int` | `nil` | Maximum response body size in bytes. |
| `RemoveTags` | `[]string` | `nil` | CSS selectors for tags to remove from HTML before processing. |
| `Content` | `ContentConfig` | — | Content extraction and conversion configuration. |
| `MapLimit` | `*int` | `nil` | Maximum number of URLs to return from a map operation. |
| `MapSearch` | `*string` | `nil` | Search filter for map results (case-insensitive substring match on URLs). |
| `DownloadAssets` | `bool` | `false` | Whether to download assets (CSS, JS, images, etc.) from the page. |
| `AssetTypes` | `[]AssetCategory` | `nil` | Filter for asset categories to download. |
| `MaxAssetSize` | `*int` | `nil` | Maximum size in bytes for individual asset downloads. |
| `Browser` | `BrowserConfig` | — | Browser configuration. |
| `Proxy` | `*ProxyConfig` | `nil` | Proxy configuration for HTTP requests. |
| `UserAgents` | `[]string` | `nil` | List of user-agent strings for rotation. If non-empty, overrides `user_agent`. |
| `CaptureScreenshot` | `bool` | `false` | Whether to capture a screenshot when using the browser. |
| `DownloadDocuments` | `bool` | `true` | Whether to download non-HTML documents (PDF, DOCX, images, code, etc.) instead of skipping them. |
| `DocumentMaxSize` | `*int` | `nil` | Maximum size in bytes for document downloads. Defaults to 50 MB. |
| `DocumentMimeTypes` | `[]string` | `nil` | Allowlist of MIME types to download. If empty, uses built-in defaults. |
| `WarcOutput` | `*string` | `nil` | Path to write WARC output. If `nil`, WARC output is disabled. |
| `BrowserProfile` | `*string` | `nil` | Named browser profile for persistent sessions (cookies, localStorage). |
| `SaveBrowserProfile` | `bool` | `false` | Whether to save changes back to the browser profile on exit. |

##### Methods

###### Default()

**Signature:**

```go
func (o *CrawlConfig) Default() CrawlConfig
```

###### Validate()

Validate the configuration, returning an error if any values are invalid.

**Signature:**

```go
func (o *CrawlConfig) Validate() error
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
| `StatusCode` | `uint16` | — | The HTTP status code of the response. |
| `ContentType` | `string` | — | The Content-Type header value. |
| `Html` | `string` | — | The HTML body of the response. |
| `BodySize` | `int` | — | The size of the response body in bytes. |
| `Metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `Links` | `[]LinkInfo` | `nil` | Links found on the page. |
| `Images` | `[]ImageInfo` | `nil` | Images found on the page. |
| `Feeds` | `[]FeedInfo` | `nil` | Feed links found on the page. |
| `JsonLd` | `[]JsonLdEntry` | `nil` | JSON-LD entries found on the page. |
| `Depth` | `int` | — | The depth of this page from the start URL. |
| `StayedOnDomain` | `bool` | — | Whether this page is on the same domain as the start URL. |
| `WasSkipped` | `bool` | — | Whether this page was skipped (binary or PDF content). |
| `IsPdf` | `bool` | — | Whether the content is a PDF. |
| `DetectedCharset` | `*string` | `nil` | The detected character set encoding. |
| `Markdown` | `*MarkdownResult` | `nil` | Markdown conversion of the page content. |
| `ExtractedData` | `*interface{}` | `nil` | Structured data extracted by LLM. Populated when extraction is configured. |
| `ExtractionMeta` | `*ExtractionMeta` | `nil` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `DownloadedDocument` | `*DownloadedDocument` | `nil` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |


---

#### CrawlResult

The result of a multi-page crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Pages` | `[]CrawlPageResult` | `nil` | The list of crawled pages. |
| `FinalUrl` | `string` | — | The final URL after following redirects. |
| `RedirectCount` | `int` | — | The number of redirects followed. |
| `WasSkipped` | `bool` | — | Whether any page was skipped during crawling. |
| `Error` | `*string` | `nil` | An error message, if the crawl encountered an issue. |
| `Cookies` | `[]CookieInfo` | `nil` | Cookies collected during the crawl. |
| `NormalizedUrls` | `[]string` | `nil` | Normalized URLs encountered during crawling (for deduplication counting). |

##### Methods

###### UniqueNormalizedUrls()

Returns the count of unique normalized URLs encountered during crawling.

**Signature:**

```go
func (o *CrawlResult) UniqueNormalizedUrls() int
```


---

#### DownloadedAsset

A downloaded asset from a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | The original URL of the asset. |
| `ContentHash` | `string` | — | The SHA-256 content hash of the asset. |
| `MimeType` | `*string` | `nil` | The MIME type from the Content-Type header. |
| `Size` | `int` | — | The size of the asset in bytes. |
| `AssetCategory` | `AssetCategory` | `AssetCategory.Image` | The category of the asset. |
| `HtmlTag` | `*string` | `nil` | The HTML tag that referenced this asset (e.g., "link", "script", "img"). |


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
| `Content` | `[]byte` | — | Raw document bytes. Skipped during JSON serialization. |
| `Size` | `int` | — | Size of the document in bytes. |
| `Filename` | `*string` | `nil` | Filename extracted from Content-Disposition or URL path. |
| `ContentHash` | `string` | — | SHA-256 hex digest of the content. |
| `Headers` | `map[string]string` | `nil` | Selected response headers. |


---

#### ExtractionMeta

Metadata about an LLM extraction pass.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Cost` | `*float64` | `nil` | Estimated cost of the LLM call in USD. |
| `PromptTokens` | `*uint64` | `nil` | Number of prompt (input) tokens consumed. |
| `CompletionTokens` | `*uint64` | `nil` | Number of completion (output) tokens generated. |
| `Model` | `*string` | `nil` | The model identifier used for extraction. |
| `ChunksProcessed` | `int` | — | Number of content chunks sent to the LLM. |


---

#### FaviconInfo

Information about a favicon or icon link.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | The icon URL. |
| `Rel` | `string` | — | The `rel` attribute (e.g., "icon", "apple-touch-icon"). |
| `Sizes` | `*string` | `nil` | The `sizes` attribute, if present. |
| `MimeType` | `*string` | `nil` | The MIME type, if present. |


---

#### FeedInfo

Information about a feed link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | The feed URL. |
| `Title` | `*string` | `nil` | The feed title, if present. |
| `FeedType` | `FeedType` | `FeedType.Rss` | The type of feed. |


---

#### HeadingInfo

A heading element extracted from the page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Level` | `uint8` | — | The heading level (1-6). |
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
| `Alt` | `*string` | `nil` | The alt text, if present. |
| `Width` | `*uint32` | `nil` | The width attribute, if present and parseable. |
| `Height` | `*uint32` | `nil` | The height attribute, if present and parseable. |
| `Source` | `ImageSource` | `ImageSource.Img` | The source of the image reference. |


---

#### JsonLdEntry

A JSON-LD structured data entry found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `SchemaType` | `string` | — | The `@type` value from the JSON-LD object. |
| `Name` | `*string` | `nil` | The `name` value, if present. |
| `Raw` | `string` | — | The raw JSON-LD string. |


---

#### LinkInfo

Information about a link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | The resolved URL of the link. |
| `Text` | `string` | — | The visible text of the link. |
| `LinkType` | `LinkType` | `LinkType.Internal` | The classification of the link. |
| `Rel` | `*string` | `nil` | The `rel` attribute value, if present. |
| `Nofollow` | `bool` | — | Whether the link has `rel="nofollow"`. |


---

#### MapResult

The result of a map operation, containing discovered URLs.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Urls` | `[]SitemapUrl` | `nil` | The list of discovered URLs. |


---

#### MarkdownResult

Rich markdown conversion result from HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | Converted markdown text. |
| `DocumentStructure` | `*interface{}` | `nil` | Structured document tree with semantic nodes. |
| `Tables` | `[]interface{}` | `nil` | Extracted tables with structured cell data. |
| `Warnings` | `[]string` | `nil` | Non-fatal processing warnings. |
| `Citations` | `*CitationResult` | `nil` | Content with links replaced by numbered citations. |
| `FitContent` | `*string` | `nil` | Content-filtered markdown optimized for LLM consumption. |


---

#### PageMetadata

Metadata extracted from an HTML page's `<meta>` tags and `<title>` element.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Title` | `*string` | `nil` | The page title from the `<title>` element. |
| `Description` | `*string` | `nil` | The meta description. |
| `CanonicalUrl` | `*string` | `nil` | The canonical URL from `<link rel="canonical">`. |
| `Keywords` | `*string` | `nil` | Keywords from `<meta name="keywords">`. |
| `Author` | `*string` | `nil` | Author from `<meta name="author">`. |
| `Viewport` | `*string` | `nil` | Viewport content from `<meta name="viewport">`. |
| `ThemeColor` | `*string` | `nil` | Theme color from `<meta name="theme-color">`. |
| `Generator` | `*string` | `nil` | Generator from `<meta name="generator">`. |
| `Robots` | `*string` | `nil` | Robots content from `<meta name="robots">`. |
| `HtmlLang` | `*string` | `nil` | The `lang` attribute from the `<html>` element. |
| `HtmlDir` | `*string` | `nil` | The `dir` attribute from the `<html>` element. |
| `OgTitle` | `*string` | `nil` | Open Graph title. |
| `OgType` | `*string` | `nil` | Open Graph type. |
| `OgImage` | `*string` | `nil` | Open Graph image URL. |
| `OgDescription` | `*string` | `nil` | Open Graph description. |
| `OgUrl` | `*string` | `nil` | Open Graph URL. |
| `OgSiteName` | `*string` | `nil` | Open Graph site name. |
| `OgLocale` | `*string` | `nil` | Open Graph locale. |
| `OgVideo` | `*string` | `nil` | Open Graph video URL. |
| `OgAudio` | `*string` | `nil` | Open Graph audio URL. |
| `OgLocaleAlternates` | `*[]string` | `nil` | Open Graph locale alternates. |
| `TwitterCard` | `*string` | `nil` | Twitter card type. |
| `TwitterTitle` | `*string` | `nil` | Twitter title. |
| `TwitterDescription` | `*string` | `nil` | Twitter description. |
| `TwitterImage` | `*string` | `nil` | Twitter image URL. |
| `TwitterSite` | `*string` | `nil` | Twitter site handle. |
| `TwitterCreator` | `*string` | `nil` | Twitter creator handle. |
| `DcTitle` | `*string` | `nil` | Dublin Core title. |
| `DcCreator` | `*string` | `nil` | Dublin Core creator. |
| `DcSubject` | `*string` | `nil` | Dublin Core subject. |
| `DcDescription` | `*string` | `nil` | Dublin Core description. |
| `DcPublisher` | `*string` | `nil` | Dublin Core publisher. |
| `DcDate` | `*string` | `nil` | Dublin Core date. |
| `DcType` | `*string` | `nil` | Dublin Core type. |
| `DcFormat` | `*string` | `nil` | Dublin Core format. |
| `DcIdentifier` | `*string` | `nil` | Dublin Core identifier. |
| `DcLanguage` | `*string` | `nil` | Dublin Core language. |
| `DcRights` | `*string` | `nil` | Dublin Core rights. |
| `Article` | `*ArticleMetadata` | `nil` | Article metadata from `article:*` Open Graph tags. |
| `Hreflangs` | `*[]HreflangEntry` | `nil` | Hreflang alternate links. |
| `Favicons` | `*[]FaviconInfo` | `nil` | Favicon and icon links. |
| `Headings` | `*[]HeadingInfo` | `nil` | Heading elements (h1-h6). |
| `WordCount` | `*int` | `nil` | Computed word count of the page body text. |


---

#### ProxyConfig

Proxy configuration for HTTP requests.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | Proxy URL (e.g. "<http://proxy:8080",> "socks5://proxy:1080"). |
| `Username` | `*string` | `nil` | Optional username for proxy authentication. |
| `Password` | `*string` | `nil` | Optional password for proxy authentication. |


---

#### ResponseMeta

Response metadata extracted from HTTP headers.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Etag` | `*string` | `nil` | The ETag header value. |
| `LastModified` | `*string` | `nil` | The Last-Modified header value. |
| `CacheControl` | `*string` | `nil` | The Cache-Control header value. |
| `Server` | `*string` | `nil` | The Server header value. |
| `XPoweredBy` | `*string` | `nil` | The X-Powered-By header value. |
| `ContentLanguage` | `*string` | `nil` | The Content-Language header value. |
| `ContentEncoding` | `*string` | `nil` | The Content-Encoding header value. |


---

#### ScrapeResult

The result of a single-page scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `StatusCode` | `uint16` | — | The HTTP status code of the response. |
| `ContentType` | `string` | — | The Content-Type header value. |
| `Html` | `string` | — | The HTML body of the response. |
| `BodySize` | `int` | — | The size of the response body in bytes. |
| `Metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `Links` | `[]LinkInfo` | `nil` | Links found on the page. |
| `Images` | `[]ImageInfo` | `nil` | Images found on the page. |
| `Feeds` | `[]FeedInfo` | `nil` | Feed links found on the page. |
| `JsonLd` | `[]JsonLdEntry` | `nil` | JSON-LD entries found on the page. |
| `IsAllowed` | `bool` | — | Whether the URL is allowed by robots.txt. |
| `CrawlDelay` | `*uint64` | `nil` | The crawl delay from robots.txt, in seconds. |
| `NoindexDetected` | `bool` | — | Whether a noindex directive was detected. |
| `NofollowDetected` | `bool` | — | Whether a nofollow directive was detected. |
| `XRobotsTag` | `*string` | `nil` | The X-Robots-Tag header value, if present. |
| `IsPdf` | `bool` | — | Whether the content is a PDF. |
| `WasSkipped` | `bool` | — | Whether the page was skipped (binary or PDF content). |
| `DetectedCharset` | `*string` | `nil` | The detected character set encoding. |
| `AuthHeaderSent` | `bool` | — | Whether an authentication header was sent with the request. |
| `ResponseMeta` | `*ResponseMeta` | `nil` | Response metadata extracted from HTTP headers. |
| `Assets` | `[]DownloadedAsset` | `nil` | Downloaded assets from the page. |
| `JsRenderHint` | `bool` | — | Whether the page content suggests JavaScript rendering is needed. |
| `BrowserUsed` | `bool` | — | Whether the browser fallback was used to fetch this page. |
| `Markdown` | `*MarkdownResult` | `nil` | Markdown conversion of the page content. |
| `ExtractedData` | `*interface{}` | `nil` | Structured data extracted by LLM. Populated when extraction is configured. |
| `ExtractionMeta` | `*ExtractionMeta` | `nil` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `Screenshot` | `*[]byte` | `nil` | Screenshot of the page as PNG bytes. Populated when browser is used and capture_screenshot is enabled. |
| `DownloadedDocument` | `*DownloadedDocument` | `nil` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |


---

#### SitemapUrl

A URL entry from a sitemap.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | The URL. |
| `Lastmod` | `*string` | `nil` | The last modification date, if present. |
| `Changefreq` | `*string` | `nil` | The change frequency, if present. |
| `Priority` | `*string` | `nil` | The priority, if present. |


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
