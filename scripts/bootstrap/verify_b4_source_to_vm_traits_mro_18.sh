#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-b4-traits-mro.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let inherited = seed_compile_ast_source("trait Printable:\n    fn render(self) -> text:\n        return \"trait\"\nclass Base with Printable:\n    fn base(self) -> text:\n        return \"base\"\nclass Child extends Base:\n    fn render(self) -> text:\n        return super().render() + \"-child\"\nlet item = Child()\nsay item.render()", "trait-mro.zp")
let inherited_state = vm_run(inherited["instructions"])
say inherited["status"]
say inherited_state["error"]
say inherited_state["output"][0]
ZP
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["compiled_ast_slice", "none", "trait-child"]:
    raise SystemExit(f"unexpected trait MRO output: {lines!r}")
PY
printf 'B4 trait MRO compatibility gate passed: inherited provider and super continuation\n'
