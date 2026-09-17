#!/usr/bin/env python3
import hashlib
import os
import platform
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "host" / "zap-bootstrap"))
import c_backend  # noqa: E402


FIXTURES = {
    "cli": ROOT / "bootstrap" / "fixtures" / "b4" / "c_backend_cli.zp",
    "datastructures": ROOT / "bootstrap" / "fixtures" / "b4" / "c_backend_datastructures.zp",
    "full_surface": ROOT / "bootstrap" / "fixtures" / "b4" / "c_backend_full_surface.zp",
    "seed": ROOT / "bootstrap" / "fixtures" / "b4" / "c_backend_seed.zp",
    "self_rebuild": ROOT / "bootstrap" / "fixtures" / "b4" / "c_backend_self_rebuild.zp",
}

EXPECTED = {
    "datastructures": [
        "keys: name,stage",
        "has_sum: true",
        "missing: false",
        "total: 31",
        "first: 3",
        "mutated: 12",
        "len: 8",
        "map: b4-determinism/c-backend",
        "option: true",
        "fallback: true",
        "nested: b4-c-backend",
    ],
    "full_surface": [
        "sum: 15",
        "scaled: [3, 6, 9, 12, 15]",
        "labels: 2",
        "folded: 45",
        "squares: [0, 1, 4, 9]",
        "graded: {alpha: 30, beta: 20}",
        "keys: 2",
        "values: 2",
        "has_beta: true",
        "score: 20",
        "negative: negative",
        "zero: zero",
        "positive: positive",
        "record: {name: zap, score: 42}",
        "record_name: zap",
        "found_is_some: true",
        "missing_is_none: true",
        "missing_fallback: unknown",
        "ok: 3",
        "bad_is_error: true",
        "factorial: 720",
        "greet: hello zap",
        "task: 1",
        "argv_is_list: true",
    ],
    "seed": [
        "list: [10, 2, 3, 4]",
        "len: 4",
        "has: true",
        "map: {a: 1, b: 2, c: 3}",
        "a: 1",
        "keys: 3",
        "squares: 14",
        "sum: 19",
        "fib: 13",
        "div: 3",
        "rem: 2",
        "bool: true",
    ],
    "self_rebuild": [
        "seed: zap-seed",
        "1-seed.zp:ok",
        "stages: 3",
        "keys: 3",
        "values: 3",
        "self: zap-seed",
        "bytes: 3",
        "has_stage: true",
        "has_bytes: true",
    ],
}

CLI_EXPECTED = {
    (): ["argc: 0", "usage: zap <command> [source] commands: 4"],
    ("check",): ["argc: 1", "command: check", "supported: true"],
    ("build",): ["argc: 1", "command: build", "supported: true"],
    ("run",): ["argc: 1", "command: run", "supported: true"],
    ("test",): ["argc: 1", "command: test", "supported: true"],
    ("unsupported",): [
        "argc: 1",
        "ZAP-DRIVER-001 unsupported driver command: unsupported",
        "supported: false",
    ],
}


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def output_lines(result):
    if result.returncode != 0:
        raise AssertionError(result.stderr or result.stdout or "executable exited nonzero")
    return result.stdout.replace("\r\n", "\n").replace("\r", "\n").splitlines()


def build(source, prefix):
    result = c_backend.build_native_binary_from_file(str(source), str(prefix))
    c_path = Path(result["c_path"])
    exe_path = Path(result["exe_path"])
    if not c_path.is_file() or not exe_path.is_file():
        raise AssertionError("C backend did not produce both C and native artifacts")
    return c_path, exe_path, result


def run(exe, args=(), env=None):
    result = subprocess.run(
        [str(exe), *args],
        cwd=ROOT,
        env=env,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )
    return output_lines(result), result


def require_equal(actual, expected, label):
    if actual != expected:
        raise AssertionError(f"{label}: expected {expected!r}, got {actual!r}")


def run_fixture(name, args=(), expected=None, env=None):
    with tempfile.TemporaryDirectory(prefix=f"b4-{name}-") as temp:
        c_path, exe_path, _ = build(FIXTURES[name], Path(temp) / name)
        actual, _ = run(exe_path, args, env)
        require_equal(actual, expected or EXPECTED[name], f"{name} output")
        return {
            "c_sha256": sha256(c_path),
            "exe_sha256": sha256(exe_path),
            "stdout_sha256": hashlib.sha256("\n".join(actual).encode("utf-8")).hexdigest(),
            "platform": platform.system(),
        }


def build_pair(name, label, compare_exe=True):
    with tempfile.TemporaryDirectory(prefix=f"b4-{label}-") as temp:
        root = Path(temp)
        first = build(FIXTURES[name], root / "first")
        second = build(FIXTURES[name], root / "second")
        first_output, _ = run(first[1])
        second_output, _ = run(second[1])
        require_equal(first_output, second_output, f"{label} output")
        if first[0].read_bytes() != second[0].read_bytes():
            raise AssertionError(f"{label} emitted C differs across fresh builds")
        if compare_exe and first[1].read_bytes() != second[1].read_bytes():
            raise AssertionError(f"{label} native executable differs across fresh builds")
        return {
            "c_sha256": sha256(first[0]),
            "exe_sha256": sha256(first[1]),
            "stdout_sha256": hashlib.sha256("\n".join(first_output).encode("utf-8")).hexdigest(),
            "platform": platform.system(),
        }


def clean_environment():
    env = os.environ.copy()
    for key in list(env):
        if key.upper() in {"CARGO", "CARGO_HOME", "RUSTC", "RUSTUP_HOME", "RUSTUP_TOOLCHAIN"}:
            del env[key]
    with tempfile.TemporaryDirectory(prefix="b4-clean-env-") as temp:
        c_path, exe_path, _ = build(FIXTURES["full_surface"], Path(temp) / "full")
        normal, _ = run(exe_path)
        clean, _ = run(exe_path, env=env)
        require_equal(clean, normal, "clean environment output")
        require_equal(clean, EXPECTED["full_surface"], "clean environment fixture output")
        return {
            "c_sha256": sha256(c_path),
            "exe_sha256": sha256(exe_path),
            "stdout_sha256": hashlib.sha256("\n".join(clean).encode("utf-8")).hexdigest(),
            "platform": platform.system(),
        }


def cli_case():
    with tempfile.TemporaryDirectory(prefix="b4-cli-") as temp:
        c_path, exe_path, _ = build(FIXTURES["cli"], Path(temp) / "cli")
        outputs = {}
        for args, expected in CLI_EXPECTED.items():
            actual, _ = run(exe_path, args)
            require_equal(actual, expected, f"cli {args or ('usage',)} output")
            outputs[args] = actual
        return {
            "c_sha256": sha256(c_path),
            "exe_sha256": sha256(exe_path),
            "stdout_sha256": hashlib.sha256("\n".join(outputs[("check",)]).encode("utf-8")).hexdigest(),
            "platform": platform.system(),
        }


def main():
    report_path = Path(os.environ.get("B4_C_BACKEND_ACCEPTANCE_REPORT", ROOT / "target" / "b4-c-backend-acceptance.tsv"))
    report_path.parent.mkdir(parents=True, exist_ok=True)

    # Also copy to c-backend-reports directory for cross-platform comparison
    reports_dir = Path(os.environ.get("B4_C_BACKEND_REPORT_DIR", ROOT / "target" / "c-backend-reports"))
    reports_dir.mkdir(parents=True, exist_ok=True)
    results = {}
    failures = []

    checks = [
        ("B4-FULL-013", "cli-entrypoint", cli_case),
        ("B4-FULL-014", "self-rebuild", lambda: build_pair("self_rebuild", "self-rebuild")),
        ("B4-FULL-015", "cross-platform-determinism", lambda: build_pair("full_surface", "cross-platform", compare_exe=False)),
        ("B4-FULL-016", "byte-determinism", lambda: build_pair("seed", "byte-determinism")),
        ("B4-FULL-017", "second-stage-rebuild", lambda: build_pair("self_rebuild", "second-stage")),
        ("B4-FULL-018", "clean-environment", clean_environment),
    ]
    for row_id, area, check in checks:
        try:
            results[row_id] = {"status": "pass", "area": area, **check()}
            print(f"PASS {row_id} {area}")
        except Exception as exc:
            failures.append((row_id, area, str(exc)))
            results[row_id] = {"status": "fail", "area": area, "error": str(exc), "platform": platform.system()}
            print(f"FAIL {row_id} {area}: {exc}", file=sys.stderr)

    reference = os.environ.get("B4_C_BACKEND_REFERENCE_REPORT")
    if reference:
        reference_path = Path(reference)
        reference_failed = False
        if not reference_path.is_file():
            reference_failed = f"missing reference report {reference_path}"
        else:
            reference_rows = {}
            for line in reference_path.read_text(encoding="utf-8").splitlines():
                fields = line.split("\t")
                if len(fields) >= 8 and fields[0].startswith("B4-FULL-"):
                    reference_rows[fields[0]] = fields
            current = results.get("B4-FULL-015", {})
            prior = reference_rows.get("B4-FULL-015", [])
            if len(prior) < 8:
                reference_failed = "reference report has no B4-FULL-015 row"
            else:
                if current.get("c_sha256") != prior[4]:
                    reference_failed = "emitted C differs from reference platform"
                elif current.get("stdout_sha256") != prior[6]:
                    reference_failed = "fixture output differs from reference platform"
        if reference_failed:
            failures.append(("B4-FULL-015", "cross-platform-determinism", reference_failed))
            results["B4-FULL-015"] = {
                "status": "fail",
                "area": "cross-platform-determinism",
                "platform": platform.system(),
                "error": reference_failed,
            }
            print(f"FAIL B4-FULL-015 cross-platform-reference: {reference_failed}", file=sys.stderr)
        else:
            print("PASS B4-FULL-015 cross-platform-reference")

    with report_path.open("w", encoding="utf-8", newline="") as report:
        report.write("schema_version\t2\n")
        report.write("contract_id\tB4-RUST-FREE-FULL-LANGUAGE\n")
        report.write("id\tarea\tstatus\tplatform\tc_sha256\texe_sha256\tstdout_sha256\terror\n")
        for row_id, area, _ in checks:
            row = results[row_id]
            report.write("\t".join([
                row_id,
                area,
                row["status"],
                row.get("platform", ""),
                row.get("c_sha256", ""),
                row.get("exe_sha256", ""),
                row.get("stdout_sha256", ""),
                row.get("error", ""),
            ]) + "\n")
        report.write(f"summary\t{len(checks) - len({item[0] for item in failures})}\t{len(failures)}\n")

    if failures:
        raise SystemExit(1)
    print(f"B4 C backend acceptance passed: {len(checks)}/6 rows on {platform.system()}")

    # Copy report to c-backend-reports directory for cross-platform comparison
    cross_platform_report = reports_dir / f"{platform.system()}.tsv"
    shutil.copy(report_path, cross_platform_report)
    print(f"Copied report to {cross_platform_report}")


if __name__ == "__main__":
    main()
