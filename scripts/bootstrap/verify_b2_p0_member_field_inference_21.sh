#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-b2-p0-member-fields.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typecheck.zp"
let parsed = from_json(parse_general("class Base:\n    let score: number = 1\nclass User extends Base:\n    let name: text = \"zap\"", "member-fields.zp"))
let classes = b2c_collect_classes(parsed["ast"]["statements"], [])
let context_function = {"body": {"statements": []}, "constraints": [], "exported": false, "is_async": false, "kind": "function", "name": "context", "params": [], "return_type": none, "span": {"column": 1, "length": 10, "line": 1}, "type_params": [], "visibility": "public"}
let context = b2c_enrich_function(context_function, classes)
let user_env = [b2c_binding("user", "User", "User", true)]
let member_name = {"kind": "member", "member": "name", "span": {"column": 1, "length": 9, "line": 1}, "target": {"kind": "name", "name": "user"}}
let member_score = {"kind": "member", "member": "score", "span": {"column": 1, "length": 10, "line": 1}, "target": {"kind": "name", "name": "user"}}
let member_missing = {"kind": "member", "member": "missing", "span": {"column": 1, "length": 12, "line": 1}, "target": {"kind": "name", "name": "user"}}
let name_result = b2c_infer_expr(member_name, user_env, [context], "name.zp")
let score_result = b2c_infer_expr(member_score, user_env, [context], "score.zp")
let missing_result = b2c_infer_expr(member_missing, user_env, [context], "missing.zp")
let parsed_member = parse_expression("user.name", 1)
say len(classes)
say parsed_member["kind"]
say name_result["type"]
say len(name_result["errors"])
say score_result["type"]
say len(score_result["errors"])
say len(missing_result["errors"])
say missing_result["errors"][0]["code"]
ZP
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["2", "member", "text", "0", "number", "0", "1", "ZAP-TYPE-005"]:
    raise SystemExit(f"unexpected member field output: {lines!r}")
PY
printf 'B2 P0 member-field gate passed: parsed member AST, declared/inherited fields, missing-member diagnostic\n'
