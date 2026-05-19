# Changelog

## Unreleased

### Features

- **Core**: Published `interact()` with `PageAction`, `ActionResult`, and `InteractionResult` for backend-neutral page interaction.
- **Browser (native)**: Added native `interact()` execution for click, type, press, scroll, wait, JavaScript, and scrape actions. Screenshot actions return an action-level unsupported result because the native backend has no visual renderer.

### Fixes

- **Core**: `interact()` now runs `BrowserConfig.eval_script` after navigation and before page actions.
- **Core**: Page-action validation now rejects invalid wait and scroll selectors before navigation.
- **MCP**: The MCP `interact` tool now delegates to the public engine API instead of returning a placeholder message.

## 0.3.0 - 2026-05-18

### Highlights

- **Native browser backend** — a Servo/Deno-derived in-process browser (`browser.backend = "native"`) joins the existing chromiumoxide backend. Stealth TLS fingerprinting via BoringSSL, proxy with credentials, URL-pattern request blocking, JavaScript evaluation, selector-wait, robots-txt awareness, cookie forwarding, and full network-event capture.
- **Five new language bindings** — Kotlin Android (AAR), Dart (Flutter Rust Bridge), Swift (XCFramework), Zig (native module), and C (cdylib + header) bring total binding coverage from 11 to 16 languages.
- **Generator migration to alef** — the entire bindings/docs/READMEs/e2e surface is now produced by [alef](https://github.com/kreuzberg-dev/alef) (pinned via `alef.toml`). Replaces the prior hand-rolled codegen scripts and dramatically tightens cross-language consistency.

### Breaking Changes

- **Rust API**: `kreuzcrawl::CrawlEngineBuilder`, strategy types, and default trait implementations are once again exported at the crate root (reverting the 0.1.2 lockdown) — downstream code that worked around the prior restriction can drop its wrappers.
- **Browser config**: `BrowserConfig.backend` is now an enum (`"chromium"` | `"native"`) instead of a free-form string. Existing configs that omitted the field continue to default to chromium.
- **Bindings**: `DownloadedDocument.content` and `ScrapeResult.screenshot` (binary fields) are hidden from non-Rust bindings — call the dedicated download/screenshot helpers instead of reading the field.
- **Bindings**: `CrawlConfig.browser_pool` is no longer exposed across the FFI boundary (was unusable from non-Rust callers anyway).
- **WASM**: Generated class names use the configured `wasm_type_prefix` (default `Wasm`) consistently across builders and tagged-enum constructors. Calls into `WasmAuthConfig` are now field-based instead of enum-based.
- **Errors**: Network errors are tagged with a stable `[network:<kind>]` prefix in their message — assertions that grepped on raw reqwest text need updating.
- **C# / Java / Go**: Tagged unions emit as sealed interfaces / records / discriminated structs (was nested classes / raw maps). Per-binding migration notes are in each package README.

### Features

- **Browser (native)**: `BrowserExtras` on `ScrapeResult.browser` carries `eval_result`, `network_events`, and `cookies` populated by the native backend.
- **Browser (native)**: `browser-native` Cargo feature added to the `full` feature set.
- **Core**: New `full` feature aggregates every optional capability (api, browser, mcp, native browser, etc.) for one-flag installs.
- **Core**: `CrawlConfig.soft_http_errors` unifies 404 handling — opt in to treat soft-404 HTML as success instead of error.
- **Core**: `scrape()` follows redirects with cycle detection and a soft stop on `max_redirects`.
- **Core**: `batch_scrape` / `batch_crawl` now return `Result` and reject empty input rather than silently succeeding.
- **Core**: Network errors carry a stable `[network:<kind>]` tag for programmatic dispatch.
- **Core**: `CrawlConfig::validate()` enforces `max_depth`, `max_body_size`, proxy URL, and auth-config shape at build time; browser endpoint URL scheme is validated.
- **Core**: `CrawlConfig.max_depth` defaults to unbounded when unset (was 1).
- **Core**: Browser fallback is now restricted to `WafBlocked` and `Forbidden` errors (was overly broad).
- **CLI**: `--config` flag accepts a JSON `CrawlConfig` for both `scrape` and `crawl`.
- **Bindings**: WASM `getter`s / `setter`s on enum-typed fields use strings for JS interop; `asset_types` accepts string arrays.
- **Bindings**: C language binding added (cdylib + cbindgen header, full e2e parity).
- **Bindings**: Dart bridge via flutter_rust_bridge with full e2e parity.
- **Bindings**: Swift package with XCFramework distribution + full e2e parity.
- **Bindings**: Zig native module + full e2e parity.
- **Bindings**: Kotlin replaced JVM facade with Kotlin-Android (AAR + Android Gradle Plugin) — drops desktop-JVM target; mobile-only.
- **Bindings (JNI)**: Migrated from `jni = "0.21"` to `jni = "0.22"` for the FFI-safe `EnvUnowned<'frame>` API.
- **API**: `CachedPage` re-exported from crate root.
- **Tooling**: All bindings, docs, READMEs, and e2e suites are generated by [alef](https://github.com/kreuzberg-dev/alef) — version pinned in `alef.toml`. The `task alef:bump` / `task alef:regen` / `task rebuild` cycle replaces the prior hand-maintained scripts.
- **Tooling**: New publish targets: Kotlin Android (Maven Central), Dart (pub.dev), Swift (Swift Package Index), Zig (build registry).
- **Tooling**: Per-language `task update` / `task upgrade` split (within-major vs latest).
- **Tooling**: Adopted `gh-actions-updater` (Goldziher) for GHA pin maintenance.
- **CI**: Split monolithic `ci.yaml` into kreuzberg-topology workflows (`ci-rust`, `ci-e2e`, `ci-docs`, `ci-mobile`, `publish`).
- **CI**: New `ci-mobile` workflow runs Android (AAR) and iOS (XCFramework) cargo checks.
- **CI**: Discord release announcements wired into `publish.yaml`.
- **Docs**: Canonical Material/Zensical docs site at `docs.kreuzcrawl.kreuzberg.dev` aligned with sibling Kreuzberg.dev properties (shared CSS, base template, GA, ecosystem grid, llms.txt).
- **Docs**: All concepts, guides, getting-started, features, and reference pages rewritten against the public binding surface only.
- **Repo**: `CITATION.cff` generated from `[workspace.citation]` in `alef.toml`.

### Fixes

- **Elixir**: `:force_build` now respects `config :rustler_precompiled, :force_build, kreuzcrawl: true` in addition to the `KREUZCRAWL_BUILD` env var, fixing the documented workaround that was previously ignored when users hit precompiled checksum errors (#7).
- **Browser (chromium)**: Filter snap-incompatible flags + drop `enable-blink-features` default arg — fixes startup under snap-packaged Chromium on Ubuntu noble.
- **Browser (chromium)**: Re-wired `browser_fetch` into the engine scrape pipeline (lost during the Tower refactor).
- **Core**: `quick-xml` 0.40 `xml_content` API migration — sitemap parsing keeps working with the upgraded dep.
- **Core**: WAF detection tests are deterministic on macOS.
- **WASM**: Capture response headers in `HttpResponse` for wasm builds (was empty).
- **WASM**: Crate compiles for `wasm32-unknown-unknown` without `mio` (gated out under wasm32).
- **WASM**: Structured error objects `{code, message}` (was plain string).
- **Bindings (PHP)**: PSR-4 namespace escaping in `composer.json`.
- **Bindings (Java)**: Added `jspecify` dep for `@Nullable` annotations; `Optional` wrapping for nullable returns.
- **Bindings (Ruby)**: `sorbet-runtime` declared as gemspec dep; `html-to-markdown-rs` 3.4 sig change handled.
- **Bindings (C#)**: Sealed-union and exception deserialization corrected.
- **Bindings (Go)**: Enum values use serde rename (`og:image`), batch functions return error instead of panic.
- **Docs**: Stale install snippets, version strings, and binding sample code reconciled with the actual public APIs.

See `Cargo.toml` for the full dependency graph; `alef.toml` for the generator pin.

## 0.2.0

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
- **WAF**: Expanded WAF detection — AWS CloudFront, `awswaf.com` challenge scripts, "Verifying your connection" interstitials, "Just a moment" Cloudflare pages
- **WAF**: WAF detection on HTTP 200 responses with challenge content (catches false-positive 200s from AWS WAF, Cloudflare)
- **Browser**: Browser fallback on `Forbidden`, `Connection`, and generic errors (was `WafBlocked` only) — catches TLS fingerprint blocks and bot-detection responses
- **Browser**: Unique user data directories per Chrome launch — prevents `SingletonLock` conflicts when multiple instances run concurrently or after crashes
- **Benchmark**: Full benchmark harness at `tools/benchmark-harness/` with:
  - Scrape-evals dataset (1000 fixtures from HuggingFace) with TF1 multiset F1 scoring
  - Reachability benchmark (16 domains across e-commerce, social, professional, review) with content verification and false-positive detection
  - CPU flamegraphs via pprof, real-time memory/CPU monitoring
  - Per-fixture output saving, baseline comparison reports
  - CLI with download, run, profile, report, validate commands
- **CI**: Pinned alef to v0.5.3 in CI workflow

### Fixes

- **Content**: Removed buggy `apply_remove_tags` that corrupted DOM on pages with repeated structural patterns — CSS selector exclusion now handled correctly by h2m during its DOM walk
- **Content**: Removed duplicate plain text extraction — h2m's `OutputFormat::Plain` handles this natively with proper preprocessing
- **Browser**: Clean up temporary user data directories after browser teardown and on launch failure

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
