#!/usr/bin/env bash

set -euo pipefail

echo "=== Cleaning previous wheel artifacts ==="
rm -rf target/wheels target/maturin
rm -f packages/python/kreuzcrawl/_internal_bindings.*
echo "Cleanup complete"
