#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-b4-method-overloads.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let artifact = seed_compile_source("class Base:\n    fn render(self, value):\n        return 10\n    fn render(self, value, extra):\n        return 20\nclass Child extends Base:\n    fn label(self):\n        return 30\nlet child = Child()\nsay child.render(1)\nsay child.render(1, 2)\nsay child.label()", "method-overloads.zp")
let result = vm_run(artifact["instructions"])
say artifact["status"]
say result["error"]
say result["output"][0]
say result["output"][1]
say result["output"][2]
ZP
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["compiled_slice", "none", "10", "20", "30"]:
    raise SystemExit(f"unexpected runtime overload output: {lines!r}")
PY
printf 'B4 method-overload gate passed: arity dispatch and inherited overload runtime calls\n'
