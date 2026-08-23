# Production OAuth2/OIDC and JWT Authentication in Zap

Zap's production host adapter should act as an **OAuth2 resource server**. The identity provider performs the browser login and OAuth2 authorization-code flow; Zap receives an access token and validates it before the request reaches the gateway or repository. Do not use an ID token as an API access token. For browser clients, use Authorization Code with PKCE and exact redirect URI registration in the identity provider, following the current OAuth 2.0 security best practice [1].

## What the Framework branch implements

`host/zap-host/src/auth.rs` provides `JwtAuthenticator` and a bounded OIDC/JWT configuration contract. `AppState::from_env` selects it when `ZAP_AUTH_MODE=jwt`; local default mode remains the demo authenticator so the starter can run without an identity provider. Production deployment templates set JWT mode and reject the demo policy.

The authenticator performs the following checks before inserting `Identity` into the request extensions:

| Check | Production behavior |
|---|---|
| Authorization header | Requires one `Bearer <token>` value and enforces a bounded token size |
| Algorithm | Requires an explicit allowlist; the deployment template allows `RS256` only |
| Key ID | Requires `kid` and looks it up in the configured JWKS document |
| Signature | Verifies the JWS signature with the selected RSA or EC JWK |
| Claims | Validates `iss`, `aud`, `exp`, and `nbf`; requires a bounded non-empty `sub` |
| Scopes | Accepts OAuth `scope` and provider-style `scp`, normalizes and de-duplicates values |
| Provider failure | Maps JWKS network failure to `503`; malformed/expired/incorrect tokens map to `401` |
| Key rotation | Caches JWKS for a bounded TTL; an unknown `kid` causes one serialized refresh, with a short refresh cooldown and no stale-key fail-open |

The algorithm, issuer, audience, JWKS URL, clock skew, cache TTL, and token-size limits are validated at startup. The JWKS URL must be HTTPS. Redirects are disabled on the JWKS HTTP client, and the client has bounded connection/request timeouts.

## Required production environment

Copy `deploy/zap-web.env.example` through a secret/configuration management process. Replace placeholders; do not commit them.

```dotenv
ZAP_AUTH_MODE=jwt
ZAP_AUTH_ISSUER=https://login.example.com/
ZAP_AUTH_AUDIENCE=https://api.example.com
ZAP_AUTH_JWKS_URL=https://login.example.com/.well-known/jwks.json
ZAP_AUTH_ALLOWED_ALGORITHMS=RS256
ZAP_AUTH_CLOCK_SKEW_SECONDS=30
ZAP_AUTH_JWKS_CACHE_SECONDS=300
ZAP_AUTH_MAX_TOKEN_BYTES=16384
```

The issuer and audience values must exactly match the access-token contract issued by the selected provider. The API should receive access tokens whose `aud` is the API, not a frontend client identifier. Keep the JWKS endpoint stable and publish the provider's new signing key before issuing tokens with its new `kid`.

## Authorization flow boundary

A browser or mobile client should use the provider's Authorization Code + PKCE flow. The client exchanges the code at the provider, obtains an access token for the Zap API, and sends it in the HTTPS `Authorization: Bearer` header. Zap validates the access token but does not implement a login page, password grant, token endpoint, refresh-token store, or browser session cookie.

After authentication, authorization remains a separate decision. Handlers should require scopes such as `users:read` or `users:write`, and repositories must enforce subject/tenant ownership in their queries. A valid signature alone does not grant access to every resource.

## HTTP error contract

| Situation | Response |
|---|---|
| Missing, malformed, expired, wrong issuer/audience, wrong signature, unsupported algorithm, or unknown `kid` | `401 unauthenticated` |
| Valid identity without the required route scope | `403 forbidden` |
| JWKS provider timeout or temporary fetch failure | `503 authentication_unavailable` |
| Invalid deployment configuration or unusable JWKS document | `500 authentication_unavailable` and a deployment alert |

Do not return provider-specific parsing details to clients. Do not log the raw `Authorization` header, access token, claims containing personal data, or the JWKS URL if it contains credentials. The existing sensitive-header middleware continues to redact authorization headers from trace output.

## Key rotation runbook

Publish the new public JWK with a new `kid` while retaining the previous public JWK. Start issuing new tokens with the new key only after the JWKS endpoint is serving both keys. Keep both keys available for at least the maximum access-token lifetime plus the configured cache TTL and clock skew. Remove the old key only after old tokens can no longer be valid. If an unknown `kid` appears, Zap performs one bounded refresh; repeated refresh attempts are throttled and do not accept stale keys after the cache expires.

Test rotation in staging by issuing an old-key token, adding the new JWK, issuing a new-key token, verifying both during overlap, then removing the old key after expiry. The test must also verify that an algorithm-confusion token, a token signed by the wrong issuer, and an ID token sent to the API are rejected.

## Deployment checklist

Run `scripts/validate_zap_host_deployment.sh` and `scripts/validate_zap_web_deployment.sh`. Confirm that `ZAP_AUTH_MODE=jwt` is present in the managed environment, the JWKS URL is HTTPS, the deployment policy allows only the reviewed algorithm list, and demo authentication is disabled. Verify `/health` independently from `/ready`; readiness should include the real repository and identity-provider dependency policy without making liveness dependent on a transient JWKS request.

## References

[1]: https://www.rfc-editor.org/rfc/rfc9700 RFC 9700 — Best Current Practice for OAuth 2.0 Security.
[2]: https://www.rfc-editor.org/rfc/rfc8725 RFC 8725 — JSON Web Token Best Current Practices.
[3]: https://openid.net/specs/openid-connect-core-1_0.html OpenID Connect Core 1.0.
