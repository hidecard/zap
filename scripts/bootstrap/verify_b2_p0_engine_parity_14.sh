#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b2-p0-engine.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b2/typecheck_engine.zp"
let base = [b2c_binding("maybe", "option<number>", "option<number>", true), b2c_binding("ready", "bool", "bool", true)]
let is_some = {"args": [{"value": {"kind": "name", "name": "maybe"}}], "callee": {"kind": "name", "name": "is_some"}, "kind": "call"}
let is_none = {"args": [{"value": {"kind": "name", "name": "maybe"}}], "callee": {"kind": "name", "name": "is_option_none"}, "kind": "call"}
let nested_guard = {"kind": "binary", "left": is_some, "op": "and", "right": {"kind": "unary", "op": "not", "value": is_none}}
let paths = b2c_guard_paths(base, nested_guard)
let mixed = {"elements": [{"kind": "literal", "literal_kind": "number", "value": 1}, {"kind": "literal", "literal_kind": "none", "value": none}], "kind": "list"}
let mixed_type = b2c_infer_expr(mixed, [], [], "engine.zp")
let generic = {"constraints": [{"bound": "number", "parameter": "T"}], "name": "identity", "params": [{"annotation": "T", "name": "value"}], "return_type": "T", "type_params": ["T"]}
let missing = {"args": [], "callee": {"kind": "name", "name": "identity"}, "kind": "call"}
let missing_result = b2c_infer_call(missing, [], [generic], "engine.zp")
let valid_call = {"args": [{"value": {"kind": "literal", "literal_kind": "number", "value": 1}}], "callee": {"kind": "name", "name": "identity"}, "kind": "call"}
let invalid_call = {"args": [{"value": {"kind": "literal", "literal_kind": "text", "value": "zap"}}], "callee": {"kind": "name", "name": "identity"}, "kind": "call"}
let valid_result = b2c_infer_call(valid_call, [], [generic], "engine.zp")
let invalid_result = b2c_infer_call(invalid_call, [], [generic], "engine.zp")
say b2c_assignable("option<number>", "none")
say b2c_assignable("result<text>", "error")
say b2c_join_type("option<number>", "none")
say b2c_lookup(paths["then"], "maybe")["current"]
say b2c_lookup(paths["then"], "ready")["current"]
say mixed_type["type"]
say len(missing_result["errors"])
say len(valid_result["errors"])
say len(invalid_result["errors"])
say invalid_result["errors"][0]["code"]
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["true", "true", "option<number>", "number", "bool", "list<option<number>>", "1", "0", "1", "ZAP-TYPE-009"]:
    raise SystemExit(f"unexpected engine parity output: {lines!r}")
PY
printf 'B2 P0 engine parity gate passed: nullable, nested guards, collection join, safe generic arity\n'
