# P0-05-B Async Resource-Limit Preflight Plan

## Goal

Make every production-oriented async adapter reject invalid or oversized resource requests before admission, while preserving deterministic errors, bounded output, cancellation cleanup, and the existing Rust 1.75 compatibility boundary.

## Scope for this step

This step covers the fixed-worker scheduler, regular-file read adapter, bounded TCP adapter, and bounded process adapter. It does not add language-level `async/await` scheduling, arbitrary foreign-blocking interruption, or platform-specific CI provisioning; those remain later P0-05/P1-05 work.

| ID | Requirement | Acceptance evidence |
|---|---|---|
| P005B-01 | Validate `ThreadRuntimeLimits` before worker creation | Zero workers, zero tasks, and zero read bytes return stable typed errors; no worker threads are started for invalid limits |
| P005B-02 | Validate adapter limits before an adapter task is admitted | Zero socket bytes, zero socket timeout, zero process-output bytes, and zero process timeout are rejected deterministically |
| P005B-03 | Reject an oversized TCP request before queue admission | A request larger than the configured socket/read bound returns a typed input-limit error and does not consume a task slot |
| P005B-04 | Preserve existing output/deadline/cancellation behavior | Existing oversized-response, oversized-process-output, deadline, and terminate-then-drain tests remain green |
| P005B-05 | Keep errors stable across callers | Error variants and messages are deterministic, do not expose addresses/secrets, and are documented in English and Burmese |
| P005B-06 | Keep capability reporting honest | `async_capabilities()` states that resource-limit preflight is enforced after this step and does not claim language-level scheduling support |

## Implementation order

First add typed validation methods to the limit structs and call them before thread creation or adapter admission. Then add the TCP request-size preflight check. Next add focused unit tests for every zero/oversized boundary and retain the existing integration tests for successful I/O, response limits, process limits, cancellation, and worker admission. Finally update the async runtime contract, standard-library index, changelog, and P0/P1 register.

## Release gates

The step is complete only when rustfmt, strict Clippy, the full native suite, focused P0-05-B tests, `git diff --check`, and English/Burmese documentation parity all pass. The commit must remain separate from later cross-platform matrix provisioning so that the boundary change can be reviewed and reverted independently.
