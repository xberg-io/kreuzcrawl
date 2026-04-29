---
title: Kreuzcrawl
description: A high-performance Rust web crawling engine for turning websites into structured data
---

# Kreuzcrawl

**A high-performance Rust web crawling engine for turning websites into structured data.**

Kreuzcrawl extracts metadata, converts HTML to markdown, supports LLM-powered structured extraction, and provides bindings for 11 programming languages -- all built on a trait-based, pluggable architecture with a Tower middleware stack.

---

<div class="grid cards" markdown>

-   :material-spider-web: **Web Crawling**

    ---

    BFS, DFS, BestFirst, and Adaptive traversal strategies with concurrent fetching, streaming events, and batch operations.

-   :material-text-box-outline: **Markdown Conversion**

    ---

    Always-on HTML-to-markdown with document structure preservation, numbered citations, and LLM-optimized content pruning.

-   :material-database-search: **Metadata Extraction**

    ---

    40+ metadata fields including Open Graph, Twitter Card, Dublin Core, JSON-LD, feeds, favicons, and hreflang links.

-   :material-robot: **AI / LLM Integration**

    ---

    Multi-provider LLM extraction via liter-llm with JSON schema support, cost tracking, and token usage counters.

-   :material-web: **Browser Automation**

    ---

    Headless Chrome fallback via chromiumoxide with WAF detection for 8 vendors, browser pooling, and persistent profiles.

-   :material-server: **MCP & REST API**

    ---

    Built-in MCP server and REST API for integrating crawl capabilities into AI agent workflows and web services.

-   :material-archive: **WARC Output**

    ---

    Standards-compliant WARC archiving for web preservation and compliance workflows.

-   :material-translate: **11 Language Bindings**

    ---

    Native bindings for Rust, Python, TypeScript, Ruby, Go, Java, C#, PHP, Elixir, WebAssembly, and C FFI.

</div>

---

## Quick Install

=== "Rust"

    ```toml title="Cargo.toml"
    [dependencies]
    kreuzcrawl = { version = "0.3", features = ["ai", "browser"] }
    tokio = { version = "1", features = ["full"] }
    ```

=== "Python"

    ```bash
    pip install kreuzcrawl
    ```

=== "TypeScript"

    ```bash
    npm install @kreuzberg/kreuzcrawl
    ```

=== "Docker"

    ```bash
    docker pull ghcr.io/kreuzberg-dev/kreuzcrawl:latest
    ```

=== "CLI"

    ```bash
    cargo install kreuzcrawl-cli
    ```

See the [full installation guide](getting-started/installation.md) for all 11 languages.

---

## Quick Example

```rust title="src/main.rs"
use kreuzcrawl::{CrawlConfig, CrawlEngine};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = CrawlEngine::builder()
        .config(CrawlConfig {
            max_depth: Some(2),
            max_pages: Some(50),
            ..Default::default()
        })
        .build()?;

    let result = engine.crawl("https://example.com").await?;

    for page in &result.pages {
        println!("{}: {}", page.url, page.metadata.title.as_deref().unwrap_or(""));
        if let Some(ref md) = page.markdown {
            println!("  {} chars of markdown", md.content.len());
        }
    }

    Ok(())
}
```

---

## Next Steps

- **[Getting Started](getting-started/installation.md)** -- Install kreuzcrawl for your language
- **[Quick Start](getting-started/quickstart.md)** -- Scrape, crawl, and map in under 5 minutes
- **[Features](features.md)** -- Full feature breakdown and competitive comparison
