# Zap v2.1.8 Release Notes

**Release date:** 2026-08-21

## Summary

Zap v2.1.8 is a patch release focused on release-integrity and documentation consistency. It makes the Cargo package version the explicit source of truth for every release-facing surface and prevents ordinary CI branch refs from being mistaken for release tags.

## Included changes

- Added the P0 single-source-of-truth validator covering `native/Cargo.toml`, `native/Cargo.lock`, `zap --version`, optional release tags, changelogs, bilingual README release links and archive names, `SECURITY.md`, conformance metadata, bilingual release notes, the release template, and installer metadata.
- Added deterministic TSV evidence and positive/negative regression coverage for package-version drift, tag drift, and branch refs such as `master`.
- Enforced the version gate in GitHub Actions and `scripts/release_preflight.sh`, with uploaded evidence for review before release publication.
- Added bilingual release-version policy documentation and refreshed onboarding, security, conformance, roadmap, and changelog metadata to the v2.1.8 baseline.

## Compatibility

This patch release does not introduce new language syntax or broaden async or traits semantics. Existing Zap programs remain within the v2.1 language contract. The release-facing change is fail-closed validation: a mismatched package version, CLI version, tag, archive name, or documentation surface blocks publication.

## Verification

The master-branch validation run passed the version gate, regression harness, formatting, strict Clippy, Cargo check, the full 254-test native suite, focused conformance/security/async/parity/ownership gates, and Linux, Windows, and macOS ARM64 build jobs. See the [GitHub Actions run](https://github.com/hidecard/zap/actions/runs/32505190955) for the complete evidence.
