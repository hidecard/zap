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
runner=$(mktemp "$ROOT_DIR/.zap-fixpoint-cycle-runner.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let before = [{"name": "value", "type": "number"}, {"name": "label", "type": "text"}]
let same_body = [{"name": "value", "type": "number"}, {"name": "label", "type": "text"}]
let changed_body = [{"name": "value", "type": "text"}, {"name": "label", "type": "text"}]
let self = call_graph_bind([], "self", "self", "number")
let mutual = call_graph_bind(call_graph_bind([], "a", "b", "number"), "b", "a", "number")
let acyclic = call_graph_bind([], "a", "b", "number")
say loop_fixpoint_type(before, same_body, "value", 5)
say loop_fixpoint_type(before, changed_body, "value", 5)
say loop_fixpoint_type(before, changed_body, "label", 5)
say call_graph_cycle_kind(self, "self")
say call_graph_cycle_kind(mutual, "a")
say call_graph_cycle_kind(acyclic, "a")
say call_graph_has_cycle(mutual, "b")
say call_graph_return_type(self, "self", "self")
say call_graph_return_type(mutual, "a", "b")
say call_graph_lookup(acyclic, "missing", "a")
EOF
cat > "$expected" <<'EOF'
number
any
text
cycle
cycle
acyclic
true
number
number
unknown
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 loop-fixpoint/cycle gate passed: 10 cases\n'
