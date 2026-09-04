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
runner=$(mktemp "$ROOT_DIR/.zap-b2-p0-inheritance-overloads.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typecheck.zp"
let parsed = from_json(parse_general("class Base:\n    fn render(self, value: number) -> text:\n        return \"number\"\n    fn render(self, value: text) -> number:\n        return 1\nclass Child extends Base:\n    let label: text = \"child\"\nfn accept(value: Base) -> text:\n    return \"accepted\"\nfn choose(value: any) -> text:\n    return \"text\"\nfn choose(value: any) -> number:\n    return 1", "inheritance-overloads.zp"))
let statements = parsed["ast"]["statements"]
let classes = b2c_collect_classes(statements, [])
let functions = b2c_enrich_functions(b2c_collect_functions(statements, []), classes, [], [])
let environment = [b2c_binding("child", "Child", "Child", true)]
let inherited_call = parse_expression("accept(child)", 1)
let number_method = parse_expression("child.render(1)", 1)
let text_method = parse_expression("child.render(\"x\")", 1)
let ambiguous = parse_expression("choose(1)", 1)
let inherited_result = b2c_infer_call(inherited_call, environment, functions, "inheritance.zp")
let number_result = b2c_infer_call(number_method, environment, functions, "number-method.zp")
let text_result = b2c_infer_call(text_method, environment, functions, "text-method.zp")
let ambiguous_result = b2c_infer_call(ambiguous, environment, functions, "ambiguous.zp")
say len(classes)
say inherited_result["type"]
say len(inherited_result["errors"])
say number_result["type"]
say len(number_result["errors"])
say text_result["type"]
say len(text_result["errors"])
say len(ambiguous_result["errors"])
say ambiguous_result["errors"][0]["code"]
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
if lines != ["2", "text", "0", "text", "0", "number", "0", "1", "ZAP-TYPE-010"]:
    raise SystemExit(f"unexpected inheritance/overload output: {lines!r}")
PY
printf 'B2 P0 inheritance/overload gate passed: child-to-parent compatibility, inherited overloads, exact overload selection, ambiguity diagnostic\n'
