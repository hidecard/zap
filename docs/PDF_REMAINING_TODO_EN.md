# Zap Remaining Engineering To-do Register

**Baseline:** Zap v2.1.6 verified release  
**Source:** `Zap_တွင်_ပြင်ဆင်သင့်သောအချက်များ.pdf`  
**Purpose:** Track every recommendation that is not fully complete, without treating already-verified release work as unfinished.

**Detailed execution plan:** [`NEXT_TODO_PLAN_EN.md`](NEXT_TODO_PLAN_EN.md) defines the milestone sequence, implementation tasks, acceptance evidence, and release gates for the remaining work.

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
| P0-02 | Consolidated language specification | Partial | `LANGUAGE_SPEC_EN.md` is now the canonical semantic index for syntax, precedence, typing, runtime behavior, compatibility, and version ownership. Full migration of fragmented rules and complete conformance fixtures remain. |
| P0-03 | Structured diagnostics | Done | CLI JSON and LSP diagnostics expose stable `ZAP-*` code, kind, severity, normalized message, source span, deterministic notes/help, and deterministic snapshot/regression coverage. |
| P0-04 | Memory and reference-cycle contract | Partial | `Rc<RefCell>` ownership policy, explicit non-thread-safe boundary, tracked `Value::object`, `clear_object_fields`, `object_field_count`, bounded `memory_stats()`, object allocation/deallocation counters, cycle-safe value validation, and deterministic memory-limit tests are implemented. Public weak references, closure-level/process-wide telemetry, automatic arbitrary-cycle reclamation, and tracing collection remain future work. |
| P0-05 | Deterministic versus production async boundary | Partial | The deterministic executor, fixed-worker, bounded network/process adapters, cancellation behavior, and descriptive `async_capabilities()` report are documented and exposed. Resource-limit preflight validation, executor-backed language-level scheduling, language-level cancellation/timeout controls, cross-platform async matrix coverage, and tooling synchronization remain. |

## P1 — Production readiness

| ID | Work item | Status | Acceptance criteria |
|---|---|---|---|
| P1-01 | Gradual type checking completion | Completed | Annotation enforcement, collection element typing, runtime mismatch diagnostics, control-flow narrowing, structured diagnostics, TC-001–TC-012 conformance evidence, and generic/inference limits are documented and tested in the bilingual type-system contracts. |
| P1-02 | Benchmark and profiling harness | Completed | Dependency-free repeatable harness covers loops, user-defined calls, captured-state closures, collection allocations, JSON conversion, deterministic async scheduling, and explicit module/import dispatch with CSV output. `scripts/aggregate_benchmark.sh` provides deterministic min/mean/max summaries, and CI runs a seven-suite smoke with artifact upload without timing thresholds. |
| P1-03 | Registry supply-chain hardening | Completed | Redaction, traversal, wrong-key/mutated-payload fail-closed tests, protected-release provenance identity checks, adversarial signed-provenance mutation coverage, full-fingerprint signing-key rotation allowlist, yanked metadata parsing/resolution enforcement, unauthorized publish rejection, invalid package identity rejection, and publish checksum mismatch rejection are implemented. Exact and range resolution skip yanked candidates, malformed yanked metadata fails closed, stable exact/range all-yanked diagnostics are covered, and the end-to-end locked-cache audit verifies manifest requirement matching, checksum integrity, explicitly locked yanked-cache reuse, offline operation, and tampered lock/cache rejection. |
| P1-04 | Deterministic package resolution | Completed | `scripts/verify_clean_machine_locked.sh` proves clean-copy repeatability for `zap install --locked` and `zap build --locked` without registry access and rejects a tampered `zap.lock`. |
| P1-05 | Conformance/property/fuzz test layers | Partial | Parser golden-style unit tests, deterministic parser/lexer/JSON/lockfile/registry-security corpora, collection/filesystem regressions, async cancellation/scheduler cases, and a seven-case deterministic fuzz-style CLI mutation corpus are now CI-visible through `scripts/test_p105_layers.sh`; malformed mutations reject safely without panic. Linux, Windows, and macOS build/test matrix coverage remains active. Long-running fuzz targets, allocator/heap-level counters, and additional platform-specific input cases remain to be added. |

## P2 — Long-term language and ecosystem

| ID | Work item | Status | Acceptance criteria |
|---|---|---|---|
| P2-01 | Composition and traits/interfaces | Deferred | A design RFC defines composition, trait/protocol rules, method resolution, diagnostics, and migration from inheritance before implementation. |
| P2-02 | Standard-library API stability policy | Partial | Every public module has stability label, deprecation period, semantic-versioning rule, and platform support matrix. |
| P2-03 | LSP/VS Code semantic parity | Partial | Rename, nested/module-aware indexing, async-aware completion/hover, and canonical parser/AST coverage are tested. |
| P2-04 | Learning/reference documentation split | Partial | Beginner guide, syntax reference, specification, stdlib reference, package author guide, runtime internals, and deployment/security docs have navigation and verified-version metadata. |

## Execution order

1. **P0-04:** Extend the implemented memory contract with heap statistics, allocation counters, weak-reference diagnostics, and closure-cycle coverage.
2. **P0-05:** Document deterministic async limitations and production boundaries.
3. **P1-05:** Expand parser golden, property, fuzz, memory, security, and platform-specific test layers.
4. **P1-02:** Add the benchmark/profiling harness before making performance claims.
5. **P1-03:** Completed. Registry redaction, fail-closed, traversal, provenance, key-rotation, yanked-release, and end-to-end locked-cache tests enforce signed tag, commit, workflow, HTTPS source, checksum, full signing-fingerprint, explicit trusted-fingerprint allowlist, adversarial signed-provenance mutation rejection, yanked-candidate skipping, malformed-yanked rejection, stable exact/range all-yanked diagnostics, manifest requirement matching, offline cache reuse, and tampered lock/cache rejection.
6. **P1-05:** Add parser golden, property, fuzz, memory, and security test layers.
7. **P1-01/P1-04:** Completed. The bilingual gradual-typing baseline is documented and the clean-machine locked-install/build verifier is executable and deterministic.
8. **P2-02/P2-03/P2-04:** Finish stdlib policy, tooling parity, and documentation navigation.
9. **P2-01:** Write and review the traits/composition RFC before changing the parser or runtime.

## Release policy

Each execution step must pass formatting, strict Clippy in the pinned CI toolchain, the full native test suite, relevant conformance tests, bilingual documentation parity, and `git diff --check`. No new release tag should be created until the corresponding acceptance criteria and CI gates pass.
