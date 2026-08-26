#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-lowering.XXXXXX.zp")
out=$(mktemp "${TMPDIR:-/tmp}/zap-lowering.XXXXXX.json")
cleanup() { rm -f "$runner" "$out"; }
trap cleanup EXIT
cat > "$runner" <<'ZP'
import "bootstrap/b2/typed_ir.zp"
import "bootstrap/b3/lower.zp"
import "bootstrap/b3/vm.zp"
let first = from_json(emit("say 2 + 3 * 4", "lower.zp"))
let lowered = lower_typed_ir(first)
let state = vm_run(lowered["instructions"])
say lowered["kind"]
say lowered["schema_version"]
say state["output"][0]
let called = lower_typed_ir(from_json(emit("say identity(1)", "call.zp")))
let call_state = vm_run(called["instructions"])
say call_state["output"][0]
let rejected = lower_typed_ir(from_json(emit("say missing(1)", "unsupported.zp")))
let rejected_state = vm_run(rejected["instructions"])
let branch_ir = {"ir": {"nodes": [{"condition": expression_node("false"), "else_branch": {"statements": [{"kind": "say", "payload": expression_node("9")}]}, "kind": "if", "then_branch": {"statements": [{"kind": "say", "payload": expression_node("1")}]}}]}, "kind": "zap.typed_ir"}
let branch = lower_typed_ir(branch_ir)
let branch_state = vm_run(branch["instructions"])
say branch_state["output"][0]
let loop_ir = {"ir": {"nodes": [{"body": {"statements": [{"kind": "say", "payload": expression_node("1")}]}, "condition": expression_node("false"), "kind": "while"}]}, "kind": "zap.typed_ir"}
let loop = lower_typed_ir(loop_ir)
let loop_state = vm_run(loop["instructions"])
say len(loop_state["output"])
say rejected_state["error"]
ZP
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["zap.bytecode", "1", "14", "1", "9", "0", "unknown_call:missing"]:
    raise SystemExit(f"unexpected lowering output: {lines!r}")
PY
printf 'Typed-IR to bytecode lowering gate passed: arithmetic, say/VM handoff, schema, and deny-by-default rejection\n'
