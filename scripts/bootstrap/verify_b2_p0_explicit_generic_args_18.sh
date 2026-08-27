#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-b2-p0-explicit-generics.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typecheck.zp"
let parsed = from_json(parse_general("fn identity<T>(value: T) -> T:\n    return value\nlet result: number = identity<number>(1)", "explicit.zp"))
let parsed_call = parsed["ast"]["statements"][1]["value"]
let generic = {"body": {"statements": []}, "constraints": [], "exported": false, "is_async": false, "kind": "function", "name": "identity", "params": [{"annotation": "T", "default": none, "name": "value"}], "return_type": "T", "span": {"column": 1, "length": 10, "line": 1}, "type_params": ["T"], "visibility": "public"}
let valid = b2c_infer_call(parsed_call, [], [generic], "valid.zp")
let mismatch = {"args": [{"value": {"kind": "literal", "literal_kind": "text", "value": "zap"}}], "callee": {"kind": "name", "name": "identity"}, "kind": "call", "span": {"column": 1, "length": 10, "line": 1}, "type_args": ["number"]}
let arity = {"args": [{"value": {"kind": "literal", "literal_kind": "number", "value": 1}}], "callee": {"kind": "name", "name": "identity"}, "kind": "call", "span": {"column": 1, "length": 10, "line": 1}, "type_args": ["number", "text"]}
let inferred = {"args": [{"value": {"kind": "literal", "literal_kind": "text", "value": "zap"}}], "callee": {"kind": "name", "name": "identity"}, "kind": "call", "span": {"column": 1, "length": 10, "line": 1}, "type_args": []}
let mismatch_result = b2c_infer_call(mismatch, [], [generic], "mismatch.zp")
let arity_result = b2c_infer_call(arity, [], [generic], "arity.zp")
let inferred_result = b2c_infer_call(inferred, [], [generic], "inferred.zp")
say parsed_call["type_args"][0]
say valid["type"]
say len(valid["errors"])
say mismatch_result["type"]
say len(mismatch_result["errors"])
say mismatch_result["errors"][0]["code"]
say len(arity_result["errors"])
say arity_result["errors"][0]["code"]
say inferred_result["type"]
say len(inferred_result["errors"])
ZP
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["number", "number", "0", "number", "1", "ZAP-TYPE-001", "1", "ZAP-TYPE-001", "text", "0"]:
    raise SystemExit(f"unexpected explicit generic output: {lines!r}")
PY
printf 'B2 P0 explicit-generic gate passed: parsed type args, explicit substitution, mismatch/arity diagnostics, inferred calls\n'
