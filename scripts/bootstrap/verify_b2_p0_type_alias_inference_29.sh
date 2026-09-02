#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b2-type-alias.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typecheck.zp"
let aliases = [{"kind": "type_alias", "name": "Count", "target": "number", "type_params": []}, {"kind": "type_alias", "name": "Box", "target": "option<list<T>>", "type_params": ["T"]}]
say b2c_expand_alias_type("Count", aliases, 0)
say b2c_expand_alias_type("Box<number>", aliases, 0)
say b2c_expand_alias_type("list<Box<text>>", aliases, 0)
let simple = from_json(check("type Count = number\nlet value: Count = 1", "simple-alias.zp"))
let nested = from_json(check("type Box<T> = option<list<T>>\nfn accept(value: Box<number>) -> number:\n    if is_some(value):\n        return 1\n    return 0\nlet result: number = accept(some([1]))", "nested-alias.zp"))
let invalid = from_json(check("type Box<T> = option<T>\nlet value: Box<number, text> = some(1)", "invalid-alias.zp"))
let nested_block = from_json(check("type Count = number\nlet i: Count = 0\nwhile i < 1:\n    let inner: Count = 1", "nested-block-alias.zp"))
say simple["ok"]
say len(simple["diagnostics"])
say nested["ok"]
say len(nested["diagnostics"])
say invalid["ok"]
say invalid["diagnostics"][0]["message"]
say nested_block["ok"]
say len(nested_block["diagnostics"])
ZP
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "number option<list<number>> list<option<list<text>>> true 0 true 0 false unknown type annotation 'Box<number, text>' true 0" ]]; then
  echo "unexpected type-alias output: ${lines[*]}" >&2
  exit 1
fi
printf 'B2 type-alias inference gate passed: simple, nested generic, call-site propagation, and arity rejection\n'
