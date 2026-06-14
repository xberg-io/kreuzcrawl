# Changelog

All notable changes to kreuzcrawl are documented here.

## [Unreleased]

### Security

- **SSRF defense in core HTTP layer.** `scrape()`, `crawl()`, `batch_crawl()`,
  sitemap fetch, robots.txt fetch, and asset download now refuse URLs
  resolving to loopback (127.0.0.0/8), RFC1918 private networks
  (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16), link-local (169.254.0.0/16),
  cloud metadata (0.0.0.0/8), multicast (224.0.0.0/4), IPv6 ULA (fc00::/7),
  IPv6 link-local (fe80::/10), IPv6 multicast (ff00::/8), or any
  non-http(s) scheme. Enabled by default; opt out via
  `KREUZCRAWL_ALLOW_PRIVATE_NETWORK=1` environment variable or
  `CrawlConfig::allow_private_networks(true)`.
- **DNS rebinding mitigation** in both core `http_fetch` and the Tower-stack
  `do_fetch`: hostnames resolving to a mix of public and private IPs are
  refused outright (all resolved IPs must individually pass the policy).
- **Redirect-chain re-validation.** Each 30x `Location` is parsed, joined,
  re-resolved, and re-validated against the SSRF policy before following
  the next hop. Mirrors the browser layer's GHSA-8v6v-g4rh-jmcm mitigation.
  Bounded at `CrawlConfig::ssrf.max_redirects` (default 5).
- **Link-enqueue validation.** Candidate URLs discovered during crawl are
  validated at enqueue time with bounded concurrency (16-permit semaphore).
  Refused targets are dropped with a structured `tracing::warn!` instead of
  silently followed.

### Added

- **`kreuzcrawl::net::ssrf` module** with `SsrfPolicy`, `HostMatcher`
  (`Exact`, `Suffix`, `Cidr` variants), `SsrfError`, and the async
  `validate_url(&Url, &SsrfPolicy)` validator.
- **`CrawlConfig::ssrf` field** (defaults to deny private networks) and
  `CrawlConfigBuilder` methods `allow_private_networks(bool)` and
  `ssrf_allowlist_host(HostMatcher)` for policy customization.
- **`CrawlError::SsrfPolicyViolation { url, reason }`** — classified as
  permanent (no retry) in the default retry policy and domain state to
  avoid wasted work on blocked endpoints.

### Changed

- **Asset downloads** in `assets::download_single_asset` now route through
  `http_fetch` (previously bypassed via raw `reqwest::get()`), ensuring
  all file fetches are subject to the SSRF policy.
- **Browser-layer SSRF validation** (`kreuzcrawl-browser`) realigned to the
  shared policy constants. `file://` scheme and the `localhost`-string
  short-circuit remain as browser-specific extras (defence-in-depth against
  DNS rebinding through the browser process resolver). Separate mirror
  module due to dependency-cycle constraint.

## [0.3.0-rc.61] - 2026-06-14

### Changed

- Bump alef pin and regenerate against the new publish-prepare lockfile-preservation fix (per-member `cargo update -p NAME` + `cargo metadata --locked` validation, replacing the prior `cargo generate-lockfile` that rebuilt from scratch with latest semver-compatible versions) and the `kreuzberg-dev/actions@v1.8.68` build-time `--locked` sweep across every `cargo build` / `maturin build` / `cargo ndk` / `cargo zigbuild` invocation. Together these stop broken upstream releases (e.g. `brotli-decompressor 5.0.1`/`5.0.2` whose `alloc-no-stdlib` v2/v3 split trips `error[E0277] StandardAlloc: alloc::Allocator<u8> is not satisfied`) from leaking past the committed `Cargo.lock` on the publish path. Unbreaks rc.60's `Build Elixir NIF (macos-arm64 nif-2.17)` and `Build PHP extension (php8.3 macos-arm64)` jobs.

### Fixed

- **Musl FFI Docker builds install GNU `sed`.** Mirrors the rc.60 `musl-build` fix: BusyBox `sed` (default on Alpine) lacks `-z` mode required by the multi-line workspace members rewrite, so `Dockerfile.musl-ffi` now `apk add`s `sed` explicitly. This unbreaks Java natives `linux-musl-{x64,aarch64}` publish.

## [0.3.0-rc.60] - 2026-06-14

### Changed

- Bump alef pin to `0.25.0` and regenerate all bindings, facades, READMEs, docs, stubs, and e2e suites against the new codegen.

### Fixed

- **CI: Go module publish guards now check `release_go == 'true'`.** Both `upload-go-release` and `push-go-subdir-tag` jobs explicitly gate on `needs.prepare.outputs.release_go == 'true'`. When Go is intentionally skipped (partial publish), both jobs skip; when Go fails, the OR condition `(success || skipped)` in `push-go-subdir-tag` ensures the tag is only created if artifacts exist.
- **C FFI staging matches test_apps download contract.** Publish-time staging now produces `kreuzcrawl-ffi-v${VERSION}-${TARGET}.tar.gz` (matching `test_apps/c/download_ffi.sh`) instead of the legacy `kreuzcrawl-ffi-${PLATFORM}` shape that was causing 404s.
- **Zig publish bundles multi-platform FFI artifacts.** `publish-zig` job now depends on `c-ffi-libraries`, downloads per-target tarballs, lays them out under `ffi-artifacts/<rid>/`, and passes the directory via `multi-platform-ffi-dir` so the zig package includes `lib/` and `include/` for every supported RID.
- **Musl Docker builds install GNU `sed`.** BusyBox `sed` lacks `-z` mode required by the multi-line workspace members rewrite; the `musl-build` image now `apk add`s `sed` explicitly.
- **Elixir NIF Windows publish.** Windows runners override `ImageOS=win25` before `erlef/setup-beam@v1` because `win25-vs2026` is not a recognised value.

## [0.3.0-rc.59] - 2026-06-13

### Changed

- Bump alef pin to `0.24.17` and regenerate all bindings, facades, READMEs, docs, stubs, and e2e suites against the new codegen.

## [0.3.0-rc.39] - 2026-05-29

### Changed

- Bump alef pin to `0.20.14` to consume the ruby Rakefile fixes (gemspec extensions path, file-only glob, relative lib_dir) and post-0.20.12 codegen fixes (project-name special-casing, dart Uri qualification, swift async overrides, dart build config-aware post-processors, csharp delegates GCHandle pin, rustler base_url wrap).

### Fixed

- **ruby binding compile**: orphaned files from older alef layouts under `packages/ruby/ext/kreuzcrawl_rb/` removed; alef-generated tree now matches the v0.20.14 layout (extconf.rb at `ext/<name>/native/`, gemspec extensions resolved correctly, rake-compiler copy task no longer trips on directories).
- **brew test_app wired into CI**: `verify-published-packages` matrix in `.github/workflows/publish.yaml` already includes `brew` as a language target; previously verified absent in CHANGELOG but confirmed live this iteration.

## [0.3.0-rc.38] - 2026-05-29

### Added

- **Tier-dispatch engine** (`feat/tier-dispatch`). The crawl engine now
  chains HTTP → Bypass → Browser tiers driven by per-attempt signals
  rather than a single bypass short-circuit. New public types in
  `kreuzcrawl::types::dispatch`: `Tier`, `EscalationStrategy`,
  `EscalationReason`, `AttemptOutcome`, `RetryDirective`, `RetryPolicy`,
  `WafSignal`, `WafClassifier`, `WafClassifyError`, `DomainStatePort`,
  `DomainRecommendation`, `DomainObservation`, `ObservedOutcome`,
  `EscalationBudget`, `BudgetExhausted`, and `DispatchProfile`. All
  enums marked `#[non_exhaustive]` so future variants are non-breaking.
- **`DispatchProfile`** bundles the dispatch trait-objects (bypass,
  retry policy, WAF classifier, domain state, escalation budget) into a
  single field on `CrawlConfig`, replacing the previous six scattered
  `Option<Arc<dyn …>>` fields.
- **Fluent builders** — `CrawlConfig::builder()` and
  `DispatchProfile::builder()` for ergonomic construction.
- **TOML WAF fingerprint corpus** — `rules/waf_fingerprints.toml`
  carrying 34 fingerprints across the major vendors. Single source of
  truth for vendor, severity, and header/body signals. Aho-Corasick
  matcher with ASCII case-insensitive lookup. Defends against malformed
  input via `MAX_FINGERPRINTS`, `MAX_PATTERN_LEN`, and
  `MAX_SIGNALS_PER_FINGERPRINT` gates in `Rules::load_from_str`.
- **`TomlClassifier::watch(path)`** — hot-reload of the WAF rules file
  via `notify`, debounced 500 ms, atomic swap via `ArcSwap`. Designed
  for Kubernetes ConfigMap projected volumes and the tmpfile-rename
  safe-write pattern.
- **`EwmaDomainState`** — process-local default `DomainStatePort` impl
  with exponentially-weighted block-rate tracking (α=0.1, ~72h half-life)
  driving promote/demote of starting tier per domain.
- **OpenTelemetry counters** — `kreuzcrawl_waf_fingerprint_matches_total`
  (per-fingerprint match count) and `kreuzcrawl_escalations_total`
  (per tier transition with `from`, `to`, `reason` labels). Always-on;
  not behind the `tracing` feature gate.
- **`AttemptOutcome.content_density`** — `text_bytes / html_bytes` for
  SPA-shell detection. Populated by the engine on every HTTP fetch.
- **Property tests** (`tests/test_proptest_invariants.rs`) — EWMA
  bounded + convergence, `FixedBudget` never-overdraw under concurrent
  use, `compute_backoff_ms` monotonic until cap.
- **Fuzz targets** (`fuzz/fuzz_targets/{toml_loader,waf_classify}.rs`) —
  cargo-fuzz panic-free invariants on the WAF subsystem's external
  attack surfaces, run for 30 s per PR by `ci-fuzz.yaml` on nightly.
- **Criterion benchmarks** (`benches/waf.rs`) — Aho-Corasick build
  cost and classify latency baselines on a 100 KB body.
- **Coverage CI** (`ci-coverage.yaml`) — cargo-llvm-cov soft-warn at
  90 % threshold.
- **Integration test suite expansion** — `test_escalation.rs` (13
  scenarios), `test_waf_detection_integration.rs` (10 WAF + 2xx
  interstitial cases), `test_waf_robustness.rs` (TOML gates + classify
  edge cases), `test_dispatch_robustness.rs` (soft_http × escalation
  interaction), `test_waf_hot_reload.rs`, `test_dispatch_types.rs`
  (serde round-trip), `test_async_borrow.rs` (Send/Clone proofs).

### Changed

- **`CrawlError::WafBlocked`** is now a struct variant —
  `WafBlocked { vendor: String, message: String }` (was tuple).
  `CrawlError` gains `#[derive(Clone)]` to allow owned `AttemptOutcome`.
- **`WafClassifier::classify`** now returns
  `Result<Option<WafSignal>, WafClassifyError>` (was `Option<WafSignal>`)
  so misconfigured classifiers surface as distinct errors.
- **`DomainStatePort`** refactored to an observation-based model:
  `recommend(domain) -> DomainRecommendation` and
  `observe(domain, observation)` replace the previous
  `get` / `record_outcome`. Backends that prefer non-EWMA semantics
  (Redis, rule-based, ML) can implement the port without forced EWMA.
  Default impl renamed `InMemoryDomainState` → `EwmaDomainState`.
- **`SimpleRetryPolicy` off-by-one fix** — `max_retries=3` now yields
  3 retries as documented (was 2).
- **`LearningRetryPolicy`** suppresses recording on permanent
  non-WAF errors (DNS, SSL, Connection, InvalidConfig, Unsupported,
  NotFound, Unauthorized, Gone, DataLoss, BrowserError, BrowserTimeout)
  to keep domain EWMA from being polluted by unreachable hosts.
- **`EwmaDomainState::observe`** uses a snapshot-compute-write pattern
  that holds the DashMap shard lock only across the write, not across
  the EWMA math.
- **`DomainRecommendation::confidence`** changed from `f32` to
  `Option<f32>`. `None` means "no opinion" — backends that do not
  produce a probability (rule-based Redis allowlists, etc.) no longer
  have to invent a numeric value. `EwmaDomainState` returns
  `Some(value)` once it has samples, `None` until then.
- **`#[non_exhaustive]`** added to `Tier`, `EscalationStrategy`,
  `EscalationReason`, `ObservedOutcome`, `RetryDirective`,
  `WafClassifyError`, `NetworkErrorKind`, and `CrawlError`. Future
  variants can be added without a breaking change for downstream
  match arms.
- **Default impl re-exports** — `EwmaDomainState`, `EwmaTracker`,
  `FixedBudget`, `LearningRetryPolicy`, `SimpleRetryPolicy`,
  `UnlimitedBudget`, and convenience constructors
  `default_retry_policy`, `unlimited_budget`,
  `in_memory_domain_state` are now reachable from the crate root.
  All carry `#[cfg_attr(alef, alef(skip))]` because their internals
  (`AtomicU32`, `DashMap`, `Arc<dyn Trait>`) are not FFI-safe.

### Fixed

- **Engine wires `WafClassifier`** at both the success and error arms.
  Previously the classifier was configured on `CrawlConfig` but never
  called by the dispatch loop, so WAF-driven escalation never fired.
- **Engine respects `max_total_attempts`** — a buggy `RetryPolicy`
  that returns `Retry` forever can no longer spin; the dispatcher
  force-returns after the global cap (default 10).
- **`Tier::Bypass + EscalationStrategy::BypassFirst`** now has an
  explicit `None` arm in `next_tier` instead of the previous
  catch-all that silently swallowed the strategy.
- **WAF body fingerprints** restored — `"request blocked"`,
  `"challenge.js"`, `"please verify you are human"` were dropped during
  the original TOML extraction; refit alongside ≥3 new fixtures.
- **`aws_cloudfront_server`** fingerprint dropped — CloudFront is a
  CDN, not a WAF; firing on every CDN-fronted site produced false
  positives.
- **`TomlClassifier::watch` hot-reload on Linux/Kubernetes** —
  canonicalize is now performed lazily inside the event closure.
  An eager canonicalize at setup silently fell back to the
  un-resolved path when the target did not yet exist, and the
  closure compared against the un-resolved path while inotify
  delivered the resolved path, so reload events were dropped.
  Kubernetes ConfigMap atomic symlink projection also went silent
  because each rotation broke the captured canonical inode.
  The closure now matches three ways (exact, canonical, file name
  inside the watched parent directory) so both cases work.
- **`WatchHandle` drop** — the debounce task `JoinHandle` is held
  on the handle and aborted from `Drop` as a backstop after the
  cooperative shutdown signal fires; a panicked task can no longer
  silently stop reloads while leaving the handle alive.
- **Engine error-path `WafSignal`** — the synthesized
  `fingerprint_id` is now empty (was `"from_error"`). The vendor
  field carries attribution; the empty string avoids a phantom
  Prometheus label cardinality if a downstream consumer reads
  `outcome.waf_signal.fingerprint_id`.

## [Earlier]

### Changed

- **Split pub.dev publish into a dedicated `publish-pubdev.yaml` workflow
  triggered by `push: tags: v*`.** pub.dev OIDC trusted publishing rejects
  tokens from `release` events; only `push` and `workflow_dispatch` events
  produce accepted tokens. The new workflow builds the Dart package and
  publishes independently. **One-time setup required:** configure pub.dev →
  kreuzcrawl package → Admin → Automated publishing with workflow path
  `.github/workflows/publish-pubdev.yaml`.
- **Regenerated all alef bindings against `alef 0.17.8`** (was `0.17.2`). Two
  behaviour changes flow through to the per-language bindings:
  - Rustdoc intra-doc links of the form `[Type::method]` are now emitted as
    plain `Type.method` in non-Rust binding source comments. The previous
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

### Fixed

- **The multi-page crawl loop now materializes discovered non-HTML documents
  (PDF, DOCX, …) into `CrawlPageResult.downloaded_document`.** The
  `download_documents` config flag was previously honored only by the
  single-page `scrape()` path; the crawl loop fetched a linked PDF, flagged
  the page `was_skipped`/`is_pdf`, then discarded the bytes and left
  `downloaded_document` hardcoded to `None`. Crawling a site for its linked
  documents therefore yielded zero documents. The crawl loop now preserves
  the raw response bytes through the fetch task and builds the
  `DownloadedDocument` via a shared `build_downloaded_document` helper used by
  both the scrape and crawl paths, so the two cannot diverge. Skipped (binary)
  pages also no longer run HTML→markdown conversion on their lossy-UTF-8 body.

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
