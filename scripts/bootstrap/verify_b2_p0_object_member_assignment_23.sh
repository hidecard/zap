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
runner=$(mktemp "$ROOT_DIR/.zap-b2-p0-object-members.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typecheck.zp"
let parsed = from_json(parse_general("class User:\n    let name: text = \"zap\"", "object-members.zp"))
let classes = b2c_collect_classes(parsed["ast"]["statements"], [])
let context_function = {"body": {"statements": []}, "constraints": [], "exported": false, "is_async": false, "kind": "function", "name": "context", "params": [], "return_type": none, "span": {"column": 1, "length": 10, "line": 1}, "type_params": [], "visibility": "public"}
let context = b2c_enrich_function(context_function, classes)
let environment = [b2c_binding("object", "object<User>", "object<User>", true)]
let member = {"kind": "member", "member": "name", "span": {"column": 1, "length": 11, "line": 1}, "target": {"kind": "name", "name": "object"}}
let member_result = b2c_infer_expr(member, environment, [context], "member.zp")
let valid_assignment = {"kind": "assignment", "name": "object.name", "span": {"column": 1, "length": 18, "line": 1}, "value": {"kind": "literal", "literal_kind": "text", "value": "ok"}}
let invalid_assignment = {"kind": "assignment", "name": "object.name", "span": {"column": 1, "length": 18, "line": 1}, "value": {"kind": "literal", "literal_kind": "bool", "value": true}}
let initial = b2c_state(environment, [], true)
let valid_result = b2c_check_assignment(valid_assignment, initial, [context], "valid.zp")
let invalid_result = b2c_check_assignment(invalid_assignment, initial, [context], "invalid.zp")
say member_result["type"]
say len(member_result["errors"])
say len(valid_result["errors"])
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
if lines != ["text", "0", "0", "1", "ZAP-TYPE-001"]:
    raise SystemExit(f"unexpected object member assignment output: {lines!r}")
PY
printf 'B2 P0 object-member gate passed: object<Class> field lookup and assignment compatibility\n'
