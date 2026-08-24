#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

fixtures=(
  bootstrap/fixtures/lexer/hello.zp
  bootstrap/fixtures/parser/precedence.zp
  bootstrap/fixtures/typecheck/list_number.zp
  bootstrap/fixtures/typecheck/type_error.zp
  bootstrap/fixtures/stdlib/pure_values.zp
)

for fixture in "${fixtures[@]}"; do
  test -s "$fixture"
done

test -s bootstrap/contracts/BOOTSTRAP_CONTRACT_EN.md
test -s bootstrap/contracts/BOOTSTRAP_CONTRACT_MM.md

grep -q 'B0' bootstrap/contracts/BOOTSTRAP_CONTRACT_EN.md
grep -q 'B1' bootstrap/contracts/BOOTSTRAP_CONTRACT_EN.md
grep -q 'B0' bootstrap/contracts/BOOTSTRAP_CONTRACT_MM.md
grep -q 'B1' bootstrap/contracts/BOOTSTRAP_CONTRACT_MM.md

if [[ -n "${ZAP_BIN:-}" ]]; then
  test -x "$ZAP_BIN"
  output="$($ZAP_BIN bootstrap/fixtures/lexer/hello.zp)"
  grep -q '^hello$' <<<"$output"
  grep -q '^42$' <<<"$output"
  "$ZAP_BIN" bootstrap/fixtures/parser/precedence.zp >/dev/null
  "$ZAP_BIN" bootstrap/fixtures/typecheck/list_number.zp >/dev/null
  "$ZAP_BIN" bootstrap/fixtures/stdlib/pure_values.zp >/dev/null
  if "$ZAP_BIN" bootstrap/fixtures/typecheck/type_error.zp >/dev/null 2>&1; then
    echo "bootstrap typecheck fixture unexpectedly succeeded" >&2
    exit 1
  fi
  echo "bootstrap contract execution passed: binary=$ZAP_BIN"
else
  echo "bootstrap contract structure passed: ${#fixtures[@]} fixtures"
fi
