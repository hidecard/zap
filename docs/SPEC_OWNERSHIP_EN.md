# P0-02-A Specification Ownership Index

## Purpose

The machine-readable [`SPEC_OWNERSHIP_INDEX.tsv`](SPEC_OWNERSHIP_INDEX.tsv) is the executable ownership index for the canonical Zap specification. Its current 37 rows cover source execution, precedence, typing, first-class callable functions, modules, memory and budget boundaries, deterministic and production async behavior, LSP synchronization/interoperability/rename, diagnostics, registry transport and security, lockfiles, JSON/filesystem limits, standard-library determinism, benchmark provenance, release versioning, CLI JSON, compatibility policy, runtime borrow/equality safety, and CI enforcement. Each public rule row names its English section, Burmese section, implementation or conformance fixture owner, implementation status, and compatibility class. The index prevents a rule from being normative only because it appears in an old guide or happens to be accepted by the legacy runtime.

## Required row fields

| Field | Contract |
|---|---|
| `rule_id` | Stable `SPEC-NNN` identifier; IDs are unique and must not be silently reused for a different rule |
| `domain` | Short semantic domain such as `values-typing`, `diagnostics`, or `async-deterministic` |
| `canonical_en` | Repository-relative English specification path plus a section reference |
| `canonical_mm` | Repository-relative Burmese specification path plus a section reference |
| `fixture_owner` | Repository-relative source, test, matrix, or script path, optionally followed by `#fragment` |
| `status` | `implemented` or `deferred` |
| `compatibility` | One of `normative`, `compatibility`, `deprecated`, or `rejected` |

The validator checks that both bilingual documents and referenced sections exist, that every fixture owner exists and contains its named fragment, that policy values are valid, that rule IDs are unique, and that all required semantic domains are represented. Its report is deterministic TSV output with a per-row `PASS` or `FAIL` decision.

## Ownership rules

The canonical specification owns semantics. Implementation modules own executable behavior within that semantic boundary: the AST/parser owns syntax construction, the evaluator owns runtime expression and statement behavior, diagnostics owns structured error fields, the registry owns package transport and integrity, and CI owns enforcement. A cross-cutting rule must still have one canonical bilingual section and may reference multiple implementation tests through a stable fixture owner policy.

A new public rule is incomplete until it has an index row, a bilingual section or an explicit cross-link to a normative subcontract, and a passing or intentionally failing fixture. A deferred rule must remain labeled `deferred` rather than being described as implemented in release documentation. Compatibility behavior must be classified explicitly; legacy acceptance alone cannot promote it to `normative` status. Future behavior changes must use the bilingual [`COMPATIBILITY_CHANGE_TEMPLATE_EN.md`](COMPATIBILITY_CHANGE_TEMPLATE_EN.md) and [`COMPATIBILITY_CHANGE_TEMPLATE_MM.md`](COMPATIBILITY_CHANGE_TEMPLATE_MM.md) records.

## Validation command and CI artifact

Run the ownership gate locally with:

```text
ZAP_SPEC_OWNERSHIP_REPORT=target/spec-ownership-report.tsv scripts/validate_spec_ownership.sh
```

GitHub Actions runs the same command in the Rust quality job and uploads `target/spec-ownership-report.tsv` as a commit-named artifact. The index may continue to expand to every fragmented rule. The current expansion explicitly owns the post-review LSP protocol and interoperability contracts, schema-2 standard-library determinism, logical memory budgets, checked runtime borrow/equality safety, registry transport limits, benchmark provenance, and release-version validation; future rows must preserve the stable IDs, required-domain coverage, and bilingual ownership fields introduced here.
