#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-b2-heterogeneous.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b2/typecheck.zp"
let number_node = {"kind": "literal", "literal_kind": "number", "span": {"column": 2, "length": 1, "line": 1}, "value": 1}
let text_node = {"kind": "literal", "literal_kind": "text", "span": {"column": 5, "length": 5, "line": 1}, "value": "hello"}
let none_node = {"kind": "literal", "literal_kind": "none", "span": {"column": 8, "length": 4, "line": 1}, "value": none}
let mixed_list = b2c_infer_expr({"elements": [number_node, text_node], "kind": "list", "span": {"column": 1, "length": 10, "line": 1}}, [], [], "mixed-list.zp")
let option_list = b2c_infer_expr({"elements": [number_node, none_node], "kind": "list", "span": {"column": 1, "length": 10, "line": 1}}, [], [], "option-list.zp")
let reverse_option_list = b2c_infer_expr({"elements": [none_node, number_node], "kind": "list", "span": {"column": 1, "length": 10, "line": 1}}, [], [], "reverse-option-list.zp")
let nested_list = b2c_infer_expr({"elements": [{"elements": [number_node], "kind": "list", "span": {"column": 1, "length": 3, "line": 1}}, {"elements": [text_node], "kind": "list", "span": {"column": 1, "length": 7, "line": 1}}], "kind": "list", "span": {"column": 1, "length": 12, "line": 1}}, [], [], "nested-list.zp")
let mixed_map = b2c_infer_expr({"entries": [{"key": text_node, "value": number_node}, {"key": text_node, "value": text_node}], "kind": "map", "span": {"column": 1, "length": 20, "line": 1}}, [], [], "mixed-map.zp")
let empty_list = b2c_infer_expr({"elements": [], "kind": "list", "span": {"column": 1, "length": 2, "line": 1}}, [], [], "empty.zp")
say mixed_list["type"]
say len(mixed_list["errors"])
say mixed_list["errors"][0]["code"]
say option_list["type"]
say len(option_list["errors"])
say reverse_option_list["type"]
say len(reverse_option_list["errors"])
say nested_list["type"]
say len(nested_list["errors"])
say mixed_map["type"]
say len(mixed_map["errors"])
say empty_list["type"]
say len(empty_list["errors"])
ZP
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "list<any> 1 ZAP-TYPE-008 list<option<number>> 0 list<option<number>> 0 list<list<any>> 1 map<text,any> 1 list<any> 0" ]]; then
  echo "unexpected heterogeneous collection output: ${lines[*]}" >&2
  exit 1
fi
printf 'B2 heterogeneous collection gate passed: nested conflicts diagnosed, option joins preserved, empty collection bounded\n'
