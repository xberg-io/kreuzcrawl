#!/usr/bin/env bash

set -euo pipefail

VARIANT="${1:-}"
RESULTS_FILE="${2:-/tmp/kreuzcrawl-docker-test-results.json}"

if [ -z "$VARIANT" ]; then
  echo "Usage: summary.sh <variant> [results-file]"
  echo "  variant: core or full"
  echo "  results-file: path to test results JSON (default: /tmp/kreuzcrawl-docker-test-results.json)"
  exit 1
fi

echo "✅ Docker image built and tested successfully!"
echo ""
echo "Variant: $VARIANT"
echo "Image: kreuzcrawl:$VARIANT"
echo ""

if [ -f "$RESULTS_FILE" ]; then
  echo "Test Results:"
  jq . <"$RESULTS_FILE" || cat "$RESULTS_FILE"
fi
