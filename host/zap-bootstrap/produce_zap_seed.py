#!/usr/bin/env python3
"""Produce Zap-produced seed binary with metadata."""
import hashlib
import json
import os
import platform
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "host" / "zap-bootstrap"))
from c_backend import build_native_binary_from_file  # noqa: E402
from compile import compile_program  # noqa: E402


def get_git_commit():
    """Get current git commit SHA."""
    try:
        result = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=ROOT,
            capture_output=True,
            text=True,
            check=True
        )
        return result.stdout.strip()
    except (subprocess.CalledProcessError, FileNotFoundError):
        return "unknown"


def get_platform_triple():
    """Get platform triple for current system."""
    system = platform.system()
    machine = platform.machine()
    
    if system == "Linux":
        return f"x86_64-unknown-linux-gnu"
    elif system == "Windows":
        return "x86_64-pc-windows-msvc"
    elif system == "Darwin":
        if machine == "arm64":
            return "aarch64-apple-darwin"
        return "x86_64-apple-darwin"
    return f"{system.lower()}-{machine.lower()}"


def sha256(path):
    """Calculate SHA-256 hash of a file."""
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def bytecode_sha256(path):
    source = path.read_text(encoding="utf-8-sig")
    program = compile_program(source)
    payload = json.dumps(program, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
    return hashlib.sha256(payload).hexdigest()


def produce_seed(source_path, output_dir):
    """Produce Zap-produced seed binary with metadata."""
    source = ROOT / source_path
    output_dir = ROOT / output_dir
    output_dir.mkdir(parents=True, exist_ok=True)
    
    platform_triple = get_platform_triple()
    artifact_name = "zap.exe" if platform.system() == "Windows" else "zap"
    # C backend adds .exe automatically on Windows, so we use base name
    base_name = "zap" if platform.system() == "Windows" else artifact_name
    output_path = output_dir / base_name
    
    print(f"Building Zap-produced seed for {platform_triple}...")
    result = build_native_binary_from_file(str(source), str(output_path))
    
    c_path = Path(result["c_path"])
    exe_path = Path(result["exe_path"])
    
    if not exe_path.is_file():
        print(f"Failed to produce executable: {exe_path}")
        sys.exit(1)
    
    # Calculate hashes
    c_hash = sha256(c_path)
    exe_hash = sha256(exe_path)
    bytecode_hash = bytecode_sha256(source)
    
    # Generate metadata
    commit = get_git_commit()
    timestamp = datetime.now(timezone.utc).isoformat()
    
    metadata = {
        "commit": commit,
        "platform": platform_triple,
        "artifact": artifact_name,
        "built_with": "python3-c-backend",
        "rust_free_provenance": "true",
        "certification_ready": "false",
        "status": "zap-produced-seed",
        "bytecode_digest": bytecode_hash,
        "c_source_digest": c_hash,
        "native_digest": exe_hash,
        "timestamp": timestamp
    }
    
    # Write SEED.tsv
    seed_tsv = output_dir / "SEED.tsv"
    with seed_tsv.open("w", encoding="utf-8") as f:
        f.write(f"commit\t{commit}\n")
        f.write(f"platform\t{platform_triple}\n")
        f.write(f"artifact\t{artifact_name}\n")
        f.write(f"built_with\tpython3-c-backend\n")
        f.write(f"rust_free_provenance\ttrue\n")
        f.write(f"certification_ready\tfalse\n")
        f.write(f"status\tzap-produced-seed\n")
        f.write(f"bytecode_digest\t{bytecode_hash}\n")
        f.write(f"c_source_digest\t{c_hash}\n")
        f.write(f"native_digest\t{exe_hash}\n")
        f.write(f"timestamp\t{timestamp}\n")
    
    # Write checksum file
    checksum_file = output_dir / f"{artifact_name}.sha256"
    with checksum_file.open("w", encoding="ascii") as f:
        f.write(f"{exe_hash}  {artifact_name}\n")
    
    print(f"Zap-produced seed generated successfully:")
    print(f"  Binary: {exe_path}")
    print(f"  Metadata: {seed_tsv}")
    print(f"  Checksum: {checksum_file}")
    print(f"  SHA-256: {exe_hash}")
    
    return exe_path, seed_tsv, checksum_file


def main():
    if len(sys.argv) > 1:
        source_path = sys.argv[1]
    else:
        source_path = "bootstrap/fixtures/b4/c_backend_seed.zp"
    
    if len(sys.argv) > 2:
        output_dir = sys.argv[2]
    else:
        output_dir = f"target/seeds/{get_platform_triple()}"
    
    produce_seed(source_path, output_dir)


if __name__ == "__main__":
    main()
