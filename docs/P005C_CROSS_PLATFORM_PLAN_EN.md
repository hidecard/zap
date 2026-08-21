# P0-05-C Cross-Platform Async Matrix Plan

## Goal

Make the production-oriented async boundary verifiable on Linux, Windows, and macOS through one reproducible focused test script and one target-native CI evidence artifact per target.

## Matrix contract

| Target | Required evidence | Platform-specific expectation |
|---|---|---|
| Linux x86_64 | Build, focused async tests, uploaded log | Regular-file read, loopback TCP exchange, bounded process output, deadline, cancellation, and typed preflight errors pass natively |
| Windows x86_64 | Build, focused async tests, uploaded log | `cmd.exe` process adapter path, Windows path/regular-file behavior, loopback TCP exchange, bounded output, cancellation, and typed preflight errors pass natively |
| macOS ARM64 | Build, focused async tests, uploaded log | Native ARM64 build, regular-file read, loopback TCP exchange, bounded process output, deadline, cancellation, and typed preflight errors pass natively |

## Focused test set

The matrix script runs the same exact test names on every target: worker concurrency, invalid-limit preflight, TCP round trip, oversized TCP response, oversized TCP request before admission, cross-platform process output, capped process output, forced process cancellation, and bounded regular-file read. The full native suite remains a separate gate; this matrix is the focused evidence that the adapter boundary itself is exercised on each target.

## Evidence and limitation policy

Each target job writes a deterministic text log containing the target triple, runner OS, Rust version, and exact test commands. The log is uploaded as a target-named CI artifact. A target that cannot run because of a documented runner/toolchain limitation must produce a versioned limitation record instead of being silently skipped. This step does not claim that arbitrary foreign blocking calls are interruptible or that language-level futures are executor-backed.

## Release gates

The step is complete when the matrix script passes locally on the host, the GitHub Actions build matrix invokes it for all three targets, artifact upload is configured, the async runtime contract documents the matrix, and the P0/P1 register moves the next execution item to P1-05-A replayable verification layers.
