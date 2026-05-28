# kreuzcrawl

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <a href="https://github.com/kreuzberg-dev/alef">
    <img src="https://img.shields.io/badge/Bindings-alef%20%D7%90-007ec6" alt="Bindings">
  </a>
  <!-- Language Bindings -->
  <a href="https://crates.io/crates/kreuzcrawl">
    <img src="https://img.shields.io/crates/v/kreuzcrawl?label=Rust&color=007ec6" alt="Rust">
  </a>
  <a href="https://pypi.org/project/kreuzcrawl/">
    <img src="https://img.shields.io/pypi/v/kreuzcrawl?label=Python&color=007ec6" alt="Python">
  </a>
  <a href="https://www.npmjs.com/package/@kreuzberg/kreuzcrawl">
    <img src="https://img.shields.io/npm/v/@kreuzberg/kreuzcrawl?label=Node.js&color=007ec6" alt="Node.js">
  </a>
  <a href="https://www.npmjs.com/package/@kreuzberg/kreuzcrawl-wasm">
    <img src="https://img.shields.io/npm/v/@kreuzberg/kreuzcrawl-wasm?label=WASM&color=007ec6" alt="WASM">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.kreuzberg.kreuzcrawl/kreuzcrawl">
    <img src="https://img.shields.io/maven-central/v/dev.kreuzberg.kreuzcrawl/kreuzcrawl?label=Java&color=007ec6" alt="Java">
  </a>
  <a href="https://pkg.go.dev/github.com/kreuzberg-dev/kreuzcrawl/packages/go">
    <img src="https://img.shields.io/github/v/tag/kreuzberg-dev/kreuzcrawl?label=Go&color=007ec6" alt="Go">
  </a>
  <a href="https://www.nuget.org/packages/Kreuzcrawl/">
    <img src="https://img.shields.io/nuget/v/Kreuzcrawl?label=C%23&color=007ec6" alt="C#">
  </a>
  <a href="https://packagist.org/packages/kreuzberg-dev/kreuzcrawl">
    <img src="https://img.shields.io/packagist/v/kreuzberg-dev/kreuzcrawl?label=PHP&color=007ec6" alt="PHP">
  </a>
  <a href="https://rubygems.org/gems/kreuzcrawl">
    <img src="https://img.shields.io/gem/v/kreuzcrawl?label=Ruby&color=007ec6" alt="Ruby">
  </a>
  <a href="https://hex.pm/packages/kreuzcrawl">
    <img src="https://img.shields.io/hexpm/v/kreuzcrawl?label=Elixir&color=007ec6" alt="Elixir">
  </a>
  <a href="https://pub.dev/packages/kreuzcrawl">
    <img src="https://img.shields.io/pub/v/kreuzcrawl?label=Dart&color=007ec6" alt="Dart">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.kreuzberg.kreuzcrawl.android/kreuzcrawl-android">
    <img src="https://img.shields.io/maven-central/v/dev.kreuzberg.kreuzcrawl.android/kreuzcrawl-android?label=Kotlin&color=007ec6" alt="Kotlin">
  </a>
  <a href="https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/swift">
    <img src="https://img.shields.io/badge/Swift-SPM-007ec6" alt="Swift">
  </a>
  <a href="https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/zig">
    <img src="https://img.shields.io/badge/Zig-package-007ec6" alt="Zig">
  </a>
  <a href="https://github.com/kreuzberg-dev/kreuzcrawl/releases">
    <img src="https://img.shields.io/badge/C-FFI-007ec6" alt="C FFI">
  </a>
  <a href="https://github.com/kreuzberg-dev/kreuzcrawl/pkgs/container/kreuzcrawl">
    <img src="https://img.shields.io/badge/Docker-ghcr.io-007ec6?logo=docker&logoColor=white" alt="Docker">
  </a>

  <!-- Project Info -->
  <a href="https://github.com/kreuzberg-dev/kreuzcrawl/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-Elastic--2.0-007ec6" alt="License">
  </a>
  <a href="https://docs.kreuzcrawl.kreuzberg.dev">
    <img src="https://img.shields.io/badge/Docs-kreuzcrawl-007ec6" alt="Documentation">
  </a>
</div>

<div align="center" style="margin: 24px 0 0;">
  <a href="https://kreuzberg.dev">
    <img alt="Kreuzcrawl" src="https://raw.githubusercontent.com/kreuzberg-dev/kreuzcrawl/main/docs/assets/docs_top_banner.svg" />
  </a>
</div>

<div align="center" style="display: flex; flex-wrap: wrap; gap: 12px; justify-content: center; margin: 28px 0 24px;">
  <a href="https://discord.gg/xt9WY3GnKR">
    <img height="22" src="https://img.shields.io/badge/Discord-Chat-007ec6?logo=discord&logoColor=white" alt="Join Discord">
  </a>
</div>

Swift bindings for **kreuzcrawl** — a high-performance Rust web crawling engine.
Via swift-bridge for macOS, iOS, and Linux with native Swift types and async/await.

## What This Package Provides

- **Same crawler as every binding** — one Rust engine behind Python, Node.js, Ruby, Go, Java, .NET, PHP, Elixir, Dart, Kotlin Android, Swift, Zig, WASM, and C FFI.
- **Structured scrape output** — HTML, Markdown, metadata, links, assets, response headers, and extraction warnings with consistent field names.
- **Crawl controls** — depth, page limits, concurrency, URL filters, robots/sitemap handling, rate limits, and partial failure reporting.
- **Rendering path** — optional browser rendering for JavaScript-heavy pages; direct HTTP path for fast static pages.
- **SwiftPM package** — swift-bridge API for macOS and Linux clients.

## Installation

```bash
.package(url: "https://github.com/kreuzberg-dev/kreuzcrawl-swift", from: "0.3.0-rc.37")
```
## Quick Start

```swift title="Swift"
import Foundation
import Kreuzcrawl

@main
struct BasicUsage {
    static func main() async throws {
        // Simplest case: scrape a single page with default settings.
        let engine = try createEngine(nil)
        let result = try await scrape(engine, "https://example.com/")
        print("Title: \(result.metadata().title()?.toString() ?? "")")
        print("Status: \(result.status_code())")
        print("Links found: \(result.links().count)")

        // Crawl from a seed URL, limited to one hop and a handful of pages.
        let crawlConfig = try crawlConfigFromJson("{\"max_depth\":1,\"max_pages\":5}")
        let crawlEngine = try createEngine(crawlConfig)
        let crawlResult = try await crawl(crawlEngine, "https://en.wikipedia.org/wiki/Web_scraping")
        print("Pages crawled: \(crawlResult.pages().count)")
    }
}
```
## API Reference

Full API documentation is available at [docs.kreuzcrawl.kreuzberg.dev](https://docs.kreuzcrawl.kreuzberg.dev).

Key functions:

- `create_engine(config?)` — Create a crawl engine with optional configuration
- `scrape(engine, url)` — Scrape a single URL
- `crawl(engine, url)` — Crawl a website following links
- `map_urls(engine, url)` — Discover all pages on a site
- `batch_scrape(engine, urls)` — Scrape multiple URLs concurrently
- `batch_crawl(engine, urls)` — Crawl multiple seed URLs concurrently

## Contributing

Contributions are welcome! Please see our [Contributing Guide](https://github.com/kreuzberg-dev/kreuzcrawl/blob/main/CONTRIBUTING.md) for details.

## Part of Kreuzberg.dev

- [Kreuzberg](https://github.com/kreuzberg-dev/kreuzberg) — document intelligence: text, tables, metadata from 90+ formats with optional OCR.
- [Kreuzberg Cloud](https://github.com/kreuzberg-dev/kreuzberg-cloud) — managed extraction API with SDKs, dashboards, and observability.
- [html-to-markdown](https://github.com/kreuzberg-dev/html-to-markdown) — fast, lossless HTML→Markdown engine.
- [liter-llm](https://github.com/kreuzberg-dev/liter-llm) — universal LLM API client with native bindings for 14 languages and 143 providers.
- [tree-sitter-language-pack](https://github.com/kreuzberg-dev/tree-sitter-language-pack) — tree-sitter grammars and code-intelligence primitives.
- [alef](https://github.com/kreuzberg-dev/alef) — the polyglot binding generator that produces this README and all per-language bindings.
- [Discord](https://discord.gg/xt9WY3GnKR) — community, roadmap, announcements.

## License

This project is licensed under [Elastic License 2.0](https://github.com/kreuzberg-dev/kreuzcrawl/blob/main/LICENSE).

## Links

- [Documentation](https://docs.kreuzcrawl.kreuzberg.dev)
- [GitHub Repository](https://github.com/kreuzberg-dev/kreuzcrawl)
- [Issue Tracker](https://github.com/kreuzberg-dev/kreuzcrawl/issues)
- [Issues](https://github.com/kreuzberg-dev/kreuzcrawl/issues)
