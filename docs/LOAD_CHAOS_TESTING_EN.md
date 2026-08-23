# Zap Production Load and Chaos Testing

The Framework branch includes two bounded, standard-library test tools. `scripts/load_zap_host.py` produces a redacted latency/status report for an explicitly supplied endpoint. `scripts/chaos_zap_host.py` runs opt-in authentication and service-recovery experiments. Both tools default to loopback targets; a remote target requires an explicit flag. They do not generate traffic against an unknown public service by accident.

## Load test stages

Run load tests in this order: first a local/staging smoke test, then a short ramp on a production canary, then a bounded soak window. Record the deployment version, instance count, database pool settings, request budget, success ratio, p95/p99 latency, CPU/memory, database saturation, pool wait time, and error categories. A green HTTP status alone is not evidence that the database, identity provider, or downstream dependencies are healthy.

| Stage | Suggested purpose | Example budget |
|---|---|---:|
| Smoke | Validate route, proxy, auth, and report generation | 10 seconds, 2 workers |
| Ramp | Find the first saturation point on one canary | 60 seconds, 8–32 workers |
| Soak | Detect leaks, pool starvation, and cache churn | 10–30 minutes, approved steady rate |

The script is duration- and concurrency-bounded. It reads an optional bearer token from `ZAP_LOAD_BEARER_TOKEN` rather than accepting a token on the command line, and it never prints that token. Remote testing requires `--allow-remote`; use it only for an approved host and canary window.

### Health endpoint smoke test

```bash
python3 scripts/load_zap_host.py \
  --url http://127.0.0.1:3000/health \
  --duration-seconds 10 \
  --concurrency 2 \
  --max-p95-ms 200 \
  --min-success-ratio 1.0 \
  --output target/load-health.json
```

### Authenticated API canary

```bash
export ZAP_LOAD_BEARER_TOKEN="$(security-tool read zap/staging/load-token)"
python3 scripts/load_zap_host.py \
  --url https://api.example.com/api/users \
  --allow-remote \
  --duration-seconds 60 \
  --concurrency 16 \
  --max-p95-ms 500 \
  --min-success-ratio 0.995 \
  --output target/load-api-canary.json
unset ZAP_LOAD_BEARER_TOKEN
```

Use a short-lived, least-privilege test token. Do not use a production administrator token, a refresh token, or a token copied from a user session. Keep the target path read-only for load testing unless the test plan explicitly provisions and cleans up synthetic data.

The report contains only the normalized target URL, status counts, request count, success ratio, and latency percentiles. It does not contain authorization headers, response bodies, query strings, or token values.

## Chaos experiments

Chaos experiments must run during an approved maintenance/canary window with an operator watching `/ready`, Nginx, systemd, database, and identity-provider telemetry. The service-control experiments require both `--allow-service-control` and the exact confirmation string `I_UNDERSTAND_DOWNTIME`.

### Invalid JWT rejection

This verifies that a protected route rejects a malformed bearer token without reaching the application gateway:

```bash
python3 scripts/chaos_zap_host.py \
  --fault invalid-jwt \
  --url http://127.0.0.1:3000/api/users
```

The expected result is `401`. If the response is `200`, the service is still in demo-authenticator mode or the protected route is not protected. Treat that as a failed production gate.

### Process restart and recovery

```bash
sudo python3 scripts/chaos_zap_host.py \
  --fault restart-service \
  --url http://127.0.0.1:3000/health \
  --service zap-web.service \
  --allow-service-control \
  --confirm I_UNDERSTAND_DOWNTIME \
  --recovery-timeout-seconds 60
```

The experiment checks that the service was healthy before the restart and that `/health` returns `200` within the recovery budget. A successful restart does not prove that `/ready`, database migrations, authentication, or pool recovery are correct; follow it with readiness and authenticated smoke tests.

### Stop/start recovery

```bash
sudo python3 scripts/chaos_zap_host.py \
  --fault stop-start-service \
  --url http://127.0.0.1:3000/health \
  --service zap-web.service \
  --allow-service-control \
  --confirm I_UNDERSTAND_DOWNTIME \
  --recovery-timeout-seconds 60
```

The experiment expects the service to become unavailable after stop and recover after start. Do not run it behind a load balancer without first removing the canary from rotation. The script validates only the local process lifecycle; it does not claim a zero-downtime deployment.

## Dependency-failure experiments

The checked-in script intentionally does not kill a remote database, identity provider, or network interface. Those failures require an approved staging fault-injection layer or provider-specific test switch. A safe plan is to direct a staging JWKS URL to a controlled endpoint that returns `503`, verify that invalidation maps to `503 authentication_unavailable`, restore the endpoint, and verify recovery. For database failure, use a staging repository/provider kill switch and verify `/ready` fails while `/health` remains a liveness signal.

Never use a public DNS poisoning, broad firewall flush, credential deletion, or production database termination as a first chaos experiment. Every experiment needs a hypothesis, a blast-radius limit, an automatic or manual rollback, an abort signal, and a post-test evidence bundle.

## CI and release gate

Use `bash -n` for shell scripts, `python3 -m py_compile` for the Python tools, the checked-in deployment validators, Rust tests/Clippy, and a local smoke load test in CI. Run remote load and service-control chaos only as separately approved environment jobs; they should not run on every pull request.
