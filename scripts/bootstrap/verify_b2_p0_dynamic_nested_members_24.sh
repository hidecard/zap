#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b2-p0-dynamic-nested.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typecheck.zp"
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let typed = from_json(parse_general("class Profile:\n    let name: text = \"profile\"\nclass User:\n    let profile: Profile = none", "nested-types.zp"))
let classes = b2c_collect_classes(typed["ast"]["statements"], [])
let context_function = {"body": {"statements": []}, "constraints": [], "exported": false, "is_async": false, "kind": "function", "name": "context", "params": [], "return_type": none, "span": {"column": 1, "length": 10, "line": 1}, "type_params": [], "visibility": "public"}
let context = b2c_enrich_function(context_function, classes)
let environment = [b2c_binding("user", "User", "User", true)]
let nested = parse_expression("user.profile.name", 1)
let nested_result = b2c_infer_expr(nested, environment, [context], "nested-types.zp")
let ast_artifact = seed_compile_ast_source("class User:\n    fn set(self):\n        self.profile.name = \"zap\"\n        return self\nlet user = User()\nlet updated = user.set()\nsay updated.profile.name", "dynamic-ast.zp")
let ast_run = vm_run(ast_artifact["instructions"])
let source_artifact = seed_compile_source("class User:\n    fn set(self):\n        set self.profile.name = \"seed\"\n        return self\nlet user = User()\nlet updated = user.set()\nsay updated.profile.name", "dynamic-source.zp")
let source_run = vm_run(source_artifact["instructions"])
say nested["kind"]
say nested["target"]["kind"]
say nested_result["type"]
say len(nested_result["errors"])
say ast_artifact["status"]
say ast_run["error"]
say ast_run["output"][0]
say source_artifact["status"]
say source_run["error"]
say source_run["output"][0]
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["member", "member", "text", "0", "compiled_ast_slice", "none", "zap", "compiled_slice", "none", "seed"]:
    raise SystemExit(f"unexpected dynamic nested output: {lines!r}")
PY
printf 'B2/B4 P0 dynamic-nested gate passed: typed nested chain and dynamic field creation in AST/source runtimes\n'
