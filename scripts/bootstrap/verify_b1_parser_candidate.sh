#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"

valid_fixture="bootstrap/fixtures/parser/arithmetic.zp"
valid_expected="bootstrap/fixtures/parser/arithmetic.ast.json"
invalid_fixture="bootstrap/fixtures/diagnostics/missing_closing_bracket.zp"
invalid_expected="bootstrap/fixtures/diagnostics/missing_closing_bracket.json"
for path in "$valid_fixture" "$valid_expected" "$invalid_fixture" "$invalid_expected" "bootstrap/b1/parser.zp"; do
  [[ -f "$path" ]] || { printf 'missing parser candidate fixture: %s\n' "$path" >&2; exit 2; }
done

runner=$(mktemp "$ROOT_DIR/.zap-b1-parser-candidate-runner.XXXXXX.zp")
output=$(mktemp "${TMPDIR:-/tmp}/zap-b1-parser-candidate-output.XXXXXX")
expected=$(mktemp "${TMPDIR:-/tmp}/zap-b1-parser-candidate-expected.XXXXXX")
trap 'rm -f "$runner" "$output" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b1/parser.zp"
let valid = read_text("bootstrap/fixtures/parser/arithmetic.zp")
let invalid = read_text("bootstrap/fixtures/diagnostics/missing_closing_bracket.zp")
say parse_or_diagnostics(valid, "bootstrap/fixtures/parser/arithmetic.zp")
say parse_or_diagnostics(invalid, "bootstrap/fixtures/diagnostics/missing_closing_bracket.zp")
EOF
{
  cat "$valid_expected"
  cat "$invalid_expected"
} > "$expected"
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$output"
cmp "$output" "$expected"
printf 'B1 Zap parser candidate differential passed: arithmetic AST and syntax rejection\n'
