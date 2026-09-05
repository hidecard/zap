#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"

RUNNER="scripts/bootstrap/_run_parser.py"

echo "=== B1 Zap parser no-Rust proof ==="
echo "Python host: python3 $RUNNER"

pass=0
fail=0

run_parser() {
  local mode=$1
  local fixture=$2
  local expected=$3

  if [[ ! -f "$fixture" ]]; then
    echo "SKIP $fixture (missing)"
    return
  fi
  if [[ ! -f "$expected" ]]; then
    echo "SKIP $fixture (expected artifact missing: $expected)"
    return
  fi

  python3 "$RUNNER" "$mode" "$fixture" 2>/dev/null || true

  if [[ ! -f "$ROOT_DIR/.zap-parser-actual.txt" ]]; then
    echo "FAIL $fixture (invalid JSON output)"
    fail=$((fail + 1))
    return
  fi

  if python3 -c "
import json, sys
with open(sys.argv[1], encoding='utf-8') as f:
    expected = json.load(f)
with open(sys.argv[2], encoding='utf-8') as f:
    actual = json.load(f)
sys.exit(0 if expected == actual else 1)
" "$expected" "$ROOT_DIR/.zap-parser-actual.txt" 2>/dev/null; then
    echo "PASS $fixture"
    pass=$((pass + 1))
  else
    echo "FAIL $fixture (output mismatch)"
    fail=$((fail + 1))
  fi
}

echo ""
echo "--- AST fixtures ---"
for fixture in bootstrap/fixtures/parser/*.zp; do
  [[ -f "$fixture" ]] || continue
  base=$(basename "$fixture" .zp)
  expected="bootstrap/fixtures/parser/${base}.ast.json"
  run_parser "ast" "$fixture" "$expected"
done

echo ""
echo "--- Diagnostic fixtures ---"
for fixture in bootstrap/fixtures/diagnostics/*.zp; do
  [[ -f "$fixture" ]] || continue
  base=$(basename "$fixture" .zp)
  expected="bootstrap/fixtures/diagnostics/${base}.json"
  run_parser "diagnostics" "$fixture" "$expected"
done

rm -f "$ROOT_DIR/.zap-parser-actual.txt"

echo ""
echo "=== Summary ==="
echo "pass=$pass fail=$fail"

if [[ "$fail" -gt 0 ]]; then
  exit 1
fi
