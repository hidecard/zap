#!/usr/bin/env python3
"""Bounded HTTP load test for an explicitly supplied Zap endpoint.

This script is intentionally opt-in for remote targets and never prints bearer tokens.
It is suitable for a staging environment or a production canary window with an
operator-approved request budget.
"""
from __future__ import annotations

import argparse
import concurrent.futures
import json
import os
import statistics
import sys
import threading
import time
from dataclasses import dataclass
from urllib.error import HTTPError, URLError
from urllib.request import Request, urlopen
from urllib.parse import urlparse


@dataclass
class Result:
    status: str
    latency_ms: float


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Bounded Zap HTTP load test")
    parser.add_argument("--url", required=True, help="Explicit endpoint URL, e.g. https://api.example.com/health")
    parser.add_argument("--duration-seconds", type=float, default=30.0)
    parser.add_argument("--concurrency", type=int, default=8)
    parser.add_argument("--max-requests", type=int, default=0, help="0 means duration-bounded only")
    parser.add_argument("--timeout-seconds", type=float, default=5.0)
    parser.add_argument("--expected-status", default="200", help="Comma-separated acceptable HTTP status codes")
    parser.add_argument("--max-p95-ms", type=float, default=500.0)
    parser.add_argument("--min-success-ratio", type=float, default=0.99)
    parser.add_argument("--allow-remote", action="store_true", help="Required for non-loopback targets")
    parser.add_argument("--output", help="Optional JSON report path")
    return parser.parse_args()


def validate_args(args: argparse.Namespace) -> set[int]:
    parsed = urlparse(args.url)
    if parsed.scheme not in {"http", "https"} or not parsed.netloc or parsed.query or parsed.fragment:
        raise ValueError("--url must be an absolute HTTP(S) URL without query or fragment")
    hostname = (parsed.hostname or "").lower()
    local_hosts = {"127.0.0.1", "::1", "localhost"}
    if hostname not in local_hosts and not args.allow_remote:
        raise ValueError("remote targets require explicit --allow-remote")
    if not (0.1 <= args.duration_seconds <= 3600):
        raise ValueError("duration must be between 0.1 and 3600 seconds")
    if not (1 <= args.concurrency <= 256):
        raise ValueError("concurrency must be between 1 and 256")
    if args.max_requests < 0 or args.max_requests > 10_000_000:
        raise ValueError("max-requests must be between 0 and 10000000")
    if not (0.1 <= args.timeout_seconds <= 120):
        raise ValueError("timeout must be between 0.1 and 120 seconds")
    if not (0.0 <= args.min_success_ratio <= 1.0):
        raise ValueError("min-success-ratio must be between 0 and 1")
    return {int(item.strip()) for item in args.expected_status.split(",") if item.strip()}


def percentile(values: list[float], fraction: float) -> float:
    if not values:
        return 0.0
    ordered = sorted(values)
    index = min(len(ordered) - 1, max(0, int(round((len(ordered) - 1) * fraction))))
    return ordered[index]


def request_once(url: str, timeout: float, headers: dict[str, str]) -> Result:
    started = time.perf_counter()
    try:
        request = Request(url, method="GET", headers=headers)
        with urlopen(request, timeout=timeout) as response:
            response.read(128)
            status = str(response.status)
    except HTTPError as error:
        status = f"http_{error.code}"
    except (URLError, TimeoutError, OSError) as error:
        status = type(error).__name__.lower()
    latency_ms = (time.perf_counter() - started) * 1000.0
    return Result(status=status, latency_ms=latency_ms)


def worker(args: argparse.Namespace, stop: threading.Event, counter: list[int], lock: threading.Lock) -> list[Result]:
    headers = {"Accept": "application/json", "User-Agent": "zap-load-test/1"}
    token = os.environ.get("ZAP_LOAD_BEARER_TOKEN")
    if token:
        headers["Authorization"] = f"Bearer {token}"
    results: list[Result] = []
    while not stop.is_set():
        with lock:
            if args.max_requests and counter[0] >= args.max_requests:
                stop.set()
                break
            counter[0] += 1
        results.append(request_once(args.url, args.timeout_seconds, headers))
    return results


def main() -> int:
    args = parse_args()
    try:
        expected = validate_args(args)
    except ValueError as error:
        print(f"argument error: {error}", file=sys.stderr)
        return 2

    stop = threading.Event()
    counter = [0]
    lock = threading.Lock()
    deadline = time.monotonic() + args.duration_seconds
    timer = threading.Thread(target=lambda: (time.sleep(args.duration_seconds), stop.set()), daemon=True)
    timer.start()
    results: list[Result] = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=args.concurrency) as executor:
        futures = [executor.submit(worker, args, stop, counter, lock) for _ in range(args.concurrency)]
        while time.monotonic() < deadline and not stop.is_set():
            time.sleep(0.05)
        stop.set()
        for future in futures:
            results.extend(future.result())

    latencies = [result.latency_ms for result in results]
    successes = sum(1 for result in results if result.status in {str(code) for code in expected})
    total = len(results)
    success_ratio = successes / total if total else 0.0
    report = {
        "target": f"{urlparse(args.url).scheme}://{urlparse(args.url).netloc}{urlparse(args.url).path}",
        "duration_seconds": args.duration_seconds,
        "concurrency": args.concurrency,
        "requests": total,
        "successes": successes,
        "success_ratio": round(success_ratio, 6),
        "status_counts": {status: sum(1 for result in results if result.status == status) for status in sorted({r.status for r in results})},
        "latency_ms": {
            "min": round(min(latencies), 3) if latencies else 0.0,
            "mean": round(statistics.fmean(latencies), 3) if latencies else 0.0,
            "p50": round(percentile(latencies, 0.50), 3),
            "p95": round(percentile(latencies, 0.95), 3),
            "p99": round(percentile(latencies, 0.99), 3),
            "max": round(max(latencies), 3) if latencies else 0.0,
        },
        "thresholds": {"min_success_ratio": args.min_success_ratio, "max_p95_ms": args.max_p95_ms},
    }
    rendered = json.dumps(report, indent=2, sort_keys=True)
    print(rendered)
    if args.output:
        with open(args.output, "w", encoding="utf-8") as handle:
            handle.write(rendered + "\n")
    p95 = report["latency_ms"]["p95"]
    return 0 if total and success_ratio >= args.min_success_ratio and p95 <= args.max_p95_ms else 1


if __name__ == "__main__":
    raise SystemExit(main())
