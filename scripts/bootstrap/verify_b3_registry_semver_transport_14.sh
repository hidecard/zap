#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b3-registry.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b3/package.zp"
let d120 = registry_package("demo", "1.2.0", "d120", [])
let d129 = registry_package("demo", "1.2.9", "d129", [])
let d142 = registry_package("demo", "1.4.2", "d142", [])
let d130 = registry_package("demo", "1.3.0", "d130", [])
let d200 = registry_package("demo", "2.0.0", "d200", [])
let caret = resolve_dependency_graph([dependency("demo", "^1.2.0", "")], [d120, d142, d200])
let tilde = resolve_dependency_graph([dependency("demo", "~1.2.0", "")], [d120, d129, d130])
let intersection = resolve_dependency_graph([dependency("demo", ">=1.2.0,<1.3.0", "")], [d120, d129, d130])
let invalid = resolve_dependency_graph([dependency("demo", "^broken", "")], [d120])
let package = registry_package("demo", "1.2.3", "checksum", [])
let https_request = registry_transport_request("https://registry.example", "GET", "/index.json", 5000, 1024, false)
let file_request = registry_transport_request("file:///tmp/registry", "GET", "/index.json", 5000, 1024, false)
let http_rejected = registry_transport_request("http://registry.example", "GET", "/index.json", 5000, 1024, false)
let http_allowed = registry_transport_request("http://registry.example", "GET", "/index.json", 5000, 1024, true)
let invalid_url = registry_transport_request("https://registry.example/../private", "GET", "/index.json", 5000, 1024, false)
let bad_method = registry_transport_request("https://registry.example", "DELETE", "/index.json", 5000, 1024, false)
let bad_limits = registry_transport_request("https://registry.example", "GET", "/index.json", 0, 1024, false)
let response = registry_transport_response(200, "application/json; charset=utf-8", "{\"packages\":[]}", 1024)
let bad_status = registry_transport_response(503, "application/json", "{}", 1024)
let bad_type = registry_transport_response(200, "text/plain", "{}", 1024)
let bad_size = registry_transport_response(200, "application/json", "0123456789", 4)
let cache_ok = registry_cache_verify(package, "checksum")
let cache_bad = registry_cache_verify(package, "wrong")
let cache_missing = registry_cache_verify(package, "")
say semver_parse("1.2")["patch"]
say caret["ok"]
say caret["resolved"][0]["version"]
say tilde["resolved"][0]["version"]
say intersection["resolved"][0]["version"]
say invalid["errors"][0]["code"]
say https_request["ok"]
say file_request["ok"]
say http_rejected["error"]
say http_allowed["ok"]
say invalid_url["error"]
say bad_method["error"]
say bad_limits["error"]
say response["status"]
say bad_status["error"]
say bad_type["error"]
say bad_size["error"]
say cache_ok["status"]
say cache_bad["error"]
say cache_missing["error"]
ZP
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "0 true 1.4.2 1.2.9 1.2.9 ZAP-PKG-VERSION-INVALID-001 true true registry_url_rejected true registry_url_rejected registry_method_rejected registry_request_limits_invalid accepted registry_http_status registry_content_type_rejected registry_response_too_large_or_empty verified cache_checksum_mismatch cache_checksum_missing" ]]; then
  echo "unexpected registry/semver output: ${lines[*]}" >&2
  exit 1
fi
printf 'B3 registry-semver transport gate passed: range selection, invalid constraints, URL policy, response limits, cache integrity, and deterministic diagnostics\n'
