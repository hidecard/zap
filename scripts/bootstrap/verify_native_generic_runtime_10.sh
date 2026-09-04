#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
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
runner=$(mktemp "$ROOT_DIR/.zap-native-generic.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'ZAP'
fn identity<T>(value: T) -> T:
    return value
fn wrap<T>(value: T) -> option<T>:
    return some(value)
fn peel<T>(items: list<T>, index: number) -> option<T>:
    if index == 0:
        return some(items[0])
    return peel(items, index - 1)
let numbers = identity([1, 2, 3])
let wrapped = wrap("zap")
let recursive = peel(numbers, 2)
say numbers
say wrapped
say recursive
say unwrap(wrapped)
say unwrap(recursive)
ZAP
cat > "$expected" <<'OUT'
[1, 2, 3]
Some(zap)
Some(1)
zap
1
OUT
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'Native generic runtime gate passed: evaluator containers and recursive generic calls\n'
