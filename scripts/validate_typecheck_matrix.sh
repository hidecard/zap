#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EN_FILE="${ROOT_DIR}/docs/TYPECHECK_CONFORMANCE_MATRIX_EN.md"
MM_FILE="${ROOT_DIR}/docs/TYPECHECK_CONFORMANCE_MATRIX_MM.md"

fail() {
  printf 'type-checking matrix validation failed: %s\n' "$1" >&2
  exit 1
}

[[ -s "$EN_FILE" ]] || fail "missing English matrix: $EN_FILE"
[[ -s "$MM_FILE" ]] || fail "missing Burmese matrix: $MM_FILE"

for id in TC-001 TC-002 TC-003 TC-004 TC-005 TC-006 TC-007 TC-008 TC-009 TC-010 TC-011 TC-012; do
  grep -F "$id" "$EN_FILE" >/dev/null || fail "English matrix is missing $id"
  grep -F "$id" "$MM_FILE" >/dev/null || fail "Burmese matrix is missing $id"
done

for marker in 'file' 'line' 'column' 'TypeError' 'L0' 'L1' 'L2' 'L3' 'L4'; do
  grep -F "$marker" "$EN_FILE" >/dev/null || fail "English matrix is missing marker '$marker'"
  grep -F "$marker" "$MM_FILE" >/dev/null || fail "Burmese matrix is missing marker '$marker'"
done

mapfile -t en_ids < <(grep -oE 'TC-[0-9]{3}' "$EN_FILE" | sort -u)
mapfile -t mm_ids < <(grep -oE 'TC-[0-9]{3}' "$MM_FILE" | sort -u)
[[ "${en_ids[*]}" == "${mm_ids[*]}" ]] || fail "English/Burmese conformance IDs differ"

git -C "$ROOT_DIR" diff --check
printf 'type-checking conformance matrix validation passed (%d IDs, bilingual parity intact).\n' "${#en_ids[@]}"
