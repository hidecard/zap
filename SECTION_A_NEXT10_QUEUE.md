# Zap Section A — Next Ten Compiler/Self-hosting Tasks

**Baseline:** `master` at `8f8a823` with clean working tree and all verifier-backed follow-up commits published. The checklist uses `[x]` only for bounded/provisional evidence; full/general/self-hosting acceptance remains `[ ]` until the corresponding acceptance gate passes.

| Order | Task | Current status | Evidence/next acceptance |
|---:|---|---|---|
| 1 | Token cursor abstraction | [x] foundation | Immutable cursor with peek/advance/eof and peek-kind; `verify_b1_token_cursor.sh` passes. |
| 2 | Span-based indentation stack | [x] partial | Parser-owned history/depth stack validates one-level nesting, valid prior-level dedent, inconsistent dedent, and jump errors; token-native full ownership remains pending. |
| 3 | Recursive `parse_block(indent)` | [x] partial | Cursor-based `parse_block_from` handles bounded arbitrary-depth bodies, same-level sibling dedent, generic top-level `if`, `try/catch`, and additional `return`/`raise`/`import`/`module` statement nodes; function/class and all control-flow grammar replacement remains pending. |
| 4 | Arbitrary mixed top-level sequence | [x] partial | Recursive CFG and flow-transfer helpers now handle arbitrary mixed statement fixtures with exact branch/fallthrough successors and real assignment propagation; all parser statement kinds and full ownership remain pending. |
| 5 | Recursive `if/elif/else` blocks | [x] partial | Same-level `else`, bounded `elif` lowering, chained final `else`, missing-body diagnostics, nested live-path merge, short-circuit path states, recursive CFG nodes, and exact branch/fallthrough successors pass; arbitrary condition-expression edge ownership remains pending. |
| 6 | Recursive loop bodies and loop control | [x] partial | Generic top-level `for`/`while` routes, `break`/`continue` AST nodes, automatic normal-body back-edge, stable/divergent fixpoint convergence, and nested break/continue ownership pass focused cases; arbitrary loop CFG/dataflow integration remains pending. |
| 7 | General expression-to-type bridge | [x] partial | Node-kind bridge now includes recursive literal/list/map/index/binary value inference, identity/known-return call inference, program declaration/reassignment propagation, condition-derived option/result narrowing, compound path states, nested scope fallback, multi-path branch merge, reassignment invalidation, multi-state loop fixpoint, call-cycle foundations, and recursive wrapper unification; full AST-driven flow environment remains pending. |
| 8 | General typed-IR emitter | [x] partial | Multi-line declaration/list/map emission၊ source spans၊ generic function/type-alias metadata၊ recursive type-unification၊ generic end-to-end၊ bounded expression nodes၊ arbitrary control-statement sequence nodes၊ and `raise/import/module/try/catch` classification pass; complete parser AST/block metadata and full ownership remain pending. |
| 9 | Diagnostic parity matrix | [x] partial | Delimiter, indentation, nested scope/branch merge, scope exit, loop mutation/fixpoint, call arity/type/constraint, cycle foundations, and bounded diagnostic field parity pass; complete reference error matrix remains pending. |
| 10 | Bootstrap package/build/VM ownership | [x] partial | Zap-written package/lock/offline-policy, stack-VM, deterministic build-plan/artifact manifest, and B4 seed preflight foundations pass focused cases; native Rust ownership transfer, complete bytecode semantics, and self-rebuild remain pending. |

## Current expansion batch

The requested 10,000 flow-sensitive/arbitrary-AST tasks, 1,000 generic runtime/recursive-call tasks, and 100,000-task expansion are represented by verifier-backed batches rather than artificial per-task marks. The latest consolidated Main-3 run passes B1 parser/control flow, B2 complete-inference/flow/CFG/diagnostic/typed-IR/generic, B3 package/VM foundations, B4 seed preflight, native generic runtime, and VM/platform suites. Full parser/type-inference ownership, native-independent package/VM execution, and B4 self-rebuild remain open.

## Execution order

Tasks 1–6 are parser prerequisites. Tasks 7–9 depend on stable AST/block ownership. Task 10 depends on general typed IR and complete diagnostics. The B4 contract must remain `self_hosted = false` until task 10 and platform-seed self-rebuild acceptance pass.

## Current verified gates

The repository currently passes the B1 parser candidate differential and B2 type-checker candidate differential suites. Existing bounded evidence includes function/loop/class ASTs, nested calls, parenthesized expressions, mixed sequences, nested blocks, arithmetic and boolean inference slices, compound option guards, and reassignment invalidation.

## Explicit non-claims

This queue does not claim that the fully arbitrary parser, complete type inference, general typed IR, package/build ownership, VM ownership, platform-seed acceptance, or B4 self-hosting is complete. Those remain unchecked until their full acceptance criteria are implemented and verified.

## References

- `SECTION_A_STATUS_CHECKLIST_MM.md`
- `bootstrap/contracts/BOOTSTRAP_CONTRACT_EN.md`
- `scripts/bootstrap/verify_b1_parser_candidate.sh`
- `scripts/bootstrap/verify_b1_statement_coverage_8.sh` — covers 8 empty-return, raise, import, and module parser statement cases
- `scripts/bootstrap/verify_b2_complete_inference_10.sh` — covers 12 recursive value, call, and program-flow inference cases
- `scripts/bootstrap/verify_b3_build_plan_10.sh` — covers 13 deterministic build-plan and artifact ownership cases
- `scripts/bootstrap/verify_b2_typed_ir_additional_5.sh` — covers 5 raise/import/module/try/catch typed-IR cases
- `scripts/bootstrap/verify_b3_zap_ownership_20.sh` — covers 20 Zap-written package/lock/offline-policy and stack-VM foundation cases
- `scripts/bootstrap/verify_b4_seed_preflight_10.sh` — covers 10 deterministic platform-seed descriptor and reproducibility cases
- `scripts/bootstrap/verify_b1_token_cursor.sh`
- `scripts/bootstrap/verify_b1_recursive_blocks.sh`
- `scripts/bootstrap/verify_b1_branch_chain.sh`
- `scripts/bootstrap/verify_b1_control_flow_blocks.sh`
- `scripts/bootstrap/verify_b2_ast_expression_bridge.sh`
- `scripts/bootstrap/verify_b2_type_generic_10.sh` — covers 15 inference/generic cases
- `scripts/bootstrap/verify_b2_typed_ir_generic_10.sh` — covers 10 typed-IR/generic metadata cases
- `scripts/bootstrap/verify_b2_function_call_inference_10.sh` — covers 13 function/call inference cases
- `scripts/bootstrap/verify_b2_scope_merge_10.sh` — covers 10 symbol-environment and branch-merge cases
- `scripts/bootstrap/verify_b2_program_symbol_graph_10.sh` — covers 14 program symbol-graph collection/scope/update cases
- `scripts/bootstrap/verify_b2_scope_exit_restore_10.sh` — covers 10 branch-merge and scope-restoration cases
- `scripts/bootstrap/verify_b2_loop_call_graph_10.sh` — covers 10 loop-mutation and call-graph cases
- `scripts/bootstrap/verify_b2_nested_scope_merge_10.sh` — covers 10 nested-scope and branch-local merge cases
- `scripts/bootstrap/verify_b2_loop_fixpoint_cycles_10.sh` — covers 10 loop-fixpoint and cycle cases
- `scripts/bootstrap/verify_b2_type_unification_10.sh` — covers 10 recursive type-unification cases
- `scripts/bootstrap/verify_section_a_next50.sh` — runs the consolidated 50-case acceptance batch
- `scripts/bootstrap/verify_section_a_type_symbol_100.sh` — runs 100+ type-inference and program-symbol-graph assertions
- `scripts/bootstrap/verify_section_a_generic_100.sh` — runs 100+ generic/type-container assertions
- `scripts/bootstrap/verify_b2_generic_bounds_10.sh` — covers 10 multiple-bound generic call cases
- `scripts/bootstrap/verify_b2_generic_type_declaration_10.sh` — covers 10 generic type declaration/container cases
- `scripts/bootstrap/verify_b2_generic_end_to_end_10.sh` — covers 10 parser/typechecker/typed-IR generic integration cases
- `scripts/bootstrap/verify_b2_generic_runtime_recursive_10.sh` — covers 11 bounded generic runtime-container compatibility and recursive generic-call cases
- `scripts/bootstrap/verify_native_generic_runtime_10.sh` — covers evaluator-integrated generic list/option values, recursive generic calls, and unwrap execution
- `scripts/bootstrap/verify_b2_typed_ir_expression_10.sh` — covers 10 typed-IR call, nested-call, binary-expression, and declaration-value cases
- `scripts/bootstrap/verify_b2_flow_sensitive_10.sh` — covers 10 multi-path branch, invalidation, loop-fixpoint, and scope-restoration cases
- `scripts/bootstrap/verify_b2_condition_cfg_10.sh` — covers 10 condition-derived narrowing, compound path, CFG node, and edge cases
- `scripts/bootstrap/verify_b2_short_circuit_loop_edges_12.sh` — covers 12 short-circuit and/or, nested branch, loop back-edge, break, and continue ownership cases
- `scripts/bootstrap/verify_b2_recursive_cfg_loop_convergence_12.sh` — covers 12 recursive nested-AST CFG, loop back-edge convergence, and nested break/continue ownership cases
- `scripts/bootstrap/verify_b2_arbitrary_flow_diagnostics_14.sh` — covers 14 arbitrary-program CFG, exact branch/fallthrough, reassignment propagation, and diagnostic parity cases
- `scripts/bootstrap/verify_b2_section_a_next20_20.sh` — covers 20 CFG category, condition metadata, exception/scope edge, reassignment, and diagnostic failure-parity assertions
- `scripts/bootstrap/verify_b2_typed_ir_arbitrary_10.sh` — covers 10 arbitrary control-statement typed-IR emission cases
- `scripts/bootstrap/verify_b2_typecheck_candidate.sh`
- `scripts/bootstrap/verify_b2_typed_ir_candidate.sh`
- `scripts/bootstrap/verify_vm_platform.sh`
