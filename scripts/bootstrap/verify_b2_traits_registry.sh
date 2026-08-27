#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
runner=$(mktemp "$ROOT_DIR/.zap-b2-traits-registry.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let satisfied = check("trait Printable:\n    fn format(self) -> text\nclass Report with Printable:\n    fn format(self) -> text:\n        return \"report\"", "satisfied.zp")
let missing = check("interface Identifiable:\n    fn id(self) -> text\nclass User implements Identifiable:\n    fn name(self):\n        return \"user\"", "missing.zp")
let conflict = check("trait JsonView:\n    fn render(self) -> text:\n        return \"json\"\ntrait TableView:\n    fn render(self) -> text:\n        return \"table\"\nclass Report with JsonView, TableView:\n    fn name(self):\n        return \"report\"", "conflict.zp")
let unknown = check("class Report with Missing:\n    fn name(self):\n        return \"report\"", "unknown.zp")
say satisfied
say missing
say conflict
say unknown
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" >"$out"
grep -q '"ok":true' "$out"
grep -q 'ZAP-TRAIT-001' "$out"
grep -q 'missing required method' "$out"
grep -q 'ZAP-TRAIT-002' "$out"
grep -q 'conflicting provided method' "$out"
grep -q 'ZAP-TRAIT-003' "$out"
grep -q "unknown trait 'Missing'" "$out"
printf 'B2 trait registry gate passed: conformance, missing requirements, provided conflicts, and unknown targets\n'
