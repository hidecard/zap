#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-recursive-cfg.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let env = [{"name": "value", "type": "number"}]
let ast = [{"condition": {}, "else_branch": {"statements": [{"kind": "return"}]}, "kind": "if", "then_branch": {"statements": [{"condition": {}, "else_branch": {"statements": [{"kind": "continue"}]}, "kind": "if", "then_branch": {"statements": [{"kind": "break"}]}}, {"kind": "return"}]}}, {"body": {"statements": [{"kind": "continue"}, {"kind": "break"}]}, "condition": {}, "kind": "while"}]
let graph = cfg_from_ast(ast, env)
let outer_if = cfg_last_node(graph, "root.0")
let inner_if = cfg_last_node(graph, "root.0.then.0")
let inner_break = cfg_last_node(graph, "root.0.then.0.then.0")
let inner_continue = cfg_last_node(graph, "root.0.then.0.else.0")
let loop = cfg_last_node(graph, "root.1")
let loop_continue = cfg_last_node(graph, "root.1.body.0")
let loop_break = cfg_last_node(graph, "root.1.body.1")
let stable = loop_backedge_converge(env, [env], ["value"], 5)
let divergent = loop_backedge_converge(env, [[{"name": "value", "type": "number"}], [{"name": "value", "type": "text"}]], ["value"], 5)
say len(graph)
say outer_if["successors"][0]
say inner_if["successors"][1]
say len(inner_break["successors"])
say len(inner_continue["successors"])
say loop["successors"][0]
say loop["successors"][1]
say loop_continue["successors"][0]
say loop_break["successors"][0]
say stable["converged"]
say stable["iterations"]
say divergent["converged"]
say ast_lookup_type(divergent["environment"], "value")
EOF
cat > "$expected" <<'EOF'
10
root.0.then
root.0.then.0.else
0
0
root.1.body
root.1.exit
root.1
root.1.exit
true
1
true
any
EOF
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 recursive-CFG/loop-convergence gate passed: 12 nested AST and loop ownership cases\n'
