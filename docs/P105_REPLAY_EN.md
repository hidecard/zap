# P1-05-A Replayable Verification Layers

## Purpose

P1-05-A makes the existing panic-free corpus checks replayable rather than dependent only on inline test data. Every replay uses a positive decimal `ZAP_CORPUS_SEED`, a stable category order, lexically sorted durable fixtures, and a deterministic seeded permutation. The default seed is `20260821`.

> A failure report is actionable only when it records the seed, fixture path, SHA-256 digest, and exact input bytes needed to reproduce the same case.

## Corpus categories

| Category | Boundary | Durable fixture examples | Validation boundary |
|---|---|---|---|
| `parser` | Lexer and AST rejection | Unterminated strings, invalid operators, unclosed groups, malformed annotation | `tokenize_with_spans` and `ast::parse_program` under repeated panic-free replay |
| `json` | Tagged JSON conversion | Unknown variants, missing payloads, invalid variant types, nested extras | `serde_json` decoding followed by `json_to_value` |
| `lockfile` | Resolved lockfile parsing | Unsupported versions, missing fields, invalid escapes, traversal-like names | `parse_resolved_lockfile` |
| `registry` | Registry index parsing | Malformed JSON, missing packages, duplicate packages, traversal-like package names | `parse_index_bytes` |
| `memory` | Bounded value graph validation | Values exceeding the node budget | `Value::validate_memory_limits` |
| `async` | Deterministic scheduler budget | Zero, one, and two poll budgets | `AsyncRuntime::run_with_budget` |

Fixtures are stored under [`corpus/p1-05`](../corpus/p1-05) and are intentionally small, reviewable text files. Each fixture has one owner category and should not be silently replaced by generated ephemeral input.

## Fixed-seed replay

The shared native replay helper reads `ZAP_CORPUS_SEED`, defaults to `20260821`, and applies a deterministic Fisher–Yates-style permutation to each category. A valid seed must be a positive decimal integer. Repeating a run with the same seed and checkout preserves fixture order and outcome; changing the seed changes only the replay order, not the fixture contents or assertions.

The local entrypoint is:

```text
ZAP_CORPUS_SEED=20260821 scripts/test_p105_replay.sh
```

The script writes `target/p105-replay.log`. Each record contains the seed, category, relative fixture path, SHA-256 digest, and base64-encoded input bytes. This is the minimum evidence needed to replay a CI failure without depending on mutable temporary files.

## CI and failure-corpus policy

`scripts/test_p105_layers.sh` runs the replay gate before the existing CLI mutation corpus. GitHub Actions supplies the documented seed and uploads `target/p105-replay.log` as the `zap-p105-replay-<commit>` artifact. A failure report must include the commit, seed, category, fixture path, digest, and the smallest durable fixture that reproduces the failure.

New security, parser, memory, or async regressions must add a fixture in the owning category, a focused assertion in the replay test or an adjacent domain test, and a bilingual changelog entry when the public contract changes. Fixtures must not contain secrets, host-specific absolute paths, timestamps, memory addresses, or nondeterministic network data.

## Boundaries and deferred scope

This slice provides deterministic bounded replay and durable regression inputs; it does not claim an unbounded fuzzing service, allocator-level telemetry, arbitrary-cycle reclamation, or language-level async cancellation. Those remain separately owned roadmap items. The replay layer is deliberately compatible with the current Rust 1.75 toolchain and does not add a third-party fuzzing runtime.

## Acceptance evidence

The milestone is accepted when the six categories load from the repository, the same seed produces identical repeated results, alternate seeds produce a different deterministic order, malformed inputs remain panic-free, the replay log contains input evidence, the script is included in the P1-05 layer runner, and the full repository quality gates pass.
