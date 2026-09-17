#!/usr/bin/env python3
"""Run focused canonical B3 bytecode to C bridge parity checks."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "host" / "zap-bootstrap"))

from c_backend import (  # noqa: E402
    BytecodeValidationError,
    build_native_binary_from_bytecode,
    compile_c,
    emit_c,
    emit_c_from_bytecode,
    find_c_compiler,
    validate_canonical_bytecode,
)
from compile import compile_program  # noqa: E402


_PROGRAMS = (
    ("arithmetic", "let x = 2 + 3\nsay x\n", ["5"]),
    ("branch", "let x = 4\nif x > 2:\n    say x\nelse:\n    say 0\n", ["4"]),
    ("function", "fn add(a, b):\n    return a + b\nsay add(2, 8)\n", ["10"]),
    ("loop", "let values = [1, 2, 3]\nlet total = 0\nfor value in values:\n    total = total + value\nsay total\n", ["6"]),
    ("map", "let values = {}\nvalues[\"answer\"] = 42\nsay values[\"answer\"]\n", ["42"]),
)


def _canonical(program):
    return {
        "kind": "zap.bytecode",
        "schema_version": 1,
        "instructions": program,
    }


def _field(value):
    return json.dumps(value, ensure_ascii=False, sort_keys=True)


def _run_program(exe_path):
    return subprocess.run(
        [str(exe_path)], capture_output=True, text=True)


def _write_report(report_path, rows, summary):
    report_path.parent.mkdir(parents=True, exist_ok=True)
    with report_path.open("w", encoding="utf-8", newline="\n") as handle:
        handle.write("schema_version\t1\n")
        handle.write("contract_id\tB3-BYTECODE-C-BRIDGE\n")
        handle.write("contract_status\tnot-certified\n")
        handle.write("bridge_status\tprovisional\n")
        handle.write("program_id\tstatus\tc_parity\tnative_parity\texpected\tactual\terror\n")
        for row in rows:
            handle.write("\t".join(_field(value) for value in row) + "\n")
        handle.write("summary\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n" % tuple(
            _field(value) for value in summary))


def main():
    report_path = Path(os.environ.get(
        "B3_BYTECODE_C_BRIDGE_REPORT",
        ROOT / "target" / "b3-bytecode-c-bridge.tsv"))
    rows = []
    failures = []
    native_skips = 0

    compiler = find_c_compiler()
    if compiler is None:
        print("C compiler unavailable; C emission parity will run and native parity will be skipped")

    with tempfile.TemporaryDirectory(prefix="zap-b3-bytecode-c-bridge-") as tmp:
        tmpdir = Path(tmp)
        for program_id, source, expected in _PROGRAMS:
            program = compile_program(source)
            canonical = _canonical(program)
            reference_c = tmpdir / (program_id + "-reference.c")
            bridge_c = tmpdir / (program_id + "-bridge.c")
            emit_c(program, str(reference_c))
            emit_c_from_bytecode(canonical, str(bridge_c))
            c_parity = reference_c.read_bytes() == bridge_c.read_bytes()
            actual = ""
            error = ""
            native_parity = "not-run"
            if not c_parity:
                error = "emitted C differs from reference backend"
                failures.append(program_id + ": emitted C differs")
            elif compiler is None:
                native_parity = "skipped-no-compiler"
                native_skips += 1
            else:
                reference_exe = tmpdir / (program_id + "-reference")
                try:
                    compile_c(str(reference_c), str(reference_exe), compiler=compiler)
                    bridge = build_native_binary_from_bytecode(
                        canonical, str(tmpdir / (program_id + "-bridge")),
                        compiler=compiler)
                    reference_run = _run_program(reference_exe)
                    bridge_run = _run_program(bridge["exe_path"])
                    actual = bridge_run.stdout
                    native_parity = "pass" if (
                        reference_run.returncode == bridge_run.returncode
                        and reference_run.stdout == bridge_run.stdout
                        and reference_run.stderr == bridge_run.stderr
                        and [line for line in bridge_run.stdout.splitlines() if line] == expected
                    ) else "fail"
                    if native_parity == "fail":
                        error = "native output or status differs from reference"
                        failures.append(program_id + ": native parity failed")
                except Exception as exc:  # pragma: no cover - compiler-specific failure path
                    native_parity = "error"
                    error = str(exc)
                    failures.append(program_id + ": " + str(exc))
            status = "pass" if c_parity and (
                compiler is None or native_parity == "pass") else "fail"
            rows.append((program_id, status, "pass" if c_parity else "fail",
                         native_parity, expected, actual, error))
            print("%s: C parity %s; native parity %s" % (
                program_id, "PASS" if c_parity else "FAIL", native_parity))

        validation_cases = (
            ("noncanonical-list", [{"op": "const", "value": 1}]),
            ("unsupported-schema", {
                "kind": "zap.bytecode", "schema_version": 2, "instructions": []}),
            ("unsupported-opcode", {
                "kind": "zap.bytecode", "schema_version": 1,
                "instructions": [{"op": "raise"}]}),
        )
        for validation_id, artifact in validation_cases:
            try:
                validate_canonical_bytecode(artifact)
            except BytecodeValidationError:
                rows.append((validation_id, "pass", "n/a", "n/a", [], [], ""))
                print("%s: validation rejection PASS" % validation_id)
            except Exception as exc:
                rows.append((validation_id, "fail", "n/a", "n/a", [], [], str(exc)))
                failures.append(validation_id + ": unexpected validation result")
                print("%s: validation rejection FAIL (%s)" % (validation_id, exc))

    passed = sum(1 for row in rows if row[1] == "pass")
    failed = len(failures)
    summary = (passed, failed, native_skips, "not-certified", "provisional",
               "C parity and validation checks", "native parity requires a C compiler")
    _write_report(report_path, rows, summary)
    print("B3 bytecode C bridge: %d checks passed, %d failed, %d native skips" % (
        passed, failed, native_skips))
    print("Report: %s" % report_path)
    if failures:
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
