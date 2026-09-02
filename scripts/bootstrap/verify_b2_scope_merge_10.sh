#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-scope-runner.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let parameters = [{"name": "value", "type": "number"}, {"name": "label", "type": "text"}]
let locals = [{"name": "total", "type": "number"}]
let env = function_body_environment(parameters, locals)
let shadowed = symbol_environment_bind(env, "value", "text")
let then_env = symbol_environment_bind(env, "value", "number")
let else_env = symbol_environment_bind(env, "value", "text")
let same_else = symbol_environment_bind(env, "value", "number")
let number = {"kind": "literal", "literal_kind": "number"}
let value = {"kind": "name", "name": "value"}
let merged = branch_environment_merge(env, then_env, else_env)
let same_merged = branch_environment_merge(env, then_env, same_else)
say ast_lookup_type(env, "value")
say ast_lookup_type(env, "total")
say ast_lookup_type(shadowed, "value")
say ast_lookup_type(merged, "value")
say ast_lookup_type(same_merged, "value")
say ast_lookup_type(env, "missing")
say ast_expression_type_env(value, env)
say ast_expression_type_env(value, shadowed)
say infer_function_body_return(value, parameters, locals, "number")
say infer_function_body_return(number, parameters, locals, "text")
EOF
cat > "$expected" <<'EOF'
number
number
text
any
number
any
number
text
number
return_error
EOF
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 scope/branch gate passed: 10 symbol-environment and branch-merge cases\n'
