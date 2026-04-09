#!/usr/bin/env bash
set -euo pipefail

echo "=========================================="
echo "FFI Library Verification"
echo "=========================================="

if [[ "${RUNNER_OS:-}" == "Windows" ]]; then
  echo "Looking for Windows library files (.dll, .a, .lib)..."
  if ls target/x86_64-pc-windows-gnu/release/libkreuzcrawl_ffi.* 2>/dev/null; then
    echo "✓ Found FFI library in GNU target"
    ls -lh target/x86_64-pc-windows-gnu/release/libkreuzcrawl_ffi.*
  elif ls target/release/libkreuzcrawl_ffi.* 2>/dev/null; then
    echo "✓ Found FFI library in release target"
    ls -lh target/release/libkreuzcrawl_ffi.*
  else
    echo "✗ Error: FFI library not found in expected locations"
    find . -name "libkreuzcrawl_ffi.*" -o -name "kreuzcrawl_ffi.*" 2>/dev/null || echo "No FFI library files found"
    exit 1
  fi
else
  echo "Looking for Unix library files (.so, .dylib, .a)..."
  if ls target/release/libkreuzcrawl_ffi.* 2>/dev/null; then
    echo "✓ Found FFI library in target/release"
    ls -lh target/release/libkreuzcrawl_ffi.*
  else
    echo "✗ Error: FFI library not found in target/release"
    exit 1
  fi
fi
