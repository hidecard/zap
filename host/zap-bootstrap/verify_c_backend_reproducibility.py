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
    prefix = ROOT / "target" / "repro-test"
    c_path = Path(str(prefix) + ".c")
    exe_path = prefix.with_suffix(".exe" if sys.platform == "win32" else "")

    try:
        print("Building first binary...")
        result1 = build_native_binary_from_file(str(source), str(prefix))
        c_path = Path(result1["c_path"])
        exe_path = Path(result1["exe_path"])
        if not (c_path.is_file() and exe_path.is_file()):
            raise SystemExit("C backend did not produce expected artifacts")
        c1_bytes = c_path.read_bytes()
        exe1_bytes = exe_path.read_bytes()
        output1 = subprocess.run(
            [str(exe_path)], cwd=ROOT, capture_output=True, text=True, check=False)
        if output1.returncode != 0:
            raise SystemExit(output1.stderr or output1.stdout or "first executable failed")

        print("Building second binary...")
        result2 = build_native_binary_from_file(str(source), str(prefix))
        c2 = Path(result2["c_path"])
        exe2 = Path(result2["exe_path"])
        if not (c2.is_file() and exe2.is_file()):
            raise SystemExit("C backend did not produce expected artifacts")
        c2_bytes = c2.read_bytes()
        exe2_bytes = exe2.read_bytes()

        if c1_bytes != c2_bytes:
            print("C source differs between builds")
            print(f"Build 1 C SHA-256: {hashlib.sha256(c1_bytes).hexdigest()}")
            print(f"Build 2 C SHA-256: {hashlib.sha256(c2_bytes).hexdigest()}")
            raise SystemExit(1)
        print("C source is byte-identical")

        if exe1_bytes != exe2_bytes:
            print("Executables differ between builds")
            print(f"Build 1 EXE SHA-256: {hashlib.sha256(exe1_bytes).hexdigest()}")
            print(f"Build 2 EXE SHA-256: {hashlib.sha256(exe2_bytes).hexdigest()}")
            raise SystemExit(1)
        print("Executables are byte-identical")

        output2 = subprocess.run(
            [str(exe2)], cwd=ROOT, capture_output=True, text=True, check=False)
        if output2.returncode != 0:
            raise SystemExit(output2.stderr or output2.stdout or "second executable failed")
        if output1.stdout != output2.stdout:
            print("Runtime output differs between builds")
            print(f"Build 1 output: {output1.stdout[:200]}")
            print(f"Build 2 output: {output2.stdout[:200]}")
            raise SystemExit(1)
        print("Runtime output is identical")

        final_hash = hashlib.sha256(exe2_bytes).hexdigest()
        print("C backend reproducibility verification passed")
        print(f"Final binary SHA-256: {final_hash}")
    finally:
        for artifact in (c_path, exe_path):
            if artifact.is_file():
                artifact.unlink()


if __name__ == "__main__":
    main()
