#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-fn-call-runner.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let add = function_signature_info("fn add(left: number, right: number) -> number:")
let identity = function_signature_info("fn identity<T>(value: T) -> T:")
let bounded = function_signature_info("fn bounded<T>(value: T) -> T where T: number:")
let functions = [add, identity, bounded]
let number = {"kind": "literal", "literal_kind": "number"}
let text = {"kind": "literal", "literal_kind": "text"}
let inner = {"kind": "call", "callee": {"kind": "name", "name": "identity"}, "args": [{"kind": "positional", "value": number}]}
let outer = {"kind": "call", "callee": {"kind": "name", "name": "bounded"}, "args": [{"kind": "positional", "value": inner}]}
let mismatch = function_call_diagnostic(functions, "add", ["number", "text"], "call.zp", 3)
say add["name"]
say len(add["params"])
say identity["name"]
say bounded["constraint"]
say infer_function_call(functions, "add", ["number", "number"])
say infer_function_call(functions, "add", ["number", "text"])
say infer_function_call(functions, "add", ["number"])
say infer_function_call(functions, "missing", ["number"])
say infer_function_call(functions, "bounded", ["text"])
say infer_ast_function_call(outer, functions, [])
say mismatch["message"]
say infer_return_type(number, [], "number")
say infer_return_type(text, [], "number")
EOF
cat > "$expected" <<'EOF'
add
2
identity
number
number
argument_error
arity_error
unknown_function
constraint_error
number
function 'add' received an incompatible argument
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
printf 'B2 function/call inference gate passed: 13 cases\n'
