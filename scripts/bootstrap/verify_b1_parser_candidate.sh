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
multiplicative_fixture="bootstrap/fixtures/parser/multiplicative_additive.zp"
multiplicative_expected="bootstrap/fixtures/parser/multiplicative_additive.ast.json"
grouped_fixture="bootstrap/fixtures/parser/grouped_expression.zp"
grouped_expected="bootstrap/fixtures/parser/grouped_expression.ast.json"
assignment_fixture="bootstrap/fixtures/parser/assignment_statement.zp"
assignment_expected="bootstrap/fixtures/parser/assignment_statement.ast.json"
logic_fixture="bootstrap/fixtures/parser/logical_comparison_matrix.zp"
logic_expected="bootstrap/fixtures/parser/logical_comparison_matrix.ast.json"
function_fixture="bootstrap/fixtures/parser/simple_function.zp"
function_expected="bootstrap/fixtures/parser/simple_function.ast.json"
loop_fixture="bootstrap/fixtures/parser/simple_loop.zp"
loop_expected="bootstrap/fixtures/parser/simple_loop.ast.json"
class_fixture="bootstrap/fixtures/parser/simple_class.zp"
class_expected="bootstrap/fixtures/parser/simple_class.ast.json"
full_expression_fixture="bootstrap/fixtures/parser/full_expression.zp"
full_expression_expected="bootstrap/fixtures/parser/full_expression.ast.json"
three_declarations_fixture="bootstrap/fixtures/parser/three_declarations.zp"
three_declarations_expected="bootstrap/fixtures/parser/three_declarations.ast.json"
nested_calls_fixture="bootstrap/fixtures/parser/nested_calls.zp"
nested_calls_expected="bootstrap/fixtures/parser/nested_calls.ast.json"
parenthesized_nested_fixture="bootstrap/fixtures/parser/parenthesized_nested.zp"
parenthesized_nested_expected="bootstrap/fixtures/parser/parenthesized_nested.ast.json"
nested_blocks_fixture="bootstrap/fixtures/parser/nested_blocks.zp"
nested_blocks_expected="bootstrap/fixtures/parser/nested_blocks.ast.json"
three_argument_call_fixture="bootstrap/fixtures/parser/three_argument_call.zp"
three_argument_call_expected="bootstrap/fixtures/parser/three_argument_call.ast.json"
control_flow_fixture="bootstrap/fixtures/parser/control_flow.zp"
control_flow_expected="bootstrap/fixtures/parser/control_flow.ast.json"
invalid_indentation_fixture="bootstrap/fixtures/parser/invalid_indentation.zp"
invalid_indentation_expected="bootstrap/fixtures/parser/invalid_indentation.json"
mixed_top_level_fixture="bootstrap/fixtures/parser/mixed_top_level.zp"
mixed_top_level_expected="bootstrap/fixtures/parser/mixed_top_level.ast.json"
nested_function_fixture="bootstrap/fixtures/parser/nested_function_blocks.zp"
nested_function_expected="bootstrap/fixtures/parser/nested_function_blocks.ast.json"
nested_class_method_fixture="bootstrap/fixtures/parser/nested_class_method.zp"
nested_class_method_expected="bootstrap/fixtures/parser/nested_class_method.ast.json"
mixed_recursive_fixture="bootstrap/fixtures/parser/mixed_recursive_sequence.zp"
mixed_recursive_expected="bootstrap/fixtures/parser/mixed_recursive_sequence.ast.json"
while_simple_fixture="bootstrap/fixtures/parser/while_simple.zp"
while_simple_expected="bootstrap/fixtures/parser/while_simple.ast.json"
deep_mixed_fixture="bootstrap/fixtures/parser/deep_mixed_blocks.zp"
deep_mixed_expected="bootstrap/fixtures/parser/deep_mixed_blocks.ast.json"
invalid_fixture="bootstrap/fixtures/diagnostics/missing_closing_bracket.zp"
invalid_expected="bootstrap/fixtures/diagnostics/missing_closing_bracket.json"
unexpected_fixture="bootstrap/fixtures/diagnostics/unexpected_closing_bracket.zp"
unexpected_expected="bootstrap/fixtures/diagnostics/unexpected_closing_bracket.json"
missing_assignment_fixture="bootstrap/fixtures/diagnostics/missing_assignment.zp"
missing_assignment_expected="bootstrap/fixtures/diagnostics/missing_assignment.json"
missing_function_paren_fixture="bootstrap/fixtures/diagnostics/missing_function_paren.zp"
missing_function_paren_expected="bootstrap/fixtures/diagnostics/missing_function_paren.json"
for path in "$valid_fixture" "$valid_expected" "$compound_fixture" "$compound_expected" "$two_fixture" "$two_expected" "$unicode_fixture" "$unicode_expected" "$number_fixture" "$number_expected" "$negative_fixture" "$negative_expected" "$decimal_fixture" "$decimal_expected" "$multiplicative_fixture" "$multiplicative_expected" "$grouped_fixture" "$grouped_expected" "$assignment_fixture" "$assignment_expected" "$logic_fixture" "$logic_expected" "$function_fixture" "$function_expected" "$loop_fixture" "$loop_expected" "$class_fixture" "$class_expected" "$full_expression_fixture" "$full_expression_expected" "$three_declarations_fixture" "$three_declarations_expected" "$nested_calls_fixture" "$nested_calls_expected" "$parenthesized_nested_fixture" "$parenthesized_nested_expected" "$nested_blocks_fixture" "$nested_blocks_expected" "$three_argument_call_fixture" "$three_argument_call_expected" "$control_flow_fixture" "$control_flow_expected" "$invalid_indentation_fixture" "$invalid_indentation_expected" "$mixed_top_level_fixture" "$mixed_top_level_expected" "$nested_function_fixture" "$nested_function_expected" "$nested_class_method_fixture" "$nested_class_method_expected" "$mixed_recursive_fixture" "$mixed_recursive_expected" "$while_simple_fixture" "$while_simple_expected" "$deep_mixed_fixture" "$deep_mixed_expected" "$invalid_fixture" "$invalid_expected" "$unexpected_fixture" "$unexpected_expected" "$missing_assignment_fixture" "$missing_assignment_expected" "$missing_function_paren_fixture" "$missing_function_paren_expected" "bootstrap/b1/parser.zp"; do
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
let multiplicative = read_text("bootstrap/fixtures/parser/multiplicative_additive.zp")
let grouped = read_text("bootstrap/fixtures/parser/grouped_expression.zp")
let assignment = read_text("bootstrap/fixtures/parser/assignment_statement.zp")
let logic = read_text("bootstrap/fixtures/parser/logical_comparison_matrix.zp")
let function = read_text("bootstrap/fixtures/parser/simple_function.zp")
let loop = read_text("bootstrap/fixtures/parser/simple_loop.zp")
let class_source = read_text("bootstrap/fixtures/parser/simple_class.zp")
let full_expression = read_text("bootstrap/fixtures/parser/full_expression.zp")
let three_declarations = read_text("bootstrap/fixtures/parser/three_declarations.zp")
let nested_calls = read_text("bootstrap/fixtures/parser/nested_calls.zp")
let parenthesized_nested = read_text("bootstrap/fixtures/parser/parenthesized_nested.zp")
let nested_blocks = read_text("bootstrap/fixtures/parser/nested_blocks.zp")
let three_argument_call = read_text("bootstrap/fixtures/parser/three_argument_call.zp")
let control_flow = read_text("bootstrap/fixtures/parser/control_flow.zp")
let invalid_indentation = read_text("bootstrap/fixtures/parser/invalid_indentation.zp")
let mixed_top_level = read_text("bootstrap/fixtures/parser/mixed_top_level.zp")
let nested_function = read_text("bootstrap/fixtures/parser/nested_function_blocks.zp")
let nested_class_method = read_text("bootstrap/fixtures/parser/nested_class_method.zp")
let mixed_recursive = read_text("bootstrap/fixtures/parser/mixed_recursive_sequence.zp")
let while_simple = read_text("bootstrap/fixtures/parser/while_simple.zp")
let deep_mixed = read_text("bootstrap/fixtures/parser/deep_mixed_blocks.zp")
let invalid = read_text("bootstrap/fixtures/diagnostics/missing_closing_bracket.zp")
let unexpected = read_text("bootstrap/fixtures/diagnostics/unexpected_closing_bracket.zp")
let missing_assignment = read_text("bootstrap/fixtures/diagnostics/missing_assignment.zp")
let missing_function_paren = read_text("bootstrap/fixtures/diagnostics/missing_function_paren.zp")
let valid_tokens = from_json(lex(valid, "bootstrap/fixtures/parser/arithmetic.zp"))
let compound_tokens = from_json(lex(compound, "bootstrap/fixtures/parser/compound.zp"))
let two_tokens = from_json(lex(two, "bootstrap/fixtures/parser/two_declarations.zp"))
let unicode_tokens = from_json(lex(unicode, "bootstrap/fixtures/parser/unicode_identifier.zp"))
let number_tokens = from_json(lex(number, "bootstrap/fixtures/parser/multi_digit_number.zp"))
let negative_tokens = from_json(lex(negative, "bootstrap/fixtures/parser/negative_number.zp"))
let decimal_tokens = from_json(lex(decimal, "bootstrap/fixtures/parser/numeric_literals.zp"))
let multiplicative_tokens = from_json(lex(multiplicative, "bootstrap/fixtures/parser/multiplicative_additive.zp"))
let grouped_tokens = from_json(lex(grouped, "bootstrap/fixtures/parser/grouped_expression.zp"))
let assignment_tokens = from_json(lex(assignment, "bootstrap/fixtures/parser/assignment_statement.zp"))
let logic_tokens = from_json(lex(logic, "bootstrap/fixtures/parser/logical_comparison_matrix.zp"))
let function_tokens = from_json(lex(function, "bootstrap/fixtures/parser/simple_function.zp"))
let loop_tokens = from_json(lex(loop, "bootstrap/fixtures/parser/simple_loop.zp"))
let class_tokens = from_json(lex(class_source, "bootstrap/fixtures/parser/simple_class.zp"))
let full_expression_tokens = from_json(lex(full_expression, "bootstrap/fixtures/parser/full_expression.zp"))
let three_declarations_tokens = from_json(lex(three_declarations, "bootstrap/fixtures/parser/three_declarations.zp"))
let nested_calls_tokens = from_json(lex(nested_calls, "bootstrap/fixtures/parser/nested_calls.zp"))
let parenthesized_nested_tokens = from_json(lex(parenthesized_nested, "bootstrap/fixtures/parser/parenthesized_nested.zp"))
let nested_blocks_tokens = from_json(lex(nested_blocks, "bootstrap/fixtures/parser/nested_blocks.zp"))
let three_argument_call_tokens = from_json(lex(three_argument_call, "bootstrap/fixtures/parser/three_argument_call.zp"))
let control_flow_tokens = from_json(lex(control_flow, "bootstrap/fixtures/parser/control_flow.zp"))
let invalid_indentation_tokens = from_json(lex(invalid_indentation, "bootstrap/fixtures/parser/invalid_indentation.zp"))
let mixed_top_level_tokens = from_json(lex(mixed_top_level, "bootstrap/fixtures/parser/mixed_top_level.zp"))
let nested_function_tokens = from_json(lex(nested_function, "bootstrap/fixtures/parser/nested_function_blocks.zp"))
let nested_class_method_tokens = from_json(lex(nested_class_method, "bootstrap/fixtures/parser/nested_class_method.zp"))
let mixed_recursive_tokens = from_json(lex(mixed_recursive, "bootstrap/fixtures/parser/mixed_recursive_sequence.zp"))
let while_simple_tokens = from_json(lex(while_simple, "bootstrap/fixtures/parser/while_simple.zp"))
let deep_mixed_tokens = from_json(lex(deep_mixed, "bootstrap/fixtures/parser/deep_mixed_blocks.zp"))
let invalid_tokens = from_json(lex(invalid, "bootstrap/fixtures/diagnostics/missing_closing_bracket.zp"))
let unexpected_tokens = from_json(lex(unexpected, "bootstrap/fixtures/diagnostics/unexpected_closing_bracket.zp"))
let missing_assignment_tokens = from_json(lex(missing_assignment, "bootstrap/fixtures/diagnostics/missing_assignment.zp"))
let missing_function_paren_tokens = from_json(lex(missing_function_paren, "bootstrap/fixtures/diagnostics/missing_function_paren.zp"))
say parse_or_diagnostics(valid, valid_tokens["tokens"], "bootstrap/fixtures/parser/arithmetic.zp")
say parse_or_diagnostics(compound, compound_tokens["tokens"], "bootstrap/fixtures/parser/compound.zp")
say parse_or_diagnostics(two, two_tokens["tokens"], "bootstrap/fixtures/parser/two_declarations.zp")
say parse_or_diagnostics(unicode, unicode_tokens["tokens"], "bootstrap/fixtures/parser/unicode_identifier.zp")
say parse_or_diagnostics(number, number_tokens["tokens"], "bootstrap/fixtures/parser/multi_digit_number.zp")
say parse_or_diagnostics(negative, negative_tokens["tokens"], "bootstrap/fixtures/parser/negative_number.zp")
say parse_or_diagnostics(decimal, decimal_tokens["tokens"], "bootstrap/fixtures/parser/numeric_literals.zp")
say parse_or_diagnostics(multiplicative, multiplicative_tokens["tokens"], "bootstrap/fixtures/parser/multiplicative_additive.zp")
say parse_or_diagnostics(grouped, grouped_tokens["tokens"], "bootstrap/fixtures/parser/grouped_expression.zp")
say parse_or_diagnostics(assignment, assignment_tokens["tokens"], "bootstrap/fixtures/parser/assignment_statement.zp")
say parse_or_diagnostics(logic, logic_tokens["tokens"], "bootstrap/fixtures/parser/logical_comparison_matrix.zp")
say parse_or_diagnostics(function, function_tokens["tokens"], "bootstrap/fixtures/parser/simple_function.zp")
say parse_or_diagnostics(loop, loop_tokens["tokens"], "bootstrap/fixtures/parser/simple_loop.zp")
say parse_or_diagnostics(class_source, class_tokens["tokens"], "bootstrap/fixtures/parser/simple_class.zp")
say parse_or_diagnostics(full_expression, full_expression_tokens["tokens"], "bootstrap/fixtures/parser/full_expression.zp")
say parse_or_diagnostics(three_declarations, three_declarations_tokens["tokens"], "bootstrap/fixtures/parser/three_declarations.zp")
say parse_or_diagnostics(nested_calls, nested_calls_tokens["tokens"], "bootstrap/fixtures/parser/nested_calls.zp")
say parse_or_diagnostics(parenthesized_nested, parenthesized_nested_tokens["tokens"], "bootstrap/fixtures/parser/parenthesized_nested.zp")
say parse_or_diagnostics(nested_blocks, nested_blocks_tokens["tokens"], "bootstrap/fixtures/parser/nested_blocks.zp")
say parse_or_diagnostics(three_argument_call, three_argument_call_tokens["tokens"], "bootstrap/fixtures/parser/three_argument_call.zp")
say parse_or_diagnostics(control_flow, control_flow_tokens["tokens"], "bootstrap/fixtures/parser/control_flow.zp")
say parse_or_diagnostics(invalid_indentation, invalid_indentation_tokens["tokens"], "bootstrap/fixtures/parser/invalid_indentation.zp")
say parse_or_diagnostics(mixed_top_level, mixed_top_level_tokens["tokens"], "bootstrap/fixtures/parser/mixed_top_level.zp")
say parse_or_diagnostics(nested_function, nested_function_tokens["tokens"], "bootstrap/fixtures/parser/nested_function_blocks.zp")
say parse_or_diagnostics(nested_class_method, nested_class_method_tokens["tokens"], "bootstrap/fixtures/parser/nested_class_method.zp")
say parse_or_diagnostics(mixed_recursive, mixed_recursive_tokens["tokens"], "bootstrap/fixtures/parser/mixed_recursive_sequence.zp")
say parse_or_diagnostics(while_simple, while_simple_tokens["tokens"], "bootstrap/fixtures/parser/while_simple.zp")
say parse_or_diagnostics(deep_mixed, deep_mixed_tokens["tokens"], "bootstrap/fixtures/parser/deep_mixed_blocks.zp")
say parse_or_diagnostics(invalid, invalid_tokens["tokens"], "bootstrap/fixtures/diagnostics/missing_closing_bracket.zp")
say parse_or_diagnostics(unexpected, unexpected_tokens["tokens"], "bootstrap/fixtures/diagnostics/unexpected_closing_bracket.zp")
say parse_or_diagnostics(missing_assignment, missing_assignment_tokens["tokens"], "bootstrap/fixtures/diagnostics/missing_assignment.zp")
say parse_or_diagnostics(missing_function_paren, missing_function_paren_tokens["tokens"], "bootstrap/fixtures/diagnostics/missing_function_paren.zp")
EOF
{
  cat "$valid_expected"
  cat "$compound_expected"
  cat "$two_expected"
  cat "$unicode_expected"
  cat "$number_expected"
  cat "$negative_expected"
  cat "$decimal_expected"
  cat "$multiplicative_expected"
  cat "$grouped_expected"
  cat "$assignment_expected"
  cat "$logic_expected"
  cat "$function_expected"
  cat "$loop_expected"
  cat "$class_expected"
  cat "$full_expression_expected"
  cat "$three_declarations_expected"
  cat "$nested_calls_expected"
  cat "$parenthesized_nested_expected"
  cat "$nested_blocks_expected"
  cat "$three_argument_call_expected"
  cat "$control_flow_expected"
  cat "$invalid_indentation_expected"
  cat "$mixed_top_level_expected"
  cat "$nested_function_expected"
  cat "$nested_class_method_expected"
  cat "$mixed_recursive_expected"
  cat "$while_simple_expected"
  cat "$deep_mixed_expected"
  cat "$invalid_expected"
  cat "$unexpected_expected"
  cat "$missing_assignment_expected"
  cat "$missing_function_paren_expected"
} > "$expected"
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$output"
cmp "$output" "$expected"
printf 'B1 Zap parser candidate differential passed: arithmetic AST, compound AST, and token-driven delimiter diagnostics\n'
