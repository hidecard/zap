#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
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
runner=$(mktemp "$ROOT_DIR/.zap-short-loop.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let environment = [{"name": "left", "type": "option<number>"}, {"name": "right", "type": "option<text>"}]
let left = {"name": "left", "guard": "is_some"}
let right = {"name": "right", "guard": "is_some"}
let and_paths = short_circuit_condition_paths(environment, left, right, "and")
let or_paths = short_circuit_condition_paths(environment, left, right, "or")
let base = [{"name": "value", "type": "number"}]
let nested = nested_branch_flow(base, [{"environment": base, "returns": false}, {"environment": [{"name": "value", "type": "text"}], "returns": true}, {"environment": base, "returns": false}])
let graph = cfg_sequence_nodes([{"kind": "while"}, {"kind": "continue"}, {"kind": "break"}, {"kind": "return"}], base, 1)
let looped = cfg_loop_back_edge(graph, 1, 1)
let owned = cfg_loop_edges(looped, 1, 5, [3], [2])
let loop_node = cfg_last_node(owned, 1)
let continue_node = cfg_last_node(owned, 2)
let break_node = cfg_last_node(owned, 3)
say ast_lookup_type(and_paths["then"], "left")
say ast_lookup_type(and_paths["then"], "right")
say ast_lookup_type(and_paths["else"], "left")
say ast_lookup_type(or_paths["then"], "right")
say ast_lookup_type(or_paths["else"], "right")
say ast_lookup_type(nested, "value")
say loop_node["successors"][1]
say continue_node["successors"][0]
say break_node["successors"][0]
say graph[1]["kind"]
say graph[2]["kind"]
say graph[3]["kind"]
EOF
cat > "$expected" <<'EOF'
number
text
any
any
none
number
1
1
5
continue
break
return
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 short-circuit/loop-edge gate passed: 12 and/or, nested-branch, back-edge, break, and continue cases\n'
