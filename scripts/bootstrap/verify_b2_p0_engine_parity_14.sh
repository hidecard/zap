#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-b2-p0-engine.XXXXXX.zp")
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
let generic = {"name": "identity", "params": [{"annotation": "T", "name": "value"}], "return_type": "T", "type_params": ["T"]}
let missing = {"args": [], "callee": {"kind": "name", "name": "identity"}, "kind": "call"}
let missing_result = b2c_infer_call(missing, [], [generic], "engine.zp")
say b2c_assignable("option<number>", "none")
say b2c_assignable("result<text>", "error")
say b2c_join_type("option<number>", "none")
say b2c_lookup(paths["then"], "maybe")["current"]
say b2c_lookup(paths["then"], "ready")["current"]
say mixed_type["type"]
say len(missing_result["errors"])
ZP
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["true", "true", "option<number>", "number", "bool", "list<option<number>>", "1"]:
    raise SystemExit(f"unexpected engine parity output: {lines!r}")
PY
printf 'B2 P0 engine parity gate passed: nullable, nested guards, collection join, safe generic arity\n'
