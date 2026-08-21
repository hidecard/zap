#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
MANIFEST="$ROOT_DIR/native/Cargo.toml"
TARGET="${ZAP_ASYNC_TARGET:-$(rustc -vV | sed -n 's/^host: //p')}"
SAFE_TARGET=$(printf '%s' "$TARGET" | tr '/:' '__')
LOG_PATH="${ZAP_ASYNC_LOG:-$ROOT_DIR/target/p005c-async-${SAFE_TARGET}.log}"
mkdir -p "$(dirname "$LOG_PATH")"

# Keep a complete, target-named record for CI artifact upload and local replay.
exec > >(tee "$LOG_PATH") 2>&1

printf 'p0-05-c: cross-platform async matrix\n'
printf 'target: %s\n' "$TARGET"
printf 'runner_os: %s\n' "${RUNNER_OS:-local}"
printf 'rust: %s\n' "$(rustc --version)"
printf 'cargo: %s\n' "$(cargo --version)"
printf 'manifest: %s\n' "$MANIFEST"
printf 'log: %s\n' "$LOG_PATH"

run_test() {
  local name="$1"
  printf '\np0-05-c: %s\n' "$name"
  cargo test --manifest-path "$MANIFEST" --target "$TARGET" "$name" --all-features -- --exact --nocapture
}

# The same exact filters run on Linux, Windows, and macOS. Platform-specific
# command/path branches live inside the Rust tests and are therefore exercised
# natively by each target runner.
for test_name in \
  threaded_runtime_runs_two_tasks_concurrently \
  threaded_runtime_rejects_invalid_limits_before_starting_workers \
  nonblocking_tcp_exchange_round_trips_with_bounded_response \
  nonblocking_tcp_exchange_rejects_oversized_response \
  tcp_exchange_rejects_oversized_request_before_admission \
  async_process_adapter_captures_output_cross_platform \
  async_process_adapter_rejects_capped_output \
  async_process_cancellation_terminates_child \
  async_file_read_is_bounded_and_returns_bytes; do
  run_test "$test_name"
done

printf '\np0-05-c: matrix passed for %s\n' "$TARGET"
