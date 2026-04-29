---
title: Features
description: Comprehensive feature breakdown for Kreuzcrawl
---

## Features

Kreuzcrawl is a Rust-native web crawling engine with deep extraction capabilities. This page covers every major feature area and includes a competitive comparison with other tools in the space.

---

## Core Crawling

The `CrawlEngine` is built with the builder pattern and validates all configuration at construction time. Invalid configs fail fast before any network requests are made.

| Feature | Description |
|---------|-------------|
| **CrawlEngine builder** | Fluent `.builder().config(...).build()` pattern with strict `serde` validation |
| **Concurrent fetching** | `JoinSet` + `Semaphore` for parallel requests (default: 10 concurrent) |
| **Multiple strategies** | BFS, DFS, BestFirst, and Adaptive traversal via the `CrawlStrategy` trait |
| **Batch crawling** | Multi-seed `batch_crawl()` and `batch_scrape()` for processing URL lists |
| **Streaming events** | Real-time `crawl_stream()` returning `CrawlEvent` items as pages are processed |
| **URL discovery** | Sitemap parsing (XML, gzip, sitemap index) combined with link extraction |
| **Redirect handling** | HTTP 3xx, Refresh header, and meta refresh detection with loop detection |

---

## Metadata Extraction

Every scraped page yields a `PageMetadata` struct with 40+ fields, plus separate collections for links, images, feeds, and structured data.

| Feature | Description |
|---------|-------------|
| **Open Graph** | `og:title`, `og:description`, `og:image`, `og:type`, `og:url`, and more |
| **Twitter Card** | `twitter:card`, `twitter:title`, `twitter:description`, `twitter:image` |
| **Dublin Core** | `dc.title`, `dc.creator`, `dc.date`, `dc.subject` |
| **Article metadata** | `article:published_time`, `article:author`, `article:section` |
| **JSON-LD** | Full JSON-LD extraction from `<script type="application/ld+json">` blocks |
| **Link extraction** | 4 link types: `Internal`, `External`, `Anchor`, `Document` |
| **Image extraction** | All sources: `<img>`, `<picture>`, `og:image`, `srcset` |
| **Feed discovery** | RSS, Atom, and JSON Feed detection from `<link>` elements |
| **Favicons** | Extraction and canonicalization of site icons |
| **hreflang** | Language and region variant links for internationalized pages |
| **Headings** | H1-H6 extraction with hierarchy preservation |
| **Response metadata** | HTTP headers, content type, charset detection, body size |

---

## Markdown Conversion

HTML-to-markdown conversion runs automatically on every page via `html-to-markdown-rs`. Results are available in the `MarkdownResult` struct.

| Feature | Description |
|---------|-------------|
| **Always-on conversion** | Every page includes a `markdown` field with converted content |
| **Document structure** | Optional structured document tree with semantic nodes |
| **Table extraction** | Structured table data preserved alongside markdown output |
| **Link-to-citations** | Numbered reference conversion (e.g., `[1]`, `[2]`) with a `CitationResult` containing all references |
| **Fit markdown** | Content pruning and heuristic-based truncation optimized for LLM consumption |
| **Warnings** | Non-fatal processing warnings surfaced in `MarkdownResult.warnings` |

---

## AI and LLM Integration

!!! info "Feature gate"
    Requires the `ai` feature: `kreuzcrawl = { version = "0.3", features = ["ai"] }`

| Feature | Description |
|---------|-------------|
| **LlmExtractor** | Multi-provider LLM extraction powered by `liter-llm` |
| **JSON schema extraction** | Pass a JSON schema and receive structured data matching it |
| **Cost tracking** | `ExtractionMeta` includes estimated USD cost, prompt tokens, and completion tokens |
| **Model metadata** | The model identifier used for each extraction is recorded |
| **Chunk tracking** | Number of content chunks processed by the LLM |

---

## Anti-Bot and Browser Automation

!!! info "Feature gate"
    Requires the `browser` feature: `kreuzcrawl = { version = "0.3", features = ["browser"] }`

| Feature | Description |
|---------|-------------|
| **WAF detection** | 8 vendors: Cloudflare, Akamai, AWS WAF, Imperva, DataDome, PerimeterX, Sucuri, F5 |
| **Browser fallback** | Headless Chrome via chromiumoxide with configurable `BrowserMode` (`Auto`, `Always`, `Never`) |
| **BrowserPool** | Multi-browser management with health checks and crash recovery |
| **Browser wait strategies** | `NetworkIdle`, `Selector` (wait for CSS selector), and `Fixed` duration |
| **Browser profiles** | Named persistent sessions preserving cookies and localStorage |
| **JS rendering detection** | Heuristic-based detection of pages requiring JavaScript rendering |
| **Screenshot capture** | PNG screenshot capture when using the browser (via `capture_screenshot`) |
| **User-Agent rotation** | `UaRotationLayer` Tower middleware for UA header diversity |

---

## Network and Caching

| Feature | Description |
|---------|-------------|
| **Per-domain rate limiting** | `PerDomainRateLimitLayer` Tower middleware with configurable delays (default: 200ms) |
| **HTTP caching** | ETag and Last-Modified conditional requests via `CrawlCacheLayer` |
| **Disk cache** | blake3-hashed file storage with TTL and automatic eviction |
| **Proxy support** | HTTP, HTTPS, and SOCKS5 proxies via `ProxyConfig` |
| **User-Agent rotation** | Configurable list of UA strings rotated across requests |
| **Cookie handling** | Cookie tracking with deduplication and persistence |
| **Authentication** | Basic, Bearer, and custom header authentication via `AuthConfig` |
| **Configurable timeouts** | Per-request timeout (default: 30s), max redirects (default: 10, max: 100) |
| **Retry logic** | Configurable retry count with specific HTTP status code triggers |
| **Body size limits** | Optional `max_body_size` to cap response payloads |

---

## Content Filtering and Relevance

| Feature | Description |
|---------|-------------|
| **BM25 scoring** | `Bm25Filter` for adaptive relevance evaluation of crawled pages |
| **Adaptive crawling** | `AdaptiveStrategy` with term saturation detection for early termination |
| **Main content extraction** | `main_content_only` strips boilerplate, leaving primary page content |
| **Tag removal** | `remove_tags` accepts CSS selectors for elements to strip before processing |
| **Path filtering** | `include_paths` and `exclude_paths` with regex pattern matching |
| **Domain scoping** | `stay_on_domain` with optional `allow_subdomains` |

---

## Document Downloads

| Feature | Description |
|---------|-------------|
| **Non-HTML documents** | Download PDFs, DOCX, images, code files via `download_documents` (enabled by default) |
| **Asset downloads** | CSS, JS, images via `download_assets` with category filtering |
| **Size limits** | `document_max_size` (default: 50 MB) and `max_asset_size` caps |
| **MIME filtering** | `document_mime_types` allowlist for controlling which document types to download |
| **Content hashing** | SHA-256 digest computed for every downloaded document |
| **Filename extraction** | Parsed from Content-Disposition headers or URL path |

---

## Compliance and Standards

| Feature | Description |
|---------|-------------|
| **robots.txt** | RFC 9309 compliant with user-agent prefix matching and crawl-delay support |
| **Sitemap parsing** | XML, gzip-compressed, and sitemap index file support |
| **noindex / nofollow** | Detection of `<meta>` robots directives and `X-Robots-Tag` headers |
| **Charset detection** | Automatic encoding detection from HTTP headers and HTML meta tags |
| **Binary/PDF skipping** | Content-type aware filtering to avoid processing non-HTML content |
| **Config validation** | `serde` with `deny_unknown_fields` -- typos in config keys are compile-time or parse-time errors |

---

## WARC Output

!!! info "Feature gate"
    Requires the `warc` feature: `kreuzcrawl = { version = "0.3", features = ["warc"] }`

| Feature | Description |
|---------|-------------|
| **WARC output** | Standards-compliant WARC archiving for entire crawl sessions |
| **Archive format** | Web ARChive (WARC) format with complete HTTP request/response pairs |
| **File storage** | Write to disk via `warc_output` configuration path |

---

## MCP and REST API

!!! info "Feature gates"
    MCP server: `features = ["mcp"]` -- REST API: `features = ["api"]`

| Feature | Description |
|---------|-------------|
| **MCP server** | Model Context Protocol server for AI agent integration |
| **REST API** | Axum-based HTTP API with OpenAPI documentation via `utoipa` |
| **Page interaction** | Execute action sequences on browser pages (feature-gated: `interact`) |

---

## Extensibility

Kreuzcrawl exposes **7 pluggable traits** that let you replace any component of the crawl pipeline:

| Trait | Purpose | Default Implementation |
|-------|---------|----------------------|
| `Frontier` | URL queue and deduplication | `InMemoryFrontier` (VecDeque + HashSet) |
| `RateLimiter` | Per-domain request throttling | `PerDomainThrottle` (200ms delay) |
| `CrawlStore` | Result storage backend | `NoopStore` (results returned, not stored) |
| `EventEmitter` | Lifecycle event callbacks | `NoopEmitter` |
| `CrawlStrategy` | Traversal algorithm and URL scoring | `BfsStrategy` |
| `ContentFilter` | Page relevance evaluation | `NoopFilter` (accept all) |
| `CrawlCache` | HTTP response caching | `NoopCache` |

The Tower service stack composes these traits into a layered pipeline:

```text
CrawlStrategy --> Frontier --> CrawlTracingLayer --> UaRotationLayer
    --> CrawlCacheLayer --> PerDomainRateLimitLayer --> HttpFetchService
```

---

## CLI

The `kreuzcrawl` CLI provides three core commands:

```bash
# Scrape a single page
kreuzcrawl scrape https://example.com

# Crawl with depth limiting
kreuzcrawl crawl https://example.com --depth 2 --max-pages 50 --format markdown

# Discover URLs via sitemap + crawling
kreuzcrawl map https://example.com --respect-robots-txt
```

Output formats: `json` (full `CrawlResult` with all metadata) and `markdown` (`MarkdownResult` with citations).

---

## Competitive Comparison

### Overview

| | kreuzcrawl | spider | firecrawl | crawl4ai | webclaw | ScrapeGraphAI | CRW |
|---|---|---|---|---|---|---|---|
| **Language** | Rust | Rust | TypeScript | Python | Rust | Python | Rust |
| **License** | Elastic-2.0 | MIT | AGPL-3.0 | Apache-2.0 | AGPL-3.0 | MIT | AGPL-3.0 |
| **Distribution** | Library + CLI | Library + CLI + SaaS | SaaS + Self-hosted | Library + CLI + API | Library + CLI + MCP | Library + SaaS API | CLI + MCP + API |
| **Headless browser** | chromiumoxide | chromey / WebDriver | Playwright | Playwright | None (TLS fingerprint) | Playwright | LightPanda / Chrome |

### Crawling

| | kreuzcrawl | spider | firecrawl | crawl4ai | webclaw | ScrapeGraphAI | CRW |
|---|---|---|---|---|---|---|---|
| **Traversal strategies** | BFS, DFS, BestFirst, Adaptive | BFS | BFS | BFS, DFS, BestFirst | BFS | LLM-driven graph | BFS |
| **Concurrent fetching** | JoinSet + Semaphore | Tokio multi-thread + AIMD | Bull queue workers | asyncio browser pool | Tokio | asyncio | Tokio |
| **Streaming events** | Real-time | Subscriber channels | SSE / polling | Yes | -- | -- | -- |
| **Batch operations** | `batch_crawl()` | -- | Async API | Deep crawl | Yes | -- | Yes |
| **Sitemap parsing** | XML, gzip, index | Yes | Yes | -- | Yes | -- | Yes |
| **robots.txt** | RFC 9309 | With caching | Yes | Basic | Yes | -- | Yes |

### Extraction and Content

| | kreuzcrawl | spider | firecrawl | crawl4ai | webclaw | ScrapeGraphAI | CRW |
|---|---|---|---|---|---|---|---|
| **Markdown conversion** | Always-on + structure | Yes | Primary output | Yes | Yes | Yes | Yes |
| **Fit markdown (LLM-pruned)** | BM25 + heuristic | -- | -- | BM25/LLM-based | Token-optimized | -- | -- |
| **Metadata fields** | 40+ (OG, DC, Twitter, Article, JSON-LD) | Basic | Basic | Basic | Moderate | -- | Basic |
| **JSON-LD extraction** | Full | -- | -- | -- | Data islands | -- | -- |
| **Feed discovery** | RSS, Atom, JSON Feed | -- | -- | -- | -- | -- | -- |
| **Link-to-citations** | Numbered refs | -- | -- | Yes | -- | -- | -- |
| **LLM extraction** | Multi-provider (liter-llm) | OpenAI, Gemini | 10+ providers | litellm | Ollama (local) | LangChain (core) | Claude, OpenAI |
| **Cost tracking** | USD + tokens | -- | Yes | Yes | -- | Token counting | -- |

### Architecture and Extensibility

| | kreuzcrawl | spider | firecrawl | crawl4ai | webclaw | ScrapeGraphAI | CRW |
|---|---|---|---|---|---|---|---|
| **Pluggable traits** | 7 traits | -- | -- | Partial (strategies) | -- | Graph nodes | -- |
| **Middleware stack** | Tower services | -- | -- | -- | -- | -- | -- |
| **Config validation** | serde strict | -- | -- | -- | -- | -- | -- |
| **BM25 relevance scoring** | Yes | -- | -- | Yes | -- | -- | -- |
| **Adaptive crawling** | Term saturation | -- | -- | Pattern learning | -- | -- | -- |
| **Asset download + dedup** | SHA-256 | -- | -- | -- | -- | -- | -- |
| **Language SDKs** | 11 languages | Rust, Python, Node.js | Python, JS, Go, Java, Elixir, Rust | Python | Rust | Python, Node.js | Rust |

### License Details

| License | Tools | Commercial use | Hosting restriction |
|---------|-------|---------------|---------------------|
| **Elastic-2.0** | kreuzcrawl | Yes | Cannot provide as managed service |
| **MIT** | spider, ScrapeGraphAI | Yes | None |
| **Apache-2.0** | crawl4ai | Yes | None |
| **AGPL-3.0** | firecrawl, webclaw, CRW | Yes | Must open-source modifications if hosting |
