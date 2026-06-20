# Kreuzcrawl

{% include 'partials/_badges.md' %}

High-performance Rust web crawling engine for structured data extraction. Scrape, crawl, and map websites with native bindings for 14 languages — same engine, identical results across every runtime.

## What and Why?

Kreuzcrawl is the crawling **substrate**: everything you need to scrape and crawl a site end-to-end from a single Rust core — HTML→Markdown, headless-Chrome fallback, robots/sitemap parsing, per-domain throttling, and an SSRF-safe policy — with identical results across 14 language bindings.

Productization concerns (managed proxy pools, tuned WAF fingerprints, authenticated-session injection, scheduling, billing) live in [kreuzberg-cloud](https://github.com/kreuzberg-dev/kreuzberg-cloud), the reference operational implementation. Every extension point (`Frontier`, `RateLimiter`, `CrawlStore`, `EventEmitter`, `ContentFilter`, `WafClassifier`, …) is a trait you inject via `CrawlEngineBuilder::with_<trait>(...)`.

### Features

| Feature | Description |
| ------- | ----------- |
| **Structured extraction** | Text, metadata, links, images, assets, JSON-LD, Open Graph, hreflang, favicons, headings, response headers |
| **Markdown conversion** | Clean Markdown with citations, document structure, and fit-content mode |
| **Concurrent crawling** | Depth-first, breadth-first, or best-first traversal with configurable depth, page limits, and concurrency |
| **14 language bindings** | Rust, Python, Node.js, Ruby, Go, Java, Kotlin (Android), C#, PHP, Elixir, Dart, Swift, Zig, and WebAssembly |
| **Smart filtering** | BM25 relevance scoring, URL include/exclude patterns, robots.txt compliance, sitemap discovery |
| **Browser rendering** | Optional headless browser for JavaScript-heavy SPAs with WAF detection and bypass |
| **Batch & streaming** | Scrape or crawl hundreds of URLs concurrently; real-time crawl events via async streams |
| **SSRF-safe by default** | Refuses loopback, private, link-local, and cloud-metadata addresses; opt out via env var or `CrawlConfig` |
| **Auth & rate limiting** | HTTP Basic, Bearer, and custom-header auth with cookie jars; per-domain request throttling |
| **MCP server & REST API** | Model Context Protocol integration for AI agents plus an HTTP server with OpenAPI spec |

### Supported Platforms

Precompiled binaries for Linux (x86_64/aarch64), macOS (ARM64), and Windows (x64) across every binding. See the [platform support reference](https://docs.kreuzcrawl.kreuzberg.dev) for the full matrix.

<div align="center">
  <a href="https://github.com/kreuzberg-dev/kreuzcrawl/stargazers">
    <img src="docs/assets/star.gif" alt="Star Kreuzcrawl on GitHub" width="640">
  </a>
</div>

<p align="center"><strong>⭐ Star this repo to show your support — it helps others discover Kreuzcrawl.</strong></p>

## Quick Start

### Language Packages

<details open>
<summary><strong>Python</strong></summary>

```sh
pip install kreuzcrawl
```

See [Python README](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/python) for full documentation.

</details>

<details>
<summary><strong>Node.js</strong></summary>

```sh
npm install @kreuzberg/kreuzcrawl
```

See [Node.js README](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/crates/kreuzcrawl-node) for full documentation.

</details>

<details>
<summary><strong>Rust</strong></summary>

```sh
cargo add kreuzcrawl
```

See [Rust README](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/crates/kreuzcrawl) for full documentation.

</details>

<details>
<summary><strong>Go</strong></summary>

```sh
go get github.com/kreuzberg-dev/kreuzcrawl/packages/go
```

See [Go README](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/go) for full documentation.

</details>

<details>
<summary><strong>Java</strong></summary>

Available on Maven Central as `dev.kreuzberg.kreuzcrawl:kreuzcrawl`. See [Java README](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/java) for the dependency snippet and current version.

</details>

<details>
<summary><strong>C#</strong></summary>

```sh
dotnet add package Kreuzcrawl
```

See [C# README](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/csharp) for full documentation.

</details>

<details>
<summary><strong>Ruby</strong></summary>

```sh
gem install kreuzcrawl
```

See [Ruby README](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/ruby) for full documentation.

</details>

<details>
<summary><strong>PHP</strong></summary>

```sh
composer require kreuzberg-dev/kreuzcrawl
```

See [PHP README](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/php) for full documentation.

</details>

<details>
<summary><strong>Elixir</strong></summary>

Add `{:kreuzcrawl, "~> 0.3"}` to your `mix.exs` dependencies. See [Elixir README](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/elixir) for full documentation.

</details>

<details>
<summary><strong>Dart / Flutter</strong></summary>

```sh
dart pub add kreuzcrawl
```

See [Dart README](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/dart) for full documentation.

</details>

<details>
<summary><strong>Kotlin (Android)</strong></summary>

Available on Maven Central as `dev.kreuzberg.kreuzcrawl.android:kreuzcrawl-android`. See [Kotlin README](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/kotlin-android) for the dependency snippet and current version.

</details>

<details>
<summary><strong>Swift</strong></summary>

Add via Swift Package Manager. See [Swift README](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/swift) for full documentation.

</details>

<details>
<summary><strong>Zig</strong></summary>

See [Zig README](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/zig) for installation and usage.

</details>

<details>
<summary><strong>WebAssembly</strong></summary>

```sh
npm install @kreuzberg/kreuzcrawl-wasm
```

See [WebAssembly README](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/crates/kreuzcrawl-wasm) for full documentation.

</details>

<details>
<summary><strong>C/C++ (FFI)</strong></summary>

C header + shared library from [GitHub Releases](https://github.com/kreuzberg-dev/kreuzcrawl/releases). See [FFI crate](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/crates/kreuzcrawl-ffi) for full documentation.

</details>

<details>
<summary><strong>CLI</strong></summary>

```sh
cargo install kreuzcrawl-cli
```

```sh
brew install kreuzberg-dev/tap/kreuzcrawl
```

See [CLI README](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/crates/kreuzcrawl-cli) for full documentation.

</details>

### AI Coding Assistants

Install the Kreuzcrawl plugin from the [`kreuzberg-dev/plugins`](https://github.com/kreuzberg-dev/plugins) marketplace. It ships the Kreuzcrawl agent skills (site crawling, HTML→Markdown scraping, headless-Chrome fallback) plus the `kreuzcrawl` MCP server, and works with every major coding agent — expand your harness below.

<details open>
<summary><strong>Claude Code</strong></summary>

```text
/plugin marketplace add kreuzberg-dev/plugins
/plugin install kreuzcrawl@kreuzberg
```

</details>

<details>
<summary><strong>Codex CLI</strong></summary>

```text
/plugins add https://github.com/kreuzberg-dev/plugins
```

Then search for `kreuzcrawl` and select **Install Plugin**.

</details>

<details>
<summary><strong>Cursor</strong></summary>

Settings → Plugins → Add from URL → `https://github.com/kreuzberg-dev/plugins`, then select **kreuzcrawl**.

</details>

<details>
<summary><strong>Gemini CLI</strong></summary>

```text
gemini extensions install https://github.com/kreuzberg-dev/plugins
```

</details>

<details>
<summary><strong>Factory Droid</strong></summary>

```text
droid plugin marketplace add https://github.com/kreuzberg-dev/plugins
droid plugin install kreuzcrawl@kreuzberg
```

</details>

<details>
<summary><strong>GitHub Copilot CLI</strong></summary>

```text
copilot plugin marketplace add https://github.com/kreuzberg-dev/plugins
copilot plugin install kreuzcrawl@kreuzberg
```

</details>

<details>
<summary><strong>opencode</strong></summary>

Add the package to `opencode.json`:

```json
{
  "$schema": "https://opencode.ai/config.json",
  "plugin": ["@kreuzberg/opencode-kreuzcrawl"]
}
```

</details>

## Documentation

Full guides, per-language API references, the substrate/operational model, antibot strategy, and observability live at **[docs.kreuzcrawl.kreuzberg.dev](https://docs.kreuzcrawl.kreuzberg.dev)**.

## Contributing

Contributions are welcome! See our [Contributing Guide](https://github.com/kreuzberg-dev/kreuzcrawl/blob/main/CONTRIBUTING.md).

## Part of Kreuzberg.dev

- [Kreuzberg](https://github.com/kreuzberg-dev/kreuzberg) — document intelligence: text, tables, metadata from 91+ formats with optional OCR.
- [Kreuzberg Cloud](https://github.com/kreuzberg-dev/kreuzberg-cloud) — managed extraction API with SDKs, dashboards, and observability.
- [kreuzcrawl](https://github.com/kreuzberg-dev/kreuzcrawl) — web crawling and scraping with HTML→Markdown and headless-Chrome fallback.
- [html-to-markdown](https://github.com/kreuzberg-dev/html-to-markdown) — fast, lossless HTML→Markdown engine.
- [liter-llm](https://github.com/kreuzberg-dev/liter-llm) — universal LLM API client with native bindings for 14 languages and 143 providers.
- [tree-sitter-language-pack](https://github.com/kreuzberg-dev/tree-sitter-language-pack) — tree-sitter grammars and code-intelligence primitives.
- [alef](https://github.com/kreuzberg-dev/alef) — the polyglot binding generator that produces every per-language binding across the 5 polyglot repos.

## License

[Elastic License 2.0](https://github.com/kreuzberg-dev/kreuzcrawl/blob/main/LICENSE)
