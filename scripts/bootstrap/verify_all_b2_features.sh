#!/bin/bash
# Master verification script for all B2 features
set -e

echo "Running comprehensive B2 feature verification..."

# Generic constraints
echo "=== B2 Generic Constraints ==="
./scripts/bootstrap/verify_b2_generic_bounds_10.sh
./scripts/bootstrap/verify_b2_p0_trait_generic_bounds_17.sh
./scripts/bootstrap/verify_b2_p0_engine_explicit_generic_args_19.sh
./scripts/bootstrap/verify_b2_compound_bounds_20.sh

# Alias checking
echo "=== B2 Alias Checking ==="
./scripts/bootstrap/verify_b2_alias_expansion_21.sh

# Dataflow
echo "=== B2 Dataflow ==="
./scripts/bootstrap/verify_b2_flow_sensitive_10.sh
./scripts/bootstrap/verify_b2_compound_guards_8.sh
./scripts/bootstrap/verify_b2_loop_fixpoint_cycles_10.sh

# Canonical AST bridge
echo "=== B3 Canonical AST Bridge ==="
./scripts/bootstrap/verify_b2_ast_expression_bridge.sh
./scripts/bootstrap/verify_b2_ast_inference_general.sh
./scripts/bootstrap/verify_b3_canonical_ast_schema.sh

# Typed-IR producer
echo "=== B3 Typed-IR Producer ==="
./scripts/bootstrap/verify_b2_typed_ir_owned_program_38.sh
./scripts/bootstrap/verify_b2_typed_ir_arbitrary_10.sh

# VM/runtime
echo "=== B3 VM/Runtime ==="
./scripts/bootstrap/verify_b4_source_to_vm_10.sh
./scripts/bootstrap/verify_b4_source_to_vm_functions_14.sh
./scripts/bootstrap/verify_b4_source_to_vm_classes_12.sh
./scripts/bootstrap/verify_b4_source_to_vm_closures_12.sh

# Differential corpus
echo "=== B1 Differential Corpus ==="
./scripts/bootstrap/verify_b1_general_parser.sh
./scripts/bootstrap/verify_b1_parser_candidate.sh

echo "=== All B2/B3/B1 feature verifications passed ==="
