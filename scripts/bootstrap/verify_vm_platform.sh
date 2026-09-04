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

if (($# > 0)); then
  printf 'usage: %s\n' "$0" >&2
  exit 2
fi

actual=$(mktemp "${TMPDIR:-/tmp}/zap-vm-platform.XXXXXX.json")
cleanup() {
  rm -f "$actual"
}
trap cleanup EXIT

if [[ -x "$ROOT_DIR/bin/zap" ]]; then
  ZAP_BIN="$ROOT_DIR/bin/zap"
elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
  ZAP_BIN="$ROOT_DIR/native/target/release/zap"
else
  ZAP_BIN="cargo run --quiet --release --locked --manifest-path native/Cargo.toml --"
fi

"$ZAP_BIN" bootstrap vm-demo > "$actual"
python3 - "$ROOT_DIR" "$actual" <<'PY'
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
actual = pathlib.Path(sys.argv[2])
expected = root / "bootstrap/fixtures/bytecode/vm_demo.json"
if actual.read_bytes() != expected.read_bytes():
    raise SystemExit("reference VM smoke output is not reproducible")

result = json.loads(actual.read_text(encoding="utf-8"))
if result != {"kind": "zap.bytecode_result", "schema_version": 1, "value": 6}:
    raise SystemExit("unexpected reference VM smoke artifact")

seed = json.loads((root / "bootstrap/fixtures/metadata/platform_seed.json").read_text(encoding="utf-8"))
if seed.get("status") != "documented-boundary-only":
    raise SystemExit("platform seed must remain a documented boundary only")
capabilities = seed.get("capabilities")
if not isinstance(capabilities, dict):
    raise SystemExit("platform seed capabilities must be an object")
if capabilities.get("network") != "not available to compiler core":
    raise SystemExit("network must remain unavailable to compiler core")
if capabilities.get("process") != "not available to compiler core":
    raise SystemExit("process must remain unavailable to compiler core")
if capabilities.get("console") != "platform_stdout/stderr":
    raise SystemExit("console boundary must remain explicit")
PY

printf 'VM/platform foundation verification passed: deterministic bytecode smoke, schema, and deny-by-default seed\n'
