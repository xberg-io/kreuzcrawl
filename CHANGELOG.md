# Changelog

All notable changes to kreuzcrawl are documented here.

## [Unreleased]

## [0.3.0-rc.86] - 2026-06-22

### Build

- Regenerated all bindings against **alef 0.25.60** (pin bumped from 0.25.59). Notable codegen fixes: the generated Rust e2e/test-app `common.rs` now resolves the mock-server binary via `env!("CARGO_BIN_EXE_mock-server")` instead of a hardcoded `target/release/mock-server` path (so `cargo test` debug builds spawn it correctly), and the Kotlin Android `MockServerListener` now parses the `MOCK_SERVERS` env map on the preset path so per-fixture `mockServer.<id>` lookups resolve under the registry-mode test runner.

## [0.3.0-rc.85] - 2026-06-21

### Fixed

- **Dart pub.dev publish no longer requires Flutter to build natives.** The per-platform native dylib build used `build-dart-package`, which installs Flutter via `subosito/flutter-action`; Flutter ships no Linux ARM64 stable SDK, so the `linux-arm64` leg failed ("Unable to determine Flutter version … architecture: arm64") and blocked the entire pub.dev publish. The native build needs only Rust (`frb_generated.rs` is committed), so the matrix now builds each native with `cargo build --locked -p kreuzcrawl-dart --release` directly — matching the canonical liter-llm / kreuzberg pattern. (`.github/workflows/publish-pubdev.yaml`)

### Build

- Regenerated all bindings against **alef 0.25.59** (pin bumped from 0.25.55). Notable codegen changes: the Swift `RustBridgeC.h` is now the full concatenated swift-bridge C header instead of a placeholder, and the JNI `NativeLib` throws a descriptive `ExceptionInInitializerError` naming the missing native symbol instead of a bare `orElseThrow`.
- Supersedes rc.84, whose Homebrew bottle-merge and release-finalize steps were stranded by a transient crates.io fetch flake in the `x86_64_linux` bottle build (leaving the tap formula's `bottle do` block pinned to the rc.83 root URL).

## [0.3.0-rc.84] - 2026-06-20

### Added

- **CLI: `batch-scrape`, `batch-crawl`, `download`, `citations`, and `version` subcommands.** Wire the existing core entry points into the CLI so the CLI, MCP server, and core surfaces are 1:1. `download` mirrors the MCP download tool (scrape with document download enabled, emitting the downloaded document's metadata); `citations` converts markdown links to numbered citations; `version` prints the crate version as JSON. (`crates/kreuzcrawl-cli`)
- **MCP: `batch_crawl` and `generate_citations` tools.** `batch_crawl` crawls multiple seed URLs concurrently (mirroring `batch_scrape`); `generate_citations` converts markdown links into numbered citations. (`crates/kreuzcrawl/src/mcp`)

### Changed

- **chore(precommit): drop the conflicting kotlin-android ktlint hook; ktfmt is the sole formatter.** ktlint's always-format mode fought ktfmt (blank-line-after-brace) and rewrote alef's `///` doc comments to `// /`, breaking `alef verify`. detekt remains for static analysis. Also excluded the vendored Gradle wrapper from shellcheck. (`.pre-commit-config.yaml`)

### Removed

- **MCP: dropped the unimplemented `screenshot`, `research`, and `crawl_status` tools.** They had no backing core capability and only ever returned "not yet implemented", advertising tools that always failed. Every remaining MCP tool is now 1:1 with a CLI subcommand and a real core function. (`crates/kreuzcrawl/src/mcp`)

### Fixed

- **Release: CLI binaries are now reliably attached to a published release.** The GitHub release is created as a draft and was only un-drafted by `release-finalize`, which is skipped whenever any unrelated publish leg fails — stranding the whole release (CLI binaries included) as an invisible draft. `upload-cli-release` now publishes the release directly via `publish-github-release@v1` with `draft: false` (un-drafting the existing draft) and runs even on partial CLI-build failures, mirroring the sibling repos. (`.github/workflows/publish.yaml`)
- **Memory-bounded streaming crawl.** `crawl_stream` / `batch_crawl_stream` now move each page into its `CrawlEvent::Page` and drop it instead of accumulating every page, bounding peak memory on large crawls (≈2.5 GB → ≈20 MB working set). `crawl()`'s batch result is unchanged; `Complete.pages_crawled` reports the exact post-filter count, and a terminal `Complete` is emitted on the seed-error path. (`crates/kreuzcrawl/src/engine`)
- **Dart package ships native libraries.** The pub.dev package previously bundled no native libs, so the flutter_rust_bridge loader fell back to a relative framework path that macOS hardened-runtime rejects. The publish pipeline now builds the native on a 5-platform matrix and stages each into `lib/src/native/<rid>/` before publishing. (`.github/workflows/publish-pubdev.yaml`)
- **Swift package links Apple system frameworks.** The published root `Package.swift` now links `Security`, `CoreFoundation`, and `SystemConfiguration` on the `RustBridge` target so the pre-built static library's `SC*` symbols (reqwest proxy detection) resolve for remote SwiftPM consumers. (regenerated via alef 0.25.55)

### Build

- Regenerated all bindings against **alef 0.25.55** — generic Go FFI provisioning (test-app runner delegates to the binding's own `download_ffi`; no project-specific download in the generic generator), swift framework linking, zig stale-hash strip, and test-app `[crates.e2e.env]` export.

## [0.3.0-rc.80] - 2026-06-19

### Changed

- **Bump alef pin 0.25.43 → 0.25.47** and regenerate all bindings.

### Fixed

- **Java (Panama FFM) crashed on the first request.** The `register_route` downcall descriptor omitted the C context parameter and the upcall adapter mis-read the request pointer; both are corrected, so route registration and request marshalling work end-to-end. (alef 0.25.47)
- **PHP ignored function-path serde defaults.** `CrawlConfig` from JSON now honors field defaults such as `SsrfPolicy::from_env()` when `ssrf` is omitted, matching the core and the other bindings instead of falling back to deny. (alef 0.25.47)
- **Kotlin e2e:** compare nullable `Boolean?` assertions explicitly so safe-call field paths compile. (alef 0.25.47)
- **Dart:** cfg-gate generated Rust-bridge items that reference feature-gated core types, and fix slice/`&Path`/`&BTreeMap`/`serde_json::Value` parameter handling so the Flutter Rust bridge crate compiles. (alef 0.25.46–0.25.47)
- **JNI:** marshal `Option<&[u8]>` and `&BTreeMap` parameters to the core's declared types. (alef 0.25.44)
- **SSRF:** allow private networks on the `wasm32` target and respect `KREUZCRAWL_ALLOW_PRIVATE_NETWORK` on native.
- **Elixir Hex publish** is gated on a complete NIF upload so a flaky NIF leg cannot ship an immutable package with an incomplete checksum file.
- **Swift `Package.swift`** receives the real artifactbundle SHA256 in the release tag instead of a placeholder, unblocking SwiftPM consumers.
- **Test-app harness:** resolve the published Go module directory, rebuild the Dart native to the published content hash, and set the SSRF private-network override at process level for C#/Elixir.

## [0.3.0-rc.79] - 2026-06-18

### Changed

- **Bump alef pin 0.25.41 → 0.25.43** and regenerate all bindings.
- **Align `task alef:format` output with the prek formatter hooks** so committing generated code leaves `ci-lint` clean:
  - node/ts/json: format git-tracked sources with `oxfmt` (matches the `oxfmt` hook; a raw `.` walk had choked on `target/`).
  - kotlin: format with `ktfmt --kotlinlang-style` directly (was gradle ktlint, which prek then reformatted).
  - zig: include the package-root `build.zig` (was `src/` only).
  - java: removed the outlier `palantir-java-format` prek hook; alef's eclipse/spotless scaffold (`spotless:check` via `java-verify`) is the single canonical java formatter.

## [0.3.0-rc.78] - 2026-06-18

Supersedes rc.77, whose CI failed on a `clippy::unused_unit` error in generated Swift glue (fixed upstream in alef 0.25.41).

### Fixed

- **SSRF config robustness.** Tolerate partial JSON SSRF config and apply the env override at the engine layer; normalize an empty `scheme_allowlist` at engine build.
- **Docker Alpine build.** Rewrite workspace members correctly so the Alpine image builds.
- **Generated Swift compiles under `clippy -D warnings`.** The swift-bridge destructor-synthesis noop shims no longer emit an explicit `-> ()` (`clippy::unused_unit`). (alef 0.25.41)

### Changed

- **Bump alef pin 0.25.32 → 0.25.41.** Notable codegen fixes pulled in:
  - java: boxed `@Nullable Boolean` serde-default fields restore their `true` default in the record compact constructor.
  - napi: `Map`-returning functions wrap their return value and compile.
  - pyo3: None-guard optional `Vec<enum>` coercion; drop a redundant `# type: ignore`.
  - php: dedup cfg-variant free functions; `@return array<T>` PHPDoc on array-returning DTO methods.
  - core: enum associated (static factory) methods are now emitted across bindings.

## [0.3.0-rc.76] - 2026-06-17

### Added

- **Public substrate parsers.** `kreuzcrawl::robots` and `kreuzcrawl::sitemap` are now `pub mod`. `parse_robots_txt`, `is_path_allowed`, `RobotsRules` (and its fields), `parse_sitemap_xml`, `parse_sitemap_index`, `is_sitemap_index` are public — usable from out-of-crate code without spinning the engine. Async fetch helpers remain crate-internal (they rely on engine HTTP/config). Substrate-only integration test at `crates/kreuzcrawl/tests/substrate_only_crawl.rs` locks in the acceptance criterion: a developer can crawl a small site with only kreuzcrawl, no `kreuzberg-cloud` or `crawl-traits` deps.
- **`ProxyProvider` trait + `StaticProxyProvider` baseline.** New extension point for per-request proxy rotation on the reqwest HTTP fetch path. Cloud impls (e.g. `BrightDataProxyProvider`) plug in via `CrawlEngineBuilder::with_proxy_provider(Arc<dyn ProxyProvider>)`. Wired into `http::build_client` as `reqwest::Proxy::custom` — provider is called per request with the target host; returning `None` short-circuits to a direct connection. Takes precedence over the static `CrawlConfig::proxy` value; browser-backend proxies (`config.browser.proxy`) still read the static value (per-request browser-relaunch is out of scope). Provider field on `CrawlConfig` is `#[serde(skip)]` + `#[cfg_attr(alef, alef(skip))]` so language bindings are unaffected.

### Changed

- **README: substrate-vs-operational section.** Documents the boundary between kreuzcrawl (substrate: parsers, fetchers, classifiers, baseline trait impls) and `kreuzberg-cloud` (productization: paid IP rotation, tuned WAF fingerprints, authenticated sessions, premium scrapers, scheduling). Lists the existing trait extension points (`Frontier`, `RateLimiter`, `CrawlStore`, `EventEmitter`, `ContentFilter`, `CrawlCache`, `WafClassifier`, `BypassProvider`, `AntibotStrategy`) with their kreuzcrawl baselines and cloud reference impls.
- **Bump alef pin to 0.25.32.** Picks up cross-binding fixes:
  - `codegen/binding_helpers`: per-field `Default` fallback when the core type lacks a `Default` impl (alef 0.25.31). Previously the lossy generator emitted `field: Default::default()` for `binding_excluded` fields even when the core struct had no `Default` impl, generating uncompilable bindings for affected types.
  - `java-backend`: targeted javadoc + suppressions reducing PMD violations from 389 to 22 (alef 0.25.31).
  - `scaffold/java`: exclude `FinalFieldCouldBeStatic` from the PMD ruleset (alef 0.25.32) — was flagging generated record-style data classes that PMD wanted as static finals.
  - `backends/pyo3/gen_stubs`: widen visitor kwarg type to accept duck-typed classes (alef 0.25.31).

## [0.3.0-rc.75] - 2026-06-17

### Fixed

- **Restore `crates/kreuzcrawl-ffi/{src/lib.rs,build.rs,cbindgen.toml}` dropped from rc.74.** rc.74's release commit deleted 9937 lines from the FFI crate, leaving the published `kreuzcrawl-ffi` v0.3.0-rc.74 with an empty `src/` and a `[lib] name = "kreuzcrawl_ffi"` declaration that pointed at nothing — every `Build FFI` matrix cell failed with `can't find library `kreuzcrawl_ffi`, rename file to `src/lib.rs``. Root cause: a partial `alef generate --lang X --clean` invocation in the rc.74 prep session wiped all alef-managed files first and only restored a subset; the FFI files were missed during manual recovery and committed in their deleted state. Full `task alef:generate` (which runs `alef all --clean --format=false`) regenerates them correctly.
- **kotlin-android type regen.** `AuthConfig`, `BrowserConfig`, `BatchCrawlStreamRequest`, and others picked up shape changes from the same regen pass; rc.74 shipped stale generated code.
- **packages/elixir/mix.exs:** include `lib` directory in Hex package `files` list. Previously the published gem omitted the Elixir wrapper module.

## [0.3.0-rc.74] - 2026-06-17

### Added

- **`SsrfPolicy` exposed as a DTO in every binding (Phase 1).** `deny_private` and `max_redirects` are now settable from all 14 language bindings. `allowlist` (requires `HostMatcher` FFI form decision) and `scheme_allowlist` (`HashSet<&'static str>` is FFI-hostile) remain `alef(skip)` for a follow-up pass. WASM e2e suite unblocked: WASM lacks `std::env::var`, so `SsrfPolicy::from_env()` always returned `deny_private=true`; generated WASM e2e tests now set `ssrf.denyPrivate = false` on every engine config directly.

### Changed

- **Bump alef pin to 0.25.29.** Picks up cross-binding fixes that the previous rc.73 pin (0.25.24) lacked:
  - `codegen/binding_helpers/lossy_fields.rs` + `backends/php/gen_bindings/helpers/enum_defaults.rs`: skip `binding_excluded` fields in the shared method-body lossy helper AND the PHP enum-tainted From generator. Previously emitted `<field>: Default::default()` for every alef(skip) field BEFORE the trailing `..Default::default()` spread, shadowing `kreuzcrawl::CrawlConfig::default()`'s custom `ssrf: SsrfPolicy::from_env()` with `<SsrfPolicy as Default>::default()` (deny_private=true). This broke `KREUZCRAWL_ALLOW_PRIVATE_NETWORK` for every binding's `Kreuzcrawl::validate()` and PHP's `crawl()`/`scrape()`/etc. PHP redirect cluster (9 tests) and PHP SSRF cluster (~45 errors) both stop failing.
  - `e2e/codegen/typescript/test_file/args.rs`: WASM e2e arg builder now injects `ssrf.denyPrivate = false` on every engine config (including null-config paths that previously skipped engine instantiation). Unblocks 155 WASM e2e tests.
  - `backends/kotlin_android`: add `ReturnCount` to generated `@file:Suppress` list. Untagged-enum (serde) sealed-class deserializers with 3+ variants (e.g. `HostMatcher` newly exposed by SsrfPolicy) now pass detekt.

### Fixed

- **(net/ssrf): lowercase `SsrfPolicyViolation::DnsResolutionFailed` message.** Was `"DNS resolution failed: ..."` (Title Case); other variants in the same enum (`"host not on allowlist"`, `"invalid URL"`, `"disallowed scheme"`) use lowercase. The Rust e2e `test_error_dns_resolution` failed because the fixture redirects to `.invalid`, which short-circuits via SSRF DNS preflight rather than reaching `NetworkErrorKind::Dns` classification — so the surfaced string was the SSRF reason verbatim, with capital `DNS`. Lowercasing aligns with the lowercase keywords used by `error_chain_string()`. Node/Python tests passed coincidentally — their alef generators emit only the value-less first assertion (`expect(...).rejects.toThrow()` / `pytest.raises(Exception)`) and silently drop the second `{type: "error", value: "dns"}` assertion. Rust correctly emits both, so the string mismatch surfaced there first.
- **Regression test: `max_depth=1` returns root + children (rc.62 regression).** Added `test_max_depth_one_returns_root_plus_children` to `crates/kreuzcrawl/tests/test_frontier_dedup.rs`. This locks in the behavior fixed in rc.71 (`#[serde(default = "SsrfPolicy::from_env")]`): a `CrawlConfig` deserialized from JSON with `max_depth=1` and `KREUZCRAWL_ALLOW_PRIVATE_NETWORK=true` must return root + all depth-1 children (3 pages). Prior to rc.71, `#[serde(default)]` on `CrawlConfig.ssrf` called `SsrfPolicy::default()` (deny_private: true), causing `discover_and_enqueue_links` to SSRF-reject every loopback child URL and return only 1 page. The Zig e2e test `cache_miss_fresh_fetch` covers the same invariant end-to-end.

## [0.3.0-rc.73] - 2026-06-17

### Changed

- **Bump alef pin to 0.25.24** to land cross-language consumer fixes batched between rc.72 and rc.73:
  - `e2e/codegen/dart`: `_setEnv` helper now passes `setenv(key, value, 1)` (force overwrite) and throws on non-zero return. Previously `overwrite=0` silently no-oped when the env var was pre-set, making `KREUZCRAWL_ALLOW_PRIVATE_NETWORK=true` invisible to the Rust FFI dylib's `std::env::var()` and failing all 166 dart e2e SSRF tests with `denied by SSRF policy: loopback`.
  - `e2e/codegen/wasm`: `render_setup` now emits the `[crates.e2e.env]` block in generated `setup.ts` (was omitted when the env feature landed for all other languages in be2426f18), unblocking the 155-test wasm e2e suite that failed identically to dart.
  - `e2e/codegen/brew`: `run_tests.sh` preflight requires the parameterized binary name only (`command -v kreuzcrawl`) instead of OR'ing with the parent project's sibling CLI (`kreuzberg`), which previously short-circuited to OK when the sibling was installed.
  - `e2e/codegen/c`: `download_ffi.sh` honors `KREUZCRAWL_FFI_LOCAL_DIR` env override to skip the GH release curl when CI stages a locally-built FFI artifact (race-free on tag-push).
  - `backends/java`: NativeLib loader RID logic aligned to `go_java_platform()` naming (`macos-arm64`, not `osx-aarch64`) so the published JAR's staged natives load on macOS-arm64 clients.
  - `scaffold/elixir`: rustler NIF Cargo.toml `[features]` block is now parameterizable via `[crates.elixir] nif_features`; set to `[]` for kreuzcrawl so the NIF no longer forwards nonexistent `config/download/serde` features to the core crate.
  - `backends/rustler`: emit blank lines between consecutive `defp` clauses so `mix format --check-formatted` passes on the generated `packages/elixir/lib/kreuzcrawl.ex`.

### Fixed

- **(ci-e2e workflow): stage FFI header at `packages/go/internal/ffi/` alongside `packages/go/include/`.** The hand-written `packages/go/ffi.go` (non-windows split) declares `#cgo CFLAGS: -I${SRCDIR}/internal/ffi` and `#include "internal/ffi/kreuzcrawl.h"`, while the alef-generated `packages/go/binding.go` uses `packages/go/include/`. Both compile together so cgo needs the header at both paths; staging only `packages/go/include/` caused `fatal error: internal/ffi/kreuzcrawl.h: No such file or directory` and broke the go E2E job. Stage to both.
- **(ci-e2e workflow): remove workflow-level `KREUZCRAWL_ALLOW_PRIVATE_NETWORK=true`.** The per-language e2e harness already injects this from alef.toml `[crates.e2e.env]`, so the workflow override was redundant — but it forced the env into the hand-written SSRF rejection tests (`e2e/node/tests/ssrf.test.ts`, `e2e/rust/tests/ssrf_test.rs`) which need to observe default `deny_private=true` policy. Drop the workflow line; hand-written rejection tests now explicitly clear the env at start of test so they're correct regardless of harness state.
- **(publish workflow): add `upload-elixir-release`, `upload-php-pie-release`, `upload-swift-release` to `release-finalize` and `announce-discord` needs.** Previously these jobs weren't in the `needs:` chain, so the `!contains(needs.*.result, 'failure')` gate didn't observe their failures — Finalize could complete or skip incorrectly. Tighten the chain so any upload failure now blocks Finalize.

## [0.3.0-rc.72] - 2026-06-16

### Changed

- **Bump alef pin to 0.25.22** to pick up three downstream consumer-blocking fixes surfaced while running `task test-apps:smoke` against published rc.71 bindings:
  - `backends/go::download_ffi`: self-truncating dylib copy fixed; pre-fix, the published kreuzcrawl-go module wrote a 0-byte `libkreuzcrawl_ffi.dylib` to the module cache and every cgo link failed with `ld: file is empty`.
  - `bin_cli::all`: `v__ALEF_SWIFT_VERSION__` placeholder is re-applied after scaffold emission so SwiftPM consumers using `.package(url:..., from:"0.3.0-rc.72")` resolve the binaryTarget URL correctly (rc.71's `Package.swift` at the v0.3.0-rc.71 git tag retained the unsubstituted placeholder, breaking the swift_e2e smoke).
  - `e2e/codegen/brew::run_tests`: emits a CLI preflight check so the absence of a formula install surfaces as one `brew install …` instruction rather than a cascade of `command not found` lines from every category script.
- **Add `kotlin_android` to `[crates.e2e].languages` and `[crates.e2e.registry.packages]`.** rc.60 to rc.71 saw no regeneration of `test_apps/kotlin_android/` because registry mode silently skipped languages not listed under `[crates.e2e].languages`, leaving the directory stale and missing `settings.gradle.kts`. Without that file Gradle defaults to the Plugin Portal alone, where AGP 8.13.0 isn't published — `Plugin [id: 'com.android.library', version: '8.13.0'] was not found`. Fresh registry regen against alef 0.25.22 emits the complete kotlin_android consumer project including settings.gradle.kts with the google() / mavenCentral() / gradlePluginPortal() chain.

### Fixed

- **(.task/tools/test-apps.yml smoke:go) materialize the Go FFI dylib before running tests.** The published `kreuzcrawl-go` module includes `cmd/download_ffi/main.go` (tagged `//go:build ignore`) to fetch the prebuilt `libkreuzcrawl_ffi.{dylib,so,dll}` from GitHub Releases. The smoke task previously did `go mod download` then `go test` directly, skipping the FFI download — link failed with `ld: library 'kreuzcrawl_ffi' not found`. The task now resolves the module directory via `go list -m -f '{{.Dir}}'`, chmods it writable, and invokes the file path directly so the `//go:build ignore` constraint doesn't block it.
- **(.task/tools/test-apps.yml smoke:dart) set `FRB_DART_LOAD_EXTERNAL_LIBRARY_NATIVE_LIB_DIR` for the in-tree dylib path.** The pub.dev published Dart tarball does not bundle the native library; the frb_generated loader's default search falls through to a relative path that macOS hardened runtime rejects. Setting the env var to `packages/dart/lib/src/native/<rid>/` per platform points the loader at the in-tree build so test_apps smoke runs without modifying the loader.
- **(carry-over from rc.72 staging) `SsrfPolicy` JSON round-trip dropped the scheme allowlist** — surfaced as the brew CLI failure mode in rc.71 CI E2E run 27598534040: every URL rejected with `disallowed scheme: http`. `crates/kreuzcrawl/src/net/ssrf.rs` declared `scheme_allowlist: HashSet<&'static str>` with `#[serde(skip)]`. Deserialization uses the field type's `Default` for skipped fields, which for `HashSet` is an empty set. `crates/kreuzcrawl-cli/src/main.rs::merge_json_config` serializes the live `CrawlConfig`, merges, and deserializes back — that round-trip clobbered `scheme_allowlist` to empty and SSRF validation rejected every URL regardless of host. Fix: add a named-fn deserialization default `#[serde(skip, default = "default_scheme_allowlist")]` that populates `http`/`https`. Regression test `ssrf_policy_json_round_trip_preserves_scheme_allowlist` covers the failure mode directly.
- **(carry-over from rc.72 staging) WASM build: `engine/builder.rs` still imported `crate::sink::EventSink` ungated.** rc.71 only gated `pub mod sink;` and the public re-export in `crates/kreuzcrawl/src/lib.rs` (lines 43, 85). `crates/kreuzcrawl/src/engine/builder.rs:7` still carried `use crate::sink::EventSink;` without a cfg gate, plus three unguarded references: the `event_sink: Option<Arc<dyn EventSink>>` field, the `Self { event_sink: None, .. }` initializer in `new()`, and the `event_sink()` builder method. `wasm-pack build` repeated the same `error[E0432]: unresolved import 'crate::sink'` on rc.71 Publish Release. Fix: wrap all four sites in `#[cfg(not(target_arch = "wasm32"))]`. `build()`'s own `event_sink` assignment was already correctly gated.

### Notes

- **Zig smoke 4/15 still failing on asset fixtures** (`scrape_asset_dedup`, `scrape_asset_max_size`, `scrape_asset_type_filter`, `scrape_download_assets`). The C test_app exercises the same fixtures via `kcrawl_scrape_result_assets()` and passes; the zig test_app uses `kcrawl_scrape_result_to_json()` and parses the result with `std.json`. Logged for follow-up — likely a zig-specific JSON shape mismatch rather than a Rust core regression. Will surface in the rc.72 post-publish smoke pass; not blocking release.

## [0.3.0-rc.71] - 2026-06-16

### Fixed

- **`CrawlConfig` JSON deserialization ignored `KREUZCRAWL_ALLOW_PRIVATE_NETWORK`** — fourth regression in the rc.62 SSRF-defense cluster (commit 277ef16f6). `crates/kreuzcrawl/src/types/config.rs:405` declared `#[serde(default)] pub ssrf: SsrfPolicy,`. `#[serde(default)]` calls `<SsrfPolicy as Default>::default()`, which hardcodes `deny_private: true` at `crates/kreuzcrawl/src/net/ssrf.rs:121`. It does *not* call `SsrfPolicy::from_env()`. Every binding that round-trips a CrawlConfig through `kcrawl_crawl_config_from_json` (FFI: Go, Java, C#, PHP, Ruby, Elixir, Dart, Swift, Zig, WASM, C, brew, rust) omits the alef-skipped `ssrf` field, so serde applied the field-level default and the env-var override never took effect. Python and Node escape this path because their PyO3 / NAPI `From<CrawlConfig> for kreuzcrawl::CrawlConfig` impls start with `kreuzcrawl::CrawlConfig::default()` (which *does* call `SsrfPolicy::from_env()` per `types/config.rs:476`) and overwrite individual fields. Fix: change the attribute to `#[serde(default = "SsrfPolicy::from_env")]`. CI E2E rc.70 push run 27580653877 surfaced this as the same 13 ssrf-policy-violation failures Python/Node never hit; the diagnosis was masked through rc.62 → rc.70 by the env-var rollout signal noise and by overlap with the other three 277ef16f6 regressions. Regression test added in `crates/kreuzcrawl/src/net/ssrf.rs::crawl_config_json_deserialize_honors_env_var` covering both the env-set and env-unset paths for `serde_json::from_str::<CrawlConfig>("{}")`.
- **WASM build: gate `sink` module on non-wasm targets.** `crates/kreuzcrawl/src/sink.rs` (added in rc.66, commit 4282603d7) unconditionally does `use crate::types::CrawlEvent;` at line 9. `CrawlEvent` is `#[cfg(not(target_arch = "wasm32"))]` because `CrawlPageResult` and the streaming machinery aren't wasm-compatible. `crates/kreuzcrawl/src/lib.rs` exposed `pub mod sink;` (line 43) and `pub use sink::{EventSink, MultiEventSink, TracingEventSink};` (line 85) without cfg gates, so `wasm-pack build` tripped `error[E0432]: unresolved import 'crate::types::CrawlEvent'`. Observed on rc.70 Publish Release Build WASM job 81540455790. The engine consumer at `engine/mod.rs:51-52` was already correctly gated, so the fix is just to wrap both `pub mod sink;` and the public re-export in `#[cfg(not(target_arch = "wasm32"))]`.

### Changed

- **Regenerate against alef 0.25.17** (pin unchanged from rc.70). Pure binding regen for the two Rust source fixes above.

## [0.3.0-rc.70] - 2026-06-16

### Fixed

- **Child crawl depth not incremented (rc.62 regression).** Sibling fix to rc.69's within-batch dedup race — both regressions originated in commit 277ef16f6 (`feat(security): SSRF defense in core HTTP layer`). When `discover_and_enqueue_links` was refactored into the two-phase candidate/SSRF-validate pipeline, the line `let child_depth = depth + 1;` was dropped. Child entries inherited the parent's depth verbatim, so every URL ran at `entry.depth = 0` regardless of how deep into the tree it lived. Two downstream consequences both surfaced in rc.69 CI E2E run 27569666762 across all 15 language suites (`tests/test_crawl.py:65: assert 5 == 3`, etc.): (1) `max_depth` guard `depth < max_depth` is never reached — `crawl_concurrent_depth` fixture with `max_depth=1` returned 5 pages (root + 2 children + 2 grandchildren) instead of 3 (root + 2 children); (2) the `include_paths` filter in `should_fetch_url` is gated on `entry.depth > 0` (the seed URL is always included regardless of pattern), so with all entries at depth 0 the filter never applied — `crawl_include_path_pattern` fixture with `include_paths=["/blog/.*"]` returned 3 pages (root + `/blog/post1` + `/about`) instead of 2. Fix: restore `let child_depth = depth + 1;` in the candidate loop of `discover_and_enqueue_links` and push `child_depth` (not the parent's `depth`) into `candidates`. Phase 2's consumer already destructures the tuple as `child_depth` and writes it to `FrontierEntry.depth`, so the consumer path is unchanged. Verified end-to-end: `cargo test -p kreuzcrawl --tests --no-fail-fast` with `KREUZCRAWL_ALLOW_PRIVATE_NETWORK=true` passes 192 unit + all integration tests with zero failures. Together with rc.69's within-batch dedup fix, this completes the cleanup of the 277ef16f6 regression cluster that masked itself across rc.62 → rc.68 by overlapping with the SSRF env-var rollout signal noise.

### Changed

- **Regenerate against alef 0.25.15** (pin unchanged from rc.68/rc.69). Pure binding regen for the depth-fix Rust source change.

## [0.3.0-rc.69] - 2026-06-15

### Fixed

- **Within-batch crawl URL dedup race.** rc.62's `feat(security): SSRF defense in core HTTP layer` (commit 277ef16f6) refactored `discover_links` (`crates/kreuzcrawl/src/engine/crawl_loop.rs`) into a two-phase pipeline — phase 1 collects link candidates after `is_seen` check; phase 2 runs concurrent SSRF validation, then `mark_seen`. The race: within a single `discover_links` call, three anchors `/page`, `/page#section`, `/page?ref=home` all normalize via `normalize_url_for_dedup` to the same key `/page`, but each passes `!is_seen(&dedup_key)` in phase 1 because none has reached phase 2's `mark_seen` yet. All three then enqueue. rc.66/67 CHANGELOG entries fixed a related-but-distinct redirect `final_url` regression from the same commit; the dedup race surfaced separately in rc.68 CI E2E run 27564198654 (all 15 language suites failing the six `test_crawl_*` over-counting fixtures — `crawl_url_deduplication` returned 4 pages instead of ≤2, plus `crawl_fragment_stripping`, `crawl_query_param_dedup`, `crawl_trailing_slash_dedup`, `crawl_concurrent_depth`, `crawl_include_path_pattern`). Fix: move `mark_seen` from phase 2 (post-validation) to phase 1 (candidate-push time) — `is_seen` + `mark_seen` now run as an atomic pair under the same await point in the candidate loop, so the second and third anchors in the batch see the dedup key as already-seen. The post-validation `mark_seen` call in phase 2 is removed (redundant), the binding renamed `_dedup_key` to silence unused warning. Verified against `cargo test -p kreuzcrawl --test test_frontier_dedup` (passes) and the full `kreuzcrawl` test suite (192 unit + all integration tests pass with `KREUZCRAWL_ALLOW_PRIVATE_NETWORK=true`).

### Changed

- **Regenerate against alef 0.25.15** (pin unchanged from rc.68). Picks up the dedup fix in Rust source via standard binding regen; no alef-side changes were needed for rc.69.
- **Remove hand-written `e2e/python/tests/test_ssrf.py`.** The file's own TODO comment marked it for deletion once `validation/validation_ssrf_loopback_denied.json` existed; that fixture now exists (skip-all-languages per rc.67) and the hand-written tests could not coexist with the suite-wide `KREUZCRAWL_ALLOW_PRIVATE_NETWORK=true` setup. SSRF deny behavior remains covered by 35 unit tests in `crates/kreuzcrawl/src/net/ssrf.rs`.

## [0.3.0-rc.68] - 2026-06-15

### Changed

- **Regenerate against published alef 0.25.15.** Picks up the four SSRF env-var emitter fixes (Java Surefire env, Go `cmd.Env` append, Ruby `require 'spec_helper'` reorder, Elixir top-of-file env block with deduplication), the elixir test_helper.exs deduplication, the Dart cfg-strip + Swift `Package.swift` shape fixes, and Rustler clippy lint quieting — all previously consumed via in-flight local alef builds and now anchored to the published 0.25.15 pin. No new behavior beyond rc.67; this RC moves the alef pin from in-flight local to a published version so the CI E2E + Publish Release runs reproduce against a registry-resolvable alef. Known carry-over: the Elixir `interact_async/3` binding fails to `Jason.encode!` action tuples (`{:click, %{...}}`) — needs alef Elixir tagged-enum codegen support; lower priority than the SSRF + redirect rc-blockers and *not* a new regression.

## [0.3.0-rc.67] - 2026-06-15

### Fixed

- **e2e SSRF env-var rollout for Java, Go, Ruby, Elixir.** rc.66 inherited an alef codegen gap where `KREUZCRAWL_ALLOW_PRIVATE_NETWORK=true` was emitted into the test process but didn't reach `SsrfPolicy::from_env()` (which reads libc `std::env::var`) in non-Python suites. Each language had a distinct root cause and gets its own fix in alef e2e codegen: (1) **Java**: previous `System.setProperty(...)` set a JVM system property, not a process env var; JVMs have no public API to mutate libc env mid-process. New emitter wires `<environmentVariables><KREUZCRAWL_ALLOW_PRIVATE_NETWORK>true</KREUZCRAWL_ALLOW_PRIVATE_NETWORK></environmentVariables>` into Maven Surefire's plugin configuration in `e2e/java/pom.xml`, so the variable lands in libc *before* the JVM starts. (2) **Go**: `os.Setenv` in `TestMain` updates the parent's libc env, but the spawned mock-server subprocess uses `cmd.Env` (explicitly set by the test fixture) which previously did not inherit the variable. New emitter appends `KREUZCRAWL_ALLOW_PRIVATE_NETWORK=<val>` to `cmd.Env` immediately before `exec.Command` spawns the mock-server in `e2e/go/main_test.go`. (3) **Ruby**: generated specs `require 'kreuzcrawl'` *before* `require 'spec_helper'`, so the Magnus NIF loaded (snapshotting libc env at module init) before spec_helper set the variable. New emitter reorders the require list so spec_helper loads first; `e2e/ruby/spec/*_spec.rb` now opens with `require 'spec_helper'` then `require 'kreuzcrawl'`. (4) **Elixir**: Rustler NIF loads at `Kreuzcrawl` module init, which fires the first time test_helper.exs references `Kreuzcrawl.*`. The original emitter put the env-var set *below* ExUnit.start and Finch.start_link, so the NIF had already loaded. New emitter prepends an unconditional `unless System.get_env(...) do System.put_env(...) end` block at line 1 of `e2e/elixir/test/test_helper.exs`, *before* the Rustler NIF can load. The duplicate-emitter regression noticed mid-iteration (the env block was emitted twice — once at top, once where it always was) is fixed by removing the legacy in-body emitter; the top-of-file block is the single source.
- **Redirect `final_url` returned base URL after SSRF defense** (carried forward from rc.66 — root commit in this RC's history). rc.62's `feat(security): SSRF defense in core HTTP layer` (commit 277ef16f6) added an internal redirect-following loop inside `do_fetch` (`crates/kreuzcrawl/src/tower/service.rs`) that consumed the entire 3xx chain transparently and returned only the terminal non-3xx body. `follow_redirects` in `crates/kreuzcrawl/src/engine/crawl_loop.rs` — which tracks `current_url` to build the final redirect outcome — never saw a `Location` header again, so `RedirectOutcome::final_url` stayed equal to the original seed URL (just `http://host:port` with no path), which also broke `normalize_url_for_dedup` keyed on `final_url`. Fix: strip the redirect loop from `do_fetch`; it now does SSRF validation of the requested URL and returns the raw response (3xx included). Per-hop SSRF validation moves into `follow_redirects` for HTTP 3xx, Refresh header, and meta-refresh; the `VDom` `Send` violation across `await` is fixed by extracting the meta-refresh target string before the async `validate_url` call. Native tests `test_scrape_redirects`, `test_batch_crawl_integration`, `test_frontier_dedup` gain `allow_private_networks(true)` for their loopback MockServers.

### Changed

- **`fixtures/validation/validation_ssrf_loopback_denied.json` is now `skip: { languages: [] }`** (alef's skip-all-languages mechanism). This fixture asserts that kreuzcrawl returns `ssrf_policy_violation` when called with `http://127.0.0.1:9/`, but every e2e suite now sets `KREUZCRAWL_ALLOW_PRIVATE_NETWORK=true` at suite setup so the loopback mock-server is reachable for *all other* fixtures. With the override on, the deny fixture's assertion fundamentally can't hold. SSRF deny behavior remains covered by 35 unit tests in `crates/kreuzcrawl/src/net/ssrf.rs` (loopback, RFC1918, link-local, cloud metadata, multicast, IPv6 ULA / link-local / multicast, non-http(s) schemes, DNS rebinding via mixed-resolution).
- **Regenerate against alef 0.25.11** (in-flight upstream patching; pin unchanged). Picks up the four SSRF env-var emitter fixes above, the elixir test_helper.exs deduplication, the Dart cfg-strip + Swift Package.swift shape fixes, and Rustler clippy lint quieting. Known carry-over: the Elixir `interact_async/3` binding fails to `Jason.encode!` action tuples (`{:click, %{...}}`) — needs alef Elixir tagged-enum codegen support; lower priority than the SSRF + redirect rc-blockers and *not* a new regression.

## [0.3.0-rc.66] - 2026-06-15

### Fixed

- **Redirect `final_url` returned base URL after SSRF defense.** rc.62's `feat(security): SSRF defense in core HTTP layer` (commit 277ef16f6) added an internal redirect-following loop inside `do_fetch` (`crates/kreuzcrawl/src/tower/service.rs`) that consumed the entire 3xx chain transparently and returned only the terminal non-3xx body. The crawler's higher-level `follow_redirects` driver (`crates/kreuzcrawl/src/engine/crawl_loop.rs`) — which tracks `current_url` to build the final redirect outcome — never saw a `Location` header again, so `RedirectOutcome::final_url` stayed equal to the original seed URL (just `http://host:port` with no path). This broke all redirect-following observability (`/perm-target` never landed in `final_url`) and broke `normalize_url_for_dedup` keyed on `final_url`, so redirect targets were not recognized when re-encountered during a crawl (rc.65 Python E2E run 27540005540 showed `test_redirect_301_permanent` … `test_redirect_chain` failing with `'/target' in 'http://127.0.0.1:34277'`, plus `test_crawl_url_deduplication`, `test_crawl_concurrent_depth`, `test_crawl_fragment_stripping`, `test_crawl_include_path_pattern`, `test_crawl_query_param_dedup`, `test_crawl_trailing_slash_dedup` all overcounting pages). Fix: strip the redirect loop from `do_fetch`, which now does SSRF validation of the requested URL and returns the raw response (3xx included). Per-hop SSRF validation moves into `follow_redirects` for all three redirect types (HTTP 3xx, Refresh header, meta-refresh); the `VDom` `Send` violation across `await` is fixed by extracting the meta-refresh target string before the async `validate_url` call. Native tests `test_scrape_redirects`, `test_batch_crawl_integration`, `test_frontier_dedup` gain `allow_private_networks(true)` against their loopback MockServers — they predate SSRF enforcement and never opted in.

### Changed

- **Regenerate against alef 0.25.11** (in-flight upstream fixes; pin unchanged). Picks up the redirect regression fix above and resets generated bindings/facades/e2e to the canonical surface for rc.66. Known carry-over from rc.65: e2e SSRF env-var only takes effect for Python's `os.environ.setdefault` (libc setenv); Go's `os.Setenv` in `TestMain`, Java's `System.setProperty` (wrong API — JVM properties ≠ libc env), Ruby's `ENV[k]=v` post-`require`, Elixir's `System.put_env` after Rustler NIF load, and Zig/Dart per-test setup don't reach `SsrfPolicy::from_env()` before the first binding call. Per-language fix work is in flight upstream in alef e2e codegen; CI E2E for those suites is expected to fail this RC.

## [0.3.0-rc.65] - 2026-06-15

### Changed

- **Regenerate against alef 0.25.11.** Picks up two rc.64 unblockers and a publish-pipeline regression guard. (1) Per-language `[e2e.env]` emitters: every generated e2e suite (15 languages — python, go, ruby, php, elixir, java, csharp, swift, dart, zig, typescript/wasm, brew, c, rust, kotlin-android) now exports `KREUZCRAWL_ALLOW_PRIVATE_NETWORK=true` at suite-setup time so `SsrfPolicy::from_env()` returns `deny_private: false` against the loopback mock-server. Unblocks rc.64's 15/15 failed CI E2E suites (run 27523705700). The override is declared once in `alef.toml` `[crates.e2e.env]` and translated per-language into the native pre-test env-var idiom (e.g. `os.environ.setdefault` in Python, `os.LookupEnv`/`os.Setenv` in Go's `TestMain`, `System.setProperty` in Java's `@BeforeAll`, `ENV[k] ||= v` in Ruby, `Environment.SetEnvironmentVariable` in a C# `[ModuleInitializer]`, `setenv(..., 0)` in Swift/C, `dart:ffi setenv` in Dart, vitest setup file in WASM, `process.env.X ??=` in TS, `: "${KEY:=val}"` in brew, `.cargo/config.toml [env] force=false` in Rust, `System.put_env` in Elixir). (2) `publish::vendor::scrub_or_regenerate_lock` canonicalizes the manifest path before spawning `cargo` subprocesses with `current_dir(manifest_dir)`. rc.64 publish run 27523709809 failed 38 source-build cells (14 Elixir NIF + Hex publish + 1 Python sdist + 4 Ruby gem + 1 Ruby gems publish + 15 PHP extension + 3 GH release uploads) with `manifest path './path/from/repo/root/Cargo.toml' does not exist` because cargo resolved the original relative `manifest_dir.join("Cargo.toml")` from the new cwd. Canonicalizing first makes the path absolute so cargo resolves it correctly regardless of the spawned process's cwd. (3) `fix(release): generate schema from current sources` (alef 0.25.11) — alef's `task set-version` now regenerates the canonical schema before tagging, so the tagged release's schema matches the source.

## [0.3.0-rc.64] - 2026-06-15

### Changed

- **Regenerate against alef 0.25.9.** Picks up the Elixir NIF scaffold fix: alef 0.25.7's generated NIF `Cargo.toml` emitted an unconditional `[patch.crates-io]` block with `name = { version = "=X" }` entries (no `path`/`git`/`url`), which cargo rejects as `patch for 'alloc-no-stdlib' points to the same source, but patches must point to different sources`. The brotli 8.0.x allocator pin now lives in `[dependencies]` as direct `=X` constraints with matching `[package.metadata.cargo-machete] ignored` entries — cargo's resolver propagates the pinned versions through the whole NIF dep tree without the no-op-patch error. Unblocks rc.63's 14 failed Elixir NIF build matrix cells + Hex publish (observed on run 27510339610). Also picks up alef 0.25.9's `publish/vendor.rs` cwd guard for `scrub_or_regenerate_lock` (cargo child commands now run from the binding crate dir) and the `[e2e.env]` config scaffolding for per-consumer e2e suite-setup environment-variable injection (per-language emitters land in a follow-up alef release; the SSRF env-var injection for CI E2E does not ship in rc.64).

## [0.3.0-rc.63] - 2026-06-14

### Fixed

- **WASM build: cfg-gate `tokio::net::lookup_host` in SSRF defense for `wasm32-unknown-unknown`.** The SSRF policy validator's DNS rebinding mitigation calls `tokio::net::lookup_host` to resolve hostnames before applying the IP allowlist. tokio's `net` feature is not compiled on wasm32 (see `kreuzcrawl/Cargo.toml`'s `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` block), so the SSRF code path failed to compile under `wasm-pack build` with `error[E0425]: cannot find function 'lookup_host' in module 'tokio::net'`. The fix splits `validate_url` into native and wasm32 arms: on native, DNS resolution + per-IP validation runs as before; on wasm32, the function short-circuits after the scheme + literal-IP checks and after the host-string allowlist short-circuit, deferring DNS-rebinding enforcement to the browser/edge runtime's own same-origin and CORS policy (which is the security boundary for outbound requests in those environments anyway). Tested with `cargo check --target wasm32-unknown-unknown -p kreuzcrawl --no-default-features` (green) and the 35 SSRF unit tests on native (all green). Observed on kreuzcrawl rc.62 `Build WASM package` job (run 27507139419).

### Changed

- **Regenerate against alef 0.25.7.** Picks up `publish prepare`: drop `--locked` from final `cargo metadata` validation + env_remove(`CARGO_BUILD_LOCKED`) safety on `cargo update -p` and `cargo metadata` calls, so non-workspace binding crates (Ruby gem NIF, Elixir NIF) can have their missing root `[[package]]` entry written into the seeded lock. Together with alef 0.25.3's `strip_workspace_member_entries`, this should unblock rc.62's 20+ failed Elixir/PHP/Ruby/Python-sdist publish jobs. Also picks up: dart cfg-gated variant catch-all (E0004 prevention), node test_app pnpm-lock regen against the bumped version, go test_app `go mod tidy` post-generation step, swift Package.swift placeholder persistence regression test, and palantir-java-format alignment for Java downcall fallback chains.

## [0.3.0-rc.62] - 2026-06-14

### Changed

- **Regenerate against alef 0.25.4.** Picks up `strip_workspace_member_entries` (alef 0.25.3) — the seeded `Cargo.lock` now drops path-source workspace-member entries before per-member `cargo update -p NAME` runs, eliminating the `specification 'NAME' is ambiguous` (exit 101) errors that failed all 14 Elixir NIF cells, all 16 PHP extension cells, 4 Ruby gem cells, the Python sdist, and the Ruby/Hex publish jobs in rc.61. Also picks up alef 0.25.4's R/extendr keyword-escape fix (no functional impact on kreuzcrawl, which does not ship an R binding).

### Docs

- **Surface antibot and observability deep-dives in navigation.** `docs/antibot.md` joins Guides → Advanced (alongside Browser Automation and WARC Output); `docs/observability.md` joins Guides → Deployment (alongside Docker, API Server, MCP Server). Both were hand-authored content quietly orphaned from the nav since cluster 6 landed.
- **llms.txt sitemap.** SSRF Defense added to the Concepts section; Antibot Strategy and Observability added to Guides; alef added to the Ecosystem block (GitHub URL per `kreuzberg-brand-and-docs` policy); `liter-llm` and `tree-sitter-language-pack` descriptions realigned to the authoritative product-list wording.

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
