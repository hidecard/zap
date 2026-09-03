#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-next20.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let env = [{"name": "value", "type": "number"}]
let condition = {"kind": "name", "name": "flag"}
let condition_node = cfg_condition_node("c", condition, "yes", "no", env)
let exception_node = cfg_exception_node("t", "body", "catch", "next", env)
let scope_node = cfg_scope_node("f", "function", "entry", "exit", env)
let number_value = {"kind": "literal", "literal_kind": "number", "value": 1}
let text_value = {"kind": "literal", "literal_kind": "text", "value": "zap"}
let assignment = {"kind": "assignment", "name": "value", "value": text_value}
let flow = cfg_flow_statement(assignment, env)
let reference = diagnostic("next20.zp", 2, 3, "bad assignment")
let parity = diagnostic_failure_parity(reference, diagnostic("next20.zp", 2, 3, "bad assignment"))
say cfg_statement_category("if")
say cfg_statement_category("while")
say cfg_statement_category("for")
say cfg_statement_category("try_catch")
say cfg_statement_category("function")
say cfg_statement_category("class")
say cfg_statement_category("assignment")
say cfg_statement_category("say")
say condition_node["kind"]
say condition_node["condition"]["name"]
say condition_node["successors"][1]
say exception_node["kind"]
say exception_node["successors"][1]
say scope_node["kind"]
say scope_node["successors"][0]
say ast_lookup_type(flow, "value")
say flow_reassignment_invalidate(env, "value", "text", ["value"])[1]["type"]
say parity["equal"]
say parity["failure"]
EOF
cat > "$expected" <<'EOF'
control
control
control
exception
scope
scope
binding
effect
condition
flag
no
try_catch
catch
function
entry
text
text
true
none
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'Section A next-20 gate passed: 20 CFG, flow, and diagnostic assertions\n'
