#!/usr/bin/env bash
# Zap B2 typed-IR benchmark runner with cross-platform baseline support.
#
# Runs the B2 candidate and B2 owned typed-IR gates a configurable number of
# times, captures wall-clock elapsed seconds and peak resident-set-size
# (RSS) in kilobytes for each run, and writes a stable raw CSV together
# with a provenance sidecar describing the recorded environment.
#
# Elapsed time and peak RSS are intentionally machine-dependent. They are
# intended for repeated measurement on the same runner with the same
# toolchain. Cross-platform comparison is recorded as a per-target baseline
# table but is NOT used to make portability or speed claims.
#
# Environment variables (all optional):
#   ZAP_TYPED_IR_BENCH_REPEATS    bounded positive integer in [1, 32]
#   ZAP_TYPED_IR_BENCH_WARMUPS    non-negative integer in [0, 8]
#   ZAP_TYPED_IR_BENCH_OUTPUT     raw CSV path (default: benchmark-results/b2-typed-ir.csv)
#   ZAP_TYPED_IR_BENCH_PROVENANCE provenance TSV path (default: <output>.provenance.tsv)
#   ZAP_TYPED_IR_BENCH_BASELINE   cross-platform baseline TSV path
#                                 (default: benchmark-results/b2-typed-ir.baseline.tsv)
#   ZAP_TYPED_IR_BENCH_TIME_CMD   override timing command (for testing/portability)

set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
REPEATS=${ZAP_TYPED_IR_BENCH_REPEATS:-5}
WARMUPS=${ZAP_TYPED_IR_BENCH_WARMUPS:-1}
OUT_FILE=${ZAP_TYPED_IR_BENCH_OUTPUT:-"$ROOT_DIR/benchmark-results/b2-typed-ir.csv"}
PROVENANCE_FILE=${ZAP_TYPED_IR_BENCH_PROVENANCE:-"$OUT_FILE.provenance.tsv"}
BASELINE_FILE=${ZAP_TYPED_IR_BENCH_BASELINE:-"$ROOT_DIR/benchmark-results/b2-typed-ir.baseline.tsv"}

[[ "$REPEATS" =~ ^[1-9][0-9]*$ ]] || { printf 'ZAP_TYPED_IR_BENCH_REPEATS must be a positive integer\n' >&2; exit 2; }
[[ "$WARMUPS" =~ ^[0-9]+$ ]] || { printf 'ZAP_TYPED_IR_BENCH_WARMUPS must be a non-negative integer\n' >&2; exit 2; }
(( REPEATS <= 32 )) || { printf 'ZAP_TYPED_IR_BENCH_REPEATS must not exceed 32\n' >&2; exit 2; }
(( WARMUPS <= 8 )) || { printf 'ZAP_TYPED_IR_BENCH_WARMUPS must not exceed 8\n' >&2; exit 2; }

mkdir -p "$(dirname "$OUT_FILE")"
mkdir -p "$(dirname "$PROVENANCE_FILE")"
mkdir -p "$(dirname "$BASELINE_FILE")"

BIN="$ROOT_DIR/native/target/release/zap"
if [[ ! -x "$BIN" && -x "$BIN.exe" ]]; then
  BIN="$BIN.exe"
fi
if [[ ! -x "$BIN" ]]; then
  cargo build --manifest-path "$ROOT_DIR/native/Cargo.toml" --release --locked
fi

# Resolve a timing command that emits "<elapsed_seconds> <peak_rss_kb>" on
# stdout. GNU time is `/usr/bin/time` on most Linux distros and `gtime` on
# macOS via Homebrew. Windows Git Bash typically lacks both; we fall back
# to bash SECONDS + a process-RSS sample, recording the fallback so the
# report makes the lower fidelity explicit.
TIME_CMD_OVERRIDE="${ZAP_TYPED_IR_BENCH_TIME_CMD:-}"
TIME_BACKEND=""

if [[ -n "$TIME_CMD_OVERRIDE" ]]; then
  TIME_BACKEND="override"
  # The override command is expected to print "<elapsed_seconds> <peak_rss_kb>"
  # to stdout. We use bash to execute it (so the call supports shell
  # metacharacters) and then read its captured stdout from the metrics
  # file. The empty preamble is intentional; see the measure_run branch.
  TIME_PREAMBLE=()
elif [[ -x /usr/bin/time ]]; then
  TIME_BACKEND="gnu-time"
  TIME_PREAMBLE=(/usr/bin/time -f '%e %M' -o)
elif command -v gtime >/dev/null 2>&1; then
  TIME_BACKEND="gtime"
  TIME_PREAMBLE=(gtime -f '%e %M' -o)
else
  TIME_BACKEND="fallback"
  TIME_PREAMBLE=()
fi

run_gate() {
  case "$1" in
    candidate) bash "$ROOT_DIR/scripts/bootstrap/verify_b2_typed_ir_candidate.sh" >/dev/null ;;
    owned) bash "$ROOT_DIR/scripts/bootstrap/verify_b2_typed_ir_owned_program_38.sh" >/dev/null ;;
    *) printf 'unknown suite: %s\n' "$1" >&2; return 2 ;;
  esac
}

measure_run() {
  local script_path="$1"
  local metrics_file
  # GNU time writes its -o target as the child process. The metrics file
  # must be created in a directory the child can write to; mktemp on
  # /tmp with the default 0600 mode is too restrictive. Use the
  # benchmark-results directory which the script already ensures exists.
  metrics_file="$ROOT_DIR/benchmark-results/.zap-typed-ir-metrics.$$.$RANDOM.txt"
  : > "$metrics_file"
  chmod 0666 "$metrics_file" 2>/dev/null || true
  local elapsed="" rss=""
  case "$TIME_BACKEND" in
    gnu-time|gtime)
      "${TIME_PREAMBLE[@]}" "$metrics_file" bash "$script_path" >/dev/null
      elapsed=$(awk 'NF {print $1; exit}' "$metrics_file")
      rss=$(awk 'NF {print $2; exit}' "$metrics_file")
      ;;
    override)
      # Run the override command through bash and capture its stdout,
      # which is expected to be "<elapsed_seconds> <peak_rss_kb>".
      bash -c "$TIME_CMD_OVERRIDE" > "$metrics_file"
      elapsed=$(awk 'NF {print $1; exit}' "$metrics_file")
      rss=$(awk 'NF {print $2; exit}' "$metrics_file")
      ;;
    fallback)
      local start_kb=0 end_kb=0
      # Best-effort portable RSS sampler; records 0 when /proc is absent
      # (e.g. macOS without ps) so the schema stays valid.
      if [[ -r /proc/$$/status ]]; then
        start_kb=$(awk '/VmHWM/ {print $2}' /proc/$$/status 2>/dev/null || printf 0)
      fi
      local start=$SECONDS
      bash "$script_path" >/dev/null
      local end=$SECONDS
      if [[ -r /proc/$$/status ]]; then
        end_kb=$(awk '/VmHWM/ {print $2}' /proc/$$/status 2>/dev/null || printf 0)
      fi
      elapsed=$(( end - start ))
      if (( end_kb > start_kb )); then rss=$end_kb; else rss=$start_kb; fi
      printf '%s %s\n' "$elapsed" "$rss" > "$metrics_file"
      ;;
  esac
  rm -f "$metrics_file"
  printf '%s %s\n' "$elapsed" "$rss"
}

printf 'suite,iteration,elapsed_seconds,peak_rss_kb\n' > "$OUT_FILE"
for suite in candidate owned; do
  for _ in $(seq 1 "$WARMUPS"); do run_gate "$suite"; done
  for iteration in $(seq 1 "$REPEATS"); do
    case "$suite" in
      candidate) measured="$ROOT_DIR/scripts/bootstrap/verify_b2_typed_ir_candidate.sh" ;;
      owned) measured="$ROOT_DIR/scripts/bootstrap/verify_b2_typed_ir_owned_program_38.sh" ;;
    esac
    read -r elapsed rss < <(measure_run "$measured")
    [[ "$elapsed" =~ ^[0-9]+([.][0-9]+)?$ ]] || { printf 'invalid elapsed metric: %s\n' "$elapsed" >&2; exit 1; }
    [[ "$rss" =~ ^[0-9]+$ ]] || { printf 'invalid peak RSS metric: %s\n' "$rss" >&2; exit 1; }
    printf '%s,%s,%s,%s\n' "$suite" "$iteration" "$elapsed" "$rss" | tee -a "$OUT_FILE"
  done
done

# Provenance sidecar (mirrors the M2-BENCH-01 schema fields used by the
# native benchmark so the cross-platform baseline and the per-target
# build matrix can be compared with the same toolchain/commit/binary
# provenance expectations).
commit=$(git -C "$ROOT_DIR" rev-parse HEAD 2>/dev/null || printf unknown)
target=$(rustc -vV 2>/dev/null | awk '/^host:/ {print $2; exit} { exit 1 }' || true)
target=${target:-$(uname -m)-unknown-$(uname -s | tr '[:upper:]' '[:lower:]')}
os=$(uname -s)
kernel=$(uname -r)
arch=$(uname -m)
rust_version=$(rustc --version 2>/dev/null | head -n 1 || printf unknown)
cargo_version=$(cargo --version 2>/dev/null | head -n 1 || printf unknown)
if [[ -x "$BIN" ]]; then
  binary_sha256=$(sha256sum "$BIN" 2>/dev/null | awk '{print $1}')
  if [[ -z "$binary_sha256" ]] && command -v shasum >/dev/null 2>&1; then
    binary_sha256=$(shasum -a 256 "$BIN" 2>/dev/null | awk '{print $1}')
  fi
else
  binary_sha256=missing
fi
script_sha256=$(sha256sum "$0" 2>/dev/null | awk '{print $1}')
if [[ -z "$script_sha256" ]] && command -v shasum >/dev/null 2>&1; then
  script_sha256=$(shasum -a 256 "$0" 2>/dev/null | awk '{print $1}')
fi
timestamp=$(date -u +%Y-%m-%dT%H:%M:%SZ 2>/dev/null || printf unknown)
if [[ "$timestamp" == unknown ]] && command -v gdate >/dev/null 2>&1; then
  timestamp=$(gdate -u +%Y-%m-%dT%H:%M:%SZ 2>/dev/null || printf unknown)
fi

: > "$PROVENANCE_FILE"
printf 'schema_version\t1\n' >> "$PROVENANCE_FILE"
printf 'status\tpassed\n' >> "$PROVENANCE_FILE"
printf 'timestamp_utc\t%s\n' "$timestamp" >> "$PROVENANCE_FILE"
printf 'git_commit\t%s\n' "$commit" >> "$PROVENANCE_FILE"
printf 'target_triple\t%s\n' "$target" >> "$PROVENANCE_FILE"
printf 'os\t%s\n' "$os" >> "$PROVENANCE_FILE"
printf 'kernel\t%s\n' "$kernel" >> "$PROVENANCE_FILE"
printf 'arch\t%s\n' "$arch" >> "$PROVENANCE_FILE"
printf 'rust_version\t%s\n' "$rust_version" >> "$PROVENANCE_FILE"
printf 'cargo_version\t%s\n' "$cargo_version" >> "$PROVENANCE_FILE"
printf 'binary_sha256\t%s\n' "$binary_sha256" >> "$PROVENANCE_FILE"
printf 'script_sha256\t%s\n' "$script_sha256" >> "$PROVENANCE_FILE"
printf 'repeats\t%s\n' "$REPEATS" >> "$PROVENANCE_FILE"
printf 'warmups\t%s\n' "$WARMUPS" >> "$PROVENANCE_FILE"
printf 'suites\tcandidate,owned\n' >> "$PROVENANCE_FILE"
printf 'time_backend\t%s\n' "$TIME_BACKEND" >> "$PROVENANCE_FILE"
printf 'raw_csv\t%s\n' "$OUT_FILE" >> "$PROVENANCE_FILE"

# Cross-platform baseline table. This is a per-target, per-suite
# record of the most recent run that successfully emitted a provenance
# sidecar. Cross-target comparisons are recorded as evidence that the
# runner executed on each target; they are NOT machine-independent
# performance claims.
if [[ ! -s "$BASELINE_FILE" ]]; then
  printf 'target_triple\tsuite\tmin_seconds\tmean_seconds\tmax_seconds\tpeak_rss_kb_min\tpeak_rss_kb_max\tgit_commit\tbinary_sha256\ttimestamp_utc\n' > "$BASELINE_FILE"
fi

awk -F, -v target="$target" -v commit="$commit" -v sha="$binary_sha256" -v ts="$timestamp" '
  NR == 1 { next }
  {
    suite = $1
    elapsed = $3 + 0
    rss = $4 + 0
    if (!(suite in min_e) || elapsed < min_e[suite]) min_e[suite] = elapsed
    if (!(suite in max_e) || elapsed > max_e[suite]) max_e[suite] = elapsed
    sum_e[suite] += elapsed
    cnt[suite]++
    if (!(suite in min_r) || rss < min_r[suite]) min_r[suite] = rss
    if (!(suite in max_r) || rss > max_r[suite]) max_r[suite] = rss
  }
  END {
    for (suite in cnt) {
      mean = sum_e[suite] / cnt[suite]
      printf "%s\t%s\t%.6f\t%.6f\t%.6f\t%d\t%d\t%s\t%s\t%s\n", target, suite, min_e[suite], mean, max_e[suite], min_r[suite], max_r[suite], commit, sha, ts
    }
  }
' "$OUT_FILE" | sort -t$'\t' -k1,1 -k2,2 >> "$BASELINE_FILE"

printf 'B2 typed-IR benchmark written to %s (provenance %s, baseline %s)\n' "$OUT_FILE" "$PROVENANCE_FILE" "$BASELINE_FILE" >&2
