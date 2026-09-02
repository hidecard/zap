#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b2-p0-member-methods.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typecheck.zp"
let parsed = from_json(parse_general("class Base:\n    fn greet(self) -> text:\n        return \"hello\"\nclass User extends Base:\n    fn name(self) -> text:\n        return \"user\"\nlet user = User()\nlet result = user.greet()", "member-methods.zp"))
let classes = b2c_collect_classes(parsed["ast"]["statements"], [])
let context_function = {"body": {"statements": []}, "constraints": [], "exported": false, "is_async": false, "kind": "function", "name": "context", "params": [], "return_type": none, "span": {"column": 1, "length": 10, "line": 1}, "type_params": [], "visibility": "public"}
let context = b2c_enrich_function(context_function, classes)
let user_env = [b2c_binding("user", "User", "User", true)]
let parsed_call = parsed["ast"]["statements"][3]["value"]
let valid_result = b2c_infer_call(parsed_call, user_env, [context], "valid.zp")
let unknown_call = parse_expression("user.missing()", 1)
let unknown_result = b2c_infer_call(unknown_call, user_env, [context], "unknown.zp")
say len(classes)
say parsed_call["callee"]["kind"]
say parsed_call["callee"]["member"]
say valid_result["type"]
say len(valid_result["errors"])
say len(unknown_result["errors"])
say unknown_result["errors"][0]["code"]
ZP
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["2", "member", "greet", "text", "0", "1", "ZAP-TYPE-005"]:
    raise SystemExit(f"unexpected member method output: {lines!r}")
PY
printf 'B2 P0 member-method gate passed: dotted method AST, inherited signature inference, missing-method diagnostic\n'
