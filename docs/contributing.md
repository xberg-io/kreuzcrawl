# Contributing

## Setup

Rust 1.95 is pinned in `rust-toolchain.toml`. Clone and build:

```bash
git clone https://github.com/kreuzberg-dev/kreuzcrawl
cd kreuzcrawl
cargo build
```

`rustup` will fetch 1.95 on first run if you don't have it. Most Rust work is under `crates/kreuzcrawl/`.

### Pre-commit

Install [pre-commit](https://pre-commit.com/) once, then wire up both hook types:

```bash
pip install pre-commit
pre-commit install --hook-type pre-commit --hook-type commit-msg
```

It runs automatically on `git commit`. To run it by hand:

```bash
pre-commit run --all-files
```

## Feature flags

The default build has no optional dependencies. Add features as needed:

| Feature | What it adds |
|---------|-------------|
| `browser` | Headless Chrome via chromiumoxide |
| `ai` | LLM extraction via liter-llm |
| `api` | REST API server via Axum |
| `mcp` | Model Context Protocol server |
| `mcp-http` | MCP over HTTP (implies `mcp` + `api`) |
| `tracing` | OpenTelemetry spans |
| `interact` | Page interaction — click, type, scroll (implies `browser`) |
| `warc` | WARC archive output |

```bash
cargo build --features api,mcp
```

`browser` and `interact` require a Chrome/Chromium binary at runtime. They compile without one but panic the first time you use them.

## Tests

To run all tests across the workspace (this includes both unit tests and integration tests, which may hit the network):

```bash
cargo test --workspace
```

To run *only* the integration test binaries:

```bash
cargo test --workspace --test '*'
```

Browser tests need a running Chrome instance. Docker is the least painful way:

```bash
docker compose -f docker-compose.test.yml up -d
cargo test --features browser --test browser_tests
```

CI treats clippy warnings as errors, so run this before pushing:

```bash
cargo clippy --workspace --all-features --all-targets -- -D warnings
```

## Code conventions

Formatting is configured in `rustfmt.toml`. `cargo fmt` (or the pre-commit hook) handles it.

Four things that aren't obvious from the code:

- Async runtime is `tokio`. Don't introduce `async-std` or `smol`.
- Public API types belong in `crates/kreuzcrawl/src/types/`. Internal types live next to the code that uses them.
- `defaults/` is `pub(crate)`. Don't re-export from it at the crate root without a discussion — the narrow public surface is intentional.
- Error types go in `error.rs` with `thiserror`. No `anyhow` in library code.

## Commits

[gitfluff](https://github.com/Goldziher/gitfluff) lints commit messages on commit. Format:

```text
<type>(<scope>): <subject>

[optional body]
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`. Scope is the crate or area — `engine`, `api`, `cli`, `python`, etc.

Good example: `fix(engine): respect max_pages limit during batch crawl`

## Pull requests

Branch from `main` using `<type>/<short-description>` — e.g. `feat/streaming-results` or `fix/rate-limiter-overflow`.

Before opening:

1. `cargo fmt --all`
2. `cargo clippy --workspace --all-features --all-targets -- -D warnings`
3. `cargo test --workspace`
4. Update `docs/` if you changed a public API or added a feature.

PR descriptions don't need a template. Say what changed and why. Link the issue if there is one.

CI runs the full binding test suite on each PR. You don't need Ruby, Java, PHP, or Elixir set up locally to work on the Rust core.

## License

Kreuzcrawl is under the [Elastic License 2.0](../LICENSE). Contributing means your changes are covered by it.
