# Security Policy

**Verified baseline:** Zap v2.2.4
**Purpose:** Security-maintainer and release-operator reference for supported versions, reporting, provenance, and release-integrity controls.
**Navigation:** [Documentation hub](docs/DOCUMENTATION_NAVIGATION_EN.md) · [Deployment boundaries](docs/DEPLOYMENT_EN.md) · [Release signing](docs/RELEASE_SIGNING_EN.md) · [Release version policy](docs/RELEASE_VERSION_POLICY_EN.md)

## Supported Versions

Security fixes are applied to the latest version published in the [GitHub Releases](https://github.com/hidecard/zap/releases) page. Older release lines may not receive security updates.

| Version | Supported |
|---|---|
| Latest `v2.2.x` | Yes |
| Older versions | Best effort only |

## Reporting a Vulnerability

Please do not open a public issue for an undisclosed security vulnerability. Report it privately through the repository's [GitHub Security Advisories](https://github.com/hidecard/zap/security/advisories/new) page. If that page is unavailable, contact the repository maintainers through the contact information listed in the GitHub repository profile.

A useful report includes the affected Zap version or commit, operating system, a minimal `.zp` example, the expected and actual behavior, and any relevant stack trace or diagnostic output. Please remove secrets, credentials, personal information, and proprietary source code before submitting a report.

Maintainers will acknowledge a valid report as soon as practical, investigate its impact, and coordinate a fix and disclosure timeline with the reporter. Please allow reasonable time for investigation before publicly discussing an unpatched vulnerability.

## Scope

Security reports may include parser crashes, panics on malformed input, sandbox or file-boundary bypasses, path traversal, unsafe module loading, denial-of-service resource-limit bypasses, diagnostic secret leakage, and release artifact integrity problems.

Zap is experimental software. Do not execute untrusted Zap programs with access to sensitive files or credentials unless the operating environment provides an appropriate sandbox.

## Release Integrity

Release artifacts are published through the repository's GitHub Actions workflow. Verify the published checksums and use the official [v2.2.4 release](https://github.com/hidecard/zap/releases/tag/v2.2.4) or a later official release when distributing Zap. The historical v2.2.0 tag and assets remain immutable.
