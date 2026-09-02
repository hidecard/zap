#!/usr/bin/env bash
# Aggregate runner for all B1 parser gates.
# Extracts the runner.zp heredoc from each verify_b1_*.sh, executes it
# via the prebuilt native binary, and checks the expected output patterns.
# Usage: bash scripts/bootstrap/aggregate_b1_parser_gates.sh
set -uo pipefail

cd "$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ZAP_BIN="$(pwd)/native/target/release/zap.exe"
SCRIPTS_DIR=scripts/bootstrap

if [ ! -x "$ZAP_BIN" ]; then
  printf 'missing zap binary: %s\n' "$ZAP_BIN" >&2
  exit 2
fi

PASS=0
FAIL=0
SKIP=0
FAILED_GATES=()

for gate in "$SCRIPTS_DIR"/verify_b1_*.sh; do
    name=$(basename "$gate" .sh)
    [ "$name" = "aggregate_b1_parser_gates" ] && continue
    runner_content=$(awk '/cat > "\$runner" <<(.|EOF)/{flag=1; next} /^EOF$/{flag=0} flag' "$gate")
    if [ -z "$runner_content" ]; then
        printf 'SKIP: %s (no runner.zp heredoc found; gate uses unquoted heredoc or external runner script)\n' "$name"
        SKIP=$((SKIP+1))
        continue
    fi
    runner_file=".${name}_runner.zp"
    printf '%s\n' "$runner_content" > "$runner_file"
    output=$("$ZAP_BIN" "$runner_file" 2>&1)
    RC=$?
    rm -f "$runner_file"
    if [ $RC -ne 0 ]; then
        printf 'FAIL: %s (zap.exe exit %d)\n' "$name" "$RC"
        printf '%s\n' "$output" | head -3 | sed 's/^/  /'
        FAIL=$((FAIL+1))
        FAILED_GATES+=("$name")
        continue
    fi
    expected_failures=()
    while IFS= read -r line; do
        if [[ "$line" =~ grep\ -q\ \"([^\"]+)\"\ \"\\\$output\"  ]]; then
            pattern="${BASH_REMATCH[1]}"
            if ! echo "$output" | grep -qF "$pattern"; then
                expected_failures+=("$pattern")
            fi
        fi
    done < <(grep 'grep -q' "$gate")
    if [ ${#expected_failures[@]} -eq 0 ]; then
        printf 'PASS: %s\n' "$name"
        PASS=$((PASS+1))
    else
        printf 'FAIL: %s (missing patterns: %s)\n' "$name" "${expected_failures[*]}"
        FAIL=$((FAIL+1))
        FAILED_GATES+=("$name")
    fi
done

printf '\n=== B1 Parser Gate Aggregate ===\n'
printf 'PASS: %d\n' "$PASS"
printf 'FAIL: %d\n' "$FAIL"
printf 'SKIP: %d\n' "$SKIP"
if [ $FAIL -gt 0 ]; then
    printf 'Failed gates:\n'
    for g in "${FAILED_GATES[@]}"; do printf '  - %s\n' "$g"; done
fi
