#!/usr/bin/env python3
"""Verify C backend produces byte-identical binaries across independent builds."""
import hashlib
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "host" / "zap-bootstrap"))
from c_backend import build_native_binary_from_file  # noqa: E402


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def main():
    source = ROOT / "bootstrap" / "fixtures" / "b4" / "c_backend_seed.zp"
    
    # Build twice independently
    print("Building first binary...")
    result1 = build_native_binary_from_file(str(source), str(ROOT / "target" / "repro-test-1"))
    c1 = Path(result1["c_path"])
    exe1 = Path(result1["exe_path"])
    
    print("Building second binary...")
    result2 = build_native_binary_from_file(str(source), str(ROOT / "target" / "repro-test-2"))
    c2 = Path(result2["c_path"])
    exe2 = Path(result2["exe_path"])
    
    # Verify artifacts exist
    if not (c1.is_file() and exe1.is_file() and c2.is_file() and exe2.is_file()):
        print("C backend did not produce expected artifacts")
        sys.exit(1)
    
    # Compare C source
    c1_bytes = c1.read_bytes()
    c2_bytes = c2.read_bytes()
    if c1_bytes != c2_bytes:
        print("C source differs between builds")
        print(f"Build 1 C SHA-256: {sha256(c1)}")
        print(f"Build 2 C SHA-256: {sha256(c2)}")
        sys.exit(1)
    print("C source is byte-identical")
    
    # Compare executables
    exe1_bytes = exe1.read_bytes()
    exe2_bytes = exe2.read_bytes()
    if exe1_bytes != exe2_bytes:
        print("Executables differ between builds")
        print(f"Build 1 EXE SHA-256: {sha256(exe1)}")
        print(f"Build 2 EXE SHA-256: {sha256(exe2)}")
        sys.exit(1)
    print("Executables are byte-identical")
    
    # Compare runtime output
    output1 = subprocess.run([str(exe1)], cwd=ROOT, capture_output=True, text=True)
    output2 = subprocess.run([str(exe2)], cwd=ROOT, capture_output=True, text=True)
    
    if output1.stdout != output2.stdout:
        print("Runtime output differs between builds")
        print(f"Build 1 output: {output1.stdout[:200]}")
        print(f"Build 2 output: {output2.stdout[:200]}")
        sys.exit(1)
    print("Runtime output is identical")
    
    final_hash = sha256(exe1)
    print(f"C backend reproducibility verification passed")
    print(f"Final binary SHA-256: {final_hash}")
    
    # Clean up test artifacts
    c1.unlink()
    exe1.unlink()
    c2.unlink()
    exe2.unlink()


if __name__ == "__main__":
    main()
