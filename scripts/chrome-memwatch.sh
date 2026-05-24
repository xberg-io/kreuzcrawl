#!/usr/bin/env bash
# chrome-memwatch.sh — chromiumoxide leak watcher for e2e runs.
#
# Polls chromiumoxide-spawned Chrome helpers (those carrying
# --remote-debugging-port) every $INTERVAL seconds. Tags samples with the
# currently active language test runner (pytest, vitest, mvn surefire, mix
# test, dotnet test, swift test, zig build test, php phpunit, ruby rspec,
# go test, dart test, etc.) so accumulation can be attributed to a specific
# binding.
#
# Outputs:
#   - stdout: anomalies ONLY — LEAK, ORPHAN-HELPERS, and the one-shot startup
#     line. The launching Monitor turns each line into a notification, so we
#     keep this signal-only. SESSION-START / SESSION-END (clean) traces go to
#     the log file rather than the chat to avoid drowning real alerts when
#     the rust binary or task runner triggers transient pattern matches.
#   - $LOG_FILE (default /tmp/kreuzcrawl-chrome-memwatch.log): every sample
#     plus every session boundary, for postmortem trend analysis.
#
# Leak signal: when a tagged session ends with helper count noticeably above
# the count we saw at session start. The threshold is intentionally low (>=2
# orphans) because the chromiumoxide page-close fix is supposed to keep this
# at zero. Set NOISE_THRESHOLD higher for chatty machines.

set -u

# Default log path is at the repo root (one level above this script) so CI
# can archive it as a build artifact via a predictable relative path. The
# file is gitignored — see kreuzcrawl/.gitignore. Override with LOG_FILE=…
# if you want to redirect (e.g. /tmp during ad-hoc debugging).
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEFAULT_LOG="$SCRIPT_DIR/../chrome-memwatch.log"

INTERVAL=${INTERVAL:-5}
LOG_FILE=${LOG_FILE:-$DEFAULT_LOG}
NOISE_THRESHOLD=${NOISE_THRESHOLD:-2}

# Match chromiumoxide-spawned helpers across macOS and Linux.
# macOS: "Google Chrome Helper (Renderer)" or "Google Chrome Helper"
# Linux: "chrome --type=renderer" etc.
HELPER_REGEX='(Google Chrome Helper|Chromium Helper|chrome|chromium).*--remote-debugging-port'

# Recognised test-runner command patterns. First field is a short tag the
# Monitor emits; second is the regex matched against the FULL command of any
# process. The order matters — more specific patterns first.
declare -a RUNNERS=(
  "rust:cargo test"
  "python:pytest"
  "node:vitest"
  "ruby:rspec"
  "php:phpunit"
  "java:org.apache.maven.surefire"
  "java:surefire-booter"
  "go:go test"
  "elixir:mix test"
  "elixir:beam.smp.*test"
  "dart:dart test"
  "csharp:dotnet test"
  "swift:swift test"
  "swift:xctest"
  "zig:zig build test"
  "zig:zig-out/test"
  "wasm:wasm-pack test"
  "wasm:vitest.*wasm"
  "c:e2e_c"
  "kotlin:gradle.*test"
)

count_helpers() {
  pgrep -fl "$HELPER_REGEX" 2>/dev/null | wc -l | tr -d ' '
}

# Return the FIRST runner tag whose regex matches any running process. Empty
# string when no test runner is active.
detect_runner() {
  local procs
  procs="$(ps -axo command= 2>/dev/null)"
  local entry tag pattern
  for entry in "${RUNNERS[@]}"; do
    tag="${entry%%:*}"
    pattern="${entry#*:}"
    if printf '%s\n' "$procs" | grep -Eq -- "$pattern"; then
      echo "$tag"
      return
    fi
  done
  echo ""
}

ts() { date -u +%Y-%m-%dT%H:%M:%SZ; }

log_sample() {
  local now="$1" runner="$2" count="$3"
  printf '%s runner=%s helpers=%d\n' "$now" "${runner:-none}" "$count" >>"$LOG_FILE"
}

# Log-only — never reaches the Monitor chat. Used for session boundaries
# (start/clean-end) which would otherwise drown real LEAK alerts when the
# test runner pattern matches transient cargo/dotnet/etc invocations.
trace() {
  printf '%s TRACE %s\n' "$(ts)" "$*" >>"$LOG_FILE"
}

# Chat-visible alert. Reserved for genuine anomalies the user should act on.
emit() { printf '%s %s\n' "$(ts)" "$*"; }

emit "MEMWATCH start interval=${INTERVAL}s threshold=${NOISE_THRESHOLD} log=$LOG_FILE"

prev_runner=""
session_start_helpers=0
session_peak_helpers=0
session_started_at=""

while true; do
  cur_runner="$(detect_runner)"
  cur_helpers="$(count_helpers)"
  now="$(ts)"
  log_sample "$now" "$cur_runner" "$cur_helpers"

  if [ -n "$cur_runner" ] && [ "$cur_runner" != "$prev_runner" ]; then
    # New session started (and either no prior session or a different one).
    if [ -n "$prev_runner" ]; then
      end_helpers="$cur_helpers"
      delta=$((end_helpers - session_start_helpers))
      if [ "$delta" -ge "$NOISE_THRESHOLD" ]; then
        emit "LEAK runner=$prev_runner started=$session_started_at delta_helpers=+$delta peak=$session_peak_helpers end=$end_helpers"
      else
        trace "SESSION-END runner=$prev_runner peak=$session_peak_helpers end=$end_helpers (clean)"
      fi
    fi
    session_start_helpers="$cur_helpers"
    session_peak_helpers="$cur_helpers"
    session_started_at="$now"
    trace "SESSION-START runner=$cur_runner baseline_helpers=$cur_helpers"
  fi

  if [ -n "$cur_runner" ] && [ "$cur_helpers" -gt "$session_peak_helpers" ]; then
    session_peak_helpers="$cur_helpers"
  fi

  if [ -z "$cur_runner" ] && [ -n "$prev_runner" ]; then
    end_helpers="$cur_helpers"
    delta=$((end_helpers - session_start_helpers))
    if [ "$delta" -ge "$NOISE_THRESHOLD" ]; then
      emit "LEAK runner=$prev_runner started=$session_started_at delta_helpers=+$delta peak=$session_peak_helpers end=$end_helpers"
    else
      trace "SESSION-END runner=$prev_runner peak=$session_peak_helpers end=$end_helpers (clean)"
    fi
    session_start_helpers=0
    session_peak_helpers=0
    session_started_at=""
  fi

  # Orphan-watch: no test runner, yet helpers persist >=$NOISE_THRESHOLD.
  if [ -z "$cur_runner" ] && [ "$cur_helpers" -ge "$NOISE_THRESHOLD" ]; then
    emit "ORPHAN-HELPERS count=$cur_helpers (no test runner running)"
  fi

  prev_runner="$cur_runner"
  sleep "$INTERVAL"
done
