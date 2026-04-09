#!/usr/bin/env bash

set -euo pipefail

pnpm --filter @kreuzcrawl/node exec napi build --platform --dts index.d.ts
mkdir -p typescript-defs
cp crates/kreuzcrawl-node/index.d.ts typescript-defs/
cp crates/kreuzcrawl-node/index.js typescript-defs/ || true
