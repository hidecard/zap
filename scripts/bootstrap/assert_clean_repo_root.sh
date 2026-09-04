#!/usr/bin/env bash
# Zap P0 CI assertion: bootstrap verifier temporary-file hygiene.
#
# Runs a representative subset of the bootstrap verifier suite, then
# inspects the repo root for any scratch artifacts INTRODUCED by the run
# (NEW files appearing after the gate started). Fails nonzero (exit 1)
# on any offending path so a leaked mktemp or absent EXIT trap is caught
# in the same CI run that produced it.
#
# The before/after snapshot pattern means this gate stays correct in
# developer environments where `zap lsp` (or another background
# process) is creating its own .zap-*.zp files in the workspace root;
# those files existed before the gate ran, so the gate ignores them.
# In CI (no LSP, no LSP) every pre-existing file is recorded as
# snapshot baseline and only script-induced ones are flagged.
#
# Patterns declared transient by .gitignore (/*.zp, rustup_*.snap,
# rustup_*.assert) are NOT treated as leaks. The gate only inspects
# the LEADING-DOT (.zap-*) family because .gitignore only allows plain
# "./*.zp" (no leading dot), so any ".zap-*" file is an unexpected
# scratch file from the viewpoint of the bootstrap pipeline.

set -uo pipefail
IFS=$'\n\t'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

if [[ -f "$HOME/.cargo/env" ]]; then
  source "$HOME/.cargo/env"
fi

pass()  { printf 'PASS: %s\n' "$1"; }
fail()  { printf 'FAIL: %s\n' "$1" >&2; LEAKS=$((LEAKS + 1)); }
note()  { printf 'NOTE: %s\n' "$1"; }

LEAKS=0

# --- 1. Snapshot the repo root BEFORE running the subset. ---
# This distinguishes NEW leaks (script-induced, this run) from
# pre-existing scratch files (e.g. from `zap lsp`, an interactive
# shell, or a developer ad-hoc run). The before/after diff is what
# keeps this gate correct in non-CI environments.
SNAPSHOT_BEFORE="$(mktemp)"
trap 'rm -f "$SNAPSHOT_BEFORE" "$SNAPSHOT_AFTER"' EXIT
find "$ROOT_DIR" -maxdepth 1 -name '.zap-*' -printf '%f\n' 2>/dev/null | sort > "$SNAPSHOT_BEFORE"
before_count=$(wc -l < "$SNAPSHOT_BEFORE")

# --- 2. Run a representative subset of the bootstrap verifier suite. ---
# PRIMARY covers the three scripts that previously lacked a complete
# EXIT trap for $ROOT_DIR scratch files and were hardened.
#
# SECONDARY exercises scripts that already had a complete trap but
# were refactored to use the run_zap() helper, so the gate proves
# the in-flight refactor remains cleanup-safe end-to-end.
declare -a PRIMARY_SUBSET=(
  scripts/bootstrap/verify_b1_lexer.sh
  scripts/bootstrap/verify_b1_general_parser.sh
  scripts/bootstrap/verify_b1_token_native_indentation.sh
)
declare -a SECONDARY_SUBSET=(
  scripts/bootstrap/verify_b1_branch_chain.sh
  scripts/bootstrap/verify_b1_parser_candidate.sh
  scripts/bootstrap/verify_b3_foundations.sh
  scripts/bootstrap/verify_b4_byte_determinism.sh
  scripts/bootstrap/verify_reference_differential_matrix_24.sh
)

run_subset() {
  local label="$1"
  shift
  local script
  for script in "$@"; do
    if [[ ! -x "$script" ]]; then
      fail "subset script not executable: $script"
      continue
    fi
    printf 'running [%s] %s\n' "$label" "$script"
    if bash "$script" >/dev/null; then
      pass "$script"
    else
      rc=$?
      note "$script exited with $rc (trap path still validated below)"
    fi
  done
}

printf 'Zap bootstrap repo-root cleanliness gate\n'
printf 'Repository: %s\n' "$ROOT_DIR"
printf '%-18s %s\n' 'CHECK' 'RESULT'
printf '%-18s %s\n' '-----' '------'
note "baseline: $before_count pre-existing .zap-* scratch files in repo root"

run_subset primary "${PRIMARY_SUBSET[@]}"
run_subset secondary "${SECONDARY_SUBSET[@]}"

# --- 3. Force a failure path to exercise EXIT trap on abnormal exit. ---
printf 'running forced-failure path: verify_b1_lexer.sh BAD_NONEXISTENT.zp\n'
bash scripts/bootstrap/verify_b1_lexer.sh BAD_NONEXISTENT.zp >/dev/null 2>&1 || true

# --- 4. Inspect repo root for NEW scratch artifacts introduced by the run. ---
SNAPSHOT_AFTER="$(mktemp)"
trap 'rm -f "$SNAPSHOT_BEFORE" "$SNAPSHOT_AFTER"' EXIT
find "$ROOT_DIR" -maxdepth 1 -name '.zap-*' -printf '%f\n' 2>/dev/null | sort > "$SNAPSHOT_AFTER"

# New files = in after, not in before. Use comm -13 to suppress column 1 (only-in-before)
# and column 2 (only-in-both when? actually comm outputs 3 columns by default: 1=only-file1,
# 2=only-file2, 3=both). -13 suppresses cols 1 and 3, leaving col 2 = only-in-file2 = NEW.
new_leaks="$(comm -13 "$SNAPSHOT_BEFORE" "$SNAPSHOT_AFTER")"
if [[ -n "$new_leaks" ]]; then
  while IFS= read -r m; do
    [[ -z "$m" ]] && continue
    fail "NEW repo-root scratch artifact introduced by this gate: $m"
  done <<< "$new_leaks"
else
  pass "no NEW repo-root .zap-* artifacts introduced by the run"
fi

# --- 5. Report final result. ---
if (( LEAKS == 0 )); then
  pass "no leaked repo-root scratch artifacts detected"
  printf 'bootstrap repo-root cleanliness gate passed\n'
  exit 0
else
  printf 'bootstrap repo-root cleanliness gate FAILED: %d leak(s)\n' "$LEAKS" >&2
  exit 1
fi