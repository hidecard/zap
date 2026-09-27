#!/usr/bin/env python3
import os
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REPORT_DIR = Path(os.environ.get("B4_C_BACKEND_REPORT_DIR", ROOT / "target" / "c-backend-reports"))
OUTPUT = Path(os.environ.get("B4_C_BACKEND_CROSS_PLATFORM_REPORT", ROOT / "target" / "b4-c-backend-cross-platform.tsv"))
REQUIRED_PLATFORMS = {"Linux", "Windows", "Darwin"}
REQUIRED_IDS = {f"B4-FULL-{number:03d}" for number in range(13, 19)}
HASH_FIELDS = ("c_sha256", "exe_sha256", "stdout_sha256")


def fail(message):
    raise SystemExit(message)


def load_reports():
    paths = sorted(REPORT_DIR.glob("*.tsv"))
    if not paths:
        fail(f"no C backend reports found in {REPORT_DIR}")
    reports = {}
    for path in paths:
        lines = path.read_text(encoding="utf-8-sig").splitlines()
        if len(lines) < 3:
            continue
        header = lines[2].split("\t")
        if not {"id", "area", "status", "platform"}.issubset(set(header)):
            continue
        rows = {}
        for line in lines[3:]:
            fields = line.split("\t")
            if fields and fields[0].startswith("B4-FULL-"):
                if len(fields) != len(header):
                    fail(f"malformed row in {path}: {line}")
                row = dict(zip(header, fields))
                row_id = row["id"]
                if row_id in rows:
                    fail(f"duplicate {row_id} row in {path}")
                rows[row_id] = row
        if not rows:
            continue
        if set(rows) != REQUIRED_IDS:
            missing = sorted(REQUIRED_IDS - set(rows))
            unexpected = sorted(set(rows) - REQUIRED_IDS)
            fail(f"{path} must contain exactly B4-FULL-013..018; missing={missing}, unexpected={unexpected}")

        platforms = {row["platform"] for row in rows.values()}
        if len(platforms) != 1:
            fail(f"report {path} contains inconsistent platforms: {sorted(platforms)}")
        platform = platforms.pop()
        if platform not in REQUIRED_PLATFORMS:
            fail(f"report {path} has unsupported platform: {platform!r}")
        for row_id, row in rows.items():
            if row["status"] != "pass":
                fail(f"{path} has a non-passing row: {row_id} ({row['status']})")
            if not row["area"]:
                fail(f"{path} has an empty area for {row_id}")
            for field in HASH_FIELDS:
                if not re.fullmatch(r"[0-9a-fA-F]{64}", row.get(field, "")):
                    fail(f"{path} has an invalid {field} for {row_id}")

        existing = reports.get(platform)
        if existing is not None:
            _, existing_rows = existing
            if rows != existing_rows:
                fail(f"conflicting duplicate reports for {platform}: {existing[0]} and {path}")
            continue
        reports[platform] = (path, rows)
    if not reports:
        fail(f"no C backend acceptance reports found in {REPORT_DIR}")
    return reports


def main():
    reports = load_reports()
    platforms = set(reports)
    if platforms != REQUIRED_PLATFORMS:
        fail(f"expected exactly Linux, Windows, Darwin reports; got {sorted(platforms)}")

    ids = [f"B4-FULL-{number:03d}" for number in range(13, 19)]
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    with OUTPUT.open("w", encoding="utf-8", newline="") as output:
        output.write("schema_version\t2\n")
        output.write("contract_id\tB4-RUST-FREE-FULL-LANGUAGE\n")
        output.write("id\tarea\tstatus\tplatforms\tc_sha256\tstdout_sha256\tnative_sha256s\n")
        failed = 0
        for row_id in ids:
            values = [reports[platform][1][row_id] for platform in sorted(REQUIRED_PLATFORMS)]
            c_hashes = {value["c_sha256"] for value in values}
            stdout_hashes = {value["stdout_sha256"] for value in values}
            native_hashes = sorted({value["exe_sha256"] for value in values})
            status = "pass" if len(c_hashes) == 1 and len(stdout_hashes) == 1 else "fail"
            if status == "fail":
                failed += 1
            output.write("\t".join([
                row_id,
                values[0]["area"],
                status,
                ",".join(sorted(platforms)),
                next(iter(c_hashes)) if len(c_hashes) == 1 else ",".join(sorted(c_hashes)),
                next(iter(stdout_hashes)) if len(stdout_hashes) == 1 else ",".join(sorted(stdout_hashes)),
                ",".join(native_hashes),
            ]) + "\n")
        output.write(f"summary\t{len(ids) - failed}\t{failed}\n")

    if failed:
        fail(f"{failed} cross-platform C backend rows failed; report: {OUTPUT}")
    print(f"B4 C backend cross-platform comparison passed: {len(ids)}/6 rows across {len(platforms)} platforms")


if __name__ == "__main__":
    main()
