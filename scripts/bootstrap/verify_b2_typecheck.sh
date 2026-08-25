#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"

valid_ir_fixture="bootstrap/fixtures/typecheck/annotated.zp"
valid_ir_expected="bootstrap/fixtures/typecheck/annotated.typed-ir.json"
for path in "$valid_ir_fixture" "$valid_ir_expected" bootstrap/fixtures/typecheck/incompatible.zp bootstrap/fixtures/typecheck/conditional.zp bootstrap/fixtures/typecheck/function.zp bootstrap/fixtures/typecheck/function_incompatible.zp bootstrap/fixtures/typecheck/collection_incompatible.zp bootstrap/fixtures/typecheck/nested_collection.zp bootstrap/fixtures/typecheck/nested_collection_incompatible.zp bootstrap/fixtures/typecheck/map_collection.zp bootstrap/fixtures/typecheck/map_collection_incompatible.zp bootstrap/fixtures/typecheck/branch_narrowing.zp bootstrap/fixtures/typecheck/branch_narrowing_incompatible.zp bootstrap/fixtures/typecheck/loop_narrowing.zp bootstrap/fixtures/typecheck/loop_narrowing_incompatible.zp bootstrap/fixtures/typecheck/else_narrowing.zp bootstrap/fixtures/typecheck/else_narrowing_incompatible.zp bootstrap/fixtures/typecheck/bool_annotation.zp bootstrap/fixtures/typecheck/bool_annotation_incompatible.zp bootstrap/fixtures/typecheck/none_annotation.zp bootstrap/fixtures/typecheck/none_annotation_incompatible.zp bootstrap/fixtures/typecheck/list_annotation.zp bootstrap/fixtures/typecheck/list_annotation_incompatible.zp bootstrap/fixtures/typecheck/map_annotation.zp bootstrap/fixtures/typecheck/map_annotation_incompatible.zp bootstrap/fixtures/typecheck/option_annotation.zp bootstrap/fixtures/typecheck/option_annotation_incompatible.zp bootstrap/fixtures/typecheck/expression_number_add.zp bootstrap/fixtures/typecheck/expression_number_add_incompatible.zp bootstrap/fixtures/typecheck/expression_text_add.zp bootstrap/fixtures/typecheck/expression_text_add_incompatible.zp bootstrap/fixtures/typecheck/expression_comparison_bool.zp bootstrap/fixtures/typecheck/expression_boolean_logic.zp bootstrap/fixtures/typecheck/expression_boolean_logic_incompatible.zp bootstrap/fixtures/typecheck/expression_result_constructor.zp bootstrap/fixtures/typecheck/expression_result_constructor_incompatible.zp bootstrap/fixtures/typecheck/collection_expression_list.zp bootstrap/fixtures/typecheck/collection_expression_list_incompatible.zp bootstrap/fixtures/typecheck/collection_expression_map.zp bootstrap/fixtures/typecheck/collection_expression_map_incompatible.zp bootstrap/fixtures/typecheck/generic_identity.zp bootstrap/fixtures/typecheck/generic_return_mismatch.zp bootstrap/fixtures/typecheck/generic_multiple_params.zp bootstrap/fixtures/typecheck/generic_option_wrapper.zp bootstrap/fixtures/typecheck/generic_result_wrapper.zp bootstrap/fixtures/typecheck/generic_arity_mismatch.zp bootstrap/fixtures/typecheck/generic_runtime_wrappers.zp bootstrap/fixtures/typecheck/generic_empty_params.zp bootstrap/fixtures/typecheck/generic_duplicate_params.zp bootstrap/fixtures/typecheck/generic_invalid_param.zp bootstrap/fixtures/typecheck/generic_list_wrapper.zp bootstrap/fixtures/typecheck/generic_list_wrapper_incompatible.zp bootstrap/fixtures/typecheck/generic_map_wrapper.zp bootstrap/fixtures/typecheck/generic_map_wrapper_incompatible.zp bootstrap/fixtures/typecheck/generic_cross_module_library.zp bootstrap/fixtures/typecheck/generic_cross_module.zp bootstrap/fixtures/typecheck/generic_cross_module_incompatible.zp bootstrap/fixtures/typecheck/generic_constraint_colon.zp bootstrap/fixtures/typecheck/generic_constraint_extends.zp bootstrap/fixtures/typecheck/generic_constraint_where.zp bootstrap/fixtures/typecheck/generic_explicit_call_deferred.zp; do
  [[ -f "$path" ]] || { printf 'missing B2 fixture: %s\n' "$path" >&2; exit 2; }
done

first=$(mktemp "${TMPDIR:-/tmp}/zap-b2-typed-ir-first.XXXXXX")
second=$(mktemp "${TMPDIR:-/tmp}/zap-b2-typed-ir-second.XXXXXX")
root=$(mktemp -d "${TMPDIR:-/tmp}/zap-b2-typecheck-projects.XXXXXX")
trap 'rm -f "$first" "$second"; rm -rf "$root"' EXIT

cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- bootstrap typed-ir "$valid_ir_fixture" > "$first"
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- bootstrap typed-ir "$valid_ir_fixture" > "$second"
cmp "$first" "$second"
cmp "$first" "$valid_ir_expected"
jq -e '.kind == "zap.typed_ir" and .schema_version == 1 and .reference_only == true and .ir.nodes[0].annotation == "number" and .ir.nodes[0].inferred_type == "number"' "$first" >/dev/null
printf 'B2 typed-IR reference reproducibility passed: annotated declaration\n'

run_check() {
  local name=$1
  mkdir -p "$root/$name"
  printf '[package]\nname = "%s"\nversion = "0.1.0"\nmain = "main.zp"\n' "$name" > "$root/$name/zap.toml"
  cp "bootstrap/fixtures/typecheck/$name.zp" "$root/$name/main.zp"
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- check --json "$root/$name"
}

run_cross_module_check() {
  local name=$1
  mkdir -p "$root/$name"
  printf '[package]\nname = "%s"\nversion = "0.1.0"\nmain = "main.zp"\n' "$name" > "$root/$name/zap.toml"
  cp "bootstrap/fixtures/typecheck/$name.zp" "$root/$name/main.zp"
  cp bootstrap/fixtures/typecheck/generic_cross_module_library.zp "$root/$name/generic_cross_module_library.zp"
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- check --json "$root/$name"
}

annotated=$(run_check annotated)
jq -e '.ok == true' <<<"$annotated" >/dev/null
printf 'B2 type-check acceptance passed: annotated\n'

conditional=$(run_check conditional)
jq -e '.ok == true' <<<"$conditional" >/dev/null
printf 'B2 type-check acceptance passed: conditional expression\n'

set +e
incompatible=$(run_check incompatible 2>/tmp/zap-b2-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 1 and .column == 1 and (.message | contains("expects number, got text"))' <<<"$incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible annotation\n'

function_check=$(run_check function)
jq -e '.ok == true' <<<"$function_check" >/dev/null
printf 'B2 type-check acceptance passed: annotated function and call\n'

set +e
function_incompatible=$(run_check function_incompatible 2>/tmp/zap-b2-function-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'function_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 3 and .column == 22 and (.message | test("argument .* for .* expects number, got text"))' <<<"$function_incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible function call\n'

set +e
collection_incompatible=$(run_check collection_incompatible 2>/tmp/zap-b2-collection-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'collection_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 2 and .column == 1 and (.message | contains("variable '\''first'\'' expects text, got number"))' <<<"$collection_incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible collection element\n'

nested_collection_check=$(run_check nested_collection)
jq -e '.ok == true' <<<"$nested_collection_check" >/dev/null
printf 'B2 type-check acceptance passed: nested collection element\n'

set +e
nested_collection_incompatible=$(run_check nested_collection_incompatible 2>/tmp/zap-b2-nested-collection-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'nested_collection_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 2 and .column == 1 and (.message | contains("variable '\''first'\'' expects text, got number"))' <<<"$nested_collection_incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible nested collection element\n'

map_collection_check=$(run_check map_collection)
jq -e '.ok == true' <<<"$map_collection_check" >/dev/null
printf 'B2 type-check acceptance passed: bounded map element\n'

set +e
map_collection_incompatible=$(run_check map_collection_incompatible 2>/tmp/zap-b2-map-collection-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'map_collection_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 2 and .column == 1 and (.message | contains("variable '\''result'\'' expects text, got number"))' <<<"$map_collection_incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible bounded map element\n'

branch_narrowing_check=$(run_check branch_narrowing)
jq -e '.ok == true' <<<"$branch_narrowing_check" >/dev/null
printf 'B2 type-check acceptance passed: bounded branch-local option narrowing\n'

set +e
branch_narrowing_incompatible=$(run_check branch_narrowing_incompatible 2>/tmp/zap-b2-branch-narrowing-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'branch_narrowing_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 5 and .column == 1 and (.message | contains("variable '\''inside'\'' expects text, got number"))' <<<"$branch_narrowing_incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible bounded branch-local narrowing\n'

loop_narrowing_check=$(run_check loop_narrowing)
jq -e '.ok == true' <<<"$loop_narrowing_check" >/dev/null
printf 'B2 type-check acceptance passed: bounded loop-body narrowing\n'

set +e
loop_narrowing_incompatible=$(run_check loop_narrowing_incompatible 2>/tmp/zap-b2-loop-narrowing-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'loop_narrowing_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 4 and .column == 1 and (.message | contains("variable '\''after_loop'\'' expects number, got option<number>"))' <<<"$loop_narrowing_incompatible" >/dev/null
printf 'B2 type-check rejection passed: loop-boundary wrapper restoration\n'

else_narrowing_check=$(run_check else_narrowing)
jq -e '.ok == true' <<<"$else_narrowing_check" >/dev/null
printf 'B2 type-check acceptance passed: bounded is_option_none else-branch narrowing\n'

set +e
else_narrowing_incompatible=$(run_check else_narrowing_incompatible 2>/tmp/zap-b2-else-narrowing-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'else_narrowing_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 5 and .column == 1 and (.message | contains("variable '\''payload'\'' expects text, got number"))' <<<"$else_narrowing_incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible bounded is_option_none else-branch narrowing\n'

bool_annotation_check=$(run_check bool_annotation)
jq -e '.ok == true' <<<"$bool_annotation_check" >/dev/null
printf 'B2 type-check acceptance passed: bool literal annotation\n'

set +e
bool_annotation_incompatible=$(run_check bool_annotation_incompatible 2>/tmp/zap-b2-bool-annotation-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'bool_annotation_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 1 and .column == 1 and (.message | contains("variable '\''enabled'\'' expects bool, got number"))' <<<"$bool_annotation_incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible bool annotation\n'

none_annotation_check=$(run_check none_annotation)
jq -e '.ok == true' <<<"$none_annotation_check" >/dev/null
printf 'B2 type-check acceptance passed: none literal annotation\n'

set +e
none_annotation_incompatible=$(run_check none_annotation_incompatible 2>/tmp/zap-b2-none-annotation-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'none_annotation_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 1 and .column == 1 and (.message | contains("variable '\''missing'\'' expects none, got bool"))' <<<"$none_annotation_incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible none annotation\n'

list_annotation_check=$(run_check list_annotation)
jq -e '.ok == true' <<<"$list_annotation_check" >/dev/null
printf 'B2 type-check acceptance passed: direct list literal annotation\n'

set +e
list_annotation_incompatible=$(run_check list_annotation_incompatible 2>/tmp/zap-b2-list-annotation-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'list_annotation_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 1 and .column == 1 and (.message | contains("variable '\''wrong'\'' expects text, got list<number>"))' <<<"$list_annotation_incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible direct list annotation\n'

map_annotation_check=$(run_check map_annotation)
jq -e '.ok == true' <<<"$map_annotation_check" >/dev/null
printf 'B2 type-check acceptance passed: direct map literal annotation\n'

set +e
map_annotation_incompatible=$(run_check map_annotation_incompatible 2>/tmp/zap-b2-map-annotation-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'map_annotation_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 1 and .column == 1 and (.message | contains("variable '\''wrong'\'' expects text, got map<text,number>"))' <<<"$map_annotation_incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible direct map annotation\n'

option_annotation_check=$(run_check option_annotation)
jq -e '.ok == true' <<<"$option_annotation_check" >/dev/null
printf 'B2 type-check acceptance passed: direct option constructor annotation\n'

set +e
option_annotation_incompatible=$(run_check option_annotation_incompatible 2>/tmp/zap-b2-option-annotation-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'option_annotation_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 1 and .column == 1 and (.message | contains("variable '\''wrong'\'' expects text, got option<number>"))' <<<"$option_annotation_incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible direct option annotation\n'

expression_number_add_check=$(run_check expression_number_add)
jq -e '.ok == true' <<<"$expression_number_add_check" >/dev/null
printf 'B2 type-check acceptance passed: exact numeric addition expression\n'

set +e
expression_number_add_incompatible=$(run_check expression_number_add_incompatible 2>/tmp/zap-b2-expression-number-add-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'expression_number_add_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 1 and .column == 1 and (.message | contains("variable '\''wrong'\'' expects text, got number"))' <<<"$expression_number_add_incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible exact numeric addition expression\n'

expression_text_add_check=$(run_check expression_text_add)
jq -e '.ok == true' <<<"$expression_text_add_check" >/dev/null
printf 'B2 type-check acceptance passed: exact text addition expression\n'

set +e
expression_text_add_incompatible=$(run_check expression_text_add_incompatible 2>/tmp/zap-b2-expression-text-add-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'expression_text_add_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 1 and .column == 1 and (.message | contains("variable '\''wrong'\'' expects number, got text"))' <<<"$expression_text_add_incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible exact text addition expression\n'

expression_comparison_bool_check=$(run_check expression_comparison_bool)
jq -e '.ok == true' <<<"$expression_comparison_bool_check" >/dev/null
printf 'B2 type-check acceptance passed: exact comparison bool expression\n'

expression_boolean_logic_check=$(run_check expression_boolean_logic)
jq -e '.ok == true' <<<"$expression_boolean_logic_check" >/dev/null
printf 'B2 type-check acceptance passed: exact boolean logic expression\n'

set +e
expression_boolean_logic_incompatible=$(run_check expression_boolean_logic_incompatible 2>/tmp/zap-b2-expression-boolean-logic-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'expression_boolean_logic_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 1 and .column == 1 and (.message | contains("variable '\''wrong'\'' expects text, got bool"))' <<<"$expression_boolean_logic_incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible exact boolean logic expression\n'

expression_result_constructor_check=$(run_check expression_result_constructor)
jq -e '.ok == true' <<<"$expression_result_constructor_check" >/dev/null
printf 'B2 type-check acceptance passed: exact result constructor expression\n'

set +e
expression_result_constructor_incompatible=$(run_check expression_result_constructor_incompatible 2>/tmp/zap-b2-expression-result-constructor-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'expression_result_constructor_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 1 and .column == 1 and (.message | contains("variable '\''wrong'\'' expects text, got result<number>"))' <<<"$expression_result_constructor_incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible exact result constructor expression\n'

collection_expression_list_check=$(run_check collection_expression_list)
jq -e '.ok == true' <<<"$collection_expression_list_check" >/dev/null
printf 'B2 type-check acceptance passed: exact list arithmetic expression\n'

set +e
collection_expression_list_incompatible=$(run_check collection_expression_list_incompatible 2>/tmp/zap-b2-collection-expression-list-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'collection_expression_list_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 1 and .column == 1 and (.message | contains("variable '\''wrong'\'' expects list<text>, got list<number>"))' <<<"$collection_expression_list_incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible exact list arithmetic expression\n'

collection_expression_map_check=$(run_check collection_expression_map)
jq -e '.ok == true' <<<"$collection_expression_map_check" >/dev/null
printf 'B2 type-check acceptance passed: exact map arithmetic expression\n'

set +e
collection_expression_map_incompatible=$(run_check collection_expression_map_incompatible 2>/tmp/zap-b2-collection-expression-map-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'collection_expression_map_incompatible fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 1 and .column == 1 and (.message | contains("variable '\''wrong'\'' expects map<text,text>, got map<text,number>"))' <<<"$collection_expression_map_incompatible" >/dev/null
printf 'B2 type-check rejection passed: incompatible exact map arithmetic expression\n'
generic_identity_check=$(run_check generic_identity)
jq -e '.ok == true and .code? == null' <<<"$generic_identity_check" >/dev/null
printf 'A3 type-check acceptance passed: generic identity substitution\n'
set +e
generic_conflict=$(run_check generic_conflict 2>/tmp/zap-a3-generic-conflict-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'generic_conflict fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 4 and .column == 19 and (.message | contains("generic argument substitution for '\''same'\'' is inconsistent"))' <<<"$generic_conflict" >/dev/null
printf 'A3 type-check rejection passed: conflicting generic substitution\n'
set +e
generic_return_mismatch=$(run_check generic_return_mismatch 2>/tmp/zap-a3-generic-return-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'generic_return_mismatch fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 2 and .column == 1 and (.message | contains("return from '\''broken'\'' expects T, got text"))' <<<"$generic_return_mismatch" >/dev/null
printf 'A3 type-check rejection passed: generic return mismatch\n'
generic_multiple_params_check=$(run_check generic_multiple_params)
jq -e '.ok == true and .code? == null' <<<"$generic_multiple_params_check" >/dev/null
printf 'A3 type-check acceptance passed: multiple generic substitutions\n'
set +e
generic_option_wrapper=$(run_check generic_option_wrapper 2>/tmp/zap-a3-generic-option-typecheck-error)
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  printf 'generic_option_wrapper fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 5 and .column == 1 and (.message | contains("variable '\''wrong'\'' expects text, got option<number>"))' <<<"$generic_option_wrapper" >/dev/null
printf 'A3 type-check rejection passed: generic option-wrapper substitution mismatch\n'
set +e
generic_result_wrapper=$(run_check generic_result_wrapper 2>/tmp/zap-a3-generic-result-typecheck-error)
result_wrapper_status=$?
set -e
if [[ "$result_wrapper_status" -eq 0 ]]; then
  printf 'generic_result_wrapper fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 5 and .column == 1 and (.message | contains("variable '\''wrong'\'' expects number, got result<text>"))' <<<"$generic_result_wrapper" >/dev/null
printf 'A3 type-check rejection passed: generic result-wrapper substitution mismatch\n'
set +e
generic_arity_mismatch=$(run_check generic_arity_mismatch 2>/tmp/zap-a3-generic-arity-typecheck-error)
arity_status=$?
set -e
if [[ "$arity_status" -eq 0 ]]; then
  printf 'generic_arity_mismatch fixture unexpectedly passed\n' >&2
  exit 1
fi
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 4 and .column == 21 and (.message | contains("function '\''first'\'' expects 2 to 2 arguments, got 1"))' <<<"$generic_arity_mismatch" >/dev/null
printf 'A3 type-check rejection passed: generic function arity\n'
generic_runtime_wrappers=$(run_check generic_runtime_wrappers)
jq -e '.ok == true and .code? == null' <<<"$generic_runtime_wrappers" >/dev/null
printf 'A3 type-check acceptance passed: generic runtime wrapper corpus\n'
runtime_output=$(cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- run "$root/generic_runtime_wrappers/main.zp" 2>/tmp/zap-a3-generic-runtime-error)
[[ -z "$runtime_output" ]] || { printf 'generic runtime fixture unexpectedly emitted output: %s\n' "$runtime_output" >&2; exit 1; }
printf 'A3 runtime substitution passed: generic option/result wrapper corpus\n'
generic_list_wrapper=$(run_check generic_list_wrapper)
jq -e '.ok == true and .code? == null' <<<"$generic_list_wrapper" >/dev/null
printf 'A3 type-check acceptance passed: generic list wrapper\n'
list_runtime_output=$(cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- run "$root/generic_list_wrapper/main.zp" 2>/tmp/zap-a3-generic-list-runtime-error)
[[ -z "$list_runtime_output" ]] || { printf 'generic list runtime fixture unexpectedly emitted output: %s\n' "$list_runtime_output" >&2; exit 1; }
printf 'A3 runtime substitution passed: generic list wrapper\n'
set +e
generic_list_wrapper_incompatible=$(run_check generic_list_wrapper_incompatible 2>/tmp/zap-a3-generic-list-incompatible-error)
list_incompatible_status=$?
set -e
[[ "$list_incompatible_status" -ne 0 ]] || { printf 'generic_list_wrapper_incompatible fixture unexpectedly passed\n' >&2; exit 1; }
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 4 and .column == 1 and (.message | contains("variable '\''wrong'\'' expects text, got list<number>"))' <<<"$generic_list_wrapper_incompatible" >/dev/null
printf 'A3 type-check rejection passed: generic list-wrapper substitution mismatch\n'
generic_map_wrapper=$(run_check generic_map_wrapper)
jq -e '.ok == true and .code? == null' <<<"$generic_map_wrapper" >/dev/null
printf 'A3 type-check acceptance passed: generic map wrapper\n'
map_runtime_output=$(cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- run "$root/generic_map_wrapper/main.zp" 2>/tmp/zap-a3-generic-map-runtime-error)
[[ -z "$map_runtime_output" ]] || { printf 'generic map runtime fixture unexpectedly emitted output: %s\n' "$map_runtime_output" >&2; exit 1; }
printf 'A3 runtime substitution passed: generic map wrapper\n'
set +e
generic_map_wrapper_incompatible=$(run_check generic_map_wrapper_incompatible 2>/tmp/zap-a3-generic-map-incompatible-error)
map_incompatible_status=$?
set -e
[[ "$map_incompatible_status" -ne 0 ]] || { printf 'generic_map_wrapper_incompatible fixture unexpectedly passed\n' >&2; exit 1; }
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 4 and .column == 1 and (.message | contains("variable '\''wrong'\'' expects text, got map<text,number>"))' <<<"$generic_map_wrapper_incompatible" >/dev/null
printf 'A3 type-check rejection passed: generic map-wrapper substitution mismatch\n'
generic_cross_module=$(run_cross_module_check generic_cross_module)
jq -e '.ok == true and .code? == null' <<<"$generic_cross_module" >/dev/null
printf 'A3 type-check acceptance passed: imported generic identity substitution\n'
cross_module_runtime_output=$(cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- run "$root/generic_cross_module/main.zp" 2>/tmp/zap-a3-cross-module-runtime-error)
[[ -z "$cross_module_runtime_output" ]] || { printf 'generic cross-module runtime fixture unexpectedly emitted output: %s\n' "$cross_module_runtime_output" >&2; exit 1; }
printf 'A3 runtime substitution passed: imported generic identity\n'
set +e
generic_cross_module_incompatible=$(run_cross_module_check generic_cross_module_incompatible 2>/tmp/zap-a3-cross-module-incompatible-error)
cross_module_incompatible_status=$?
set -e
[[ "$cross_module_incompatible_status" -ne 0 ]] || { printf 'generic_cross_module_incompatible fixture unexpectedly passed\n' >&2; exit 1; }
jq -e '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 3 and .column == 1 and (.message | contains("variable '\''wrong'\'' expects text, got number"))' <<<"$generic_cross_module_incompatible" >/dev/null
printf 'A3 type-check rejection passed: imported generic substitution mismatch\n'
generic_explicit_call_deferred=$(run_check generic_explicit_call_deferred)
jq -e '.ok == true and .code? == null' <<<"$generic_explicit_call_deferred" >/dev/null
printf 'A3 deferred syntax probe passed: explicit generic call statically accepted by current reference path\n'
set +e
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- run "$root/generic_explicit_call_deferred/main.zp" >/tmp/zap-a3-explicit-call-runtime-output 2>/tmp/zap-a3-explicit-call-runtime-error
explicit_call_runtime_status=$?
set -e
[[ "$explicit_call_runtime_status" -ne 0 ]] || { printf 'explicit generic call deferred fixture unexpectedly ran successfully\n' >&2; exit 1; }
grep -F "undefined variable: number" /tmp/zap-a3-explicit-call-runtime-error >/dev/null || { printf 'explicit generic call runtime diagnostic changed unexpectedly\n' >&2; cat /tmp/zap-a3-explicit-call-runtime-error >&2; exit 1; }
printf 'A3 deferred syntax probe passed: explicit generic call runtime rejection preserved\n'
for malformed in empty_params duplicate_params invalid_param; do
  set +e
  malformed_output=$(run_check "generic_${malformed}" 2>/tmp/zap-a3-malformed-generic-error)
  malformed_status=$?
  set -e
  [[ "$malformed_status" -ne 0 ]] || { printf 'generic_%s fixture unexpectedly passed\n' "$malformed" >&2; exit 1; }
  jq -e '.ok == false and .code == "ZAP-SYNTAX-001" and .kind == "SyntaxError" and .severity == "error" and .line == 1 and .column == 1' <<<"$malformed_output" >/dev/null
  printf 'A3 parser rejection passed: generic_%s\n' "$malformed"
done
jq -e '(.message == "generic type-parameter list cannot be empty")' <<<"$(run_check generic_empty_params 2>/dev/null || true)" >/dev/null
jq -e '(.message == "duplicate generic type parameter: T")' <<<"$(run_check generic_duplicate_params 2>/dev/null || true)" >/dev/null
jq -e '(.message == "invalid generic type parameter '\''t'\''")' <<<"$(run_check generic_invalid_param 2>/dev/null || true)" >/dev/null
