#!/usr/bin/env python3
"""Check all registries for a given version. Prints a summary table.

Usage:
    python3 scripts/publish/check_all_registries.py <version>
"""


import importlib.util
import os
import sys
from pathlib import Path

# Locate check.py from the shared actions repo
SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent.parent
CHECK_PY = REPO_ROOT.parent / "actions" / "check-registry" / "scripts" / "check.py"

if not CHECK_PY.exists():
    print(f"Error: check.py not found at {CHECK_PY}", file=sys.stderr)
    print("Ensure the shared actions repo is checked out at ../actions/", file=sys.stderr)
    sys.exit(1)

# Import check.py as a module
spec = importlib.util.spec_from_file_location("check_registry", CHECK_PY)
assert spec and spec.loader
check_mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(check_mod)


GREEN = "\033[32m"
RED = "\033[31m"
RESET = "\033[0m"

CHECKS: list[tuple[str, str, str, dict[str, str]]] = [
    # (registry, package, label, kwargs)
    ("pypi", "kreuzcrawl", "PyPI", {}),
    ("npm", "@kreuzcrawl/node", "npm (@kreuzcrawl/node)", {}),
    ("npm", "@kreuzcrawl/wasm", "npm (WASM)", {}),
    ("rubygems", "kreuzcrawl", "RubyGems", {}),
    ("cratesio", "kreuzcrawl", "crates.io (kreuzcrawl)", {}),
    ("cratesio", "kreuzcrawl-cli", "crates.io (cli)", {}),
    ("cratesio", "kreuzcrawl-tesseract", "crates.io (tesseract)", {}),
    ("cratesio", "kreuzcrawl-paddle-ocr", "crates.io (paddle-ocr)", {}),
    ("cratesio", "kreuzcrawl-pdfium-render", "crates.io (pdfium)", {}),
    ("maven", "dev.kreuzcrawl:kreuzcrawl", "Maven", {}),
    ("nuget", "Kreuzcrawl", "NuGet", {}),
    ("packagist", "kreuzcrawl/kreuzcrawl", "Packagist", {}),
    ("hex", "kreuzcrawl", "Hex.pm", {}),
    ("homebrew", "kreuzcrawl", "Homebrew", {"tap_repo": "kreuzcrawl-dev/homebrew-tap"}),
]

GH_ASSET_CHECKS: list[tuple[str, str, dict[str, str]]] = [
    # (label, asset_prefix_or_assets, kwargs)
    ("CLI binaries", "", {"asset_prefix": "kreuzcrawl-cli-"}),
    ("Go FFI", "", {"asset_prefix": "go-ffi-"}),
    ("C FFI", "", {"asset_prefix": "c-ffi-"}),
    ("Elixir NIF", "", {"asset_prefix": "libkreuzcrawl_rustler-"}),
    ("Homebrew bottles", "", {"asset_prefix": "kreuzcrawl-"}),
]


def main() -> None:
    if len(sys.argv) < 2:
        print("Usage: check_all_registries.py <version>", file=sys.stderr)
        sys.exit(1)

    version = sys.argv[1].lstrip("v")
    tag = f"v{version}"

    # Suppress GITHUB_OUTPUT writes during local check-all
    old_output = os.environ.pop("GITHUB_OUTPUT", None)

    results: list[tuple[str, bool]] = []

    print(f"\nChecking all registries for version {version}...")
    print("=" * 60)

    for registry, package, label, kwargs in CHECKS:
        check_fn = check_mod.REGISTRIES[registry]
        exists = check_fn(package, version, **kwargs)
        results.append((label, exists))

    # GitHub release asset checks
    print("\nChecking GitHub Release assets...")
    for label, _, kwargs in GH_ASSET_CHECKS:
        exists = check_mod.check_github_release("kreuzcrawl", version, tag=tag, **kwargs)
        results.append((f"GH: {label}", exists))

    # Summary
    print("\n" + "=" * 60)
    print("Results:\n")
    passed = 0
    failed = 0
    for label, exists in results:
        if exists:
            print(f"  {GREEN}EXISTS{RESET}   {label}@{version}")
            passed += 1
        else:
            print(f"  {RED}MISSING{RESET}  {label}@{version}")
            failed += 1

    print(f"\nTotal: {passed + failed} checks -- {GREEN}{passed} exist{RESET}, {RED}{failed} missing{RESET}\n")

    if old_output is not None:
        os.environ["GITHUB_OUTPUT"] = old_output

    sys.exit(1 if failed > 0 else 0)


if __name__ == "__main__":
    main()
