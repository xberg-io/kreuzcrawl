# kreuzcrawl

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
    <img src="https://img.shields.io/github/v/tag/kreuzberg-dev/kreuzcrawl?label=Go&color=007ec6&filter=v0.2.0" alt="Go">
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
  <a href="https://docs.kreuzcrawl.kreuzberg.dev">
    <img src="https://img.shields.io/badge/Docs-kreuzcrawl.dev-007ec6" alt="Documentation">
  </a>
  <a href="https://github.com/kreuzberg-dev/kreuzcrawl/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-Elastic--2.0-blue.svg" alt="License">
  </a>
</div>

<img width="3384" height="573" alt="Kreuzcrawl" src="https://github.com/user-attachments/assets/1b6c6ad7-3b6d-4171-b1c9-f2026cc9deb8" />

<div align="center" style="margin-top: 20px;">
  <a href="https://discord.gg/xt9WY3GnKR">
      <img height="22" src="https://img.shields.io/badge/Discord-Join%20our%20community-7289da?logo=discord&logoColor=white" alt="Discord">
  </a>
</div>

Node.js bindings for **kreuzcrawl** — a high-performance Rust web crawling engine. Powered by
NAPI-RS with full TypeScript types, async/await support, and native performance.

## Installation

```bash
npm install @kreuzberg/kreuzcrawl
```

## Quick Start

```typescript
import { createEngine, scrape } from "@kreuzberg/kreuzcrawl";

// Create engine with default settings
const engine = createEngine();

// Scrape a single page
const result = await scrape(engine, "https://example.com");
console.log(`Title: ${result.metadata.title}`);
console.log(`Status: ${result.statusCode}`);
console.log(`Links: ${result.links.length}`);
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

## License

This project is licensed under [Elastic License 2.0](https://github.com/kreuzberg-dev/kreuzcrawl/blob/main/LICENSE).

## Links

- [Documentation](https://docs.kreuzcrawl.kreuzberg.dev)
- [GitHub Repository](https://github.com/kreuzberg-dev/kreuzcrawl)
- [Issue Tracker](https://github.com/kreuzberg-dev/kreuzcrawl/issues)
- [Discussions](https://github.com/kreuzberg-dev/kreuzcrawl/discussions)
