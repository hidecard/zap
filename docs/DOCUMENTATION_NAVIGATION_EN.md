# Zap Documentation Navigation

**Verified baseline:** Zap v2.11.6
**Purpose:** This page is the English entry point for learners, language users, package authors, runtime maintainers, and release operators. Normative behavior belongs to the canonical specification or an explicitly linked contract; explanatory guides must not silently override those contracts.

## Choose a path

| Audience | Start here | Continue with |
|---|---|---|
| New learner | [Language Guide](LEARN_ZAP_EN.md) | [Usage guide](USAGE_EN.md), [Syntax reference](SYNTAX_GUIDE_EN.md), [examples](../examples) |
| Language user | [Syntax reference](SYNTAX_GUIDE_EN.md) | [Language specification](LANGUAGE_SPEC_EN.md), [type-check matrix](TYPECHECK_CONFORMANCE_MATRIX_EN.md) |
| Package author | [Package guide](PACKAGE_EN.md) | [Stdlib index](STDLIB_INDEX_EN.md), [registry/authentication contract](REGISTRY_AUTH_EN.md) |
| Framework contributor | [Framework guide](FRAMEWORK_EN.md) | [Zap-first Web guide](ZAP_WEB_NATIVE_EN.md), [Web Framework guide](WEB_FRAMEWORK_EN.md), [frontend integration guide](FRONTEND_INTEGRATION_EN.md), [production deployment](PRODUCTION_DEPLOYMENT_EN.md), [database operations](DATABASE_PRODUCTION_EN.md), [OAuth2/JWT authentication](AUTH_OAUTH2_JWT_EN.md), [load/chaos testing](LOAD_CHAOS_TESTING_EN.md), [zap-host adapter](ZAP_HOST_EN.md), [zap-host quickstart](ZAP_HOST_QUICKSTART_EN.md), [Framework starters](../frameworks), [ecosystem roadmap](ECOSYSTEM.md), [package guide](PACKAGE_EN.md) |
| Runtime maintainer | [Language specification](LANGUAGE_SPEC_EN.md) | [Memory model](MEMORY_MODEL_EN.md), [diagnostics](DIAGNOSTIC_MODEL_EN.md), [async boundaries](ASYNC_BOUNDARIES_EN.md), [bootstrap/self-hosting contract](BOOTSTRAP_CONTRACT_EN.md) |
| Tooling contributor | [Async/LSP guide](ASYNC_LSP_EN.md) | [LSP implementation](../native/src/lsp.rs), [canonical VS Code extension](../vscode-extension), [editor assets](../editors/vscode), [semantic-parity validator](../scripts/test_lsp_semantic_parity.sh), [protocol synchronization contract](../scripts/test_lsp_protocol_sync.sh) |
| Deployment operator | [Deployment guide](DEPLOYMENT_EN.md) | [Production operations](PRODUCTION_OPERATIONS_EN.md), [Registry deployment policy](../deploy/registry-deployment-policy.toml), [security policy](../SECURITY.md), [RustSec audit evidence](RUSTSEC_AUDIT_EN.md) |
| Release operator | [Release version policy](RELEASE_VERSION_POLICY_EN.md) | [Current status](CURRENT_STATUS_EN.md), [Release signing](RELEASE_SIGNING_EN.md), [rollback runbook](RELEASE_ROLLBACK_RUNBOOK_EN.md), [benchmark contract](BENCHMARK_HARNESS_EN.md), [RustSec audit evidence](RUSTSEC_AUDIT_EN.md), [post-v2.2.0 remediation/provenance](POST_V2.2.0_REMEDIATION_EN.md), [post-v2.4.0 remediation/provenance](POST_V2.4.0_REMEDIATION_EN.md), [v2.11.6 release notes](RELEASE_2.11.6_EN.md), [release preflight](../scripts/release_preflight.sh) |
| Language designer | [Traits/composition RFC](TRAITS_RFC_EN.md) | [Language specification](LANGUAGE_SPEC_EN.md), [compatibility template](COMPATIBILITY_CHANGE_TEMPLATE_EN.md) |

## M3-DOC-01 checked surfaces

| Audience section | Verified entry point | Canonical companion |
|---|---|---|
| Learner | [Language Guide](LEARN_ZAP_EN.md) — v2.11.6 | [Syntax reference](SYNTAX_GUIDE_EN.md) |
| Language user | [Syntax reference](SYNTAX_GUIDE_EN.md) — v2.11.6 | [Language specification](LANGUAGE_SPEC_EN.md) |
| Package author | [Package guide](PACKAGE_EN.md) — v2.11.6 | [Stdlib reference](STDLIB_INDEX_EN.md), [registry contract](REGISTRY_AUTH_EN.md) |
| Framework contributor | [Framework guide](FRAMEWORK_EN.md) — v2.11.6 | [Zap-first Web guide](ZAP_WEB_NATIVE_EN.md), [Web Framework guide](WEB_FRAMEWORK_EN.md), [frontend integration guide](FRONTEND_INTEGRATION_EN.md), [production deployment](PRODUCTION_DEPLOYMENT_EN.md), [database operations](DATABASE_PRODUCTION_EN.md), [OAuth2/JWT authentication](AUTH_OAUTH2_JWT_EN.md), [load/chaos testing](LOAD_CHAOS_TESTING_EN.md), [zap-host adapter](ZAP_HOST_EN.md), [zap-host quickstart](ZAP_HOST_QUICKSTART_EN.md), [Framework starters](../frameworks), [ecosystem roadmap](ECOSYSTEM.md) |
| Runtime maintainer | [Memory model](MEMORY_MODEL_EN.md) — v2.11.6 | [Runtime state](RUNTIME_STATE_EN.md), [memory budget](MEMORY_BUDGET_OBJECT_STORE_EN.md) |
| Deployment/security operator | [Deployment/security boundaries](DEPLOYMENT_EN.md) — v2.11.6 | [Production operations](PRODUCTION_OPERATIONS_EN.md), [Security policy](../SECURITY.md), [release signing](RELEASE_SIGNING_EN.md), [RustSec audit evidence](RUSTSEC_AUDIT_EN.md) |

## Normative contract map

| Domain | Canonical contract | Executable evidence |
|---|---|---|
| Language semantics | [Language specification](LANGUAGE_SPEC_EN.md) | [Specification ownership index](SPEC_OWNERSHIP_INDEX.tsv) |
| Diagnostics | [Diagnostic model](DIAGNOSTIC_MODEL_EN.md) | Native diagnostic tests |
| Memory and borrowing | [Memory model](MEMORY_MODEL_EN.md) | Borrow and memory-limit regressions |
| Memory budget/object store | [MemoryBudget and ObjectStore contract](MEMORY_BUDGET_OBJECT_STORE_EN.md) | Run-owned budget and object-store isolation regressions |
| Runtime state | [Runtime state and execution context](RUNTIME_STATE_EN.md) | Runtime-state isolation, workspace, and reset regressions |
| AST foundation | [AST foundation status](P0_FOUNDATION_STATUS_EN.md) | Canonical AST, export, and compatibility-boundary regressions |
| Bootstrap/self-hosting | [Bootstrap and self-hosting contract](BOOTSTRAP_CONTRACT_EN.md) | B0 status, canonical token/AST/diagnostic artifacts, and B0 verifier |
| Async boundaries | [Async boundary contract](ASYNC_BOUNDARIES_EN.md) | Async runtime and adapter tests |
| Framework adapters | [Framework guide](FRAMEWORK_EN.md) · [Zap-first Web guide](ZAP_WEB_NATIVE_EN.md) · [Web Framework guide](WEB_FRAMEWORK_EN.md) · [frontend integration guide](FRONTEND_INTEGRATION_EN.md) · [production deployment](PRODUCTION_DEPLOYMENT_EN.md) · [database operations](DATABASE_PRODUCTION_EN.md) · [OAuth2/JWT authentication](AUTH_OAUTH2_JWT_EN.md) · [load/chaos testing](LOAD_CHAOS_TESTING_EN.md) · [zap-host adapter](ZAP_HOST_EN.md) · [zap-host quickstart](ZAP_HOST_QUICKSTART_EN.md) | Zap-first scaffold checks, starter smoke tests, Web contract tests, host-capability contract tests, and Axum/Tower adapter tests |
| Standard library | [Stdlib index](STDLIB_INDEX_EN.md) · [Stability policy](STDLIB_POLICY_EN.md) | Machine-readable builtin catalog, stability/deprecation metadata, schema-2 determinism classes, and security corpus |
| Native/legacy compatibility | [P0-01 parity matrix](P001_PARITY_MATRIX_EN.md) | `scripts/test_p001_parity.sh` |
| Verification/replay | [P1-05 replay and M2-VERIFY-01 bounded replay contract](P105_REPLAY_EN.md) | `scripts/test_p105_replay.sh` and `scripts/test_m2_verify_replay.sh` |
| Performance | [Benchmark harness](BENCHMARK_HARNESS_EN.md) | Checked-in `benchmark-results/native-summary.csv` and CI threshold gate |
| Security/dependencies | [RustSec audit evidence](RUSTSEC_AUDIT_EN.md) · [Security policy](../SECURITY.md) | CI and release `cargo-audit` gates |
| Releases | [Current status](CURRENT_STATUS_EN.md) · [Release version policy](RELEASE_VERSION_POLICY_EN.md) · [post-v2.2.0 remediation/provenance](POST_V2.2.0_REMEDIATION_EN.md) | Versioned manifest, signed provenance asset, `scripts/validate_release_version.sh`, and release preflight |

## Version and contribution rules

The authoritative package version is `native/Cargo.toml`. Release-facing surfaces must agree with it, and CI validates the agreement. When a normative rule changes, update the English and Burmese contract together, add or update its fixture owner in `SPEC_OWNERSHIP_INDEX.tsv`, record compatibility impact using the bilingual compatibility template, and include regression evidence before merging. Public standard-library changes must also update the catalog and its stability policy pair.

Documentation changes must preserve the English/Burmese pair, use repository-relative links, identify deferred behavior explicitly, and avoid claiming production scheduling, cancellation, sandboxing, or performance guarantees that are not covered by executable gates. Framework changes must update the Framework guide pair, starter manifests/lockfiles, and the host-adapter boundary without adding unsupported core syntax. The [post-v2.4.0 remediation/provenance record](POST_V2.4.0_REMEDIATION_EN.md) distinguishes immutable v2.4.0 assets from later `master` corrections. See the [v2.11.6 release notes](RELEASE_2.11.6_EN.md), [language specification](LANGUAGE_SPEC_EN.md), and [release version policy](RELEASE_VERSION_POLICY_EN.md) for the current release boundary and maintained work.
