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
runner=$(mktemp "$ROOT_DIR/.zap-generic-e2e-runner.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
import "bootstrap/b2/typed_ir.zp"
let function = function_signature_info("fn pair<T,U>(left: T, right: U) -> map<T,list<U>> where T: number and U: text:")
let alias = generic_type_declaration_info("type Pair<T,U> = map<T,list<U>>")
say function["name"]
say function["generic_parameters"]
say function["constraints"][0]["bound"]
say function["constraints"][1]["bound"]
say infer_function_call([function], "pair", ["number", "text"])
say infer_function_call([function], "pair", ["text", "number"])
say alias["name"]
say alias["valid"]
say generic_type_declaration_instance("type Pair<T,U> = map<T,list<U>>", ["number", "text"])
say generic_type_alias_info("type Pair<T,U> = map<T,list<U>>")["type_params"]
EOF
cat > "$expected" <<'EOF'
pair
[T, U]
number
text
map<number,list<text>>
constraint_error
Pair
true
map<number,list<text>>
[T, U]
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 generic end-to-end gate passed: 10 parser/typechecker/typed-IR integration cases\n'
