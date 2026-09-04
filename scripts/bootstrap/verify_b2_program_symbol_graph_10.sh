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
runner=$(mktemp "$ROOT_DIR/.zap-symbol-graph-runner.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let source = "fn add(left: number, right: number) -> number:\n    let total: number = 1\n    return total\nlet count: number = 1\nlet label: text = \"zap\"\n"
let graph = symbol_graph_collect(source)
let updated = symbol_graph_reassign(graph, "count", "text", "module")
say len(graph)
say symbol_graph_kind(graph, "add")
say symbol_graph_type(graph, "add")
say symbol_graph_scope(graph, "add")
say symbol_graph_kind(graph, "left")
say symbol_graph_type(graph, "left")
say symbol_graph_scope(graph, "left")
say symbol_graph_kind(graph, "total")
say symbol_graph_type(graph, "total")
say symbol_graph_scope(graph, "total")
say symbol_graph_type(graph, "count")
say symbol_graph_type(updated, "count")
say symbol_graph_scope(graph, "label")
say symbol_graph_kind(graph, "missing")
EOF
cat > "$expected" <<'EOF'
6
function
number
module
parameter
number
add
variable
number
add
number
text
module
unknown
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 program symbol graph gate passed: 14 collection, parameter, scope, and update cases\n'
