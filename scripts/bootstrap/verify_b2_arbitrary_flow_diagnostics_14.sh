#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-arbitrary-flow.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let number_value = {"kind": "literal", "literal_kind": "number", "value": 7}
let text_value = {"kind": "literal", "literal_kind": "text", "value": "zap"}
let base = [{"name": "value", "type": "number"}]
let program = [{"kind": "assignment", "name": "value", "value": text_value}, {"kind": "if", "condition": {}, "then_branch": {"statements": [{"kind": "assignment", "name": "value", "value": number_value}]}, "else_branch": {"statements": [{"kind": "assignment", "name": "value", "value": text_value}]}}, {"kind": "assignment", "name": "value", "value": number_value}]
let graph = cfg_from_ast(program, base)
let final_type = flow_reassignment_propagate(program, base, "value")
let reference = diagnostic("flow.zp", 3, 5, "incompatible assignment")
let candidate = diagnostic("flow.zp", 3, 5, "incompatible assignment")
let parity = diagnostic_parity(reference, candidate)
let if_node = cfg_last_node(graph, "root.1")
let then_node = cfg_last_node(graph, "root.1.then.0")
let else_node = cfg_last_node(graph, "root.1.else.0")
say len(graph)
say if_node["successors"][0]
say if_node["successors"][1]
say then_node["successors"][0]
say else_node["successors"][0]
say final_type
say parity["equal"]
say reference["kind"]
say reference["code"]
say reference["line"]
say reference["column"]
say reference["message"]
say reference["severity"]
say len(reference["notes"])
say reference["source_name"]
EOF
cat > "$expected" <<'EOF'
5
root.1.then
root.1.else
root.2
root.2
number
true
TypeError
ZAP-TYPE-001
3
5
incompatible assignment
error
1
flow.zp
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 arbitrary-flow/diagnostic gate passed: 14 CFG, reassignment, and parity cases\n'
