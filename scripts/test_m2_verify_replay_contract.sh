#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
REPLAY="$ROOT_DIR/scripts/test_m2_verify_replay.sh"
WORK_DIR=$(mktemp -d)
trap 'rm -rf "$WORK_DIR"' EXIT

fail() {
  printf 'm2-verify replay contract: %s\n' "$1" >&2
  exit 1
}

run_success_case() {
  local rounds="$1"
  local report="$WORK_DIR/success-${rounds}.tsv"
  local log="$WORK_DIR/success-${rounds}.log"
  ZAP_CORPUS_ROUNDS="$rounds" \
    ZAP_BOUNDED_REPLAY_REPORT="$report" \
    ZAP_BOUNDED_REPLAY_LOG="$log" \
    "$REPLAY" >/dev/null
  grep -q $'\tpassed$' "$report" || fail "successful report is not marked passed for $rounds rounds"
  test "$(awk -F '\t' 'NR == 2 { print $2 }' "$report")" = "$rounds" || \
    fail "successful report recorded the wrong round count"
}

run_failure_case() {
  local variable="$1"
  local value="$2"
  local expected="$3"
  local output="$WORK_DIR/failure-${variable}-${value}.out"
  if env "$variable=$value" "$REPLAY" >"$output" 2>&1; then
    cat "$output" >&2
    fail "$variable=$value was accepted"
  fi
  grep -q "$expected" "$output" || {
    cat "$output" >&2
    fail "$variable=$value did not report $expected"
  }
}

run_success_case 1
run_failure_case ZAP_CORPUS_ROUNDS 0 'positive decimal integer'
run_failure_case ZAP_CORPUS_ROUNDS 65 'must not exceed 64'
run_failure_case ZAP_CORPUS_ROUNDS invalid 'positive decimal integer'
run_failure_case ZAP_CORPUS_MAX_FIXTURE_BYTES 1 'exceeds 1 bytes'
run_failure_case ZAP_CORPUS_MAX_TOTAL_BYTES 1 'exceeds 1 bytes'

printf 'm2-verify bounded replay contract regression passed\n'
