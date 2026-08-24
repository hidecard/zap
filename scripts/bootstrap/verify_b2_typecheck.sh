#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"

valid_ir_fixture="bootstrap/fixtures/typecheck/annotated.zp"
valid_ir_expected="bootstrap/fixtures/typecheck/annotated.typed-ir.json"
for path in "$valid_ir_fixture" "$valid_ir_expected" bootstrap/fixtures/typecheck/incompatible.zp bootstrap/fixtures/typecheck/conditional.zp bootstrap/fixtures/typecheck/function.zp bootstrap/fixtures/typecheck/function_incompatible.zp bootstrap/fixtures/typecheck/collection_incompatible.zp bootstrap/fixtures/typecheck/nested_collection.zp bootstrap/fixtures/typecheck/nested_collection_incompatible.zp bootstrap/fixtures/typecheck/map_collection.zp bootstrap/fixtures/typecheck/map_collection_incompatible.zp bootstrap/fixtures/typecheck/branch_narrowing.zp bootstrap/fixtures/typecheck/branch_narrowing_incompatible.zp bootstrap/fixtures/typecheck/loop_narrowing.zp bootstrap/fixtures/typecheck/loop_narrowing_incompatible.zp; do
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
