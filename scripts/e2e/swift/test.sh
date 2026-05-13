#!/usr/bin/env bash
# Run Swift e2e tests, starting the alef-generated mock server and setting
# per-fixture env vars (MOCK_SERVER_<FIXTURE_ID>) so robots/sitemap tests
# can hit the correct per-fixture listener.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
MOCK_SERVER_BIN="${REPO_ROOT}/e2e/rust/target/release/mock-server"
FIXTURES_DIR="${REPO_ROOT}/fixtures"
SWIFT_PKG_DIR="${REPO_ROOT}/packages/swift"

if [ ! -f "$MOCK_SERVER_BIN" ]; then
  echo "mock-server not found at $MOCK_SERVER_BIN" >&2
  echo "Run: (cd e2e/rust && cargo build --release --bin mock-server)" >&2
  exit 1
fi

MOCK_OUT=$(mktemp)
# Trap cleans up the mock-server pipeline. The server blocks on stdin until
# EOF (see e2e/rust/src/main.rs), so we feed it from a FIFO whose write end
# we hold open ourselves. Closing the FD on exit lets the server return
# cleanly; killing it is a fallback for the abort path.
MOCK_FIFO=$(mktemp -u)
mkfifo "$MOCK_FIFO"
trap 'exec 9>&- 2>/dev/null || true; rm -f "$MOCK_OUT" "$MOCK_FIFO"; kill "$MOCK_PID" 2>/dev/null || true; wait "$MOCK_PID" 2>/dev/null || true' EXIT INT TERM

# Open FIFO for writing in the parent shell so the server sees an open stdin
# until we close fd 9. This avoids the `sleep 9999` pipeline hack which left
# the sleep alive when the trap ran and blocked the shell from exiting.
# Start the server first (it opens the read end), then open fd 9 for writing
# in the parent. Both opens must succeed before the FIFO is unblocked, so do
# the server start in the background and follow with the parent open.
"$MOCK_SERVER_BIN" "$FIXTURES_DIR" <"$MOCK_FIFO" >"$MOCK_OUT" 2>&1 &
MOCK_PID=$!
exec 9>"$MOCK_FIFO"

# Wait up to 5 s for the server to emit MOCK_SERVERS= (the second startup line).
for _ in $(seq 1 50); do
  if grep -q "^MOCK_SERVERS=" "$MOCK_OUT" 2>/dev/null; then
    break
  fi
  sleep 0.1
done

# Export MOCK_SERVER_URL from the first startup line.
URL_LINE=$(grep "^MOCK_SERVER_URL=" "$MOCK_OUT" | head -1)
if [ -n "$URL_LINE" ]; then
  export MOCK_SERVER_URL="${URL_LINE#MOCK_SERVER_URL=}"
fi

# Export per-fixture env vars from the MOCK_SERVERS JSON map.
# E.g. {"robots_disallow_path":"http://..."} → MOCK_SERVER_ROBOTS_DISALLOW_PATH=http://...
SERVERS_LINE=$(grep "^MOCK_SERVERS=" "$MOCK_OUT" | head -1)
if [ -n "$SERVERS_LINE" ]; then
  SERVERS_JSON="${SERVERS_LINE#MOCK_SERVERS=}"
  export MOCK_SERVERS="$SERVERS_JSON"
  # shellcheck disable=SC2046
  eval "$(python3 -c "
import json, sys
d = json.loads(sys.argv[1])
for k, v in d.items():
    key = 'MOCK_SERVER_' + k.upper()
    # Escape double quotes in value just in case
    v = v.replace('\"', '\\\\\"')
    print(f'export {key}=\"{v}\"')
" "$SERVERS_JSON")"
fi

cd "$SWIFT_PKG_DIR"
swift test "$@"
