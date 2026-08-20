# Zap Security Audit and Remediation Status

**Audit source:** `zap-security-audit-mm.pdf`  
**Audited baseline:** v2.1 development line after the v2.1 lockfile/checksum work  
**Assessment type:** Defensive source review, local build/test validation, and regression verification

## Executive Summary

The attached audit identified five findings in the public runtime surface. Three findings affected untrusted execution directly: arbitrary process execution, unrestricted filesystem/environment access, and SSRF/local-network access. Two additional findings concerned registry local-source trust and the absence of a true process deadline.

The runtime now provides an explicit restricted mode through `ZAP_UNTRUSTED=1`. In that mode, filesystem access, environment access, process execution, outbound network access, and local registry sources are denied by default. HTTP requests additionally reject loopback/private/link-local destinations, disable automatic redirects, enforce a request-body limit, and retain response-size and timeout limits. Process execution uses a real deadline with child termination rather than checking elapsed time only after completion.

> Restricted mode is a runtime capability boundary, not a complete operating-system sandbox. Production multi-tenant deployments must still run Zap inside an OS-level sandbox, container, VM, or equivalent isolated worker with least-privilege credentials and network egress controls.

## Finding Status

| ID | Finding | Original risk | Remediation status | Current control |
|---|---|---:|---|---|
| F-01 | Arbitrary process execution | High | Mitigated in restricted mode | `process_run` requires process capability; hard deadline and bounded stdout/stderr are enforced |
| F-02 | Unrestricted filesystem and environment access | High | Mitigated in restricted mode; trusted mode remains intentionally powerful | File and environment builtins require their capabilities; restricted mode denies them by default |
| F-03 | SSRF and local-network probing | High | Mitigated in restricted mode; OS egress control still required | Loopback, private, link-local, unspecified, broadcast, ULA, and IPv6 link-local destinations are rejected; redirects are disabled |
| F-04 | Registry local-source trust boundary | Medium | Mitigated in restricted mode | `file://` and bare local registry sources are denied when `ZAP_UNTRUSTED=1`; trusted local development remains supported |
| F-05 | Post-hoc process timeout | Medium | Fixed | Child process is spawned and polled until the deadline; overdue children are killed and reported deterministically |

## Implemented Controls

### Restricted capability mode

Set `ZAP_UNTRUSTED=1` in the host environment before executing untrusted Zap source. The following capabilities are denied by default:

- filesystem reads and writes;
- environment and configuration reads;
- external process execution;
- outbound HTTP/HTTPS requests and local HTTP serving; and
- local registry index or package sources.

Trusted local developer behavior remains backward-compatible when the variable is absent. Hosts that need a stronger policy should wrap the runtime and provide an explicit allowlist or isolated worker rather than exposing trusted mode to downloaded code.

### Network policy

HTTP requests continue to accept only `http` and `https` schemes. In restricted mode, DNS resolution is performed before the request and every resolved address is checked. Loopback, RFC1918/private IPv4, link-local, unspecified, broadcast, IPv6 unique-local, and IPv6 link-local destinations are rejected. Automatic redirects are disabled to prevent a permitted public URL from redirecting into a blocked destination. Request bodies are limited to 64 KiB, responses to 8 MiB, and connect/read/write operations to bounded timeouts.

### Process policy

`process_run` remains a non-shell API, so command and argument values are not concatenated into a shell command. It now requires process capability, captures output with fixed limits, polls the child until the ten-second deadline, kills overdue children, and returns a stable timeout error. OS-level CPU, memory, process-count, and process-group isolation remain deployment responsibilities.

### Registry source policy

Registry package identity and SHA-256 artifact validation remain enforced. In restricted mode, `file://` and bare local paths are rejected as registry sources. Remote transport continues to follow the configured secure-transport policy. Trusted local development fixtures remain available outside restricted mode.

## Verification

The security remediation added regression coverage for capability denial, private-network rejection, and oversized HTTP request bodies. The native suite, formatting, compilation, P3 smoke fixture, and whitespace validation all pass.

| Verification | Result |
|---|---|
| Rust formatting check | Passed |
| Native compilation | Passed |
| Native tests | 236 passed, 0 failed |
| P3.3 smoke fixture | `P3.3 smoke OK` |
| Diff whitespace check | Passed |
| Cross-platform release baseline | v2.0.3 Linux, Windows, and macOS matrix passed |

## Remaining Deployment Requirements

The audit's P0 recommendation for an OS-level sandbox is not something the language runtime can guarantee by itself. Before using Zap for multi-tenant services, downloaded plugins, or untrusted CI submissions, the deployment must add an isolated worker boundary, least-privilege filesystem permissions, restricted environment injection, network egress filtering, resource quotas, process-group cleanup, and audit logging.

The next v2.1 security tasks are to add workspace-confined filesystem mode with symlink-safe canonicalization, explicit trusted-registry policy, process-group/resource controls where supported by each target OS, dependency auditing in CI, and expanded integration fixtures for SSRF, path escape, environment filtering, timeout, body-size, and registry-source cases.

## Responsible Use

`ZAP_UNTRUSTED=1` should be treated as a defensive runtime mode for capability denial and request policy. It should not be described as a substitute for a kernel-enforced sandbox. Security-sensitive hosts must combine the mode with operating-system isolation and should treat all downloaded Zap source, package indexes, and package archives as untrusted input until verified.
