# Changelog

All notable changes to kreuzcrawl are documented here.

## [Unreleased]

### Changed

- **Regenerated all alef bindings against `alef 0.17.8`** (was `0.17.2`). Two
  behaviour changes flow through to the per-language bindings:
  - Rustdoc intra-doc links of the form `[Type::method]` are now emitted as
    plain ``Type.method`` in non-Rust binding source comments. The previous
    output left raw `[CrawlConfig::default()]` markers that rustdoc could not
    resolve in the binding crates (no `CrawlConfig` symbol in scope), causing
    `clippy::doc-link-with-quotes` and `rustdoc::broken-intra-doc-links`
    warnings on the binding crates.
  - PHP binding: `CrawlEngineHandle::batchCrawlStream` was renamed to
    `crawlStream` to match the canonical method name on the core engine.
    Affects callers of the PHP package only; no impact on other bindings.
- **Swift binding surface** expanded — `RustBridgeC.h` and
  `RustBridge/kreuzcrawl-swift.swift` now include the full pluggable-component
  surface (was partial in rc.23).
- **Reference docs (`docs/reference/api-*.md`, `configuration.md`, `types.md`,
  `errors.md`)** regenerated with the new alef table renderer — content is
  identical, table formatting is the new canonical style.

## [0.3.0-rc.23] - 2026-05-20

### Fixed

- **Per-binding READMEs in `packages/*/README.md` and `crates/kreuzcrawl-{node,wasm}/README.md`**
  refreshed so the badge `filter=v{{ version }}` and install snippets resolve to `0.3.0-rc.23`.
  The rc.22 release regenerated package manifests but the README cache in `.alef/` was not
  invalidated by the version bump, leaving stale `rc.21` badges that failed CI's
  "Check README freshness" step. Workaround applied locally: `alef cache clear && alef readme`.
  Upstream fix tracked in alef so future `alef sync-versions --set …` bumps also invalidate
  the README cache automatically.

## [0.3.0-rc.22] - 2026-05-20

### Fixed

- **`crates/kreuzcrawl-browser/src/dom/tree_sink.rs`** — `DomElemName`'s three `unsafe`
  blocks (`Debug::fmt`, `ElemName::ns`, `ElemName::local_name`) now carry
  `#[allow(unsafe_code)]` with SAFETY comments documenting the `Ref`-backed
  pointer-lifetime invariant. The workspace lint `unsafe_code = "warn"` combined with
  `-D warnings` was promoting these to compile errors and breaking `task rust:lint:check`
  and macOS clippy in CI.

## [0.3.0-rc.21] - 2026-05-20

### Added

- **`BrowserPool` and `BrowserPoolConfig` are now part of the public API** (re-exported from
  the crate root under `#[cfg(feature = "browser")]`). Downstream consumers such as
  `kreuzberg-cloud`'s worker can construct and `warm()` a pool at process startup then
  reuse it across all crawl jobs.

- **`NativeBrowserExecutor` and `NativeBrowserExecutorConfig` are now re-exported** from the
  crate root under `#[cfg(feature = "browser-native")]` (forwarded from
  `kreuzcrawl_browser::adapter`). Allows the same startup-once pattern for the native
  browser backend.

- **`CrawlEngineBuilder::with_browser_pool(Arc<BrowserPool>)`** — inject a pre-built
  chromiumoxide pool into the engine at build time. Takes precedence over any pool set in
  `CrawlConfig.browser_pool`. Gated on `#[cfg(feature = "browser")]`.

- **`CrawlEngineBuilder::with_native_executor(Arc<NativeBrowserExecutor>)`** — inject a
  pre-built native browser worker pool into the engine. Takes precedence over the executor
  constructed from config. Gated on `#[cfg(all(not(target_arch = "wasm32"), feature = "browser-native"))]`.

- **`#[cfg_attr(alef, alef(skip))]` annotations** on `BrowserPool`, `BrowserPoolConfig`,
  `NativeBrowserExecutor`, `NativeBrowserExecutorConfig`, and both new builder methods —
  these types are Rust-only and intentionally excluded from alef-generated polyglot
  bindings. Language clients drive the engine API directly and never touch the pool layer.

- **`[lints] workspace = true`** added to `kreuzcrawl-browser/Cargo.toml` so the workspace
  `check-cfg = ['cfg(alef)']` registration applies to the browser crate; without it the
  new annotations emit `unexpected_cfgs` warnings in regular builds.

- **`CrawlEngineHandle::from_engine(engine: CrawlEngine) -> Self`** — wrap a builder-built
  `CrawlEngine` as a `CrawlEngineHandle`. Required when injecting a pre-built
  `NativeBrowserExecutor` via `CrawlEngineBuilder::with_native_executor`, since the
  `create_engine(Option<CrawlConfig>)` shorthand has no way to thread an executor through
  (only `CrawlConfig.browser_pool` for chromiumoxide). Rust-only: `#[cfg_attr(alef, alef(skip))]`.

### Notes

- The existing `CrawlConfig.browser_pool` field (added in rc.20) remains the canonical
  low-level injection point; the new builder methods are convenience wrappers around it.
- `Arc::strong_count` on the injected pool increases by exactly 1 per engine built — the
  engine holds a clone inside its `CrawlConfig`, no additional clones are created per
  crawl call.
