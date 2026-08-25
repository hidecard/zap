#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"

valid_fixture="bootstrap/fixtures/parser/arithmetic.zp"
valid_expected="bootstrap/fixtures/parser/arithmetic.ast.json"
compound_fixture="bootstrap/fixtures/parser/compound.zp"
compound_expected="bootstrap/fixtures/parser/compound.ast.json"
two_fixture="bootstrap/fixtures/parser/two_declarations.zp"
two_expected="bootstrap/fixtures/parser/two_declarations.ast.json"
unicode_fixture="bootstrap/fixtures/parser/unicode_identifier.zp"
unicode_expected="bootstrap/fixtures/parser/unicode_identifier.ast.json"
number_fixture="bootstrap/fixtures/parser/multi_digit_number.zp"
number_expected="bootstrap/fixtures/parser/multi_digit_number.ast.json"
negative_fixture="bootstrap/fixtures/parser/negative_number.zp"
negative_expected="bootstrap/fixtures/parser/negative_number.ast.json"
decimal_fixture="bootstrap/fixtures/parser/numeric_literals.zp"
decimal_expected="bootstrap/fixtures/parser/numeric_literals.diagnostics.json"
invalid_fixture="bootstrap/fixtures/diagnostics/missing_closing_bracket.zp"
invalid_expected="bootstrap/fixtures/diagnostics/missing_closing_bracket.json"
unexpected_fixture="bootstrap/fixtures/diagnostics/unexpected_closing_bracket.zp"
unexpected_expected="bootstrap/fixtures/diagnostics/unexpected_closing_bracket.json"
missing_assignment_fixture="bootstrap/fixtures/diagnostics/missing_assignment.zp"
missing_assignment_expected="bootstrap/fixtures/diagnostics/missing_assignment.json"
for path in "$valid_fixture" "$valid_expected" "$compound_fixture" "$compound_expected" "$two_fixture" "$two_expected" "$unicode_fixture" "$unicode_expected" "$number_fixture" "$number_expected" "$negative_fixture" "$negative_expected" "$decimal_fixture" "$decimal_expected" "$invalid_fixture" "$invalid_expected" "$unexpected_fixture" "$unexpected_expected" "$missing_assignment_fixture" "$missing_assignment_expected" "bootstrap/b1/parser.zp"; do
  [[ -f "$path" ]] || { printf 'missing parser candidate fixture: %s\n' "$path" >&2; exit 2; }
done

runner=$(mktemp "$ROOT_DIR/.zap-b1-parser-candidate-runner.XXXXXX.zp")
output=$(mktemp "${TMPDIR:-/tmp}/zap-b1-parser-candidate-output.XXXXXX")
expected=$(mktemp "${TMPDIR:-/tmp}/zap-b1-parser-candidate-expected.XXXXXX")
trap 'rm -f "$runner" "$output" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b1/lexer.zp"
import "bootstrap/b1/parser.zp"
let valid = read_text("bootstrap/fixtures/parser/arithmetic.zp")
let compound = read_text("bootstrap/fixtures/parser/compound.zp")
let two = read_text("bootstrap/fixtures/parser/two_declarations.zp")
let unicode = read_text("bootstrap/fixtures/parser/unicode_identifier.zp")
let number = read_text("bootstrap/fixtures/parser/multi_digit_number.zp")
let negative = read_text("bootstrap/fixtures/parser/negative_number.zp")
let decimal = read_text("bootstrap/fixtures/parser/numeric_literals.zp")
let invalid = read_text("bootstrap/fixtures/diagnostics/missing_closing_bracket.zp")
let unexpected = read_text("bootstrap/fixtures/diagnostics/unexpected_closing_bracket.zp")
let missing_assignment = read_text("bootstrap/fixtures/diagnostics/missing_assignment.zp")
let valid_tokens = from_json(lex(valid, "bootstrap/fixtures/parser/arithmetic.zp"))
let compound_tokens = from_json(lex(compound, "bootstrap/fixtures/parser/compound.zp"))
let two_tokens = from_json(lex(two, "bootstrap/fixtures/parser/two_declarations.zp"))
let unicode_tokens = from_json(lex(unicode, "bootstrap/fixtures/parser/unicode_identifier.zp"))
let number_tokens = from_json(lex(number, "bootstrap/fixtures/parser/multi_digit_number.zp"))
let negative_tokens = from_json(lex(negative, "bootstrap/fixtures/parser/negative_number.zp"))
let decimal_tokens = from_json(lex(decimal, "bootstrap/fixtures/parser/numeric_literals.zp"))
let invalid_tokens = from_json(lex(invalid, "bootstrap/fixtures/diagnostics/missing_closing_bracket.zp"))
let unexpected_tokens = from_json(lex(unexpected, "bootstrap/fixtures/diagnostics/unexpected_closing_bracket.zp"))
let missing_assignment_tokens = from_json(lex(missing_assignment, "bootstrap/fixtures/diagnostics/missing_assignment.zp"))
say parse_or_diagnostics(valid, valid_tokens["tokens"], "bootstrap/fixtures/parser/arithmetic.zp")
say parse_or_diagnostics(compound, compound_tokens["tokens"], "bootstrap/fixtures/parser/compound.zp")
say parse_or_diagnostics(two, two_tokens["tokens"], "bootstrap/fixtures/parser/two_declarations.zp")
say parse_or_diagnostics(unicode, unicode_tokens["tokens"], "bootstrap/fixtures/parser/unicode_identifier.zp")
say parse_or_diagnostics(number, number_tokens["tokens"], "bootstrap/fixtures/parser/multi_digit_number.zp")
say parse_or_diagnostics(negative, negative_tokens["tokens"], "bootstrap/fixtures/parser/negative_number.zp")
say parse_or_diagnostics(decimal, decimal_tokens["tokens"], "bootstrap/fixtures/parser/numeric_literals.zp")
say parse_or_diagnostics(invalid, invalid_tokens["tokens"], "bootstrap/fixtures/diagnostics/missing_closing_bracket.zp")
say parse_or_diagnostics(unexpected, unexpected_tokens["tokens"], "bootstrap/fixtures/diagnostics/unexpected_closing_bracket.zp")
say parse_or_diagnostics(missing_assignment, missing_assignment_tokens["tokens"], "bootstrap/fixtures/diagnostics/missing_assignment.zp")
EOF
{
  cat "$valid_expected"
  cat "$compound_expected"
  cat "$two_expected"
  cat "$unicode_expected"
  cat "$number_expected"
  cat "$negative_expected"
  cat "$decimal_expected"
  cat "$invalid_expected"
  cat "$unexpected_expected"
  cat "$missing_assignment_expected"
} > "$expected"
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$output"
cmp "$output" "$expected"
printf 'B1 Zap parser candidate differential passed: arithmetic AST, compound AST, and token-driven delimiter diagnostics\n'
