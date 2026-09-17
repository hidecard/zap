#!/usr/bin/env python3
"""Verify C backend binary runs in clean environment without Rust/Cargo variables."""
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "host" / "zap-bootstrap"))
from c_backend import build_native_binary_from_file  # noqa: E402


def main():
    # Build test binary
    source = ROOT / "bootstrap" / "fixtures" / "b4" / "c_backend_full_surface.zp"
    result = build_native_binary_from_file(str(source), str(ROOT / "target" / "test-zap-seed"))
    exe = Path(result["exe_path"])
    
    if not exe.is_file():
        print(f"C backend did not produce executable: {exe}")
        sys.exit(1)
    
    # Test normal execution
    normal = subprocess.run([str(exe)], cwd=ROOT, capture_output=True, text=True)
    if normal.returncode != 0:
        print(f"Normal execution failed: {normal.stderr}")
        sys.exit(1)
    print("Normal execution passed")
    
    # Test clean environment (Rust/Cargo variables removed)
    clean_env = os.environ.copy()
    for key in list(clean_env):
        if key.upper() in {"CARGO", "CARGO_HOME", "RUSTC", "RUSTUP_HOME", "RUSTUP_TOOLCHAIN"}:
            del clean_env[key]
    
    clean = subprocess.run([str(exe)], cwd=ROOT, env=clean_env, capture_output=True, text=True)
    if clean.returncode != 0:
        print(f"Clean environment execution failed: {clean.stderr}")
        sys.exit(1)
    print("Clean environment execution passed")
    
    # Verify output matches
    if normal.stdout != clean.stdout:
        print("Output mismatch between normal and clean environment")
        print(f"Normal: {normal.stdout[:200]}")
        print(f"Clean: {clean.stdout[:200]}")
        sys.exit(1)
    print("Output matches: clean environment produces identical results")
    
    print("C backend clean environment verification passed")


if __name__ == "__main__":
    main()
