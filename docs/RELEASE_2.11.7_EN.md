# Zap v2.11.7 Release Notes

**Release line:** v2.11.7
**Scope:** Malformed-source no-panic safety regression gate
**Status:** Incremental release-engineering and safety-evidence release

## Summary

Zap v2.11.7 adds a deterministic malformed-source safety harness for the native CLI. The harness exercises a small invalid-source corpus covering a malformed generic annotation, an unknown annotation, and an incompatible annotation. Each case must terminate with a nonzero status and must not emit panic, unchecked-`unwrap`, unchecked-`expect`, or stack-backtrace signatures.

The regression is required in CI and release preflight. It supplements the existing parser, adversarial-input, and malformed-program corpus checks; it does not replace them and it does not claim that every possible malformed program has been exhaustively fuzzed.

The bootstrap stage remains **B0**. Rust remains the reference owner for complete compiler and runtime semantics. This release does not claim a fully Zap-only compiler, self-hosting, B4, complete panic-freedom, or complete fuzz coverage.

## Changes

| Area | Change | Boundary |
|---|---|---|
| Source safety | Added `scripts/test_malformed_source_safety.sh` with timeout and panic-signature checks. | Small deterministic corpus only |
| CI | Added the malformed-source regression to the required quality job. | Failure remains fail-closed |
| Release preflight | Added the script to required release files and preflight gates. | Required for future candidates |
| Documentation | Updated current-status pages, TODO checkpoint, and bilingual release metadata. | B0 boundary unchanged |

## Verification contract

The safety harness uses the release binary when available and otherwise builds the locked native binary. Every fixture must fail with a nonzero exit status, must not time out, and must not contain panic or unchecked-failure signatures. The broader parser and malformed-program suites remain separate required evidence.

## Historical and release policy

The published v2.11.6 release remains immutable. v2.11.7 uses a new annotated tag and does not rewrite or retag v2.11.4, v2.11.5, or v2.11.6. Historical changelog entries and prior release notes remain unchanged.

## References

[1]: ../scripts/test_malformed_source_safety.sh
[2]: ../scripts/release_preflight.sh
[3]: ../.github/workflows/ci.yml
[4]: ../docs/CURRENT_STATUS_EN.md
[5]: ../docs/BOOTSTRAP_CONTRACT_EN.md
