# Zap v2.1-B Registry Authentication and Trusted Registries

## Purpose

Zap separates **transport security**, **registry trust**, **authentication**, and **artifact integrity**. HTTPS protects the request transport, the trusted-registry policy decides which remote origins may be used, credentials identify the caller, and lockfile checksums verify downloaded artifacts. None of these controls replaces another.

## Trusted registry policy

Remote registry origins must be canonicalized before comparison. Zap lowercases host names, removes default ports, normalizes path prefixes, rejects userinfo, queries, fragments, backslashes, whitespace, and traversal segments, and stores entries in deterministic order. Local paths and `file://` sources retain their explicit local-source behavior.

Use the following commands to manage trusted origins:

```text
zap registry trust list
zap registry trust add https://registry.example/team
zap registry trust remove https://registry.example/team
```

An untrusted remote is rejected before a network request is made. `http://` remains disabled by default and is intended only for controlled fixtures when `ZAP_ALLOW_INSECURE_HTTP=1` is explicitly set.

## Credential configuration

Credentials are scoped to canonical registry origins and path prefixes. A more-specific path credential takes precedence over a broader origin credential. Tokens are accepted only for HTTPS or local file origins, are bounded to 4096 bytes, and must not contain whitespace or control characters.

To avoid exposing secrets in shell history, configure a token through an environment-variable reference:

```text
export ZAP_REGISTRY_TOKEN_CI='replace-with-a-secret'
zap registry credential set https://registry.example/team --token-env ZAP_REGISTRY_TOKEN_CI
zap registry credential list
zap registry credential remove https://registry.example/team
```

The list command prints origins only; it never prints token values. The persistent configuration path is selected by `ZAP_REGISTRY_CONFIG`, then `$HOME/.config/zap/registry.json`, and finally `.zap/registry.json`. The file is bounded to 64 KiB and updates use a temporary file followed by an atomic replacement.

Credential resolution follows this order: an explicit API token, an origin-scoped configured credential, and finally `ZAP_REGISTRY_TOKEN`. Credentials are never written to manifests, lockfiles, logs, diagnostics, or changelogs.

## Stable authentication diagnostics

HTTP authentication responses use stable codes while preserving the existing string-based API:

| Code | Meaning |
|---|---|
| `ZAP-REG-AUTH-001` | Credentials are missing for a `401` response. |
| `ZAP-REG-AUTH-002` | Supplied credentials were rejected for a `401` response. |
| `ZAP-REG-AUTH-003` | The credential lacks permission for a `403` response. |

Diagnostics contain the canonical origin but never contain the bearer token. Non-authentication service responses retain their existing HTTP status diagnostics.

## Operation order

Install, update, cache, and publish operations must follow this order: normalize the source, enforce trusted-origin policy, resolve an origin-scoped credential, enforce secure transport, perform the request, validate the response, and verify checksum or signature. When `ZAP_REGISTRY_INDEX` names a remote index URL, dependency resolution loads the same persisted and environment-backed credential store for the index request before resolving packages. Offline operations do not perform authentication or network access.

## Provenance policy

A protected release must use signed provenance and must fail closed when provenance identity is incomplete. Signed mode requires a semantic-version tag ref (`refs/tags/vX.Y.Z`), a full 40-hex commit SHA, a numeric CI workflow run ID, and an HTTPS source URI. The signing key must resolve to a full 40-hex OpenPGP fingerprint; that fingerprint, rather than an ambiguous short key ID, is recorded in `signing.key_id`. Signed mode also requires the `TRUSTED_SIGNING_FINGERPRINTS` allowlist. The active fingerprint must match one of its full 40-hex entries; an empty, malformed, or non-matching allowlist fails closed. During a planned key rotation, the old and new full fingerprints may coexist in this allowlist for a bounded transition window. The old fingerprint must be removed before the window closes, and a key outside the allowlist must never sign a protected release.

The release signer validates every manifest subject against its per-artifact checksum before signing the archives, manifest, and aggregate checksum index. It then signs the generated provenance document and verifies every detached signature before returning success. Development-only unsigned mode remains explicitly labeled in provenance and must never be used by the protected release workflow. Missing identity fields, an invalid tag/ref, a malformed commit, an insecure source URI, an unavailable key, a checksum mismatch, or a signature verification failure aborts the operation without publishing artifacts. The published-release verifier also signs each adversarial fixture before checking it, and rejects mutated commit, subject, source, ref, workflow, and signing-mode metadata rather than treating a valid detached signature as sufficient.

## Yanked-release policy

A registry package record may set `yanked: true` for a version that must not be selected for a new dependency resolution. The field defaults to `false` for legacy index records, but an explicitly malformed yanked value is rejected rather than silently treated as safe. Exact-version and range resolution both skip yanked candidates; an exact yanked request returns `registry package is yanked: <name> <version>`, while a range whose matching candidates are all yanked returns `all matching registry packages are yanked: <name> <requirement>`. These stable diagnostics fail closed instead of selecting an unsafe release.

An existing lockfile may continue to use a yanked version only for an explicitly locked, checksum-verified offline or update operation. The resolver must not introduce a yanked version into a new lockfile, and an update operation must not silently replace a healthy locked version with a yanked one. Cache presence does not override the yanked flag: cached artifacts remain usable only when the lockfile explicitly names the version and its checksum matches. The compatibility test verifies that checksum validation remains usable for such an explicitly locked yanked artifact without allowing resolver selection. Publishing or mutating registry metadata must preserve deterministic ordering and must not clear a yanked marker without an authenticated, signed metadata update.

## End-to-end lockfile/cache compatibility audit

The end-to-end compatibility contract now validates the complete locked-cache path rather than isolated helpers. A clean-machine fixture builds the native runtime with `--locked`, runs the lockfile and cache verification tests, and verifies that a cached artifact can be reused only when its explicit lockfile version satisfies the manifest requirement and its bytes match the recorded SHA-256 checksum. The fixture also exercises an explicitly locked yanked artifact: the artifact remains usable from the verified cache for the locked operation, while new exact and range resolution continues to reject yanked candidates.

The audit rejects tampered cache bytes, mismatched lockfile checksums, malformed lockfile records, and cached versions that do not satisfy the manifest requirement. It also verifies the offline boundary: the clean copy completes without registry network access, and the locked path does not allow cache presence to weaken dependency, checksum, or yanked-release policy. The reproducible command is `scripts/verify_clean_machine_locked.sh`; its final gate includes `lockfile_security_tests` and the existing checksum-cache regression test.

## Current v2.1-B boundary

The current slice includes canonical origins, bounded trust policy, persistent policy configuration, credential persistence, environment-backed token selection, credential-aware remote index loading, successful authenticated HTTPS fetch/publish coverage through a test-only rustls fixture, secret redaction, CLI trust and credential commands, stable 401/403 diagnostics, fail-closed signed-index verification tests that reject wrong keys and mutated payloads without panicking, and protected-release provenance checks for tag, commit, workflow, source, checksum, full signing-fingerprint identity, and an explicit full-fingerprint rotation allowlist. The fixture uses a generated localhost certificate trusted only by the injected test agent; production requests retain normal certificate verification. OS keychains, certificate pinning, automatic redirect support, and production signed-index key-management policy remain later hardening work.
