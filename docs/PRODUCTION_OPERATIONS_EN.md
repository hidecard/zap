# Zap Production Operations Runbook

**Verified baseline:** Zap v2.11.3 development line

**Scope:** This runbook deploys the built-in authenticated registry behind a Linux systemd service and an nginx TLS ingress. It is a reference runbook, not an automatic cloud provisioning system. Production operators must adapt the firewall, certificate authority, secret manager, monitoring, backup, and approval steps to their environment.

> **Non-negotiable boundary:** Never expose `zap registry serve` directly to the Internet. Bind the backend to loopback, terminate TLS at a maintained ingress proxy, and keep the backend's filesystem and network permissions narrow.

## 1. Architecture and required components

The reference deployment has four components: a release binary installed at `/usr/local/bin/zap`, a systemd-managed backend bound to `127.0.0.1:8787`, an nginx TLS virtual host that is the only public entry point, and an external secret/monitoring/backup system.

| Component | Required production control |
|---|---|
| Zap binary | Download from a trusted release, verify SHA-256, pin the version, and retain the previous binary for rollback. |
| Registry data | Store under `/var/lib/zap-registry`; back it up independently and test restoration. |
| Service | Use `deploy/zap-registry.service`, including `DynamicUser`, `StateDirectory`, quotas, loopback binding, and process-group cleanup. |
| Ingress | Use `deploy/zap-registry.nginx.conf` as a starting point; replace the example hostname and certificate paths. |
| Credentials | Inject `ZAP_REGISTRY_TOKEN` and `ZAP_REGISTRY_SIGNING_SECRET` from a secret manager or a mode-0600 environment file. |
| Monitoring | Collect systemd journal and nginx logs, host resource metrics, and external health checks. |
| Backup and recovery | Back up registry data, preserve signing-key material according to policy, and perform a restore drill before launch. |

The backend now uses eight request workers and a bounded queue of 32 connections. When the queue is full it returns `503 Service Unavailable` rather than admitting unbounded work. The service exposes unauthenticated `GET /healthz` and `GET /readyz` for local probes; nginx restricts these paths to loopback.

## 2. Host preparation

Use a dedicated Linux host or VM. Apply current operating-system security updates, restrict administrative access, synchronize time, and configure the host firewall before starting the service. Expose only the TLS port required by the ingress, normally TCP 443. Do not expose TCP 8787 externally.

Install the required host packages through the operating system's trusted package channel:

```bash
sudo apt-get update
sudo apt-get install --yes nginx curl ca-certificates
```

Create the service configuration directory and protect it:

```bash
sudo install -d -m 0750 /etc/zap
sudo install -d -m 0755 /var/lib/zap-registry
```

The systemd unit uses `StateDirectory=zap-registry`, so systemd owns the final state-directory setup. Do not place secrets or private keys in the repository checkout or in `/var/lib/zap-registry`.

## 3. Install and verify the binary

Download the release archive and checksum through an approved channel. Verify the checksum before extracting or installing:

```bash
sha256sum -c zap-<version>-linux-x86_64.tar.gz.sha256
```

Install the binary atomically. Keep the old binary available until the new version passes health and smoke checks:

```bash
install -d -m 0755 /usr/local/bin
install -m 0755 ./bin/zap /usr/local/bin/zap.new
/usr/local/bin/zap.new --version
mv /usr/local/bin/zap.new /usr/local/bin/zap
/usr/local/bin/zap --version
```

For a source build, use the repository's pinned Rust toolchain and locked dependency graph instead of an unreviewed floating build:

```bash
cargo build --release --locked --manifest-path native/Cargo.toml
install -m 0755 native/target/release/zap /usr/local/bin/zap.new
```

A production release should come from the project's signed/reviewed release process. The repository-side RustSec gate and provenance checks do not replace operator verification of the downloaded artifact.

## 4. Configure secrets safely

Create `/etc/zap/registry.env` using the deployment secret manager. The file must contain real values only on the host and must never be committed:

```bash
sudo install -m 0600 /dev/null /etc/zap/registry.env
sudoedit /etc/zap/registry.env
```

Required variables:

```text
ZAP_REGISTRY_TOKEN=generated-service-token
ZAP_REGISTRY_SIGNING_SECRET=generated-signing-secret
```

Use high-entropy, independently generated values. The bearer token authenticates service requests. The signing secret protects the persisted signed index; rotating it without a documented migration can invalidate existing signed metadata. Plan token rotation and signing-secret rotation separately, with an approval, backup, overlap, and rollback procedure. Do not put either value in a systemd command line, nginx configuration, source file, manifest, lockfile, log, or chat message.

## 5. Install and validate systemd

Copy the reviewed unit and validate its exact contents:

```bash
sudo install -m 0644 deploy/zap-registry.service /etc/systemd/system/zap-registry.service
scripts/validate_registry_deployment.sh
sudo systemd-analyze verify /etc/systemd/system/zap-registry.service
sudo systemctl daemon-reload
sudo systemctl enable --now zap-registry.service
```

The unit's command order is significant:

```text
/usr/local/bin/zap registry serve /var/lib/zap-registry 127.0.0.1:8787
```

The registry root is the first argument and the optional bind address is second. Confirm the listening address and process identity:

```bash
sudo systemctl status --no-pager zap-registry
sudo ss -ltnp | grep ':8787'
```

The service uses `DynamicUser`, `StateDirectory=zap-registry`, `ProtectSystem=strict`, `ProtectHome`, `NoNewPrivileges`, a restrictive umask, empty capability sets, memory/CPU/task/open-file quotas, loopback-only address policy, and `KillMode=control-group`. Do not remove a control to make an unrelated application issue disappear; record an explicit security review if a deployment-specific change is unavoidable.

## 6. Configure nginx TLS ingress

Copy the reference configuration and replace the example hostname and certificate paths with certificates issued and renewed by the organization's approved certificate system:

```bash
sudo install -m 0644 deploy/zap-registry.nginx.conf /etc/nginx/conf.d/zap-registry.conf
sudoedit /etc/nginx/conf.d/zap-registry.conf
sudo nginx -t
sudo systemctl reload nginx
```

The reference proxy redirects HTTP to HTTPS, allows TLS 1.2 and TLS 1.3, limits request bodies, permits only `GET` and `POST`, sets bounded proxy timeouts, and forwards to `127.0.0.1:8787`. Add organization-approved rate limiting, WAF rules, request logging, and upstream access policy before exposing a high-volume public service.

The `/healthz` and `/readyz` locations allow only `127.0.0.1` and `::1`. If a load balancer must probe them, permit only that load balancer's fixed source CIDR and do not use a broad Internet allow rule. Keep TCP 8787 blocked at the host/network firewall.

## 7. Smoke test before public traffic

Verify liveness, readiness, logs, and the public TLS endpoint in that order:

```bash
curl --fail http://127.0.0.1:8787/healthz
curl --fail http://127.0.0.1:8787/readyz
sudo journalctl -u zap-registry --since '5 minutes ago' --no-pager
sudo nginx -t
curl --fail --silent --show-error https://registry.example/healthz
```

The public health request should be allowed only if the nginx policy intentionally permits its source. Test authentication and method restrictions with a disposable package/index fixture. Verify that an unauthenticated publish request receives `401`, a traversal-style package identity is rejected, an invalid checksum is rejected before persistence, and a valid signed index can be read back.

## 8. Client configuration and package workflow

On a developer or CI machine, trust the exact HTTPS origin and configure credentials through an environment-variable reference:

```bash
zap registry trust add https://registry.example/team
export ZAP_REGISTRY_TOKEN_CI="$(secret-manager read zap/registry/read-token)"
zap registry credential set https://registry.example/team --token-env ZAP_REGISTRY_TOKEN_CI
zap install --locked .
```

For offline deployment or an emergency network freeze:

```bash
ZAP_OFFLINE=1 zap install --locked .
```

Offline mode must use only already-cached, checksum-verified artifacts. Before publishing, format, lint, check, test, and build the package with the lockfile:

```bash
zap fmt main.zp
zap lint main.zp
zap check .
zap test --fail-fast .
zap build --locked .
```

Compute the archive checksum immediately before publishing:

```bash
checksum="$(sha256sum ./demo.pkg | awk '{print $1}')"
export ZAP_REGISTRY_TOKEN="$(secret-manager read zap/registry/publish-token)"
zap registry publish https://registry.example/team/publish ./demo.pkg demo 1.0.0 "$checksum"
```

Review the package identity, version, provenance, and checksum in the resulting index. Do not overwrite a released version without an approved registry policy.

## 9. Monitoring and alerting

Zap does not expose built-in Prometheus metrics or a durable job queue. At minimum, monitor systemd service state, restart count, CPU/memory/task/file-descriptor pressure, nginx 4xx/5xx rates, TLS certificate expiry, disk usage under `/var/lib/zap-registry`, health/readiness failures, and authentication failures.

Recommended alert conditions include repeated restarts, any readiness failure, sustained `503` responses, disk usage above the operator threshold, certificate expiry within the renewal window, unexpected changes to the registry data directory, and a sudden increase in unauthorized or forbidden responses. Collect logs centrally with secret redaction and a retention policy appropriate to the environment.

## 10. Backup, restore, and rollback

Back up `/var/lib/zap-registry` using an approved encrypted backup system. Keep the environment file and signing-secret backup under separate access controls. A backup is not complete until a clean restore has been tested.

A basic operator procedure is:

```bash
sudo systemctl stop zap-registry
sudo tar --xattrs --acls -czf /secure-backup/zap-registry-$(date -u +%Y%m%dT%H%M%SZ).tar.gz /var/lib/zap-registry
sudo systemctl start zap-registry
sudo curl --fail http://127.0.0.1:8787/readyz
```

For restore, stop the service, restore into a staging directory, verify ownership/permissions and signed-index integrity, move the restored state into place, start the service, and run the full smoke test. Never restore untrusted archives directly over live state.

For application rollback, stop the service, install the previously verified binary, start it, verify readiness, and compare registry/index behavior. Do not silently roll back the signing secret or delete registry data as part of a binary rollback. Record the incident, version, checksum, operator approval, and validation results.

## 11. Security boundaries and deferred controls

The runtime's `ZAP_UNTRUSTED=1` mode provides capability denials and bounded execution behavior, but it is not a universal kernel sandbox. For untrusted customer code, use a separate VM or container policy with read-only source mounts, a dedicated writable directory, a minimal environment, no host credentials, CPU/memory/process/time quotas, syscall/network policy, and an explicit egress allowlist.

The reference deployment does not provide built-in certificate pinning, OS keychain integration, universal cross-platform sandboxing, automatic cloud firewall provisioning, or production signed-index key-management policy. Those controls must be supplied and reviewed by the deployment owner. Filesystem canonicalization also does not eliminate every check/use race on every operating system; do not treat it as a substitute for descriptor-relative or handle-based isolation where an attacker can race the host filesystem.

## 12. Release gate

Before accepting production traffic, obtain evidence for every row:

| Gate | Evidence |
|---|---|
| Artifact integrity | Release checksum/signature verified and version recorded. |
| Dependency security | RustSec audit passed in CI with the current advisory database. |
| Runtime quality | Locked format/check/Clippy/tests passed. |
| Deployment contract | `scripts/validate_registry_deployment.sh` and `systemd-analyze verify` passed. |
| Network boundary | Backend listens only on loopback; firewall blocks 8787; TLS ingress is active. |
| Secrets | Secret manager injection verified; repository and logs contain no secret values. |
| Recovery | Backup and clean restore drill passed. |
| Observability | Logs, health checks, certificate alerts, resource alerts, and 5xx alerts are active. |
| Rollback | Previous binary and operator-approved rollback procedure are available. |

Only after all gates pass should DNS or the public load balancer route production traffic to the ingress. Keep the release boundary in mind: a repository branch or pull request is not itself a published release artifact. Install only a tagged release whose checksums, signatures, provenance, and published assets have been verified.
