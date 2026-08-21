#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
NATIVE_DIR="$ROOT_DIR/native"
REPEATS=${ZAP_BENCH_REPEATS:-5}
OUT_FILE=${ZAP_BENCH_OUTPUT:-"$ROOT_DIR/benchmark-results/native.csv"}

if ! [[ "$REPEATS" =~ ^[1-9][0-9]*$ ]]; then
  printf 'ZAP_BENCH_REPEATS must be a positive integer\n' >&2
  exit 2
fi

mkdir -p "$(dirname "$OUT_FILE")"
WORK_DIR=$(mktemp -d "${TMPDIR:-/tmp}/zap-bench.XXXXXX")
cleanup() { rm -rf "$WORK_DIR"; }
trap cleanup EXIT

cat >"$WORK_DIR/loops.zp" <<'ZAP'
let total = 0
let index = 0
while index < 10000:
    total = total + index
    index = index + 1
say total
ZAP

cat >"$WORK_DIR/calls.zp" <<'ZAP'
fn square(value):
    return value * value

let total = 0
let index = 0
while index < 2000:
    total = total + square(index)
    index = index + 1
say total
ZAP

cat >"$WORK_DIR/closures.zp" <<'ZAP'
fn run_counter():
    let count = 0
    fn increment():
        count = count + 1
        return count
    let total = 0
    let index = 0
    while index < 2000:
        total = total + increment()
        index = index + 1
    return total

say run_counter()
ZAP

cat >"$WORK_DIR/allocations.zp" <<'ZAP'
let values = range(10000)
let indexed = enumerate(values)
say len(indexed)
ZAP

cat >"$WORK_DIR/json.zp" <<'ZAP'
let payload = range(1000)
let encoded = json(payload)
let decoded = from_json(encoded)
say len(decoded)
ZAP

cat >"$WORK_DIR/async.zp" <<'ZAP'
async fn load():
    return 7
let handle = spawn(load())
let ready: bool = task_is_ready(handle)
let result: number = task_join(handle)
say result
ZAP

mkdir -p "$WORK_DIR/imports/modules/app"
cat >"$WORK_DIR/imports/zap.toml" <<'ZAP'
[package]
name = "benchmark-imports"
version = "0.1.0"
main = "main.zp"

[module]
root = "modules"
entries = ["app/core.zp", "app/util.zp"]
ZAP
cat >"$WORK_DIR/imports/main.zp" <<'ZAP'
module app.main
import app.core as core
import app.util as util
ZAP
cat >"$WORK_DIR/imports/modules/app/core.zp" <<'ZAP'
module app.core
import app.util as util
ZAP
cat >"$WORK_DIR/imports/modules/app/util.zp" <<'ZAP'
module app.util
ZAP

BIN="$NATIVE_DIR/target/release/zap"
if [[ ! -x "$BIN" ]]; then
  cargo build --manifest-path "$NATIVE_DIR/Cargo.toml" --release --locked
fi

printf 'suite,iteration,elapsed_seconds\n' >"$OUT_FILE"
printf '# zap benchmark suite\n# binary=%s\n# repeats=%s\n' "$BIN" "$REPEATS" >&2

for fixture in loops calls closures allocations json async imports; do
  for iteration in $(seq 1 "$REPEATS"); do
    elapsed=$(
      TIMEFORMAT='%R'
      if [[ "$fixture" == "imports" ]]; then
        { time "$BIN" check "$WORK_DIR/imports" >/dev/null; } 2>&1
      else
        { time "$BIN" run "$WORK_DIR/$fixture.zp" >/dev/null; } 2>&1
      fi
    )
    if ! [[ "$elapsed" =~ ^[0-9]+([.][0-9]+)?$ ]]; then
      printf 'benchmark %s iteration %s returned invalid timing: %s\n' "$fixture" "$iteration" "$elapsed" >&2
      exit 1
    fi
    printf '%s,%s,%s\n' "$fixture" "$iteration" "$elapsed" | tee -a "$OUT_FILE"
  done
done

printf 'wrote %s\n' "$OUT_FILE" >&2
