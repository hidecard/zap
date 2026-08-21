# Release Version Single-Source-of-Truth Policy

## Authority

The native package version in `native/Cargo.toml` is the authoritative Zap release version. `native/Cargo.lock` must record the same `zap-native` package version, and the compiled CLI must report the same value through `zap --version`.

No release workflow may publish artifacts when the package version, tag, CLI output, changelogs, bilingual README onboarding, security policy, or release notes disagree. The version is intentionally validated rather than inferred from an older document or manually copied across release surfaces.

## Required release surfaces

| Surface | Required contract |
|---|---|
| `native/Cargo.toml` | Authoritative semantic version |
| `native/Cargo.lock` | Matching `zap-native` package version |
| `zap --version` | Matching CLI output |
| `CHANGELOG.md`, `CHANGELOG_EN.md`, `CHANGELOG_MM.md` | Current release version is mentioned |
| `README.md`, `README_MM.md` | Current release line, release URL, and all three platform archive names are current |
| `SECURITY.md` | Supported release line and official release-integrity URL are current |
| `docs/RELEASE_<VERSION>_EN.md`, `docs/RELEASE_<VERSION>_MM.md` | Bilingual release notes exist for the version |
| Git tag `v<VERSION>` | Matches the authoritative package version when a tag is supplied |

## Validation and evidence

Run the gate locally with:

```bash
EXPECTED_VERSION=2.1.9 \
RELEASE_TAG=v2.1.9 \
ZAP_VERSION_REPORT=target/version-consistency.tsv \
scripts/validate_release_version.sh 2.1.9
scripts/test_validate_release_version.sh
```

The validator emits deterministic TSV evidence and fails closed on package/lockfile drift, CLI drift, stale onboarding links or archive names, stale security links, missing bilingual release notes, a hard-coded release template version, or a mismatched tag. Plain branch refs such as `master` are not treated as release tags; implicit tag validation activates only for semver-shaped `v<VERSION>` refs, while an explicitly supplied `RELEASE_TAG` is always enforced. The GitHub Actions quality job uploads the report and runs the positive and negative regression harness. `scripts/release_preflight.sh` runs the same validator before its other P0/P1 contract and deployment gates.

## Release workflow

Use `scripts/bump_release.sh` in dry-run mode first. Review its generated Cargo and changelog diff, update the versioned bilingual release notes, run the version gate and full release preflight, commit the result, and only then create and push the matching annotated tag. The tag-triggered workflow repeats the gate on the tagged source before building or publishing assets.

A documentation-only release mismatch is a release-blocking defect because it can direct a new user to an older binary. Do not bypass the version gate with `ALLOW_DIRTY`, an unrelated tag, or a manually edited artifact name.
