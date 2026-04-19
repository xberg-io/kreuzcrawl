# Changelog

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
