#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
for path in bootstrap/b2/typecheck.zp bootstrap/fixtures/typecheck/annotated.zp bootstrap/fixtures/typecheck/conditional.zp bootstrap/fixtures/typecheck/incompatible.zp bootstrap/fixtures/typecheck/function.zp bootstrap/fixtures/typecheck/function_incompatible.zp bootstrap/fixtures/typecheck/collection_incompatible.zp bootstrap/fixtures/typecheck/nested_collection.zp bootstrap/fixtures/typecheck/nested_collection_incompatible.zp bootstrap/fixtures/typecheck/map_collection.zp bootstrap/fixtures/typecheck/map_collection_incompatible.zp bootstrap/fixtures/typecheck/branch_narrowing.zp bootstrap/fixtures/typecheck/branch_narrowing_incompatible.zp bootstrap/fixtures/typecheck/loop_narrowing.zp bootstrap/fixtures/typecheck/loop_narrowing_incompatible.zp bootstrap/fixtures/typecheck/else_narrowing.zp bootstrap/fixtures/typecheck/else_narrowing_incompatible.zp bootstrap/fixtures/typecheck/bool_annotation.zp bootstrap/fixtures/typecheck/bool_annotation_incompatible.zp bootstrap/fixtures/typecheck/none_annotation.zp bootstrap/fixtures/typecheck/none_annotation_incompatible.zp bootstrap/fixtures/typecheck/list_annotation.zp bootstrap/fixtures/typecheck/list_annotation_incompatible.zp bootstrap/fixtures/typecheck/map_annotation.zp bootstrap/fixtures/typecheck/map_annotation_incompatible.zp; do
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
EOF_RUNNER
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$first"
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$second"
cmp "$first" "$second"
[[ "$(wc -l < "$first")" -eq 24 ]] || { printf 'unexpected B2 candidate output line count\n' >&2; exit 1; }
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
printf 'B2 Zap type-checker candidate differential semantics passed: annotated, conditional, incompatible, function, function-call mismatch, list-element mismatch, nested-list element, nested-list mismatch, bounded map element, bounded map mismatch, branch-local narrowing, branch narrowing mismatch, loop-body narrowing, loop-boundary restoration mismatch, is_option_none else-body narrowing, else-body narrowing mismatch, bool literal annotation, bool annotation mismatch, none literal annotation, none annotation mismatch, direct list literal annotation, list annotation mismatch, direct map literal annotation, map annotation mismatch\n'
