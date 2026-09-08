#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
run_zap() {
  if [[ -x "$ROOT_DIR/bin/zap" || -x "$ROOT_DIR/bin/zap.exe" ]]; then
    if [[ -x "$ROOT_DIR/bin/zap" ]]; then
      "$ROOT_DIR/bin/zap" "$@"
    else
      "$ROOT_DIR/bin/zap.exe" "$@"
    fi
  elif [[ -x "$ROOT_DIR/native/target/release/zap" || -x "$ROOT_DIR/native/target/release/zap.exe" ]]; then
    if [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
      "$ROOT_DIR/native/target/release/zap" "$@"
    else
      "$ROOT_DIR/native/target/release/zap.exe" "$@"
    fi
  elif [[ -x "$ROOT_DIR/native/target/debug/zap" || -x "$ROOT_DIR/native/target/debug/zap.exe" ]]; then
    if [[ -x "$ROOT_DIR/native/target/debug/zap" ]]; then
      "$ROOT_DIR/native/target/debug/zap" "$@"
    else
      "$ROOT_DIR/native/target/debug/zap.exe" "$@"
    fi
  else
    cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$@"
  fi
}

fixtures=(
  "bootstrap/fixtures/parser/compound.zp|bootstrap/fixtures/parser/compound.ast.json|ast"
  "bootstrap/fixtures/diagnostics/missing_closing_bracket.zp|bootstrap/fixtures/diagnostics/missing_closing_bracket.json|diagnostics"
)

run_native() {
  local mode=$1
  local fixture=$2
  if [[ -x "$ROOT_DIR/bin/zap" || -x "$ROOT_DIR/bin/zap.exe" ]]; then
    if [[ -x "$ROOT_DIR/bin/zap" ]]; then
      "$ROOT_DIR/bin/zap" bootstrap "$mode" "$fixture"
    else
      "$ROOT_DIR/bin/zap.exe" bootstrap "$mode" "$fixture"
    fi
  elif [[ -x "$ROOT_DIR/native/target/release/zap" || -x "$ROOT_DIR/native/target/release/zap.exe" ]]; then
    if [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
      "$ROOT_DIR/native/target/release/zap" bootstrap "$mode" "$fixture"
    else
      "$ROOT_DIR/native/target/release/zap.exe" bootstrap "$mode" "$fixture"
    fi
  else
    run_zap bootstrap "$mode" "$fixture"
  fi
}

for entry in "${fixtures[@]}"; do
  IFS='|' read -r fixture expected mode <<<"$entry"
  [[ -f "$fixture" && -f "$expected" ]] || {
    printf 'missing B1 parser fixture or expected artifact: %s\n' "$fixture" >&2
    exit 2
  }
  first=$(mktemp "${TMPDIR:-/tmp}/zap-b1-parser-first.XXXXXX")
  second=$(mktemp "${TMPDIR:-/tmp}/zap-b1-parser-second.XXXXXX")
  normalized_first=$(mktemp "${TMPDIR:-/tmp}/zap-b1-parser-normalized.XXXXXX")
  normalized_expected=$(mktemp "${TMPDIR:-/tmp}/zap-b1-parser-expected-normalized.XXXXXX")
  trap 'rm -f "$first" "$second" "$normalized_first" "$normalized_expected"' EXIT
  run_native "$mode" "$fixture" > "$first"
  run_native "$mode" "$fixture" > "$second"
  cmp "$first" "$second"
  normalized_first=$(mktemp "${TMPDIR:-/tmp}/zap-b1-parser-normalized.XXXXXX")
  normalized_expected=$(mktemp "${TMPDIR:-/tmp}/zap-b1-parser-expected-normalized.XXXXXX")
  python3 - "$first" "$expected" "$normalized_first" "$normalized_expected" <<'PY'
import json
import pathlib
import sys

def normalize_source_name(source_name):
    if not isinstance(source_name, str):
        return source_name
    source_name = source_name.replace("\\", "/")
    if source_name.startswith("/"):
        parts = source_name.split("/")
        if "bootstrap" in parts:
            idx = parts.index("bootstrap")
            source_name = "/".join(parts[idx:])
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

first_path = pathlib.Path(sys.argv[1])
expected_path = pathlib.Path(sys.argv[2])
normalized_first_path = pathlib.Path(sys.argv[3])
normalized_expected_path = pathlib.Path(sys.argv[4])

first_data = json.loads(first_path.read_text(encoding="utf-8-sig"))
expected_data = json.loads(expected_path.read_text(encoding="utf-8-sig"))
normalize_paths(first_data)
normalize_paths(expected_data)
normalized_first_path.write_text(json.dumps(first_data, ensure_ascii=False, separators=(",", ":")), encoding="utf-8")
normalized_expected_path.write_text(json.dumps(expected_data, ensure_ascii=False, separators=(",", ":")), encoding="utf-8")
PY
  cmp "$normalized_first" "$normalized_expected"
  if [[ "$mode" == "ast" ]]; then
    python3 -c "import json,sys; d=json.load(open(sys.argv[1],encoding='utf-8-sig')); sys.exit(0 if d.get('kind')=='zap.ast' and d.get('schema_version')==1 and len(d.get('ast',{}).get('statements',[]))>0 else 1)" "$first" >/dev/null
  else
    python3 -c "import json,sys; d=json.load(open(sys.argv[1],encoding='utf-8-sig')); diags=d.get('diagnostics',[]); sys.exit(0 if d.get('kind')=='zap.diagnostics' and d.get('schema_version')==1 and diags and diags[0].get('code')=='ZAP-SYNTAX-001' else 1)" "$first" >/dev/null
  fi
  rm -f "$first" "$second" "$normalized_first" "$normalized_expected"
  trap - EXIT
  printf 'B1 reference parser differential passed: %s (%s)\n' "$fixture" "$mode"
done
