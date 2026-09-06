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
trap cleanup EXIT

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

python3 - "$ROOT_DIR" "$tmp_dir" <<'PY'
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
tmp_dir = pathlib.Path(sys.argv[2])

def normalize_source_name(source_name):
    if not isinstance(source_name, str):
        return source_name
    source_name = source_name.replace("\\", "/")
    if source_name.startswith(str(root) + "/"):
        source_name = source_name[len(str(root) + "/"):]
    elif source_name.startswith(str(root) + "\\"):
        source_name = source_name[len(str(root) + "\\"):]
    return source_name

def normalize_paths(obj):
    if isinstance(obj, dict):
        for key, value in obj.items():
            if key == "source_name":
                obj[key] = normalize_source_name(value)
            else:
                normalize_paths(value)
    elif isinstance(obj, list):
        for item in obj:
            normalize_paths(item)

def load_normalized(path):
    data = json.loads(path.read_text(encoding="utf-8"))
    normalize_paths(data)
    return json.dumps(data, ensure_ascii=False, separators=(",", ":"))

# Compare normalized actual outputs with normalized golden files
comparisons = [
    ("status.json", "bootstrap/fixtures/metadata/b0_stage.json", False),
    ("basic.tokens.json", "bootstrap/fixtures/lexer/basic.tokens.json", True),
    ("unicode.tokens.json", "bootstrap/fixtures/lexer/unicode.tokens.json", True),
    ("basic.ast.json", "bootstrap/fixtures/lexer/basic.ast.json", True),
    ("basic.typed-ir.json", "bootstrap/fixtures/lexer/basic.typed-ir.json", True),
    ("invalid.json", "bootstrap/fixtures/diagnostics/invalid_character.json", True),
    ("valid.json", "bootstrap/fixtures/diagnostics/valid.json", True),
]

for actual_name, golden_rel, normalize in comparisons:
    actual_path = tmp_dir / actual_name
    golden_path = root / golden_rel
    
    if not golden_path.exists():
        raise SystemExit(f"missing golden file: {golden_path}")
    
    if normalize:
        actual_data = load_normalized(actual_path)
        golden_data = load_normalized(golden_path)
    else:
        actual_data = actual_path.read_text(encoding="utf-8")
        golden_data = golden_path.read_text(encoding="utf-8")
    
    if actual_data != golden_data:
        import difflib
        actual_lines = actual_data.splitlines(keepends=True)
        golden_lines = golden_data.splitlines(keepends=True)
        diff = list(difflib.unified_diff(golden_lines, actual_lines, fromfile=str(golden_path), tofile=str(actual_path), lineterm=""))
        sys.stderr.write("\n".join(diff) + "\n")
        raise SystemExit(f"mismatch: {actual_path} != {golden_path}")

# Metadata checks
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
