#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
runner=$(mktemp "$ROOT_DIR/.zap-b1-branches.XXXXXX.zp")
out=$(mktemp "${TMPDIR:-/tmp}/zap-b1-branches-out.XXXXXX")
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZAP'
import "bootstrap/b1/parser.zp"
let elif_result = parse_block_program(["if first:", "    say 1", "elif second:", "    say 2", "elif third:", "    say 3", "else:", "    say 4"])
let root = elif_result[0]
let second = root["else_branch"]["statements"][0]
let third = second["else_branch"]["statements"][0]
let try_result = parse_block_program(["try:", "    say 1", "catch:", "    say 2"])
let try_node = try_result[0]
say root["kind"]
say second["kind"]
say third["kind"]
say len(third["else_branch"]["statements"])
say try_node["kind"]
say len(try_node["catch_body"]["statements"])
ZAP
export PATH="$HOME/.cargo/bin:$PATH"
export RUSTUP_TOOLCHAIN=1.88.0
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi >"$out"
grep -q '^if$' "$out"
grep -q '^1$' "$out"
grep -q '^try_catch$' "$out"
printf 'B1 unified branches gate passed: arbitrary elif chain and try/catch AST\n'
