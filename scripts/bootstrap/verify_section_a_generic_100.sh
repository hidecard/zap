#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
./scripts/bootstrap/verify_b2_type_generic_10.sh
./scripts/bootstrap/verify_b2_type_unification_10.sh
./scripts/bootstrap/verify_b2_function_call_inference_10.sh
./scripts/bootstrap/verify_b2_typed_ir_generic_10.sh
./scripts/bootstrap/verify_b2_loop_call_graph_10.sh
./scripts/bootstrap/verify_b2_program_symbol_graph_10.sh
./scripts/bootstrap/verify_b2_nested_scope_merge_10.sh
./scripts/bootstrap/verify_b2_scope_exit_restore_10.sh
./scripts/bootstrap/verify_b2_scope_merge_10.sh
./scripts/bootstrap/verify_b2_typed_ir_candidate.sh
printf 'Section A generic/type-container batch passed: 100+ focused assertions across 10 gate families\n'
