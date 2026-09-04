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
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b2-p0-for-types.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b2/typecheck.zp"
let list_statement = {"binding": "item", "body": {"statements": [{"kind": "assignment", "name": "seen", "value": {"kind": "name", "name": "item", "span": {"column": 1, "length": 4, "line": 1}}, "span": {"column": 1, "length": 8, "line": 1}}]}, "iterable": {"elements": [{"kind": "literal", "literal_kind": "number", "value": 1}], "kind": "list", "span": {"column": 12, "length": 3, "line": 1}}, "kind": "for", "span": {"column": 1, "length": 15, "line": 1}}
let map_statement = {"binding": "value", "body": {"statements": [{"kind": "assignment", "name": "flag", "value": {"kind": "name", "name": "value", "span": {"column": 1, "length": 5, "line": 1}}, "span": {"column": 1, "length": 9, "line": 1}}]}, "iterable": {"entries": [{"key": {"kind": "literal", "literal_kind": "text", "value": "k"}, "value": {"kind": "literal", "literal_kind": "bool", "value": true}}], "kind": "map", "span": {"column": 12, "length": 8, "line": 1}}, "kind": "for", "span": {"column": 1, "length": 20, "line": 1}}
let invalid_statement = {"binding": "item", "body": {"statements": []}, "iterable": {"kind": "literal", "literal_kind": "number", "value": 1, "span": {"column": 12, "length": 1, "line": 1}}, "kind": "for", "span": {"column": 1, "length": 15, "line": 1}}
let initial = b2c_state([b2c_binding("seen", "number", "number", true), b2c_binding("flag", "bool", "bool", true)], [], true)
let list_result = b2c_check_for(list_statement, initial, [], "any", "list.zp", "")
let map_result = b2c_check_for(map_statement, initial, [], "any", "map.zp", "")
let invalid_result = b2c_check_for(invalid_statement, initial, [], "any", "invalid.zp", "")
say b2c_lookup(list_result["environment"], "seen")["current"]
say b2c_lookup(map_result["environment"], "flag")["current"]
say len(list_result["errors"])
say len(map_result["errors"])
say len(invalid_result["errors"])
say invalid_result["errors"][0]["code"]
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["number", "bool", "0", "0", "1", "ZAP-TYPE-001"]:
    raise SystemExit(f"unexpected for iterable output: {lines!r}")
PY
printf 'B2 P0 for-iterable gate passed: list/map element inference and invalid iterable diagnostic\n'
