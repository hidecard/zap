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
runner=$(mktemp "$ROOT_DIR/.zap-b2-engine-explicit.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b2/typecheck_engine.zp"
let generic = {"constraints": [], "name": "identity", "params": [{"annotation": "T", "name": "value"}], "return_type": "T", "type_params": ["T"]}
let valid = {"args": [{"value": {"kind": "literal", "literal_kind": "number", "value": 1}}], "callee": {"kind": "name", "name": "identity"}, "kind": "call", "type_args": ["number"]}
let mismatch = {"args": [{"value": {"kind": "literal", "literal_kind": "text", "value": "zap"}}], "callee": {"kind": "name", "name": "identity"}, "kind": "call", "type_args": ["number"]}
let arity = {"args": [{"value": {"kind": "literal", "literal_kind": "number", "value": 1}}], "callee": {"kind": "name", "name": "identity"}, "kind": "call", "type_args": ["number", "text"]}
let valid_result = b2c_infer_call(valid, [], [generic], "valid.zp")
let mismatch_result = b2c_infer_call(mismatch, [], [generic], "mismatch.zp")
let arity_result = b2c_infer_call(arity, [], [generic], "arity.zp")
say valid_result["type"]
say len(valid_result["errors"])
say mismatch_result["type"]
say len(mismatch_result["errors"])
say mismatch_result["errors"][0]["code"]
say len(arity_result["errors"])
say arity_result["errors"][0]["code"]
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
if lines != ["number", "0", "number", "1", "ZAP-TYPE-004", "1", "ZAP-TYPE-003"]:
    raise SystemExit(f"unexpected engine explicit generic output: {lines!r}")
PY
#!/bin/bash
# Verify B2 explicit generic arguments
set -e
printf 'B2 engine explicit-generic gate passed: explicit substitution, mismatch/arity diagnostics\n'
