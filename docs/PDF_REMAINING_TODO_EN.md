# Zap Remaining Engineering To-do Register

**Baseline:** Zap v2.1.14 verified release
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
| P0-01 | Canonical native/legacy conformance contract | Partial — P0-01-A implemented | Native behavior is canonical. P0-01-A adds a versioned six-case matrix with `common`, `native-only`, and `rejected` policies, normalized stdout digests, deterministic tab-separated reports, migration guidance, and a CI parity artifact. Broader legacy inventory and future drift approvals remain. |
| P0-02 | Consolidated language specification | Partial — ownership expansion slice implemented | `LANGUAGE_SPEC_EN.md` and `LANGUAGE_SPEC_MM.md` remain the canonical bilingual semantic index. The ownership index now contains 28 stable rule IDs across required semantic domains, validates unique IDs and domain coverage, maps fixture/test owners, and includes bilingual compatibility/deprecation templates. Expansion to every remaining fragmented rule and complete conformance fixture ownership remains. Release preflight now runs the ownership gate together with parity, replay, and focused async contract gates. |
| P0-03 | Structured diagnostics | Done | CLI JSON and LSP diagnostics expose stable `ZAP-*` code, kind, severity, normalized message, source span, deterministic notes/help, and deterministic snapshot/regression coverage. |
| P0-04 | Memory and reference-cycle contract | Partial — M2-MEM-02/M2-FN-02 lifecycle and closure hardening implemented | `Rc<RefCell>` ownership policy, explicit non-thread-safe boundary, tracked `Value::object`, checked object-field `try_borrow`/`try_borrow_mut` accessors, stable `ZAP-BORROW-001` diagnostics, fallible `clear_object_fields`/`object_field_count`, bounded `memory_stats()`, object allocation/deallocation counters, cycle-safe value validation, and deterministic memory-limit tests are implemented. Public weak references, closure-level/process-wide telemetry, allocator-level measurement, automatic arbitrary-cycle reclamation, and tracing collection remain future work. M2-MEM-02 adds run-owned logical byte/task/output reservation APIs, deterministic object charges, validation/cleanup lifecycle counters, reset detachment, and `ObjectStore` counters with isolation/reset evidence. |
| P0-RS-01 | Explicit `RuntimeState` and `ExecutionContext` boundary | First slice implemented | Each run owns module-cache state, import-cycle tracking, and execution-depth accounting through an explicit context propagated across AST, legacy, function, method, object-field, and module execution. Independent-context isolation/reset regressions and the full native suite provide acceptance evidence. |
| P0-05 | Deterministic versus production async boundary | Partial — M2-ASYNC-01/M2-ASYNC-02 scheduling, cancellation, timeout, and M2-VERIFY-02 platform hardening implemented | The deterministic executor, context-owned `ScheduledFuture` language handles, fixed-worker, bounded network/process adapters, cooperative `task_cancel`, poll-budget `task_join_timeout`, deterministic `Cancelled`/`TimedOut` diagnostics, descriptive `async_capabilities()` report, typed resource-limit preflight validation, TCP request-size admission check, and reproducible Linux/Windows/macOS focused matrix with target-named CI artifacts are documented and exposed. M2-VERIFY-02 executes file newline/directory/symlink, process status/cancellation, socket, and deterministic archive cases on native targets; the Unix-only symlink boundary is documented explicitly. Full reactor semantics and broader tooling synchronization remain. |
| P0-06 | Release version single-source-of-truth gate | Completed — P0 release slice | `native/Cargo.toml` is the authoritative version source. The validator checks Cargo, Cargo.lock, CLI output, optional release tags, changelogs, bilingual README release links/archive names, `SECURITY.md`, conformance metadata, bilingual release notes, the release template, and installer metadata. Deterministic TSV evidence, a positive/negative regression harness, CI artifact upload, release-preflight enforcement, and bilingual policy documentation are in place. |

## P1 — Production readiness

| ID | Work item | Status | Acceptance criteria |
|---|---|---|---|
| P1-01 | Gradual type checking completion | Completed | Annotation enforcement, collection element typing, runtime mismatch diagnostics, control-flow narrowing, structured diagnostics, TC-001–TC-012 conformance evidence, and generic/inference limits are documented and tested in the bilingual type-system contracts. |
| P1-02 | Benchmark and profiling harness | Completed with M2-BENCH-01 provenance/variance slice | Dependency-free repeatable harness covers loops, user-defined calls, captured-state closures, collection allocations, JSON conversion, deterministic async scheduling, and explicit module/import dispatch with raw CSV output. Bounded repeat/warm-up controls emit commit/target/OS/toolchain/binary-digest provenance TSV metadata; `scripts/aggregate_benchmark.sh` provides deterministic min/mean/p95/max plus standard deviation, variance, and coefficient-of-variation summaries. CI and release preflight validate the expanded schema while mean/p95 remain the regression thresholds. |
| P1-03 | Registry supply-chain hardening | Completed | Redaction, traversal, wrong-key/mutated-payload fail-closed tests, protected-release provenance identity checks, adversarial signed-provenance mutation coverage, full-fingerprint signing-key rotation allowlist, yanked metadata parsing/resolution enforcement, unauthorized publish rejection, invalid package identity rejection, and publish checksum mismatch rejection are implemented. Exact and range resolution skip yanked candidates, malformed yanked metadata fails closed, stable exact/range all-yanked diagnostics are covered, and the end-to-end locked-cache audit verifies manifest requirement matching, checksum integrity, explicitly locked yanked-cache reuse, offline operation, and tampered lock/cache rejection. |
| P1-04 | Deterministic package resolution | Completed | `scripts/verify_clean_machine_locked.sh` proves clean-copy repeatability for `zap install --locked` and `zap build --locked` without registry access and rejects a tampered `zap.lock`. |
| P1-05 | Conformance/property/fuzz test layers | Partial — P1-05-A replay, M2-VERIFY-01 bounded replay, and M2-VERIFY-02 platform hardening implemented | Parser golden-style unit tests, deterministic parser/lexer/JSON/lockfile/registry-security corpora, collection/filesystem regressions, async cancellation/scheduler cases, and a seven-case deterministic fuzz-style CLI mutation corpus are CI-visible through `scripts/test_p105_layers.sh`. P1-05-A now adds fixed-seed `ZAP_CORPUS_SEED` replay, six durable parser/JSON/lockfile/registry/memory/async fixture categories, SHA-256/base64 replay evidence, and CI artifact upload. M2-VERIFY-01 adds a default 12-round bounded replay job with a 64-round cap, fail-closed 64 KiB fixture and 8 MiB corpus limits, repeated semantic outcome digests, and TSV/log artifacts in CI and release preflight. M2-VERIFY-02 executes the focused async matrix on native Linux x86_64, Windows x86_64, and macOS ARM64 runners, including target-named logs, Unix symlink coverage where supported, and repeated deterministic archive checks. Unbounded fuzz targets and allocator/heap-level counters remain. |

## P2 — Long-term language and ecosystem

| ID | Work item | Status | Acceptance criteria |
|---|---|---|---|
| P2-01 | Composition and traits/interfaces | Deferred | A design RFC defines composition, trait/protocol rules, method resolution, diagnostics, and migration from inheritance before implementation. |
| P2-02 | Standard-library API stability policy | Partial | Every public module has stability label, deprecation period, semantic-versioning rule, and platform support matrix. |
| P2-03 | LSP/VS Code semantic parity | Partial | Rename, nested/module-aware indexing, async-aware completion/hover, and canonical parser/AST coverage are tested. |
| P2-04 | Learning/reference documentation split | Partial | Beginner guide, syntax reference, specification, stdlib reference, package author guide, runtime internals, and deployment/security docs have navigation and verified-version metadata. |

## Execution order

1. **P0-06:** Completed. The Cargo-authoritative version validator, deterministic evidence, negative drift regression harness, bilingual policy documentation, CI gate, and release-preflight enforcement are implemented.
2. **P1-05-A:** Completed. Fixed-seed property/fuzz replay, durable parser/JSON/lockfile/registry/memory/async failure corpora, replay evidence, and a CI artifact gate are implemented. The broader P1-05 fuzz and platform extensions remain separately tracked.
3. **M2-VERIFY-01:** Completed. Bounded repeated replay, fixture/corpus byte limits, identical semantic outcome digests, and CI/release-preflight TSV/log evidence are implemented.
4. **M2-VERIFY-02:** Completed. Native Linux x86_64, Windows x86_64, and macOS ARM64 matrix execution now covers async adapter edge cases, newline and path boundaries, Unix symlink policy, deterministic archive bytes, and target-named CI logs.
5. **P0-01-A:** Completed as the first executable parity slice. The six-case native/legacy policy matrix, normalized report, migration guidance, and CI artifact gate are implemented; broader legacy inventory remains separately tracked.
6. **P0-02-A:** Ownership expansion slice implemented. The bilingual machine-readable index now covers 28 stable rule IDs, unique-ID/domain validation, fixture/test ownership, compatibility/deprecation templates, and a release-preflight contract gate. Expansion to every remaining fragmented rule remains.
7. **P1-03:** Completed. Registry redaction, fail-closed, traversal, provenance, key-rotation, yanked-release, and end-to-end locked-cache tests enforce signed tag, commit, workflow, HTTPS source, checksum, full signing-fingerprint, explicit trusted-fingerprint allowlist, adversarial signed-provenance mutation rejection, yanked-candidate skipping, malformed-yanked rejection, stable exact/range all-yanked diagnostics, manifest requirement matching, offline cache reuse, and tampered lock/cache rejection.
8. **P0-04/P0-RS-01:** The checked-borrow and first explicit runtime-state slices are implemented. M2-MEM-01 and M2-MEM-02 add run-owned logical byte/task/output budget APIs, deterministic object charges, lifecycle counters, reset detachment, and `ObjectStore` counters without allocator/tracing claims. Continue only the remaining public weak-reference, closure/process-wide telemetry, allocator-level measurement, arbitrary-cycle reclamation, tracing-collector design, and broader hidden-state migration work.
9. **P1-01/P1-04:** Completed. The bilingual gradual-typing baseline is documented and the clean-machine locked-install/build verifier is executable and deterministic.
10. **M2-ASYNC-02:** Completed. Cooperative language task cancellation, poll-budget timeout joins, deterministic `Cancelled`/`TimedOut` diagnostics, bilingual contract updates, and end-to-end regression coverage are implemented.
11. **P2-02/P2-03/P2-04:** Finish stdlib policy, tooling parity, and documentation navigation.
12. **P2-01:** Write and review the traits/composition RFC before changing the parser or runtime.

## Release policy

Each execution step must pass formatting, strict Clippy in the pinned CI toolchain, the full native test suite, relevant conformance tests, bilingual documentation parity, and `git diff --check`. No new release tag should be created until the corresponding acceptance criteria and CI gates pass.
