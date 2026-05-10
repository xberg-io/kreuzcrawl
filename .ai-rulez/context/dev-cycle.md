---
priority: high
---
Iteration loops are codified as Taskfile tasks. Prefer them over ad-hoc commands.

### After alef changes
```
task alef:install              # cargo install --path ../alef/crates/alef-cli
task alef:regen                # alef all && alef e2e generate
VERSION=0.15.30 task alef:bump # bump pin in alef.toml + reinstall + regen
```

### After kreuzcrawl-core changes
```
task rebuild        # all bindings + e2e mock-server
task rebuild:fast   # rust + python + node + mock-server only
```

### E2E cycles
```
task e2e:reset           # clean + regen + rebuild + run all e2e
task e2e:refresh         # regen + rebuild + run all e2e (no clean)
task e2e:refresh:fast    # regen + rebuild rust/python/node + run their e2e
task python:cycle        # rebuild py + uv sync --reinstall + run e2e
```

### Cleanup
```
task clean           # per-language artifacts
task clean:workspace # cargo target + alef IR cache + legacy mock-server bins
task clean:e2e       # venvs, node_modules, _build, lockfiles, vendor/
task clean:full      # all of the above
```

### Gotchas (codified, but worth knowing)

- **Two mock-server binaries.** `tools/mock-server` is legacy/unused; `e2e/rust/src/main.rs` is alef-generated and the binary all language conftests actually spawn. `task rebuild` (and the `mock-server:rebuild` sub-task) builds the right one at `e2e/rust/target/release/mock-server`.
- **Python venv stale `.so`.** After `task python:build:dev`, the e2e venv at `e2e/python/.venv` keeps a stale extension. `task python:cycle` runs `uv sync --reinstall` to refresh.
- **Elixir precompiled NIF.** Rustler's precompiled binary is tagged at the package's release version and lags local source. `KREUZCRAWL_BUILD=1` is set in `.task/languages/elixir.yml::e2e:test` so the e2e suite always builds the NIF from local source.
- **Generated e2e files.** `e2e/<lang>/` is alef-generated — never hand-edit. Modify fixtures or alef codegen, then `task alef:regen`.
