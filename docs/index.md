---
title: Kreuzcrawl
description: "Kreuzcrawl – High-performance Rust web crawling engine with always-on HTML→Markdown, headless-Chrome fallback for WAF-protected pages, and native bindings for 14 languages."
---

## kreuzcrawl

High-performance web crawling and scraping with a Rust core and native bindings for 14 languages. Always-on HTML→Markdown conversion, structured metadata, and a headless-Chrome fallback for WAF-protected pages — usable as a library, CLI, REST API, MCP server, or Docker image.

<div class="hero-badges" markdown>

[:material-lightning-bolt: Quick Start](getting-started/quickstart.md){ .md-button .md-button--primary }
[:material-package-variant: Installation](getting-started/installation.md){ .md-button }
[:material-feature-search-outline: Features](features.md){ .md-button }
[:fontawesome-brands-discord: Join our Community](https://discord.gg/xt9WY3GnKR){ .md-button }

</div>

---

### Why Kreuzcrawl

<div class="grid cards" markdown>

- :material-spider-web:{ .lg .middle } **Flexible Crawling**

  BFS, DFS, BestFirst, and Adaptive traversal with concurrent fetching, streaming events, and batch crawl/scrape.

- :material-text-box-outline:{ .lg .middle } **Always-On Markdown**

  Every fetched page is converted to clean, LLM-ready Markdown with citation tracking and content pruning.

- :material-database-search:{ .lg .middle } **Rich Metadata**

  `PageMetadata` carries Open Graph, Twitter Card, Dublin Core, article fields, JSON-LD, links, images, feeds, favicons, and hreflang.

- :material-web:{ .lg .middle } **Browser Fallback**

  Headless Chrome via chromiumoxide. Auto-detects WAF blocks across 8 vendors (Cloudflare, Akamai, Imperva, DataDome, PerimeterX, Sucuri, F5, AWS-WAF) and retries through a legitimate Chrome fingerprint.

- :material-server:{ .lg .middle } **MCP & REST Servers**

  Built-in MCP server for AI agents, REST API for service deployments, both gated behind cargo features.

- :material-archive:{ .lg .middle } **WARC Output**

  Standards-compliant WARC archiving for web preservation and audit-grade compliance pipelines.

- :material-translate:{ .lg .middle } **14 Language Bindings**

  Native bindings for Rust, Python, TypeScript, Go, Java, Kotlin (Android), C#, Ruby, PHP, Elixir, Dart, Swift, Zig, and WebAssembly — plus a C FFI surface for everything else.

</div>

→ **[See all features](features.md)**

---

### Language Support

| Language              | Package                                                    | Docs                                         |
| :-------------------- | :--------------------------------------------------------- | :------------------------------------------- |
| **Rust**              | `cargo add kreuzcrawl`                                     | [API Reference](reference/api-rust.md)       |
| **Python**            | `pip install kreuzcrawl`                                   | [API Reference](reference/api-python.md)     |
| **TypeScript / Node** | `npm install @kreuzberg/kreuzcrawl`                        | [API Reference](reference/api-typescript.md) |
| **WebAssembly**       | `npm install @kreuzberg/kreuzcrawl-wasm`                   | [API Reference](reference/api-wasm.md)       |
| **Go**                | `go get github.com/kreuzberg-dev/kreuzcrawl/packages/go`   | [API Reference](reference/api-go.md)         |
| **Java**              | Maven Central `dev.kreuzberg.kreuzcrawl:kreuzcrawl`        | [API Reference](reference/api-java.md)       |
| **Kotlin (Android)**  | Maven Central `dev.kreuzberg.kreuzcrawl:kreuzcrawl-android` | [API Reference](reference/api-kotlin-android.md) |
| **C#**                | `dotnet add package Kreuzcrawl`                            | [API Reference](reference/api-csharp.md)     |
| **Ruby**              | `gem install kreuzcrawl`                                   | [API Reference](reference/api-ruby.md)       |
| **PHP**               | `composer require kreuzberg-dev/kreuzcrawl`                | [API Reference](reference/api-php.md)        |
| **Elixir**            | `{:kreuzcrawl, "~> 0.3.0-rc.19"}`                          | [API Reference](reference/api-elixir.md)     |
| **Dart / Flutter**    | `dart pub add kreuzcrawl`                                  | [API Reference](reference/api-dart.md)       |
| **Swift**             | Swift Package Manager                                      | [API Reference](reference/api-swift.md)      |
| **Zig**               | `zig fetch --save` from GitHub                             | [API Reference](reference/api-zig.md)        |
| **C (FFI)**           | Shared library + header                                    | [API Reference](reference/api-c.md)          |
| **CLI**               | `cargo install kreuzcrawl-cli`                             | [CLI Guide](cli/usage.md)                    |
| **Docker**            | `ghcr.io/kreuzberg-dev/kreuzcrawl`                         | [Docker Guide](guides/docker.md)             |

!!! tip "Choosing between TypeScript packages"

    **`@kreuzberg/kreuzcrawl`** — Native NAPI-RS bindings. Use for Node.js servers and CLI tools. Full feature set including the browser fallback.

    **`@kreuzberg/kreuzcrawl-wasm`** — Pure WebAssembly. Use for browsers, Cloudflare Workers, Deno, Bun, and serverless. No headless-Chrome support.

---

### Quick Example

=== "Rust"

    ```rust title="src/main.rs"
    use kreuzcrawl::{CrawlConfig, ContentConfig, create_engine, crawl};

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let config = CrawlConfig {
            max_depth: Some(2),
            max_pages: Some(50),
            content: ContentConfig::default(),
            ..Default::default()
        };
        let engine = create_engine(Some(config))?;

        let result = crawl(&engine, "https://example.com").await?;
        for page in &result.pages {
            let title = page.metadata.title.as_deref().unwrap_or("(no title)");
            println!("{} — {}", page.url, title);
        }
        Ok(())
    }
    ```

=== "Python"

    ```python title="main.py"
    import asyncio
    from kreuzcrawl import CrawlConfig, create_engine, crawl

    async def main():
        engine = create_engine(CrawlConfig(max_depth=2, max_pages=50))
        result = await crawl(engine, "https://example.com")
        for page in result.pages:
            print(f"{page.url} — {page.metadata.title or '(no title)'}")

    asyncio.run(main())
    ```

=== "TypeScript"

    ```typescript title="index.ts"
    import { createEngine, crawl } from "@kreuzberg/kreuzcrawl";

    const engine = createEngine({ maxDepth: 2, maxPages: 50 });
    const result = await crawl(engine, "https://example.com");

    for (const page of result.pages) {
      console.log(`${page.url} — ${page.metadata.title ?? "(no title)"}`);
    }
    ```

---

### Part of kreuzberg.dev

<div class="grid cards" markdown>

- :material-file-document-multiple:{ .lg .middle } **[Kreuzberg](https://docs.kreuzberg.dev)**

  Document intelligence — text, tables, and metadata from 91+ file formats with optional OCR.

- :material-cloud:{ .lg .middle } **[Kreuzberg Cloud](https://docs.kreuzberg.cloud)**

  Managed document-extraction API with SDKs, dashboards, and observability built in.

- :material-language-html5:{ .lg .middle } **[html-to-markdown](https://docs.html-to-markdown.kreuzberg.dev)**

  The HTML→Markdown engine powering Kreuzcrawl's always-on conversion. Use it stand-alone for static HTML.

- :material-robot-outline:{ .lg .middle } **[liter-llm](https://docs.liter-llm.kreuzberg.dev)**

  Multi-provider LLM orchestration with cost and token accounting.

- :material-code-tags:{ .lg .middle } **[tree-sitter-language-pack](https://docs.tree-sitter-language-pack.kreuzberg.dev)**

  306 tree-sitter grammars and code-intelligence primitives. Used downstream when crawled pages contain code.

- :fontawesome-brands-discord:{ .lg .middle } **[Discord](https://discord.gg/xt9WY3GnKR)**

  Join the Kreuzberg community for help, roadmap discussion, and announcements.

</div>

---

### Explore the Docs

<div class="grid cards" markdown>

- :material-rocket-launch:{ .lg .middle } **Get Started**

  Install Kreuzcrawl and run your first crawl in under five minutes.

  [:octicons-arrow-right-24: Quick Start](getting-started/quickstart.md)

- :material-book-open-variant:{ .lg .middle } **Guides**

  Crawling, scraping, URL discovery, browser automation, WARC output, and deployment.

  [:octicons-arrow-right-24: All Guides](guides/crawling.md)

- :material-puzzle-outline:{ .lg .middle } **Concepts**

  Public surface, data flow, the binding matrix, feature gates, and the content-extraction pipeline.

  [:octicons-arrow-right-24: Architecture](concepts/architecture.md)

- :material-api:{ .lg .middle } **Reference**

  Per-language API docs, the configuration schema, type catalogue, and error matrix.

  [:octicons-arrow-right-24: References](reference/api-rust.md)

- :material-console:{ .lg .middle } **CLI & Servers**

  The `kreuzcrawl` CLI, REST API server, and MCP server for AI agents.

  [:octicons-arrow-right-24: CLI Usage](cli/usage.md)

- :material-feature-search-outline:{ .lg .middle } **Features**

  Complete feature breakdown: crawl strategies, metadata extraction, browser fallback, WARC, MCP, REST.

  [:octicons-arrow-right-24: Features](features.md)

</div>

---

### Getting Help

- **Bugs & feature requests** — [Open an issue on GitHub](https://github.com/kreuzberg-dev/kreuzcrawl/issues)
- **Community chat** — [Join the Discord](https://discord.gg/xt9WY3GnKR)
- **Contributing** — [Read the contributor guide](contributing.md)
