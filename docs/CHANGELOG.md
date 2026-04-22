# Changelog

## Unreleased

### Breaking Changes

- **Config**: `MarkdownConfig` and `PlainTextConfig` replaced by unified `ContentConfig` on `CrawlConfig.content`
- **Config**: `main_content_only` removed from `CrawlConfig` — use `content.preprocessing_preset: "aggressive"` instead
- **Config**: `CrawlConfig.remove_tags` now forwarded to h2m's `exclude_selectors` instead of kreuzcrawl's DOM manipulation
- **Results**: `plain_text` field removed from `ScrapeResult` — set `content.output_format: "plain"` to get plain text in the `markdown` result field

### Features

- **Config**: `ContentConfig` with `output_format` supporting `"markdown"` (default), `"plain"`, `"djot"` — all powered by html-to-markdown-rs
- **Config**: `ContentConfig.exclude_selectors` for CSS selector-based element exclusion (`.class`, `#id`, `[attr]`) — replaces the buggy `apply_remove_tags` DOM manipulation
- **Config**: `ContentConfig.preprocessing_preset` (`"minimal"`, `"standard"`, `"aggressive"`) for controlling noise removal aggressiveness
- **Config**: Full h2m configuration exposed: `strip_tags`, `preserve_tags`, `skip_images`, `max_depth`, `wrap`, `wrap_width`
- **Encoding**: Non-UTF-8 charset detection and re-decoding via `encoding_rs` (fixes Shift_JIS, EUC-JP, etc.)
- **Benchmark**: Full benchmark harness at `tools/benchmark-harness/` — 1000-fixture scrape-evals dataset, quality scoring, CPU/memory profiling, flamegraphs, per-fixture output saving

### Fixes

- **Content**: Removed buggy `apply_remove_tags` that corrupted DOM on pages with repeated structural patterns — CSS selector exclusion now handled correctly by h2m during its DOM walk
- **Content**: Removed duplicate plain text extraction — h2m's `OutputFormat::Plain` handles this natively with proper preprocessing

## 0.1.2

### Breaking Changes

- **Rust API**: Public surface restricted to binding API only. `CrawlEngine`, `CrawlEngineBuilder`, all traits (`Frontier`, `RateLimiter`, `CrawlStore`, `EventEmitter`, `CrawlStrategy`, `ContentFilter`, `CrawlCache`), and all default implementations are now `pub(crate)`. Use `create_engine`, `scrape`, `crawl`, `map_urls`, `batch_scrape`, `batch_crawl` instead.
- **Rust API**: `BrowserPool`, `BrowserPoolConfig`, `PooledPage` no longer re-exported
- **Rust API**: `CachedPage`, `InteractionResult`, `ActionResult`, `CrawlEvent` removed from bindings

### Features

- **Config**: Added `rate_limit_ms: Option<u64>` to `CrawlConfig` for per-domain rate limiting across all languages
- **CLI**: Added `--browser-mode` and `--browser-endpoint` flags to `scrape` and `crawl` subcommands
- **CLI**: Browser fallback now works in the crawl path (was scrape-only)
- **CLI**: `--timeout` propagated to browser page-load timeout
- **CLI**: `--browser-endpoint` validated as `ws://` or `wss://` URL
- **CLI**: Refactored to use only the public binding API (`create_engine`, `scrape`, `crawl`, `map_urls`, `batch_crawl`)
- **Rust API**: `serve_api` and `start_mcp_server` re-exported at crate root for server deployments
- **Bindings**: TypeScript discriminated unions for `AuthConfig` and `CrawlEvent`
- **Bindings**: TypeScript non-optional fields are now required (no `?`) in `.d.ts`
- **Bindings**: JSDoc on all TypeScript types, functions, enums, and fields
- **Bindings**: Javadoc on all Java records, enums, and builders (with HTML escaping)
- **Bindings**: Go uses `json.RawMessage` for JSON value fields (was `interface{}`)
- **Bindings**: Elixir enum modules generated for all 9 enums
- **Bindings**: WASM rustdoc on all generated types and functions
- **Bindings**: WASM structured error objects `{code, message}` (was plain strings)
- **Infra**: Workspace-level Cargo lints (`clippy::all`, `unsafe_code`) inherited by all crates
- **Docs**: Lychee link checker added to CI docs workflow
- **E2E**: 48 new test fixtures covering batch_crawl, downloads, interaction, WARC, proxy, browser crawl, and more

### Fixes

- **Bindings**: Python mypy passes without `ignore-errors` (enum lookups, return type imports)
- **Bindings**: Go enum values use serde rename (e.g., `og:image` not `og_image`)
- **Bindings**: Go batch functions return error instead of panic
- **Bindings**: TypeScript string enum values use correct casing (snake_case)
- **Bindings**: TypeScript `format!("{:?}")` replaced with `.to_string()` for string fields
- **Bindings**: Elixir NIF crate name `kreuzcrawl_nif` (was `kreuzcrawl_rustler`)
- **Docs**: Removed internal type references (`LlmExtractor`, strategy names) from generated API reference
- **Docs**: Fixed broken links to deleted repos in comparisons page
- **Docs**: Added `CONTRIBUTING.md` at project root
- **Docs**: Snippet validator skips bare Ruby method signatures
- **Docs**: Snippet validator skips bare TypeScript method signatures
- **CI**: Homebrew formula sha256 handles single-quoted values
- **CI**: Docs workflow setup-go@v6 for Go 1.26 toolchain
- **CI**: Ruby `Gemfile.lock` synced for v0.1.1
- **Alef**: `alef docs` now uses filtered IR matching binding surface
- **Alef**: Deterministic C# NativeMethods.cs ordering (sorted DllImport entries)
- **Alef**: `alef.toml` uses exclude blacklist instead of include whitelist

## 0.1.1

### Fixes

- **WASM**: Added `getrandom` with `wasm_js` feature for wasm32 target compatibility
- **Java**: Downgraded Maven compiler and source plugins from beta to stable (4.0.0-beta → 3.x)
- **Elixir**: NIF scaffold lib.path + `MIX_ENV=prod` for Hex publish (from v0.1.0)
- **CI**: Fixed PEP 440 version conversion for stable releases (0.1.0 no longer becomes 0.10)

## 0.1.0

First stable release. High-performance web crawling engine with bindings for 11 languages.

### Highlights

- Rust core with async Tokio runtime, configurable crawl depth/concurrency/rate limiting
- REST API server (Firecrawl v1-compatible) with OpenAPI 3.1 spec
- MCP (Model Context Protocol) server for AI agent integration
- Docker image (Alpine, multi-arch amd64/arm64)
- CLI with Homebrew tap (`brew install kreuzberg-dev/tap/kreuzcrawl`)

### Language Bindings

Python (PyPI), Node.js (npm), Ruby (RubyGems), Go (pkg.go.dev), Java (Maven Central), C# (NuGet), PHP (Packagist), Elixir (Hex.pm), WebAssembly (npm), C FFI (GitHub Releases), Rust (crates.io)

### Changes Since rc.10

- Fixed Elixir Hex publish (NIF lib.path + MIX_ENV=prod)
- Fixed version.rb sync regex (pre-release suffix matching)
- Fixed Ruby native scaffold missing lib.path
- Clean prek run (all hooks pass)
- Idempotent `alef verify` via blake3 output content hashing

## 0.1.0-rc.10

### Features

- **Go**: Added FFI download pattern — `go generate` downloads prebuilt libraries from GitHub releases, enabling standalone `go get` without local C build
- **API**: Added schemathesis property-based contract tests (12 tests covering all endpoints)
- **CLI**: Added Homebrew installation instructions (`brew install kreuzberg-dev/tap/kreuzcrawl`)

### Fixes

- **Go**: Fixed non-opaque struct methods using `r.ptr` — now marshals to JSON via `_from_json` FFI
- **Go e2e**: Pass `nil` to `CreateEngine` when no config specified
- **Python stubs**: Removed docstrings from `.pyi` files (ruff PYI021 compliance)
- **WASM e2e**: Quote hyphenated keys in object literals, use `WasmCrawlConfig` class construction
- **Brew e2e**: Fixed jq `| length` pipe syntax (was `.length`), skip output capture for all-skipped assertions
- **Python e2e**: Wrap long `CrawlConfig` lines for E501 compliance
- **Rust e2e**: Removed `[workspace]` from generated Cargo.toml (conflicts with parent workspace)
- **Elixir**: Fixed long line formatting in `native.ex` scaffold
- **PHP**: Unified Packagist package name to `kreuzberg-dev/kreuzcrawl`
- **CI**: Removed prepare job gate that skipped release events
- **Docs**: Fixed stale version references in Java/Elixir READMEs and installation guide
- **Pre-commit**: Replaced local sync-versions hook with `alef-verify` + `alef-sync-versions`

## 0.1.0-rc.9

### Fixes

- **WASM**: Remove wasm-pack-generated `.gitignore` from `pkg/` subdirectories after build — npm respects nested `.gitignore` and was excluding compiled WASM artifacts even with `files` field set

## 0.1.0-rc.8

### Fixes

- **WASM**: Removed `pkg/` from `.gitignore` so npm publish includes compiled WASM artifacts
- **Ruby**: Fixed gem version format in test_apps (`0.1.0.pre.rc.3` instead of `0.1.0.rc3`)

## 0.1.0-rc.6

### Fixes

- **WASM**: Fixed npm package publishing — added `files`, `main`, `module`, `types` fields to package.json so compiled artifacts are included instead of raw Rust source
- **WASM e2e**: Added `tsconfig.json` to generated test_app (prevents Vite from walking to root tsconfig)
- **Elixir**: Removed non-existent `Cargo.lock` from mix.exs files list (NIF crate uses workspace lockfile)
- **Rust toolchain**: Switched from pinned 1.91 to `stable` (transitive dep `constant_time_eq` 0.4.3 requires 1.95)

### Features

- **Docker**: Added `publish-docker.yaml` workflow with Alpine CLI image, multi-arch builds (amd64/arm64)

## 0.1.0-rc.5

### Fixes

- **Version sync**: All workspace member Cargo.toml files now synced (binding crates were stuck at rc.2)
- **Ruby**: Fixed Duration conversion in validate method (`.map()` on `u64`)
- **Browser**: Re-wired `browser_fetch` into engine scrape pipeline (lost during Tower refactor)
- **Brew e2e**: Implemented 5 missing assertion types (greater_than_or_equal, contains_all, is_empty, less_than, not_contains)

## 0.1.0-rc.4

### Fixes

- **Node**: Added missing `serde` dependency to Node binding crate — fixes compilation failure
- **Elixir**: Added missing `serde` dependency to NIF crate + serde derives on enums — fixes compilation failure
- **Ruby**: Fixed conflicting `Default` implementations — derive vs manual impl no longer collide
- **Ruby**: Fixed enum conversion codegen — enum fields now use pattern matching instead of dot access
- **Ruby**: Fixed `Box<T>` deref in enum tuple variant conversion (CrawlEvent::Page)
- **Version sync**: Added root `package.json` and `kreuzcrawl-node/package.json` to sync-versions extra_paths

## 0.1.0-rc.3

### Fixes

- **Go**: Fixed module path to `github.com/kreuzberg-dev/kreuzcrawl/packages/go` for proper Go module resolution
- **Java**: Added extract-from-JAR native library loading — published Maven artifact now works standalone without manual `java.library.path` configuration
- **Elixir**: Switched to `RustlerPrecompiled` with GitHub release URLs for precompiled NIF binaries
- **PHP**: Fixed `createEngineFromJson()` — now uses `CrawlConfig` object construction matching the binding API
- **PHP**: Fixed risky test warning for fixtures with all skipped assertions
- **NuGet**: Use `PackageLicenseFile` instead of `PackageLicenseExpression` (Elastic-2.0 not OSI-approved)
- **Docker (musl)**: Source cargo env before build (PATH not inherited on ARM)
- **Ruby (macOS)**: Removed `setup-openssl` action that caused OpenSSL conflicts

### Features

- **Test apps**: Added test_apps for all 11 languages (Rust, Python, Node, Go, Java, C#, PHP, Ruby, Elixir, WASM, Homebrew CLI)
- **Brew generator**: New shell-script e2e test generator for Homebrew CLI testing
- **WASM**: Full e2e test support — removed incorrect language skips from all fixtures
- **WASM codegen**: Fixed `mock_url` and `handle` argument handling in generated tests
- **Go**: Updated to Go 1.26
- **Idempotency**: All 14 registry publish jobs check for existing packages before publishing

### Infrastructure

- **Publish workflow**: 66/76 jobs succeeded (0 failures, 10 skipped) on rc.2
- **Shared actions**: Upstreamed `setup-openssl` fix, leveraged shared build/publish actions from `kreuzberg-dev/actions`
- **Fixtures**: Removed all language skip blocks — all bindings are full crawlers

## 0.1.0-rc.2

- Initial multi-registry publish (crates.io, PyPI, npm, RubyGems, Maven Central, NuGet, Packagist, Hex.pm, Go, WASM, CLI binaries, Docker, Homebrew)
- Published kreuzcrawl and kreuzcrawl-cli to crates.io
- Created Homebrew formula in homebrew-tap repo

## 0.1.0-rc.1

- Initial release candidate
