#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-generic-runtime.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let functions = [function_signature_info("fn identity<T>(value: T) -> T:"), function_signature_info("fn wrap<T>(value: T) -> option<T>:")]
say generic_runtime_container_kind("list<number>")
say generic_runtime_container_kind("map<text,number>")
say generic_runtime_container_kind("option<text>")
say generic_runtime_container_accepts("list_push", "list<number>", "number")
say generic_runtime_container_accepts("map_key", "map<text,number>", "text")
say generic_runtime_container_accepts("map_value", "map<text,number>", "number")
say generic_runtime_container_accepts("option_some", "option<text>", "text")
say generic_runtime_container_accepts("list_push", "list<number>", "text")
say generic_recursive_call_result(functions, "identity", ["text"], ["T"], "T", 4)
say generic_recursive_call_result(functions, "wrap", ["number"], ["T"], "option<T>", 4)
say generic_recursive_call_result(functions, "identity", ["text"], ["T"], "T", 0)
EOF
cat > "$expected" <<'EOF'
list
map
option
true
true
true
true
false
text
option<number>
recursion_limit
EOF
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 generic runtime/recursive-call gate passed: 11 container and recursive generic cases\n'
