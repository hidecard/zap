## v2.2.7

- Remediated the six RustSec advisories present on the Framework branch by updating the native dependency graph to the clean v2.2.6 security baseline.
- Updated the Framework branch to Rust 1.88.0, added native/host RustSec CI gates, and fixed rcgen 0.13 TLS test compatibility.
- Added the Zap-native Framework starter validation and synchronized release-facing documentation.

# Zap Changelog

## [Unreleased]

## [2.3.0] — 2026-08-23

### LSP diagnostics and code actions
- Added server-side `textDocument/codeAction` support with `quickfix`, `source`, and `source.organizeImports` capability advertisement.
- Added stable style diagnostic codes and warning severity for tabs, trailing whitespace, and lines over 120 characters.
- Improved diagnostic ranges to target tabs, trailing whitespace, long-line overflow, and quoted identifiers instead of highlighting the entire line.
- Added safe server quick fixes for replacing tabs, removing trailing whitespace, adding missing function parentheses, and inserting a uniquely inferred unmatched closing delimiter.
- Added diagnostic `fixIds` metadata and line-aware malformed function-signature errors while preserving CLI diagnostics compatibility.

## [2.2.6] — 2026-08-23

### Release engineering
- Hardens filesystem confinement, locked-build validation, bounded builtins, URL parsing, test discovery, registry-test isolation, and cross-platform compatibility without adding framework work.
- Updates the authorized remediation branch to `ureq 2.12.1`, `url 2.5.8`, `idna 1.1.0`, `rustls 0.23.40`, `rustls-webpki 0.103.15`, `rcgen 0.13.2`, and development-only `time 0.3.47`; strict `cargo-audit 0.22.2` reports zero unresolved advisories across 87 locked crate dependencies.
- Pins Rust 1.88.0 for the released source and CI quality job because `time 0.3.47` requires it; v2.2.6 was published only after clean-commit, GitHub CI, final preflight, and signed-artifact verification gates passed. See the [published release](https://github.com/hidecard/zap/releases/tag/v2.2.6) and [release workflow run](https://github.com/hidecard/zap/actions/runs/32638479414).

## [2.2.5] — 2026-08-23

### Release engineering
- Hardened HTTP request URL invariant handling with deterministic errors while preserving parser/runtime syntax, eager async semantics, and the no-framework scope.

## [2.2.4] — 2026-08-23

### Release engineering
- Synchronize the active v2.2.3 language-specification and generic type-check release-gate references after the post-release audit; no parser, runtime, or generic-syntax behavior changes.

## [2.2.3] — 2026-08-22

### Release engineering
- Runtime reliability hardening with bounded cycle-safe equality, checked object and EnvFrame borrow propagation, panic-free task and frame invariants, a checked LSP rename scope boundary, and synchronized bilingual documentation.

## [2.2.2] — 2026-08-22

### Release engineering
- Completed checked EnvFrame borrows and explicit cycle-policy reporting; restored canonical AST assert/sort/sqrt dispatch; synchronized standard-library metadata and VS Code grammars; and cleared the full serial native suite and CI gates.

### Post-v2.2.0 corrective cycle
- Completed LSP-SYNC-01, LSP-REN-01, LSP-INTEROP-01, and EXT-201 on `master`: standard full-document synchronization with versioned state, file-local scope-aware rename, negotiated UTF-8/UTF-16/UTF-32 positions, strict file URIs, bounded workspace indexing, and canonical VS Code packaging. Incremental range changes and cross-file rename remain unsupported.
- Completed API-301 on post-release `master`: added schema-2 `determinism_class` values (`pure`, `input-deterministic`, `runtime-dependent`, and `external-io`) with explicit domain/builtin coverage and a compatibility-preserving legacy `deterministic` boolean.
- DOC-401 records the provenance boundary: v2.2.0 remains immutable at tag commit [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb), and the corrected behavior is planned for the new v2.2.1 patch. See [`POST_V2.2.0_REMEDIATION_EN.md`](docs/POST_V2.2.0_REMEDIATION_EN.md).

## [2.2.1] — 2026-08-22

### Corrective release
- Published the post-v2.2.0 LSP synchronization, scope-aware file-local rename, URI/position/workspace-boundary hardening, canonical VS Code package, and schema-2 standard-library determinism taxonomy.
- Preserved the explicit limitations: full-text LSP synchronization only, no cross-file rename, bounded async foundation rather than a full production reactor, and design-only traits/composition.
- See [`RELEASE_2.2.1_EN.md`](docs/RELEASE_2.2.1_EN.md) and [`POST_V2.2.0_REMEDIATION_EN.md`](docs/POST_V2.2.0_REMEDIATION_EN.md) for release scope and provenance.

## [2.2.0] — 2026-08-22

### Release engineering
- Completed the audited runtime, verification, registry, standard-library, LSP/editor, bilingual documentation, and traits/composition RFC milestones.

### Compatibility and language design
- Preserved canonical AST execution, single-inheritance `extends`, deterministic async scheduling, bounded registry behavior, and existing standard-library contracts.
- Added the reviewed bilingual traits/composition RFC as a design-only record; `trait`, `interface`, `with`, and new conflict-resolution syntax remain deferred and unsupported in v2.2.0.

### Tooling and documentation
- Added parser/lexer-backed LSP rename, didClose cleanup, nested/module-aware workspace symbols, catalog-driven completion, async builtin hover/signature metadata, and validated VS Code assets.
- Completed bilingual learner/reference navigation with verified-version metadata and canonical companion links.
## [2.1.14] — 2026-08-21

### Release engineering
- Hardened the explicit workspace and LSP state migration and preserved Windows-compatible line-helper behavior after cross-platform CI regression.

## [2.1.13] — 2026-08-21

### Release engineering
- Migrated workspace confinement and LSP document ownership into explicit state objects with isolation regressions.

## [2.1.12] — 2026-08-22

### Release engineering
- Canonical AST execution now owns normal programs and local modules; legacy line execution is compatibility-only.

## [2.1.11] — 2026-08-21

### Release engineering
- Added the first explicit RuntimeState/ExecutionContext boundary for per-run module-cache, import-cycle, and execution-depth isolation, with regression tests and bilingual documentation.

### Documentation maintenance
- Corrected the Burmese learner guide's historical v0.x references and outdated feature-status claims, and aligned its current boundary with v2.1.10 contracts and deferred scope.
- Corrected English Result propagation examples to use the supported `result<any>` annotation and extended the documentation consistency gate to cover the v2.1.10 English/Burmese release-note pair.
- Updated release-note verification references to the final CI and release workflow runs.

## [2.1.10] — 2026-08-21

### Release engineering
- Added bilingual documentation consistency validation, navigation landing pages, and p95 benchmark regression protection with configurable warm-up and threshold gates.

### Documentation consistency and navigation
- Established a v2.1.9 bilingual documentation baseline for syntax, language specification, async boundaries, generic type-checking design, P2 progress, and benchmark policy metadata.
- Added English/Burmese documentation navigation landing pages and linked them from both READMEs so normative contracts, verification evidence, and contribution paths are discoverable.
- Added `scripts/validate_documentation_consistency.sh` with section-parity, code-fence-parity, stale-version, required-file, and navigation-link checks, plus a positive/negative regression harness.

### Benchmark regression protection
- Extended benchmark aggregation with a deterministic p95 column and added configurable warm-up iterations through `ZAP_BENCH_WARMUPS`.
- Added mean/p95 threshold comparison through `scripts/check_benchmark_regression.sh`, with CI and release-preflight enforcement and checked-in `benchmark-results/native-summary.csv` evidence.

## [2.1.9] — 2026-08-21

### Release engineering
- Added panic-free object borrow diagnostics with checked field access and stable ZAP-BORROW-001 errors.

### Memory borrow safety
- Added checked object-field `try_borrow`/`try_borrow_mut` accessors and fail-closed `BorrowError` handling with stable `ZAP-BORROW-001` diagnostics instead of `RefCell` panics.
- Added recursive JSON error propagation and regressions for conflicting object borrows, structured diagnostics, and safe object-field access.

## [2.1.8] — 2026-08-21

### Release engineering
- Hardened release version consistency validation across Cargo, CLI output, tags, bilingual onboarding, security metadata, release notes, templates, and installers; CI and release preflight now fail closed on drift.

### Release version consistency
- Added the Cargo-authoritative version validator, dynamic CLI/lockfile/tag checks, bilingual README archive checks, security-link checks, and hard-coded release-template detection.
- Added deterministic TSV evidence, positive/negative version-drift regression tests, and CI/release-preflight enforcement.
- Fixed CI branch refs such as `master` being mistaken for release tags; implicit tag validation now activates only for semver-shaped `v<version>` refs, while explicit `RELEASE_TAG` values remain enforced.

## [2.1.7] — 2026-08-21

### Release engineering
- Expanded bilingual specification ownership to 27 stable rule IDs and added release preflight gates for ownership, parity, replay, and async contracts.

### Cross-platform CI hardening
- Sanitized test-thread labels before using them in temporary directory names so the Windows native test matrix no longer receives invalid `::` path separators.
- Normalized accepted registry-service sockets back to blocking mode before request reads, preventing macOS targets from intermittently observing an empty response after a non-blocking listener accept.

### Specification ownership hardening
- Expanded `docs/SPEC_OWNERSHIP_INDEX.tsv` to 27 stable rule IDs covering source execution, precedence, typing, functions, modules, memory, deterministic/production async boundaries, diagnostics, registry, lockfiles, JSON/filesystem limits, standard-library catalog, CLI JSON, compatibility policy, and CI enforcement.
- Strengthened `scripts/validate_spec_ownership.sh` to reject missing sections, missing fixture owners, duplicate IDs, invalid policy values, and missing required semantic domains.
- Added bilingual `COMPATIBILITY_CHANGE_TEMPLATE_EN.md` and `COMPATIBILITY_CHANGE_TEMPLATE_MM.md` records for future normative, compatibility, deprecation, and rejection decisions.
- Extended `scripts/release_preflight.sh` to run ownership, native/legacy parity, fixed-seed replay, and focused async contract gates before deployment validation.

### Native/legacy parity hardening
- Added a versioned six-case native/legacy matrix with `common`, `native-only`, and `rejected` policy classes.
- Added normalized stdout digest comparison, deterministic TSV reports, migration guidance, and a CI parity artifact gate through `scripts/test_p001_parity.sh`.
- Added bilingual `docs/P001_PARITY_MATRIX_EN.md` and `docs/P001_PARITY_MATRIX_MM.md` documentation.

### Replayable verification hardening
- Added fixed-seed `ZAP_CORPUS_SEED` replay for parser, JSON, lockfile, registry, memory, and async boundaries.
- Added 21 durable failure fixtures, deterministic replay ordering, SHA-256/base64 input evidence in `target/p105-replay.log`, and a CI artifact gate through `scripts/test_p105_layers.sh`.
- Added bilingual `docs/P105_REPLAY_EN.md` and `docs/P105_REPLAY_MM.md` documentation defining seed, fixture ownership, replay evidence, and deferred fuzz scope.

### Async boundary hardening
- Added the deterministic `async_capabilities()` builtin and catalog entry describing the single-threaded executor, fixed-worker adapter, bounded network/process adapters, cancellation behavior, default limits, deferred language-level scheduling/cancellation/timeout, and unsupported interruption of arbitrary foreign blocking calls.
- Added typed preflight validation for zero/oversized worker, task, read, socket, and process limits, plus TCP request-size rejection before queue admission.
- Added a reproducible Linux x86_64, Windows x86_64, and macOS ARM64 focused async matrix with target-named CI log artifacts covering process, file, socket, deadline, cancellation, and output-limit behavior.
- Added runtime and AST regression coverage and synchronized the English/Burmese async runtime and standard-library documentation.

### Memory contract hardening
- Added bounded `memory_stats()` diagnostics with live-object, allocation, deallocation, and runtime-limit fields.
- Added cycle-safe validation for text, list, map, object, Result/Option, and Future values at public builtin boundaries, with deterministic memory-limit errors.
- Kept public weak references and tracing collection explicitly unsupported/deferred and documented the single-threaded ownership boundary.

### Structured diagnostics hardening
- Added a stable structured diagnostic contract shared by CLI JSON and LSP output, including stable `ZAP-*` codes, severity, deterministic notes, optional help, source locations, and regression assertions for TypeError parity.
- Added bilingual `docs/DIAGNOSTIC_MODEL_EN.md` and `docs/DIAGNOSTIC_MODEL_MM.md` documentation for diagnostic fields and compatibility rules.

## [2.1.6] — 2026-08-21

### Release engineering
- Hardened TC-001–TC-012 conformance coverage, pinned Rust quality gates, corrected Clippy compatibility, and verified cross-platform release packaging.

### Type checking and CI hardening
- Synchronized the English type-checking conformance matrix with the verified v2.1.5 TC-001 through TC-012 baseline and documented advanced generic inference as deferred scope.
- Added named CI gates for TC-001 through TC-010 conformance fixtures and the CLI/LSP `TypeError` diagnostic-parity regression.
- Pinned the repository Rust toolchain to 1.75.0 with `rustfmt` and `clippy` components for reproducible local and CI validation.
- Added `scripts/validate_v216_preflight.sh` for repeatable type-check, LSP parity, formatting, and CI-contract preflight checks.
- Fixed published-release archive verification pipe handling so successful tar-entry checks cannot be reported as false failures under `pipefail`.

## [2.1.5] — 2026-08-21

### Release engineering
- Hardened signed release publication, provenance verification, and cross-platform reproducible packaging.

## [2.1.4] — 2026-08-21

### Release engineering
- Avoided the read-only PowerShell HOME variable collision in clean-profile installer verification.

## [2.1.3] — 2026-08-21

### Release engineering
- Corrected deterministic Windows archive-root packaging so ZIP contents match the documented zap/ release layout.

## [2.1.2] — 2026-08-21

### Release engineering
- Cross-platform deterministic archive packaging and release-workflow reproducibility hardening.

## [2.1.1] — 2026-08-21

### Release engineering
- Hardened TC-006 through TC-012 conformance coverage, stabilized structured diagnostics, and aligned CLI and LSP TypeError locations.

### Package reliability

- Added the explicit `zap lock-migrate [dir]` command for conservative legacy lockfile migration.
- Preserved v1 lockfile compatibility; migration refuses to invent registry versions or checksums and requires verified registry metadata for registry-backed projects.
- Updated `zap install` to report the complete resolved registry graph, including transitive packages, with deterministic `name@version` ordering while preserving the established dependency-count prefix.
- Added offline nested-registry integration fixtures covering transitive resolution, cache verification, stable install output, missing transitive artifacts, cached checksum mismatches, and incomplete v2 lockfiles with deterministic diagnostics.
- Added `zap registry gc [--dry-run] [dir]`, which derives keep entries from the canonical project lockfile, preserves referenced artifacts, reports stale and temporary candidates without mutation in dry-run mode, and deletes candidates in deterministic lexical order.
- Added transport and registry-service failure coverage: insecure HTTP rejection, malformed remote-index diagnostics, and deterministic HTTP-status errors for non-2xx fetch and publish responses.
- Completed the v2.1-B trusted-registry enforcement slice with canonical origin normalization, a bounded deterministic allowlist, persistent `zap registry trust list|add|remove` commands, origin-scoped bearer credentials, bounded `zap registry credential list|set|remove` management, token validation/redaction, stable `ZAP-REG-AUTH-001`/`002`/`003` diagnostics, credential-aware remote index loading, effective-policy checks across dependency resolution and registry fetch/cache/publish paths, and a Rust 1.75-compatible local TLS fixture covering successful authenticated HTTPS fetch/publish. Final v2.1.0 release integration is complete.

### Type checking and conformance

- Accepted TC-012 generic syntax as the v2.1 implemented baseline for `list<T>`, `map<K, V>`, `option<T>`, and `result<T>`. Malformed forms remain rejected, while user-defined generic declarations and advanced inference are explicitly deferred in the design record.

- Added explicit `is_option_none(value)` else-branch narrowing: the true branch retains `option<T>`, while the else branch receives the payload type when soundly known.
- Added loop-boundary narrowing for guarded `while` bodies. The narrowed payload is available inside the loop, while the original wrapper type is restored after the loop so reassignment and post-loop use remain sound.
- Added permanent TC-006 conformance coverage for in-loop option payload access and post-loop wrapper restoration.
- Added control-flow expression typing for `if ... then ... else ...` expressions. Conditions must be `bool`, both branch result types must agree, and incompatible branches produce a structured `TypeError`.
- Added permanent TC-009 conformance fixtures covering compatible branches, incompatible branch results, and non-boolean conditions through `zap check --json`, plus an L3 regression asserting stable `ok`, `kind`, `file`, `line`, `column`, `message`, and `error` fields for conditional type errors.
- Added permanent TC-010 alias-narrowing fixtures for `option<T>` and `result<T>`, including wrapper preservation through alias assignment and invalidation after reassignment.
- Updated the bilingual type-checking conformance matrices to record TC-006 loop-boundary coverage and TC-012 generic syntax as implemented baseline evidence; future generic declarations and advanced inference remain deferred.
- Added an L4 LSP diagnostic regression and shared source-diagnostic bridge so CLI and LSP type errors use the same `TypeError` code, source-location semantics, and normalized message.

### Security and release hardening

- Added deterministic security-property corpus tests for canonical registry URL normalization, adversarial URL rejection, trusted-registry and credential scope boundaries, bounded allowlist behavior, longest-prefix token selection, token validation, and secret redaction.
- Added signed registry-index mutation coverage that feeds malformed and byte-mutated inputs through `catch_unwind`, requiring deterministic errors rather than parser panics or acceptance of tampered indexes.
- Added an explicit `security_property` CI step alongside the complete native test suite. Formatting, Cargo check, 248 native tests, strict Clippy in CI, cross-platform builds, and the v2.1.0 release checksum gates remain enforced.
- Added runtime workspace confinement for filesystem builtins. Relative and absolute paths are resolved against the active project workspace, parent traversal is rejected, and existing symlinks are canonicalized before containment checks so reads and writes cannot escape the workspace.
- Added adversarial filesystem regression coverage for parent traversal and symlinks targeting outside files, plus an independent `filesystem_builtins` CI corpus step.
- Added deterministic lexer and parser corpus coverage for huge numeric literals, unterminated strings, unknown punctuation, malformed indentation and delimiters, broken nested syntax, panic-free repeated parsing, and monotonic token spans.
- Added JSON conversion security coverage for malformed tagged variants, oversized integers, recursive malformed input, deterministic conversion, and panic-free rejection.
- Added lockfile security coverage for unsupported versions, incomplete or duplicate fields, invalid escapes, traversal-like package names, strict quoted values, deterministic rejection, and panic-free parsing.
- Added an explicit `parser JSON lockfile corpus` CI gate running `adversarial_corpus`, `malformed_program_corpus`, `json_security_corpus`, `malformed_lockfile_corpus`, and `lockfile_quoted_values` independently of the complete native suite.
- Hardened Unix release packaging for reproducibility by normalizing archive order, timestamps, ownership, numeric ownership, and gzip metadata; CI rebuilds each Unix archive and requires byte-for-byte equality before upload.
- Replaced Windows `Compress-Archive` packaging with a deterministic .NET ZIP writer that sorts slash-separated file entries, fixes entry timestamps to the Unix epoch, uses stable compression settings, and rebuilds the archive byte-for-byte before upload; content and SHA-256 verification remain enforced.
- Added cross-platform installer verification for clean Unix homes and Windows user profiles. Release archives now include uninstall scripts, and CI verifies installation, version reporting, executable launch, reinstall/upgrade, uninstall cleanup, archive contents, and SHA-256 metadata on the matching platform.
- Added the `stdlib_security_corpus` adversarial test gate for oversized typed-JSON input, runtime category mismatches, Unicode index boundaries, duration overflow, structured-log limits, and oversized atomic-write content. Each case is repeated under `catch_unwind` to require deterministic rejection without a panic.

### Async and tooling

- Added `textDocument/documentSymbol` support to the LSP and recursive nested-symbol indexing for function and class bodies. Symbols include deterministic ranges, selection ranges, details, and child declarations; regression coverage verifies nested symbols in both class and function scopes.
- Added module-aware workspace-symbol indexing for explicit local imports. The indexer safely canonicalizes and bounds imported files, rejects traversal-like paths, deduplicates nested modules, and returns deterministic symbols from unopened local package files; regression coverage verifies safe discovery and traversal exclusion.
- Added the bounded `ThreadedRuntime` standard-library adapter for production-oriented blocking work: fixed worker scheduling, task admission limits, wakeable cross-thread joins, panic-to-error conversion, and capped asynchronous regular-file reads. Regression coverage verifies parallel execution, admission bounds, wake-up behavior, panic propagation, and file-size limits; the bilingual async/LSP guide documents the security contract.
- Added bounded non-blocking TCP request/response exchange and asynchronous process execution on `ThreadedRuntime`. Socket operations use deadlines, non-blocking polling, and response caps; process operations use null stdin, separately drained stdout/stderr, hard deadlines, output caps, and structured status reporting. Regression coverage verifies socket round trips, oversized responses, cross-platform process output, capped output, and deadline failures. Forced cancellation of arbitrary blocking system calls remains explicitly outside this adapter contract.
- Added cancellation-aware child-process execution that terminates the child on cancellation or deadline, drains bounded output, and resolves with deterministic status/error results without claiming arbitrary foreign blocking-call interruption. Added a controlled authenticated loopback registry service with `zap registry serve`, bounded HTTP parsing, safe in-root paths, atomic signed-index persistence, managed shutdown, and deterministic unauthorized, traversal, malformed, and oversized-request rejection. Public deployment controls such as TLS termination, ingress policy, external supervision, sandboxing, quotas, and egress controls remain explicit boundaries.

- Added deterministic `AsyncRuntime::spawn_joinable(future)` task submission with `JoinHandle<T>::is_ready()` and future-based output joining.
- Propagated `SpawnError::TaskLimitReached` from joinable task admission, preserving runtime task order, poll budgets, Rust 1.75 compatibility, and the no-worker-thread execution model.
- Added regression coverage for successful joined output and task-limit errors; documented the first async slice in `docs/ASYNC_RUNTIME_EN.md` and `docs/ASYNC_RUNTIME_MM.md`.
- Added `AsyncRuntime::spawn_joinable_cancellable(future)`, which returns a `CancellationToken` and resolves cancelled joins as `JoinError::Cancelled` without polling the inner future after cancellation.
- Added deterministic `timeout_ticks(future, ticks)`, which propagates `TimeoutError` based on executor polls rather than wall-clock time; regression tests cover cancellation, timeout failure, and successful completion paths.
- Added `spawn_joinable_result(future)` and `spawn_joinable_result_cancellable(future)`, preserving typed task failures through `TaskJoinError::Failed(E)`. Cancellation is checked before inner polling, repeated joins return `AlreadyJoined`, and regression coverage verifies typed failure, cancellation precedence, and repeated joins. Updated the bilingual async runtime guides.
- Added language-level task facade builtins `spawn`, `task_join`, and `task_is_ready` in both evaluator and legacy expression paths. The eager Future boundary, strict arity/type diagnostics, async spawn/readiness/join behavior, and invalid-input regressions are documented in the bilingual async runtime and async/LSP guides.
- Synchronized formatter, LSP, and VS Code tooling with the finalized async task vocabulary. LSP completion now describes `spawn`, `task_join`, and `task_is_ready`; the TextMate grammar highlights them as builtins; and extension smoke validation rejects grammar drift.

### Standard library

- Added `file_metadata(path)`, returning portable `kind`, byte `size`, and `readonly` fields from symlink-safe metadata.
- Added bounded `atomic_write(path, content)`, which writes and synchronizes a same-directory temporary file before committing it with rename semantics and cleans up failed temporary writes.
- Added `from_json_typed(source, expected)` for deterministic runtime-category validation under the existing 8 MiB JSON limit.
- Added Unicode-safe `char_at`, `substring`, and `codepoints` operations that index by Unicode scalar values rather than UTF-8 bytes, with stable bounds diagnostics.
- Added deterministic `entries(map)` and `enumerate(list)` collection helpers with bounded output and stable runtime errors.
- Added UTC time APIs: `utc_now()` returns seconds and millisecond timestamps, while `duration_parts(milliseconds)` and `duration_between(end_millis, start_millis)` provide signed, checked decomposition with overflow diagnostics.
- Added focused time regression tests covering UTC timestamp consistency, positive and negative durations, and invalid inputs; documented the APIs in `docs/STDLIB_TIME_EN.md` and `docs/STDLIB_TIME_MM.md`.
- Added deterministic structured logging builtins: `log_record(level, message, fields)` returns a validated map, while `log_json(level, message, fields)` returns canonical JSON with sorted field names.
- Bounded structured logging with an 8 KiB message limit, 64-field limit, 256-byte field-name limit, and 64 KiB encoded-output limit; added regression coverage for ordering, accepted levels, validation errors, and safety limits; documented the APIs in `docs/STDLIB_LOGGING_EN.md` and `docs/STDLIB_LOGGING_MM.md`.


## [2.0.4] — 2026-08-20

### Package reliability

- Extended registry-backed lockfiles to version 2 with deterministic `[resolved]` entries for selected package versions, sources, and SHA-256 checksums.
- Made `zap lock` and `zap update` record resolved transitive packages, while `zap install` re-resolves and verifies the complete pinned graph.
- Preserved compatibility with existing v1 lockfiles and added integration coverage for offline cache reuse and checksum-pinned installs.

### Security audit remediation

- Added `ZAP_UNTRUSTED=1` restricted mode that denies filesystem, environment, process, network, local HTTP serving, and local registry-source capabilities by default.
- Added SSRF defenses for loopback, private, link-local, unspecified, broadcast, IPv6 unique-local, and IPv6 link-local destinations; automatic HTTP redirects are disabled in restricted mode.
- Added bounded HTTP request bodies, hard child-process deadlines with termination, and regression tests for capability denial, private destinations, and oversized request bodies.
- Documented the remaining requirement for OS-level sandboxing, least-privilege deployment, resource quotas, and network egress controls.

### Documentation and editor integration

- Updated the main README with the v2.0.4 installation links, current project status, security-mode note, and the official VS Code Marketplace v0.5.0 installation path.
- Synchronized the extension manifest and documentation with Marketplace publisher `ArkarYan` and added `code --install-extension ArkarYan.zap-language-support`.

## [2.0.3] — 2026-08-20

Zap 2.0.3 completes the P3.3 Production Standard Library milestone.

### Added

- Bounded `url_parse`, `url_encode`, and `url_decode` builtins with deterministic validation.
- `http_get` and `http_request` builtins restricted to HTTP/HTTPS URLs with bounded timeouts and response size.
- Direct non-shell `process_run` with text arguments, UTF-8 stdout/stderr capture, status reporting, and output limits.
- Safe configuration helpers: `env_get` with deterministic defaults, platform-aware `config_dir`, and traversal-resistant `config_path` for one relative file name.
- Bounded `http_serve_once` local server that binds to loopback, serves one request, and enforces request, response, and wait limits.
- Deterministic standard-library catalog entries and bilingual documentation for the new APIs.
- Initial `vscode-extension` folder with `.zp` registration, TextMate syntax highlighting, snippets, completion, CLI-backed diagnostics, workspace checking, and run-current-file commands.
- Native and VS Code LSP signature help for Zap function calls, including active-parameter tracking after `(` and `,`.
- LSP-backed document formatting with normalized line endings, four-space indentation, and trailing-whitespace cleanup.

### Verification

- Native suite: **235 tests passed**.
- Focused P3.3 URL, process, HTTP validation, configuration, local-server argument, and compatibility regressions passed.
- Cross-platform test hardening covers native path separators in evaluator expectations, JSON-escaped Windows file fixtures, option-aware URL-port assertions, and CRLF-safe process-output normalization in the Windows smoke gate.
- The Linux native suite passed **235 tests**; Windows and macOS target-native tests, strict Clippy, and release packaging remain enforced by GitHub Actions.

## [2.0.1] — 2026-08-20

Zap v2.0.1 is a production-quality maintenance release following the P2 Ecosystem foundation. It completes the P3.1 module/workspace architecture slice and closes the remaining v2 audit findings.

### Added and fixed

- Explicit `module <name>` declarations and `import <path> as <alias>` syntax with deterministic manifest-backed resolution.
- Recursive multi-module graph validation with traversal protection, missing-target diagnostics, repeated-import caching, and full circular-dependency chains.
- LSP indexing for module declarations and import aliases across completion, hover, definitions, and workspace symbols.
- Stable runtime `Error` and `KeyError` diagnostic categories with structured human-readable and JSON output.
- Declaration-time annotation validation for supported scalar and generic forms.
- Canonical CLI help and usage output for help, invalid arguments, and invalid paths.
- JSON-RPC `-32601 Method not found` responses for unknown LSP requests while preserving notification behavior.
- Collection literal parsing and AST/legacy parity for `join` and map-key `contains` behavior.
- Cross-process integration and end-to-end tests for annotation errors, CLI help, and framed LSP requests.
- Hardened cross-platform GitHub Actions release packaging for Linux x86_64, macOS ARM64, and Windows x86_64, including archive checksums and smoke tests.

### Verification

- Native unit and integration suite: **229 tests passed**.
- Audit regression and end-to-end tests: **3 passed**.
- Formatting, whitespace, release-build, CLI smoke, example execution, and package checksum checks passed.
- GitHub Actions release workflow validates tag/Cargo version matching and publishes verified platform archives.

## [2.0.0] — 2026-08-20

Zap P2 completes the Ecosystem milestone for the native runtime, deterministic package registry, async foundation, and editor integration.

### Added

- Deterministic exact, caret, tilde, and comparator version-range selection for registry dependencies.
- HTTPS registry transport, SHA-256 artifact verification, signed-index HMAC verification, deterministic cache pruning, offline reuse, and authenticated local registry persistence.
- Checksum-verified package publishing with atomic artifact storage and signed index rewriting.
- `async fn`, deterministic `Future` values, `await`, poll-based timers, cancellation tokens, cancellable tasks, task limits, poll budgets, and deterministic suspension controls.
- LSP document synchronization, diagnostics, hover, context-aware completion, formatting, go-to-definition, and workspace symbols.
- English and Burmese documentation updates covering the complete P2 foundation.

### Verification

- Native test suite: **223 tests passed**.
- Formatting, `cargo check`, whitespace, and strict Clippy gates passed.
- Linux x86_64, Windows x86_64, and macOS ARM64 CI checks passed.
- Release artifacts are generated by the tag-triggered GitHub Actions workflow.

## [2.0.2] — 2026-08-20

### P3.2 Structured Error Model

- Added `raise <expression>` and same-level `try`/`catch <binding>:` syntax with deterministic parser diagnostics for bare `raise`, malformed bindings, missing catches, and missing catch bodies.
- Implemented structured raise propagation through functions, loops, nested blocks, and modules, including catch-binding restoration and re-raise behavior.
- Preserved uncaught raised values as stable process-boundary diagnostics using the `raised error: <value>` format.
- Verified the native suite with 229 passing tests while preserving Rust 1.75 compatibility and deterministic AST/legacy behavior.

### P2 Ecosystem progress

- Added HTTPS registry index and artifact transport, content-addressed caching, and SHA-256 integrity enforcement.
- Added validated remote publishing and deterministic nested dependency traversal with cycle diagnostics.
- Added a stable-Rust-compatible single-threaded async runtime foundation with `async fn`, deterministic `Future` values, and `await` expressions.
- Extended the stdio JSON-RPC LSP with text synchronization, deterministic diagnostics, parser-span hover, and context-aware completion.
- Synchronized the English and Burmese P2 roadmaps, async/LSP guides, and syntax references.

### Verification

- Native test suite: **223 tests passed**.
- Formatting, `cargo check`, and `git diff --check` pass locally.
- Strict Clippy and Linux, Windows, and macOS ARM64 checks pass in GitHub Actions.
- No release tag is created until every P2 track is green and verified.

## [1.0.0] — 2026-08-20

Zap P1 establishes the first complete Language Core milestone for the standalone native runtime. This release focuses on predictable language semantics, direct AST execution, safe diagnostics, and a stable foundation for future ecosystem work.

### Added

- Direct AST evaluation for functions, methods, closures, indexing, built-ins, filesystem, JSON, environment, path, and time helpers.
- Default parameters and named arguments for user-defined functions, methods, and closures.
- Static type narrowing for `option<T>` and `result<T>` guards, complex boolean conditions, aliases, and `else` branch restoration.
- OOP method and field visibility with `public`, `private`, and `protected` access rules.
- Module-aware private access checks and protected inheritance behavior.
- Constructor visibility, field default initialization, explicit `super.init()` delegation, and single implicit parent-constructor delegation.
- Stabilized text, math, collection, filesystem, JSON, environment, path, and time standard-library APIs.
- Deterministic public standard-library domain catalog and bilingual API indexes.
- Canonical `zap.lock` generation, sorted dependency entries, missing/stale lockfile rejection, and deterministic local dependency validation.
- Structured diagnostics, JSON diagnostics, source locations, secret redaction, and runtime resource limits.
- Cross-platform CI smoke checks for Linux, macOS, and Windows CLI version, help, and example execution.

### Documentation

- Updated the main README and bilingual learning guides.
- Added type-narrowing, package/lockfile, and public standard-library indexes in English and Burmese.
- Synchronized the P1 progress roadmap and release documentation.

### Verification

- 109 native tests pass: 31 unit tests and 78 integration tests.
- Formatting, whitespace, release build, CLI version/help, and runnable example checks pass locally.
- The GitHub Actions release workflow performs stable Rust formatting, Clippy, check, test, version/tag matching, and Linux/macOS/Windows artifact verification.

### Scope

P1 intentionally does not include remote package registries, package publishing, async execution, or LSP/editor integration. Those belong to the P2 Ecosystem roadmap.

## Previous releases

See the historical entries in [`CHANGELOG.md`](CHANGELOG.md).
