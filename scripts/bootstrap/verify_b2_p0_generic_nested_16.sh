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
runner=$(mktemp "$ROOT_DIR/.zap-b2-p0-generic.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typecheck.zp"
let parsed = from_json(parse_general("fn bounded<T>(value: T) -> T where T: number:\n    return value", "generic.zp"))
let parsed_function = parsed["ast"]["statements"][0]
let bounded = {"constraints": [{"bound": "number", "parameter": "T"}], "name": "bounded", "params": [{"annotation": "T", "default": none, "name": "value"}], "return_type": "T", "span": {"column": 1, "length": 10, "line": 1}, "type_params": ["T"]}
let valid_call = {"args": [{"value": {"kind": "literal", "literal_kind": "number", "value": 1}}], "callee": {"kind": "name", "name": "bounded"}, "span": {"column": 1, "length": 10, "line": 1}, "kind": "call"}
let invalid_call = {"args": [{"value": {"kind": "literal", "literal_kind": "text", "value": "zap"}}], "callee": {"kind": "name", "name": "bounded"}, "span": {"column": 1, "length": 10, "line": 1}, "kind": "call"}
let valid_result = b2c_infer_call(valid_call, [], [bounded], "generic-valid.zp")
let invalid_result = b2c_infer_call(invalid_call, [], [bounded], "generic-invalid.zp")
let wrap = {"name": "wrap", "params": [{"annotation": "T", "default": none, "name": "value"}], "return_type": "option<list<map<text,T>>>", "span": {"column": 1, "length": 10, "line": 1}, "type_params": ["T"]}
let nested_call = {"args": [{"value": {"kind": "literal", "literal_kind": "number", "value": 1}}], "callee": {"kind": "name", "name": "wrap"}, "span": {"column": 1, "length": 10, "line": 1}, "kind": "call"}
let nested_result = b2c_infer_call(nested_call, [], [wrap], "nested-generic.zp")
say parsed_function["return_type"]
say len(parsed_function["constraints"])
say parsed_function["constraints"][0]["parameter"]
say parsed_function["constraints"][0]["bound"]
say valid_result["type"]
say len(valid_result["errors"])
say len(invalid_result["errors"])
say invalid_result["errors"][0]["code"]
say nested_result["type"]
say len(nested_result["errors"])
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
if lines != ["T where T: number", "1", "T", "number", "number", "0", "1", "ZAP-TYPE-009", "option<list<map<text,number>>>", "0"]:
    raise SystemExit(f"unexpected generic/nested output: {lines!r}")
PY
printf 'B2 P0 generic/nested gate passed: where bounds, instantiation, invalid bounds, deep collections\n'
