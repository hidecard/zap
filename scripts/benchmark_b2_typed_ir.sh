#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
REPEATS=${ZAP_TYPED_IR_BENCH_REPEATS:-5}
WARMUPS=${ZAP_TYPED_IR_BENCH_WARMUPS:-1}
OUT_FILE=${ZAP_TYPED_IR_BENCH_OUTPUT:-"$ROOT_DIR/benchmark-results/b2-typed-ir.csv"}

[[ "$REPEATS" =~ ^[1-9][0-9]*$ ]] || { printf 'ZAP_TYPED_IR_BENCH_REPEATS must be a positive integer\n' >&2; exit 2; }
[[ "$WARMUPS" =~ ^[0-9]+$ ]] || { printf 'ZAP_TYPED_IR_BENCH_WARMUPS must be a non-negative integer\n' >&2; exit 2; }
(( REPEATS <= 32 )) || { printf 'ZAP_TYPED_IR_BENCH_REPEATS must not exceed 32\n' >&2; exit 2; }
(( WARMUPS <= 8 )) || { printf 'ZAP_TYPED_IR_BENCH_WARMUPS must not exceed 8\n' >&2; exit 2; }

mkdir -p "$(dirname "$OUT_FILE")"
BIN="$ROOT_DIR/native/target/release/zap"
if [[ ! -x "$BIN" ]]; then
  cargo build --manifest-path "$ROOT_DIR/native/Cargo.toml" --release --locked
fi

run_gate() {
  case "$1" in
    candidate) bash "$ROOT_DIR/scripts/bootstrap/verify_b2_typed_ir_candidate.sh" >/dev/null ;;
    owned) bash "$ROOT_DIR/scripts/bootstrap/verify_b2_typed_ir_owned_program_38.sh" >/dev/null ;;
    *) printf 'unknown suite: %s\n' "$1" >&2; return 2 ;;
  esac
}

printf 'suite,iteration,elapsed_seconds,peak_rss_kb\n' > "$OUT_FILE"
for suite in candidate owned; do
  for _ in $(seq 1 "$WARMUPS"); do run_gate "$suite"; done
  for iteration in $(seq 1 "$REPEATS"); do
    metrics_file=$(mktemp "${TMPDIR:-/tmp}/zap-typed-ir-metrics.XXXXXX")
    case "$suite" in
      candidate) measured="$ROOT_DIR/scripts/bootstrap/verify_b2_typed_ir_candidate.sh" ;;
      owned) measured="$ROOT_DIR/scripts/bootstrap/verify_b2_typed_ir_owned_program_38.sh" ;;
    esac
    /usr/bin/time -f '%e %M' -o "$metrics_file" bash "$measured" >/dev/null
    elapsed=$(awk 'NF {print $1; exit}' "$metrics_file")
    rss=$(awk 'NF {print $2; exit}' "$metrics_file")
    rm -f "$metrics_file"
    [[ "$elapsed" =~ ^[0-9]+([.][0-9]+)?$ ]] || { printf 'invalid elapsed metric: %s\n' "$elapsed" >&2; exit 1; }
    [[ "$rss" =~ ^[0-9]+$ ]] || { printf 'invalid peak RSS metric: %s\n' "$rss" >&2; exit 1; }
    printf '%s,%s,%s,%s\n' "$suite" "$iteration" "$elapsed" "$rss" | tee -a "$OUT_FILE"
  done
done
printf 'B2 typed-IR benchmark written to %s\n' "$OUT_FILE" >&2
