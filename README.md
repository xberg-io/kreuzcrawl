# Kreuzcrawl

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
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
  <a href="https://github.com/kreuzberg-dev/kreuzcrawl/releases">
    <img src="https://img.shields.io/badge/C-FFI-007ec6" alt="C">
  </a>

  <!-- Project Info -->
  <a href="https://github.com/kreuzberg-dev/kreuzcrawl/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-Elastic--2.0-blue.svg" alt="License">
  </a>
  <a href="https://docs.kreuzcrawl.kreuzberg.dev">
    <img src="https://img.shields.io/badge/docs-kreuzcrawl.dev-007ec6" alt="Documentation">
  </a>
</div>

<img width="3384" height="573" alt="Kreuzcrawl" src="https://github.com/user-attachments/assets/1b6c6ad7-3b6d-4171-b1c9-f2026cc9deb8" />

<div align="center" style="margin-top: 20px;">
  <a href="https://discord.gg/xt9WY3GnKR">
      <img height="22" src="https://img.shields.io/badge/Discord-Join%20our%20community-7289da?logo=discord&logoColor=white" alt="Discord">
  </a>
</div>

High-performance Rust web crawling engine for structured data extraction. Scrape, crawl, and map websites with native bindings for 10 languages — same engine, identical results across every runtime.

## Key Features

- **Structured extraction** — Text, metadata, links, images, assets, JSON-LD, Open Graph, hreflang, favicons, headings, and response headers
- **Markdown conversion** — Clean Markdown output with citations, document structure, and fit-content mode
- **Concurrent crawling** — Depth-first, breadth-first, or best-first traversal with configurable depth, page limits, and concurrency
- **10 language bindings** — Rust, Python, Node.js, Ruby, Go, Java, C#, PHP, Elixir, and WebAssembly
- **Smart filtering** — BM25 relevance scoring, URL include/exclude patterns, robots.txt compliance, and sitemap discovery
- **Browser rendering** — Optional headless browser for JavaScript-heavy SPAs with WAF detection and bypass
- **Batch operations** — Scrape or crawl hundreds of URLs concurrently with partial failure handling
- **Streaming** — Real-time crawl events via async streams for progress tracking
- **Authentication** — HTTP Basic, Bearer token, and custom header auth with persistent cookie jars
- **Rate limiting** — Per-domain request throttling with configurable delays
- **Asset download** — Download, deduplicate, and filter images, documents, and other linked assets
- **MCP server** — Model Context Protocol integration for AI agents
- **REST API** — HTTP server with OpenAPI spec

**[Documentation](https://docs.kreuzcrawl.kreuzberg.dev)** | **[API Reference](https://docs.kreuzcrawl.kreuzberg.dev/reference/)**

## Installation

| Language | Package | Install |
|----------|---------|---------|
| **[Python](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/python)** | [kreuzcrawl](https://pypi.org/project/kreuzcrawl/) | `pip install kreuzcrawl` |
| **[Node.js](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/typescript)** | [@kreuzberg/kreuzcrawl](https://www.npmjs.com/package/@kreuzberg/kreuzcrawl) | `npm install @kreuzberg/kreuzcrawl` |
| **[Rust](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/crates/kreuzcrawl)** | [kreuzcrawl](https://crates.io/crates/kreuzcrawl) | `cargo add kreuzcrawl` |
| **[Go](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/go)** | [pkg.go.dev](https://pkg.go.dev/github.com/kreuzberg-dev/kreuzcrawl/packages/go) | `go get github.com/kreuzberg-dev/kreuzcrawl/packages/go` |
| **[Java](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/java)** | [Maven Central](https://central.sonatype.com/artifact/dev.kreuzberg.kreuzcrawl/kreuzcrawl) | See [README](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/java) |
| **[C#](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/csharp)** | [NuGet](https://www.nuget.org/packages/Kreuzcrawl/) | `dotnet add package Kreuzcrawl` |
| **[Ruby](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/ruby)** | [kreuzcrawl](https://rubygems.org/gems/kreuzcrawl) | `gem install kreuzcrawl` |
| **[PHP](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/php)** | [kreuzberg-dev/kreuzcrawl](https://packagist.org/packages/kreuzberg-dev/kreuzcrawl) | `composer require kreuzberg-dev/kreuzcrawl` |
| **[Elixir](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/elixir)** | [kreuzcrawl](https://hex.pm/packages/kreuzcrawl) | `{:kreuzcrawl, "~> 0.1"}` |
| **[WASM](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/wasm)** | [@kreuzberg/kreuzcrawl-wasm](https://www.npmjs.com/package/@kreuzberg/kreuzcrawl-wasm) | `npm install @kreuzberg/kreuzcrawl-wasm` |
| **[C FFI](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/crates/kreuzcrawl-ffi)** | [GitHub Releases](https://github.com/kreuzberg-dev/kreuzcrawl/releases) | C header + shared library |
| **[CLI](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/crates/kreuzcrawl-cli)** | [crates.io](https://crates.io/crates/kreuzcrawl-cli) | `cargo install kreuzcrawl-cli` |
| **CLI (Homebrew)** | [kreuzberg-dev/tap](https://github.com/kreuzberg-dev/homebrew-tap) | `brew install kreuzberg-dev/tap/kreuzcrawl` |

## Quick Start

<details>
<summary><strong>Python</strong> — <a href="https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/python">Full docs</a></summary>

```python
from kreuzcrawl import create_engine, scrape

engine = create_engine()
result = scrape(engine, "https://example.com")

print(result.metadata.title)
print(result.markdown.content)
print(len(result.links))
```

</details>

<details>
<summary><strong>Node.js / TypeScript</strong> — <a href="https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/typescript">Full docs</a></summary>

```typescript
import { createEngine, scrape } from "@kreuzberg/kreuzcrawl";

const engine = createEngine();
const result = await scrape(engine, "https://example.com");

console.log(result.metadata.title);
console.log(result.markdown.content);
console.log(result.links.length);
```

</details>

<details>
<summary><strong>Rust</strong> — <a href="https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/crates/kreuzcrawl">Full docs</a></summary>

```rust
let engine = kreuzcrawl::create_engine(None)?;
let result = kreuzcrawl::scrape(&engine, "https://example.com").await?;

println!("{}", result.metadata.title);
println!("{}", result.markdown.content);
println!("{}", result.links.len());
```

</details>

<details>
<summary><strong>Go</strong> — <a href="https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/go">Full docs</a></summary>

```go
engine, _ := kcrawl.CreateEngine()
result, _ := kcrawl.Scrape(engine, "https://example.com")

fmt.Println(result.Metadata.Title)
fmt.Println(result.Markdown.Content)
fmt.Println(len(result.Links))
```

</details>

<details>
<summary><strong>Java</strong> — <a href="https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/java">Full docs</a></summary>

```java
var engine = Kreuzcrawl.createEngine(null);
var result = Kreuzcrawl.scrape(engine, "https://example.com");

System.out.println(result.metadata().title());
System.out.println(result.markdown().content());
System.out.println(result.links().size());
```

</details>

<details>
<summary><strong>C#</strong> — <a href="https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/csharp">Full docs</a></summary>

```csharp
var engine = KreuzcrawlLib.CreateEngine(null);
var result = await KreuzcrawlLib.Scrape(engine, "https://example.com");

Console.WriteLine(result.Metadata.Title);
Console.WriteLine(result.Markdown.Content);
Console.WriteLine(result.Links.Count);
```

</details>

<details>
<summary><strong>Ruby</strong> — <a href="https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/ruby">Full docs</a></summary>

```ruby
engine = Kreuzcrawl.create_engine(nil)
result = Kreuzcrawl.scrape(engine, "https://example.com")

puts result.metadata.title
puts result.markdown.content
puts result.links.length
```

</details>

<details>
<summary><strong>PHP</strong> — <a href="https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/php">Full docs</a></summary>

```php
$engine = Kreuzcrawl::createEngine(null);
$result = Kreuzcrawl::scrape($engine, "https://example.com");

echo $result->metadata->title;
echo $result->markdown->content;
echo count($result->links);
```

</details>

<details>
<summary><strong>Elixir</strong> — <a href="https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/elixir">Full docs</a></summary>

```elixir
{:ok, engine} = Kreuzcrawl.create_engine(nil)
{:ok, result} = Kreuzcrawl.scrape(engine, "https://example.com")

IO.puts(result.metadata.title)
IO.puts(result.markdown.content)
IO.puts(length(result.links))
```

</details>

## Platform Support

| Language | Linux x86_64 | Linux aarch64 | macOS ARM64 | Windows x64 |
|----------|:------------:|:-------------:|:-----------:|:-----------:|
| Python | ✅ | ✅ | ✅ | ✅ |
| Node.js | ✅ | ✅ | ✅ | ✅ |
| WASM | ✅ | ✅ | ✅ | ✅ |
| Ruby | ✅ | ✅ | ✅ | — |
| Elixir | ✅ | ✅ | ✅ | ✅ |
| Go | ✅ | ✅ | ✅ | ✅ |
| Java | ✅ | ✅ | ✅ | ✅ |
| C# | ✅ | ✅ | ✅ | ✅ |
| PHP | ✅ | ✅ | ✅ | ✅ |
| Rust | ✅ | ✅ | ✅ | ✅ |
| C (FFI) | ✅ | ✅ | ✅ | ✅ |
| CLI | ✅ | ✅ | ✅ | ✅ |

## Architecture

```text
Your Application (Python, Node.js, Ruby, Java, Go, C#, PHP, Elixir, ...)
    │
Language Bindings (PyO3, NAPI-RS, Magnus, ext-php-rs, Rustler, cgo, Panama, P/Invoke)
    │
Rust Core Engine (async, concurrent, SIMD-optimized)
    │
    ├── HTTP Client (reqwest + tower middleware stack)
    ├── HTML Parser (html5ever + lol_html)
    ├── Markdown Converter (html-to-markdown-rs)
    ├── Content Extraction (metadata, JSON-LD, Open Graph, readability)
    ├── Link Discovery (robots.txt, sitemaps, anchor analysis)
    └── Browser Rendering (optional headless Chrome/Firefox)
```

## Contributing

Contributions are welcome! See our [Contributing Guide](https://github.com/kreuzberg-dev/kreuzcrawl/blob/main/CONTRIBUTING.md).

## License

[Elastic License 2.0](https://github.com/kreuzberg-dev/kreuzcrawl/blob/main/LICENSE)

## Links

- [Documentation](https://docs.kreuzcrawl.kreuzberg.dev)
- [API Reference](https://docs.kreuzcrawl.kreuzberg.dev/reference/)
- [GitHub](https://github.com/kreuzberg-dev/kreuzcrawl)
- [Issues](https://github.com/kreuzberg-dev/kreuzcrawl/issues)
- [Discussions](https://github.com/kreuzberg-dev/kreuzcrawl/discussions)
- [Discord](https://discord.gg/xt9WY3GnKR)
