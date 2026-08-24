#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT_DIR"

ready_output=$(ZAP_BIN=/bin/echo bash scripts/doctor.sh)
grep -Fq 'Environment ready: all doctor checks passed.' <<<"$ready_output"
grep -Fq 'runtime-binary     available (/bin/echo)' <<<"$ready_output"

incomplete_output=$(ZAP_BIN=/definitely/missing/zap bash scripts/doctor.sh)
grep -Fq 'Environment incomplete:' <<<"$incomplete_output"
grep -Fq 'No tests were run.' <<<"$incomplete_output"

set +e
ZAP_BIN=/definitely/missing/zap bash scripts/doctor.sh --strict >/tmp/zap-doctor-strict-output 2>&1
strict_status=$?
set -e
[[ "$strict_status" -eq 1 ]]
grep -Fq 'Environment incomplete:' /tmp/zap-doctor-strict-output
rm -f /tmp/zap-doctor-strict-output

printf 'doctor regression harness passed\n'
