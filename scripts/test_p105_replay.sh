#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
MANIFEST="$ROOT_DIR/native/Cargo.toml"
SEED="${ZAP_CORPUS_SEED:-20260821}"
LOG_PATH="${ZAP_CORPUS_REPLAY_LOG:-$ROOT_DIR/target/p105-replay.log}"
CORPUS_ROOT="$ROOT_DIR/corpus/p1-05"

if [[ ! "$SEED" =~ ^[1-9][0-9]*$ ]]; then
  printf 'p1-05 replay: ZAP_CORPUS_SEED must be a positive decimal integer\n' >&2
  exit 2
fi

mkdir -p "$(dirname "$LOG_PATH")"
: > "$LOG_PATH"
printf 'seed=%s\n' "$SEED" >> "$LOG_PATH"
printf 'corpus_root=%s\n' "$CORPUS_ROOT" >> "$LOG_PATH"

while IFS= read -r fixture; do
  relative=${fixture#"$CORPUS_ROOT/"}
  category=${relative%%/*}
  encoded=$(base64 < "$fixture" | tr -d '\n')
  if command -v sha256sum >/dev/null 2>&1; then
    digest=$(sha256sum "$fixture" | awk '{print $1}')
  else
    digest=$(shasum -a 256 "$fixture" | awk '{print $1}')
  fi
  printf 'fixture=%s category=%s sha256=%s input_base64=%s\n' \
    "$relative" "$category" "$digest" "$encoded" >> "$LOG_PATH"
done < <(find "$CORPUS_ROOT" -type f | LC_ALL=C sort)

printf 'p1-05 replay: seed=%s fixtures recorded at %s\n' "$SEED" "$LOG_PATH"
ZAP_CORPUS_SEED="$SEED" cargo test \
  --manifest-path "$MANIFEST" \
  replayable_failure_corpus_is_seeded_panic_free_and_deterministic \
  --all-features -- --nocapture
printf 'p1-05 replayable corpus passed with seed=%s\n' "$SEED"
