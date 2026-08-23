#!/usr/bin/env bash
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
unit="$root_dir/deploy/zap-web.service"
migrate_unit="$root_dir/deploy/zap-web-migrate.service"
ingress="$root_dir/deploy/zap-web.nginx.conf"
policy="$root_dir/deploy/zap-web-deployment-policy.toml"
env_example="$root_dir/deploy/zap-web.env.example"

for required in "$unit" "$migrate_unit" "$ingress" "$policy" "$env_example"; do
    test -f "$required" || { echo "missing Zap Web deployment artifact: $required" >&2; exit 1; }
done

require_exact() {
    local file="$1"
    local text="$2"
    grep -Fqx -- "$text" "$file" || { echo "missing required line in $file: $text" >&2; exit 1; }
}

require_exact "$unit" 'User=zap'
require_exact "$unit" 'Group=zap'
require_exact "$unit" 'WorkingDirectory=/srv/zap/app'
require_exact "$unit" 'EnvironmentFile=-/etc/zap/zap-web.env'
require_exact "$unit" 'ExecStartPre=/usr/local/bin/zap build --locked /srv/zap/app'
require_exact "$unit" 'ExecStartPre=/usr/local/bin/zap web check /srv/zap/app'
require_exact "$unit" 'ExecStart=/usr/local/bin/zap dev /srv/zap/app'
require_exact "$unit" 'ReadWritePaths=/srv/zap/app/data'
require_exact "$unit" 'NoNewPrivileges=yes'
require_exact "$unit" 'ProtectSystem=strict'
require_exact "$unit" 'IPAddressDeny=any'
require_exact "$unit" 'IPAddressAllow=127.0.0.0/8'
require_exact "$unit" 'MemoryMax=512M'

require_exact "$migrate_unit" 'Type=oneshot'
require_exact "$migrate_unit" 'ExecStartPre=/usr/local/bin/zap build --locked /srv/zap/app'
require_exact "$migrate_unit" 'ExecStartPre=/usr/local/bin/zap db check /srv/zap/app'
require_exact "$migrate_unit" 'ExecStart=/usr/bin/flock -n /run/zap/zap-web-migrate.lock /usr/local/bin/zap db migrate /srv/zap/app'
require_exact "$migrate_unit" 'ExecStartPost=/usr/local/bin/zap db migrate --check /srv/zap/app'
require_exact "$migrate_unit" 'ReadWritePaths=/srv/zap/app/data /run/zap'
require_exact "$migrate_unit" 'RestrictAddressFamilies=AF_UNIX'

for directive in \
    'ssl_protocols       TLSv1.2 TLSv1.3;' \
    'client_max_body_size 64k;' \
    'client_body_timeout 10s;' \
    'proxy_pass http://127.0.0.1:3000;' \
    'proxy_read_timeout 15s;' \
    'proxy_send_timeout 15s;' \
    'limit_except GET POST { deny all; }' \
    'return 308 https://$host$request_uri;'; do
    grep -Fq -- "$directive" "$ingress" || { echo "missing Nginx policy in $ingress: $directive" >&2; exit 1; }
done

grep -Fq 'proxy_set_header X-Forwarded-Proto https;' "$ingress" || { echo "missing forwarded-proto policy" >&2; exit 1; }
grep -Eq '^[[:space:]]*ssl_certificate_key[[:space:]]+/etc/zap/tls/privkey\.pem;' "$ingress" || { echo "missing certificate-key path" >&2; exit 1; }

require_exact "$policy" 'service = "zap-web"'
require_exact "$policy" 'bind_address = "127.0.0.1:3000"'
require_exact "$policy" 'public_ingress = "tls-terminating-proxy-only"'
require_exact "$policy" 'migration_lock = "/run/zap/zap-web-migrate.lock"'
require_exact "$policy" 'checksum_drift = "fail-closed"'
require_exact "$policy" 'transactional_apply = true'
require_exact "$policy" 'pool_owner = "host-adapter-repository"'
require_exact "$policy" 'migration_lock_required = true'
require_exact "$policy" 'pool_acquire_timeout_required = true'
require_exact "$policy" 'query_timeout_required = true'
require_exact "$policy" 'mode = "jwt-bearer-resource-server"'
require_exact "$policy" 'allowed_algorithms = ["RS256"]'
require_exact "$policy" 'raw_access_token_logging = false'
require_exact "$policy" 'id_token_as_api_access_token = false'

require_exact "$env_example" 'ZAP_WEB_PORT=3000'
require_exact "$env_example" 'ZAP_DB_MAX_CONNECTIONS=16'
require_exact "$env_example" 'ZAP_DB_ACQUIRE_TIMEOUT_MS=1000'
require_exact "$env_example" 'ZAP_DB_QUERY_TIMEOUT_MS=5000'
grep -Fq 'DATABASE_URL=<injected-by-secret-manager>' "$env_example" || { echo "missing external database secret placeholder" >&2; exit 1; }
require_exact "$env_example" 'ZAP_AUTH_MODE=jwt'
require_exact "$env_example" 'ZAP_AUTH_ALLOWED_ALGORITHMS=RS256'
require_exact "$env_example" 'ZAP_AUTH_CLOCK_SKEW_SECONDS=30'
require_exact "$env_example" 'ZAP_AUTH_JWKS_CACHE_SECONDS=300'
require_exact "$env_example" 'ZAP_AUTH_MAX_TOKEN_BYTES=16384'

if find "$root_dir/deploy" -type f \( -name 'zap-web.env' -o -name 'zap-host.env' -o -name '*.key' -o -name '*.pem' \) -print -quit | grep -q .; then
    echo 'deployment tree contains a populated secret environment or private-key file' >&2
    exit 1
fi

printf '%s\n' 'Zap Web deployment policy: valid reference controls'
