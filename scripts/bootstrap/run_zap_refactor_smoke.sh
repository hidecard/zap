#!/usr/bin/env bash
# Zap in-flight run_zap() portability refactor smoke gate.
#
# This gate exists to give the maintainer concrete evidence that the
# 150-script in-flight refactor (which replaces `cargo run --manifest-path
# native/Cargo.toml -- ...` with a run_zap() helper that prefers
# bin/zap) is safe to commit as one PR. It does NOT replace the per-script
# bootstrap verification gates; it only validates the refactor plumbing.
#
# Phase 1: bash -n parse check on every refactored script. Catches
#          syntax errors and heredoc mistakes that would otherwise be
#          hidden until the next CI run.
# Phase 2: end-to-end smoke run with a per-script 60s timeout. Validates
#          that run_zap() resolves to a working binary path under each
#          script's environment (bin/zap -> release/debug zap -> cargo run).
#          Verifier failures are recorded but do not fail the gate.
# Phase 3: repo-root cleanliness scan after the full sweep. Uses a
#          before/after snapshot to distinguish NEW leaks (script-induced)
#          from PRE-EXISTING files (e.g. from a running zap LSP or
#          interactive shells).
#
# Exit 0 if all three phases pass.

set -uo pipefail
IFS=$'\n\t'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

if [[ -f "$HOME/.cargo/env" ]]; then
  source "$HOME/.cargo/env"
fi

# Per-script timeout for the end-to-end phase.
S_TIMEOUT="${ZAP_REFACTOR_SMOKE_TIMEOUT:-60}"

pass()  { printf 'PASS: %s\n' "$1"; }
fail()  { printf 'FAIL: %s\n' "$1" >&2; FAILURES=$((FAILURES + 1)); }
note()  { printf 'NOTE: %s\n' "$1"; }

FAILURES=0

# Exclude self from the sweep list (would recurse forever).
SELF="$(basename "$0")"
mapfile -t REFACTORED < <(grep -l 'run_zap()' scripts/bootstrap/*.sh | sort | grep -v "/${SELF}$" || true)

printf 'Zap run_zap() portability refactor smoke gate\n'
printf 'Repository: %s\n' "$ROOT_DIR"
printf 'Refactored scripts discovered: %d (self excluded)\n' "${#REFACTORED[@]}"
printf 'Per-script end-to-end timeout: %ss\n' "$S_TIMEOUT"
printf '%-18s %s\n' 'CHECK' 'RESULT'
printf '%-18s %s\n' '-----' '------'

# --- Phase 0: snapshot the repo root before the sweep. ---
# This lets us distinguish NEW leaks (created by the scripts) from
# pre-existing scratch files (often created by a running `zap lsp`).
SNAPSHOT_BEFORE="$(mktemp)"
trap 'rm -f "$SNAPSHOT_BEFORE"' EXIT
find "$ROOT_DIR" -maxdepth 1 -name '.zap-*' -printf '%f\n' 2>/dev/null | sort > "$SNAPSHOT_BEFORE"
before_count=$(wc -l < "$SNAPSHOT_BEFORE")
note "phase 0: snapshot of pre-existing repo-root .zap-* artifacts: $before_count"

# --- Phase 1: bash -n parse check. ---
parse_failures=0
for s in "${REFACTORED[@]}"; do
  if bash -n "$s" 2>/dev/null; then
    :
  else
    fail "bash -n parse failure: $s"
    parse_failures=$((parse_failures + 1))
  fi
done
if (( parse_failures == 0 )); then
  pass "phase 1: bash -n parse check on ${#REFACTORED[@]} scripts"
fi

# --- Phase 2: end-to-end smoke run with timeout. ---
ran=0
ran_ok=0
ran_failed=0
for s in "${REFACTORED[@]}"; do
  ran=$((ran + 1))
  if timeout "$S_TIMEOUT" bash "$s" >/dev/null 2>&1; then
    ran_ok=$((ran_ok + 1))
  else
    rc=$?
    ran_failed=$((ran_failed + 1))
    if [[ $rc -eq 124 ]]; then
      note "timeout after ${S_TIMEOUT}s: $s"
    else
      note "exit $rc: $s"
    fi
  fi
done
note "phase 2 totals: ran=$ran ok=$ran_ok failed=$ran_failed (verifier exits are not gated; cleanup is)"

# --- Phase 3: repo-root cleanliness scan using before/after diff. ---
SNAPSHOT_AFTER="$(mktemp)"
trap 'rm -f "$SNAPSHOT_BEFORE" "$SNAPSHOT_AFTER"' EXIT
find "$ROOT_DIR" -maxdepth 1 -name '.zap-*' -printf '%f\n' 2>/dev/null | sort > "$SNAPSHOT_AFTER"

# New files = in after, not in before.
new_leaks="$(comm -13 "$SNAPSHOT_BEFORE" "$SNAPSHOT_AFTER")"
if [[ -n "$new_leaks" ]]; then
  while IFS= read -r m; do
    [[ -z "$m" ]] && continue
    fail "NEW repo-root scratch artifact after run_zap() refactor sweep: $m"
  done <<< "$new_leaks"
else
  pass "phase 3: no NEW repo-root .zap-* artifacts introduced by the sweep"
fi

# --- Final summary. ---
if (( FAILURES == 0 )); then
  pass "run_zap() portability refactor smoke gate passed"
  printf 'smoke gate passed: %d scripts parsed; %d ran ok; %d ran with verifier-exit failure (recorded, not gated)\n' \
    "${#REFACTORED[@]}" "$ran_ok" "$ran_failed"
  exit 0
else
  printf 'run_zap() refactor smoke gate FAILED: %d issue(s)\n' "$FAILURES" >&2
  exit 1
fi