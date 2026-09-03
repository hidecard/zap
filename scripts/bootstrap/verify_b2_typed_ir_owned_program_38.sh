#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b2-owned-typed-ir.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b2/typed_ir.zp"
let result = from_json(emit_program_typed_ir("class Base:\n    let value: number = 7\n    fn read(self) -> number:\n        return self.value\nclass Child extends Base:\n    fn read(self) -> number:\n        if true:\n            return self.value\n        return 0\nfn main(item: Child) -> number:\n    for value in [1, 2]:\n        say value\n    return item.read()", "owned-ir.zp"))
let nodes = result["ir"]["nodes"]
say result["kind"]
say result["schema_version"]
say result["candidate_only"]
say result["typed_metadata"]
say len(nodes)
say nodes[0]["kind"]
say nodes[0]["body"]["statements"][0]["kind"]
say nodes[0]["body"]["statements"][1]["body"]["statements"][0]["value"]["target"]["kind"]
say nodes[1]["parents"][0]
say nodes[2]["body"]["statements"][0]["kind"]
say nodes[2]["body"]["statements"][1]["value"]["callee"]["kind"]
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
if lines != ["zap.typed_ir", "3", "true", "true", "3", "class", "declaration", "typed_expression", "Base", "for", "typed_expression"]:
    raise SystemExit(f"unexpected owned typed-IR output: {lines!r}")
PY
printf 'B2 owned typed-IR gate passed: class/inheritance/function/control-flow/member AST coverage\n'
