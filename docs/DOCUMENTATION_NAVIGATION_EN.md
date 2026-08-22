# Zap Documentation Navigation

**Verified baseline:** Zap v2.1.14
**Purpose:** This page is the English entry point for learners, language users, package authors, runtime maintainers, and release operators. Normative behavior belongs to the canonical specification or an explicitly linked contract; explanatory guides must not silently override those contracts.

## Choose a path

| Audience | Start here | Continue with |
|---|---|---|
| New learner | [Learning guide](LEARN_ZAP_EN.md) | [Syntax reference](SYNTAX_GUIDE_EN.md), [examples](../examples) |
| Language user | [Syntax reference](SYNTAX_GUIDE_EN.md) | [Language specification](LANGUAGE_SPEC_EN.md), [type-check matrix](TYPECHECK_CONFORMANCE_MATRIX_EN.md) |
| Package author | [Package guide](PACKAGE_EN.md) | [Stdlib index](STDLIB_INDEX_EN.md), [registry/authentication contract](REGISTRY_AUTH_EN.md) |
| Runtime maintainer | [Language specification](LANGUAGE_SPEC_EN.md) | [Memory model](MEMORY_MODEL_EN.md), [diagnostics](DIAGNOSTIC_MODEL_EN.md), [async boundaries](ASYNC_BOUNDARIES_EN.md) |
| Tooling contributor | [Async/LSP guide](ASYNC_LSP_EN.md) | [LSP implementation](../native/src/lsp.rs), [VS Code extension](../vscode-extension) |
| Deployment operator | [Deployment guide](DEPLOYMENT_EN.md) | [Registry deployment policy](../deploy/registry-deployment-policy.toml), [security policy](../SECURITY.md) |
| Release operator | [Release version policy](RELEASE_VERSION_POLICY_EN.md) | [Release signing](RELEASE_SIGNING_EN.md), [rollback runbook](RELEASE_ROLLBACK_RUNBOOK_EN.md), [benchmark contract](BENCHMARK_HARNESS_EN.md) |

## Normative contract map

| Domain | Canonical contract | Executable evidence |
|---|---|---|
| Language semantics | [Language specification](LANGUAGE_SPEC_EN.md) | [Specification ownership index](SPEC_OWNERSHIP_INDEX.tsv) |
| Diagnostics | [Diagnostic model](DIAGNOSTIC_MODEL_EN.md) | Native diagnostic tests |
| Memory and borrowing | [Memory model](MEMORY_MODEL_EN.md) | Borrow and memory-limit regressions |
| Memory budget/object store | [MemoryBudget and ObjectStore contract](MEMORY_BUDGET_OBJECT_STORE_EN.md) | Run-owned budget and object-store isolation regressions |
| Runtime state | [Runtime state and execution context](RUNTIME_STATE_EN.md) | Runtime-state isolation, workspace, and reset regressions |
| AST foundation | [AST foundation status](P0_FOUNDATION_STATUS_EN.md) | Canonical AST, export, and compatibility-boundary regressions |
| Async boundaries | [Async boundary contract](ASYNC_BOUNDARIES_EN.md) | Async runtime and adapter tests |
| Standard library | [Stdlib index](STDLIB_INDEX_EN.md) | Builtin catalog and security corpus |
| Native/legacy compatibility | [P0-01 parity matrix](P001_PARITY_MATRIX_EN.md) | `scripts/test_p001_parity.sh` |
| Verification/replay | [P1-05 replay contract](P105_REPLAY_EN.md) | `scripts/test_p105_replay.sh` |
| Performance | [Benchmark harness](BENCHMARK_HARNESS_EN.md) | Checked-in `benchmark-results/native-summary.csv` and CI threshold gate |
| Releases | [Release version policy](RELEASE_VERSION_POLICY_EN.md) | `scripts/validate_release_version.sh` and release preflight |

## Version and contribution rules

The authoritative package version is `native/Cargo.toml`. Release-facing surfaces must agree with it, and CI validates the agreement. When a normative rule changes, update the English and Burmese contract together, add or update its fixture owner in `SPEC_OWNERSHIP_INDEX.tsv`, record compatibility impact using the bilingual compatibility template, and include regression evidence before merging.

Documentation changes must preserve the English/Burmese pair, use repository-relative links, identify deferred behavior explicitly, and avoid claiming production scheduling, cancellation, sandboxing, or performance guarantees that are not covered by executable gates. See the [remaining TODO register](PDF_REMAINING_TODO_EN.md) and [next-step plan](NEXT_TODO_PLAN_EN.md) for tracked work.
