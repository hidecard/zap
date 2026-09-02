#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-scope-exit-runner.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let base = [{"name": "value", "type": "number"}]
let then_env = symbol_environment_bind(base, "value", "number")
let else_env = symbol_environment_bind(base, "value", "text")
let child = scope_enter(base, [{"name": "local", "type": "bool"}])
let restored = scope_exit(base, child)
let merged = branch_environment_merge_paths(base, then_env, else_env, false, false)
let then_return = branch_environment_merge_paths(base, then_env, else_env, true, false)
let else_return = branch_environment_merge_paths(base, then_env, else_env, false, true)
let both_return = branch_environment_merge_paths(base, then_env, else_env, true, true)
let snapshot = symbol_environment_snapshot(child)
say ast_lookup_type(child, "local")
say ast_lookup_type(restored, "local")
say ast_lookup_type(merged, "value")
say ast_lookup_type(then_return, "value")
say ast_lookup_type(else_return, "value")
say ast_lookup_type(both_return, "value")
say ast_lookup_type(snapshot, "local")
say ast_lookup_type(base, "value")
say ast_lookup_type(scope_exit(base, child), "value")
say ast_lookup_type(scope_exit(base, child), "local")
EOF
cat > "$expected" <<'EOF'
bool
any
any
text
number
number
bool
number
number
any
EOF
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 scope-exit gate passed: 10 branch-merge and scope-restoration cases\n'
