#!/usr/bin/env bash
set -euo pipefail

INPUT=${1:-}
OUTPUT=${2:-}

if [[ -z "$INPUT" || -z "$OUTPUT" ]]; then
  printf 'usage: aggregate_benchmark.sh INPUT.csv OUTPUT.csv\n' >&2
  exit 2
fi

[[ -f "$INPUT" ]] || { printf 'benchmark input does not exist: %s\n' "$INPUT" >&2; exit 1; }
mkdir -p "$(dirname "$OUTPUT")"

tmp=$(mktemp)
trap 'rm -f "$tmp"' EXIT

awk -F, '
  BEGIN { OFS="," }
  NR == 1 {
    if ($0 != "suite,iteration,elapsed_seconds") { print "invalid benchmark header" > "/dev/stderr"; exit 2 }
    next
  }
  NF != 3 { print "invalid benchmark row at line " NR > "/dev/stderr"; exit 2 }
  {
    suite=$1; iteration=$2; elapsed=$3
    if (suite !~ /^[A-Za-z0-9_-]+$/ || iteration !~ /^[1-9][0-9]*$/ || elapsed !~ /^[0-9]+([.][0-9]+)?$/) {
      print "invalid benchmark value at line " NR > "/dev/stderr"; exit 2
    }
    count[suite]++
    sum[suite]+=elapsed
    sumsq[suite]+=elapsed * elapsed
    values[suite, count[suite]]=elapsed
    if (!(suite in min) || elapsed < min[suite]) min[suite]=elapsed
    if (!(suite in max) || elapsed > max[suite]) max[suite]=elapsed
  }
  END {
    if (NR < 2) { print "benchmark input has no data rows" > "/dev/stderr"; exit 2 }
    for (suite in count) names[++n]=suite
    for (i=1; i<=n; i++) for (j=i+1; j<=n; j++) if (names[j] < names[i]) { t=names[i]; names[i]=names[j]; names[j]=t }
    print "suite,iterations,min_seconds,mean_seconds,p95_seconds,max_seconds,stddev_seconds,variance_seconds,cv_percent"
    for (i=1; i<=n; i++) {
      suite=names[i]
      for (a=1; a<=count[suite]; a++) for (b=a+1; b<=count[suite]; b++) {
        if (values[suite,b] < values[suite,a]) { t=values[suite,a]; values[suite,a]=values[suite,b]; values[suite,b]=t }
      }
      rank=int(count[suite] * 0.95)
      if (rank < count[suite] * 0.95) rank++
      if (rank < 1) rank=1
      mean=sum[suite]/count[suite]
      variance=(sumsq[suite]/count[suite])-(mean*mean)
      if (variance < 0 && variance > -0.000000000001) variance=0
      if (variance < 0) { print "negative benchmark variance for " suite > "/dev/stderr"; exit 2 }
      stddev=sqrt(variance)
      cv=mean > 0 ? (stddev/mean)*100 : 0
      printf "%s,%d,%.6f,%.6f,%.6f,%.6f,%.6f,%.6f,%.3f\n", suite,count[suite],min[suite],mean,values[suite,rank],max[suite],stddev,variance,cv
    }
  }
' "$INPUT" > "$tmp"

mv "$tmp" "$OUTPUT"
printf 'wrote %s\n' "$OUTPUT" >&2
