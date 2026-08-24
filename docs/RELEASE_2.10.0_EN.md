# Zap v2.10.0 Release Notes

**Release line:** v2.10.0
**Verified baseline:** Zap v2.9.2 on the latest master before the v2.10.0 tag
**Status:** Published Web validation and Result middleware increment

## Summary

Zap v2.10.0 adds a bounded native Web request-validation contract and a centralized Result-aware response boundary. Web handlers can validate an already parsed map or raw JSON text with `web_validate_request(body, schema)` and return typed `ResultOk` or `ResultErr` values without inventing a second error protocol.

The native Web server now maps safe handler `ResultErr` values centrally to JSON HTTP errors while preserving existing direct response maps. The feature is deliberately bounded: validator-generated malformed-input and field-validation failures use HTTP `400`, while a handler may choose `422` for a semantically invalid payload.

## Implemented changes

| Area | Change | Evidence |
|---|---|---|
| Typed validation | Validate up to 64 schema fields and a 64 KiB raw JSON/map body using bounded `text`, `number`, `bool`, `map`, `list`, and `none` types. | Native unit regressions |
| Validation results | Return `ResultOk` for the validated map and bounded `ResultErr` maps for invalid JSON, body shape, schema, unknown/missing fields, type mismatches, and text-length violations. | Native unit regressions and scaffold test |
| Central middleware | Convert safe `ResultErr` maps with status `400..599`, bounded token codes, and bounded messages into JSON containing `error`, `message`, and `request_id`. | Native Web loopback test |
| Compatibility | Keep direct response maps working; malformed Result shapes and handler raises fail closed as `500 handler_error`. | Native Web loopback test |
| `zap new` scaffold | Demonstrate validation, `is_err`, `unwrap`, and `ok` in the generated `create_user` function and test it with valid and invalid JSON. | Generated-project smoke test |
| Public contract | Register the `web` standard-library domain and `web_validate_request` in the catalog, policy tables, bilingual guides, and VS Code grammars. | Catalog, policy, documentation, asset, and link gates |

## Usage

```zap
export fn create_user(request):
    let schema = {"name": {"type": "text", "max_len": 120}, "email": {"type": "text", "max_len": 254}}
    let checked = web_validate_request(request["body"], schema)
    if is_err(checked):
        return checked
    let payload = unwrap(checked)
    return ok({"status": 201, "body": json({"created": true, "body": payload})})
```

A successful validation returns `ResultOk(map)`. Expected request or schema failures return `ResultErr({"status": 400, "code": "...", "message": "...", "field": "..."})`; the `field` entry is available to Zap code, while the native HTTP boundary emits the bounded public fields `error`, `message`, and `request_id`. Handler code can return another safe error status, such as `422`, when the payload is syntactically valid but semantically unacceptable.

## Compatibility and boundaries

This is an additive native Web capability for the existing user-managed project structure. It does not introduce hidden app registration or a Django-style `startapp` command. The `models/`, `functions/`, `ui/`, `routes/`, `middleware/`, `migrations/`, `admin/`, `public/`, and `tests/` directories remain editable Zap modules, and browser build output remains deployable without Node.js as a runtime prerequisite.

The validator is not a complete schema compiler: it intentionally supports a small bounded type set and does not perform coercion, nested schema compilation, database-backed uniqueness checks, authentication, or business-rule validation. The Result adapter is not a complete middleware graph and does not claim production TLS, graceful shutdown, backpressure, observability, async I/O, provider-neutral ORM/database support, cross-file refactoring, SSR/template compilation, WebSocket/streaming uploads, built-in admin UI, or real mobile/AI/IoT provider adapters. Those remain separate milestones requiring implementation and evidence.

## Verification

The release candidate passed native formatting, the full native test suite, release compilation, framework starter validation, standard-library policy checks, documentation consistency, Markdown link validation, VS Code asset validation, and the full clean-tree release preflight. The generated project smoke path passed `zap new`, `zap check`, `zap web check`, `zap web routes`, database migration commands, and `zap test`. The tagged workflow must additionally verify the platform archives, checksums, signatures, provenance, manifests, installers, and published assets.

## References

[1]: ../docs/ZAP_WEB_NATIVE_EN.md
[2]: ../docs/WEB_FRAMEWORK_EN.md
[3]: ../docs/LEARN_ZAP_EN.md
[4]: ../docs/STDLIB_POLICY_EN.md
