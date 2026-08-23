# Security Policy

**Verified baseline:** Zap v2.2.6
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

Release artifacts are published through the repository's GitHub Actions workflow. Before publication, use the published v2.2.5 release as the latest official distribution; after publication, verify the [v2.2.6 release](https://github.com/hidecard/zap/releases/tag/v2.2.6) checksums and signatures before distributing it. The published v2.2.5 release and historical v2.2.0–v2.2.4 tags and assets remain immutable. v2.2.6 confines line-based file I/O to the active workspace, bounds synchronous sleep and exponentiation, validates strict locked builds, rejects malformed URL ports, skips symlink loops during test discovery, and uses best-effort platform process-tree termination on timeout/cancellation. These controls are not an OS sandbox: portable filesystem check/use races, complete DNS-to-connection pinning, and universal descendant cleanup remain host/deployment boundaries. The isolated v2.2.6 dependency-remediation branch updates the locked graph to `ureq 2.12.1`, `url 2.5.8`, `idna 1.1.0`, `rustls 0.23.40`, `rustls-webpki 0.103.15`, `rcgen 0.13.2`, and dev-only `time 0.3.47`; strict `cargo-audit 0.22.2` reports zero unresolved advisories across 87 locked crate dependencies. Because `time 0.3.47` requires Rust 1.88.0, the branch and CI quality job use Rust 1.88.0. This remediation is not a claim that v2.2.6 has been published: clean commit, GitHub CI, final release preflight, and published checksum/signature verification remain required.
