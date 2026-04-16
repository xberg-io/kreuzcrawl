"""Print the installed kreuzcrawl Python package version."""

from __future__ import annotations

import sys


def main() -> int:
    try:
        import kreuzcrawl  # type: ignore[import-untyped]
    except ImportError as exc:  # pragma: no cover - runtime helper
        print(f"Failed to import kreuzcrawl: {exc}", file=sys.stderr)
        return 1

    print(f"Kreuzcrawl version: {getattr(kreuzcrawl, '__version__', 'unknown')}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
