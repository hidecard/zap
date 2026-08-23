# Zap Next Engineering TODO Plan

**Baseline:** Zap v2.2.6 is the current release candidate following the verified published v2.2.5 release. The candidate commit is `efe44a621251a1d61e85480fced6593b9bd27941`; no v2.2.6 tag or release exists yet. The v2.2.0, v2.2.1, and v2.2.2 tags and assets remain immutable; the later runtime, equality, borrow, and LSP hardening changes are published in v2.2.3, while broader deferred work remains on the roadmap.

**Purpose:** Define the next implementation sequence after the stable diagnostic contract. This plan turns the remaining TODO register into executable milestones without treating already-completed release work as unfinished.

**Current progress:** The first P0-04 slice and the checked object-field borrow-safety sub-slice are implemented: tracked object allocation/deallocation statistics, bounded `memory_stats()` diagnostics, cycle-safe value validation, deterministic value limits, regression tests, and bilingual memory-contract documentation. P0-05-A, P0-05-B, and P0-05-C are also implemented: the descriptive deterministic `async_capabilities()` builtin, catalog entry, typed resource-limit preflight validation, TCP request-size admission checks, and a reproducible Linux x86_64/Windows x86_64/macOS ARM64 focused matrix with target-named CI artifacts. P1-05-A replayable verification is now implemented with a fixed `ZAP_CORPUS_SEED`, six durable failure-corpus categories, deterministic replay logs, and CI artifact upload. M2-VERIFY-01 extends this into a bounded 12-round default CI/release-preflight job with a 64-round cap, fail-closed fixture/corpus byte bounds, repeated SHA-256 semantic outcome comparison, and durable TSV/log evidence. M2-VERIFY-02 hardens the native Linux x86_64, Windows x86_64, and macOS ARM64 matrix with executed async file/process/socket cases, Unix symlink policy coverage, newline preservation, deterministic archive checks, and target-named logs. M2-BENCH-01 adds bounded 1–64 repetition and 0–16 warm-up controls, commit/target/OS/toolchain/binary-digest provenance sidecars, deterministic standard deviation/variance/coefficient-of-variation summaries, and schema/regression contract coverage while keeping mean/p95 thresholds authoritative. M2-REG-01 hardens registry transport with five-second client deadlines, 16 MiB response bounds, explicit truncated/invalid Content-Length rejection, chunked-body support, and stable slow-peer diagnostics backed by focused TCP fixtures. M3-STDLIB-01 defines a machine-readable stability catalog and bilingual policy for every public domain and builtin, including stability labels, introduction releases, deprecation windows, semver rules, platform support, limits, timeout/error contracts, and determinism metadata, with catalog and policy regression gates. M3-LSP-01 adds parser/lexer-backed rename edits, didClose workspace cleanup, nested/module-aware symbol indexing, catalog-driven completion, async builtin hover/signature metadata, a VS Code grammar/configuration package, and a named semantic-parity regression gate. M3-DOC-01 completes the bilingual learner/reference split with navigation hubs, active v2.2.6 metadata, explicit historical v2.2.0 and v2.2.1 provenance, canonical companion links, and repository-relative entry points for learner, syntax, specification, standard-library, package-author, runtime, deployment/security, tooling, and release audiences. M4-RFC-01 is complete as a bilingual design-only traits/composition record, and M4-REL-01 is complete with the v2.2.0 tag, signed cross-platform release assets, published checksums/provenance, and successful release-workflow verification. P0-01-A executable native/legacy parity is now implemented with a versioned six-case policy matrix, normalized output digests, deterministic reports, and CI artifact upload. P0-02-A specification ownership is now implemented and expanded to 37 stable rule IDs covering the required semantic domains, including post-review LSP synchronization/interoperability/rename, standard-library determinism, logical memory budgets, registry transport, benchmark provenance, and release-version validation, with unique-ID/domain coverage validation and bilingual compatibility-change templates. Release preflight now runs ownership, native/legacy parity, fixed-seed replay, focused async contract gates, and the Cargo-authoritative release version consistency gate before deployment validation. The version validator covers Cargo/Cargo.lock, CLI output, tags, changelogs, bilingual README archive links, security metadata, release notes, the release template, and installer metadata, with deterministic TSV evidence and negative drift regression tests. The first explicit runtime-state slice is now implemented: `ExecutionContext` owns `RuntimeState` for per-run module-cache isolation, import-cycle tracking, and bounded execution-depth accounting across AST, legacy, function, method, and module paths. The AST canonicalization slice is now implemented: normal programs and local modules parse and execute through the canonical AST path, exported bindings/functions are represented in AST nodes, and parse failures no longer fall back to the line interpreter. The line interpreter is retained only for legacy line-bodied function records under an explicit compatibility-only policy. M2-MEM-01 now adds run-owned `MemoryBudget` and `ObjectStore` state to `RuntimeState`, context-aware object allocation counters, deterministic byte/task/output reservation APIs, and isolation/reset regressions; these are logical accounting boundaries rather than allocator or tracing-collector measurements. M2-MEM-02 completes stable per-run lifecycle statistics, deterministic object charges, cleanup and validation counters, output/task admission wiring, reset detachment, and the `ZAP-MEMORY-001` structured diagnostic contract. PR-7/PR-8 completes the safe bounded follow-up: the public `memory_stats()` capability record exposes `cycle_policy=explicit_clear_object_fields`, strong object/capture cycles remain an explicit `clear_object_fields()` responsibility, and active canonical AST EnvFrame operations return deterministic `BorrowError` results instead of panicking when a frame is already borrowed. No public weak-reference API, automatic cycle collector, or tracing collector is introduced. Post-v2.2.2 hardening adds cycle-safe bounded equality, checked logical-size/validation frame borrows, deterministic AST object-member borrow errors, non-panicking task-join/frame invariants, and a checked LSP rename scope stack; these changes are published in v2.2.3. The ordered native-suite diagnosis also found that the canonical AST path was missing already documented `assert`, `sort`, and `sqrt` helper dispatch; these compatibility-preserving cases are now restored with focused AST/integration regressions and catalog metadata. M2-FN-01 adds first-class callable values with assignment, higher-order arguments, returns, deterministic display/JSON behavior, and function annotations. M2-FN-02 replaces function closure ownership with parent-linked `EnvFrame` chains backed by shared binding cells, nearest-binding mutation, returned-closure lifetime retention, and regression coverage for outer reassignment, sibling sharing, shadowing, recursion, and repeated captured-state updates. M2-ASYNC-01 now provides a context-owned executor-backed language scheduling boundary: async calls return scheduled task handles, `await`/`task_join` drive deterministic polling, readiness is observable without polling, and task admission/reset are isolated per run. M2-ASYNC-02 adds cooperative `task_cancel`, poll-budget `task_join_timeout`, deterministic `Cancelled`/`TimedOut` task diagnostics, and cancellation/timeout end-to-end regressions. Broader long-running fuzz, allocator-level, and additional platform-specific P1-05 work remains. The M3-LSP corrections, API-301 taxonomy, and DOC-401 provenance record described above landed after the immutable v2.2.0 tag on `master` and were published in v2.2.1; M4-REL-01 refers only to the historical v2.2.0 publication. The post-v2.2.4 HTTP maintenance slice replaces three `http_request` invariant panics with deterministic errors for invalid parser results or missing scheme/host. Cross-language review then fixed Unix process-group termination argument parsing in both direct and async adapters and added focused regressions. The candidate does not add parser/runtime syntax or framework work, and publication remains held while six RustSec advisories remain in the intentionally unchanged locked graph.

## Next P0/P1 execution queue

After the first P0-04 slice, the next work is ordered by runtime risk rather than feature size:

| Order | Work package | Implementation slice | Acceptance evidence |
|---|---|---|---|
| M2-MEM-01 | MemoryBudget/ObjectStore foundation | Add run-owned logical byte/task/output budgets and ObjectStore counters without allocator/tracing claims | Context budget/object-store isolation and reset regressions; full native suite |
| M2-MEM-02 | Memory lifecycle and statistics hardening | Implemented logical accounting slice plus PR-7/PR-8 safe cycle/borrow policy: deterministic nested-value, callable-capture/default, builtin-output, and finalized-object charges with byte/output rollback; explicit cycle clearing; checked EnvFrame operations; no public weak refs or automatic collector | `memory_stats()` budget/lifecycle fields including `cycle_policy`, `ZAP-MEMORY-001`, explicit cleanup counters, checked-borrow regressions, repeated module-cache behavior, focused value/evaluator regressions, and full native suite |
| M2-FN-01 | First-class callable values | Represent functions as assignable, passable, returnable, invokable runtime values with deterministic type, arity, display, and JSON boundaries | Callable assignment/higher-order/return tests, function annotation parsing, compatibility alias regression, and full native suite |
| M2-FN-02 | Parent-linked EnvFrame closures | Completed: parent-linked lexical frames backed by shared binding cells define lookup, nearest-binding mutation, recursion, shadowing, and returned-closure lifetime semantics | Returned-closure mutation plus outer-reassignment, sibling-sharing, shadowing, and recursion regressions; callable higher-order tests; deterministic type/arity diagnostics; bilingual memory/spec contracts; and full native suite |
| M2-ASYNC-01 | Executor-backed language scheduling | Implemented explicit `Pending`/`Ready`/`Cancelled`/`TimedOut`/`Joined` task states in the context-owned scheduler; first join releases exactly one admitted-task slot, while unknown/repeated joins do not release again | ScheduledFuture AST tests, explicit terminal-state and one-time-release regressions, scheduler isolation/reset regression, bilingual async-boundary contract, and full native suite |
| M2-ASYNC-02 | Cooperative language task cancellation and timeout | Implemented stable cancellation/timeout terminal transitions and deterministic `UnknownTask`/`AlreadyJoined` diagnostics; the current language contract explicitly defines async body execution as eager before scheduling the completed value | `task_cancel`/`task_join_timeout` AST and end-to-end regressions, eager-invocation output-order regression, cancellation/timeout diagnostics, one-time task-budget-release regression, bilingual async documentation, and full native suite |
| M2-VERIFY-01 | Bounded replayable verification | Extend fixed-seed corpus replay into capped repeated semantic verification with fail-closed fixture/corpus limits and durable evidence | `scripts/test_m2_verify_replay.sh`, 12-round deterministic digest agreement, invalid-setting regressions, CI/release-preflight TSV/log artifacts, bilingual replay contract, and full native suite |
| M2-VERIFY-02 | Platform-native hardening matrix | Execute the supported async adapter and packaging cases on native Linux, Windows, and macOS targets, with explicit Unix-only limitations | Expanded `scripts/test_p005c_async_matrix.sh`, newline/directory/symlink/process-status regressions, deterministic archive regression, target-named CI logs, bilingual boundary contract, and full native suite |
| M2-BENCH-01 | Benchmark provenance and variance reporting | Record reproducibility metadata and quantify run-to-run spread without turning machine-dependent timing into a portability claim | Bounded runner controls, provenance TSV, binary/script SHA-256 digests, deterministic variance fields, expanded comparator schema, contract regressions, CI/preflight artifacts, bilingual benchmark contract, and full native suite |
| M2-REG-01 | Registry transport edge cases | Make partial reads, chunked responses, truncated declarations, slow peers, and oversized response headers fail safely and deterministically | Bounded client timeout/response reader, chunked/partial/slow/oversized TCP fixtures, stable diagnostics, focused CI/preflight corpus, bilingual package contract, and full native suite |
| M3-STDLIB-01 | Standard-library stability policy | Make public domain and builtin compatibility metadata explicit and reviewable before API evolution | Machine-readable catalog, twelve-domain stability table, deprecation/semver/platform/limit/error metadata, schema-2 determinism taxonomy with legacy-boolean compatibility, bilingual policy, catalog regressions, policy contract, CI/preflight gate, and full native suite |
| M3-LSP-01 | LSP and VS Code semantic parity | Keep editor features aligned with canonical AST/lexer spans, the standard-library catalog, async scheduling, and per-session document state | Standard full-sync/version regressions, file-local scope-aware rename with explicit cross-file limitation, nested/module-aware symbols, catalog-driven completion, async hover/signature metadata, checked-in canonical VS Code package, editor parity validator, CI/preflight gate, bilingual LSP contract, and full native suite |
| M3-DOC-01 | Bilingual learning/reference documentation split | Separate learner, reference, specification, package-author, runtime, deployment/security, tooling, and release content behind explicit navigation and verified-version metadata | Checked-surface matrix in both navigation hubs, canonical companion links, repository-relative link audit, bilingual headers, active v2.2.6 metadata with explicit historical v2.2.0, v2.2.1, and v2.2.2 provenance, documentation consistency regression, and full native suite |
| M4-RFC-01 | Traits and composition RFC | Freeze a reviewable composition-first design without changing the v2.2.0 parser/runtime | Bilingual RFC, terminology, lookup/visibility rules, stable diagnostic plan, migration path, dispatch boundary, rejected alternatives, compatibility impact, and explicit deferred implementation decision |
| M4-REL-01 | v2.2.0 release preparation and publication | Publish the audited roadmap result from a clean, version-consistent commit | Cargo/Cargo.lock/CLI/tag agreement, bilingual release notes, README/security/archive updates, clean release preflight, signed Linux/macOS/Windows assets, checksums/provenance, and successful workflow verification |
| M1-AST-01 | AST canonicalization | Route normal source and local modules through the canonical AST executor, represent exports in AST nodes, and remove the normal-program line-interpreter fallback | AST parser/export regressions, AST module-import execution test, syntax-failure boundary test, and unchanged full native suite |
| M1-RS-01 | Explicit runtime state | Pass `ExecutionContext` through evaluation and move module cache, import-cycle stack, and execution-depth accounting out of process-global state | Independent-context isolation/reset regressions and unchanged full native suite |
| P0-05-A | Async boundary capability contract | Add a deterministic `async_capabilities()` report and document executor/worker/network/process/cancellation boundaries | Stable capability map, bilingual contract, AST/runtime tests, and explicit distinction between deterministic language scheduling and production I/O |
| P0-05-B | Async resource-limit validation | Validate worker/task/read/socket/process limits before admission and keep deadline/output/cancellation behavior fail-closed | Invalid-limit tests, deterministic errors, and bounded process cleanup |
| P0-05-C | Cross-platform async matrix | Exercise path, process, socket, deadline, cancellation, and output-limit behavior on Linux, Windows, and macOS targets | Target-native CI evidence or a versioned documented limitation |
| P1-05-A | Replayable verification layers | Add fixed-seed property/fuzz corpus replay and durable failure fixtures for parser, JSON, lockfile, registry, memory, and async boundaries | CI artifact contains the seed/input and repeat runs are deterministic |
| P0-01-A | Native/legacy parity report | Classify legacy fixtures and gate unapproved semantic drift | Versioned parity matrix and executable CI report |
| P0-02-A | Specification ownership index | Map public syntax, typing, runtime, limit, and error rules to canonical bilingual sections and fixtures | No unowned normative rule and every rule has fixture ownership |
| P0-06 | Release version single-source-of-truth gate | Validate every release-facing version surface from the Cargo authority and fail closed on drift | Deterministic TSV report, negative drift harness, CI artifact, preflight enforcement, and bilingual policy |

P2-02 standard-library stability, P2-03 LSP/VS Code parity, and P2-04 documentation navigation are complete as M3-STDLIB-01, M3-LSP-01, and M3-DOC-01. M4-RFC-01 is complete as a reviewed RFC, while traits implementation remains deferred. M4-REL-01 is complete as the historical published v2.2.0 release; its tag and assets remain immutable. Post-release corrections were published in v2.2.1 and v2.2.2 and are recorded below; the post-v2.2.2 hardening is included in v2.2.3, and the active-baseline documentation synchronization is published in v2.2.4; the HTTP URL invariant hardening is prepared for the v2.2.6 release candidate.

## Planning principles

Zap should continue from reliability foundations toward ecosystem features. Memory ownership, deterministic execution, conformance, and documentation contracts should be strengthened before adding new asynchronous syntax, HTTP server frameworks, package publishing, or traits implementation. Every completed item must be backed by code, focused regression tests, bilingual documentation where the public contract changes, and the repository release gates.

> **Rule:** A feature is not complete when it merely works on Linux. It is complete when its semantics, failure behavior, limits, cross-platform expectations, and compatibility status are documented and tested.

## Priority sequence

| Priority | Work item | Current status | Main outcome | Suggested milestone |
|---|---|---|---|---|
| 1 | **M1-AST-01 Canonical AST execution boundary** | Implemented | Normal programs and local modules use AST execution; legacy line execution is compatibility-only | M1 — Runtime safety |
| 2 | **M1-RS-01 Explicit runtime state and execution context** | First slice implemented | Per-run module/cache/depth isolation without process-global ownership | M1 — Runtime safety |
| 3 | **M2-MEM-01 MemoryBudget/ObjectStore foundation** | Foundation implemented | Run-owned logical budget and object-store counters; allocator-level and tracing collection remain deferred |
| 4 | **P0-04 Memory and reference-cycle contract** | Partial | Observable ownership policy, bounded memory behavior, and deterministic cycle-breaking diagnostics | M1 — Runtime safety |
| 5 | **P0-05 Deterministic versus production async boundary** | Partial | A precise contract separating the deterministic executor from production I/O and blocking work | M1 — Runtime safety |
| 6 | **P1-05 Conformance, property, and fuzz layers** | Partial | Broader panic-free, deterministic, and platform-aware verification | M2 — Verification |
| 7 | **P0-01 Native/legacy conformance contract** | Partial | Native behavior becomes canonical with an executable legacy-parity report and migration policy | M2 — Verification |
| 8 | **P0-02 Consolidated language specification** | Partial | One bilingual semantic index owns syntax, typing, runtime behavior, compatibility, and version decisions | M2 — Verification |
| 9 | **P2-02 Standard-library API stability policy** | M3-STDLIB-01 completed | Public APIs receive stability labels, deprecation rules, versioning rules, and platform support records | M3 — Tooling and documentation |
| 10 | **P2-03 LSP and VS Code semantic parity** | M3-LSP-01 completed | Rename, module-aware indexing, async-aware editor behavior, and parser/AST parity become release-tested | M3 — Tooling and documentation |
| 11 | **P2-04 Learning/reference documentation split** | M3-DOC-01 completed | Learner, reference, specification, package-author, runtime, and deployment information become navigable and versioned | M3 — Tooling and documentation |
| 12 | **P2-01 Composition and traits/interfaces RFC** | M4-RFC-01 completed (design-only) | A reviewed bilingual design exists before parser/runtime changes begin; parser/runtime implementation remains deferred | M4 — Language design |
| 13 | **M4-REL-01 v2.2.0 release** | Published and verified as historical | Cargo-authoritative versioning, bilingual release notes, signed cross-platform assets, checksums/provenance, and successful GitHub Actions verification | M4 — Release |

## M1 — Runtime safety

### M1.1 — Complete P0-04 memory contract

The first task is to make the existing `Rc<RefCell>` ownership policy measurable without prematurely introducing a tracing garbage collector. The implementation should document which values are reference-counted, which boundaries are explicitly single-threaded, how object fields are cleared, and which operations are safe during shutdown or error recovery.

| ID | Task | Acceptance evidence |
|---|---|---|
| M1-04-01 | Define a stable heap-statistics shape containing live object count, tracked allocation count, and any deliberately unavailable metrics | Bilingual contract document and a deterministic API/test fixture |
| M1-04-02 | Add allocation and deallocation counters at the chosen runtime ownership boundary | Repeated execution produces deterministic counters; counters do not expose raw addresses or secrets |
| M1-04-03 | Define whether weak references are supported, intentionally unavailable, or restricted to an internal diagnostic boundary | Explicit design decision, error behavior for unsupported use, and no accidental thread-safety claim |
| M1-04-04 | Add closure-capture and object-cycle fixtures that exercise `clear_object_fields` and release behavior | Regression tests demonstrate cycle breaking and stable post-cleanup statistics |
| M1-04-05 | Define memory-limit behavior for strings, lists, maps, objects, and total execution state | Limit table, stable error codes, and malformed/oversized-input coverage |
| M1-04-06 | Replace runtime `RefCell` panic paths with checked borrow access and stable diagnostics | `try_borrow`/`try_borrow_mut`, checked EnvFrame operations, fallible object-field helpers, `ZAP-BORROW-001`, JSON propagation, and panic-free object/frame conflict regressions |

The milestone is complete only when the runtime can explain its ownership model, detect or report the bounded conditions it promises to detect, and pass the full native suite without making a tracing-collection claim that the implementation does not provide.

### M1.2 — Complete P0-05 async boundary

The async documentation and implementation must clearly distinguish deterministic single-threaded scheduling from production adapters that use controlled worker resources or operating-system I/O. The next slice should not add broad `async fn`/`await` syntax until task lifecycle and cancellation semantics are stable.

| ID | Task | Acceptance evidence |
|---|---|---|
| M1-05-01 | Publish a single async boundary table for deterministic executor, blocking adapter, network adapter, process adapter, and cancellation behavior | English/Burmese documentation parity and linked examples |
| M1-05-02 | Specify task admission, poll budget, join, timeout, cancellation precedence, repeated join, and panic-to-error behavior | Normative contract plus focused unit tests |
| M1-05-03 | Record which operations are cancellable and which foreign blocking calls cannot be interrupted | Explicit limitation tests and deterministic diagnostics |
| M1-05-04 | Add resource-limit tests for worker count, task count, output bytes, deadlines, and child-process cleanup | Cross-platform regression evidence and no orphan-process guarantee within the supported boundary |
| M1-05-05 | Add a release checklist for local registry-service deployment versus public production deployment | Deployment documentation identifies TLS, supervision, sandbox, quota, credential, and egress responsibilities |

## M2 — Verification and language contracts

### M2.1 — Expand P1-05 verification layers

The existing deterministic corpus is a strong foundation, but the remaining scope should be split into bounded jobs instead of one unreviewable fuzz target. The focus is panic freedom, deterministic rejection, input-size limits, and platform-specific behavior.

| ID | Task | Acceptance evidence |
|---|---|---|
| M2-05-01 | Add long-running parser, JSON, lockfile, registry, and standard-library fuzz targets with fixed seed/replay support | CI can reproduce a failing seed and archive the minimized input |
| M2-05-02 | Add allocator/heap-level tests around object cycles, oversized values, and repeated module execution | Memory-related regression artifacts and bounded runtime behavior |
| M2-05-03 | Add Windows and macOS-specific path, process, newline, permission, and archive cases | Target-native CI evidence or a documented, reproducible limitation |
| M2-05-04 | Add property tests for deterministic ordering, diagnostic normalization, checksum verification, and lockfile round trips | Properties are named, replayable, and run in CI |
| M2-05-05 | Add a failure-corpus ownership policy so every security or parser regression has a durable fixture and rationale | Corpus index, test naming convention, and changelog procedure |
| M2-VERIFY-01 | Extend fixed-seed replay into a bounded repeated verification job | Capped rounds, fixture/corpus byte limits, identical semantic outcome digests, and archived TSV/log evidence in CI and release preflight |
| M2-VERIFY-02 | Extend native platform verification | Linux/Windows/macOS target-native async and archive cases, with explicit Unix symlink limitation and target-named logs |
| M2-BENCH-01 | Add benchmark provenance and variance indicators | Commit/target/OS/toolchain/binary metadata, bounded repeats, deterministic spread fields, and schema validation in CI/preflight |
| M2-REG-01 | Harden registry transport edges | Partial/chunked body handling, slow-peer deadlines, explicit response-size bounds, and stable truncation diagnostics |

### M2.2 — Complete P0-01 native/legacy conformance

Native execution should be declared canonical only after the repository can show where legacy behavior differs. The conformance layer should report accepted, rejected, compatible, deprecated, and intentionally divergent cases rather than hiding differences inside broad smoke tests.

| ID | Task | Acceptance evidence |
|---|---|---|
| M2-01-01 | Inventory legacy fixtures and classify each behavior as normative, compatibility, deprecated, or rejected | Versioned parity matrix |
| M2-01-02 | Add an executable conformance command that runs native and legacy fixtures with normalized output | Deterministic report suitable for CI artifacts |
| M2-01-03 | Define migration guidance for behavior that is intentionally native-only | Bilingual migration notes and examples |
| M2-01-04 | Add release gating for newly introduced parity drift | CI fails on unapproved drift and points to the owning fixture |

### M2.3 — Complete P0-02 consolidated specification

The language specification should become the owner of semantic truth. Existing guides may remain learner-friendly, but rules should not silently diverge between the syntax guide, usage guide, runtime notes, type-checking matrix, and release notes.

| ID | Task | Acceptance evidence |
|---|---|---|
| M2-02-01 | Build a rule index mapping every syntax/runtime/type rule to its canonical specification section | No unowned public rule remains |
| M2-02-02 | Move or cross-link fragmented precedence, error, path, module, resource-limit, and async rules | Bilingual links and version ownership |
| M2-02-03 | Add conformance fixture IDs beside normative rules | A specification rule points to a passing or intentionally failing fixture |
| M2-02-04 | Add a compatibility/deprecation template for future semantics changes | Template is used in changelog and migration documents |

## M3 — Tooling and documentation

### M3.1 — Standard-library stability policy

Every public standard-library API should state whether it is experimental, provisional, stable, deprecated, or platform-specific. Each API record should include input limits, output limits, timeout behavior, error codes, determinism, and platform differences.

The first deliverable is a machine-readable or consistently structured API inventory. The second is a documentation and CI check that every public helper has the required fields. The third is a deprecation workflow that preserves old behavior for a documented period or records an explicit breaking release decision.

### M3.2 — LSP and VS Code parity

The editor tooling should use the canonical parser and shared diagnostic contract rather than maintaining a second interpretation of the language. The next work should prioritize rename, module-aware indexing, nested symbol ranges, async-aware completion and hover, and stable diagnostic snapshots.

Acceptance requires deterministic LSP responses, zero-based range conversion from the one-based CLI span, stable `ZAP-*` diagnostic codes, traversal-safe imported-file indexing, and fixtures for both open and unopened local modules.

### M3.3 — Learning/reference documentation split

The documentation should be divided into learner material, syntax reference, language specification, standard-library reference, package-author guidance, runtime internals, and deployment/security operations. Each section should display its verified Zap version and link back to the canonical semantic rules.

The documentation milestone is complete: a beginner can learn without reading internals, an experienced user can find normative syntax and error behavior, and an operator can identify deployment responsibilities without confusing a local fixture with a public production service. The checked-surface matrix and bilingual navigation hubs are verified for v2.2.6.

## M4 — Language-design RFC

### M4.1 — Composition and traits/interfaces

M4-RFC-01 is complete as a reviewed, design-only RFC. It compares composition with the current single-inheritance model, defines method lookup and visibility, explains diagnostics for missing or conflicting implementations, describes migration from inheritance, and specifies the hybrid static/dynamic dispatch boundary. The RFC contains bilingual terminology, examples, rejected alternatives, compatibility impact, and an explicit version decision.

No parser or runtime implementation is enabled by this milestone. Traits, interfaces, composition, and method-resolution syntax remain deferred beyond v2.2.6 until a later implementation proposal passes the listed conformance and specification gates.

## Release-gate checklist for every milestone

| Gate | Required result |
|---|---|
| Formatting | Rust formatting check passes with the pinned toolchain |
| Static quality | Strict Clippy passes with `-D warnings` |
| Tests | Full native unit/integration suite and relevant focused fixtures pass |
| Determinism | Repeated runs produce stable output, ordering, diagnostics, and archives |
| Security | Malformed, oversized, traversal, secret-redaction, and checksum cases remain fail-closed; modern `cargo-audit` reports no unresolved advisories |
| Documentation | English/Burmese public contracts remain synchronized |
| Compatibility | Version impact, migration path, deprecation status, and changelog entry are explicit |
| Release version | Cargo-authoritative version, CLI output, tag, README/archive, security, release-note, template, and installer surfaces agree |
| Repository hygiene | `git diff --check` passes and no generated or secret files are committed |

## Immediate next task

The **M1-AST-01 canonical AST execution boundary**, **M1-RS-01 explicit runtime state**, **P0-06 Release version single-source-of-truth gate**, **P1-05-A/M2-VERIFY-01/M2-VERIFY-02 replayable and platform-native verification layers**, **M2-BENCH-01 benchmark provenance and variance reporting**, **M2-REG-01 registry transport edge-case coverage**, **M3-STDLIB-01 standard-library stability policy**, **M3-LSP-01 LSP and VS Code semantic parity**, **P0-01-A Executable native/legacy parity**, **P0-02-A Specification ownership expansion**, the **M1-04-06 checked object-field borrow-safety slice**, and **M2-ASYNC-01/M2-ASYNC-02 executor-backed language scheduling with cooperative cancellation and poll-budget timeout** are implemented. Release preflight now includes the version gate, ownership, parity, fixed-seed replay, bounded replay, platform archive/async checks, benchmark provenance/schema/regression checks, registry transport tests, the standard-library policy contract, the LSP/VS Code semantic-parity gate, and the strict RustSec audit before deployment validation. All historical roadmap milestones through the published M4-REL-01 v2.2.0 tag are complete; API-301 and DOC-401 are complete on post-release `master`, corrective work through v2.2.2 is published, and the post-v2.2.2 hardening is published in v2.2.3; the active-baseline documentation synchronization is published in v2.2.4. The published v2.2.6 release includes the cross-language-informed Unix process-group cleanup fix, bilingual release-surface corrections, and the separately authorized dependency remediation. Its strict `cargo-audit 0.22.2` scan reports zero unresolved advisories across 87 locked crate dependencies; the Rust 1.88.0 toolchain is required by `time 0.3.47`. The release workflow [32638479414](https://github.com/hidecard/zap/actions/runs/32638479414) published v2.2.6 from tagged commit [`d1d6816`](https://github.com/hidecard/zap/commit/d1d6816d7d39198b4a9778d531e29cd7b4e1f38a), and independent checksum/signature verification passed. Future implementation work may propose a post-v2.2 traits subset only after the RFC gates are reviewed; parser/runtime implementation and broad async syntax remain deferred.

## Post-v2.2.0 corrective-release cycle

The published v2.2.0 release remains the immutable historical baseline at tag commit [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb). The published v2.2.1 and v2.2.2 releases are also immutable. Corrective work was implemented later on `master` and published in those patch releases rather than described as part of v2.2.0.

| Milestone | Status | Record |
|---|---|---|
| LSP-SYNC-01, LSP-REN-01, LSP-INTEROP-01 | Implemented on post-release `master` | Standard full-sync/version handling, file-local scope-aware rename, negotiated positions, strict URIs, and workspace caps; see [remediation/provenance record](POST_V2.2.0_REMEDIATION_EN.md). |
| EXT-201 | Implemented on post-release `master` | Canonical `vscode-extension/` source, native-LSP rename provider, catalog-aligned assets, and package contract; see [remediation/provenance record](POST_V2.2.0_REMEDIATION_EN.md). |
| API-301 | Implemented on post-release `master` | Schema-2 `determinism_class` taxonomy with compatibility-preserving `deterministic` boolean and bilingual policy/index updates; see [standard-library policy](STDLIB_POLICY_EN.md). |
| DOC-401 | Completed on post-release `master` | Bilingual traceability record, roadmap/progress updates, README provenance, changelog clarification, and navigation links; see [remediation/provenance record](POST_V2.2.0_REMEDIATION_EN.md). |
| v2.2.1 | Published and verified | LSP/editor, standard-library, and documentation correction assets from a clean version-consistent commit; v2.2.0 was not retagged or rewritten. |
| v2.2.2 | Published and verified | Runtime-safety, canonical-helper, grammar, and documentation correction assets from a clean version-consistent commit; v2.2.0 and v2.2.1 were not retagged or rewritten. |
| v2.2.3 | Published and verified | Runtime reliability, bounded cycle-safe equality, checked borrow propagation, invariant fallbacks, LSP rename hardening, and synchronized bilingual documentation; v2.2.0, v2.2.1, and v2.2.2 were not retagged or rewritten. |
| v2.2.4 | Published and verified | Active language-specification, generic type-check, README, security, conformance, navigation, and package metadata references were synchronized after audit; no parser, runtime, or generic-syntax behavior changed, and v2.2.0 through v2.2.3 were not retagged or rewritten. |
| v2.2.6 | Published and verified | HTTP URL parser-result, scheme, and host invariant handling returns deterministic errors; Unix direct/async process-group cleanup uses an explicit `kill` option terminator with focused regressions; the released locked graph is `ureq 2.12.1`, `url 2.5.8`, `idna 1.1.0`, `rustls 0.23.40`, `rustls-webpki 0.103.15`, `rcgen 0.13.2`, and dev-only `time 0.3.47`. Strict `cargo-audit 0.22.2` reports zero unresolved advisories across 87 locked crate dependencies. Rust 1.88.0 is required by `time 0.3.47`; release workflow [32638479414](https://github.com/hidecard/zap/actions/runs/32638479414) published from tagged commit [`d1d6816`](https://github.com/hidecard/zap/commit/d1d6816d7d39198b4a9778d531e29cd7b4e1f38a), and all published archives and signed release metadata passed independent verification. Parser/runtime syntax, eager async behavior, and framework scope are unchanged. |

## Related records

This plan extends the remaining-work register in [`PDF_REMAINING_TODO_EN.md`](PDF_REMAINING_TODO_EN.md), the [post-v2.2.0 remediation/provenance record](POST_V2.2.0_REMEDIATION_EN.md), the v2.1 roadmap in [`V2.1_ROADMAP_EN.md`](V2.1_ROADMAP_EN.md), and the consolidated language specification in [`LANGUAGE_SPEC_EN.md`](LANGUAGE_SPEC_EN.md).
