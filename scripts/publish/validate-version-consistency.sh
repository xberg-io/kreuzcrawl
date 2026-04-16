#!/usr/bin/env bash
set -euo pipefail

expected="${1:-${EXPECTED_VERSION:-}}"
if [ -z "$expected" ]; then
  echo "Usage: $0 <expected-version> (or set EXPECTED_VERSION)" >&2
  exit 2
fi

errors=0
checked=0

# Convert semver pre-release to PEP 440 for Python comparison
# e.g., "0.1.0-rc.1" → "0.1.0rc1", "0.1.0-alpha.2" → "0.1.0a2"
expected_pep440="$(echo "$expected" | sed -E 's/-alpha\.?/a/; s/-beta\.?/b/; s/-rc\.?/rc/; s/\.([0-9]+)$/\1/')"

echo "Expected version: $expected (PEP 440: $expected_pep440)"
echo "----------------------------------------"

check_version() {
  local label="$1" actual="$2" want="$3"
  checked=$((checked + 1))
  echo "$label: $actual"
  if [ "$actual" != "$want" ]; then
    echo "❌ $label mismatch (got '$actual', want '$want')"
    errors=$((errors + 1))
  fi
}

# ── Required sources ──

cargo_version="$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2 || true)"
check_version "Cargo.toml" "$cargo_version" "$expected"

root_version="$(jq -r '.version' package.json)"
check_version "package.json (root)" "$root_version" "$expected"

# ── Conditional sources (skip if file doesn't exist) ──

if [ -f crates/kreuzcrawl-wasm/package.json ]; then
  wasm_version="$(jq -r '.version' crates/kreuzcrawl-wasm/package.json)"
  check_version "crates/kreuzcrawl-wasm/package.json" "$wasm_version" "$expected"
elif [ -f crates/kreuzcrawl-wasm/Cargo.toml ]; then
  wasm_version="$(grep '^version' crates/kreuzcrawl-wasm/Cargo.toml | head -1 | cut -d'"' -f2 || true)"
  check_version "crates/kreuzcrawl-wasm/Cargo.toml" "$wasm_version" "$expected"
fi

if [ -f crates/kreuzcrawl-node/package.json ]; then
  node_version="$(jq -r '.version' crates/kreuzcrawl-node/package.json)"
  check_version "crates/kreuzcrawl-node/package.json" "$node_version" "$expected"
fi

if [ -f packages/python/pyproject.toml ]; then
  python_version="$(grep '^version' packages/python/pyproject.toml | head -1 | cut -d'"' -f2 || true)"
  check_version "packages/python/pyproject.toml" "$python_version" "$expected_pep440"
fi

ruby_version_file="$(find packages/ruby -name version.rb -path '*/kreuzcrawl/version.rb' 2>/dev/null | head -1)"
if [ -n "$ruby_version_file" ]; then
  ruby_version="$(grep 'VERSION' "$ruby_version_file" | sed -n 's/.*VERSION *= *"\([^"]*\)".*/\1/p')"
  check_version "$ruby_version_file" "$ruby_version" "$expected"
fi

if [ -f packages/r/DESCRIPTION ]; then
  r_version="$(grep '^Version:' packages/r/DESCRIPTION | sed 's/Version: //')"
  check_version "packages/r/DESCRIPTION" "$r_version" "$expected"
fi

if [ -f packages/java/pom.xml ]; then
  java_version="$(
    python3 - <<'PY'
import re
import xml.etree.ElementTree as ET
from pathlib import Path

text = Path("packages/java/pom.xml").read_text(encoding="utf-8")
text = re.sub(r'xmlns="[^"]+"', '', text, count=1)
root = ET.fromstring(text)
version = root.findtext("version") or ""
print(version.strip())
PY
  )"
  check_version "packages/java/pom.xml" "$java_version" "$expected"
fi

if [ -f packages/csharp/Kreuzcrawl/Kreuzcrawl.csproj ]; then
  csharp_version="$(
    python3 - <<'PY'
import re
import xml.etree.ElementTree as ET
from pathlib import Path

text = Path("packages/csharp/Kreuzcrawl/Kreuzcrawl.csproj").read_text(encoding="utf-8")
text = re.sub(r'xmlns="[^"]+"', '', text, count=1)
root = ET.fromstring(text)
version = ""
for elem in root.iter():
    if elem.tag == "Version" and (elem.text or "").strip():
        version = elem.text.strip()
        break
print(version)
PY
  )"
  check_version "packages/csharp/Kreuzcrawl/Kreuzcrawl.csproj" "$csharp_version" "$expected"
fi

if [ -f packages/go/v4/doc.go ]; then
  go_version="$(
    python3 - <<'PY'
import re
from pathlib import Path

text = Path("packages/go/v4/doc.go").read_text(encoding="utf-8")
m = re.search(r"This binding targets Kreuzcrawl\s+([^\s]+)", text)
print(m.group(1) if m else "")
PY
  )"
  check_version "packages/go/v4/doc.go" "$go_version" "$expected"
elif [ -f packages/go/go.mod ]; then
  echo "packages/go/v4/doc.go: (not found, skipping Go version check)"
fi

if [ -f packages/php/composer.json ]; then
  php_version="$(jq -r '.version // empty' packages/php/composer.json)"
  if [ -n "$php_version" ]; then
    check_version "packages/php/composer.json" "$php_version" "$expected"
  else
    echo "packages/php/composer.json: (no version field, skipping)"
  fi
fi

if [ -f packages/elixir/mix.exs ]; then
  elixir_version="$(grep 'version:' packages/elixir/mix.exs | head -1 | sed -n 's/.*version: *"\([^"]*\)".*/\1/p')"
  check_version "packages/elixir/mix.exs" "$elixir_version" "$expected"
fi

echo "----------------------------------------"
if [ "$errors" -gt 0 ]; then
  echo "❌ $errors version mismatches found (checked $checked sources)"
  exit 1
fi

echo "✅ All $checked version sources consistent: $expected"
