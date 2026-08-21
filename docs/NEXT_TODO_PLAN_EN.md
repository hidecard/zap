# Zap Next Engineering TODO Plan

**Baseline:** Zap v2.1.7 verified release, with P0-03 structured diagnostics completed in the `master` branch.

**Purpose:** Define the next implementation sequence after the stable diagnostic contract. This plan turns the remaining TODO register into executable milestones without treating already-completed release work as unfinished.

**Current progress:** The first P0-04 slice is implemented: tracked object allocation/deallocation statistics, bounded `memory_stats()` diagnostics, cycle-safe value validation, deterministic value limits, regression tests, and bilingual memory-contract documentation. P0-05-A, P0-05-B, and P0-05-C are also implemented: the descriptive deterministic `async_capabilities()` builtin, catalog entry, typed resource-limit preflight validation, TCP request-size admission checks, and a reproducible Linux x86_64/Windows x86_64/macOS ARM64 focused matrix with target-named CI artifacts. P1-05-A replayable verification is now implemented with a fixed `ZAP_CORPUS_SEED`, six durable failure-corpus categories, deterministic replay logs, and CI artifact upload. P0-01-A executable native/legacy parity is now implemented with a versioned six-case policy matrix, normalized output digests, deterministic reports, and CI artifact upload. P0-02-A specification ownership is now implemented and expanded to 27 stable rule IDs covering the required semantic domains, with unique-ID/domain coverage validation and bilingual compatibility-change templates. Release preflight now runs ownership, native/legacy parity, fixed-seed replay, focused async contract gates, and the Cargo-authoritative release version consistency gate before deployment validation. The version validator covers Cargo/Cargo.lock, CLI output, tags, changelogs, bilingual README archive links, security metadata, release notes, the release template, and installer metadata, with deterministic TSV evidence and negative drift regression tests. Executor-backed language scheduling and language-level cancellation/timeout controls remain deferred; broader long-running fuzz, allocator-level, and additional platform-specific P1-05 work remains.

## Next P0/P1 execution queue

After the first P0-04 slice, the next work is ordered by runtime risk rather than feature size:

| Order | Work package | Implementation slice | Acceptance evidence |
|---|---|---|---|
| P0-05-A | Async boundary capability contract | Add a deterministic `async_capabilities()` report, document executor/worker/network/process/cancellation boundaries, and make unsupported language-level cancellation/timeout explicit | Stable capability map, bilingual contract, AST/runtime tests, and no claim that eager futures are executor-backed |
| P0-05-B | Async resource-limit validation | Validate worker/task/read/socket/process limits before admission and keep deadline/output/cancellation behavior fail-closed | Invalid-limit tests, deterministic errors, and bounded process cleanup |
| P0-05-C | Cross-platform async matrix | Exercise path, process, socket, deadline, cancellation, and output-limit behavior on Linux, Windows, and macOS targets | Target-native CI evidence or a versioned documented limitation |
| P1-05-A | Replayable verification layers | Add fixed-seed property/fuzz corpus replay and durable failure fixtures for parser, JSON, lockfile, registry, memory, and async boundaries | CI artifact contains the seed/input and repeat runs are deterministic |
| P0-01-A | Native/legacy parity report | Classify legacy fixtures and gate unapproved semantic drift | Versioned parity matrix and executable CI report |
| P0-02-A | Specification ownership index | Map public syntax, typing, runtime, limit, and error rules to canonical bilingual sections and fixtures | No unowned normative rule and every rule has fixture ownership |
| P0-06 | Release version single-source-of-truth gate | Validate every release-facing version surface from the Cargo authority and fail closed on drift | Deterministic TSV report, negative drift harness, CI artifact, preflight enforcement, and bilingual policy |

P2-02 standard-library stability, P2-03 LSP/VS Code parity, and P2-04 documentation navigation follow after the P0/P1 safety gates. P2-01 traits/composition remains an RFC-only milestone until the conformance and specification contracts are complete.

## Planning principles

Zap should continue from reliability foundations toward ecosystem features. Memory ownership, deterministic execution, conformance, and documentation contracts should be strengthened before adding new asynchronous syntax, HTTP server frameworks, package publishing, or traits implementation. Every completed item must be backed by code, focused regression tests, bilingual documentation where the public contract changes, and the repository release gates.

> **Rule:** A feature is not complete when it merely works on Linux. It is complete when its semantics, failure behavior, limits, cross-platform expectations, and compatibility status are documented and tested.

## Priority sequence

| Priority | Work item | Current status | Main outcome | Suggested milestone |
|---|---|---|---|---|
| 1 | **P0-04 Memory and reference-cycle contract** | Partial | Observable ownership policy, bounded memory behavior, and deterministic cycle-breaking diagnostics | M1 — Runtime safety |
| 2 | **P0-05 Deterministic versus production async boundary** | Partial | A precise contract separating the deterministic executor from production I/O and blocking work | M1 — Runtime safety |
| 3 | **P1-05 Conformance, property, and fuzz layers** | Partial | Broader panic-free, deterministic, and platform-aware verification | M2 — Verification |
| 4 | **P0-01 Native/legacy conformance contract** | Partial | Native behavior becomes canonical with an executable legacy-parity report and migration policy | M2 — Verification |
| 5 | **P0-02 Consolidated language specification** | Partial | One bilingual semantic index owns syntax, typing, runtime behavior, compatibility, and version decisions | M2 — Verification |
| 6 | **P2-02 Standard-library API stability policy** | Partial | Public APIs receive stability labels, deprecation rules, versioning rules, and platform support records | M3 — Tooling and documentation |
| 7 | **P2-03 LSP and VS Code semantic parity** | Partial | Rename, module-aware indexing, async-aware editor behavior, and parser/AST parity become release-tested | M3 — Tooling and documentation |
| 8 | **P2-04 Learning/reference documentation split** | Partial | Learner, reference, specification, package-author, runtime, and deployment information become navigable and versioned | M3 — Tooling and documentation |
| 9 | **P2-01 Composition and traits/interfaces RFC** | Deferred | A reviewed design exists before parser/runtime changes begin | M4 — Language design |

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

The documentation milestone is complete when a beginner can learn without reading internals, an experienced user can find normative syntax and error behavior, and an operator can identify deployment responsibilities without confusing a local fixture with a public production service.

## M4 — Language-design RFC

### M4.1 — Composition and traits/interfaces

Traits, interfaces, composition, and method resolution should remain deferred until a reviewed RFC answers the hard compatibility questions. The RFC must compare composition with the current single-inheritance model, define method lookup and visibility, explain diagnostics for missing or conflicting implementations, describe migration from inheritance, and specify whether dynamic dispatch or static conformance is intended.

No parser or runtime implementation should begin until the RFC has bilingual terminology, examples, rejected alternatives, compatibility impact, and an explicit version decision.

## Release-gate checklist for every milestone

| Gate | Required result |
|---|---|
| Formatting | Rust formatting check passes with the pinned toolchain |
| Static quality | Strict Clippy passes with `-D warnings` |
| Tests | Full native unit/integration suite and relevant focused fixtures pass |
| Determinism | Repeated runs produce stable output, ordering, diagnostics, and archives |
| Security | Malformed, oversized, traversal, secret-redaction, and checksum cases remain fail-closed |
| Documentation | English/Burmese public contracts remain synchronized |
| Compatibility | Version impact, migration path, deprecation status, and changelog entry are explicit |
| Release version | Cargo-authoritative version, CLI output, tag, README/archive, security, release-note, template, and installer surfaces agree |
| Repository hygiene | `git diff --check` passes and no generated or secret files are committed |

## Immediate next task

The **P0-06 Release version single-source-of-truth gate**, **P1-05-A Replayable verification layers**, **P0-01-A Executable native/legacy parity**, and **P0-02-A Specification ownership expansion** slices are implemented. Release preflight now includes the version gate and all four P0/P1 contract gates before deployment validation. The next implementation work is the remaining documented P1-05 extensions; traits implementation and broad async syntax remain deferred until the P0/P1 safety gates are complete.

## Related records

This plan extends the remaining-work register in [`PDF_REMAINING_TODO_EN.md`](PDF_REMAINING_TODO_EN.md), the v2.1 roadmap in [`V2.1_ROADMAP_EN.md`](V2.1_ROADMAP_EN.md), and the consolidated language specification in [`LANGUAGE_SPEC_EN.md`](LANGUAGE_SPEC_EN.md).
