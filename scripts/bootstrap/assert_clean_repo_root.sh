#!/usr/bin/env bash
# Zap P0 CI assertion: bootstrap verifier temporary-file hygiene.
#
# Runs a representative subset of the bootstrap verifier suite, then
# inspects the repo root for any leftover scratch artifacts that were not
# declared as transient by .gitignore. Fails nonzero (exit 1) on any
# offending path so a leaked mktemp or absent EXIT trap is caught in the
# same CI run that produced it.
#
# Patterns declared transient by .gitignore (/* zp, rustup_*.snap,
# rustup_*.assert) are NOT treated as leaks because the bootstrap
# pipeline may legitimately leave them behind between ad-hoc runs and
# they remain untracked.
#
# Anything else matching the leaked-pattern list below is a leak.

set -euo pipefail
IFS=$'\n\t'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

if [[ -f "$HOME/.cargo/env" ]]; then
  source "$HOME/.cargo/env"
fi

pass()  { printf 'PASS: %s\n' "$1"; }
fail()  { printf 'FAIL: %s\n' "$1" >&2; LEAKS=1; }
note()  { printf 'NOTE: %s\n' "$1"; }

LEAKS=0

# --- 1. Run a representative subset of the bootstrap verifier suite. ---
# The PRIMARY subset covers the three scripts that previously lacked a
# complete EXIT trap for $ROOT_DIR scratch files and were hardened in
# this change.
#
# The SECONDARY subset exercises scripts that already had a complete
# trap but were refactored to use the run_zap() helper, so the gate
# proves the in-flight refactor remains cleanup-safe end-to-end.
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
      # Verifier failure is reported separately; cleanup behavior is still
      # validated below by the leak-pattern check.
      note "$script exited with $rc (trap path still validated below)"
    fi
  done
}

printf 'Zap bootstrap repo-root cleanliness gate\n'
printf 'Repository: %s\n' "$ROOT_DIR"
printf '%-18s %s\n' 'CHECK' 'RESULT'
printf '%-18s %s\n' '-----' '------'

run_subset primary "${PRIMARY_SUBSET[@]}"
run_subset secondary "${SECONDARY_SUBSET[@]}"

# --- 2. Force a failure path to exercise EXIT trap on abnormal exit. ---
# Trigger the lexer script with a bad fixture so it exits non-zero while
# having already created a $ROOT_DIR scratch file; the trap must still
# clean it up.
printf 'running forced-failure path: verify_b1_lexer.sh BAD_NONEXISTENT.zp\n'
bash scripts/bootstrap/verify_b1_lexer.sh BAD_NONEXISTENT.zp >/dev/null 2>&1 || true

# --- 3. Inspect repo root for leftover scratch artifacts. ---
# Patterns considered transient (allowed by .gitignore and not flagged):
#   ./*.zp
#   ./rustup_*.snap
#   ./rustup_*.assert
# Patterns that indicate a leaked temp file (must be empty):
LEAK_PATTERNS=(
  '.zap-b1-runner.????????.zp'
  '.zap-b1-general-parser.????????.zp'
  '.zap-b1-token-expression.????????.zp'
  '.zap-b1-traits-parser.????????.zp'
  '.zap-b1-branches.????????.zp'
  '.zap-b1-class-methods.????????.zp'
  '.zap-arbitrary-blocks-runner.????????.zp'
  '.zap-boundary-runner.????????.zp'
  '.zap-branch-chain-runner.????????.zp'
  '.zap-control-flow-runner.????????.zp'
  '.zap-decls.????????.zp'
  '.zap-recursive-block-runner.????????.zp'
  '.zap-parser-stmt.????????.zp'
  '.zap-token-cursor-runner.????????.zp'
  '.zap-token-native-runner.????????.zp'
  '.zap-diff.????????'
  '.zap-byte-det.????????.zp'
  '.zap-clean-env.????????.zp'
  '.zap-rebuild-bytes.????????.zp'
  '.zap-runner.????????.zp'
  '.zap-source-vm.????????.zp'
  '.zap-b4-*.????????.zp'
  '.zap-b3-*.????????.zp'
  '.zap-b2-*.????????.zp'
  '.zap-a*.????????.zp'
  '.zap-p2-*.????????.zp'
)

for pat in "${LEAK_PATTERNS[@]}"; do
  matches=$(find "$ROOT_DIR" -maxdepth 1 -name "$pat" -print 2>/dev/null | head -20)
  if [[ -n "$matches" ]]; then
    while IFS= read -r m; do
      [[ -z "$m" ]] && continue
      fail "leaked repo-root scratch artifact: $m"
    done <<< "$matches"
  fi
done

# Generic catch-all: any hidden file in repo root whose name starts with
# ".zap-" that wasn't matched above. .gitignore only allows plain "./*.zp"
# (no leading dot), so anything like ".zap-*" is an unexpected scratch file.
unexpected=$(find "$ROOT_DIR" -maxdepth 1 -name '.zap-*' -print 2>/dev/null)
if [[ -n "$unexpected" ]]; then
  while IFS= read -r m; do
    [[ -z "$m" ]] && continue
    fail "unexpected hidden repo-root scratch: $m"
  done <<< "$unexpected"
fi

# --- 4. Report final result. ---
if (( LEAKS == 0 )); then
  pass "no leaked repo-root scratch artifacts detected"
  printf 'bootstrap repo-root cleanliness gate passed\n'
  exit 0
else
  printf 'bootstrap repo-root cleanliness gate FAILED: %d leak(s)\n' "$LEAKS" >&2
  exit 1
fi