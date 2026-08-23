#!/usr/bin/env bash
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
unit="$root_dir/deploy/zap-host.service"
ingress="$root_dir/deploy/zap-host.nginx.conf"
policy="$root_dir/deploy/zap-host-deployment-policy.toml"
env_example="$root_dir/deploy/zap-host.env.example"

for required in "$unit" "$ingress" "$policy" "$env_example"; do
    [[ -f "$required" ]] || { echo "missing zap-host deployment artifact: $required" >&2; exit 1; }
done

require_exact() {
    local file="$1"
    local text="$2"
    grep -Fqx -- "$text" "$file" || { echo "missing policy line in $file: $text" >&2; exit 1; }
}

require_exact "$unit" 'User=zap-host'
require_exact "$unit" 'Group=zap-host'
require_exact "$unit" 'DynamicUser=yes'
require_exact "$unit" 'EnvironmentFile=-/etc/zap/zap-host.env'
require_exact "$unit" 'ExecStart=/usr/local/bin/zap-host'
require_exact "$unit" 'TimeoutStopSec=35s'
require_exact "$unit" 'NoNewPrivileges=yes'
require_exact "$unit" 'ProtectSystem=strict'
require_exact "$unit" 'MemoryMax=256M'
require_exact "$unit" 'TasksMax=128'
require_exact "$unit" 'IPAddressDeny=any'
require_exact "$unit" 'IPAddressAllow=127.0.0.0/8'
require_exact "$unit" 'IPAddressAllow=::1/128'

grep -Eq '^[[:space:]]*ssl_protocols[[:space:]]+TLSv1\.2 TLSv1\.3;' "$ingress" || { echo "missing TLS protocol policy in $ingress" >&2; exit 1; }
grep -Eq '^[[:space:]]*client_max_body_size[[:space:]]+64k;' "$ingress" || { echo "missing request-size policy in $ingress" >&2; exit 1; }
grep -Eq '^[[:space:]]*client_body_timeout[[:space:]]+10s;' "$ingress" || { echo "missing client-body timeout policy in $ingress" >&2; exit 1; }
grep -Eq '^[[:space:]]*proxy_pass[[:space:]]+http://127\.0\.0\.1:3000;' "$ingress" || { echo "missing loopback upstream policy in $ingress" >&2; exit 1; }
grep -Eq '^[[:space:]]*proxy_read_timeout[[:space:]]+15s;' "$ingress" || { echo "missing upstream read timeout policy in $ingress" >&2; exit 1; }
grep -Eq '^[[:space:]]*proxy_send_timeout[[:space:]]+15s;' "$ingress" || { echo "missing upstream send timeout policy in $ingress" >&2; exit 1; }
grep -Fq 'limit_except GET POST' "$ingress" || { echo "missing HTTP method allowlist in $ingress" >&2; exit 1; }
grep -Fq 'return 308 https://$host$request_uri;' "$ingress" || { echo "missing HTTP to HTTPS redirect in $ingress" >&2; exit 1; }

require_exact "$policy" 'service = "zap-host"'
require_exact "$policy" 'bind_address = "127.0.0.1:3000"'
require_exact "$policy" 'public_ingress = "tls-terminating-proxy-only"'
require_exact "$policy" 'max_request_bytes = 65536'
require_exact "$policy" 'request_timeout_seconds = 10'
require_exact "$policy" 'shutdown_drain_timeout_seconds = 30'
require_exact "$policy" 'readiness_path = "/ready"'
require_exact "$policy" 'external_egress = false'
require_exact "$policy" 'raw_credentials_enter_zap_contract = false'
require_exact "$policy" 'state = "shared-atomic-store-required-for-multiple-instances"'
require_exact "$policy" 'demo_repository_allowed = false'
require_exact "$policy" 'demo_authenticator_allowed = false'
require_exact "$policy" 'local_rate_limit_store_allowed = false'
require_exact "$policy" 'runtime_cli_subprocess_per_request_allowed = false'

require_exact "$env_example" 'ZAP_HOST_ADDR=127.0.0.1:3000'
require_exact "$env_example" 'ZAP_HOST_SHUTDOWN_TIMEOUT_MS=30000'
grep -Fq 'DATABASE_URL=<injected-by-secret-manager>' "$env_example" || { echo "missing database secret-source placeholder in $env_example" >&2; exit 1; }
grep -Fq 'OIDC_ISSUER=<deployment-configured>' "$env_example" || { echo "missing identity-provider placeholder in $env_example" >&2; exit 1; }

if find "$root_dir/deploy" -type f \( -name 'zap-host.env' -o -name '*.key' -o -name '*.pem' \) -print -quit | grep -q .; then
    echo 'deployment tree contains a populated zap-host secret or private-key file' >&2
    exit 1
fi

printf '%s\n' 'zap-host deployment policy: valid reference controls'
