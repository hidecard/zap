#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
MANIFEST="$ROOT_DIR/native/Cargo.toml"

run_test() {
  local name="$1"
  local filter="$2"
  printf 'p1-05: %s (%s)\n' "$name" "$filter"
  cargo test --manifest-path "$MANIFEST" "$filter" --all-features -- --nocapture
}

# Deterministic parser, lexer, JSON, lockfile, stdlib, and registry-security corpora.
for filter in \
  arithmetic_operator_corpus_is_deterministic_and_complete \
  adversarial_corpus \
  parser_property_corpus_is_panic_free_and_deterministic \
  malformed_program_corpus \
  json_security_corpus_is_deterministic_and_panic_free \
  malformed_lockfile_corpus_is_deterministic_and_panic_free \
  stdlib_security_corpus \
  security_property_origin_normalization_is_idempotent_for_adversarial_corpus \
  security_property_trust_and_credential_scopes_do_not_cross_boundaries \
  security_property_trusted_registry_allowlist_is_bounded_and_deterministic \
  security_property_signed_index_mutations_never_panic_or_accept_tampering \
  security_property_secret_redaction_removes_all_token_occurrences; do
  run_test 'parser/property/security corpus' "$filter"
done

# Memory/lifecycle regressions and deterministic collection behavior.
for filter in \
  collection_iteration_helpers_are_deterministic \
  filesystem_metadata_and_atomic_write_are_deterministic; do
  run_test 'memory, lifecycle, and collection regressions' "$filter"
done

# Async scheduling, cancellation, and bounded adapter determinism.
for filter in delay_ticks_is_deterministic_and_non_blocking cancellation_stops_inner_future_before_polling cancellable_spawn_is_removed_after_cancel; do
  run_test 'async determinism and cancellation' "$filter"
done

"$ROOT_DIR/scripts/test_p105_replay.sh"
"$ROOT_DIR/scripts/test_p105_fuzz_corpus.sh"

printf 'p1-05 deterministic layer validation passed\n'
