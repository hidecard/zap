#!/usr/bin/env python3
"""Verify that the Zap-owned C backend emits native programs that work.

The reference C backend in ``host/zap-bootstrap`` is exercised by
``verify_b4_c_backend_acceptance.py``. That gate never runs the C emitted by
``bootstrap/b4/c_backend.zp``, so a broken emission could pass unnoticed. This
verifier closes that gap: it asks the Zap-owned backend to emit C for canonical
bytecode, compiles the result with the system C compiler, runs the produced
executable, and compares the observed output with the expected output.
"""
import hashlib
import json
import os
import platform
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "host" / "zap-bootstrap"))
import c_backend  # noqa: E402


# Canonical bytecode programs expressed in the shared opcode contract.
# Each entry is (name, instructions, expected stdout lines).
FIXTURES = [
    (
        "arithmetic",
        [
            {"op": "const", "value": 2},
            {"op": "const", "value": 3},
            {"op": "add"},
            {"op": "print"},
            {"op": "halt"},
        ],
        ["5"],
    ),
    (
        "string",
        [
            {"op": "const", "value": "zap"},
            {"op": "print"},
            {"op": "halt"},
        ],
        ["zap"],
    ),
    (
        "list_length",
        [
            {"op": "const", "value": 1},
            {"op": "const", "value": 2},
            {"op": "const", "value": 3},
            {"op": "make_list", "count": 3},
            {"op": "dup"},
            {"op": "list_len"},
            {"op": "print"},
            {"op": "pop"},
            {"op": "halt"},
        ],
        ["3"],
    ),
    (
        "list_index",
        [
            {"op": "const", "value": 10},
            {"op": "const", "value": 20},
            {"op": "make_list", "count": 2},
            {"op": "const", "value": 0},
            {"op": "list_get"},
            {"op": "print"},
            {"op": "halt"},
        ],
        ["10"],
    ),
    (
        "list_append",
        [
            {"op": "const", "value": 1},
            {"op": "const", "value": 2},
            {"op": "make_list", "count": 2},
            {"op": "const", "value": 30},
            {"op": "list_append"},
            {"op": "dup"},
            {"op": "list_len"},
            {"op": "print"},
            {"op": "pop"},
            {"op": "halt"},
        ],
        ["3"],
    ),
    (
        "list_reverse",
        [
            {"op": "const", "value": 1},
            {"op": "const", "value": 2},
            {"op": "make_list", "count": 2},
            {"op": "list_reverse"},
            {"op": "list_len"},
            {"op": "print"},
            {"op": "halt"},
        ],
        ["2"],
    ),
    (
        "map_roundtrip",
        [
            {"op": "make_map"},
            {"op": "const", "value": 1},
            {"op": "const", "value": 10},
            {"op": "map_set_pair"},
            {"op": "dup"},
            {"op": "const", "value": 1},
            {"op": "map_get"},
            {"op": "print"},
            {"op": "halt"},
        ],
        ["10"],
    ),
    (
        "option_surface",
        [
            {"op": "option_none"},
            {"op": "dup"},
            {"op": "option_is_none"},
            {"op": "print"},
            {"op": "pop"},
            {"op": "const", "value": 7},
            {"op": "option_some"},
            {"op": "option_is_some"},
            {"op": "print"},
            {"op": "halt"},
        ],
        ["1", "1"],
    ),
    (
        "error_surface",
        [
            {"op": "const", "value": "bad"},
            {"op": "error_new"},
            {"op": "error_is_error"},
            {"op": "print"},
            {"op": "halt"},
        ],
        ["1"],
    ),
    (
        "control_flow",
        [
            {"op": "const", "value": False},
            {"op": "jump_if_false", "target": 4},
            {"op": "const", "value": 42},
            {"op": "print"},
            {"op": "const", "value": 7},
            {"op": "print"},
            {"op": "halt"},
        ],
        ["7"],
    ),
    (
        "async_surface",
        [
            {"op": "const", "value": 5},
            {"op": "async_new"},
            {"op": "await"},
            {"op": "print"},
            {"op": "halt"},
        ],
        ["5"],
    ),
    (
        "module_surface",
        [
            {"op": "const", "value": "mod"},
            {"op": "import_module"},
            {"op": "const", "value": "sym"},
            {"op": "import_symbol"},
            {"op": "print"},
            {"op": "export_value"},
            {"op": "halt"},
        ],
        ["mod"],
    ),
    (
        "function_call",
        [
            {"op": "function_def", "name": "add", "params": ["a", "b"], "entry": 1, "end": 5},
            {"op": "load", "name": "a"},
            {"op": "load", "name": "b"},
            {"op": "add"},
            {"op": "return_value"},
            {"op": "const", "value": 2},
            {"op": "const", "value": 3},
            {"op": "call", "name": "add", "argc": 2},
            {"op": "print"},
            {"op": "halt"},
        ],
        ["5"],
    ),
    (
        "closure_capture",
        [
            {"op": "const", "value": 10},
            {"op": "store", "name": "n"},
            {"op": "function_def", "name": "add", "binding": "f", "captures": ["n"], "params": ["x"], "entry": 4, "end": 8},
            {"op": "load", "name": "f"},
            {"op": "load", "name": "x"},
            {"op": "load", "name": "n"},
            {"op": "add"},
            {"op": "return_value"},
            {"op": "const", "value": 5},
            {"op": "call", "name": "f", "argc": 1},
            {"op": "print"},
            {"op": "halt"},
        ],
        ["15"],
    ),
]

STATUS_MARKER = "ZAP-C-BACKEND-STATUS:"


def sha256_bytes(payload):
    return hashlib.sha256(payload).hexdigest()


def zap_seed():
    for candidate in (
        os.environ.get("ZAP_BOOTSTRAP_BIN"),
        os.environ.get("ZAP_BIN"),
        str(ROOT / "bin" / "zap"),
        str(ROOT / "bin" / "zap.exe"),
        str(ROOT / "native" / "target" / "release" / "zap"),
        str(ROOT / "native" / "target" / "release" / "zap.exe"),
    ):
        if candidate and Path(candidate).is_file():
            return candidate
    raise AssertionError("a verified Zap seed is required; set ZAP_BOOTSTRAP_BIN")


def emit_c(seed, work_dir, instructions):
    """Ask the Zap-owned backend to emit C for the given canonical bytecode."""
    artifact = {
        "kind": "zap.bytecode",
        "schema_version": 1,
        "instructions": instructions,
    }
    artifact_path = ROOT / f".zap-c-backend-emit-{os.getpid()}.json"
    artifact_path.write_text(json.dumps(artifact), encoding="utf-8")
    runner_path = ROOT / f".zap-c-backend-emit-{os.getpid()}.zp"
    runner_path.write_text(
        'import "bootstrap/b4/c_backend.zp"\n'
        f'let artifact = from_json(read_text("{artifact_path.name}"))\n'
        "let generated = c_backend_emit(artifact[\"instructions\"])\n"
        f'say "{STATUS_MARKER}" + generated["status"]\n'
        "for line in generated[\"lines\"]:\n"
        "    say line\n",
        encoding="utf-8",
    )
    try:
        result = subprocess.run(
            [seed, str(runner_path)],
            cwd=str(ROOT),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            encoding="utf-8",
            errors="replace",
            check=False,
        )
    finally:
        runner_path.unlink(missing_ok=True)
        artifact_path.unlink(missing_ok=True)
    if result.returncode != 0:
        raise AssertionError(
            f"Zap-owned emission failed: {result.stderr or result.stdout}".strip())
    lines = result.stdout.replace("\r\n", "\n").replace("\r", "\n").splitlines()
    if not lines or not lines[0].startswith(STATUS_MARKER):
        raise AssertionError(f"missing emission status line: {lines[:1]}")
    status = lines[0][len(STATUS_MARKER):].strip()
    if status == "rejected":
        raise AssertionError("Zap-owned backend rejected canonical bytecode")
    return "\n".join(lines[1:]) + "\n"


def run_executable(exe_path):
    result = subprocess.run(
        [str(exe_path)],
        cwd=str(ROOT),
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )
    if result.returncode != 0:
        raise AssertionError(result.stderr or result.stdout or "executable exited nonzero")
    return result.stdout.replace("\r\n", "\n").replace("\r", "\n").splitlines()


def verify_fixture(seed, name, instructions, expected):
    with tempfile.TemporaryDirectory(prefix=f"zap-c-backend-{name}-") as temp:
        work_dir = Path(temp)
        c_source = emit_c(seed, work_dir, instructions)
        if "int main(" not in c_source:
            raise AssertionError("emitted C has no entrypoint")
        c_path = work_dir / f"{name}.c"
        c_path.write_text(c_source, encoding="utf-8")
        exe_path = work_dir / f"{name}.exe"
        c_backend.compile_c(str(c_path), str(exe_path))
        if not exe_path.is_file():
            raise AssertionError("C backend did not produce a native executable")
        actual = run_executable(exe_path)
        if actual != expected:
            raise AssertionError(f"expected {expected!r}, got {actual!r}")
        # The emitted C must be reproducible from a fresh Zap process.
        repeat_source = emit_c(seed, work_dir, instructions)
        if repeat_source != c_source:
            raise AssertionError("emitted C differs across fresh Zap processes")
        return {
            "c_sha256": sha256_bytes(c_source.encode("utf-8")),
            "exe_sha256": sha256_bytes(exe_path.read_bytes()),
            "stdout_sha256": sha256_bytes("\n".join(actual).encode("utf-8")),
            "platform": platform.system(),
        }


def main():
    report_path = Path(
        os.environ.get(
            "B4_ZAP_C_BACKEND_REPORT",
            ROOT / "target" / "b4-zap-c-backend-emitted-c.tsv"))
    report_path.parent.mkdir(parents=True, exist_ok=True)

    compiler = c_backend.find_c_compiler()
    if not compiler:
        print("SKIP: no system C compiler available; Zap-owned emission not verified")
        return 0

    seed = zap_seed()
    results = {}
    failures = []
    for name, instructions, expected in FIXTURES:
        try:
            results[name] = {"status": "pass", **verify_fixture(seed, name, instructions, expected)}
            print(f"PASS {name}")
        except Exception as exc:
            failures.append((name, str(exc)))
            results[name] = {"status": "fail", "error": str(exc), "platform": platform.system()}
            print(f"FAIL {name}: {exc}", file=sys.stderr)

    with report_path.open("w", encoding="utf-8", newline="") as report:
        report.write("schema_version\t1\n")
        report.write("contract_id\tB4-ZAP-OWNED-C-BACKEND\n")
        report.write("id\tstatus\tplatform\tc_sha256\texe_sha256\tstdout_sha256\terror\n")
        for name, _, _ in FIXTURES:
            row = results[name]
            report.write("\t".join([
                name,
                row["status"],
                row.get("platform", ""),
                row.get("c_sha256", ""),
                row.get("exe_sha256", ""),
                row.get("stdout_sha256", ""),
                row.get("error", ""),
            ]) + "\n")
        report.write(f"summary\t{len(FIXTURES) - len(failures)}\t{len(failures)}\n")

    if failures:
        raise SystemExit(1)
    print(
        f"B4 Zap-owned C backend passed: {len(FIXTURES)}/{len(FIXTURES)} "
        f"emitted programs compiled and ran correctly on {platform.system()}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
