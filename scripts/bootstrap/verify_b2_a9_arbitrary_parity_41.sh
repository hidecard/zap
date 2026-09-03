#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true

positive=(
  bootstrap/fixtures/typecheck/expression_number_add.zp:1:1
  bootstrap/fixtures/typecheck/conditional.zp:1:1
  bootstrap/fixtures/typecheck/list_annotation.zp:1:1
  bootstrap/fixtures/typecheck/map_annotation.zp:1:1
  bootstrap/fixtures/typecheck/two_declarations.zp:2:0
)
negative=(
  bootstrap/fixtures/diagnostics/invalid_character.zp
  bootstrap/fixtures/diagnostics/missing_closing_bracket.zp
  bootstrap/fixtures/diagnostics/unterminated_string.zp
  bootstrap/fixtures/diagnostics/integer_overflow.zp
)

run_case() {
  local fixture=$1
  local reference_mode=$2
  local expected_nodes=$3
  local expected_semantic=${4:-1}
  local runner out reference reference_literal source_literal name
  runner=$(mktemp "$ROOT_DIR/.zap-a9-arbitrary-parity.XXXXXX.zp")
  runner_rel=$(basename "$runner")
  out=$(mktemp)
  trap 'rm -f "$runner" "$out"' RETURN
  name=$(basename "$fixture")
  if [[ "$reference_mode" == "typed-ir" ]]; then
    reference=$(cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- bootstrap typed-ir "$fixture")
  else
    reference=$(cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- bootstrap diagnostics "$fixture")
  fi
  reference_literal=$(printf '%s' "$reference" | jq -c .)
  source_literal=$(jq -Rs . < "$fixture")
  cat >"$runner" <<ZP
import "bootstrap/b2/typed_ir.zp"
let reference = $reference_literal
let source = $source_literal
let first = from_json(emit_inferred_program_typed_ir(source, "$fixture"))
let second = from_json(emit_inferred_program_typed_ir(source, "$fixture"))
let comparison = typed_ir_compare_reference_program(reference, first)
say json(comparison)
say json(first) == json(second)
ZP
  ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
  mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
  [[ "${#lines[@]}" -eq 2 ]] || { echo "unexpected output for $fixture: ${lines[*]}" >&2; return 1; }
  printf '%s\n' "${lines[0]}" | jq -e --argjson expected_nodes "$expected_nodes" --argjson expected_semantic "$expected_semantic" \
    '(.candidate_kind == "zap.typed_ir") and (.candidate_only == true) and (.kind_match == true) and (.schema_match == true) and (.node_count_match == true) and ((.semantic_equal == true) == ($expected_semantic == 1)) and (.diagnostic_equal == true) and (.diagnostic_count_match == true) and (.semantic_reference_nodes == $expected_nodes) and (.semantic_candidate_nodes == $expected_nodes)' >/dev/null
  [[ "${lines[1]}" == "true" ]] || { echo "non-deterministic candidate output for $fixture" >&2; return 1; }
  rm -f "$runner" "$out"
  trap - RETURN
}

for case in "${positive[@]}"; do
  IFS=: read -r fixture expected_nodes expected_semantic <<< "$case"
  run_case "$fixture" typed-ir "$expected_nodes" "$expected_semantic"
done
for fixture in "${negative[@]}"; do
  run_case "$fixture" diagnostics 0 1
done

printf 'A9 bounded arbitrary parity gate passed: %d positive typed-IR programs and %d negative diagnostic programs, each compared twice\n' "${#positive[@]}" "${#negative[@]}"
