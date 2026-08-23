#!/usr/bin/env bash
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
unit="$root_dir/deploy/zap-registry.service"
ingress="$root_dir/deploy/zap-registry.nginx.conf"
policy="$root_dir/deploy/registry-deployment-policy.toml"
env_example="$root_dir/deploy/registry.env.example"

for required in "$unit" "$ingress" "$policy" "$env_example"; do
    test -f "$required" || { echo "missing deployment artifact: $required" >&2; exit 1; }
done

require_text() {
    local file="$1"
    local text="$2"
    grep -Fqx "$text" "$file" || { echo "missing policy line in $file: $text" >&2; exit 1; }
}

require_text "$unit" 'User=zap-registry'
require_text "$unit" 'DynamicUser=yes'
require_text "$unit" 'StateDirectory=zap-registry'
require_text "$unit" 'StateDirectoryMode=0700'
require_text "$unit" 'NoNewPrivileges=yes'
require_text "$unit" 'ProtectSystem=strict'
require_text "$unit" 'ProtectKernelLogs=yes'
require_text "$unit" 'RestrictSUIDSGID=yes'
require_text "$unit" 'LockPersonality=yes'
require_text "$unit" 'CapabilityBoundingSet='
require_text "$unit" 'AmbientCapabilities='
require_text "$unit" 'MemoryMax=256M'
require_text "$unit" 'TasksMax=64'
require_text "$unit" 'IPAddressDeny=any'
require_text "$unit" 'IPAddressAllow=127.0.0.0/8'
require_text "$unit" 'IPAddressAllow=::1/128'
require_text "$unit" 'KillMode=control-group'
require_text "$unit" 'ExecStart=/usr/local/bin/zap registry serve /var/lib/zap-registry 127.0.0.1:8787'
grep -Fqx '    location = /healthz {' "$ingress" || { echo "missing loopback health endpoint in $ingress" >&2; exit 1; }
grep -Fqx '    location = /readyz {' "$ingress" || { echo "missing loopback readiness endpoint in $ingress" >&2; exit 1; }
grep -Fqx '        proxy_pass http://127.0.0.1:8787/healthz;' "$ingress" || { echo "missing health upstream in $ingress" >&2; exit 1; }
grep -Fqx '        proxy_pass http://127.0.0.1:8787/readyz;' "$ingress" || { echo "missing readiness upstream in $ingress" >&2; exit 1; }
grep -Fqx '        deny all;' "$ingress" || { echo "missing health endpoint access restriction in $ingress" >&2; exit 1; }
grep -Eq '^[[:space:]]*ssl_protocols[[:space:]]+TLSv1\.2 TLSv1\.3;' "$ingress" || { echo "missing TLS protocol policy in $ingress" >&2; exit 1; }
grep -Eq '^[[:space:]]*client_max_body_size[[:space:]]+1m;' "$ingress" || { echo "missing request-size policy in $ingress" >&2; exit 1; }
grep -Eq '^[[:space:]]*proxy_pass[[:space:]]+http://127\.0\.0\.1:8787;' "$ingress" || { echo "missing loopback upstream policy in $ingress" >&2; exit 1; }
require_text "$policy" 'external_egress = false'
require_text "$policy" 'source = "deployment-secret-manager"'
require_text "$env_example" 'ZAP_REGISTRY_TOKEN=replace-with-secret-manager-token'
require_text "$env_example" 'ZAP_REGISTRY_SIGNING_SECRET=replace-with-secret-manager-signing-secret'

# Never permit populated secret files or private keys in the deployment tree.
if find "$root_dir/deploy" -type f \( -name '*.key' -o -name '*.pem' -o -name 'registry.env' \) -print -quit | grep -q .; then
    echo 'deployment tree contains a populated secret or private-key file' >&2
    exit 1
fi

printf '%s\n' 'registry deployment policy: valid reference controls'
