**Current release line:** v2.11.17
# P0 Full Language Ownership Matrix

**Verified baseline:** Zap v2.11.17
**Purpose:** Canonical owner/fixture/status registry for every Zap language domain — grammar, generics, aliases, dataflow, overloads, traits/MRO, closures, diagnostics, stdlib, package resolver, lockfile, CLI/LSP, and VM behavior.
**Machine-readable index:** [`LANGUAGE_OWNERSHIP_MATRIX.tsv`](LANGUAGE_OWNERSHIP_MATRIX.tsv)

This matrix is the single source of truth for which implementation module owns each language rule, which fixture or test proves it, and what its current status is. Every public rule must have an owner, at least one fixture, and an explicit status. Gaps are explicit — unlisted behavior is not implicitly normative.

## Status definitions

| Status | Meaning |
|---|---|
| `implemented` | Code + tests exist in the current release line; behavior is normative or compatibility as labeled |
| `partial` | Baseline implemented but known gaps remain (documented in the `notes` column) |
| `deferred` | Explicitly postponed to a future design phase; not implemented |
| `not-implemented` | No code or test exists; explicitly out of scope or awaiting design |

## Compatibility classes

| Class | Meaning |
|---|---|
| `normative` | Canonical behavior for the current release line |
| `compatibility` | Preserved for migration from an older release; not the preferred form |
| `deprecated` | Retained with a documented migration path and removal window |
| `rejected` | Explicitly forbidden; must produce a stable diagnostic |

---

## 1. Grammar

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| LANG-001 | Indentation-delimited blocks (mixed tabs/spaces rejected) | `native/src/ast.rs#parse_program` | `bootstrap/fixtures/parser/invalid_indentation.zp` | implemented | normative | |
| LANG-002 | Statement declarations (let, say, return) | `native/src/ast.rs` | `bootstrap/fixtures/parser/assignment_statement.zp` | implemented | normative | |
| LANG-003 | Expression statements and operator precedence | `native/src/ast.rs#parse_expression` | `bootstrap/fixtures/parser/multiplicative_additive.zp` | implemented | normative | |
| LANG-004 | Function declaration syntax (fn name(args) -> Ret:) | `native/src/parser.rs` | `bootstrap/fixtures/parser/simple_function.zp` | implemented | normative | |
| LANG-005 | Class declaration syntax (class Name extends Base:) | `native/src/ast.rs` | `bootstrap/fixtures/parser/simple_class.zp` | implemented | normative | |
| LANG-006 | Control-flow syntax (if/else, while, for) | `native/src/ast.rs` | `bootstrap/fixtures/parser/control_flow.zp` | implemented | normative | |
| LANG-007 | Module/import declarations (module, import/export) | `native/src/ast.rs` | `native/tests/core.rs#validates_explicit_module_manifest_entries` | implemented | normative | |
| LANG-008 | Legacy use import syntax | `native/src/ast.rs` | `conformance/p0-01/common/` | implemented | compatibility | Legacy path retained |
| LANG-009 | Blank line and comment handling | `native/src/lexer.rs` | `bootstrap/fixtures/parser/mixed_top_level.zp` | implemented | normative | |
| LANG-010 | Unused token / trailing garbage rejection | `native/src/ast.rs` | `bootstrap/fixtures/parser/malformed_recovery.zp` | implemented | normative | |
| LANG-011 | Nested block depth and indentation jump rejection | `native/src/ast.rs` | `bootstrap/fixtures/parser/invalid_indentation_jump.zp` | implemented | normative | |
| LANG-012 | Deep nesting and complex expressions | `native/src/ast.rs` | `bootstrap/fixtures/parser/arbitrary_deep_nesting.zp` | implemented | normative | |
| LANG-013 | Grouped expressions and parenthesized forms | `native/src/ast.rs` | `bootstrap/fixtures/parser/grouped_expression.zp` | implemented | normative | |
| LANG-014 | Logical/comparison expression matrix | `native/src/ast.rs` | `bootstrap/fixtures/parser/logical_comparison_matrix.zp` | implemented | normative | |
| LANG-015 | Mixed recursive statement sequences | `native/src/ast.rs` | `bootstrap/fixtures/parser/mixed_recursive_sequence.zp` | implemented | normative | |
| LANG-016 | Unicode identifiers and source spans | `native/src/lexer.rs` | `bootstrap/fixtures/parser/unicode_identifier.zp` | implemented | normative | |
| LANG-017 | Number literals (multi-digit, negative, decimal) | `native/src/lexer.rs` | `bootstrap/fixtures/parser/multi_digit_number.zp` | implemented | normative | |
| LANG-018 | Deep mixed block structures | `native/src/ast.rs` | `bootstrap/fixtures/parser/deep_mixed_blocks.zp` | implemented | normative | |
| LANG-019 | Multi-argument call expressions | `native/src/ast.rs` | `bootstrap/fixtures/parser/three_argument_call.zp` | implemented | normative | |
| LANG-020 | Nested class/method declarations | `native/src/ast.rs` | `bootstrap/fixtures/parser/nested_class_method.zp` | implemented | normative | |
| LANG-021 | While/else syntax variants | `native/src/ast.rs` | `bootstrap/fixtures/parser/while_else_syntax.zp` | implemented | normative | |
| LANG-022 | Nested assignment within blocks | `native/src/ast.rs` | `bootstrap/fixtures/parser/nested_assignment_block.zp` | implemented | normative | |
| LANG-023 | Full expression surface | `native/src/ast.rs` | `bootstrap/fixtures/parser/full_expression.zp` | implemented | normative | |

---

## 2. Generics / Constraints

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| GEN-001 | list<T> annotation parsing | `native/src/parser.rs` | `bootstrap/fixtures/typecheck/nested_collection.zp` | implemented | normative | |
| GEN-002 | map<K, V> annotation parsing | `native/src/parser.rs` | `bootstrap/fixtures/typecheck/map_collection.zp` | implemented | normative | Key type restricted to text/any |
| GEN-003 | option<T> annotation parsing | `native/src/parser.rs` | `bootstrap/fixtures/typecheck/option_annotation.zp` | implemented | normative | |
| GEN-004 | result<T> annotation parsing | `native/src/parser.rs` | `bootstrap/fixtures/typecheck/result_error_handling.zp` | implemented | normative | |
| GEN-005 | Nested generic matching (list<list<number>>) | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/nested_collection.zp` | implemented | normative | |
| GEN-006 | Generic annotation incompatible assignment rejection | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/collection_incompatible.zp` | implemented | normative | |
| GEN-007 | Malformed generic rejection (list<>, unbalanced) | `native/src/parser.rs` | `bootstrap/fixtures/typecheck/generic_empty_params.zp` | implemented | normative | |
| GEN-008 | Generic arity mismatch rejection | `native/src/parser.rs` | `bootstrap/fixtures/typecheck/generic_arity_mismatch.zp` | implemented | normative | |
| GEN-009 | Generic cross-module signatures | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/generic_cross_module.zp` | implemented | normative | |
| GEN-010 | Generic function declaration (identity<T>) | `native/src/parser.rs` | `bootstrap/fixtures/typecheck/generic_function_simple.zp` | implemented | normative | Bounded slice |
| GEN-011 | Generic class declaration | `native/src/parser.rs` | `bootstrap/fixtures/typecheck/generic_class.zp` | implemented | normative | Bounded slice |
| GEN-012 | Generic wrapper runtime and arity | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/generic_runtime_wrappers.zp` | implemented | normative | |
| GEN-013 | Generic identity typed-IR metadata | `native/src/bootstrap.rs` | `bootstrap/fixtures/typecheck/generic_identity.zp` | implemented | normative | |
| GEN-014 | Generic scope boundaries | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/generic_scope_positive.zp` | implemented | normative | |
| GEN-015 | Imported generic body boundary | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/generic_cross_module_body.zp` | implemented | normative | |
| GEN-016 | Nested generic substitution | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/generic_nested_option_list.zp` | implemented | normative | |
| GEN-017 | Generic constraint syntax (where, extends, :) | — | `bootstrap/fixtures/typecheck/generic_constraint_where.zp` | deferred | — | No parser/runtime support yet |
| GEN-018 | User-defined generic declarations (full) | — | `bootstrap/fixtures/typecheck/generic_class_deferred.zp` | deferred | — | Separate design phase |
| GEN-019 | Explicit generic call syntax | — | `bootstrap/fixtures/typecheck/generic_explicit_call_deferred.zp` | deferred | — | Separate design phase |
| GEN-020 | Generic alias declarations | — | `bootstrap/fixtures/typecheck/generic_alias_deferred.zp` | deferred | — | Separate design phase |
| GEN-021 | Generic compound bounds | — | `bootstrap/fixtures/typecheck/generic_compound_bounds.zp` | deferred | — | |
| GEN-022 | Generic variance rules | — | — | not-implemented | — | Future design |

---

## 3. Alias Environment

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| ALIAS-001 | Alias assignment and resolution | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/alias_generic.zp` | implemented | normative | |
| ALIAS-002 | Alias of alias resolution | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/alias_of_alias.zp` | implemented | normative | |
| ALIAS-003 | Nested list alias | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/alias_nested_list.zp` | implemented | normative | |
| ALIAS-004 | Nested map alias | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/alias_nested_map.zp` | implemented | normative | |
| ALIAS-005 | Nested option alias | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/alias_nested_option.zp` | implemented | normative | |
| ALIAS-006 | Recursive alias detection | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/alias_recursive.zp` | implemented | normative | |
| ALIAS-007 | Undeclared parameter alias rejection | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/alias_undeclared_param.zp` | implemented | normative | |
| ALIAS-008 | Alias mutation invalidation | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/alias_mutation_invalidation.zp` | implemented | normative | |
| ALIAS-009 | Alias import across modules | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/generic_alias_import.zp` | implemented | normative | |

---

## 4. Dataflow Analysis

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| FLOW-001 | Branch-local narrowing (if is_some) | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/branch_narrowing.zp` | implemented | normative | |
| FLOW-002 | Branch narrowing incompatible rejection | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/branch_narrowing_incompatible.zp` | implemented | normative | |
| FLOW-003 | Loop-body narrowing (while + guard) | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/loop_narrowing.zp` | implemented | normative | |
| FLOW-004 | Loop narrowing incompatible rejection | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/loop_narrowing_incompatible.zp` | implemented | normative | |
| FLOW-005 | Loop wrapper restoration after exit | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/loop_narrowing.zp` | implemented | normative | |
| FLOW-006 | Else-body narrowing (is_option_none) | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/else_narrowing.zp` | implemented | normative | |
| FLOW-007 | Else narrowing incompatible rejection | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/else_narrowing_incompatible.zp` | implemented | normative | |
| FLOW-008 | Compound guard narrowing | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/compound_guard.zp` | implemented | normative | Bounded slice |
| FLOW-009 | Compound guard incompatible rejection | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/compound_guard_incompatible.zp` | implemented | normative | |
| FLOW-010 | Nested control-flow narrowing | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/nested_control_flow_narrowing.zp` | implemented | normative | |
| FLOW-011 | Reassignment invalidation | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/reassignment_invalidation.zp` | implemented | normative | |
| FLOW-012 | Reassignment incompatible rejection | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/reassignment_invalidation_incompatible.zp` | implemented | normative | |
| FLOW-013 | Mutation invalidation inside branch | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/mutation_invalidation.zp` | implemented | normative | |
| FLOW-014 | Loop mutation invalidation | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/loop_mutation.zp` | implemented | normative | |
| FLOW-015 | Short-circuit and evaluation | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/short_circuit_and.zp` | implemented | normative | |
| FLOW-016 | Short-circuit or evaluation | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/short_circuit_or.zp` | implemented | normative | |
| FLOW-017 | Flow engine merge (return/break/continue) | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/return_break_merge.zp` | implemented | normative | |
| FLOW-018 | Loop fixpoint with break | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/loop_fixpoint_break.zp` | implemented | normative | |
| FLOW-019 | Loop fixpoint with continue | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/loop_fixpoint_continue.zp` | implemented | normative | |
| FLOW-020 | Arbitrary index expression narrowing | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/arbitrary_index_expression.zp` | implemented | normative | |
| FLOW-021 | Arbitrary index type mismatch rejection | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/arbitrary_index_type_mismatch.zp` | implemented | normative | |
| FLOW-022 | Arbitrary map key narrowing | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/arbitrary_map_key.zp` | implemented | normative | |
| FLOW-023 | Arbitrary map key type mismatch rejection | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/arbitrary_map_key_type_mismatch.zp` | implemented | normative | |
| FLOW-024 | Loop narrowing with reassignment | `native/src/evaluator.rs` | `bootstrap/fixtures/typecheck/loop_narrowing_reassignment.zp` | implemented | normative | |

---

## 5. Overloads

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| OVR-001 | Function overloading (same name, different arity/types) | — | — | not-implemented | — | Not part of current design |
| OVR-002 | Operator overloading | — | — | not-implemented | — | Not part of current design |
| OVR-003 | Method overloading within class | — | — | not-implemented | — | Not part of current design |

---

## 6. Traits / MRO

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| TRAIT-001 | Single inheritance (class Child extends Parent) | `native/src/ast.rs` | `bootstrap/fixtures/parser/simple_class.zp` | implemented | normative | |
| TRAIT-002 | Inherited constructor semantics | `native/src/evaluator.rs` | `native/tests/core.rs` | implemented | normative | |
| TRAIT-003 | Method override | `native/src/evaluator.rs` | `native/tests/core.rs` | implemented | normative | |
| TRAIT-004 | super.method() call | — | — | not-implemented | — | |
| TRAIT-005 | super.init() explicit call | — | — | not-implemented | — | |
| TRAIT-006 | Traits / interfaces | — | — | deferred | — | M4-RFC-01 design-only |
| TRAIT-007 | Multiple inheritance | — | — | not-implemented | — | Explicitly excluded |
| TRAIT-008 | Method resolution order (MRO) | — | — | deferred | — | Tied to traits design |
| TRAIT-009 | Circular inheritance detection | `native/src/evaluator.rs` | — | implemented | normative | Part of class validation |
| TRAIT-010 | Abstract class / abstract method | — | — | not-implemented | — | |
| TRAIT-011 | Public/private/protected visibility | — | — | not-implemented | — | Module-level export only |

---

## 7. Closures

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| CLOS-001 | Lexical environment capture | `native/src/evaluator.rs` | `bootstrap/fixtures/vm/closure.zp` | implemented | normative | |
| CLOS-002 | Parent-linked EnvFrame binding | `native/src/evaluator.rs` | `native/tests/core.rs#parent_linked_closures_preserve_mutation_after_outer_return` | implemented | normative | |
| CLOS-003 | Shared binding-cell capture | `native/src/evaluator.rs` | `native/tests/core.rs#live_closures_share_reassigned_outer_cells_without_breaking_shadowing_or_recursion` | implemented | normative | |
| CLOS-004 | Returned closure lifetime | `native/src/evaluator.rs` | `native/tests/core.rs` | implemented | normative | |
| CLOS-005 | Shadowing vs mutation semantics | `native/src/evaluator.rs` | `native/tests/core.rs` | implemented | normative | |
| CLOS-006 | Recursion through closures | `native/src/evaluator.rs` | `native/tests/core.rs` | implemented | normative | |
| CLOS-007 | Closure cycle behavior | `native/src/value.rs` | `native/tests/core.rs#cyclic_object_graph_can_be_explicitly_broken` | implemented | normative | Explicit clear required |
| CLOS-008 | Callable value serialization | `native/src/value.rs` | `native/tests/core.rs` | implemented | normative | {"__zap_variant":"callable"} |

---

## 8. Diagnostics

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| DIAG-001 | Stable ZAP-* code registry | `native/src/diagnostics.rs` | `docs/DIAGNOSTIC_MODEL_EN.md` | implemented | normative | |
| DIAG-002 | kind field (SyntaxError, TypeError, etc.) | `native/src/diagnostics.rs` | `bootstrap/fixtures/diagnostics/` | implemented | normative | |
| DIAG-003 | Source span (file/line/column) | `native/src/diagnostics.rs` | `bootstrap/fixtures/diagnostics/missing_closing_bracket.json` | implemented | normative | |
| DIAG-004 | notes and help fields | `native/src/diagnostics.rs` | `native/tests/core.rs` | implemented | normative | |
| DIAG-005 | CLI/LSP diagnostic parity | `native/src/lsp.rs` | `native/tests/core.rs#lsp_diagnostics_match_cli_type_error_contract` | implemented | normative | |
| DIAG-006 | JSON diagnostic schema (zap check --json) | `native/src/diagnostics.rs` | `bootstrap/fixtures/diagnostics/invalid_character.json` | implemented | normative | |
| DIAG-007 | Secret redaction in error messages | `native/src/diagnostics.rs` | `native/tests/core.rs` | implemented | normative | |
| DIAG-008 | ZAP-BORROW-001 borrow conflict | `native/src/value.rs` | `native/tests/core.rs#conflicting_object_borrows_return_typed_failures` | implemented | normative | |
| DIAG-009 | ZAP-MEMORY-001 memory limit | `native/src/value.rs` | `native/tests/core.rs` | implemented | normative | |
| DIAG-010 | Invalid character diagnostics | `native/src/diagnostics.rs` | `bootstrap/fixtures/diagnostics/invalid_character.json` | implemented | normative | |
| DIAG-011 | Unterminated string diagnostics | `native/src/diagnostics.rs` | `bootstrap/fixtures/diagnostics/unterminated_string.json` | implemented | normative | |
| DIAG-012 | Missing closing bracket diagnostics | `native/src/diagnostics.rs` | `bootstrap/fixtures/diagnostics/missing_closing_bracket.json` | implemented | normative | |
| DIAG-013 | Unexpected closing bracket diagnostics | `native/src/diagnostics.rs` | `bootstrap/fixtures/diagnostics/unexpected_closing_bracket.json` | implemented | normative | |
| DIAG-014 | Missing assignment diagnostics | `native/src/diagnostics.rs` | `bootstrap/fixtures/diagnostics/missing_assignment.json` | implemented | normative | |
| DIAG-015 | Missing function paren diagnostics | `native/src/diagnostics.rs` | `bootstrap/fixtures/diagnostics/missing_function_paren.json` | implemented | normative | |
| DIAG-016 | Integer overflow diagnostics | `native/src/diagnostics.rs` | `bootstrap/fixtures/diagnostics/integer_overflow.json` | implemented | normative | |
| DIAG-017 | Deterministic diagnostic snapshots | `native/src/diagnostics.rs` | `native/tests/core.rs` | implemented | normative | |
| DIAG-018 | Span coverage validation | `native/src/lexer.rs` | `bootstrap/fixtures/parser/span_coverage.zp` | implemented | normative | |

---

## 9. Standard Library

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| STD-001 | text domain (8 KiB limit, pure) | `native/src/stdlib.rs` | `native/tests/core.rs` | implemented | normative | |
| STD-002 | math domain (bounded int, pure) | `native/src/stdlib.rs` | `native/tests/core.rs` | implemented | normative | |
| STD-003 | collections domain (8 MiB, pure) | `native/src/stdlib_collection.rs` | `native/tests/core.rs` | implemented | normative | |
| STD-004 | filesystem domain (8 MiB, external-io) | `native/src/stdlib_file.rs` | `native/tests/core.rs` | implemented | normative | |
| STD-005 | web domain (64 KiB, pure) | `native/src/stdlib.rs` | `native/tests/core.rs` | implemented | normative | |
| STD-006 | json domain (8 MiB, pure) | `native/src/stdlib_json.rs` | `native/tests/core.rs` | implemented | normative | |
| STD-007 | system domain (8 KiB, runtime-dependent) | `native/src/stdlib_system.rs` | `native/tests/core.rs` | implemented | normative | |
| STD-008 | time domain (runtime-dependent) | `native/src/stdlib.rs` | `native/tests/core.rs` | implemented | normative | |
| STD-009 | logging domain (external-io) | `native/src/stdlib.rs` | `native/tests/core.rs` | implemented | normative | |
| STD-010 | runtime domain (runtime-dependent) | `native/src/stdlib.rs` | `native/tests/core.rs` | implemented | normative | |
| STD-011 | async domain (runtime-dependent) | `native/src/async_runtime.rs` | `native/tests/core.rs` | implemented | normative | |
| STD-012 | network domain (8 MiB, external-io) | `native/src/stdlib.rs` | `native/tests/core.rs` | implemented | normative | |
| STD-013 | process domain (1 MiB, external-io) | `native/src/stdlib_system.rs` | `native/tests/core.rs` | implemented | normative | |
| STD-014 | Stability labels (stable/experimental/deprecated) | `native/src/stdlib_catalog.rs` | `docs/STDLIB_POLICY_EN.md` | implemented | normative | |
| STD-015 | Determinism taxonomy (pure/input-deterministic/runtime-dependent/external-io) | `native/src/stdlib_catalog.rs` | `docs/STDLIB_POLICY_EN.md` | implemented | normative | |
| STD-016 | Stdlib catalog regression gate | `native/src/stdlib_catalog.rs` | `native/tests/core.rs#public_builtin_catalog_is_unique_and_domain_grouped` | implemented | normative | |
| STD-017 | Stdlib fixture catalog | `native/src/stdlib_catalog.rs` | `bootstrap/fixtures/stdlib/catalog.json` | implemented | normative | |
| STD-018 | Stdlib JSON security corpus | `native/src/stdlib_json.rs` | `native/tests/core.rs#json_security_corpus_is_deterministic_and_panic_free` | implemented | normative | |
| STD-019 | Stdlib filesystem atomic write | `native/src/stdlib_file.rs` | `native/tests/core.rs#filesystem_metadata_and_atomic_write_are_deterministic` | implemented | normative | |

---

## 10. Package Resolver

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| PKG-001 | Local module resolution (main dir, modules/, lib/) | `native/src/project.rs` | `native/tests/core.rs` | implemented | normative | |
| PKG-002 | Explicit import/export visibility | `native/src/project.rs` | `native/tests/core.rs` | implemented | normative | |
| PKG-003 | Canonical-path module cache (load once) | `native/src/project.rs` | `native/tests/core.rs` | implemented | normative | |
| PKG-004 | Circular import detection | `native/src/project.rs` | `native/tests/core.rs` | implemented | normative | |
| PKG-005 | Absolute path rejection | `native/src/project.rs` | `native/tests/core.rs` | implemented | normative | |
| PKG-006 | ../ traversal rejection (project root boundary) | `native/src/project.rs` | `native/tests/core.rs` | implemented | normative | |
| PKG-007 | Export symbol isolation (private by default) | `native/src/project.rs` | `native/tests/core.rs` | implemented | normative | |
| PKG-008 | Module manifest validation | `native/src/project.rs` | `native/tests/core.rs#validates_explicit_module_manifest_entries` | implemented | normative | |
| PKG-009 | Relative path resolution rules | `native/src/project.rs` | `native/tests/core.rs` | implemented | normative | |
| PKG-010 | Package name resolution | `native/src/project.rs` | `native/tests/core.rs` | implemented | normative | |
| PKG-011 | Standard module resolution order | `native/src/project.rs` | `native/tests/core.rs` | implemented | normative | |
| PKG-012 | Dependency graph cycle detection | `native/src/project.rs` | `native/tests/core.rs` | implemented | normative | |

---

## 11. Lockfile

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| LOCK-001 | zap.lock v2 schema | `native/src/project.rs` | `native/tests/core.rs` | implemented | normative | |
| LOCK-002 | SHA-256 checksum verification | `native/src/project.rs` | `native/tests/core.rs` | implemented | normative | |
| LOCK-003 | Malformed lockfile rejection | `native/src/project.rs` | `native/tests/core.rs#malformed_lockfile_corpus_is_deterministic_and_panic_free` | implemented | normative | |
| LOCK-004 | Lockfile generation | `native/src/project.rs` | `native/tests/core.rs#write_lockfile` | implemented | normative | |
| LOCK-005 | Reproducible install (--locked) | `native/src/project.rs` | `scripts/verify_clean_machine_locked.sh` | implemented | normative | |
| LOCK-006 | Tampered lockfile rejection | `native/src/project.rs` | `scripts/verify_clean_machine_locked.sh` | implemented | normative | |
| LOCK-007 | Lockfile round-trip determinism | `native/src/project.rs` | `native/tests/core.rs` | implemented | normative | |

---

## 12. CLI / LSP

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| CLI-001 | zap run command | `native/src/cli.rs` | `native/tests/core.rs` | implemented | normative | |
| CLI-002 | zap init command | `native/src/cli.rs` | `native/tests/core.rs` | implemented | normative | |
| CLI-003 | zap build command | `native/src/cli.rs` | `native/tests/core.rs` | implemented | normative | |
| CLI-004 | zap test command | `native/src/cli.rs` | `native/tests/core.rs` | implemented | normative | |
| CLI-005 | zap check / zap check --json | `native/src/cli.rs` | `native/tests/core.rs` | implemented | normative | |
| CLI-006 | zap fmt command | `native/src/cli.rs` | `native/tests/core.rs` | implemented | normative | |
| CLI-007 | zap lint command | `native/src/cli.rs` | `native/tests/core.rs` | implemented | normative | |
| CLI-008 | zap --version output | `native/src/cli.rs` | `scripts/validate_release_version.sh` | implemented | normative | |
| CLI-009 | CLI exit codes (0/1/2) | `native/src/cli.rs` | `native/tests/core.rs` | implemented | normative | |
| CLI-010 | Project JSON output | `native/src/main.rs` | `native/tests/core.rs#print_project_json` | implemented | normative | |
| LSP-001 | LSP document sync (full/incremental) | `native/src/lsp.rs` | `native/tests/core.rs` | implemented | normative | |
| LSP-002 | LSP position encoding negotiation | `native/src/lsp.rs` | `native/src/lsp.rs#negotiate_position_encoding` | implemented | normative | |
| LSP-003 | LSP rename (file-local scope-aware) | `native/src/lsp.rs` | `native/src/lsp.rs#rename_response` | implemented | normative | |
| LSP-004 | LSP hover (generic-aware) | `native/src/lsp.rs` | `native/src/lsp.rs#hover_includes_generic_function_type_parameters` | implemented | normative | |
| LSP-005 | LSP document symbols (generic-aware) | `native/src/lsp.rs` | `native/src/lsp.rs#document_symbols_include_generic_type_parameters` | implemented | normative | |
| LSP-006 | LSP signature help (generic-aware) | `native/src/lsp.rs` | `native/src/lsp.rs#signature_help_includes_generic_function_declaration` | implemented | normative | |
| LSP-007 | LSP completion (catalog-driven) | `native/src/lsp.rs` | `native/tests/core.rs` | implemented | normative | |
| LSP-008 | LSP cross-file limitation (explicit) | `native/src/lsp.rs` | `native/tests/core.rs` | implemented | normative | |
| LSP-009 | LSP didClose cleanup | `native/src/lsp.rs` | `native/tests/core.rs` | implemented | normative | |
| LSP-010 | LSP UTF-8/16/32 support | `native/src/lsp.rs` | `native/tests/core.rs` | implemented | normative | |

---

## 13. VM / Bytecode

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| VM-001 | Stack-based bytecode VM (arithmetic) | `native/src/bytecode.rs` | `bootstrap/fixtures/vm/vm_demo.json` | partial | normative | Arithmetic only |
| VM-002 | VM variable load/store | — | — | not-implemented | — | |
| VM-003 | VM function call | — | — | not-implemented | — | |
| VM-004 | VM class instantiation | — | — | not-implemented | — | |
| VM-005 | VM map/index operations | — | — | not-implemented | — | |
| VM-006 | VM for/while loop execution | — | — | not-implemented | — | |
| VM-007 | VM try-catch execution | — | — | not-implemented | — | |
| VM-008 | VM demo fixtures | `native/src/bytecode.rs` | `bootstrap/fixtures/vm/call_arity.zp` | implemented | normative | |
| VM-009 | VM class instance fixture | `native/src/bytecode.rs` | `bootstrap/fixtures/vm/class_instance.zp` | implemented | normative | |
| VM-010 | VM closure fixture | `native/src/bytecode.rs` | `bootstrap/fixtures/vm/closure.zp` | implemented | normative | |
| VM-011 | VM error semantics fixture | `native/src/bytecode.rs` | `bootstrap/fixtures/vm/error_semantics.zp` | implemented | normative | |
| VM-012 | VM member mutation fixture | `native/src/bytecode.rs` | `bootstrap/fixtures/vm/member_mutation.zp` | implemented | normative | |
| VM-013 | VM variable store fixture | `native/src/bytecode.rs` | `bootstrap/fixtures/vm/variable_store.zp` | implemented | normative | |
| VM-014 | Cross-version AST reader | `native/src/bootstrap.rs` | `scripts/test_cross_version_ast_reader.sh` | implemented | normative | |
| VM-015 | Bytecode platform verification | `native/src/bootstrap.rs` | `scripts/bootstrap/verify_vm_platform.sh` | implemented | normative | |

---

## 14. Runtime State / Memory

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| MEM-001 | Rc<RefCell> ownership model | `native/src/value.rs` | `docs/MEMORY_MODEL_EN.md` | implemented | normative | |
| MEM-002 | Checked try_borrow/try_borrow_mut | `native/src/value.rs` | `native/tests/core.rs#conflicting_object_borrows_return_typed_failures` | implemented | normative | |
| MEM-003 | clear_object_fields explicit cycle breaking | `native/src/value.rs` | `native/tests/core.rs#cyclic_object_graph_can_be_explicitly_broken` | implemented | normative | |
| MEM-004 | memory_stats() bounded diagnostic | `native/src/value.rs` | `native/tests/core.rs` | implemented | normative | |
| MEM-005 | Object allocation/deallocation counters | `native/src/value.rs` | `native/tests/core.rs` | implemented | normative | |
| MEM-006 | Cycle-safe value equality | `native/src/value.rs` | `native/tests/core.rs#value_equality_is_cycle_safe_and_borrow_checked` | implemented | normative | |
| MEM-007 | Logical memory budget (byte/task/output) | `native/src/runtime_state.rs` | `native/tests/core.rs` | implemented | normative | |
| MEM-008 | ObjectStore isolation and reset | `native/src/runtime_state.rs` | `native/tests/core.rs` | implemented | normative | |
| MEM-009 | Per-context validation/cleanup counters | `native/src/runtime_state.rs` | `native/tests/core.rs` | implemented | normative | |
| MEM-010 | Text value limit (8 MiB) | `native/src/value.rs` | `native/tests/core.rs` | implemented | normative | |
| MEM-011 | Collection item limit (100,000) | `native/src/value.rs` | `native/tests/core.rs` | implemented | normative | |
| MEM-012 | Value graph node limit (100,000) | `native/src/value.rs` | `native/tests/core.rs` | implemented | normative | |
| MEM-013 | Public weak references | — | — | not-implemented | — | Explicitly unsupported |
| MEM-014 | Tracing garbage collector | — | — | not-implemented | — | Explicitly not implemented |
| MEM-015 | Deterministic byte/output rollback | `native/src/runtime_state.rs` | `native/tests/core.rs` | implemented | normative | |

---

## 15. Async Runtime

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| ASYNC-001 | Deterministic single-threaded executor | `native/src/async_runtime.rs` | `native/tests/core.rs` | implemented | normative | |
| ASYNC-002 | ScheduledFuture task handles | `native/src/async_runtime.rs` | `native/tests/core.rs` | implemented | normative | |
| ASYNC-003 | Explicit task states (Pending/Ready/Cancelled/TimedOut/Joined) | `native/src/async_runtime.rs` | `native/tests/core.rs` | implemented | normative | |
| ASYNC-004 | One-time task-budget release on join | `native/src/async_runtime.rs` | `native/tests/core.rs` | implemented | normative | |
| ASYNC-005 | Cooperative cancellation | `native/src/async_runtime.rs` | `native/tests/core.rs` | implemented | normative | |
| ASYNC-006 | Poll-budget timeout | `native/src/async_runtime.rs` | `native/tests/core.rs` | implemented | normative | |
| ASYNC-007 | async_capabilities() builtin | `native/src/evaluator.rs` | `native/tests/core.rs` | implemented | normative | |
| ASYNC-008 | UnknownTask / AlreadyJoined diagnostics | `native/src/async_runtime.rs` | `native/tests/core.rs` | implemented | normative | |
| ASYNC-009 | Production I/O boundary (file/TCP/process adapters) | `native/src/async_runtime.rs` | `docs/ASYNC_BOUNDARIES_EN.md` | implemented | normative | Bounded adapters only |
| ASYNC-010 | Scheduler reset detachment | `native/src/async_runtime.rs` | `native/tests/core.rs` | implemented | normative | |
| ASYNC-011 | Cross-platform async matrix | `native/src/async_runtime.rs` | `scripts/test_p005c_async_matrix.sh` | implemented | normative | |
| ASYNC-012 | Threaded runtime rejection of invalid limits | `native/src/async_runtime.rs` | `native/tests/core.rs` | implemented | normative | |
| ASYNC-013 | Full reactor semantics (OS-level) | — | — | not-implemented | — | Explicitly out of scope |

---

## 16. Registry / Supply Chain

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| REG-001 | Registry index parsing | `native/src/registry.rs` | `native/tests/core.rs` | implemented | normative | |
| REG-002 | Signed index verification | `native/src/registry.rs` | `native/tests/core.rs` | implemented | normative | |
| REG-003 | Signed index mutation rejection | `native/src/registry.rs` | `native/tests/core.rs#security_property_signed_index_mutations_never_panic_or_accept_tampering` | implemented | normative | |
| REG-004 | Trusted registry policy | `native/src/registry.rs` | `docs/REGISTRY_AUTH_EN.md` | implemented | normative | |
| REG-005 | Credential scoping and resolution | `native/src/registry.rs` | `native/tests/core.rs` | implemented | normative | |
| REG-006 | Authentication diagnostics (ZAP-REG-AUTH-*) | `native/src/registry.rs` | `native/tests/core.rs` | implemented | normative | |
| REG-007 | Chunked response body handling | `native/src/registry.rs` | `native/tests/core.rs` | implemented | normative | |
| REG-008 | Truncated Content-Length rejection | `native/src/registry.rs` | `native/tests/core.rs` | implemented | normative | |
| REG-009 | Oversized response rejection | `native/src/registry.rs` | `native/tests/core.rs` | implemented | normative | |
| REG-010 | Slow-peer timeout normalization | `native/src/registry.rs` | `native/tests/core.rs` | implemented | normative | |
| REG-011 | Yanked release handling | `native/src/registry.rs` | `native/tests/core.rs` | implemented | normative | |
| REG-012 | Registry cache policy | `native/src/registry.rs` | `native/tests/core.rs` | implemented | normative | |
| REG-013 | Checksum mismatch rejection | `native/src/registry.rs` | `native/tests/core.rs` | implemented | normative | |

---

## 17. Release Engineering

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| REL-001 | Cargo-authoritative version | `native/Cargo.toml` | `scripts/validate_release_version.sh` | implemented | normative | |
| REL-002 | Version surface consistency (CLI/tag/changelog/README) | `scripts/validate_release_version.sh` | `scripts/test_validate_release_version.sh` | implemented | normative | |
| REL-003 | Cross-platform release workflow | `.github/workflows/release.yml` | GitHub Actions | implemented | normative | |
| REL-004 | SHA-256 checksum generation | `.github/workflows/release.yml` | GitHub Actions | implemented | normative | |
| REL-005 | Release preflight gate | `scripts/release_preflight.sh` | CI artifact | implemented | normative | |
| REL-006 | Signed release assets | `.github/workflows/release.yml` | GitHub Actions | implemented | normative | |
| REL-007 | Bilingual release notes | `docs/RELEASE_<VERSION>_EN.md` | `scripts/validate_release_version.sh` | implemented | normative | |

---

## 18. Self-Rebuild Acceptance

| ID | Rule | Owner | Fixture/Test | Status | Compatibility | Notes |
|---|---|---|---|---|---|---|
| REBUILD-001 | Byte-for-byte token determinism | `scripts/bootstrap/verify_b4_byte_determinism.sh` | `bootstrap/b4/native_independent.zp` | implemented | normative | |
| REBUILD-002 | Byte-for-byte AST determinism | `scripts/bootstrap/verify_b4_byte_determinism.sh` | `bootstrap/b4/native_independent.zp` | implemented | normative | |
| REBUILD-003 | Byte-for-byte typed IR determinism | `scripts/bootstrap/verify_b4_byte_determinism.sh` | `bootstrap/b4/native_independent.zp` | implemented | normative | |
| REBUILD-004 | Byte-for-byte bytecode determinism | `scripts/bootstrap/verify_b4_byte_determinism.sh` | `bootstrap/b4/native_independent.zp` | implemented | normative | |
| REBUILD-005 | Byte-for-byte pipeline determinism | `scripts/bootstrap/verify_b4_byte_determinism.sh` | `bootstrap/b4/native_independent.zp` | implemented | normative | |
| REBUILD-006 | Multi-line source determinism | `scripts/bootstrap/verify_b4_byte_determinism.sh` | `bootstrap/b4/native_independent.zp` | implemented | normative | |
| REBUILD-007 | Control-flow source determinism | `scripts/bootstrap/verify_b4_byte_determinism.sh` | `bootstrap/b4/native_independent.zp` | implemented | normative | |
| REBUILD-008 | Stage 1 (source→bytecode) determinism | `scripts/bootstrap/verify_b4_second_stage_rebuild.sh` | `bootstrap/b4/native_independent.zp` | implemented | normative | |
| REBUILD-009 | Stage 2 (bytecode→execution) determinism | `scripts/bootstrap/verify_b4_second_stage_rebuild.sh` | `bootstrap/b4/native_independent.zp` | implemented | normative | |
| REBUILD-010 | Full pipeline replay determinism | `scripts/bootstrap/verify_b4_second_stage_rebuild.sh` | `bootstrap/b4/native_independent.zp` | implemented | normative | |
| REBUILD-011 | Cross-stage execution determinism | `scripts/bootstrap/verify_b4_second_stage_rebuild.sh` | `bootstrap/b4/native_independent.zp` | implemented | normative | |
| REBUILD-012 | Typed IR second-stage determinism | `scripts/bootstrap/verify_b4_second_stage_rebuild.sh` | `bootstrap/b4/native_independent.zp` | implemented | normative | |
| REBUILD-013 | Self-rebuild typed IR determinism | `scripts/bootstrap/verify_b4_second_stage_rebuild.sh` | `bootstrap/b4/native_independent.zp` | implemented | normative | |
| REBUILD-014 | No Rust toolchain dependency | `scripts/bootstrap/verify_b4_clean_environment.sh` | `bootstrap/b4/native_independent.zp` | implemented | normative | |
| REBUILD-015 | Normal env matches clean env | `scripts/bootstrap/verify_b4_clean_environment.sh` | `bootstrap/b4/native_independent.zp` | implemented | normative | |
| REBUILD-016 | No state leakage between runs | `scripts/bootstrap/verify_b4_clean_environment.sh` | `bootstrap/b4/native_independent.zp` | implemented | normative | |
| REBUILD-017 | Platform evidence validation | `scripts/bootstrap/verify_b4_clean_environment.sh` | `bootstrap/b4/seed_pipeline.zp` | implemented | normative | |
| REBUILD-018 | Diverse source surface execution | `scripts/bootstrap/verify_b4_clean_environment.sh` | `bootstrap/b4/native_independent.zp` | implemented | normative | |

---

## Summary

| Domain | Implemented | Partial | Deferred | Not-Implemented |
|---|---|---|---|---|
| Grammar (LANG) | 23 | 0 | 0 | 0 |
| Generics (GEN) | 16 | 0 | 5 | 1 |
| Alias Environment (ALIAS) | 9 | 0 | 0 | 0 |
| Dataflow (FLOW) | 24 | 0 | 0 | 0 |
| Overloads (OVR) | 0 | 0 | 0 | 3 |
| Traits/MRO (TRAIT) | 4 | 0 | 2 | 5 |
| Closures (CLOS) | 8 | 0 | 0 | 0 |
| Diagnostics (DIAG) | 18 | 0 | 0 | 0 |
| Standard Library (STD) | 19 | 0 | 0 | 0 |
| Package Resolver (PKG) | 12 | 0 | 0 | 0 |
| Lockfile (LOCK) | 7 | 0 | 0 | 0 |
| CLI/LSP (CLI/LSP) | 20 | 0 | 0 | 0 |
| VM/Bytecode (VM) | 8 | 1 | 0 | 6 |
| Runtime State/Memory (MEM) | 13 | 0 | 0 | 2 |
| Async Runtime (ASYNC) | 12 | 0 | 0 | 1 |
| Registry (REG) | 13 | 0 | 0 | 0 |
| Release Engineering (REL) | 7 | 0 | 0 | 0 |
| Self-Rebuild Acceptance (REBUILD) | 18 | 0 | 0 | 0 |
| **Total** | **231** | **1** | **7** | **18** |

---

## Validation

The machine-readable TSV is validated by `scripts/validate_spec_ownership.sh` which checks:
- Every rule ID is unique
- Every owner path exists in the repository
- Every fixture path exists
- Required domains are represented
- Status and compatibility values are valid

Run locally:
```bash
ZAP_SPEC_OWNERSHIP_REPORT=target/language-ownership-report.tsv scripts/validate_spec_ownership.sh
```

---

## Related documents

| Document | Purpose |
|---|---|
| [`SPEC_OWNERSHIP_INDEX.tsv`](SPEC_OWNERSHIP_INDEX.tsv) | Existing 49-rule ownership index (superseded by this matrix) |
| [`LANGUAGE_SPEC_EN.md`](LANGUAGE_SPEC_EN.md) | Canonical language specification |
| [`TYPECHECK_CONFORMANCE_MATRIX_EN.md`](TYPECHECK_CONFORMANCE_MATRIX_EN.md) | Type-checking conformance (TC-001 to TC-012) |
| [`MEMORY_MODEL_EN.md`](MEMORY_MODEL_EN.md) | Memory ownership contract |
| [`ASYNC_BOUNDARIES_EN.md`](ASYNC_BOUNDARIES_EN.md) | Async boundary contract |
| [`STDLIB_POLICY_EN.md`](STDLIB_POLICY_EN.md) | Standard-library stability policy |
| [`DIAGNOSTIC_MODEL_EN.md`](DIAGNOSTIC_MODEL_EN.md) | Structured diagnostic model |
| [`REGISTRY_AUTH_EN.md`](REGISTRY_AUTH_EN.md) | Registry authentication |
| [`RELEASE_VERSION_POLICY_EN.md`](RELEASE_VERSION_POLICY_EN.md) | Release version policy |
| [`bootstrap/contracts/OWNERS.tsv`](../bootstrap/contracts/OWNERS.tsv) | Bootstrap stage ownership (BOOT-001 to B4-003) |