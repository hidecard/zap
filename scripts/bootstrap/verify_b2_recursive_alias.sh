#!/usr/bin/env bash
# Verify B2 type alias cycle / recursive expansion detection.
# Confirms that type aliases which form a cycle (A = B; B = A) or self-recurse
# (Node = option<map<text, Node>>) emit a clear ZAP-TYPE-011 diagnostic and
# that the existing alias-of-alias case continues to typecheck.
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

ZAP_BIN="$ROOT_DIR/native/target/release/zap.exe"
if [ ! -x "$ZAP_BIN" ]; then
  printf 'missing zap binary: %s\n' "$ZAP_BIN" >&2
  exit 2
fi

runner=$(mktemp "$ROOT_DIR/.zap-b2-recursive-alias.XXXXXX.zp")
runner_rel=$(basename "$runner")
output=$(mktemp "${TMPDIR:-/tmp}/zap-b2-recursive-alias.XXXXXX")
trap 'rm -f "$runner" "$output"' EXIT

cat > "$runner" <<'ZP'
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typecheck.zp"

# Case 1: direct mutual cycle A = B; B = A. Must fail and emit ZAP-TYPE-011.
let cycle = from_json(check("type A = B" + "\n" + "type B = A" + "\n" + "let value: A = none", "cycle.zp"))
say "case:cycle"
say cycle["ok"]
say len(cycle["diagnostics"])
let has_cycle_code = false
let i = 0
let done = false
while not done:
    if i >= len(cycle["diagnostics"]):
        done = true
    else:
        if cycle["diagnostics"][i]["code"] == "ZAP-TYPE-011":
            has_cycle_code = true
        i = i + 1
say has_cycle_code

# Case 2: self-recursive Node = option<map<text, Node>>. Must fail with ZAP-TYPE-011.
let recursive = from_json(check("type Node = option<map<text, Node>>" + "\n" + "let tree: Node = none", "recursive.zp"))
say "case:recursive"
say recursive["ok"]
say len(recursive["diagnostics"])
let has_recursive_code = false
let j = 0
let jdone = false
while not jdone:
    if j >= len(recursive["diagnostics"]):
        jdone = true
    else:
        if recursive["diagnostics"][j]["code"] == "ZAP-TYPE-011":
            has_recursive_code = true
        j = j + 1
say has_recursive_code

# Case 3: alias-of-alias must still typecheck (regression guard).
let of_alias = from_json(check("type Inner = list<number>" + "\n" + "type Outer = Inner" + "\n" + "let values: Outer = [1, 2, 3]", "of-alias.zp"))
say "case:of-alias"
say of_alias["ok"]
say len(of_alias["diagnostics"])

# Case 4: generic alias with type parameter that references itself via container must not be a cycle.
let generic = from_json(check("type Box<T> = option<list<T>>" + "\n" + "let value: Box<number> = some([1, 2])", "generic.zp"))
say "case:generic"
say generic["ok"]
say len(generic["diagnostics"])
ZP

"$ZAP_BIN" "$runner_rel" > "$output"

# Case 1: A <-> B cycle produces ZAP-TYPE-011.
grep -q "^case:cycle$" "$output" || { printf 'FAIL: cycle case header missing\n' >&2; cat "$output" >&2; exit 1; }
sed -n '/^case:cycle$/,/^case:recursive$/p' "$output" | grep -q "^false$" || { printf 'FAIL: cycle case did not report ok=false\n' >&2; exit 1; }
sed -n '/^case:cycle$/,/^case:recursive$/p' "$output" | grep -q "^true$" || { printf 'FAIL: cycle case did not emit ZAP-TYPE-011\n' >&2; exit 1; }

# Case 2: self-recursive Node produces ZAP-TYPE-011.
grep -q "^case:recursive$" "$output" || { printf 'FAIL: recursive case header missing\n' >&2; cat "$output" >&2; exit 1; }
sed -n '/^case:recursive$/,/^case:of-alias$/p' "$output" | grep -q "^false$" || { printf 'FAIL: recursive case did not report ok=false\n' >&2; exit 1; }
sed -n '/^case:recursive$/,/^case:of-alias$/p' "$output" | grep -q "^true$" || { printf 'FAIL: recursive case did not emit ZAP-TYPE-011\n' >&2; exit 1; }

# Case 3: alias-of-alias still typechecks.
grep -q "^case:of-alias$" "$output" || { printf 'FAIL: of-alias case header missing\n' >&2; cat "$output" >&2; exit 1; }
sed -n '/^case:of-alias$/,/^case:generic$/p' "$output" | grep -q "^true$" || { printf 'FAIL: alias-of-alias regressed\n' >&2; exit 1; }
sed -n '/^case:of-alias$/,/^case:generic$/p' "$output" | grep -q "^0$" || { printf 'FAIL: alias-of-alias should emit 0 diagnostics\n' >&2; exit 1; }

# Case 4: generic alias with self-referential T is not a cycle.
grep -q "^case:generic$" "$output" || { printf 'FAIL: generic case header missing\n' >&2; cat "$output" >&2; exit 1; }
sed -n '/^case:generic$/,$p' "$output" | grep -q "^true$" || { printf 'FAIL: generic alias regressed\n' >&2; exit 1; }
sed -n '/^case:generic$/,$p' "$output" | grep -q "^0$" || { printf 'FAIL: generic alias should emit 0 diagnostics\n' >&2; exit 1; }

printf 'B2 recursive alias gate passed: mutual cycle and self-recursive aliases emit ZAP-TYPE-011; alias-of-alias and generic aliases still typecheck\n'
