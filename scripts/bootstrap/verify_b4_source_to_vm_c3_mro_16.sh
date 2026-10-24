#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"

run_zap() {
  if [[ -x "$ROOT_DIR/bin/zap.exe" ]]; then
    "$ROOT_DIR/bin/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap.exe" ]]; then
    "$ROOT_DIR/native/target/release/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap.exe" ]]; then
    "$ROOT_DIR/native/target/debug/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/bin/zap" ]]; then
    "$ROOT_DIR/bin/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
    "$ROOT_DIR/native/target/release/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap" ]]; then
    "$ROOT_DIR/native/target/debug/zap" "$@"
  else
    cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$@"
  fi
}
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true

ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
TMPDIR_GATE=$(mktemp -d "$ROOT_DIR/.zap-c3-gate.XXXXXX")

run_case() {
  local runner_rel="$1"
  local out="$2"
  if [ -x "$ZAP_BIN" ]; then
    (cd "$ROOT_DIR" && "$ZAP_BIN" "$runner_rel") > "$out" 2>&1
  else
    (cd "$ROOT_DIR" && run_zap "$runner_rel") > "$out" 2>&1
  fi
}

# Each case runs in a fresh process to keep memory bounded
cat > "$TMPDIR_GATE/case_diamond.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let diamond = driver_seed_compile_source("class Root:\n    fn value(self):\n        return 1\nclass Left extends Root:\n    fn value(self):\n        return 2\nclass Right extends Root:\n    fn value(self):\n        return 3\nclass Child extends Left, Right:\n    fn marker(self):\n        return 0\nlet child = Child()\nsay child.value()", "diamond.zp")
let r = vm_run(diamond["instructions"])
say diamond["status"]
say r["error"]
say r["output"][0]
EOF
run_case "case_diamond.zp" "$TMPDIR_GATE/diamond.out"

cat > "$TMPDIR_GATE/case_reverse.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let reverse = driver_seed_compile_source("class Root:\n    fn value(self):\n        return 1\nclass Left extends Root:\n    fn value(self):\n        return 2\nclass Right extends Root:\n    fn value(self):\n        return 3\nclass Child extends Right, Left:\n    fn marker(self):\n        return 0\nlet child = Child()\nsay child.value()", "reverse.zp")
let r = vm_run(reverse["instructions"])
say r["output"][0]
EOF
run_case "case_reverse.zp" "$TMPDIR_GATE/reverse.out"

cat > "$TMPDIR_GATE/case_super.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let super_chain = driver_seed_compile_source("class Root:\n    fn value(self):\n        return 1\nclass Left extends Root:\n    fn value(self):\n        return super().value() + 10\nclass Right extends Root:\n    fn value(self):\n        return 3\nclass Child extends Left, Right:\n    fn marker(self):\n        return 0\nlet child = Child()\nsay child.value()", "super-chain.zp")
let r = vm_run(super_chain["instructions"])
say r["output"][0]
EOF
run_case "case_super.zp" "$TMPDIR_GATE/super.out"

cat > "$TMPDIR_GATE/case_ctor.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let constructors = driver_seed_compile_source("class Root:\n    fn __init__(self):\n        set self.value = 1\n        return self\nclass Left extends Root:\n    fn __init__(self):\n        super().__init__()\n        set self.value = 2\n        return self\nclass Right extends Root:\n    fn __init__(self):\n        super().__init__()\n        set self.value = 3\n        return self\nclass Child extends Left, Right:\n    fn marker(self):\n        return 0\nlet child = Child()\nsay child.value", "constructors.zp")
let r = vm_run(constructors["instructions"])
say r["output"][0]
EOF
run_case "case_ctor.zp" "$TMPDIR_GATE/ctor.out"

cat > "$TMPDIR_GATE/case_conflict.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let conflict = driver_seed_compile_source("class A:\n    fn marker(self):\n        return 0\nclass B:\n    fn marker(self):\n        return 0\nclass C extends A, B:\n    fn marker(self):\n        return 0\nclass D extends B, A:\n    fn marker(self):\n        return 0\nclass E extends C, D:\n    fn marker(self):\n        return 0\nlet value = E()", "conflict.zp")
let r = vm_run(conflict["instructions"])
say r["error"]
EOF
run_case "case_conflict.zp" "$TMPDIR_GATE/conflict.out"

cat > "$TMPDIR_GATE/case_unknown.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let unknown = driver_seed_compile_source("class Child extends Missing:\n    fn marker(self):\n        return 0\nlet value = Child()", "unknown.zp")
let r = vm_run(unknown["instructions"])
say r["error"]
EOF
run_case "case_unknown.zp" "$TMPDIR_GATE/unknown.out"

cat > "$TMPDIR_GATE/case_cycle.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let cycle = driver_seed_compile_source("class A extends B:\n    fn marker(self):\n        return 0\nclass B extends A:\n    fn marker(self):\n        return 0\nlet value = A()", "cycle.zp")
let r = vm_run(cycle["instructions"])
say r["error"]
EOF
run_case "case_cycle.zp" "$TMPDIR_GATE/cycle.out"

cat > "$TMPDIR_GATE/case_canonical.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let canonical = driver_seed_compile_ast_source("class Root:\n    fn value(self):\n        return 1\nclass Left extends Root:\n    fn value(self):\n        return 2\nclass Right extends Root:\n    fn value(self):\n        return 3\nclass Child extends Left, Right:\n    fn marker(self):\n        return 0\nlet child = Child()\nsay child.value()", "canonical.zp")
let r = vm_run(canonical["instructions"])
say canonical["status"]
say r["error"]
say r["output"][0]
EOF
run_case "case_canonical.zp" "$TMPDIR_GATE/canonical.out"

out=$(mktemp)
{
  grep -v '^$' "$TMPDIR_GATE/diamond.out"
  grep -v '^$' "$TMPDIR_GATE/reverse.out"
  grep -v '^$' "$TMPDIR_GATE/super.out"
  grep -v '^$' "$TMPDIR_GATE/ctor.out"
  grep -v '^$' "$TMPDIR_GATE/conflict.out"
  grep -v '^$' "$TMPDIR_GATE/unknown.out"
  grep -v '^$' "$TMPDIR_GATE/cycle.out"
  grep -v '^$' "$TMPDIR_GATE/canonical.out"
} > "$out"

expected=$(mktemp)
cat > "$expected" <<'EOF'
compiled_slice
none
2
3
13
2
inconsistent_mro:E
unknown_class:Missing
inheritance_cycle:A
compiled_ast_slice
none
2
EOF

cmp "$out" "$expected" || { echo '--- actual ---'; cat "$out"; echo '--- expected ---'; cat "$expected"; rm -rf "$TMPDIR_GATE" "$out" "$expected"; exit 1; }
rm -rf "$TMPDIR_GATE" "$out" "$expected"
printf 'B4 C3 MRO gate passed: deterministic diamond precedence, MRO-based super continuation, inherited cooperative constructors, canonical AST multiple parents, and stable hierarchy diagnostics\n'
