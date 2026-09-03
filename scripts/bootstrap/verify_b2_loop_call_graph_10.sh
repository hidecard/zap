#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-loop-call-runner.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let before = [{"name": "value", "type": "number"}, {"name": "label", "type": "text"}]
let same_body = [{"name": "value", "type": "number"}, {"name": "label", "type": "text"}]
let changed_body = [{"name": "value", "type": "text"}, {"name": "label", "type": "text"}]
let add = function_signature_info("fn add(left: number, right: number) -> number:")
let identity = function_signature_info("fn identity<T>(value: T) -> T:")
let bounded = function_signature_info("fn bounded<T>(value: T) -> T where T: number:")
let functions = [add, identity, bounded]
let calls = [{"caller": "main", "callee": "add", "argument_types": ["number", "number"]}, {"caller": "main", "callee": "identity", "argument_types": ["text"]}, {"caller": "worker", "callee": "bounded", "argument_types": ["text"]}]
let graph = call_graph_propagate([], functions, calls)
say loop_mutation_type(before, same_body, "value")
say loop_mutation_type(before, changed_body, "value")
say loop_mutation_type(before, changed_body, "label")
say len(graph)
say call_graph_return_type(graph, "main", "add")
say call_graph_return_type(graph, "main", "identity")
say call_graph_return_type(graph, "worker", "bounded")
say call_graph_lookup(graph, "missing", "add")
say infer_function_call(functions, "add", ["number", "number"])
say infer_function_call(functions, "bounded", ["text"])
EOF
cat > "$expected" <<'EOF'
number
any
text
3
number
text
constraint_error
unknown
number
constraint_error
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 loop/call-graph gate passed: 10 mutation and propagation cases\n'
