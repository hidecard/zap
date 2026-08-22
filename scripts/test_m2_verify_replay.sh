#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
MANIFEST="$ROOT_DIR/native/Cargo.toml"
CORPUS_ROOT="$ROOT_DIR/corpus/p1-05"
SEED="${ZAP_CORPUS_SEED:-20260821}"
ROUNDS="${ZAP_CORPUS_ROUNDS:-12}"
MAX_ROUNDS=64
MAX_FIXTURE_BYTES="${ZAP_CORPUS_MAX_FIXTURE_BYTES:-65536}"
MAX_TOTAL_BYTES="${ZAP_CORPUS_MAX_TOTAL_BYTES:-8388608}"
REPORT="${ZAP_BOUNDED_REPLAY_REPORT:-$ROOT_DIR/target/m2-verify-replay.tsv}"
LOG="${ZAP_BOUNDED_REPLAY_LOG:-$ROOT_DIR/target/m2-verify-replay.log}"

fail() {
  printf 'm2-verify replay: %s\n' "$1" >&2
  exit 1
}

if [[ -f "$HOME/.cargo/env" ]]; then
  # shellcheck disable=SC1091
  source "$HOME/.cargo/env"
fi
command -v cargo >/dev/null 2>&1 || fail 'cargo is required'

WORK_DIR=$(mktemp -d)
trap 'rm -rf "$WORK_DIR"' EXIT

positive_decimal() {
  [[ "$1" =~ ^[1-9][0-9]*$ ]]
}

sha256_file() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

positive_decimal "$SEED" || fail 'ZAP_CORPUS_SEED must be a positive decimal integer'
positive_decimal "$ROUNDS" || fail 'ZAP_CORPUS_ROUNDS must be a positive decimal integer'
positive_decimal "$MAX_FIXTURE_BYTES" || fail 'ZAP_CORPUS_MAX_FIXTURE_BYTES must be a positive decimal integer'
positive_decimal "$MAX_TOTAL_BYTES" || fail 'ZAP_CORPUS_MAX_TOTAL_BYTES must be a positive decimal integer'
(( ROUNDS <= MAX_ROUNDS )) || fail "ZAP_CORPUS_ROUNDS must not exceed $MAX_ROUNDS"
[[ -d "$CORPUS_ROOT" ]] || fail "missing corpus directory: $CORPUS_ROOT"

mkdir -p "$(dirname "$REPORT")" "$(dirname "$LOG")"
printf 'seed\trounds\tmax_rounds\tfixture_count\tfixture_bytes\tmax_fixture_bytes\tmax_total_bytes\tmanifest_sha256\toutcome_sha256s\tstatus\n' > "$REPORT"
: > "$LOG"

manifest="$WORK_DIR/manifest.tsv"
: > "$manifest"
fixture_count=0
fixture_bytes=0
while IFS= read -r fixture; do
  relative=${fixture#"$CORPUS_ROOT/"}
  size=$(wc -c < "$fixture" | tr -d ' ')
  (( size <= MAX_FIXTURE_BYTES )) || fail "fixture exceeds $MAX_FIXTURE_BYTES bytes: $relative"
  fixture_bytes=$((fixture_bytes + size))
  fixture_count=$((fixture_count + 1))
  digest=$(sha256_file "$fixture")
  printf '%s\t%s\t%s\n' "$relative" "$size" "$digest" >> "$manifest"
done < <(find "$CORPUS_ROOT" -type f | LC_ALL=C sort)

(( fixture_count > 0 )) || fail 'corpus contains no fixture files'
(( fixture_bytes <= MAX_TOTAL_BYTES )) || fail "corpus exceeds $MAX_TOTAL_BYTES bytes"
manifest_sha256=$(sha256_file "$manifest")

ZAP_CORPUS_SEED="$SEED" ZAP_CORPUS_ROUNDS="$ROUNDS" cargo test \
  --manifest-path "$MANIFEST" \
  --bin zap \
  --all-features \
  corpus::tests::replayable_failure_corpus_is_seeded_panic_free_and_deterministic \
  -- --exact --nocapture > "$LOG" 2>&1 || {
    cat "$LOG" >&2
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\tfailed\n' \
      "$SEED" "$ROUNDS" "$MAX_ROUNDS" "$fixture_count" "$fixture_bytes" \
      "$MAX_FIXTURE_BYTES" "$MAX_TOTAL_BYTES" "$manifest_sha256" '' >> "$REPORT"
    exit 1
  }

mapfile -t markers < <(grep '^M2_VERIFY_REPLAY ' "$LOG" || true)
if (( ${#markers[@]} != ROUNDS )); then
  cat "$LOG" >&2
  fail "expected $ROUNDS deterministic replay markers, got ${#markers[@]}"
fi

outcome_digests=()
for expected_round in $(seq 1 "$ROUNDS"); do
  marker="${markers[$((expected_round - 1))]}"
  marker_round=$(awk '{print $2}' <<< "$marker")
  marker_seed=$(awk '{print $3}' <<< "$marker")
  marker_cases=$(awk '{print $4}' <<< "$marker")
  marker_digest=$(awk '{print $5}' <<< "$marker")
  [[ "$marker_round" == "round=$expected_round" ]] || fail "replay marker order changed: $marker"
  [[ "$marker_seed" == "seed=$SEED" ]] || fail "replay marker seed changed: $marker"
  [[ "$marker_cases" == "cases=$fixture_count" ]] || fail "replay fixture count changed: $marker"
  [[ "$marker_digest" =~ ^digest=[0-9a-f]{64}$ ]] || fail "replay outcome digest is invalid: $marker"
  outcome_digests+=("${marker_digest#digest=}")
done

first_digest="${outcome_digests[0]}"
for digest in "${outcome_digests[@]}"; do
  [[ "$digest" == "$first_digest" ]] || fail 'repeated replay rounds produced different outcome digests'
done

joined_digests=$(IFS=,; printf '%s' "${outcome_digests[*]}")
printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\tpassed\n' \
  "$SEED" "$ROUNDS" "$MAX_ROUNDS" "$fixture_count" "$fixture_bytes" \
  "$MAX_FIXTURE_BYTES" "$MAX_TOTAL_BYTES" "$manifest_sha256" "$joined_digests" >> "$REPORT"
printf 'm2-verify bounded replay passed: seed=%s rounds=%s fixtures=%s report=%s\n' \
  "$SEED" "$ROUNDS" "$fixture_count" "$REPORT"
