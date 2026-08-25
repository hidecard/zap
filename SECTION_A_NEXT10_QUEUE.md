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
| 7 | General expression-to-type bridge | [x] partial | Node-kind bridge now includes recursive environment lookup, homogeneous collections, nested list/map indexing, generic identity calls, and nested function-call argument inference; full expression environment remains pending. |
| 8 | General typed-IR emitter | [x] partial | Multi-line declaration/list/map emission၊ source spans နှင့် generic/function metadata foundation pass; inferred call expressions and arbitrary block typed-IR remain pending. |
| 9 | Diagnostic parity matrix | [x] partial | Delimiter, indentation, inconsistent dedent, jump, and missing-block cases pass in focused verifiers; complete reference error matrix remains pending. |
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
- `scripts/bootstrap/verify_b2_typecheck_candidate.sh`
- `scripts/bootstrap/verify_b2_typed_ir_candidate.sh`
- `scripts/bootstrap/verify_vm_platform.sh`
