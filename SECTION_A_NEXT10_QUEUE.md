# Zap Section A — Next Ten Compiler/Self-hosting Tasks

**Baseline:** `master` at `346f7d5` plus verified working-tree follow-up history. The checklist uses `[x]` only for bounded/provisional evidence; full/general/self-hosting acceptance remains `[ ]` until the corresponding acceptance gate passes.

| Order | Task | Current status | Evidence/next acceptance |
|---:|---|---|---|
| 1 | Token cursor abstraction | [x] foundation | Immutable cursor with peek/advance/eof and peek-kind; `verify_b1_token_cursor.sh` passes. |
| 2 | Span-based indentation stack | [x] partial | Parser-owned history/depth stack validates one-level nesting, valid prior-level dedent, inconsistent dedent, and jump errors; token-native full ownership remains pending. |
| 3 | Recursive `parse_block(indent)` | [x] partial | Cursor-based `parse_block_from` handles bounded arbitrary-depth bodies, same-level sibling dedent, and generic top-level `if`; function/class and all control-flow grammar replacement remains pending. |
| 4 | Arbitrary mixed top-level sequence | [x] bounded | Flat append-backed sequence works for the current bounded statement corpus; arbitrary grammar remains pending. |
| 5 | Recursive `if/elif/else` blocks | [x] partial | Same-level `else`, bounded `elif` lowering, chained final `else`, and missing-body diagnostics pass in 10-case and control-flow verifiers; full branch semantics remain pending. |
| 6 | Recursive loop bodies and loop control | [x] partial | Generic top-level `for`/`while` routes and `break`/`continue` AST nodes pass focused cases; function/class integration remains pending. |
| 7 | General expression-to-type bridge | [x] partial | Node-kind bridge now includes recursive list/map environment typing, function-aware call inference, program expression propagation, nested scope fallback, branch-local merge, loop fixpoint widening, call-cycle foundations, and recursive wrapper unification; full AST-driven flow environment remains pending. |
| 8 | General typed-IR emitter | [x] partial | Multi-line declaration/list/map emission၊ source spans၊ generic/function metadata နှင့် generic type declaration foundations pass; recursive type-unification is available but symbol-graph-driven call/block typed-IR remains pending. |
| 9 | Diagnostic parity matrix | [x] partial | Delimiter, indentation, nested scope/branch merge, scope exit, loop mutation/fixpoint, call arity/type/constraint and cycle foundations pass in focused verifiers; complete reference error matrix remains pending. |
| 10 | Bootstrap package/build/VM ownership | [ ] | Move compiler/build/VM execution ownership from native Rust boundary and prove seed rebuild. |

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
- `scripts/bootstrap/verify_b2_typecheck_candidate.sh`
- `scripts/bootstrap/verify_b2_typed_ir_candidate.sh`
- `scripts/bootstrap/verify_vm_platform.sh`
