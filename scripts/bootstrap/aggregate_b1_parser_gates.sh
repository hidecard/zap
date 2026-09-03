#!/usr/bin/env bash
# Aggregate runner for all B1 parser gates.
# Extracts the runner.zp heredoc from each verify_b1_*.sh, executes it
# via the prebuilt native binary, and checks the expected output patterns.
# Usage: bash scripts/bootstrap/aggregate_b1_parser_gates.sh
set -uo pipefail

cd "$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPTS_DIR=scripts/bootstrap
ROOT_DIR=$(pwd)

# Detect platform and set appropriate binary name.
# Use MSYSTEM/mingw/msys/cygwin detection for Windows-bash environments,
# otherwise fall back to uname. Allow override via ZAP_BIN_OVERRIDE.
uname_s=$(uname -s 2>/dev/null | tr '[:upper:]' '[:lower:]')
if [[ -n "${MSYSTEM:-}" || "$uname_s" == *"mingw"* || "$uname_s" == *"msys"* || "$uname_s" == *"cygwin"* ]]; then
  ZAP_BIN="$ROOT_DIR/native/target/release/zap.exe"
else
  case "$uname_s" in
    linux*)     ZAP_BIN="$ROOT_DIR/native/target/release/zap" ;;
    darwin*)    ZAP_BIN="$ROOT_DIR/native/target/release/zap" ;;
    *)          ZAP_BIN="$ROOT_DIR/native/target/release/zap" ;;
  esac
fi

if [[ -n "${ZAP_BIN_OVERRIDE:-}" ]]; then
  ZAP_BIN="$ZAP_BIN_OVERRIDE"
fi

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
    # Extract runner heredoc content. The opener regex accepts both
    # `cat > "$runner"` and `cat >"$runner"` (some gates omit the space) plus
    # any of `<<EOF`, `<<'EOF'`, `<<'ZAP'` (and other uppercase tag
    # delimiters). The closing delimiter is captured from the opener line and
    # used as the closing match (so EOF/ZAP/etc. all work). We strip CR from
    # each line so files stored with CRLF line endings don't trip up the
    # closing delimiter match (and so the extracted runner content uses LF
    # only, avoiding spurious indentation issues from \r being counted as
    # content).
    runner_content=$(awk '
        BEGIN { delim = "" }
        {
            # Strip CR so CRLF files work on Windows.
            sub(/\r$/, "")
            # Detect opener: cat >"<runner>" <<TAG or cat > "<runner>" <<TAG
            # where TAG may be EOF, ZAP, etc., optionally wrapped in quotes.
            if (delim == "" &&
                match($0, /cat >[[:space:]]*"\$runner"[[:space:]]*<</) > 0) {
                # Extract text after << on this line.
                tail = substr($0, RSTART + RLENGTH)
                # Skip optional whitespace, optional opening quote.
                sub(/^[[:space:]]+/, "", tail)
                q = substr(tail, 1, 1)
                if (q == "\047" || q == "\042") {
                    # Quoted delimiter: <<'\XEOF' or <<"\XEOF"
                    endq = index(substr(tail, 2), q)
                    if (endq > 0) {
                        delim = substr(tail, 2, endq - 1)
                    } else {
                        delim = substr(tail, 2)
                    }
                } else {
                    # Bare delimiter: <<EOF
                    delim = tail
                }
                flag = 1
                next
            }
            # Closing delimiter line matches exactly (already CR-stripped).
            if (flag == 1 && $0 == delim) {
                flag = 0
                delim = ""
                next
            }
            if (flag == 1) {
                print
            }
        }
    ' "$gate")
    if [ -z "$runner_content" ]; then
        printf 'SKIP: %s (gate does not use the cat > "<file>" heredoc pattern this runner extracts)\n' "$name"
        SKIP=$((SKIP+1))
        continue
    fi
    runner_file=".${name}_runner.zp"
    printf '%s\n' "$runner_content" > "$runner_file"
    # Substitute path placeholders (__FIXTURE__ etc.) with a representative
    # fixture so gates that loop over fixtures can still execute under the
    # aggregate runner. We default to the lexer `basic.zp` since the only
    # gate using this pattern is verify_b1_lexer.sh.
    if grep -q '__FIXTURE__' "$runner_file"; then
        sed -i 's|__FIXTURE__|bootstrap/fixtures/lexer/basic.zp|g' "$runner_file"
    fi
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
    exit 1
fi
exit 0