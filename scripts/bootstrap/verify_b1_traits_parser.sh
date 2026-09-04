#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
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
runner=$(mktemp "$ROOT_DIR/.zap-b1-traits-parser.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'EOF'
import "bootstrap/b1/parser.zp"
let source = "trait Printable:\n    fn format(self) -> text\n    fn render(self) -> text:\n        return \"rendered\"\ninterface Identifiable:\n    fn id(self) -> text\nclass Report extends Base with Printable implements Identifiable:\n    fn format(self) -> text:\n        return \"report\"\n    fn id(self) -> text:\n        return \"id\""
say parse_general(source, "traits-parser.zp")
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi >"$out"
grep -q '"kind":"trait"' "$out"
grep -q '"name":"Printable"' "$out"
grep -q '"required":true' "$out"
grep -q '"required":false' "$out"
grep -q '"kind":"interface"' "$out"
grep -q '"name":"Identifiable"' "$out"
grep -q '"parents":\["Base"\]' "$out"
grep -q '"traits":\["Printable"\]' "$out"
grep -q '"interfaces":\["Identifiable"\]' "$out"
printf 'B1 trait parser gate passed: trait/interface declarations, required/provided methods, and class composition metadata\n'
