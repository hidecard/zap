#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
runner=$(mktemp "$ROOT_DIR/.zap-b1-class-methods.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp "${TMPDIR:-/tmp}/zap-b1-class-methods-out.XXXXXX")
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZAP'
import "bootstrap/b1/parser.zp"
let one = parse_generic_class(["class Box:", "    fn get(self):", "        return 1"])
let two = parse_generic_class(["class Box:", "    fn get(self):", "        return 1", "    fn set(self, value):", "        return value"])
let three = parse_generic_class(["class Box:", "    fn get(self):", "        return 1", "    fn set(self, value):", "        return value", "    fn clear(self):", "        return 0"])
let mixed_nested = parse_generic_class(["class Counter:", "    let value = 0", "    fn get(self):", "        if self:", "            return 1", "        else:", "            return 0", "    fn set(self, value):", "        return value"])
let one_members = one[0]["body"]["statements"]
let two_members = two[0]["body"]["statements"]
let three_members = three[0]["body"]["statements"]
let mixed_members = mixed_nested[0]["body"]["statements"]
say len(one_members)
say len(two_members)
say len(three_members)
say two_members[0]["name"]
say two_members[1]["name"]
say three_members[2]["name"]
say len(mixed_members)
say mixed_members[1]["body"]["statements"][0]["kind"]
ZAP
export PATH="$HOME/.cargo/bin:$PATH"
export RUSTUP_TOOLCHAIN=1.88.0
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
grep -q '^1$' "$out"
grep -q '^2$' "$out"
grep -q '^3$' "$out"
grep -q '^get$' "$out"
grep -q '^set$' "$out"
grep -q '^clear$' "$out"
grep -q '^3$' "$out"
grep -q '^if$' "$out"
printf 'B1 multiple class methods gate passed: fixed and nested method bodies with mixed members\n'
