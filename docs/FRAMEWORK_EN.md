# Zap Framework Guide

**Verified baseline:** Zap v2.2.3
**Framework branch:** `Framework`
**Status:** Framework Foundation v0.1 — runnable contract starters, not production adapters

## Purpose

The `frameworks/` directory provides small, executable Zap programs that define domain contracts for Web, App, IoT, and AI integrations. These starters are intentionally written with the current stable Zap syntax. They demonstrate request/response models, application state, telemetry records, and bounded validation without pretending that the current interpreter is already an HTTP server, a native mobile UI runtime, an MCU firmware runtime, or an AI provider client.

The Framework layer is therefore a **contract-first adapter boundary**. Zap owns domain logic, validation, deterministic transformations, and application policy. A host adapter owns sockets, native windows, mobile lifecycle, device drivers, MQTT sessions, process supervision, credentials, and platform-specific scheduling.

> A framework starter is complete when it runs with the published Zap runtime, has a valid manifest and lockfile, documents its host boundary, and has negative cases for invalid input. It is not complete merely because it contains an aspirational DSL.

## Current starter matrix

| Starter | Current deliverable | Host integration still required | Production status |
|---|---|---|---|
| `frameworks/web` | Deterministic route/request/response contract | HTTP listener, TLS, middleware, body limits, deployment supervision | Contract prototype |
| `frameworks/mobile` | Portable app model, screens, and actions | Tauri, Flutter, React Native/Expo, or another native shell | Contract prototype |
| `frameworks/iot` | Bounded sensor-event and device-state contract | MQTT/Paho, gateway transport, ESP-IDF, Zephyr, or Embassy host | Contract prototype |
| `frameworks/ai` | Prompt/response boundary example | Provider SDK, local model, credential and quota adapter | Contract prototype |

The starters intentionally do not add new parser syntax. They use functions, maps, lists, loops, `assert`, and `json()` so that a developer can run them today with `zap main.zp`.

## Quick start

From the repository root, run the following commands for any starter:

```bash
cd frameworks/web
zap lock
zap check
zap run main.zp
```

The same sequence works for `frameworks/mobile`, `frameworks/iot`, and `frameworks/ai`. `zap.lock` is generated output and should be committed with each starter. If a future adapter dependency is added, regenerate the lockfile and run `zap install --locked` before testing.

The starter programs produce deterministic JSON or text output and exit without opening a network socket, native window, device connection, or external model session. This makes them suitable for CI smoke tests and learning examples.

## Package boundaries

The Framework layer follows a one-way dependency direction:

```text
Zap source and domain contract
          ↓
framework starter package
          ↓
zap-host capability and DTO boundary
          ↓
platform adapter
          ↓
OS, network, native UI, device SDK, or provider
```

A starter must not import an undeclared provider package, silently read credentials, open a socket, spawn an unrestricted process, or assume that a host has a particular operating system. Platform behavior belongs in a separately versioned adapter package.

| Layer | Owns | Must not own |
|---|---|---|
| Zap core | Parsing, evaluation, diagnostics, deterministic values | HTTP, mobile rendering, MCU drivers |
| Framework contract | Domain records, validation, routing/state/telemetry policy | OS handles, credentials, native threads |
| `zap-host` boundary | Capability names, typed DTOs, limits, errors, tracing | Hidden global state or unrestricted ambient authority |
| Platform adapter | HTTP/TLS, native UI lifecycle, MQTT, board SDK, provider API | New language semantics without an RFC |
| Deployment | Identity, sandbox, egress, quotas, supervision, secrets | Assumptions that runtime limits are OS isolation |

## Web starter

`frameworks/web/web_contract.zp` is the reusable Web contract module. It exports `normalize_request`, `security_headers`, `response`, and `route`. The request contract normalizes `GET`/`POST`, rejects traversal-shaped paths, bounds paths to 2,048 bytes, bounds bodies to 65,536 bytes, and requires a request ID of 1–128 bytes. The response contract contains `status`, `content_type`, `headers`, and `body`.

`frameworks/web/main.zp` demonstrates root, health, echo, not-found, traversal-rejection, and unsupported-method cases. The Web API layer now also includes reusable `api_contract.zp`, `dto_contract.zp`, `database_contract.zp`, `auth_contract.zp`, and `rate_limit_contract.zp` modules, with `api_contract_test.zp` covering 200/201/400/401/403/404/429 behavior, mapping, quota transitions, and policy failures. The detailed schema, threat controls, database boundary, authentication policy, rate-limit semantics, adapter pipeline, and Web-specific definition of done are documented in [`WEB_FRAMEWORK_EN.md`](WEB_FRAMEWORK_EN.md).

A future `zap-web` adapter may translate an incoming HTTP request into the bounded Zap request map and translate the returned response map into an HTTP response. That adapter must define method/path normalization, maximum headers and body bytes, timeout, cancellation, error mapping, logging redaction, and connection shutdown. The starter itself performs none of those network operations.

The recommended first implementation is a host adapter over an established Rust HTTP stack such as [Axum](https://docs.rs/axum/latest/axum/) and Tower middleware rather than a second HTTP runtime inside the Zap interpreter. The adapter should pass a fake-host contract test before a real listener is exposed.

## App starter

`frameworks/mobile/main.zp` models an application manifest containing a name, initial route, screens, and actions. It demonstrates that navigation and action policy can be expressed as ordinary data before selecting a native shell.

The first App implementation should generate or consume a shell rather than implement a custom renderer. Suitable host options include [Tauri](https://v2.tauri.app/) for a Rust/native-web shell, [Flutter](https://docs.flutter.dev/) for a widget-based multiplatform UI, or [React Native with Expo](https://reactnative.dev/docs/environment-setup) when the project needs the established JavaScript/native ecosystem. The Zap contract should remain independent of the chosen renderer.

App adapters must define lifecycle events, foreground/background behavior, offline storage, IPC authentication, permission prompts, deep links, update/rollback behavior, and crash reporting. A screen map alone is not a mobile runtime.

## IoT starter

`frameworks/iot/main.zp` models a device identity, bounded sensor samples, an accepted-reading count, and a device state record. It deliberately simulates readings instead of touching GPIO, serial, Bluetooth, Wi-Fi, or a real broker.

The first IoT implementation should target a gateway or Linux/SBC process. It can use MQTT through an established client such as [Eclipse Paho](https://eclipse.dev/paho/) and should make topic policy, payload size, QoS, retained messages, reconnect behavior, duplicate handling, and offline replay explicit. For firmware, Zap should initially generate bindings or communicate with an existing [ESP-IDF](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/) or [Zephyr](https://docs.zephyrproject.org/latest/) component. A full Zap interpreter on a small MCU remains out of scope because interrupts, DMA, `no_std`, flash/RAM budgets, watchdogs, and board support require a different execution boundary. [Embassy](https://embassy.dev/book/) is a useful reference for those embedded constraints.

IoT adapters must reject malformed or oversized telemetry, authenticate device identity, make commands idempotent, record a correlation identifier, support safe reconnect, and define what happens after reset or brownout. A successful simulated loop is not hardware-in-the-loop evidence.

## AI starter

The AI starter is a contract example only. It models prompt and response records without contacting a provider or storing credentials. A future provider adapter must define model selection, timeout, request/response size limits, retry policy, quota errors, prompt/response redaction, content handling, and audit retention.

## Capability and security contract

Every real adapter should receive an explicit host capability object rather than ambient access. At minimum, the boundary should carry:

| Field | Requirement |
|---|---|
| `capability` | Stable name such as `web.listen`, `iot.publish`, or `app.storage.read` |
| `identity` | Authenticated caller/device/app identity; never inferred only from a string field |
| `limits` | Input, output, task, timeout, queue, and payload bounds |
| `deadline` | Monotonic deadline or documented poll budget |
| `cancellation` | Cooperative cancellation and resource-close behavior |
| `idempotency_key` | Required for commands that may be retried |
| `trace_id` | Correlates domain result, host operation, and audit record |
| `redaction` | Secret/token/password fields excluded from diagnostics |
| `error` | Stable typed category rather than an unstructured provider string |
| `replay_class` | Whether the operation is pure, input-deterministic, runtime-dependent, or external I/O |

The host must default to deny. A denied capability must produce a deterministic typed error before an external side effect. Runtime logical budgets are useful but are not a replacement for an OS sandbox, network egress policy, process identity, or secret manager.

## Testing and acceptance

Every Framework starter should have four kinds of evidence:

1. **Executable smoke:** `zap check`, `zap build`, and `zap run main.zp` succeed from a clean directory with a committed lockfile.
2. **Contract assertions:** Valid output shape, deterministic ordering, and representative error/edge behavior are asserted in Zap source or host tests.
3. **Negative security cases:** Oversized input, unsupported capability, malformed route/topic/action, and missing identity are rejected before side effects.
4. **Adapter parity:** The fake host and real host produce the same normalized domain result for the same fixture, while external errors remain typed and traceable.

The CI gate should fail if a starter contains undeclared dependencies, a missing lockfile, unresolved placeholder imports, unsupported aspirational syntax, or a documentation claim that calls a contract prototype a production runtime.

## What is deliberately not implemented in v0.1

The Framework branch does not yet provide a native HTTP server, custom mobile renderer, MCU interpreter, MQTT client, OTA manager, cloud deployment command, ORM, or provider-specific AI client. Adding those directly to the language core would duplicate mature ecosystems and expand the security surface before the host contract is stable.

The initial `zap-host` adapter prototype is now available under `host/zap-host`. It provides an Axum/Tower HTTP boundary, capability-facing traits, typed DTO mapping, bounded request/response handling, deterministic in-process tests, structured errors, redaction of sensitive headers, and graceful shutdown. The detailed setup and production checklist are documented in [`ZAP_HOST_EN.md`](ZAP_HOST_EN.md). Real Zap runtime embedding, database/authentication providers, shared quota storage, TLS, and deployment-specific evidence remain separate follow-up work.

## Definition of done for Framework Foundation v0.1

The Framework Foundation is complete when all four starters have valid manifests and lockfiles, use only current Zap syntax, pass clean smoke validation, document their non-production boundary, expose deterministic domain records, contain no secret or unrestricted host access, and are linked from the bilingual documentation navigation. Real platform adapters are separate milestones and must not be implied by the starter directory alone.

## References

[1]: https://docs.rs/axum/latest/axum/ — Axum HTTP routing and request-handling documentation
[2]: https://v2.tauri.app/ — Tauri desktop and mobile application shell documentation
[3]: https://docs.flutter.dev/ — Flutter multiplatform UI toolkit documentation
[4]: https://reactnative.dev/docs/environment-setup — React Native and Expo environment guidance
[5]: https://eclipse.dev/paho/ — Eclipse Paho MQTT client project
[6]: https://docs.zephyrproject.org/latest/ — Zephyr RTOS and embedded platform documentation
[7]: https://docs.espressif.com/projects/esp-idf/en/latest/esp32/ — Espressif ESP-IDF documentation
[8]: https://embassy.dev/book/ — Embassy embedded async framework documentation
[9]: https://github.com/hidecard/zap/blob/master/docs/ASYNC_BOUNDARIES_EN.md — Zap v2.2.3 async boundary contract
[10]: https://github.com/hidecard/zap/blob/master/SECURITY.md — Zap security policy and untrusted-execution boundary
