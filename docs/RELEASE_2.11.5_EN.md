# Zap v2.11.5 Release Notes

**Release line:** v2.11.5
**Scope:** Release-gate, provenance, and developer-validation hardening
**Status:** Incremental infrastructure and documentation release

## Summary

Zap v2.11.5 hardens the cross-platform release contract by making the Windows CLI workflow exercise version/help output, example execution, project creation, project checking, locked build, and project tests. A failure in any required Windows smoke operation remains fatal to the platform job and therefore blocks publication.

The release also adds bilingual canonical current-status pages, explicitly documents the signed provenance asset as the machine-readable release identity record, and adds `make doctor` / `scripts/doctor.sh` to distinguish missing local prerequisites from actual test failures.

The bootstrap stage remains **B0**. Rust remains the reference owner for complete compiler and runtime semantics. These changes improve release and developer evidence; they do not claim a fully Zap-only compiler, self-hosting, or B4.

## Changes

| Area | Change | Boundary |
|---|---|---|
| Windows release gate | Added required `zap.exe` version/help, example, `zap new`, `zap check`, `zap build --locked`, and `zap test` smoke operations. | Windows job remains fail-closed |
| Developer diagnostics | Added `scripts/doctor.sh`, `scripts/test_doctor.sh`, and `make doctor`; normal and strict modes distinguish incomplete environments from test failures. | Diagnostic helper, not a test substitute |
| Current status | Added bilingual `docs/CURRENT_STATUS_EN.md` and `docs/CURRENT_STATUS_MM.md` pages for active, completed, provisional, and deferred areas. | Current-status index; historical records remain immutable |
| Release provenance | Documented the versioned manifest/provenance asset fields and linked them from the documentation hubs. | Existing signed release schema remains authoritative |
| Validation | Added current-status pair checks and doctor regression checks to documentation consistency and release preflight. | No bootstrap-stage advancement |

## Open pull requests and branch history

Open PR #13 and PR #14 contain stale README/bootstrap work that is already superseded by the current master documentation and bootstrap contracts. Open PR #1 contains an older security-hardening line whose substantive safeguards are represented by the current master runtime, registry, deployment, and RustSec gates. Existing tags are not rewritten. PR closure is handled separately from source history, and no fork branch is deleted by this release.

## Explicit limits

The B0 bootstrap boundary remains unchanged. B1/B2 candidates remain provisional and corpus-limited. General arbitrary-program parsing, complete diagnostic parity, broad type inference, typed-IR ownership, package/build ownership, VM ownership, platform-seed acceptance, and self-hosting remain deferred.

The latest published v2.11.4 release remains immutable. v2.11.5 is a new tag and does not rewrite or retag v2.11.3 or v2.11.4.

## Verification

The exact committed v2.11.5 candidate must pass version, bilingual documentation, Markdown links, ownership, formatting, bootstrap, native/host tests, dependency audit, deployment policy, and release preflight checks before publication. The final preflight total is recorded from that exact candidate.

## References

[1]: ../docs/CURRENT_STATUS_EN.md
[2]: ../.github/workflows/release.yml
[3]: ../scripts/doctor.sh
[4]: ../scripts/test_doctor.sh
[5]: ../scripts/aggregate_release_manifest.sh
[6]: ../scripts/sign_release_artifacts.sh
