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
runner=$(mktemp "$ROOT_DIR/.zap-typed-ir-generic-runner.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typed_ir.zp"
let source = "let count: number = 1\nlet label: text = \"zap\"\nlet values: list<number> = [1, 2]\nlet flags: map<text,bool> = {\"ok\": true}\n"
let output = from_json(emit(source, "multi.zp"))
let nodes = output["ir"]["nodes"]
let metadata = generic_function_metadata("fn pair<T, U>(value: T) -> U:", "U")
say len(nodes)
say nodes[0]["name"]
say nodes[1]["inferred_type"]
say nodes[2]["inferred_type"]
say nodes[3]["inferred_type"]
say nodes[3]["span"]["line"]
say metadata["name"]
say len(metadata["type_params"])
say metadata["return_type"]
say generic_parameters("fn pair<T, U>(value: T) -> U:")[1]
EOF
cat > "$expected" <<'EOF'
4
count
text
list<number>
map<text,bool>
4
pair
2
U
U
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 typed-IR/generic gate passed: 10 multi-declaration and metadata cases\n'
