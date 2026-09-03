#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-nested-scope-runner.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let scopes = nested_scope_bind([], "module", "", "value", "variable", "number")
let scopes2 = nested_scope_bind(scopes, "fn:add", "module", "value", "parameter", "number")
let scopes3 = nested_scope_bind(scopes2, "if:add", "fn:add", "local", "variable", "text")
let scopes4 = nested_scope_bind(scopes3, "loop:add", "if:add", "item", "variable", "bool")
let base = [{"name": "value", "type": "number"}]
let then_env = symbol_environment_bind(symbol_environment_bind(base, "local", "text"), "value", "number")
let else_env = symbol_environment_bind(symbol_environment_bind(base, "local", "text"), "value", "text")
let merged = branch_local_declaration_merge(base, then_env, else_env, ["local", "value"])
say nested_scope_type(scopes4, "loop:add", "item")
say nested_scope_type(scopes4, "loop:add", "local")
say nested_scope_type(scopes4, "loop:add", "value")
say nested_scope_type(scopes4, "loop:add", "missing")
say ast_lookup_type(merged, "local")
say ast_lookup_type(merged, "value")
say ast_lookup_type(base, "local")
say ast_lookup_type(base, "value")
say symbol_environment_has(then_env, "local")
say symbol_environment_has(base, "local")
EOF
cat > "$expected" <<'EOF'
bool
text
number
any
text
any
any
number
true
false
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 nested-scope/branch-merge gate passed: 10 cases\n'
