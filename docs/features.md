---
title: Features
description: Feature breakdown for Kreuzcrawl 0.3
---

## Features

Kreuzcrawl is a Rust-native web crawling engine. Every feature below is wired through the public surface of the Rust crate (`pub use` from the crate root) or the equivalent surface in the native bindings, unless explicitly marked as an internal preview.

Native bindings ship for **14 languages** — Rust, Python, TypeScript (Node + WebAssembly), Go, Java, Kotlin, C#, Ruby, PHP, Elixir, Dart, Swift, Zig — plus a stable C FFI surface for everything else.

The public Rust surface from `kreuzcrawl::*` is six free functions over an opaque `CrawlEngineHandle`: `create_engine`, `scrape`, `crawl`, `map_urls`, `batch_scrape`, `batch_crawl`. The same six operations are exposed by every binding, plus `serve_api` and `start_mcp_server` under the `api` and `mcp` cargo features.

---

### Core Crawling

| Feature                 | Description                                                                                               |
| ----------------------- | --------------------------------------------------------------------------------------------------------- |
| **Engine construction** | `create_engine(config)` — opaque `CrawlEngineHandle`; all configuration validated up front via `serde`.   |
| **Concurrent fetching** | Tokio `JoinSet` + `Semaphore` for parallel requests, bounded by `CrawlConfig::max_concurrent`.            |
| **Sequential crawl**    | `crawl(&engine, url)` follows links from a seed up to `max_depth` / `max_pages`.                          |
| **Batch operations**    | `batch_crawl(&engine, urls)` and `batch_scrape(&engine, urls)` for multi-seed processing.                 |
| **URL discovery**       | `map_urls(&engine, url)` returns a `MapResult` from sitemap parsing and link extraction.                  |
| **Redirect handling**   | HTTP 3xx, `Refresh` header, and meta-refresh detection with loop detection (`max_redirects`, default 10). |

Four traversal strategies — BFS (default), DFS, BestFirst, and Adaptive — are implemented internally; the public surface always uses BFS. A configuration knob for strategy selection is on the roadmap.

---

### Metadata Extraction

Every scraped page yields a `PageMetadata` struct alongside separate collections for links, images, feeds, and structured data.

| Feature               | Description                                                              |
| --------------------- | ------------------------------------------------------------------------ |
| **Open Graph**        | `og:title`, `og:description`, `og:image`, `og:type`, `og:url`, and more. |
| **Twitter Card**      | `twitter:card`, `twitter:title`, `twitter:description`, `twitter:image`. |
| **Dublin Core**       | `dc.title`, `dc.creator`, `dc.date`, `dc.subject`.                       |
| **Article metadata**  | `article:published_time`, `article:author`, `article:section`.           |
| **JSON-LD**           | Full extraction from `<script type="application/ld+json">` blocks.       |
| **Link extraction**   | 4 categories: `Internal`, `External`, `Anchor`, `Document`.              |
| **Image extraction**  | `<img>`, `<picture>`, `og:image`, and `srcset` sources.                  |
| **Feed discovery**    | RSS, Atom, and JSON Feed `<link>` elements.                              |
| **Favicons**          | Extraction and canonicalisation of site icons.                           |
| **hreflang**          | Language and region variants for internationalised pages.                |
| **Headings**          | H1–H6 with hierarchy preservation.                                       |
| **Response metadata** | HTTP headers, content type, charset detection, body size.                |

---

### Markdown Conversion

HTML→Markdown conversion runs automatically on every page via [html-to-markdown](https://docs.html-to-markdown.kreuzberg.dev). Results land in the `MarkdownResult` struct attached to every page.

| Feature                  | Description                                                                                          |
| ------------------------ | ---------------------------------------------------------------------------------------------------- |
| **Always-on conversion** | Every page result includes a `markdown` field with converted content.                                |
| **Document structure**   | Optional structured tree of semantic nodes alongside the rendered Markdown.                          |
| **Table extraction**     | Structured table data preserved alongside Markdown output.                                           |
| **Link-to-citations**    | Numbered references (`[1]`, `[2]`) with a `CitationResult` carrying every reference.                 |
| **Fit Markdown**         | Heuristic-based pruning and truncation optimised for LLM consumption (`MarkdownResult.fit_content`). |
| **Warnings**             | Non-fatal processing warnings surfaced in `MarkdownResult.warnings`.                                 |

---

### Browser Fallback

!!! info "Feature gate"
Requires the `browser` feature: `kreuzcrawl = { version = "0.3", features = ["browser"] }`

| Feature                 | Description                                                                                                                                 |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| **Headless Chrome**     | chromiumoxide-driven Chrome with `BrowserMode::Auto` / `Always` / `Never`.                                                                  |
| **WAF detection**       | 8 vendors auto-detected on HTTP and browser responses: Cloudflare, Akamai, AWS WAF, Imperva, DataDome, PerimeterX, Sucuri, F5.              |
| **Auto fallback**       | In `Auto` mode, WAF-blocked or JS-render-required responses retry through Chrome with a legitimate browser fingerprint.                     |
| **Wait strategies**     | `NetworkIdle` (default), `Selector` (wait for CSS selector), `Fixed` (fixed duration); plus optional `extra_wait` after the wait condition. |
| **CDP endpoint**        | Point at an already-running browser via `BrowserConfig::endpoint` instead of launching one locally.                                         |
| **Persistent profiles** | Named profiles via `CrawlConfig::browser_profile` / `save_browser_profile`. Profile names validated against path-traversal.                 |
| **Screenshot capture**  | PNG screenshot captured when `capture_screenshot` is enabled and the browser is used.                                                       |
| **JS-render detection** | SPA-shell and noscript-warning heuristics flag pages that need browser rendering.                                                           |

---

### Network and Caching

| Feature                      | Description                                                                  |
| ---------------------------- | ---------------------------------------------------------------------------- |
| **Per-domain rate limiting** | Default 200 ms delay per origin; configurable in `CrawlConfig`.              |
| **HTTP caching**             | ETag and Last-Modified conditional requests with an on-disk cache.           |
| **Proxy support**            | HTTP, HTTPS, and SOCKS5 via `ProxyConfig`.                                   |
| **User-Agent rotation**      | Configurable list rotated across requests.                                   |
| **Cookie handling**          | Tracking, deduplication, and persistence across requests.                    |
| **Authentication**           | Basic, Bearer, and custom-header authentication via `AuthConfig`.            |
| **Timeouts**                 | Per-request timeout (default 30 s); `max_redirects` default 10 (cap 100).    |
| **Retry logic**              | Configurable retry count with explicit status-code triggers (`retry_codes`). |
| **Body-size limits**         | Optional `CrawlConfig::max_body_size` to cap response payloads.              |

---

### Content Processing

| Feature                   | Description                                                                                                            |
| ------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| **Preprocessing presets** | `content.preprocessing_preset` accepts `"minimal"`, `"standard"` (default), or `"aggressive"` (boilerplate-stripping). |
| **Tag removal**           | `remove_tags` takes CSS selectors stripped before extraction.                                                          |
| **Path filtering**        | `include_paths` and `exclude_paths` accept regex patterns; excludes take priority.                                     |
| **Domain scoping**        | `stay_on_domain` with optional `allow_subdomains`.                                                                     |

---

### Document Downloads

| Feature                 | Description                                                                                |
| ----------------------- | ------------------------------------------------------------------------------------------ |
| **Non-HTML documents**  | Download PDFs, DOCX, images, and code files via `download_documents` (enabled by default). |
| **Asset downloads**     | CSS, JS, images via `download_assets` with `asset_types` category filtering.               |
| **Size limits**         | `document_max_size` (default 50 MB) and `max_asset_size` caps.                             |
| **MIME filtering**      | `document_mime_types` allowlist for permitted document types.                              |
| **Content hashing**     | SHA-256 digest computed for every downloaded document.                                     |
| **Filename extraction** | Parsed from `Content-Disposition` or the URL path.                                         |

---

### Compliance and Standards

| Feature                | Description                                                                   |
| ---------------------- | ----------------------------------------------------------------------------- |
| **robots.txt**         | RFC 9309 compliant with user-agent prefix matching and `Crawl-delay` support. |
| **Sitemap parsing**    | XML, gzip-compressed, and sitemap-index files.                                |
| **noindex / nofollow** | Detection of `<meta>` robots directives and `X-Robots-Tag` headers.           |
| **Charset detection**  | Automatic from HTTP headers and HTML meta tags.                               |
| **Config validation**  | `serde` with `deny_unknown_fields` — typos in config keys fail at parse time. |

---

### WARC Output

!!! info "Feature gate"
Requires the `warc` feature: `kreuzcrawl = { version = "0.3", features = ["warc"] }`

| Feature           | Description                                                                                          |
| ----------------- | ---------------------------------------------------------------------------------------------------- |
| **WARC 1.1**      | Standards-compliant `warcinfo` + per-page `response` records, written to `CrawlConfig::warc_output`. |
| **Header safety** | Header names and values validated against CR/LF injection before being written.                      |

---

### REST API, MCP, and CLI

| Feature        | Description                                                                                                                                                         |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **REST API**   | `serve_api(config).await` starts an Axum-based, [Firecrawl v1-compatible](reference/rest-api.md) HTTP API (feature `api`). OpenAPI schema is generated by `utoipa`. |
| **MCP server** | `start_mcp_server(...)` starts a Model Context Protocol server for AI-agent integration (feature `mcp`).                                                            |
| **CLI**        | The `kreuzcrawl` binary exposes `scrape`, `crawl`, `map`, and `serve` subcommands.                                                                                  |

---

### CLI quickstart

```bash
# Scrape a single page
kreuzcrawl scrape https://example.com

# Crawl with depth limiting
kreuzcrawl crawl https://example.com --depth 2 --max-pages 50 --format markdown

# Discover URLs via sitemap and link extraction
kreuzcrawl map https://example.com --respect-robots-txt
```

Output formats: `json` (full `CrawlResult` / `ScrapeResult` / `MapResult`) and `markdown` (`MarkdownResult` content with citations).
