#!/usr/bin/env python3
"""Docker image test script for kreuzcrawl CLI."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys

GREEN = "\033[0;32m"
RED = "\033[0;31m"
YELLOW = "\033[1;33m"
NC = "\033[0m"


def run(image: str, *args: str, check: bool = True) -> subprocess.CompletedProcess[str]:
    cmd = ["docker", "run", "--rm", image, *args]
    return subprocess.run(cmd, capture_output=True, text=True, check=check)


def test_version(image: str) -> bool:
    result = run(image, "--version", check=False)
    ok = result.returncode == 0 and "kreuzcrawl" in result.stdout
    print(f"  {'PASS' if ok else 'FAIL'}: --version → {result.stdout.strip()}")
    return ok


def test_help(image: str) -> bool:
    result = run(image, "--help", check=False)
    ok = result.returncode == 0 and "scrape" in result.stdout and "crawl" in result.stdout
    print(f"  {'PASS' if ok else 'FAIL'}: --help lists subcommands")
    return ok


def test_scrape_help(image: str) -> bool:
    result = run(image, "scrape", "--help", check=False)
    ok = result.returncode == 0 and "URL" in result.stdout
    print(f"  {'PASS' if ok else 'FAIL'}: scrape --help")
    return ok


def test_scrape_json(image: str) -> bool:
    """Test scraping a public URL with JSON output."""
    result = run(image, "scrape", "https://example.com", "--format", "json", "--browser-mode", "never", check=False)
    if result.returncode != 0:
        print(f"  FAIL: scrape json — exit code {result.returncode}")
        if result.stderr:
            print(f"    stderr: {result.stderr[:200]}")
        return False
    try:
        data = json.loads(result.stdout)
        ok = "metadata" in data or "title" in str(data) or "status_code" in data
        print(f"  {'PASS' if ok else 'FAIL'}: scrape json — got valid JSON response")
        return ok
    except json.JSONDecodeError:
        print(f"  FAIL: scrape json — invalid JSON output")
        return False


def test_nonroot_user(image: str) -> bool:
    """Verify the container runs as non-root."""
    cmd = ["docker", "run", "--rm", "--entrypoint", "id", image]
    result = subprocess.run(cmd, capture_output=True, text=True, check=False)
    ok = result.returncode == 0 and "root" not in result.stdout
    print(f"  {'PASS' if ok else 'FAIL'}: non-root user ({result.stdout.strip()})")
    return ok


def test_invalid_url(image: str) -> bool:
    """Test error handling for invalid URL."""
    result = run(image, "scrape", "not-a-valid-url", check=False)
    ok = result.returncode != 0
    print(f"  {'PASS' if ok else 'FAIL'}: invalid URL returns error (exit code {result.returncode})")
    return ok


def main() -> None:
    parser = argparse.ArgumentParser(description="Test kreuzcrawl Docker image")
    parser.add_argument("--image", default="kreuzcrawl:latest", help="Docker image to test")
    parser.add_argument("--variant", default="cli", help="Image variant (unused, kept for compat)")
    parser.add_argument("--verbose", action="store_true", help="Verbose output")
    args = parser.parse_args()

    image = args.image
    print(f"\n{YELLOW}Testing Docker image: {image}{NC}\n")

    tests = [
        ("Version check", test_version),
        ("Help output", test_help),
        ("Scrape help", test_scrape_help),
        ("Scrape JSON", test_scrape_json),
        ("Non-root user", test_nonroot_user),
        ("Invalid URL error", test_invalid_url),
    ]

    passed = 0
    failed = 0
    failed_names: list[str] = []

    for name, test_fn in tests:
        print(f"\n{YELLOW}[TEST]{NC} {name}")
        try:
            if test_fn(image):
                passed += 1
            else:
                failed += 1
                failed_names.append(name)
        except Exception as e:
            print(f"  FAIL: {name} — exception: {e}")
            failed += 1
            failed_names.append(name)

    print(f"\n{'='*50}")
    if failed == 0:
        print(f"{GREEN}All {passed} tests passed!{NC}")
    else:
        print(f"{RED}Failed tests:{NC}")
        for name in failed_names:
            print(f"  - {name}")
        print(f"\n{passed} passed, {failed} failed")

    sys.exit(1 if failed > 0 else 0)


if __name__ == "__main__":
    main()
