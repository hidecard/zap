#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
for path in bootstrap/b2/typed_ir.zp bootstrap/fixtures/typecheck/annotated.zp bootstrap/fixtures/typecheck/annotated.typed-ir.json bootstrap/fixtures/typecheck/two_declarations.zp bootstrap/fixtures/typecheck/two_declarations.typed-ir.json bootstrap/fixtures/typecheck/generic_identity.zp; do
  [[ -f "$path" ]] || { printf 'missing B2 typed-IR candidate fixture: %s\n' "$path" >&2; exit 2; }
done
runner=$(mktemp "$ROOT_DIR/.zap-b2-typed-ir-candidate-runner.XXXXXX.zp")
first=$(mktemp "${TMPDIR:-/tmp}/zap-b2-typed-ir-candidate-first.XXXXXX")
second=$(mktemp "${TMPDIR:-/tmp}/zap-b2-typed-ir-candidate-second.XXXXXX")
reference=$(mktemp "${TMPDIR:-/tmp}/zap-b2-typed-ir-reference.XXXXXX")
generic_reference=$(mktemp "${TMPDIR:-/tmp}/zap-b2-typed-ir-generic-reference.XXXXXX")
two_reference=$(mktemp "${TMPDIR:-/tmp}/zap-b2-typed-ir-two-reference.XXXXXX")
trap 'rm -f "$runner" "$first" "$second" "$reference" "$generic_reference" "$two_reference"' EXIT
cat > "$runner" <<'EOF_RUNNER'
import "bootstrap/b2/typed_ir.zp"
let source = read_text("bootstrap/fixtures/typecheck/annotated.zp")
let generic_source = read_text("bootstrap/fixtures/typecheck/generic_identity.zp")
let two_source = read_text("bootstrap/fixtures/typecheck/two_declarations.zp")
say emit(source, "bootstrap/fixtures/typecheck/annotated.zp")
say emit(generic_source, "bootstrap/fixtures/typecheck/generic_identity.zp")
say emit(two_source, "bootstrap/fixtures/typecheck/two_declarations.zp")
EOF_RUNNER
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$first"
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$second"
cmp "$first" "$second"
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- bootstrap typed-ir bootstrap/fixtures/typecheck/annotated.zp > "$reference"
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- bootstrap typed-ir bootstrap/fixtures/typecheck/generic_identity.zp > "$generic_reference"
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- bootstrap typed-ir bootstrap/fixtures/typecheck/two_declarations.zp > "$two_reference"
jq -e 'select(.source_name == "bootstrap/fixtures/typecheck/annotated.zp") | .candidate_only == true and .kind == "zap.typed_ir" and .schema_version == 1 and .ir.nodes[0].annotation == "number" and .ir.nodes[0].inferred_type == "number" and .ir.nodes[0].name == "value" and .ir.nodes[0].value.value == 1' "$first" >/dev/null
jq --slurpfile reference "$reference" -e '.[0].ir.nodes[0].annotation == $reference[0].ir.nodes[0].annotation and .[0].ir.nodes[0].inferred_type == $reference[0].ir.nodes[0].inferred_type and .[0].ir.nodes[0].name == $reference[0].ir.nodes[0].name and .[0].ir.nodes[0].value == $reference[0].ir.nodes[0].value' <(jq -s '.' "$first") >/dev/null
jq -e 'select(.source_name == "bootstrap/fixtures/typecheck/generic_identity.zp") | .ir.nodes[0].type_params == ["T"] and .ir.nodes[1].inferred_type == "number" and .ir.nodes[2].inferred_type == "text"' "$first" >/dev/null
jq -e 'select(.source_name == "bootstrap/fixtures/typecheck/two_declarations.zp") | .ir.nodes | length == 2 and .[0].name == "first" and .[0].inferred_type == "number" and .[1].name == "label" and .[1].inferred_type == "text"' "$first" >/dev/null
jq --slurpfile two_reference "$two_reference" -e 'select(.source_name == "bootstrap/fixtures/typecheck/two_declarations.zp") | .ir == $two_reference[0].ir' "$first" >/dev/null
jq --slurpfile generic_reference "$generic_reference" -e 'select(.source_name == "bootstrap/fixtures/typecheck/generic_identity.zp") | .ir == $generic_reference[0].ir' "$first" >/dev/null
printf 'B2 Zap typed-IR candidate differential semantics passed: annotated declaration and bounded generic identity metadata\n'
