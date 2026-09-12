#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
run_zap() {
  if [[ -x "${ZAP_BOOTSTRAP_BIN:-}" ]]; then
    "$ZAP_BOOTSTRAP_BIN" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
    "$ROOT_DIR/native/target/release/zap" "$@"
  else
    cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$@"
  fi
}
runner=$(mktemp "$ROOT_DIR/.zap-b3-disassembler.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat > "$runner" <<'ZP'
import "bootstrap/b2/typed_ir.zp"
import "bootstrap/b3/lower.zp"
import "bootstrap/b3/disassembler.zp"
let typed = from_json(emit("say 2 + 3 * 4", "disassembler.zp"))
let bytecode = lower_typed_ir(typed)
let first = disassemble_bytecode(bytecode)
let second = disassemble_bytecode(bytecode)
let invalid = disassemble_bytecode({"instructions": [], "kind": "wrong.kind", "schema_version": 1})
say first["kind"]
say first["schema_version"]
say first["status"]
say first["instruction_count"]
say first["lines"][0]["index"]
say first["lines"][0]["text"]
say json(first) == json(second)
say invalid["status"]
say invalid["diagnostics"][0]["code"]
ZP
run_zap "$(basename "$runner")" > "$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if len(lines) != 9:
    raise SystemExit(f"unexpected disassembler output: {lines!r}")
if lines[0:5] != ["zap.disassembly", "1", "ok", lines[3], "0"]:
    raise SystemExit(f"unexpected disassembly header: {lines!r}")
if lines[5] == "":
    raise SystemExit("empty instruction rendering")
if lines[6:] != ["true", "error", "ZAP-BYTECODE-001"]:
    raise SystemExit(f"unexpected determinism/error output: {lines!r}")
PY
printf 'B3 bytecode disassembler gate passed: schema, stable instruction order, deterministic rendering, and invalid-input diagnostics\n'
