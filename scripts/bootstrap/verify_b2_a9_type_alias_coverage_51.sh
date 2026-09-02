#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-a9-type-alias.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b2/typed_ir.zp"
let source = "type Pair<T,U> = map<T,list<U>>"
let first = from_json(emit_inferred_program_typed_ir(source, "alias.zp"))
let second = from_json(emit_inferred_program_typed_ir(source, "alias.zp"))
let contract = typed_ir_general_contract(first, second)
let node = first["ir"]["nodes"][0]
say contract["valid"]
say contract["deterministic"]
say node["kind"]
say node["name"]
say node["body"]
say node["inferred_type"]
say len(node["type_params"])
say node["span"]["line"]
ZP
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "true true type_alias Pair map<T,list<U>> map<T,list<U>> 2 1" ]]; then
  echo "unexpected A9 type-alias output: ${lines[*]}" >&2
  exit 1
fi
printf 'A9 type-alias coverage gate passed: generic alias metadata, inferred body type, span, and deterministic repeated emission\n'
