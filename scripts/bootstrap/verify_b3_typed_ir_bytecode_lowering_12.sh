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
let mutation_ir = {"ir": {"nodes": [{"kind": "declaration", "name": "i", "value": expression_node("0")}, {"body": {"statements": [{"kind": "say", "payload": expression_node("i")}, {"kind": "assignment", "name": "i", "value": expression_node("i + 1")}]}, "condition": expression_node("i < 3"), "kind": "while"}]}, "kind": "zap.typed_ir"}
let mutation = lower_typed_ir(mutation_ir)
let mutation_state = vm_run(mutation["instructions"])
say mutation_state["output"][0]
say mutation_state["output"][1]
say mutation_state["output"][2]
let logical_ir = {"ir": {"nodes": [{"condition": expression_node("true and not false"), "else_branch": {"statements": [{"kind": "say", "payload": expression_node("0")}]}, "kind": "if", "then_branch": {"statements": [{"kind": "say", "payload": expression_node("7")}]}}]}, "kind": "zap.typed_ir"}
let logical = lower_typed_ir(logical_ir)
let logical_state = vm_run(logical["instructions"])
say logical_state["output"][0]
let short_and_ir = {"ir": {"nodes": [{"condition": expression_node("false and missing(1)"), "else_branch": {"statements": [{"kind": "say", "payload": expression_node("2")}]}, "kind": "if", "then_branch": {"statements": [{"kind": "say", "payload": expression_node("1")}]}}]}, "kind": "zap.typed_ir"}
let short_and = lower_typed_ir(short_and_ir)
let short_and_state = vm_run(short_and["instructions"])
say short_and_state["output"][0]
let short_or_ir = {"ir": {"nodes": [{"condition": expression_node("true or missing(1)"), "else_branch": {"statements": [{"kind": "say", "payload": expression_node("2")}]}, "kind": "if", "then_branch": {"statements": [{"kind": "say", "payload": expression_node("3")}]}}]}, "kind": "zap.typed_ir"}
let short_or = lower_typed_ir(short_or_ir)
let short_or_state = vm_run(short_or["instructions"])
say short_or_state["output"][0]
let break_ir = {"ir": {"nodes": [{"body": {"statements": [{"kind": "say", "payload": expression_node("1")}, {"kind": "break"}]}, "condition": expression_node("true"), "kind": "while"}]}, "kind": "zap.typed_ir"}
let broken = lower_typed_ir(break_ir)
let break_state = vm_run(broken["instructions"])
say break_state["output"][0]
let continue_ir = {"ir": {"nodes": [{"kind": "declaration", "name": "i", "value": expression_node("0")}, {"body": {"statements": [{"kind": "assignment", "name": "i", "value": expression_node("i + 1")}, {"kind": "continue"}]}, "condition": expression_node("i < 2"), "kind": "while"}]}, "kind": "zap.typed_ir"}
let continued = lower_typed_ir(continue_ir)
let continue_state = vm_run(continued["instructions"])
say len(continue_state["output"])
say rejected_state["error"]
ZP
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["zap.bytecode", "1", "14", "1", "9", "0", "0", "1", "2", "7", "2", "3", "1", "0", "unknown_call:missing"]:
    raise SystemExit(f"unexpected lowering output: {lines!r}")
PY
printf 'Typed-IR to bytecode lowering gate passed: arithmetic, say/VM handoff, schema, and deny-by-default rejection\n'
