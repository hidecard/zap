#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
if ! command -v rustc >/dev/null 2>&1 && [[ -f "$HOME/.cargo/env" ]]; then
  # Keep local contract tests usable without requiring callers to pre-source Rust.
  # CI toolchain actions already place the pinned tools on PATH.
  source "$HOME/.cargo/env"
fi
NATIVE_DIR="$ROOT_DIR/native"
REPEATS=${ZAP_BENCH_REPEATS:-5}
WARMUPS=${ZAP_BENCH_WARMUPS:-1}
OUT_FILE=${ZAP_BENCH_OUTPUT:-"$ROOT_DIR/benchmark-results/native.csv"}
PROVENANCE_FILE=${ZAP_BENCH_PROVENANCE:-"${OUT_FILE%.csv}.provenance.tsv"}

if ! [[ "$REPEATS" =~ ^[1-9][0-9]*$ ]]; then
  printf 'ZAP_BENCH_REPEATS must be a positive integer\n' >&2
  exit 2
fi
if ! [[ "$WARMUPS" =~ ^[0-9]+$ ]]; then
  printf 'ZAP_BENCH_WARMUPS must be a non-negative integer\n' >&2
  exit 2
fi
if (( REPEATS > 64 )); then
  printf 'ZAP_BENCH_REPEATS must not exceed 64\n' >&2
  exit 2
fi
if (( WARMUPS > 16 )); then
  printf 'ZAP_BENCH_WARMUPS must not exceed 16\n' >&2
  exit 2
fi

mkdir -p "$(dirname "$OUT_FILE")" "$(dirname "$PROVENANCE_FILE")"
WORK_DIR=$(mktemp -d "${TMPDIR:-/tmp}/zap-bench.XXXXXX")
BENCH_STATUS=failed
PROVENANCE_READY=0
cleanup() {
  if [[ "$PROVENANCE_READY" == 1 ]]; then
    write_provenance "$BENCH_STATUS"
  fi
  rm -rf "$WORK_DIR"
}
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

sha256_file() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

sanitize_value() {
  printf '%s' "$1" | tr '\r\n\t' '   '
}

host_target=$(rustc -vV | sed -n 's/^host: //p')
runner_os=$(uname -s)
kernel=$(uname -r)
architecture=$(uname -m)
os_release="unknown"
if [[ -r /etc/os-release ]]; then
  os_release=$(awk -F= '$1 == "PRETTY_NAME" {gsub(/"/, "", $2); print $2; exit}' /etc/os-release)
elif command -v sw_vers >/dev/null 2>&1; then
  os_release="macOS $(sw_vers -productVersion)"
elif [[ -n "${PROCESSOR_IDENTIFIER:-}" ]]; then
  os_release="Windows"
fi
cpu_model="${PROCESSOR_IDENTIFIER:-unknown}"
if [[ -r /proc/cpuinfo ]]; then
  cpu_model=$(awk -F: '/^model name[[:space:]]*:/ {sub(/^[[:space:]]+/, "", $2); print $2; exit}' /proc/cpuinfo)
elif command -v sysctl >/dev/null 2>&1; then
  cpu_model=$(sysctl -n machdep.cpu.brand_string 2>/dev/null || sysctl -n hw.model 2>/dev/null || printf '%s' "$cpu_model")
fi
cpu_model=$(sanitize_value "$cpu_model")
os_release=$(sanitize_value "$os_release")
git_commit=$(git -C "$ROOT_DIR" rev-parse HEAD 2>/dev/null || printf 'unknown')
rust_version=$(rustc --version)
cargo_version=$(cargo --version)
binary_sha256=$(sha256_file "$BIN")
benchmark_script_sha256=$(sha256_file "$ROOT_DIR/scripts/benchmark_native.sh")

write_provenance() {
  local status="$1"
  local temporary="${PROVENANCE_FILE}.tmp"
  {
    printf 'field\tvalue\n'
    printf 'schema_version\t1\n'
    printf 'status\t%s\n' "$status"
    printf 'generated_at_utc\t%s\n' "$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
    printf 'git_commit\t%s\n' "$git_commit"
    printf 'target_triple\t%s\n' "$host_target"
    printf 'runner_os\t%s\n' "$(sanitize_value "$runner_os")"
    printf 'os_release\t%s\n' "$os_release"
    printf 'kernel\t%s\n' "$(sanitize_value "$kernel")"
    printf 'architecture\t%s\n' "$(sanitize_value "$architecture")"
    printf 'cpu_model\t%s\n' "$cpu_model"
    printf 'rustc\t%s\n' "$rust_version"
    printf 'cargo\t%s\n' "$cargo_version"
    printf 'binary\t%s\n' "$BIN"
    printf 'binary_sha256\t%s\n' "$binary_sha256"
    printf 'benchmark_script_sha256\t%s\n' "$benchmark_script_sha256"
    printf 'repeats\t%s\n' "$REPEATS"
    printf 'warmups\t%s\n' "$WARMUPS"
    printf 'suites\tloops,calls,closures,allocations,json,async,imports\n'
    printf 'raw_observations\t%s\n' "$OUT_FILE"
  } >"$temporary"
  mv "$temporary" "$PROVENANCE_FILE"
}

PROVENANCE_READY=1
printf 'suite,iteration,elapsed_seconds\n' >"$OUT_FILE"
printf '# zap benchmark suite\n# binary=%s\n# repeats=%s\n# warmups=%s\n# provenance=%s\n# target=%s\n' "$BIN" "$REPEATS" "$WARMUPS" "$PROVENANCE_FILE" "$host_target" >&2
write_provenance running

run_fixture() {
  local fixture="$1"
  if [[ "$fixture" == "imports" ]]; then
    "$BIN" check "$WORK_DIR/imports" >/dev/null
  else
    "$BIN" run "$WORK_DIR/$fixture.zp" >/dev/null
  fi
}

for fixture in loops calls closures allocations json async imports; do
  for _ in $(seq 1 "$WARMUPS"); do
    run_fixture "$fixture"
  done
  for iteration in $(seq 1 "$REPEATS"); do
    elapsed=$(
      TIMEFORMAT='%R'
      { time run_fixture "$fixture"; } 2>&1
    )
    if ! [[ "$elapsed" =~ ^[0-9]+([.][0-9]+)?$ ]]; then
      printf 'benchmark %s iteration %s returned invalid timing: %s\n' "$fixture" "$iteration" "$elapsed" >&2
      exit 1
    fi
    printf '%s,%s,%s\n' "$fixture" "$iteration" "$elapsed" | tee -a "$OUT_FILE"
  done

done

BENCH_STATUS=passed
printf 'wrote %s\n' "$OUT_FILE" >&2
printf 'wrote %s\n' "$PROVENANCE_FILE" >&2
