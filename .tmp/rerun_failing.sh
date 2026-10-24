#!/usr/bin/env bash
set -uo pipefail
cd /d/zap
for g in verify_b4_source_to_vm_traits_17 verify_b4_source_to_vm_traits_mro_18 verify_b4_source_to_vm_c3_mro_16 verify_b4_source_to_vm_constructors_inheritance_14 verify_b4_source_to_vm_super_lookup_14 verify_b4_source_to_vm_break_payload_10; do
  echo "=== $g ==="
  bash "scripts/bootstrap/$g.sh" 2>&1 | tail -5
done
