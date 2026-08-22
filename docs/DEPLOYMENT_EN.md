# Zap Registry Production Deployment Boundaries

**Verified baseline:** Zap v2.2.0
**Purpose:** Operator reference for local and public registry deployment boundaries, validation, TLS, supervision, credentials, quotas, and egress controls.
**Navigation:** [Documentation hub](DOCUMENTATION_NAVIGATION_EN.md) · [Package author guide](PACKAGE_EN.md) · [Stdlib reference](STDLIB_INDEX_EN.md) · [Language specification](LANGUAGE_SPEC_EN.md) · [Security policy](../SECURITY.md) · [Release policy](RELEASE_VERSION_POLICY_EN.md)

## Scope

Zap includes a controlled local registry service, but a public production deployment must add an explicit operating boundary around that service. This guide defines the repository's reproducible reference policy. It is an operator contract and validation target; it does not provision certificates, create cloud resources, or publish a registry automatically.

## Reference artifacts

| Artifact | Purpose |
|---|---|
| `deploy/zap-registry.service` | Linux service supervision, least privilege, filesystem protection, quotas, and loopback-only network access |
| `deploy/zap-registry.nginx.conf` | TLS termination, HTTP-to-HTTPS redirect, request limits, allowed methods, and loopback upstream policy |
| `deploy/registry.env.example` | Redacted environment template for deployment-secret-manager integration |
| `deploy/registry-deployment-policy.toml` | Machine-readable deployment contract for bind address, limits, sandboxing, credentials, and egress |
| `scripts/validate_registry_deployment.sh` | Dependency-free CI/operator validation of the reference controls and secret-file hygiene |

## TLS and ingress

The registry process binds to `127.0.0.1:8787` and is not intended to receive public traffic directly. The reference nginx configuration terminates TLS with TLS 1.2 or TLS 1.3, redirects cleartext HTTP to HTTPS, limits request bodies to 1 MiB, permits only `GET` and `POST`, and forwards requests to the loopback service with bounded proxy timeouts. Operators must replace the example hostname and certificate paths with certificates managed by their platform. Private keys must remain outside the repository.

The backend receives only loopback traffic. The ingress layer is responsible for certificate renewal, public DNS, external rate limiting, and any organization-specific WAF policy. Those controls are deliberately deployment-provider responsibilities rather than runtime assumptions.

## Supervision and sandbox

The systemd unit runs the service as a dynamic `zap-registry` user with `NoNewPrivileges`, private temporary devices, protected system and home paths, a restrictive umask, and one explicit writable directory: `/var/lib/zap-registry`. It restarts failed services, stops the complete process group on shutdown, and uses a bounded stop timeout. `IPAddressDeny=any` with loopback allow rules prevents the backend from making external network connections; the TLS proxy remains the only public-facing component.

The unit is a Linux reference. Windows and macOS deployments must provide equivalent controls through their native service manager, sandbox, firewall, and secret-management facilities. CI validates the portable policy contract and the Linux artifact text; it does not claim that all operating systems implement identical kernel isolation.

## Resource quotas

The reference policy limits memory to 256 MiB, CPU to 50 percent, tasks to 64, and open files to 1,024. The registry protocol also retains its bounded request, body, response, and timeout limits. Operators may lower these values after measuring workload requirements, but must not remove the limits without recording an explicit risk decision.

## Credentials

`ZAP_REGISTRY_TOKEN` and `ZAP_REGISTRY_SIGNING_SECRET` are required for the authenticated service. They must be injected by a deployment secret manager or an equivalent protected facility, stored with mode `0600` when file-backed, and excluded from logs, archives, process arguments, and source control. `deploy/registry.env.example` contains placeholders only. The validator rejects populated `registry.env`, private-key, and certificate files under `deploy/`.

## Egress controls

The registry backend is loopback-only. External egress is disabled in the reference policy, and the service may write only to its registry data directory. If an installation needs outbound package retrieval, that operation must be performed by a separate, explicitly allowlisted component rather than silently broadening the registry service's network permissions.

## Validation

Run the following command from the repository root before installing or publishing the service:

```bash
scripts/validate_registry_deployment.sh
```

The validator checks that all reference artifacts exist, that TLS and loopback ingress rules are present, that sandbox and quota controls are declared, that credentials are sourced from a secret manager, that external egress is disabled, and that no populated secret or private-key file is tracked in the deployment tree. The GitHub Actions quality workflow runs the same gate.

## Boundary and non-goals

This reference layer completes the repository-side production-boundary contract. It does not perform public deployment, issue certificates, configure DNS, install system packages, create a cloud firewall, or provide a universal OS sandbox abstraction. Those steps remain platform-specific operational work and must be reviewed before exposing a registry to the Internet.
