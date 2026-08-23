# Zap Web Production Deployment Runbook

This runbook describes the production boundary for a Zap-native Web application on an Ubuntu host. The checked-in service keeps Zap on loopback, places Nginx at the public TLS boundary, and runs database migrations as a separate, explicitly invoked unit. The systemd restrictions follow the principle of reducing service filesystem and kernel authority rather than treating a process limit as a complete sandbox [1]. The templates are reference controls; replace the domain, certificate paths, filesystem paths, identity provider, database provider, and resource limits with deployment-reviewed values.

## Deployment topology

| Layer | Responsibility | Publicly reachable |
|---|---|---:|
| Nginx | TLS termination, HTTP-to-HTTPS redirect, request-size/time limits, forwarded headers, method allowlist | Yes, ports 80/443 |
| `zap-web.service` | Zap project validation and Web server process | No, loopback `127.0.0.1:3000` |
| `zap-web-migrate.service` | One-shot migration execution under an exclusive `flock` lock | No |
| Database | SQLite file for the current native adapter, or a future provider-backed adapter | No direct public access |

The browser and frontend build toolchain are not part of the runtime service. The deployed application needs the Zap executable, the Zap project, and the emitted `public/` tree. React, Vue, Svelte, or another JavaScript compiler is a build-time dependency only.

## Files supplied by the Framework branch

```text
deploy/zap-web.service
deploy/zap-web-migrate.service
deploy/zap-web.nginx.conf
deploy/zap-web.env.example
deploy/zap-web-deployment-policy.toml
scripts/validate_zap_web_deployment.sh
```

Run the repository gate before copying artifacts:

```bash
./scripts/validate_zap_web_deployment.sh
```

## Initial host preparation

Create a dedicated unprivileged account and directories. Do not run the application as root and do not place a populated secret environment file in the repository.

```bash
sudo useradd --system --user-group --home /srv/zap --shell /usr/sbin/nologin zap
sudo install -d -o zap -g zap -m 0750 /srv/zap/app
sudo install -d -o zap -g zap -m 0700 /srv/zap/app/data
sudo install -d -o root -g root -m 0755 /etc/zap/tls
sudo install -d -o root -g root -m 0755 /etc/zap
sudo install -m 0755 bin/zap /usr/local/bin/zap
sudo install -m 0644 deploy/zap-web.service /etc/systemd/system/zap-web.service
sudo install -m 0644 deploy/zap-web-migrate.service /etc/systemd/system/zap-web-migrate.service
sudo install -m 0644 deploy/zap-web.nginx.conf /etc/nginx/sites-available/zap-web.conf
sudo ln -sfn /etc/nginx/sites-available/zap-web.conf /etc/nginx/sites-enabled/zap-web.conf
```

Copy the application artifact to `/srv/zap/app`, preserving ownership and modes. The service uses `ProtectSystem=strict` and only permits writes under `/srv/zap/app/data`. If an external provider needs a different writable directory, change both the service policy and the deployment review; do not silently widen the filesystem.

Create the environment file through a secret-management or configuration-management process:

```bash
sudo install -o root -g zap -m 0640 deploy/zap-web.env.example /etc/zap/zap-web.env
sudoedit /etc/zap/zap-web.env
```

The template includes `ZAP_DB_MAX_CONNECTIONS`, `ZAP_DB_ACQUIRE_TIMEOUT_MS`, and `ZAP_DB_QUERY_TIMEOUT_MS`. These values are policy inputs for the production host adapter. They do not create a database pool inside the demo `zap-host` executable; a real repository must consume them.

## Nginx and TLS

Edit `/etc/nginx/sites-enabled/zap-web.conf` and replace `app.example.com`, `fullchain.pem`, and `privkey.pem`. Nginx reverse-proxy forwarding and HTTPS listener behavior follow its documented proxy and TLS configuration model [2] [3]. The Nginx template redirects port 80 to HTTPS, allows only GET and POST, limits request bodies to 64 KiB, forwards the original host/protocol/client chain, and sends requests to loopback. Keep the proxy upstream private.

Validate before reload:

```bash
sudo nginx -t
sudo systemctl reload nginx
curl -fsS https://app.example.com/health
curl -i https://app.example.com/ready
```

`/health` is a liveness signal. `/ready` is the dependency/readiness signal and may return `503` while the process is alive but not safe to receive application traffic. The load balancer should remove an instance from rotation when `/ready` fails and should not treat a green `/health` response as proof that the database is usable.

TLS certificates and private keys must be provisioned outside Git. Restrict the private-key file to root, rotate it through the certificate-management process, and reload Nginx only after `nginx -t` passes. The HSTS header in the template is appropriate only when the domain is permanently HTTPS-capable.

## Migration-first rollout

A deployment must validate the artifact before it touches the database. The recommended order is:

```bash
sudo systemctl daemon-reload
sudo systemctl stop zap-web.service
sudo -u zap /usr/local/bin/zap build --locked /srv/zap/app
sudo -u zap /usr/local/bin/zap web check /srv/zap/app
sudo -u zap /usr/local/bin/zap db check /srv/zap/app
sudo -u zap /usr/local/bin/zap db plan /srv/zap/app
sudo -u zap /usr/local/bin/zap db migrate --dry-run /srv/zap/app
```

For the SQLite adapter, make a verified backup of the database before applying a production migration:

```bash
sudo install -o zap -g zap -m 0600 /srv/zap/app/data/zap.sqlite3 \
  "/srv/zap/app/data/zap.sqlite3.$(date -u +%Y%m%dT%H%M%SZ).bak"
sudo systemctl start zap-web-migrate.service
sudo systemctl status zap-web-migrate.service --no-pager
sudo -u zap /usr/local/bin/zap db migrate --check /srv/zap/app
sudo systemctl start zap-web.service
sudo systemctl is-active --quiet zap-web.service
```

The migration unit applies migrations under `/usr/bin/flock -n /run/zap/zap-web-migrate.lock`. This prevents two deploy operators from applying the same database concurrently on one host. In a multi-host deployment, use a provider-backed advisory lock or a deployment orchestrator lock; a local filesystem lock is not distributed.

Do not enable the migration unit as an unconditional dependency of every Web process start. Migrations are a release operation, not a per-worker boot hook. If a migration fails, keep the Web service stopped, inspect the journal, restore the verified backup when appropriate, and deploy a forward corrective migration. Applied migration files are checksummed; edit an applied migration only by creating a new migration.

## Starting and observing the service

```bash
sudo systemctl enable --now zap-web.service
sudo systemctl status zap-web.service --no-pager
sudo journalctl -u zap-web.service -n 100 --no-pager
sudo ss -ltnp | grep ':3000'
curl -fsS http://127.0.0.1:3000/health
```

The expected socket is loopback-only. If the process listens on a public address, stop the deployment and fix the service/environment configuration before exposing Nginx. During a rollout, stop accepting new traffic through readiness, allow the configured drain period, then terminate the old process. Do not use `kill -9` as the normal deployment path.

## Rollback boundary

The Framework migration format currently provides transactional, checksum-protected SQLite apply and does not provide automatic down migrations. Application rollback and schema rollback are therefore separate decisions. A backward-compatible application release can usually be rolled back without reversing an additive schema change; a destructive schema change requires a tested backup/restore procedure or a forward compatibility migration. Never claim a rollback is safe merely because the systemd unit restarted successfully.

## Production limitations

The checked-in `zap-web` units do not turn the demo `zap-host` authenticator or memory repository into production identity and persistence. A production deployment still needs a real authenticator, a real repository/pool implementation, shared rate-limit state for multiple instances, observability with secret redaction, provider-specific egress controls, certificate automation, backup verification, and load/chaos evidence.

## References

[1]: https://documentation.suse.com/smart/security/html/systemd-securing/index.html SUSE Linux Enterprise Server — Securing systemd Services.
[2]: https://docs.nginx.com/nginx/admin-guide/web-server/reverse-proxy/ NGINX — Reverse Proxy Administration Guide.
[3]: https://nginx.org/en/docs/http/configuring_https_servers.html NGINX — Configuring HTTPS Servers.
