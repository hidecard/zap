# Zap Compatibility Policy

**Specification status:** Normative for B0→B4 transition
**Owner:** `docs/COMPATIBILITY_POLICY_EN.md`
**Validation:** `scripts/validate_compatibility_policy.sh`

## 1. Purpose

This document classifies every behavior deviation between the Rust reference implementation and the emerging Zap-owned compiler stages. It prevents Rust-accepted behavior from becoming normative by accident, and it gives users a deterministic migration path when behavior changes.

## 2. Behavior classes

| Class | Meaning | Allowed in stable release | Required action |
|---|---|---|---|
| `normative` | Canonical Zap behavior defined by the language specification | Yes | None beyond specification |
| `compatibility` | Accepted today but not normative; may become normative only through RFC | Yes | Document in release notes; do not introduce new `compatibility` behavior without RFC |
| `deprecated` | Accepted today with deterministic warning; removal requires RFC and one major-version notice | Yes | Emit `ZAP-DEPRECATED-*` diagnostic; schedule removal in next major version |
| `rejected` | Parser/runtime rejects with stable diagnostic | Yes | None; fixture must assert rejection |
| `native-only` | Implementation detail with no user-visible contract | Yes | May change without notice; never expose in public API or diagnostics |

## 3. Stable diagnostic contract

Every user-facing diagnostic must carry:

```json
{
  "code": "ZAP-LEX-001",
  "severity": "error",
  "line": 1,
  "column": 1,
  "message": "invalid character",
  "notes": ["..."],
  "help": "..."
}
```

- `code` is stable and never reused for a different error class.
- `severity` is one of `error`, `warning`, `info`.
- `line`, `column`, and `source_name` form a stable span.
- `notes` and `help` are optional but must be deterministic when present.
- Exit status is `0` for success, `1` for user-source error, `2` for tool/internal error.

## 4. Differential verification rule

During B1–B3, every owned corpus fixture must be run through both:
1. The Zap-owned candidate path (`bootstrap/b1/lexer.zp`, `bootstrap/b1/parser.zp`, `bootstrap/b2/typecheck.zp`, etc.)
2. The Rust reference path (`cargo run -- bootstrap tokens|ast|typed-ir|diagnostics <file>`)

If the outputs diverge, the divergence must be recorded as one of:
- **Zap bug** — fix the Zap candidate
- **Rust legacy** — document the Rust behavior as `compatibility` or `deprecated` in this policy, add a fixture asserting the new normative behavior, and update the specification
- **Intentional deviation** — record in `bootstrap/contracts/DEVIATIONS.tsv` with `rule_id`, `fixture`, `rust_behavior`, `zap_behavior`, `rationale`, `version`

## 5. No hidden fallback rule

The Zap compiler MUST NOT silently invoke the Rust evaluator, parser, or type checker when the Zap-owned path fails. Fallbacks are forbidden unless:
1. The fallback mode is explicitly opt-in via CLI flag (e.g., `--reference-oracle`)
2. The diagnostic output clearly states which path was used
3. The CI default runs with fallbacks disabled

## 6. Specification change workflow

1. **Draft RFC** in `docs/rfc/` with `rule_id`, `proposed_change`, `rationale`, `affected_fixtures`, `migration_guide`
2. **Bilingual review** — update `LANGUAGE_SPEC_EN.md` and `LANGUAGE_SPEC_MM.md`
3. **Fixture update** — add positive/negative corpus, update `SPEC_OWNERSHIP_INDEX.tsv`
4. **Compatibility decision** — record in this policy; if behavior changes, classify old behavior as `compatibility` or `deprecated`
5. **Changelog** — record in `CHANGELOG.md`, `CHANGELOG_EN.md`, `CHANGELOG_MM.md`
6. **CI gate** — `scripts/validate_compatibility_policy.sh` must pass

## 7. Version ownership

- Language version, compiler version, standard-library version, and token/AST/diagnostic/typed-IR/manifest/lockfile schemas are defined in `bootstrap/contracts/VERSIONS.toml`.
- A schema change requires a major-version bump unless it is backward-compatible additive.
- The platform-seed version starts at `0` and increments only when the seed ABI changes.

## 8. Bootstrap-stage transition rule

A stage transition (B0→B1→B2→B3→B4) requires:
1. All acceptance rows in `bootstrap/contracts/B4_ACCEPTANCE.tsv` (or the stage-specific gate) are `pass`
2. No `provisional` or `provisional-pending-capture` rows remain in `bootstrap/contracts/OWNERS.tsv` for the transitioning components
3. Clean-environment rebuild passes without Rust/Cargo in `PATH`
4. Deterministic byte-for-byte rebuild evidence is uploaded as a CI artifact
5. Cross-platform verification passes on Linux x86_64, Windows x86_64, and macOS ARM64

## 9. Rust reference retirement

After B4 certification:
- The Rust implementation moves to `legacy/` or `reference/`
- Normal `zap build`, `zap test`, `zap run`, `zap compiler rebuild` paths MUST NOT depend on `cargo`, `rustc`, or `rustup`
- `zap --reference-oracle` may still use Rust for differential testing, but this mode must be explicitly requested
- Release artifacts are scanned for Rust/Cargo dependency strings; presence in the user-facing path fails the release gate
