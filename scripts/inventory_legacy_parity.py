#!/usr/bin/env python3
import hashlib
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REPORT = Path(os.environ.get("ZAP_LEGACY_PARITY_INVENTORY", ROOT / "target" / "legacy-parity-inventory.tsv"))
TIMEOUT_SECONDS = float(os.environ.get("ZAP_LEGACY_PARITY_TIMEOUT", "15"))
INVENTORY_VERSION = "2026-09-19"


def fail(message):
    print(f"legacy parity inventory: {message}", file=sys.stderr)
    raise SystemExit(2)


def executable(path):
    return path.is_file() and os.access(path, os.X_OK)


def with_extension(path):
    if executable(path):
        return path
    candidate = Path(str(path) + ".exe")
    if executable(candidate):
        return candidate
    return None


def resolve_native():
    override = os.environ.get("ZAP_BIN_OVERRIDE") or os.environ.get("ZAP_BIN")
    if override:
        resolved = with_extension(Path(override))
        if resolved is None:
            fail(f"native binary is not executable: {override}")
        return resolved

    candidates = (
        ROOT / "native" / "target" / "release" / "zap",
        ROOT / "native" / "target" / "debug" / "zap",
        ROOT / "bin" / "zap",
    )
    for candidate in candidates:
        resolved = with_extension(candidate)
        if resolved is not None:
            return resolved

    cargo = shutil.which("cargo") or shutil.which("cargo.exe")
    if cargo is None:
        fail("native binary missing and cargo is not available")
    subprocess.run(
        [cargo, "build", "--locked", "--manifest-path", str(ROOT / "native" / "Cargo.toml"), "--bin", "zap"],
        cwd=ROOT,
        check=True,
    )
    for candidate in candidates:
        resolved = with_extension(candidate)
        if resolved is not None:
            return resolved
    fail("cargo build did not produce an executable native binary")


def resolve_legacy_python():
    configured = os.environ.get("ZAP_LEGACY_PYTHON", "python3")
    if shutil.which(configured) is None:
        fail(f"legacy Python interpreter is not available: {configured}")
    return configured


def tracked_fixtures():
    process = subprocess.run(
        ["git", "ls-files", "--", "*.zp"],
        cwd=ROOT,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        check=True,
    )
    included = []
    for raw_path in process.stdout.splitlines():
        relative = raw_path.replace("\\", "/")
        if relative.startswith("bootstrap/fixtures/") or relative.startswith("conformance/"):
            included.append(relative)
        elif relative.startswith("examples/") or relative.startswith("tests/"):
            included.append(relative)
        elif relative.startswith("corpus/") or relative.startswith("frameworks/"):
            included.append(relative)
        elif relative == "scripts/p3_smoke.zp":
            included.append(relative)
    return sorted(set(included))


def fixture_id(relative):
    slug = re.sub(r"[^A-Za-z0-9]+", "-", relative).strip("-").upper()
    digest = hashlib.sha256(relative.encode("utf-8")).hexdigest()[:8].upper()
    return f"P001-INV-{slug}-{digest}"


def category_for(relative):
    parts = relative.split("/")
    if parts[0] == "bootstrap" and len(parts) > 1:
        return f"bootstrap/{parts[1]}"
    return parts[0] if parts else "unknown"


def normalize_output(data):
    text = data.decode("utf-8", "replace")
    text = text.replace("\r\n", "\n").replace("\r", "\n")
    lines = [line.rstrip() for line in text.split("\n") if line.strip()]
    return "\n".join(lines).encode("utf-8")


def digest(data):
    return hashlib.sha256(data).hexdigest()


def engine_source(source, native):
    value = str(source).replace("\\", "/")
    if str(native).lower().endswith(".exe"):
        match = re.match(r"^/([A-Za-z])/(.*)$", value)
        if match:
            return f"{match.group(1).upper()}:\\{match.group(2).replace('/', chr(92))}"
    return str(source)


def run_engine(command, source, native):
    with tempfile.TemporaryDirectory(prefix="zap-legacy-parity-") as work_dir:
        output_path = Path(work_dir) / "stdout"
        error_path = Path(work_dir) / "stderr"
        try:
            with output_path.open("wb") as output_handle, error_path.open("wb") as error_handle:
                process = subprocess.run(
                    [*command, engine_source(source, native)],
                    cwd=work_dir,
                    stdout=output_handle,
                    stderr=error_handle,
                    timeout=TIMEOUT_SECONDS,
                )
            status = process.returncode
            stdout = output_path.read_bytes()
        except subprocess.TimeoutExpired:
            status = 124
            stdout = b""
        except OSError as error:
            fail(f"could not execute engine: {error}")
    return status, normalize_output(stdout)


def classify(native_status, legacy_status, native_bytes, legacy_bytes):
    if native_status == 0 and legacy_status == 0:
        if digest(native_bytes) == digest(legacy_bytes):
            return "normative", "Both runtimes succeed with identical normalized stdout"
        return "compatibility", "Both runtimes succeed with different normalized stdout"
    if native_status == 0:
        return "native-only", "Native runtime succeeds and legacy runtime rejects"
    if legacy_status == 0:
        return "deprecated", "Legacy runtime succeeds and native runtime rejects"
    return "rejected", "Both runtimes reject the fixture"


def main():
    native = resolve_native()
    legacy_python = resolve_legacy_python()
    legacy_script = ROOT / "legacy" / "zap.py"
    if not legacy_script.is_file():
        fail(f"legacy runtime is missing: {legacy_script}")

    fixtures = tracked_fixtures()
    if not fixtures:
        fail("no tracked legacy-parity fixtures were found")

    native_command = [str(native), "run"]
    legacy_command = [legacy_python, str(legacy_script), "run"]
    rows = []
    for relative in fixtures:
        source = ROOT / relative
        native_status, native_bytes = run_engine(native_command, source, native)
        legacy_status, legacy_bytes = run_engine(legacy_command, source, native)
        if native_status == 124 or legacy_status == 124:
            fail(f"fixture timed out after {TIMEOUT_SECONDS:g}s: {relative}")
        classification, rationale = classify(
            native_status,
            legacy_status,
            native_bytes,
            legacy_bytes,
        )
        rows.append(
            (
                fixture_id(relative),
                category_for(relative),
                relative,
                native_status,
                legacy_status,
                digest(native_bytes),
                digest(legacy_bytes),
                classification,
                rationale,
            )
        )

    REPORT.parent.mkdir(parents=True, exist_ok=True)
    temporary_report = REPORT.with_suffix(REPORT.suffix + ".tmp")
    with temporary_report.open("w", encoding="utf-8", newline="") as handle:
        handle.write(
            "inventory_version\tfixture_id\tcategory\tfixture\tnative_status\tlegacy_status\t"
            "native_output_sha256\tlegacy_output_sha256\tclassification\trationale\n"
        )
        for row in rows:
            handle.write("\t".join(str(value) for value in row) + "\n")
    temporary_report.replace(REPORT)

    counts = {name: 0 for name in ("normative", "compatibility", "native-only", "deprecated", "rejected")}
    for row in rows:
        counts[row[7]] += 1
    print(f"Legacy parity inventory: {len(rows)} tracked fixtures")
    for name in ("normative", "compatibility", "native-only", "deprecated", "rejected"):
        print(f"{name}: {counts[name]}")
    print(f"Report: {REPORT}")


if __name__ == "__main__":
    main()
