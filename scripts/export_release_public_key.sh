#!/usr/bin/env bash
# Export only the public verification key from an injected CI keyring.
# No private key material is read from repository files or written to output.
set -euo pipefail

SIGNING_KEY_ID="${SIGNING_KEY_ID:-}"
OUTPUT="${1:-}"

fail() {
  echo "release public key export: $*" >&2
  exit 1
}

[[ -n "$SIGNING_KEY_ID" && -n "$OUTPUT" ]] || {
  echo "usage: SIGNING_KEY_ID=KEY_ID $0 OUTPUT_ASC" >&2
  exit 2
}
command -v gpg >/dev/null 2>&1 || fail 'gpg is required'
[[ -n "${GNUPGHOME:-}" ]] || fail 'GNUPGHOME must point to the ephemeral or approved verification keyring'
[[ -d "$GNUPGHOME" ]] || fail "GNUPGHOME does not exist: $GNUPGHOME"

mkdir -p "$(dirname "$OUTPUT")"
tmp="$(mktemp "${OUTPUT}.tmp.XXXXXX")"
trap 'rm -f "$tmp"' EXIT

if ! gpg --batch --armor --export "$SIGNING_KEY_ID" > "$tmp"; then
  fail "public key not found: $SIGNING_KEY_ID"
fi
[[ -s "$tmp" ]] || fail 'public key export is empty'
! grep -Fq 'BEGIN PGP PRIVATE KEY BLOCK' "$tmp" || fail 'private key material detected in export'
grep -Fq 'BEGIN PGP PUBLIC KEY BLOCK' "$tmp" || fail 'public key armor was not produced'
chmod 0644 "$tmp"
mv -f "$tmp" "$OUTPUT"
trap - EXIT

printf '%s\n' 'release public key export: passed'
printf '  key: %s\n' "$SIGNING_KEY_ID"
printf '  output: %s\n' "$OUTPUT"
