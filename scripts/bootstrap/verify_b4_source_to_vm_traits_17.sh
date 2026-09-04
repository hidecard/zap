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
runner=$(mktemp "$ROOT_DIR/.zap-b4-traits.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let provided = seed_compile_ast_source("trait Printable:\n    fn render(self) -> text:\n        return \"trait-render\"\nclass Report with Printable:\n    fn name(self) -> text:\n        return \"report\"\nlet item = Report()\nsay item.render()", "provided.zp")
let selected = seed_compile_ast_source("trait JsonView:\n    fn render(self) -> text:\n        return \"json\"\ntrait TableView:\n    fn render(self) -> text:\n        return \"table\"\nclass Report with JsonView, TableView:\n    use JsonView.render as render\nlet item = Report()\nsay item.render()", "selected.zp")
let conflict = seed_compile_ast_source("trait JsonView:\n    fn render(self) -> text:\n        return \"json\"\ntrait TableView:\n    fn render(self) -> text:\n        return \"table\"\nclass Report with JsonView, TableView:\n    fn name(self):\n        return \"report\"\nlet item = Report()\nsay item.render()", "conflict.zp")
let provided_state = vm_run(provided["instructions"])
let selected_state = vm_run(selected["instructions"])
let conflict_state = vm_run(conflict["instructions"])
say provided["status"]
say provided_state["error"]
say provided_state["output"][0]
say selected_state["error"]
say selected_state["output"][0]
say conflict_state["error"]
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["compiled_ast_slice", "none", "trait-render", "none", "json", "trait_conflict:Report:render"]:
    raise SystemExit(f"unexpected trait VM output: {lines!r}")
PY
printf 'B4 trait VM gate passed: provided dispatch, explicit selection, and deterministic conflict\n'
