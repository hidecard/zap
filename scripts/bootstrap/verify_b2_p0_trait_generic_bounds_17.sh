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
runner=$(mktemp "$ROOT_DIR/.zap-b2-p0-trait-bound.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typecheck.zp"
let parsed = from_json(parse_general("interface Identifiable:\n    fn id(self) -> text\nclass User implements Identifiable:\n    fn id(self) -> text:\n        return \"user\"\nclass Admin extends User:\n    fn id(self) -> text:\n        return \"admin\"\nclass Plain:\n    fn name(self) -> text:\n        return \"plain\"\nclass Missing implements Identifiable:\n    fn name(self) -> text:\n        return \"missing\"", "trait-bound.zp"))
let classes = b2c_collect_classes(parsed["ast"]["statements"], [])
let registry = b2c_collect_trait_registry(parsed["ast"]["statements"], [])
let bounded = {"body": {"statements": []}, "constraints": [{"bound": "Identifiable", "parameter": "T"}], "exported": false, "is_async": false, "kind": "function", "name": "accept", "params": [{"annotation": "T", "default": none, "name": "value"}], "return_type": "T", "span": {"column": 1, "length": 10, "line": 1}, "type_params": ["T"], "visibility": "public"}
let enriched = b2c_enrich_function_with_registry(bounded, classes, registry)
let bounded_list = {"body": {"statements": []}, "constraints": [{"bound": "Identifiable", "parameter": "T"}], "exported": false, "is_async": false, "kind": "function", "name": "accept_list", "params": [{"annotation": "list<T>", "default": none, "name": "values"}], "return_type": "list<T>", "span": {"column": 1, "length": 10, "line": 1}, "type_params": ["T"], "visibility": "public"}
let enriched_list = b2c_enrich_function_with_registry(bounded_list, classes, registry)
let user_env = [b2c_binding("user", "User", "User", true)]
let admin_env = [b2c_binding("admin", "Admin", "Admin", true)]
let plain_env = [b2c_binding("plain", "Plain", "Plain", true)]
let missing_env = [b2c_binding("missing", "Missing", "Missing", true)]
let user_list_env = [b2c_binding("users", "list<User>", "list<User>", true)]
let plain_list_env = [b2c_binding("plains", "list<Plain>", "list<Plain>", true)]
let user_call = {"args": [{"value": {"kind": "name", "name": "user"}}], "callee": {"kind": "name", "name": "accept"}, "span": {"column": 1, "length": 10, "line": 1}, "kind": "call"}
let admin_call = {"args": [{"value": {"kind": "name", "name": "admin"}}], "callee": {"kind": "name", "name": "accept"}, "span": {"column": 1, "length": 10, "line": 1}, "kind": "call"}
let plain_call = {"args": [{"value": {"kind": "name", "name": "plain"}}], "callee": {"kind": "name", "name": "accept"}, "span": {"column": 1, "length": 10, "line": 1}, "kind": "call"}
let missing_call = {"args": [{"value": {"kind": "name", "name": "missing"}}], "callee": {"kind": "name", "name": "accept"}, "span": {"column": 1, "length": 10, "line": 1}, "kind": "call"}
let user_list_call = {"args": [{"value": {"kind": "name", "name": "users"}}], "callee": {"kind": "name", "name": "accept_list"}, "span": {"column": 1, "length": 10, "line": 1}, "kind": "call"}
let plain_list_call = {"args": [{"value": {"kind": "name", "name": "plains"}}], "callee": {"kind": "name", "name": "accept_list"}, "span": {"column": 1, "length": 10, "line": 1}, "kind": "call"}
let user_result = b2c_infer_call(user_call, user_env, [enriched], "user.zp")
let admin_result = b2c_infer_call(admin_call, admin_env, [enriched], "admin.zp")
let plain_result = b2c_infer_call(plain_call, plain_env, [enriched], "plain.zp")
let missing_result = b2c_infer_call(missing_call, missing_env, [enriched], "missing.zp")
let user_list_result = b2c_infer_call(user_list_call, user_list_env, [enriched_list], "user-list.zp")
let plain_list_result = b2c_infer_call(plain_list_call, plain_list_env, [enriched_list], "plain-list.zp")
say len(classes)
say user_result["type"]
say len(user_result["errors"])
say admin_result["type"]
say len(admin_result["errors"])
say len(plain_result["errors"])
say plain_result["errors"][0]["code"]
say len(missing_result["errors"])
say missing_result["errors"][0]["code"]
say user_list_result["type"]
say len(user_list_result["errors"])
say len(plain_list_result["errors"])
say plain_list_result["errors"][0]["code"]
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
if lines != ["4", "User", "0", "Admin", "0", "1", "ZAP-TYPE-009", "1", "ZAP-TYPE-009", "list<User>", "0", "1", "ZAP-TYPE-009"]:
    raise SystemExit(f"unexpected trait bound output: {lines!r}")
PY
printf 'B2 P0 trait-generic gate passed: direct/interface conformance, inherited conformance, rejection\n'
