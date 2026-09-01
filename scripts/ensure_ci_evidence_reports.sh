#!/usr/bin/env bash
set -euo pipefail
mkdir -p target benchmark-results

write_placeholder() {
  local path="$1" header="$2"
  if [[ ! -s "$path" ]]; then
    printf '%s\n' "$header" > "$path"
  fi
}

write_placeholder target/version-consistency.tsv $'source\texpected\tobserved\tstatus'
write_placeholder target/documentation-consistency.tsv $'source\tcheck\tstatus'
write_placeholder target/b2-milestone-report.tsv $'gate\tstatus\tdetails'
write_placeholder target/b3-unified-evidence.tsv $'gate\tcategory\tstatus\tduration_seconds\tlog'
write_placeholder target/framework-starters.tsv $'starter\tstatus\tdetails'
write_placeholder target/p105-replay.log 'fallback P1-05 replay evidence initialized before validation'
write_placeholder target/m2-verify-replay.tsv $'case\tstatus\tdigest'
write_placeholder target/m2-verify-replay.log 'fallback M2-VERIFY-01 replay evidence initialized before validation'
write_placeholder target/p001-parity-report.tsv $'fixture\tstatus\tdigest'
write_placeholder target/spec-ownership-report.tsv $'rule_id\towner\tstatus'
write_placeholder target/b3-unified-evidence-fallback.log 'fallback evidence initialized before validation'
printf 'CI evidence fallback paths initialized\n'
