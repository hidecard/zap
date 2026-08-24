#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
for path in bootstrap/b2/typed_ir.zp bootstrap/fixtures/typecheck/annotated.zp bootstrap/fixtures/typecheck/annotated.typed-ir.json; do
  [[ -f "$path" ]] || { printf 'missing B2 typed-IR candidate fixture: %s\n' "$path" >&2; exit 2; }
done
runner=$(mktemp "$ROOT_DIR/.zap-b2-typed-ir-candidate-runner.XXXXXX.zp")
first=$(mktemp "${TMPDIR:-/tmp}/zap-b2-typed-ir-candidate-first.XXXXXX")
second=$(mktemp "${TMPDIR:-/tmp}/zap-b2-typed-ir-candidate-second.XXXXXX")
reference=$(mktemp "${TMPDIR:-/tmp}/zap-b2-typed-ir-reference.XXXXXX")
trap 'rm -f "$runner" "$first" "$second" "$reference"' EXIT
cat > "$runner" <<'EOF_RUNNER'
import "bootstrap/b2/typed_ir.zp"
let source = read_text("bootstrap/fixtures/typecheck/annotated.zp")
say emit(source, "bootstrap/fixtures/typecheck/annotated.zp")
EOF_RUNNER
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$first"
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$second"
cmp "$first" "$second"
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- bootstrap typed-ir bootstrap/fixtures/typecheck/annotated.zp > "$reference"
jq -e '.candidate_only == true and .kind == "zap.typed_ir" and .schema_version == 1 and .source_name == "bootstrap/fixtures/typecheck/annotated.zp" and .ir.nodes[0].annotation == "number" and .ir.nodes[0].inferred_type == "number" and .ir.nodes[0].name == "value" and .ir.nodes[0].value.value == 1' "$first" >/dev/null
jq --slurpfile reference "$reference" -e '.ir.nodes[0].annotation == $reference[0].ir.nodes[0].annotation and .ir.nodes[0].inferred_type == $reference[0].ir.nodes[0].inferred_type and .ir.nodes[0].name == $reference[0].ir.nodes[0].name and .ir.nodes[0].value == $reference[0].ir.nodes[0].value' "$first" >/dev/null
printf 'B2 Zap typed-IR candidate differential semantics passed: annotated declaration\n'
