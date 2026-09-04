#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
run_zap() {
  if [[ -x "$ROOT_DIR/bin/zap" ]]; then
    "$ROOT_DIR/bin/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
    "$ROOT_DIR/native/target/release/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap" ]]; then
    "$ROOT_DIR/native/target/debug/zap" "$@"
  else
    cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$@"
  fi
}
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
tmp_dir=$(mktemp -d "$ROOT_DIR/.zap-diff.XXXXXX")
trap 'rm -rf "$tmp_dir"' EXIT
parser_fixtures=(
  bootstrap/fixtures/parser/arithmetic.zp
  bootstrap/fixtures/parser/compound.zp
  bootstrap/fixtures/parser/parenthesized_nested.zp
  bootstrap/fixtures/parser/nested_calls.zp
  bootstrap/fixtures/parser/simple_loop.zp
  bootstrap/fixtures/parser/simple_function.zp
)
diagnostic_fixtures=(
  bootstrap/fixtures/diagnostics/missing_closing_bracket.zp
  bootstrap/fixtures/diagnostics/invalid_character.zp
  bootstrap/fixtures/diagnostics/unterminated_string.zp
  bootstrap/fixtures/diagnostics/integer_overflow.zp
)
typed_fixtures=(
  bootstrap/fixtures/typecheck/expression_number_add.zp
  bootstrap/fixtures/typecheck/expression_text_add.zp
  bootstrap/fixtures/typecheck/expression_boolean_logic.zp
  bootstrap/fixtures/typecheck/list_annotation.zp
  bootstrap/fixtures/typecheck/map_annotation.zp
  bootstrap/fixtures/typecheck/generic_identity.zp
  bootstrap/fixtures/typecheck/generic_nested_option_list.zp
  bootstrap/fixtures/typecheck/conditional.zp
)
count=0
for fixture in "${parser_fixtures[@]}"; do
  base=$(basename "$fixture")
  run_zap bootstrap ast "$fixture" > "$tmp_dir/$base.ast.1"
  run_zap bootstrap ast "$fixture" > "$tmp_dir/$base.ast.2"
  cmp "$tmp_dir/$base.ast.1" "$tmp_dir/$base.ast.2"
  jq -e '.kind == "zap.ast" and (.schema_version | type == "number") and (.ast | type == "object")' "$tmp_dir/$base.ast.1" >/dev/null
  count=$((count + 1))
done
for fixture in "${diagnostic_fixtures[@]}"; do
  base=$(basename "$fixture")
  run_zap bootstrap diagnostics "$fixture" > "$tmp_dir/$base.diag.1"
  run_zap bootstrap diagnostics "$fixture" > "$tmp_dir/$base.diag.2"
  cmp "$tmp_dir/$base.diag.1" "$tmp_dir/$base.diag.2"
  jq -e '.kind == "zap.diagnostics" and (.schema_version | type == "number") and (.diagnostics | type == "array")' "$tmp_dir/$base.diag.1" >/dev/null
  count=$((count + 1))
done
for fixture in "${typed_fixtures[@]}"; do
  base=$(basename "$fixture")
  run_zap bootstrap typed-ir "$fixture" > "$tmp_dir/$base.ir.1"
  run_zap bootstrap typed-ir "$fixture" > "$tmp_dir/$base.ir.2"
  cmp "$tmp_dir/$base.ir.1" "$tmp_dir/$base.ir.2"
  jq -e '.kind == "zap.typed_ir" and (.schema_version | type == "number") and (.ir | type == "object")' "$tmp_dir/$base.ir.1" >/dev/null
  count=$((count + 1))
done
printf 'Reference differential matrix passed: %s deterministic parser/diagnostic/typed-IR cases\n' "$count"
