#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
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
runner=$(mktemp "$ROOT_DIR/.zap-a9-typed-ir-compare.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b2/typed_ir.zp"
let reference = {"ir": {"nodes": [{"annotation": "number", "exported": false, "inferred_type": "number", "ir_schema_version": 1, "kind": "declaration", "name": "total", "span": {"column": 1, "length": 25, "line": 1}, "value": {"kind": "binary", "left": {"kind": "literal", "literal_kind": "number", "span": {"column": 1, "length": 1, "line": 1}, "value": 1}, "op": "add", "right": {"kind": "literal", "literal_kind": "number", "span": {"column": 5, "length": 1, "line": 1}, "value": 2}, "span": {"column": 1, "length": 5, "line": 1}}}]}, "kind": "zap.typed_ir", "reference_only": true, "schema_version": 1, "source_name": "expression_number_add.zp"}
let source = "let total: number = 1 + 2"
let first = from_json(emit_inferred_program_typed_ir(source, "expression_number_add.zp"))
let second = from_json(emit_inferred_program_typed_ir(source, "expression_number_add.zp"))
let comparison = typed_ir_compare_reference(reference, first)
say comparison["candidate_kind"]
say comparison["candidate_only"]
say comparison["kind_match"]
say comparison["schema_match"]
say comparison["node_count_match"]
say comparison["semantic_equal"]
say comparison["deterministic"]
say json(first) == json(second)
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["zap.typed_ir", "true", "true", "true", "true", "true", "true", "true"]:
    raise SystemExit(f"unexpected A9 comparison output: {lines!r}")
PY
printf 'A9 typed-IR comparison gate passed: schema projection, semantic declaration parity, and deterministic repeat\n'
