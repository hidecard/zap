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

## Current v2.1-B boundary

The current slice includes canonical origins, bounded trust policy, persistent policy configuration, credential persistence, environment-backed token selection, credential-aware remote index loading, secret redaction, CLI trust and credential commands, and stable 401/403 diagnostics. A local TLS server fixture for successful authenticated HTTPS fetch/publish coverage remains a release-review item because plaintext HTTP fixtures cannot legally carry credentials. OS keychains, certificate pinning, automatic redirect support, and signed-index policy enforcement remain later hardening work.
