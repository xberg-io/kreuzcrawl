#!/usr/bin/env bash

set -euo pipefail

VARIANT="${1:-}"
OUTPUT_DIR="${2:-/tmp}"

if [ -z "$VARIANT" ]; then
  echo "Usage: save-image.sh <variant> [output-dir]"
  echo "  variant: core or full"
  echo "  output-dir: directory to save tarball (default: /tmp)"
  exit 1
fi

echo "=== Saving Docker image as tar archive ==="
mkdir -p "$OUTPUT_DIR"
docker save "kreuzcrawl:$VARIANT" | gzip >"$OUTPUT_DIR/kreuzcrawl-$VARIANT.tar.gz"
ls -lh "$OUTPUT_DIR/kreuzcrawl-$VARIANT.tar.gz"
