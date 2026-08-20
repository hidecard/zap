# Zap Async Runtime and LSP Integration

## Status

Zap P2 now includes a deterministic async language layer and an editor protocol foundation. The runtime remains single-threaded and stable-Rust compatible, while the language supports `async fn`, deferred `Future` values, and `await` expressions. The LSP server provides JSON-RPC initialization, document synchronization, diagnostics, parser-backed hover, and context-aware completion.

The implementation deliberately keeps scheduling deterministic. An async call currently evaluates its body to a completed `Future` value, and `await` unwraps that value during evaluation. `delay_ticks`, `yield_now`, poll budgets, and runtime task limits provide deterministic scheduling controls, while `CancellationToken` and `Cancellable` provide cooperative cancellation without worker threads or wall-clock dependence. Multi-thread scheduling and external I/O remain outside this foundation.

## Async Runtime

The native runtime exposes three deterministic executor operations:

| Operation | Purpose |
|---|---|
| `spawn(future)` | Add an async task to the deterministic task queue. |
| `spawn_cancellable(future)` | Add a task controlled by a returned `CancellationToken`. |
| `run_until_idle()` | Poll tasks in spawn order until the current queue pass is idle. |
| `block_on(future)` | Drive one future to completion synchronously. |
| `delay_ticks(n)` | Return a deterministic poll-count delay future. |
| `yield_now()` | Suspend once and resume on the next deterministic poll. |
| `spawn_limited(future)` | Enforce the configured maximum task count. |
| `run_with_budget(n)` | Poll at most `n` times and return a deterministic `RunReport`. |

The executor avoids worker threads and external runtime dependencies. `RuntimeLimits` bounds task count and polls per run, and `RunReport` exposes the number of polls and remaining tasks. This provides a stable base for future I/O integrations without changing existing synchronous behavior. Cancellation is cooperative: a cancelled task completes without polling its inner future.

## Async Language Syntax

Declare an asynchronous function by placing `async` before `fn`. The function call returns a `Future` value rather than the plain result. Use `await` to obtain the completed result:

```zap
async fn load_version() -> number:
    return 7

let pending = load_version()
let version: number = await pending
say version
```

An `async` function may use the same parameter and return-type annotations as an ordinary function. The evaluator preserves the declaration flag on the runtime function and validates the declared result before wrapping it in a `Future`.

`await` is an expression and may be used in a declaration, assignment, return expression, or nested call where an expression is accepted:

```zap
async fn answer() -> number:
    return 42

let value = await answer()
say value + 1
```

The current deterministic model has no background threads: a `Future` is a stable runtime value containing the completed result, and `await` unwraps it. Awaiting a non-Future value is rejected with a runtime error instead of silently changing the value.

## LSP Server

Start the editor server with:

```bash
zap lsp
```

The server communicates over standard input and output using JSON-RPC messages framed with `Content-Length` headers.

| Message | Behavior |
|---|---|
| `initialize` | Returns Zap server information and advertises text synchronization, completion, diagnostics, hover, definition, and workspace-symbol capabilities. |
| `shutdown` | Returns a successful null result. |
| `textDocument/didOpen` | Stores the document and publishes lint diagnostics with deterministic source ranges. |
| `textDocument/didChange` | Replaces the stored document and publishes updated diagnostics. |
| `textDocument/completion` | Filters deterministic keywords by the active source prefix and adds top-level `let` and function declarations from the document. |
| `textDocument/hover` | Parses the stored document and reports parser-owned metadata for top-level functions, classes, and declarations. |
| `textDocument/definition` | Resolves a referenced top-level declaration to its parser-span source range. |
| `workspace/symbol` | Searches all indexed in-memory documents for deterministic top-level declaration symbols. |
| `textDocument/formatting` | Returns one deterministic full-document edit that normalizes line endings, tabs, trailing spaces, and the final newline. |

A minimal initialize request is:

```text
Content-Length: 67\r\n
\r\n
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
```

Completion is context-aware rather than a fixed unfiltered list. For example, after typing `lo` in a document containing `async fn load():`, the completion response includes `load` as a function item. Hover uses the source position to identify the active word and the parser’s `SourceSpan`-carrying AST to describe the matching declaration.

Diagnostics continue to reuse Zap’s existing lint implementation. This keeps command-line and editor diagnostics aligned. When a lint message reports a source line, the server maps it to a zero-based LSP range spanning that line’s character width; diagnostics without a parsed line use the first line as a deterministic fallback.

## Remaining P2 Boundary

The completed foundation does not yet claim a production asynchronous I/O runtime or multi-thread scheduler. Remaining boundaries are external I/O integration, richer nested-symbol indexing, module-aware package indexing, and a network registry service deployment. Signed index verification, deterministic cache garbage collection, authenticated local registry persistence, runtime resource limits, one-poll suspension, formatting, definitions, and workspace symbols are implemented and tested.

For the package workflow, see the [English package guide](PACKAGE_EN.md) and [P2 progress](P2_PROGRESS.md). For the Burmese version of this guide, see [ASYNC_LSP_MM.md](ASYNC_LSP_MM.md).
