#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
for path in bootstrap/b2/typecheck.zp bootstrap/fixtures/typecheck/annotated.zp bootstrap/fixtures/typecheck/conditional.zp bootstrap/fixtures/typecheck/incompatible.zp bootstrap/fixtures/typecheck/function.zp bootstrap/fixtures/typecheck/function_incompatible.zp bootstrap/fixtures/typecheck/collection_incompatible.zp bootstrap/fixtures/typecheck/nested_collection.zp bootstrap/fixtures/typecheck/nested_collection_incompatible.zp bootstrap/fixtures/typecheck/map_collection.zp bootstrap/fixtures/typecheck/map_collection_incompatible.zp bootstrap/fixtures/typecheck/branch_narrowing.zp bootstrap/fixtures/typecheck/branch_narrowing_incompatible.zp bootstrap/fixtures/typecheck/loop_narrowing.zp bootstrap/fixtures/typecheck/loop_narrowing_incompatible.zp bootstrap/fixtures/typecheck/else_narrowing.zp bootstrap/fixtures/typecheck/else_narrowing_incompatible.zp bootstrap/fixtures/typecheck/bool_annotation.zp bootstrap/fixtures/typecheck/bool_annotation_incompatible.zp bootstrap/fixtures/typecheck/none_annotation.zp bootstrap/fixtures/typecheck/none_annotation_incompatible.zp bootstrap/fixtures/typecheck/list_annotation.zp bootstrap/fixtures/typecheck/list_annotation_incompatible.zp bootstrap/fixtures/typecheck/map_annotation.zp bootstrap/fixtures/typecheck/map_annotation_incompatible.zp bootstrap/fixtures/typecheck/option_annotation.zp bootstrap/fixtures/typecheck/option_annotation_incompatible.zp bootstrap/fixtures/typecheck/expression_number_add.zp bootstrap/fixtures/typecheck/expression_number_add_incompatible.zp bootstrap/fixtures/typecheck/expression_text_add.zp bootstrap/fixtures/typecheck/expression_text_add_incompatible.zp bootstrap/fixtures/typecheck/expression_comparison_bool.zp bootstrap/fixtures/typecheck/expression_boolean_logic.zp bootstrap/fixtures/typecheck/expression_boolean_logic_incompatible.zp bootstrap/fixtures/typecheck/expression_result_constructor.zp bootstrap/fixtures/typecheck/expression_result_constructor_incompatible.zp bootstrap/fixtures/typecheck/collection_expression_list.zp bootstrap/fixtures/typecheck/collection_expression_list_incompatible.zp bootstrap/fixtures/typecheck/collection_expression_map.zp bootstrap/fixtures/typecheck/collection_expression_map_incompatible.zp bootstrap/fixtures/typecheck/generic_identity.zp bootstrap/fixtures/typecheck/generic_conflict.zp bootstrap/fixtures/typecheck/generic_return_mismatch.zp bootstrap/fixtures/typecheck/generic_multiple_params.zp bootstrap/fixtures/typecheck/generic_option_wrapper.zp bootstrap/fixtures/typecheck/generic_result_wrapper.zp bootstrap/fixtures/typecheck/generic_arity_mismatch.zp bootstrap/fixtures/typecheck/generic_runtime_wrappers.zp bootstrap/fixtures/typecheck/generic_empty_params.zp bootstrap/fixtures/typecheck/generic_duplicate_params.zp bootstrap/fixtures/typecheck/generic_invalid_param.zp bootstrap/fixtures/typecheck/generic_list_wrapper.zp bootstrap/fixtures/typecheck/generic_list_wrapper_incompatible.zp bootstrap/fixtures/typecheck/generic_map_wrapper.zp bootstrap/fixtures/typecheck/generic_map_wrapper_incompatible.zp; do
  [[ -f "$path" ]] || { printf 'missing B2 candidate fixture: %s\n' "$path" >&2; exit 2; }
done
runner=$(mktemp "$ROOT_DIR/.zap-b2-typecheck-candidate-runner.XXXXXX.zp")
first=$(mktemp "${TMPDIR:-/tmp}/zap-b2-typecheck-candidate-first.XXXXXX")
second=$(mktemp "${TMPDIR:-/tmp}/zap-b2-typecheck-candidate-second.XXXXXX")
trap 'rm -f "$runner" "$first" "$second"' EXIT
cat > "$runner" <<'EOF_RUNNER'
import "bootstrap/b2/typecheck.zp"
let annotated = read_text("bootstrap/fixtures/typecheck/annotated.zp")
let conditional = read_text("bootstrap/fixtures/typecheck/conditional.zp")
let incompatible = read_text("bootstrap/fixtures/typecheck/incompatible.zp")
let function_source = read_text("bootstrap/fixtures/typecheck/function.zp")
let function_incompatible = read_text("bootstrap/fixtures/typecheck/function_incompatible.zp")
let collection_incompatible = read_text("bootstrap/fixtures/typecheck/collection_incompatible.zp")
let nested_collection = read_text("bootstrap/fixtures/typecheck/nested_collection.zp")
let nested_collection_incompatible = read_text("bootstrap/fixtures/typecheck/nested_collection_incompatible.zp")
let map_collection = read_text("bootstrap/fixtures/typecheck/map_collection.zp")
let map_collection_incompatible = read_text("bootstrap/fixtures/typecheck/map_collection_incompatible.zp")
let branch_narrowing = read_text("bootstrap/fixtures/typecheck/branch_narrowing.zp")
let branch_narrowing_incompatible = read_text("bootstrap/fixtures/typecheck/branch_narrowing_incompatible.zp")
let loop_narrowing = read_text("bootstrap/fixtures/typecheck/loop_narrowing.zp")
let loop_narrowing_incompatible = read_text("bootstrap/fixtures/typecheck/loop_narrowing_incompatible.zp")
let else_narrowing = read_text("bootstrap/fixtures/typecheck/else_narrowing.zp")
let else_narrowing_incompatible = read_text("bootstrap/fixtures/typecheck/else_narrowing_incompatible.zp")
let bool_annotation = read_text("bootstrap/fixtures/typecheck/bool_annotation.zp")
let bool_annotation_incompatible = read_text("bootstrap/fixtures/typecheck/bool_annotation_incompatible.zp")
let none_annotation = read_text("bootstrap/fixtures/typecheck/none_annotation.zp")
let none_annotation_incompatible = read_text("bootstrap/fixtures/typecheck/none_annotation_incompatible.zp")
let list_annotation = read_text("bootstrap/fixtures/typecheck/list_annotation.zp")
let list_annotation_incompatible = read_text("bootstrap/fixtures/typecheck/list_annotation_incompatible.zp")
let map_annotation = read_text("bootstrap/fixtures/typecheck/map_annotation.zp")
let map_annotation_incompatible = read_text("bootstrap/fixtures/typecheck/map_annotation_incompatible.zp")
let option_annotation = read_text("bootstrap/fixtures/typecheck/option_annotation.zp")
let option_annotation_incompatible = read_text("bootstrap/fixtures/typecheck/option_annotation_incompatible.zp")
let expression_number_add = read_text("bootstrap/fixtures/typecheck/expression_number_add.zp")
let expression_number_add_incompatible = read_text("bootstrap/fixtures/typecheck/expression_number_add_incompatible.zp")
let expression_text_add = read_text("bootstrap/fixtures/typecheck/expression_text_add.zp")
let expression_text_add_incompatible = read_text("bootstrap/fixtures/typecheck/expression_text_add_incompatible.zp")
let expression_comparison_bool = read_text("bootstrap/fixtures/typecheck/expression_comparison_bool.zp")
let expression_boolean_logic = read_text("bootstrap/fixtures/typecheck/expression_boolean_logic.zp")
let expression_boolean_logic_incompatible = read_text("bootstrap/fixtures/typecheck/expression_boolean_logic_incompatible.zp")
let expression_result_constructor = read_text("bootstrap/fixtures/typecheck/expression_result_constructor.zp")
let expression_result_constructor_incompatible = read_text("bootstrap/fixtures/typecheck/expression_result_constructor_incompatible.zp")
let collection_expression_list = read_text("bootstrap/fixtures/typecheck/collection_expression_list.zp")
let collection_expression_list_incompatible = read_text("bootstrap/fixtures/typecheck/collection_expression_list_incompatible.zp")
let collection_expression_map = read_text("bootstrap/fixtures/typecheck/collection_expression_map.zp")
let collection_expression_map_incompatible = read_text("bootstrap/fixtures/typecheck/collection_expression_map_incompatible.zp")
let generic_identity = read_text("bootstrap/fixtures/typecheck/generic_identity.zp")
let generic_conflict = read_text("bootstrap/fixtures/typecheck/generic_conflict.zp")
let generic_return_mismatch = read_text("bootstrap/fixtures/typecheck/generic_return_mismatch.zp")
let generic_multiple_params = read_text("bootstrap/fixtures/typecheck/generic_multiple_params.zp")
let generic_option_wrapper = read_text("bootstrap/fixtures/typecheck/generic_option_wrapper.zp")
let generic_result_wrapper = read_text("bootstrap/fixtures/typecheck/generic_result_wrapper.zp")
let generic_arity_mismatch = read_text("bootstrap/fixtures/typecheck/generic_arity_mismatch.zp")
let generic_runtime_wrappers = read_text("bootstrap/fixtures/typecheck/generic_runtime_wrappers.zp")
let generic_empty_params = read_text("bootstrap/fixtures/typecheck/generic_empty_params.zp")
let generic_duplicate_params = read_text("bootstrap/fixtures/typecheck/generic_duplicate_params.zp")
let generic_invalid_param = read_text("bootstrap/fixtures/typecheck/generic_invalid_param.zp")
let generic_list_wrapper = read_text("bootstrap/fixtures/typecheck/generic_list_wrapper.zp")
let generic_list_wrapper_incompatible = read_text("bootstrap/fixtures/typecheck/generic_list_wrapper_incompatible.zp")
let generic_map_wrapper = read_text("bootstrap/fixtures/typecheck/generic_map_wrapper.zp")
let generic_map_wrapper_incompatible = read_text("bootstrap/fixtures/typecheck/generic_map_wrapper_incompatible.zp")
say check(annotated, "bootstrap/fixtures/typecheck/annotated.zp")
say check(conditional, "bootstrap/fixtures/typecheck/conditional.zp")
say check(incompatible, "bootstrap/fixtures/typecheck/incompatible.zp")
say check(function_source, "bootstrap/fixtures/typecheck/function.zp")
say check(function_incompatible, "bootstrap/fixtures/typecheck/function_incompatible.zp")
say check(collection_incompatible, "bootstrap/fixtures/typecheck/collection_incompatible.zp")
say check(nested_collection, "bootstrap/fixtures/typecheck/nested_collection.zp")
say check(nested_collection_incompatible, "bootstrap/fixtures/typecheck/nested_collection_incompatible.zp")
say check(map_collection, "bootstrap/fixtures/typecheck/map_collection.zp")
say check(map_collection_incompatible, "bootstrap/fixtures/typecheck/map_collection_incompatible.zp")
say check(branch_narrowing, "bootstrap/fixtures/typecheck/branch_narrowing.zp")
say check(branch_narrowing_incompatible, "bootstrap/fixtures/typecheck/branch_narrowing_incompatible.zp")
say check(loop_narrowing, "bootstrap/fixtures/typecheck/loop_narrowing.zp")
say check(loop_narrowing_incompatible, "bootstrap/fixtures/typecheck/loop_narrowing_incompatible.zp")
say check(else_narrowing, "bootstrap/fixtures/typecheck/else_narrowing.zp")
say check(else_narrowing_incompatible, "bootstrap/fixtures/typecheck/else_narrowing_incompatible.zp")
say check(bool_annotation, "bootstrap/fixtures/typecheck/bool_annotation.zp")
say check(bool_annotation_incompatible, "bootstrap/fixtures/typecheck/bool_annotation_incompatible.zp")
say check(none_annotation, "bootstrap/fixtures/typecheck/none_annotation.zp")
say check(none_annotation_incompatible, "bootstrap/fixtures/typecheck/none_annotation_incompatible.zp")
say check(list_annotation, "bootstrap/fixtures/typecheck/list_annotation.zp")
say check(list_annotation_incompatible, "bootstrap/fixtures/typecheck/list_annotation_incompatible.zp")
say check(map_annotation, "bootstrap/fixtures/typecheck/map_annotation.zp")
say check(map_annotation_incompatible, "bootstrap/fixtures/typecheck/map_annotation_incompatible.zp")
say check(option_annotation, "bootstrap/fixtures/typecheck/option_annotation.zp")
say check(option_annotation_incompatible, "bootstrap/fixtures/typecheck/option_annotation_incompatible.zp")
say check(expression_number_add, "bootstrap/fixtures/typecheck/expression_number_add.zp")
say check(expression_number_add_incompatible, "bootstrap/fixtures/typecheck/expression_number_add_incompatible.zp")
say check(expression_text_add, "bootstrap/fixtures/typecheck/expression_text_add.zp")
say check(expression_text_add_incompatible, "bootstrap/fixtures/typecheck/expression_text_add_incompatible.zp")
say check(expression_comparison_bool, "bootstrap/fixtures/typecheck/expression_comparison_bool.zp")
say check(expression_boolean_logic, "bootstrap/fixtures/typecheck/expression_boolean_logic.zp")
say check(expression_boolean_logic_incompatible, "bootstrap/fixtures/typecheck/expression_boolean_logic_incompatible.zp")
say check(expression_result_constructor, "bootstrap/fixtures/typecheck/expression_result_constructor.zp")
say check(expression_result_constructor_incompatible, "bootstrap/fixtures/typecheck/expression_result_constructor_incompatible.zp")
say check(collection_expression_list, "bootstrap/fixtures/typecheck/collection_expression_list.zp")
say check(collection_expression_list_incompatible, "bootstrap/fixtures/typecheck/collection_expression_list_incompatible.zp")
say check(collection_expression_map, "bootstrap/fixtures/typecheck/collection_expression_map.zp")
say check(collection_expression_map_incompatible, "bootstrap/fixtures/typecheck/collection_expression_map_incompatible.zp")
say check(generic_identity, "bootstrap/fixtures/typecheck/generic_identity.zp")
say check(generic_conflict, "bootstrap/fixtures/typecheck/generic_conflict.zp")
say check(generic_return_mismatch, "bootstrap/fixtures/typecheck/generic_return_mismatch.zp")
say check(generic_multiple_params, "bootstrap/fixtures/typecheck/generic_multiple_params.zp")
say check(generic_option_wrapper, "bootstrap/fixtures/typecheck/generic_option_wrapper.zp")
say check(generic_result_wrapper, "bootstrap/fixtures/typecheck/generic_result_wrapper.zp")
say check(generic_arity_mismatch, "bootstrap/fixtures/typecheck/generic_arity_mismatch.zp")
say check(generic_runtime_wrappers, "bootstrap/fixtures/typecheck/generic_runtime_wrappers.zp")
say check(generic_empty_params, "bootstrap/fixtures/typecheck/generic_empty_params.zp")
say check(generic_duplicate_params, "bootstrap/fixtures/typecheck/generic_duplicate_params.zp")
say check(generic_invalid_param, "bootstrap/fixtures/typecheck/generic_invalid_param.zp")
say check(generic_list_wrapper, "bootstrap/fixtures/typecheck/generic_list_wrapper.zp")
say check(generic_list_wrapper_incompatible, "bootstrap/fixtures/typecheck/generic_list_wrapper_incompatible.zp")
say check(generic_map_wrapper, "bootstrap/fixtures/typecheck/generic_map_wrapper.zp")
say check(generic_map_wrapper_incompatible, "bootstrap/fixtures/typecheck/generic_map_wrapper_incompatible.zp")
EOF_RUNNER
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$first"
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$second"
cmp "$first" "$second"
[[ "$(wc -l < "$first")" -eq 54 ]] || { printf 'unexpected B2 candidate output line count\n' >&2; exit 1; }
sed -n '1p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '2p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '3p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 1) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("expects number, got text")))' >/dev/null
sed -n '4p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '5p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 3) and (.diagnostics[0].column == 22) and ((.diagnostics[0].message | test("argument .* for .* expects number, got text")))' >/dev/null
sed -n '6p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 2) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''first'\'' expects text, got number")))' >/dev/null
sed -n '7p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '8p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 2) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''first'\'' expects text, got number")))' >/dev/null
sed -n '9p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '10p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 2) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''result'\'' expects text, got number")))' >/dev/null
sed -n '11p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '12p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 5) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''inside'\'' expects text, got number")))' >/dev/null
sed -n '13p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '14p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 4) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''after_loop'\'' expects number, got option<number>")))' >/dev/null
sed -n '15p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '16p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 5) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''payload'\'' expects text, got number")))' >/dev/null
sed -n '17p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '18p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 1) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''enabled'\'' expects bool, got number")))' >/dev/null
sed -n '19p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '20p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 1) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''missing'\'' expects none, got bool")))' >/dev/null
sed -n '21p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '22p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 1) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''wrong'\'' expects text, got list<number>")))' >/dev/null
sed -n '23p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '24p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 1) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''wrong'\'' expects text, got map<text,number>")))' >/dev/null
sed -n '25p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '26p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 1) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''wrong'\'' expects text, got option<number>")))' >/dev/null
sed -n '27p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '28p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 1) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''wrong'\'' expects text, got number")))' >/dev/null
sed -n '29p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '30p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 1) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''wrong'\'' expects number, got text")))' >/dev/null
sed -n '31p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '32p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '33p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 1) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''wrong'\'' expects text, got bool")))' >/dev/null
sed -n '34p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '35p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 1) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''wrong'\'' expects text, got result<number>")))' >/dev/null
sed -n '36p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '37p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 1) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''wrong'\'' expects list<text>, got list<number>")))' >/dev/null
sed -n '38p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '39p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 1) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''wrong'\'' expects map<text,text>, got map<text,number>")))' >/dev/null
sed -n '40p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '41p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 4) and ((.diagnostics[0].message | contains("generic argument substitution for '\''same'\'' is inconsistent")))' >/dev/null
sed -n '42p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 2) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("return from '\''broken'\'' expects T, got text")))' >/dev/null
sed -n '43p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '44p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 5) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''wrong'\'' expects text, got option<number>")))' >/dev/null
sed -n '45p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 5) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''wrong'\'' expects number, got result<text>")))' >/dev/null
sed -n '46p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 4) and (.diagnostics[0].column == 21) and ((.diagnostics[0].message | contains("function '\''first'\'' expects 2 to 2 arguments, got 1")))' >/dev/null
sed -n '47p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '48p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-SYNTAX-001") and (.diagnostics[0].kind == "SyntaxError") and (.diagnostics[0].line == 1) and (.diagnostics[0].column == 1) and (.diagnostics[0].message == "generic type-parameter list cannot be empty")' >/dev/null
sed -n '49p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-SYNTAX-001") and (.diagnostics[0].kind == "SyntaxError") and (.diagnostics[0].line == 1) and (.diagnostics[0].column == 1) and (.diagnostics[0].message == "duplicate generic type parameter: T")' >/dev/null
sed -n '50p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-SYNTAX-001") and (.diagnostics[0].kind == "SyntaxError") and (.diagnostics[0].line == 1) and (.diagnostics[0].column == 1) and (.diagnostics[0].message == "invalid generic type parameter '\''t'\''")' >/dev/null
sed -n '51p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '52p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 4) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''wrong'\'' expects text, got list<number>")))' >/dev/null
sed -n '53p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '54p' "$first" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].line == 4) and (.diagnostics[0].column == 1) and ((.diagnostics[0].message | contains("variable '\''wrong'\'' expects text, got map<text,number>")))' >/dev/null
printf '%s\n' 'B2 Zap type-checker candidate differential semantics passed: exact collection, bounded generic-declaration, malformed-header, list-wrapper, and map-wrapper matrix included'
