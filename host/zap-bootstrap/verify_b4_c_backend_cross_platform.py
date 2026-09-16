#!/usr/bin/env python3
import os
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REPORT_DIR = Path(os.environ.get("B4_C_BACKEND_REPORT_DIR", ROOT / "target" / "c-backend-reports"))
OUTPUT = Path(os.environ.get("B4_C_BACKEND_CROSS_PLATFORM_REPORT", ROOT / "target" / "b4-c-backend-cross-platform.tsv"))


def fail(message):
    raise SystemExit(message)


def load_reports():
    paths = sorted(REPORT_DIR.glob("*.tsv"))
    if not paths:
        fail(f"no C backend reports found in {REPORT_DIR}")
    reports = []
    for path in paths:
        lines = path.read_text(encoding="utf-8").splitlines()
        if len(lines) < 3:
            fail(f"invalid C backend report: {path}")
        header = lines[2].split("\t")
        rows = {}
        for line in lines[3:]:
            fields = line.split("\t")
            if fields and fields[0].startswith("B4-FULL-"):
                rows[fields[0]] = dict(zip(header, fields))
        if not rows:
            fail(f"report has no B4 rows: {path}")
        reports.append((path, rows))
    return reports


def main():
    reports = load_reports()
    platforms = {row.get("platform", "") for _, rows in reports for row in rows.values() if row.get("status") == "pass"}
    required = {"Linux", "Windows", "Darwin"}
    if not required.issubset(platforms):
        fail(f"missing platform reports: expected Linux, Windows, Darwin; got {sorted(platforms)}")

    ids = [f"B4-FULL-{number:03d}" for number in range(13, 19)]
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    with OUTPUT.open("w", encoding="utf-8", newline="") as output:
        output.write("schema_version\t2\n")
        output.write("contract_id\tB4-RUST-FREE-FULL-LANGUAGE\n")
        output.write("id\tarea\tstatus\tplatforms\tc_sha256\tstdout_sha256\tnative_sha256s\n")
        failed = 0
        for row_id in ids:
            values = []
            for _, rows in reports:
                row = rows.get(row_id)
                if row is None or row.get("status") != "pass":
                    values.append(None)
                else:
                    values.append(row)
            if any(value is None for value in values):
                failed += 1
                output.write(f"{row_id}\t\tfail\t\t\t\tmissing or failing row\n")
                continue
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
