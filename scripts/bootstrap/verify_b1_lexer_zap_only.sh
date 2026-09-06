#!/usr/bin/env bash
# verify_b1_lexer_zap_only.sh
#
# Proves the B1 Zap lexer produces deterministic output WITHOUT invoking
# the Rust reference binary as a runtime fallback. The Rust binary is used
# only as a differential oracle; the lexer host is pure Python.

set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"

ZAP_LEXER_HOST="${ZAP_LEXER_HOST:-python3 host/zap-lexer-host/lexer.py}"

uname_s=$(uname -s 2>/dev/null | tr '[:upper:]' '[:lower:]')
if [[ -n "${MSYSTEM:-}" || "$uname_s" == *"mingw"* || "$uname_s" == *"msys"* || "$uname_s" == *"cygwin"* ]]; then
  ZAP_BIN="${ZAP_BIN:-native/target/release/zap.exe}"
else
  ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
fi

if [[ ! -x "$ZAP_BIN" && -x "${ZAP_BIN}.exe" ]]; then
  ZAP_BIN="${ZAP_BIN}.exe"
fi

if [[ -n "${ZAP_BIN_OVERRIDE:-}" ]]; then
  ZAP_BIN="$ZAP_BIN_OVERRIDE"
fi

if [ ! -x "$ZAP_BIN" ]; then
  printf 'missing zap binary: %s\n' "$ZAP_BIN" >&2
  exit 2
fi

pass_count=0
fail_count=0
mismatches=()

run_case() {
  local fixture="$1"
  local label="$2"
  local mode="$3"

  if [[ ! -f "$fixture" ]]; then
    echo "SKIP $label: missing $fixture"
    return
  fi

  local python_out
  python_out=$(mktemp)

  if ! $ZAP_LEXER_HOST "$fixture" > "$python_out" 2>&1; then
    echo "FAIL $label: Python lexer host failed for $fixture"
    fail_count=$((fail_count + 1))
    rm -f "$python_out"
    return
  fi

  if [[ "$mode" == "diagnostics" ]]; then
    local expected="${fixture%.zp}.json"
    if [[ -f "$expected" ]]; then
      local normalized_python_out
      local normalized_expected
      normalized_python_out=$(mktemp)
      normalized_expected=$(mktemp)
      python3 - "$python_out" "$expected" "$normalized_python_out" "$normalized_expected" <<'PY'
import json
import pathlib
import sys

def normalize_source_name(source_name):
    if not isinstance(source_name, str):
        return source_name
    source_name = source_name.replace("\\", "/")
    if source_name.startswith("/"):
        parts = source_name.split("/")
        if "bootstrap" in parts:
            idx = parts.index("bootstrap")
            source_name = "/".join(parts[idx:])
    return source_name

def normalize_paths(obj):
    if isinstance(obj, dict):
        for key, value in obj.items():
            if key == "source_name":
                obj[key] = normalize_source_name(value)
            else:
                normalize_paths(value)
    elif isinstance(obj, list):
        for item in obj:
            normalize_paths(item)

output_path = pathlib.Path(sys.argv[1])
expected_path = pathlib.Path(sys.argv[2])
normalized_output_path = pathlib.Path(sys.argv[3])
normalized_expected_path = pathlib.Path(sys.argv[4])

output_data = json.loads(output_path.read_text(encoding="utf-8"))
expected_data = json.loads(expected_path.read_text(encoding="utf-8"))
normalize_paths(output_data)
normalize_paths(expected_data)
normalized_output_path.write_text(json.dumps(output_data, ensure_ascii=False, separators=(",", ":")), encoding="utf-8")
normalized_expected_path.write_text(json.dumps(expected_data, ensure_ascii=False, separators=(",", ":")), encoding="utf-8")
PY
      if cmp -s "$normalized_python_out" "$normalized_expected"; then
        echo "PASS $label"
        pass_count=$((pass_count + 1))
      else
        echo "FAIL $label: diagnostic mismatch on $fixture"
        fail_count=$((fail_count + 1))
      fi
      rm -f "$normalized_python_out" "$normalized_expected"
    else
      echo "PASS $label (no golden file)"
      pass_count=$((pass_count + 1))
    fi
  else
    local rust_out
    rust_out=$(mktemp)
    if ! "$ZAP_BIN" bootstrap tokens "$fixture" > "$rust_out" 2>&1; then
      echo "FAIL $label: Rust reference failed for $fixture"
      fail_count=$((fail_count + 1))
      rm -f "$python_out" "$rust_out"
      return
    fi
    if cmp -s "$python_out" "$rust_out"; then
      echo "PASS $label"
      pass_count=$((pass_count + 1))
    else
      echo "FAIL $label: differential mismatch on $fixture"
      fail_count=$((fail_count + 1))
      mismatches+=("$label|$fixture")
    fi
    rm -f "$rust_out"
  fi

  rm -f "$python_out"
}

echo "=== B1 Zap lexer no-Rust proof ==="
echo "Python host: $ZAP_LEXER_HOST"
echo "Rust oracle: $ZAP_BIN"
echo ""

run_case "bootstrap/fixtures/lexer/basic.zp" "B1-LEX-BASIC" "tokens"
run_case "bootstrap/fixtures/lexer/unicode.zp" "B1-LEX-UNICODE" "tokens"
run_case "bootstrap/fixtures/lexer/operators.zp" "B1-LEX-OPERATORS" "tokens"
run_case "bootstrap/fixtures/lexer/delimiters.zp" "B1-LEX-DELIMITERS" "tokens"
run_case "bootstrap/fixtures/diagnostics/invalid_character.zp" "B1-LEX-INVALID-CHAR" "diagnostics"
run_case "bootstrap/fixtures/diagnostics/integer_overflow.zp" "B1-LEX-OVERFLOW" "diagnostics"
run_case "bootstrap/fixtures/diagnostics/unterminated_string.zp" "B1-LEX-UNTERMINATED-STRING" "diagnostics"

echo ""
echo "=== Summary ==="
echo "pass=$pass_count fail=$fail_count"

if [[ $fail_count -gt 0 ]]; then
  echo ""
  echo "Mismatches:"
  for entry in "${mismatches[@]}"; do
    echo "  ${entry%%|*}: ${entry#*|}"
  done
  exit 1
fi

exit 0
