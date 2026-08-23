#!/usr/bin/env python3
"""Opt-in, bounded chaos experiments for a Zap service.

Service-control experiments require an exact confirmation string and should only
run during an approved canary window. No experiment targets a remote host unless
--allow-remote is explicitly supplied.
"""
from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import time
from urllib.error import HTTPError, URLError
from urllib.request import Request, urlopen
from urllib.parse import urlparse


CONFIRM = "I_UNDERSTAND_DOWNTIME"


def args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Bounded Zap chaos experiments")
    parser.add_argument("--fault", required=True, choices=("invalid-jwt", "restart-service", "stop-start-service"))
    parser.add_argument("--url", required=True, help="Health or protected endpoint")
    parser.add_argument("--service", default="zap-web.service")
    parser.add_argument("--allow-remote", action="store_true")
    parser.add_argument("--allow-service-control", action="store_true")
    parser.add_argument("--confirm", default="")
    parser.add_argument("--recovery-timeout-seconds", type=float, default=60.0)
    return parser.parse_args()


def validate_target(url: str, allow_remote: bool) -> None:
    parsed = urlparse(url)
    if parsed.scheme not in {"http", "https"} or not parsed.netloc or parsed.query or parsed.fragment:
        raise ValueError("--url must be an absolute HTTP(S) URL without query or fragment")
    if (parsed.hostname or "").lower() not in {"127.0.0.1", "localhost", "::1"} and not allow_remote:
        raise ValueError("remote targets require explicit --allow-remote")


def probe(url: str, headers: dict[str, str] | None = None, timeout: float = 5.0) -> int | None:
    try:
        request = Request(url, method="GET", headers=headers or {"Accept": "application/json"})
        with urlopen(request, timeout=timeout) as response:
            response.read(128)
            return response.status
    except HTTPError as error:
        return error.code
    except (URLError, TimeoutError, OSError):
        return None


def wait_for(url: str, expected: int, timeout: float) -> bool:
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if probe(url) == expected:
            return True
        time.sleep(0.5)
    return False


def run_systemctl(action: str, service: str) -> None:
    if not re.fullmatch(r"[A-Za-z0-9_.@-]+", service):
        raise ValueError("service name contains unsupported characters")
    subprocess.run(["systemctl", action, service], check=True, timeout=30)


def main() -> int:
    parsed = args()
    try:
        validate_target(parsed.url, parsed.allow_remote)
        if parsed.recovery_timeout_seconds <= 0 or parsed.recovery_timeout_seconds > 600:
            raise ValueError("recovery timeout must be between 1 and 600 seconds")
        if parsed.fault in {"restart-service", "stop-start-service"}:
            if not parsed.allow_service_control or parsed.confirm != CONFIRM:
                raise ValueError(
                    f"{parsed.fault} requires --allow-service-control --confirm {CONFIRM}"
                )
    except ValueError as error:
        print(f"argument error: {error}", file=sys.stderr)
        return 2

    report: dict[str, object] = {"fault": parsed.fault, "target": f"{urlparse(parsed.url).scheme}://{urlparse(parsed.url).netloc}{urlparse(parsed.url).path}", "passed": False}
    if parsed.fault == "invalid-jwt":
        status = probe(parsed.url, {"Accept": "application/json", "Authorization": "Bearer chaos-invalid-token"})
        report.update({"observed_status": status, "expected_status": 401, "passed": status == 401})
    elif parsed.fault == "restart-service":
        before = probe(parsed.url)
        run_systemctl("restart", parsed.service)
        recovered = wait_for(parsed.url, 200, parsed.recovery_timeout_seconds)
        report.update({"status_before": before, "recovered": recovered, "passed": before == 200 and recovered})
    else:
        before = probe(parsed.url)
        run_systemctl("stop", parsed.service)
        down = probe(parsed.url)
        run_systemctl("start", parsed.service)
        recovered = wait_for(parsed.url, 200, parsed.recovery_timeout_seconds)
        report.update({"status_before": before, "status_during_stop": down, "recovered": recovered, "passed": before == 200 and down != 200 and recovered})

    print(json.dumps(report, indent=2, sort_keys=True))
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
