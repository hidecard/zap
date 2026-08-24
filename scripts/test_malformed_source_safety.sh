#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT_DIR"

ZAP_BIN="${ZAP_BIN:-$ROOT_DIR/native/target/release/zap}"
work_root=$(mktemp -d "${TMPDIR:-/tmp}/zap-malformed-source.XXXXXX")
trap 'rm -rf "$work_root"' EXIT

if [[ ! -x "$ZAP_BIN" ]]; then
  cargo build --quiet --release --locked --manifest-path native/Cargo.toml --bin zap
fi
[[ -x "$ZAP_BIN" ]] || { printf 'malformed-source safety: missing executable: %s\n' "$ZAP_BIN" >&2; exit 2; }

cases=(
  malformed_generic
  unknown_annotation
  incompatible_annotation
)

write_case() {
  local name="$1"
  local project="$work_root/$name"
  mkdir -p "$project"
  printf '[package]\nname = "%s"\nversion = "0.1.0"\nmain = "main.zp"\n' "$name" > "$project/zap.toml"
  case "$name" in
    malformed_generic)
      printf 'let values: list<number = [1]\n' > "$project/main.zp"
      ;;
    unknown_annotation)
      printf 'let value: mystery = 1\n' > "$project/main.zp"
      ;;
    incompatible_annotation)
      printf 'let value: text = 1\n' > "$project/main.zp"
      ;;
    *)
      printf 'unknown malformed-source case: %s\n' "$name" >&2
      return 2
      ;;
  esac
}

for name in "${cases[@]}"; do
  write_case "$name"
  project="$work_root/$name"
  output="$work_root/$name.output"
  set +e
  timeout 30s "$ZAP_BIN" check --json "$project" >"$output" 2>&1
  status=$?
  set -e
  if [[ "$status" -eq 0 ]]; then
    printf 'malformed-source case unexpectedly passed: %s\n' "$name" >&2
    cat "$output" >&2
    exit 1
  fi
  if [[ "$status" -eq 124 ]]; then
    printf 'malformed-source case timed out: %s\n' "$name" >&2
    exit 1
  fi
  if grep -Eiq "panicked at|thread '.*' panicked|called .*unwrap|called .*expect|stack backtrace" "$output"; then
    printf 'panic/unchecked-failure signature found for malformed-source case: %s\n' "$name" >&2
    cat "$output" >&2
    exit 1
  fi
  printf 'malformed-source safety passed: %s (status=%s)\n' "$name" "$status"
done

printf 'malformed-source safety contract passed: cases=%s\n' "${#cases[@]}"
