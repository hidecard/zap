#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-condition-cfg.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let environment = [{"name": "maybe", "type": "option<number>"}, {"name": "outcome", "type": "result<text>"}]
let some_paths = condition_path_states(environment, "maybe", "is_some")
let ok_paths = condition_path_states(environment, "outcome", "is_ok")
let compound = compound_condition_paths(environment, [{"name": "maybe", "guard": "is_some"}, {"name": "outcome", "guard": "is_ok"}])
let statements = [{"kind": "assignment"}, {"kind": "if"}, {"kind": "return"}, {"kind": "while"}]
let graph = cfg_sequence_nodes(statements, environment, 1)
let edged = cfg_add_edge(graph, 2, 4)
say ast_lookup_type(some_paths["then"], "maybe")
say ast_lookup_type(some_paths["else"], "maybe")
say ast_lookup_type(ok_paths["then"], "outcome")
say ast_lookup_type(ok_paths["else"], "outcome")
say ast_lookup_type(compound["then"], "maybe")
say ast_lookup_type(compound["then"], "outcome")
say len(graph)
say graph[1]["kind"]
say graph[1]["successors"][0]
say edged[4]["successors"][1]
EOF
cat > "$expected" <<'EOF'
number
none
text
error
number
text
4
if
3
4
EOF
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 condition-derived narrowing/CFG gate passed: 10 guard, path, node, and edge cases\n'
