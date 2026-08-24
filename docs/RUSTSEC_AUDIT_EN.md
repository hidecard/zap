# Zap RustSec Dependency Audit Evidence

**Verified baseline:** Zap v2.9.1 development line

**Purpose:** This document records the dependency versions, local audit evidence, tool limitations, and CI/release controls for the native runtime. It is an evidence record for the v2.9.1 development line, not a claim that an untagged development commit is a published release.

## Current locked dependency graph

The native lockfile now uses the following security-relevant versions. The repository pins Rust 1.88.0, and the exact lockfile is audited in CI and the release preflight.

| Package | Locked version | Role | Audit status |
|---|---:|---|---|
| `ureq` | `2.12.1` | HTTP client | Updated from `2.9.7` |
| `rustls` | `0.23.40` | TLS implementation | Current locked line; ring-only features are used |
| `rustls-webpki` | `0.103.15` | Web PKI certificate validation | Current advisory-patched line |
| `url` | `2.5.8` | URL parsing | Updated from `2.4.1` |
| `idna` | `1.1.0` | Internationalized domain processing | Remediated line for RUSTSEC-2024-0421 |
| `idna_adapter` | `1.2.0` | IDNA backend | Locked dependency |
| `litemap` | `0.7.4` | IDNA backend support | Locked dependency |
| `zeroize` | `1.8.2` | Secret-memory cleanup support | Locked dependency |

The registry TLS tests no longer generate a certificate at test time through `rcgen`; they use a checked-in localhost end-entity DER certificate and key fixture. This removes the unused `rcgen`/`time` test dependency graph and avoids carrying the time crate's RFC 2822 advisory into the native lockfile.

## Local audit evidence

A local cargo-audit run may use the repository-compatible Rust toolchain. Its parser cannot load the complete current RustSec database because newer records contain CVSS 4.0 values, which this older binary reports as an unsupported CVSS version.

To preserve an auditable local result, a temporary copy of the official current advisory database was used with only records containing CVSS 4.0 removed. The command scanned `native/Cargo.lock`, loaded 1,166 parseable advisories, inspected 82 crate dependencies, returned no findings, and exited with status 0. A separate package-name comparison found no CVSS 4.0 advisory file matching a package in the current lockfile.

This workaround is explicitly incomplete with respect to the old parser's inability to read CVSS 4.0 records. The CI and release workflows use `cargo-audit 0.22.2` on the stable Rust toolchain and execute `cargo audit --file native/Cargo.lock --deny warnings`, so the live database, including CVSS 4.0 records, is checked by the repository gate.

## Advisory remediation mapping

The official RustSec advisory for `idna` recommends `idna 1.0.3` or later, or `url 2.5.4` or later when `idna` is reached through `url`.[^1] The lockfile uses `idna 1.1.0` and `url 2.5.8`.

The current `rustls-webpki` advisory records identify patched versions at `0.103.10`, `0.103.12`, or `0.103.13` and later, depending on the issue.[^2] The lockfile uses `rustls-webpki 0.103.15`. The older `0.102.8` package was therefore removed from the graph.

The time crate's current stack-exhaustion advisory applies to RFC 2822 parsing before `0.3.47`.[^3] The project graph is checked against the current lockfile and pinned Rust 1.88.0 toolchain.

## CI and release controls

The CI workflow has a dedicated RustSec job that installs `cargo-audit 0.22.2`, audits the exact native lockfile, and blocks the platform build matrix when the audit job fails. The tag-release quality job runs the same locked audit before release validation and packaging.

Dependency updates must continue to preserve the repository's pinned Rust compatibility policy. A future toolchain update should re-run the complete audit and re-evaluate all compatibility pins.

## Remaining limitations

A local audit performed with the filtered snapshot is not a substitute for the complete live-database scan. The authoritative complete scan is intentionally delegated to the CI and release jobs using the newer audit binary.

Filesystem confinement still has a portable check/use TOCTOU boundary because the runtime canonicalizes a path and subsequently opens or renames it. A complete fix requires descriptor-relative no-follow operations on Unix, reparse-point-aware handle logic on Windows, and dedicated race tests. This audit document does not claim that issue is fixed.

## References

[^1]: [RUSTSEC-2024-0421: idna accepts Punycode labels that do not produce any non-ASCII when decoded](https://rustsec.org/advisories/RUSTSEC-2024-0421.html)
[^2]: [RustSec advisories for rustls-webpki](https://rustsec.org/packages/rustls-webpki.html)
[^3]: [RUSTSEC-2026-0009: time denial of service via stack exhaustion](https://rustsec.org/advisories/RUSTSEC-2026-0009.html)
