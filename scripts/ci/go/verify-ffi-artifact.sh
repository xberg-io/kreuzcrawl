#!/usr/bin/env bash
set -euo pipefail

ARTIFACT="${1}"

if [ ! -f "${ARTIFACT}" ]; then
  echo "✗ Artifact not found: ${ARTIFACT}"
  exit 1
fi

echo "=== Verifying artifact structure ==="
tar -tzf "${ARTIFACT}"

cleanup() {
  rm -rf verify-temp
}
trap cleanup EXIT

mkdir -p verify-temp
tar -xzf "${ARTIFACT}" -C verify-temp

REQUIRED_FILES=(
  "kreuzcrawl-ffi/include/kreuzcrawl.h"
  "kreuzcrawl-ffi/share/pkgconfig/kreuzcrawl-ffi.pc"
)

echo ""
echo "=== Checking required files ==="
for file in "${REQUIRED_FILES[@]}"; do
  if [ -f "verify-temp/$file" ]; then
    echo "✓ Found: $file"
  else
    echo "✗ Missing: $file"
    exit 1
  fi
done

echo ""
echo "=== Checking static library (required for Go) ==="
STATIC_LIB="verify-temp/kreuzcrawl-ffi/lib/libkreuzcrawl_ffi.a"
if [ -f "$STATIC_LIB" ]; then
  echo "✓ Found static library: libkreuzcrawl_ffi.a ($(du -h "$STATIC_LIB" | cut -f1))"
else
  echo "✗ Missing static library: libkreuzcrawl_ffi.a"
  exit 1
fi

echo ""
echo "=== Checking platform-specific dynamic libraries (optional) ==="
PLATFORM_LIBS_FOUND=0

if find verify-temp/kreuzcrawl-ffi/lib -name "*.so" -o -name "*.so.*" 2>/dev/null | grep -q .; then
  LIBKREUZBERG=$(find verify-temp/kreuzcrawl-ffi/lib -name "libkreuzcrawl_ffi.so*" 2>/dev/null | head -1)
  if [ -n "$LIBKREUZBERG" ]; then
    echo "✓ Found Linux dynamic library: $(basename "$LIBKREUZBERG")"
    PLATFORM_LIBS_FOUND=1
  fi
fi

if find verify-temp/kreuzcrawl-ffi/lib -name "*.dylib" 2>/dev/null | grep -q .; then
  LIBKREUZBERG=$(find verify-temp/kreuzcrawl-ffi/lib -name "libkreuzcrawl_ffi.dylib" 2>/dev/null | head -1)
  if [ -n "$LIBKREUZBERG" ]; then
    echo "✓ Found macOS dynamic library: $(basename "$LIBKREUZBERG")"
    PLATFORM_LIBS_FOUND=1
  fi
fi

if find verify-temp/kreuzcrawl-ffi/lib -name "*.dll" 2>/dev/null | grep -q .; then
  LIBKREUZBERG=$(find verify-temp/kreuzcrawl-ffi/lib -name "kreuzcrawl_ffi.dll" 2>/dev/null | head -1)
  if [ -n "$LIBKREUZBERG" ]; then
    echo "✓ Found Windows dynamic library: $(basename "$LIBKREUZBERG")"
    PLATFORM_LIBS_FOUND=1
  fi
fi

if [ $PLATFORM_LIBS_FOUND -eq 0 ]; then
  echo "  (No dynamic libraries found - static linking only)"
fi

echo ""
echo "✓ Artifact verification passed"
