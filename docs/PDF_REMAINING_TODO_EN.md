# Zap Remaining Engineering To-do Register

**Baseline:** Zap v2.1.6 verified release  
**Source:** `Zap_တွင်_ပြင်ဆင်သင့်သောအချက်များ.pdf`  
**Purpose:** Track every recommendation that is not fully complete, without treating already-verified release work as unfinished.

## Status legend

| Status | Meaning |
|---|---|
| Done | Implemented and covered by code/tests or release evidence. |
| Partial | A usable foundation exists, but the PDF recommendation still has important gaps. |
| Todo | Not implemented or not yet supported by repository evidence. |
| Deferred | Deliberately scheduled for a later language-design milestone because it changes core semantics or deployment architecture. |

## P0 — Reliability foundation

| ID | Work item | Status | Acceptance criteria |
|---|---|---|---|
| P0-01 | Canonical native/legacy conformance contract | Partial | Native behavior is canonical; legacy fixtures have a parity report, migration policy, and CI conformance command. |
| P0-02 | Consolidated language specification | Partial | One normative specification defines syntax, precedence, typing, runtime behavior, compatibility, and version ownership. |
| P0-03 | Structured diagnostics | Partial | All user-facing errors expose severity, stable code, message, source span, notes/help, and snapshot-tested output. |
| P0-04 | Memory and reference-cycle contract | Todo | Document `Rc<RefCell>` policy; add cycle regression tests and an explicit non-thread-safe boundary. |
| P0-05 | Deterministic versus production async boundary | Partial | Deterministic executor is documented separately; production I/O, blocking-call, cancellation, and scheduling boundaries are explicit. |

## P1 — Production readiness

| ID | Work item | Status | Acceptance criteria |
|---|---|---|---|
| P1-01 | Gradual type checking completion | Partial | Annotation enforcement, collection element typing, runtime mismatch diagnostics, and generic/inference limits are documented and tested. |
| P1-02 | Benchmark and profiling harness | Todo | Repeatable benchmarks cover loops, calls, closures, allocation, dispatch, imports, JSON, and async scheduling. |
| P1-03 | Registry supply-chain hardening | Partial | Redaction tests, traversal/security fuzzing, signature/checksum fail-closed tests, provenance policy, key rotation, and yanked-release rules exist. |
| P1-04 | Deterministic package resolution | Partial | Clean-machine `zap install --locked` and `zap build --locked` produce verified, reproducible results. |
| P1-05 | Conformance/property/fuzz test layers | Partial | Parser golden tests, property tests, fuzz targets, memory regressions, async determinism, security inputs, and cross-platform cases are CI-visible. |

## P2 — Long-term language and ecosystem

| ID | Work item | Status | Acceptance criteria |
|---|---|---|---|
| P2-01 | Composition and traits/interfaces | Deferred | A design RFC defines composition, trait/protocol rules, method resolution, diagnostics, and migration from inheritance before implementation. |
| P2-02 | Standard-library API stability policy | Partial | Every public module has stability label, deprecation period, semantic-versioning rule, and platform support matrix. |
| P2-03 | LSP/VS Code semantic parity | Partial | Rename, nested/module-aware indexing, async-aware completion/hover, and canonical parser/AST coverage are tested. |
| P2-04 | Learning/reference documentation split | Partial | Beginner guide, syntax reference, specification, stdlib reference, package author guide, runtime internals, and deployment/security docs have navigation and verified-version metadata. |

## Execution order

1. **P0-03:** Finish the structured diagnostic schema and snapshot fixtures.
2. **P0-04:** Define the memory contract and add object/closure cycle regressions.
3. **P0-05:** Document deterministic async limitations and production boundaries.
4. **P1-02:** Add the benchmark/profiling harness before making performance claims.
5. **P1-03:** Add registry redaction, fail-closed, traversal, and provenance tests.
6. **P1-05:** Add parser golden, property, fuzz, memory, and security test layers.
7. **P1-01/P1-04:** Complete gradual-typing documentation and clean-machine locked-install verification.
8. **P2-02/P2-03/P2-04:** Finish stdlib policy, tooling parity, and documentation navigation.
9. **P2-01:** Write and review the traits/composition RFC before changing the parser or runtime.

## Release policy

Each execution step must pass formatting, strict Clippy in the pinned CI toolchain, the full native test suite, relevant conformance tests, bilingual documentation parity, and `git diff --check`. No new release tag should be created until the corresponding acceptance criteria and CI gates pass.
