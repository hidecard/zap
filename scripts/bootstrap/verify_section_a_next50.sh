#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
./scripts/bootstrap/verify_b2_type_unification_10.sh
./scripts/bootstrap/verify_b2_function_call_inference_10.sh
./scripts/bootstrap/verify_b2_nested_scope_merge_10.sh
./scripts/bootstrap/verify_b2_scope_exit_restore_10.sh
./scripts/bootstrap/verify_b2_loop_fixpoint_cycles_10.sh
printf 'Section A next-50 acceptance batch passed: 50 focused inference, scope, loop, and call-graph cases\n'
