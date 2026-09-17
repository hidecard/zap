#!/usr/bin/env python3
"""Focused parity tests for the canonical bytecode to C bridge."""
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

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


class BytecodeCBridgeTest(unittest.TestCase):
    def test_emit_c_matches_reference_for_compile_programs(self):
        with tempfile.TemporaryDirectory(prefix="zap-bytecode-c-") as tmp:
            tmpdir = Path(tmp)
            for name, source, expected in _PROGRAMS:
                with self.subTest(program=name):
                    program = compile_program(source)
                    reference_path = tmpdir / (name + "-reference.c")
                    bridge_path = tmpdir / (name + "-bridge.c")
                    emit_c(program, str(reference_path))
                    emit_c_from_bytecode(_canonical(program), str(bridge_path))
                    self.assertEqual(reference_path.read_bytes(), bridge_path.read_bytes())

    def test_native_artifacts_match_reference_when_compiler_exists(self):
        compiler = find_c_compiler()
        if compiler is None:
            self.skipTest("no system C compiler is available")

        with tempfile.TemporaryDirectory(prefix="zap-bytecode-c-native-") as tmp:
            tmpdir = Path(tmp)
            for name, source, expected in _PROGRAMS:
                with self.subTest(program=name):
                    program = compile_program(source)
                    reference_c = tmpdir / (name + "-reference.c")
                    reference_exe = tmpdir / (name + "-reference")
                    emit_c(program, str(reference_c))
                    compile_c(str(reference_c), str(reference_exe), compiler=compiler)

                    bridge = build_native_binary_from_bytecode(
                        _canonical(program), str(tmpdir / (name + "-bridge")),
                        compiler=compiler)
                    bridge_exe = Path(bridge["exe_path"])
                    self.assertTrue(bridge_exe.is_file())

                    reference_run = subprocess.run(
                        [str(reference_exe)], capture_output=True, text=True)
                    bridge_run = subprocess.run(
                        [str(bridge_exe)], capture_output=True, text=True)
                    self.assertEqual(reference_run.returncode, bridge_run.returncode)
                    self.assertEqual(reference_run.stdout, bridge_run.stdout)
                    self.assertEqual(reference_run.stderr, bridge_run.stderr)
                    self.assertEqual(
                        [line for line in bridge_run.stdout.splitlines() if line],
                        expected)

    def test_b3_dup_and_index_instructions_emit_native_artifacts(self):
        compiler = find_c_compiler()
        if compiler is None:
            self.skipTest("no system C compiler is available")

        program = [
            {"op": "const", "value": 1},
            {"op": "const", "value": 2},
            {"op": "make_list", "count": 2},
            {"op": "const", "value": 0},
            {"op": "index"},
            {"op": "const", "value": 1},
            {"op": "dup"},
            {"op": "add"},
            {"op": "print"},
            {"op": "halt"},
        ]
        with tempfile.TemporaryDirectory(prefix="zap-bytecode-c-b3-") as tmp:
            result = build_native_binary_from_bytecode(
                _canonical(program), str(Path(tmp) / "b3"), compiler=compiler)
            run = subprocess.run(
                [result["exe_path"]], capture_output=True, text=True)
            self.assertEqual(run.returncode, 0, run.stderr)
            self.assertEqual(run.stdout.splitlines(), ["2"])

    def test_rejects_noncanonical_and_unsupported_bytecode(self):
        cases = (
            [{"op": "const", "value": 1}],
            {"kind": "zap.bytecode", "schema_version": 2, "instructions": []},
            {"kind": "zap.bytecode", "schema_version": 1,
             "instructions": [{"op": "raise"}]},
            {"artifact_kind": "bytecode", "instructions": "not-a-list"},
        )
        for artifact in cases:
            with self.subTest(artifact=artifact):
                with self.assertRaises(BytecodeValidationError):
                    validate_canonical_bytecode(artifact)

    def test_accepts_driver_bytecode_artifact(self):
        artifact = {
            "artifact_kind": "bytecode",
            "instructions": [
                {"op": "const", "value": "driver"},
                {"op": "print"},
                {"op": "halt"},
            ],
        }
        with tempfile.TemporaryDirectory(prefix="zap-bytecode-c-driver-") as tmp:
            output = Path(tmp) / "driver.c"
            emit_c_from_bytecode(artifact, str(output))
            self.assertIn("driver", output.read_text(encoding="utf-8"))


if __name__ == "__main__":
    unittest.main(verbosity=2)
