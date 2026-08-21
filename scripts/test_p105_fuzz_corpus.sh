#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
MANIFEST="$ROOT_DIR/native/Cargo.toml"
BINARY="$ROOT_DIR/native/target/debug/zap"
WORK_DIR=$(mktemp -d)
trap 'rm -rf "$WORK_DIR"' EXIT

if [[ ! -x "$BINARY" ]]; then
  cargo build --manifest-path "$MANIFEST" --bin zap
fi

run_reject_case() {
  local name="$1"
  local source="$2"
  local file="$WORK_DIR/${name}.zp"
  local output="$WORK_DIR/${name}.out"
  printf '%s\n' "$source" > "$file"
  if "$BINARY" run "$file" >"$output" 2>&1; then
    printf 'p1-05 fuzz corpus: unexpected acceptance: %s\n' "$name" >&2
    cat "$output" >&2
    return 1
  fi
  if grep -qE 'panicked at|thread .* panicked|stack backtrace' "$output"; then
    printf 'p1-05 fuzz corpus: panic detected: %s\n' "$name" >&2
    cat "$output" >&2
    return 1
  fi
  printf 'p1-05 fuzz corpus: rejected safely: %s\n' "$name"
}

run_reject_case unterminated_string 'let value = "unterminated'
run_reject_case invalid_operator 'let value = 1 @ 2'
run_reject_case unclosed_group 'let value = (1 + 2'
run_reject_case trailing_tokens 'let value = 1 ???'
run_reject_case invalid_function_body 'fn broken( { return 1 }'
run_reject_case malformed_import 'import "../outside"'
run_reject_case invalid_annotation '@'

printf 'p1-05 deterministic fuzz-style CLI corpus passed\n'
