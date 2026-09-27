#!/usr/bin/env python3
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import verify_b4_c_backend_cross_platform as verifier


PLATFORMS = ("Linux", "Windows", "Darwin")
ROW_IDS = tuple(f"B4-FULL-{number:03d}" for number in range(13, 19))
HASHES = {
    "c_sha256": "a" * 64,
    "exe_sha256": "b" * 64,
    "stdout_sha256": "c" * 64,
}


def write_report(path, platform, *, omit_row=None, failing_row=None, c_hash=None):
    lines = [
        "schema_version\t2",
        "contract_id\tB4-RUST-FREE-FULL-LANGUAGE",
        "id\tarea\tstatus\tplatform\tc_sha256\texe_sha256\tstdout_sha256\terror",
    ]
    for row_id in ROW_IDS:
        if row_id == omit_row:
            continue
        hashes = dict(HASHES)
        if c_hash and row_id == ROW_IDS[0]:
            hashes["c_sha256"] = c_hash
        status = "fail" if row_id == failing_row else "pass"
        lines.append(
            "\t".join(
                (
                    row_id,
                    row_id.lower(),
                    status,
                    platform,
                    hashes["c_sha256"],
                    hashes["exe_sha256"],
                    hashes["stdout_sha256"],
                    "",
                )
            )
        )
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


class CrossPlatformAggregationTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.directory = Path(self.temp.name)
        self.output = self.directory / "aggregate.tsv"

    def tearDown(self):
        self.temp.cleanup()

    def aggregate(self):
        with patch.object(verifier, "REPORT_DIR", self.directory), patch.object(
            verifier, "OUTPUT", self.output
        ):
            verifier.main()

    def test_accepts_three_complete_platform_reports(self):
        for platform in PLATFORMS:
            write_report(self.directory / f"{platform}.tsv", platform)

        self.aggregate()

        report = self.output.read_text(encoding="utf-8")
        self.assertIn("summary\t6\t0\n", report)
        self.assertEqual(report.count("\tpass\t"), 6)

    def test_rejects_missing_platform(self):
        for platform in PLATFORMS[:2]:
            write_report(self.directory / f"{platform}.tsv", platform)

        with self.assertRaisesRegex(SystemExit, "exactly Linux, Windows, Darwin"):
            self.aggregate()

    def test_rejects_incomplete_platform_report(self):
        for platform in PLATFORMS:
            write_report(
                self.directory / f"{platform}.tsv",
                platform,
                omit_row=ROW_IDS[-1] if platform == "Darwin" else None,
            )

        with self.assertRaisesRegex(SystemExit, "must contain exactly"):
            self.aggregate()

    def test_rejects_failing_platform_row(self):
        for platform in PLATFORMS:
            write_report(
                self.directory / f"{platform}.tsv",
                platform,
                failing_row=ROW_IDS[0] if platform == "Windows" else None,
            )

        with self.assertRaisesRegex(SystemExit, "non-passing row"):
            self.aggregate()

    def test_rejects_cross_platform_hash_mismatch(self):
        for platform in PLATFORMS:
            write_report(
                self.directory / f"{platform}.tsv",
                platform,
                c_hash="d" * 64 if platform == "Darwin" else None,
            )

        with self.assertRaisesRegex(SystemExit, "1 cross-platform"):
            self.aggregate()

    def test_rejects_conflicting_duplicate_platform_report(self):
        for platform in PLATFORMS:
            write_report(self.directory / f"{platform}.tsv", platform)
        write_report(self.directory / "Linux-extra.tsv", "Linux", c_hash="d" * 64)

        with self.assertRaisesRegex(SystemExit, "conflicting duplicate reports"):
            self.aggregate()

    def test_rejects_malformed_hash(self):
        for platform in PLATFORMS:
            write_report(self.directory / f"{platform}.tsv", platform)
        path = self.directory / "Windows.tsv"
        text = path.read_text(encoding="utf-8")
        path.write_text(text.replace("a" * 64, "not-a-hash", 1), encoding="utf-8")

        with self.assertRaisesRegex(SystemExit, "invalid c_sha256"):
            self.aggregate()


if __name__ == "__main__":
    unittest.main()
