#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"

if [[ "${1:-}" == "--release" ]]; then
  PROFILE=release
  shift
else
  PROFILE=debug
fi

if (($# > 0)); then
  printf 'usage: %s [--release]\n' "$0" >&2
  exit 2
fi

run_zap() {
  if [[ -x "$ROOT_DIR/bin/zap" ]]; then
    "$ROOT_DIR/bin/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
    "$ROOT_DIR/native/target/release/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap" ]]; then
    "$ROOT_DIR/native/target/debug/zap" "$@"
  elif [[ "$PROFILE" == release ]]; then
    cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$@"
  else
    cargo run --quiet --locked --manifest-path native/Cargo.toml -- "$@"
  fi
}

tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/zap-b0-artifacts.XXXXXX")
cleanup() {
  rm -rf "$tmp_dir"
}
#trap cleanup EXIT

run_zap bootstrap status > "$tmp_dir/status.json"
run_zap bootstrap tokens bootstrap/fixtures/lexer/basic.zp > "$tmp_dir/basic.tokens.json"
run_zap bootstrap tokens bootstrap/fixtures/lexer/unicode.zp > "$tmp_dir/unicode.tokens.json"
run_zap bootstrap ast bootstrap/fixtures/lexer/basic.zp > "$tmp_dir/basic.ast.json"
run_zap bootstrap typed-ir bootstrap/fixtures/lexer/basic.zp > "$tmp_dir/basic.typed-ir.json"
run_zap bootstrap diagnostics bootstrap/fixtures/diagnostics/invalid_character.zp > "$tmp_dir/invalid.json"
run_zap bootstrap diagnostics bootstrap/fixtures/lexer/basic.zp > "$tmp_dir/valid.json"

for file in "$tmp_dir"/*.json; do
  python3 -m json.tool "$file" >/dev/null
done

cmp "$tmp_dir/status.json" <(run_zap bootstrap status)
cmp "$tmp_dir/basic.tokens.json" bootstrap/fixtures/lexer/basic.tokens.json
cmp "$tmp_dir/unicode.tokens.json" bootstrap/fixtures/lexer/unicode.tokens.json
cmp "$tmp_dir/basic.ast.json" bootstrap/fixtures/lexer/basic.ast.json
cmp "$tmp_dir/basic.typed-ir.json" bootstrap/fixtures/lexer/basic.typed-ir.json
cmp "$tmp_dir/invalid.json" bootstrap/fixtures/diagnostics/invalid_character.json
cmp "$tmp_dir/valid.json" bootstrap/fixtures/diagnostics/valid.json

python3 - "$ROOT_DIR" <<'PY'
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
required = {
    "bootstrap/fixtures/metadata/b0_stage.json": {"bootstrap_stage", "compiler_version", "language_version", "reference_owner", "self_hosted", "stdlib_version"},
    "bootstrap/fixtures/metadata/artifact_schema.json": {"artifact_schema_version", "schemas"},
    "bootstrap/fixtures/metadata/platform_seed.json": {"capabilities", "platform_seed_version", "status"},
    "bootstrap/fixtures/stdlib/catalog.json": {"bootstrap_stage", "catalog_schema_version", "entries"},
    "bootstrap/fixtures/lexer/basic.typed-ir.json": {"ir", "kind", "reference_only", "schema_version", "source_name"},
}
for relative, keys in required.items():
    value = json.loads((root / relative).read_text(encoding="utf-8"))
    missing = keys - value.keys()
    if missing:
        raise SystemExit(f"{relative}: missing keys: {sorted(missing)}")

status = json.loads((root / "bootstrap/fixtures/metadata/b0_stage.json").read_text(encoding="utf-8"))
if status["bootstrap_stage"] != "B0" or status["self_hosted"] is not False:
    raise SystemExit("B0 metadata must remain explicitly non-self-hosted")

typed_ir = json.loads((root / "bootstrap/fixtures/lexer/basic.typed-ir.json").read_text(encoding="utf-8"))
if typed_ir["reference_only"] is not True or typed_ir["schema_version"] != 1:
    raise SystemExit("typed IR must remain an explicitly reference-only schema-1 artifact")

entries = json.loads((root / "bootstrap/fixtures/stdlib/catalog.json").read_text(encoding="utf-8"))["entries"]
identities = [(entry["domain"], entry["name"]) for entry in entries]
if identities != sorted(identities) or len(identities) != len(set(identities)):
    raise SystemExit("stdlib catalog entries must be unique and sorted by domain/name")
PY

printf 'B0 bootstrap artifact verification passed: status, token, AST, typed IR, diagnostics, metadata, and stdlib catalog\n'
