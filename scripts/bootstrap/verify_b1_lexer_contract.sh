#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

# The differential corpus remains the compatibility oracle for the B1 lexer.
bash scripts/bootstrap/verify_b1_lexer.sh

# B1 lexer ownership must be Zap-owned while the overall bootstrap stays B0.
awk -F '\t' '$1 == "BOOT-002" { if ($5 != "bootstrap/b1/lexer.zp" || $7 != "stable") exit 1; found=1 } END { exit(found ? 0 : 1) }' bootstrap/contracts/OWNERS.tsv
awk -F '\t' '$1 == "BOOT-003" { if ($5 != "bootstrap/b1/lexer.zp" || $7 != "stable") exit 1; found=1 } END { exit(found ? 0 : 1) }' bootstrap/contracts/OWNERS.tsv

grep -qx 'lexer_stage = "B1"' <(sed -n '/^\[bootstrap\]/,/^\[platform_seed\]/p' bootstrap/contracts/VERSIONS.toml | grep '^lexer_stage')
grep -qx 'lexer_owner = "bootstrap/b1/lexer.zp"' <(sed -n '/^\[bootstrap\]/,/^\[platform_seed\]/p' bootstrap/contracts/VERSIONS.toml | grep '^lexer_owner')

# The Zap lexer candidate must not delegate to Rust or Cargo.
if awk '!/^#/ && tolower($0) ~ /rust|cargo/ { print NR ":" $0; found = 1 } END { exit found ? 0 : 1 }' bootstrap/b1/lexer.zp; then
  echo 'B1 lexer contract failed: Rust/Cargo reference found in executable B1 lexer code' >&2
  exit 1
fi

printf 'B1 lexer contract gate passed: differential corpus, ownership, version milestone, and no-Rust boundary\n'
