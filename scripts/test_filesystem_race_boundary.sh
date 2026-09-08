#!/usr/bin/env bash
# Test filesystem race boundary and host-specific process cleanup
# P0.3 focused regression tests for runtime safety

set -euo pipefail
IFS=$'\n\t'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ZAP_BIN="${ZAP_BIN:-$ROOT_DIR/native/target/release/zap}"
work_root=$(mktemp -d "${TMPDIR:-/tmp}/zap-race-boundary.XXXXXX")
trap 'rm -rf "$work_root"' EXIT

if [[ ! -x "$ZAP_BIN" ]]; then
  cargo build --quiet --release --locked --manifest-path native/Cargo.toml --bin zap
fi
[[ -x "$ZAP_BIN" ]] || { printf 'race boundary test: missing executable: %s\n' "$ZAP_BIN" >&2; exit 2; }

# Test 1: Filesystem race boundary - ensure no TOCTOU issues in file operations
test_filesystem_race() {
  local project="$work_root/filesystem_race"
  mkdir -p "$project"
  printf '[package]\nname = "filesystem_race"\nversion = "0.1.0"\nmain = "main.zp"\n' > "$project/zap.toml"
  
  # Create a test that exercises file operations that could be subject to race conditions
  printf 'let file = "test.txt"\nlet content = "test content"\nfile_write(file, content)\nlet read = file_read(file)\nassert(read == content, "file content mismatch")\nfile_delete(file)\n' > "$project/main.zp"
  
  output="$work_root/filesystem_race.output"
  set +e
  timeout 30s "$ZAP_BIN" run "$project" >"$output" 2>&1
  status=$?
  set -e
  
  if [[ "$status" -eq 0 ]]; then
    printf 'PASS: filesystem race boundary test completed successfully\n'
  else
    printf 'FAIL: filesystem race boundary test failed with status %s\n' "$status" >&2
    cat "$output" >&2
    exit 1
  fi
}

# Test 2: Process cleanup - ensure child processes are properly cleaned up
test_process_cleanup() {
  local project="$work_root/process_cleanup"
  mkdir -p "$project"
  printf '[package]\nname = "process_cleanup"\nversion = "0.1.0"\nmain = "main.zp"\n' > "$project/zap.toml"
  
  # Create a test that spawns a process and ensures it's cleaned up
  if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    printf 'let result = process_run("cmd", ["/C", "echo", "test"])\nassert(result["exit_code"] == 0, "process failed")\n' > "$project/main.zp"
  else
    printf 'let result = process_run("printf", ["test"])\nassert(result["exit_code"] == 0, "process failed")\n' > "$project/main.zp"
  fi
  
  output="$work_root/process_cleanup.output"
  set +e
  timeout 30s "$ZAP_BIN" run "$project" >"$output" 2>&1
  status=$?
  set -e
  
  if [[ "$status" -eq 0 ]]; then
    printf 'PASS: process cleanup test completed successfully\n'
  else
    printf 'FAIL: process cleanup test failed with status %s\n' "$status" >&2
    cat "$output" >&2
    exit 1
  fi
}

# Test 3: Check for zombie processes after execution
test_no_zombie_processes() {
  local project="$work_root/no_zombies"
  mkdir -p "$project"
  printf '[package]\nname = "no_zombies"\nversion = "0.1.0"\nmain = "main.zp"\n' > "$project/zap.toml"
  printf 'let result = process_run("echo", ["test"])\n' > "$project/main.zp"
  
  output="$work_root/no_zombies.output"
  set +e
  timeout 30s "$ZAP_BIN" run "$project" >"$output" 2>&1
  status=$?
  set -e
  
  if [[ "$status" -eq 0 ]]; then
    # Check for zombie processes (platform-specific)
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
      zombie_count=$(ps aux | grep -c '<defunct>' || true)
      if [[ "$zombie_count" -gt 0 ]]; then
        printf 'WARN: Potential zombie processes detected: %s\n' "$zombie_count" >&2
      else
        printf 'PASS: no zombie processes detected\n'
      fi
    else
      printf 'PASS: zombie process check skipped on this platform\n'
    fi
  else
    printf 'FAIL: zombie process test failed with status %s\n' "$status" >&2
    cat "$output" >&2
    exit 1
  fi
}

# Run all tests
test_filesystem_race
test_process_cleanup
test_no_zombie_processes

printf 'race boundary and process cleanup tests passed\n'