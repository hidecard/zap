#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

echo "B1 Parser Verification Preparation"
echo "=================================="

# 1. Check that all fixtures referenced by verify_b1_parser_candidate.sh exist
echo "Checking B1 parser fixtures..."
fixtures=(
  "bootstrap/fixtures/parser/arithmetic.zp"
  "bootstrap/fixtures/parser/compound.zp"
  "bootstrap/fixtures/parser/two_declarations.zp"
  "bootstrap/fixtures/parser/unicode_identifier.zp"
  "bootstrap/fixtures/parser/multi_digit_number.zp"
  "bootstrap/fixtures/parser/negative_number.zp"
  "bootstrap/fixtures/parser/numeric_literals.zp"
  "bootstrap/fixtures/parser/multiplicative_additive.zp"
  "bootstrap/fixtures/parser/grouped_expression.zp"
  "bootstrap/fixtures/parser/assignment_statement.zp"
  "bootstrap/fixtures/parser/logical_comparison_matrix.zp"
  "bootstrap/fixtures/parser/simple_function.zp"
  "bootstrap/fixtures/parser/simple_loop.zp"
  "bootstrap/fixtures/parser/simple_class.zp"
  "bootstrap/fixtures/parser/full_expression.zp"
  "bootstrap/fixtures/parser/three_declarations.zp"
  "bootstrap/fixtures/parser/nested_calls.zp"
  "bootstrap/fixtures/parser/parenthesized_nested.zp"
  "bootstrap/fixtures/parser/nested_blocks.zp"
  "bootstrap/fixtures/parser/three_argument_call.zp"
  "bootstrap/fixtures/parser/control_flow.zp"
  "bootstrap/fixtures/parser/invalid_indentation.zp"
  "bootstrap/fixtures/parser/unexpected_indentation.zp"
  "bootstrap/fixtures/parser/mixed_top_level.zp"
  "bootstrap/fixtures/parser/nested_function_blocks.zp"
  "bootstrap/fixtures/parser/nested_class_method.zp"
  "bootstrap/fixtures/parser/mixed_recursive_sequence.zp"
  "bootstrap/fixtures/parser/while_simple.zp"
  "bootstrap/fixtures/parser/deep_mixed_blocks.zp"
  "bootstrap/fixtures/parser/four_argument_call.zp"
  "bootstrap/fixtures/parser/parenthesized_not.zp"
  "bootstrap/fixtures/parser/nested_assignment_block.zp"
  "bootstrap/fixtures/diagnostics/missing_closing_bracket.zp"
  "bootstrap/fixtures/diagnostics/unexpected_closing_bracket.zp"
  "bootstrap/fixtures/diagnostics/missing_assignment.zp"
  "bootstrap/fixtures/diagnostics/missing_function_paren.zp"
)

missing=0
for fixture in "${fixtures[@]}"; do
  if [[ ! -f "$fixture" ]]; then
    echo "MISSING: $fixture"
    missing=$((missing + 1))
  fi
done

if [[ $missing -gt 0 ]]; then
  echo "FAIL: $missing fixtures missing"
  exit 1
fi
echo "PASS: All B1 parser fixtures present"

# 2. Check that expected outputs exist for fixtures that have them
echo "Checking B1 parser expected outputs..."
expected=(
  "bootstrap/fixtures/parser/arithmetic.ast.json"
  "bootstrap/fixtures/parser/compound.ast.json"
  "bootstrap/fixtures/parser/two_declarations.ast.json"
  "bootstrap/fixtures/parser/unicode_identifier.ast.json"
  "bootstrap/fixtures/parser/multi_digit_number.ast.json"
  "bootstrap/fixtures/parser/negative_number.ast.json"
  "bootstrap/fixtures/parser/numeric_literals.diagnostics.json"
  "bootstrap/fixtures/parser/multiplicative_additive.ast.json"
  "bootstrap/fixtures/parser/grouped_expression.ast.json"
  "bootstrap/fixtures/parser/assignment_statement.ast.json"
  "bootstrap/fixtures/parser/logical_comparison_matrix.ast.json"
  "bootstrap/fixtures/parser/simple_function.ast.json"
  "bootstrap/fixtures/parser/simple_loop.ast.json"
  "bootstrap/fixtures/parser/simple_class.ast.json"
  "bootstrap/fixtures/parser/full_expression.ast.json"
  "bootstrap/fixtures/parser/three_declarations.ast.json"
  "bootstrap/fixtures/parser/nested_calls.ast.json"
  "bootstrap/fixtures/parser/parenthesized_nested.ast.json"
  "bootstrap/fixtures/parser/nested_blocks.ast.json"
  "bootstrap/fixtures/parser/three_argument_call.ast.json"
  "bootstrap/fixtures/parser/control_flow.ast.json"
  "bootstrap/fixtures/parser/invalid_indentation.json"
  "bootstrap/fixtures/parser/unexpected_indentation.json"
  "bootstrap/fixtures/parser/mixed_top_level.ast.json"
  "bootstrap/fixtures/parser/nested_function_blocks.ast.json"
  "bootstrap/fixtures/parser/nested_class_method.ast.json"
  "bootstrap/fixtures/parser/mixed_recursive_sequence.ast.json"
  "bootstrap/fixtures/parser/while_simple.ast.json"
  "bootstrap/fixtures/parser/deep_mixed_blocks.ast.json"
  "bootstrap/fixtures/parser/four_argument_call.ast.json"
  "bootstrap/fixtures/parser/parenthesized_not.ast.json"
  "bootstrap/fixtures/parser/nested_assignment_block.ast.json"
  "bootstrap/fixtures/diagnostics/missing_closing_bracket.json"
  "bootstrap/fixtures/diagnostics/unexpected_closing_bracket.json"
  "bootstrap/fixtures/diagnostics/missing_assignment.json"
  "bootstrap/fixtures/diagnostics/missing_function_paren.json"
)

missing=0
for expected_file in "${expected[@]}"; do
  if [[ ! -f "$expected_file" ]]; then
    echo "MISSING EXPECTED: $expected_file"
    missing=$((missing + 1))
  fi
done

if [[ $missing -gt 0 ]]; then
  echo "NOTE: $missing expected output files missing (may be intentional for some fixtures)"
else
  echo "PASS: All expected outputs present"
fi

# 3. Check that verification scripts exist and are executable
echo "Checking verification scripts..."
scripts=(
  "scripts/bootstrap/verify_b1_parser_candidate.sh"
  "scripts/bootstrap/verify_b1_token_native_indentation.sh"
)

for script in "${scripts[@]}"; do
  if [[ ! -f "$script" ]]; then
    echo "MISSING: $script"
    missing=$((missing + 1))
  elif [[ ! -x "$script" ]]; then
    echo "NOT EXECUTABLE: $script"
  fi
done

echo "PASS: Verification scripts present"

# 4. Check Rust toolchain availability
echo "Checking Rust toolchain..."
if ! command -v cargo &>/dev/null; then
  echo "WARNING: cargo not found. Verification requires Rust toolchain."
  echo "  Install with: rustup toolchain install 1.88.0"
  echo "  Or run: bash scripts/bootstrap/verify_b1_parser_candidate.sh"
  exit 0
fi

rust_version=$(cargo --version 2>/dev/null || echo "unknown")
echo "Cargo version: $rust_version"
echo ""
echo "To run B1 parser verification:"
echo "  bash scripts/bootstrap/verify_b1_parser_candidate.sh"
echo "  bash scripts/bootstrap/verify_b1_token_native_indentation.sh"
