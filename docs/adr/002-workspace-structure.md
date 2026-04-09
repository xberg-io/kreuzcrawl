# ADR-002: Workspace Structure — Crates + Tools Layout

**Status**: Accepted (updated 2026-03-10)

**Date**: 2026-03-09

## Context

kreuzcrawl needs to be consumable both as a Rust library (embedded in kreuzberg-cloud or other services) and as a standalone CLI tool. The codebase must support independent versioning of the library API vs the CLI interface, and allow downstream consumers to depend on the library without pulling in CLI-specific dependencies (clap, terminal formatting).

Additionally, the project requires internal tooling (E2E test generators, benchmarks) that should not be published or included in downstream dependency trees.

We considered three approaches:

1. **Single crate** with CLI behind a feature flag
2. **Two-crate workspace**: library + CLI binary
3. **Multi-crate workspace**: core types, web engine, CLI, tools (4+ crates)

## Decision

### Cargo workspace with crates/ and tools/ directories

```text
kreuzcrawl/
├── Cargo.toml              # Workspace root
├── crates/
│   └── kreuzcrawl/         # Library crate (the engine)
├── tools/
│   └── e2e-generator/      # Test generation tool (publish = false)
├── fixtures/               # E2E test fixtures (JSON + response bodies)
├── e2e/                    # Generated test suites (gitignored)
└── docs/adr/               # Architecture Decision Records
```

### Workspace Configuration

```toml
[workspace]
members = ["crates/*", "tools/*"]
exclude = ["e2e/rust", ".claude"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "Elastic-2.0"
rust-version = "1.91"
```

All dependencies declared in `[workspace.dependencies]` — members reference via `workspace = true`.

### Why crates/ + tools/ separation

- **crates/**: Published library and binary crates. These form the public API and downstream dependency graph.
- **tools/**: Internal build-time tooling (`publish = false`). Not included in `cargo publish` or downstream `cargo install`. Currently contains the E2E test generator; will host benchmarks and codegen tools.
- **e2e/**: Generated output from tools, excluded from workspace members and gitignored. Regenerated on demand via `task e2e:generate`.

### Why not a single crate with feature-gated CLI

Feature-gated binaries in Cargo are awkward:

- `cargo install kreuzcrawl --features cli` is poor UX
- Conditional `[[bin]]` sections don't exist in Cargo
- Library consumers would still download clap sources even if not compiled

### Relationship to kreuzberg-dev repos

Follows the same patterns established in kreuzberg-cloud:

- `crates/` directory for Rust code
- Workspace-level dependency management
- Workspace-level version inheritance
- Edition 2024, resolver 2

## Consequences

### Positive

- **Clean library API**: `kreuzcrawl` crate has zero CLI dependencies
- **Independent binary**: CLI can evolve its UX without affecting library API
- **Simple dependency**: `kreuzcrawl = "0.1"` in downstream Cargo.toml
- **Consistent with org patterns**: Same structure as kreuzberg, kreuzberg-cloud
- **Tools don't pollute**: Internal tooling stays out of the published dependency graph

### Negative

- **Multiple Cargo.toml files to maintain**: Minor overhead vs single crate
- **Version coupling**: All crates share workspace version — CLI release tied to library release

## Notes

Implementation:

- `/Cargo.toml` — Workspace definition
- `crates/kreuzcrawl/Cargo.toml` — Library crate
- `tools/e2e-generator/Cargo.toml` — E2E test generator (internal tool)
