# Zap Section A — Next Ten Compiler/Self-hosting Tasks

**Baseline:** `master` at `346f7d5` plus verified working-tree follow-up history. The checklist uses `[x]` only for bounded/provisional evidence; full/general/self-hosting acceptance remains `[ ]` until the corresponding acceptance gate passes.

| Order | Task | Current status | Evidence/next acceptance |
|---:|---|---|---|
| 1 | Token cursor abstraction | [ ] | Add a cursor over lexer token JSON with deterministic peek/consume/eof operations. |
| 2 | Span-based indentation stack | [ ] | Derive indentation levels from token line/column spans and reject inconsistent dedents. |
| 3 | Recursive `parse_block(indent)` | [ ] | Replace line-count branches with recursive statement-list parsing. |
| 4 | Arbitrary mixed top-level sequence | [x] bounded | Flat append-backed sequence works for the current bounded statement corpus; arbitrary grammar remains pending. |
| 5 | Recursive `if/elif/else` blocks | [ ] | Add arbitrary-depth branch-chain reference differential fixtures. |
| 6 | Recursive loop bodies and loop control | [x] bounded | `for`/`while` and `break`/`continue` fixtures pass; generic indentation ownership remains pending. |
| 7 | General expression-to-type bridge | [ ] | Connect parser expression nodes to type inference instead of string-pattern cases. |
| 8 | General typed-IR emitter | [ ] | Emit typed IR for arbitrary declarations, calls, blocks, and control flow. |
| 9 | Diagnostic parity matrix | [ ] | Compare error kind, message, line, column, and failure behavior across malformed programs. |
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
- `scripts/bootstrap/verify_b2_typecheck_candidate.sh`
- `scripts/bootstrap/verify_b2_typed_ir_candidate.sh`
- `scripts/bootstrap/verify_vm_platform.sh`
