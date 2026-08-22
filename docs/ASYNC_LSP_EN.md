# Zap Async Runtime and LSP Integration

## Status

Zap P2 now includes a deterministic async language layer, a bounded threaded I/O adapter, and an editor protocol foundation. The deterministic executor remains stable-Rust compatible, while the language supports `async fn`, context-owned `ScheduledFuture` values, and `await` expressions. The LSP server provides JSON-RPC initialization, document synchronization, diagnostics, parser-backed hover, semantic rename edits, and context-aware completion. M3-LSP-01 keeps these editor surfaces aligned with the canonical AST, lexer spans, async facade, and standard-library catalog.

The implementation separates deterministic language scheduling from production-oriented blocking adapters. An async call schedules its result in the caller's `RuntimeState` and returns a context-owned `ScheduledFuture`; `await` drives the executor and unwraps that value. `delay_ticks`, `yield_now`, poll budgets, and runtime task limits provide deterministic scheduling controls, while `CancellationToken`, `Cancellable`, and the language `task_cancel` API provide cooperative cancellation. `ThreadedRuntime` supplies a bounded fixed worker set for explicitly submitted blocking work and asynchronous file reads; it does not replace the deterministic language executor.

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
| `ThreadedRuntime::spawn_blocking(task)` | Run a `Send + 'static` blocking adapter on a bounded worker set and return a wakeable join handle. |
| `ThreadedRuntime::read_file_async(path)` | Read one regular file asynchronously with a configured byte limit and typed worker-join errors. |
| `ThreadedRuntime::tcp_exchange(address, request)` | Perform one bounded non-blocking TCP request/response exchange with a response-byte cap and deadline. |
| `ThreadedRuntime::process_async(command, arguments)` | Run a process asynchronously with bounded stdout/stderr capture, a hard deadline, and structured output. |
| `spawn(future)` | Language-level facade that preserves or schedules a context-owned `ScheduledFuture`. |
| `task_join(value)` | Drive the context executor and consume a language-level task result. |
| `task_is_ready(value)` | Check whether a language-level task value is ready without consuming or polling it. |
| `task_cancel(value)` | Request cooperative cancellation for a pending language task and return whether it was accepted. |
| `task_join_timeout(value, poll_budget)` | Drive at most the supplied poll budget and report `TimedOut` if the task remains pending. |

The deterministic executor has no external runtime dependency. `RuntimeLimits` bounds task count and polls per run, and `RunReport` exposes the number of polls and remaining tasks. The separate `ThreadedRuntime` uses only Rust's standard library: `ThreadRuntimeLimits` bounds worker count, admitted tasks, and maximum file-read bytes. Worker panics become `ThreadJoinError::WorkerPanicked`, queue admission is rejected at the task limit, and completed workers wake their joiners. File reads require regular files, reject directories and other non-files, and never read beyond the configured byte limit. Cooperative cancellation remains the default for deterministic tasks, while process adapters provide explicit child termination on cancellation or deadline; arbitrary foreign blocking calls remain outside the safe cancellation contract.

## Production I/O and Multi-thread Scheduling

The native runtime exposes a bounded threaded adapter for blocking operations that must not run on the deterministic language executor. `ThreadedRuntime::new(ThreadRuntimeLimits { max_workers, max_tasks, max_read_bytes })` starts a fixed worker set. `spawn_blocking` admits only up to `max_tasks` active jobs and returns a `ThreadJoinHandle`; the handle is a `Future` that is woken when the worker finishes. A panic in a worker is converted to `WorkerPanicked` rather than crossing the runtime boundary.

`read_file_async` is the regular-file production I/O facade. `tcp_exchange(address, request)` adds a bounded TCP request/response adapter: address resolution and connection are deadline-bounded, the stream is switched to non-blocking mode before I/O, writes and reads yield while the socket would block, and the response is rejected once `max_socket_bytes` is exceeded. `process_async(command, arguments)` runs a child on the worker set with null stdin, concurrently drained stdout/stderr, a hard deadline, and per-stream output caps. `process_async_cancellable(command, arguments, token)` additionally terminates the child through the platform process API when the cancellation token is triggered, then joins the reaped child and drains bounded output before resolving. Both APIs return an outer scheduler result and an inner adapter result so admission failures remain distinct from I/O or process failures. Forced cancellation is limited to Zap-owned child processes; arbitrary foreign blocking calls and OS-level sandboxing remain outside this adapter contract.

## Async Language Syntax

Declare an asynchronous function by placing `async` before `fn`. The function call returns a context-owned `ScheduledFuture` rather than the plain result. Use `await` to drive the deterministic executor and obtain the completed result:

```zap
async fn load_version() -> number:
    return 7

let pending = load_version()
let version: number = await pending
say version
```

An `async` function may use the same parameter and return-type annotations as an ordinary function. The evaluator preserves the declaration flag on the runtime function, validates the declared result, and schedules it through the caller's `RuntimeState`.

`await` is an expression and may be used in a declaration, assignment, return expression, or nested call where an expression is accepted:

```zap
async fn answer() -> number:
    return 42

let value = await answer()
say value + 1
```

The current deterministic model has no background threads: a `ScheduledFuture` is a stable runtime value containing a per-run task ID, and `await` or `task_join` drives the context executor before consuming its result. `spawn(async_call())` preserves the scheduled handle, while `task_is_ready` checks readiness without consuming or polling it. `task_cancel` requests cooperative cancellation, and `task_join_timeout` bounds executor polling with a deterministic `TimedOut` failure. Awaiting or joining a non-Future value is rejected with a runtime error instead of silently changing the value.

```zap
async fn answer() -> number:
    return 42

let task = spawn(answer())
let ready = task_is_ready(task)
let value = task_join(task)
say value
```

## LSP Server

Start the editor server with:

```bash
zap lsp
```

The server communicates over standard input and output using JSON-RPC messages framed with `Content-Length` headers. `initialize` advertises `textDocumentSync` with `openClose: true` and `change: 1`, meaning clients must send full-document changes through `params.contentChanges`; the server does not read a non-standard `textDocument.text` field from `didChange`.

| Message | Behavior |
|---|---|
| `initialize` | Returns Zap server information and advertises text synchronization, completion, diagnostics, hover, definition, rename, and workspace-symbol capabilities. |
| `shutdown` | Returns a successful null result. |
| `textDocument/didOpen` | Stores the document text and optional monotonically increasing version, then publishes lint diagnostics with deterministic source ranges. |
| `textDocument/didChange` | Uses the standard `params.contentChanges` payload for the advertised full-sync mode, replaces the stored document from the final full-text change, rejects stale/unversioned updates after a versioned open, and publishes diagnostics from the accepted text. Range-based incremental changes are rejected safely until a position-aware incremental mode is implemented. |
| `textDocument/didClose` | Removes the document from the per-session workspace index without affecting other LSP sessions. |
| `textDocument/completion` | Filters deterministic language keywords, all cataloged standard-library builtins, and top-level `let` and function declarations by the active source prefix. |
| `textDocument/hover` | Parses the stored document and reports parser-owned metadata for top-level functions, classes, and declarations; async builtins expose stable scheduling documentation. |
| `textDocument/definition` | Resolves a referenced top-level declaration to its parser-span source range. |
| `textDocument/rename` | Produces a deterministic file-local `WorkspaceEdit` for the selected lexical declaration and its bound references, including parameters, closures, shadowed scopes, and import aliases; it protects strings/comments and rejects invalid names, keywords, and standard-library builtins. Cross-file rename is not advertised as supported. |
| `workspace/symbol` | Searches indexed in-memory documents and safely follows explicit local imports to discover deterministic symbols from package modules that are not open in the editor. |
| `textDocument/formatting` | Returns one deterministic full-document edit that normalizes line endings, tabs, trailing spaces, and the final newline. |

A minimal initialize request is:

```text
Content-Length: 67\r\n
\r\n
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
```

Completion is context-aware rather than a fixed unfiltered list. It combines language keywords, the machine-readable standard-library catalog, and declarations discovered in the active document. For example, after typing `lo` in a document containing `async fn load():`, the completion response includes `load` as a function item. Hover uses the source position to identify the active word and the parser’s `SourceSpan`-carrying AST to describe the matching declaration; `spawn`, `task_join`, `task_is_ready`, `task_cancel`, `task_join_timeout`, and `async_capabilities` expose matching async-boundary text. Signature help provides the same stable parameter labels for these builtins.

Workspace symbol indexing follows explicit local imports such as `import app.util as util` from the opened file’s directory and maps the dotted path to `app/util.zp`. Imported files are canonicalized before indexing, must remain under the importing directory, and are bounded to 8 MiB. Invalid, missing, oversized, unreadable, or traversal-like modules are skipped deterministically rather than becoming an editor or filesystem escape. Discovered module URIs are inserted into the same sorted index as open documents, so nested imports are traversed once and results remain stable. Indexing is bounded to 256 documents, 32 levels of import depth, and 32 MiB of aggregate in-memory source text; over-limit documents/modules are skipped without evicting accepted open buffers.

Rename uses a file-local lexical binding model rather than replacing every same-spelling token. Function, class, module, `let`, `for`, `catch`, parameter, and `import ... as alias` declarations receive binding identities; references resolve through nested scopes to the nearest visible declaration. A selected outer binding does not rename an inner shadow, and strings, comments, module-path segments, keywords, and builtins are excluded. Cross-file edits remain outside the current contract and return only edits for the active URI.

Diagnostics continue to reuse Zap’s existing lint implementation. This keeps command-line and editor diagnostics aligned. Accepted document versions are monotonic within an LSP session; stale or unversioned changes after a versioned document are ignored without replacing the last known-good buffer. A full-sync change publishes diagnostics from the new `contentChanges` text, so completion, hover, definition, symbols, formatting, and rename observe the same accepted in-memory document. Incremental range edits are intentionally rejected rather than applied approximately. The server negotiates `utf-8`, `utf-16`, or `utf-32` from `initialize.params.capabilities.general.positionEncodings`, preferring the first supported encoding and defaulting to UTF-16; all inbound cursor positions and outbound diagnostic, symbol, formatting, and rename ranges use that encoding. URI handling percent-decodes safely, rejects malformed escapes, URI hosts, NUL bytes, and decoded traversal segments, and canonicalizes local paths before import containment checks. When a lint message reports a source line, the server maps it to a zero-based LSP range spanning that line’s encoded width; diagnostics without a parsed line use the first line as a deterministic fallback.

## Tooling Synchronization

The formatter and LSP now share the finalized async vocabulary. Completion advertises every cataloged public builtin with its domain as deterministic detail, including `spawn`, `task_join`, `task_is_ready`, `task_cancel`, and `task_join_timeout`. The VS Code TextMate grammar highlights the same catalog vocabulary as callable Zap functions. The editor parity validation script parses the grammar and rejects a package when any catalog builtin or async keyword is missing, preventing drift between the language facade, catalog, and editor assets.

## Production Deployment Boundaries

The repository now includes a reproducible deployment reference layer for the authenticated registry service. `deploy/zap-registry.service` defines supervised Linux execution with a dynamic least-privilege user, protected filesystem paths, explicit writable storage, memory/CPU/task/file quotas, and loopback-only network access. `deploy/zap-registry.nginx.conf` defines TLS 1.2/1.3 termination, HTTP-to-HTTPS redirect, bounded request bodies and proxy timeouts, restricted methods, and a loopback upstream. `deploy/registry.env.example` contains placeholders only, while `deploy/registry-deployment-policy.toml` records the credential, sandbox, quota, and egress contract. Run `scripts/validate_registry_deployment.sh` before installation; the same gate runs in CI.

The bounded production I/O adapter and multi-thread scheduler now cover regular-file reads, bounded non-blocking TCP exchange, bounded asynchronous process execution, cancellation-aware child termination, and explicitly submitted blocking tasks. The authenticated loopback registry service now supports bounded requests, bearer authentication, safe in-root GET paths, signed-index persistence, managed shutdown, and deterministic failure responses. Repository-side production boundaries are implemented and tested. Public deployment remains platform-specific: operators must provision real TLS certificates, DNS, ingress/WAF/rate limiting, external service supervision, OS-native sandbox equivalents, monitoring, and any explicitly reviewed egress allowlist. Arbitrary foreign blocking-call cancellation remains outside the safe runtime contract.

For the package workflow, see the [English package guide](PACKAGE_EN.md) and [P2 progress](P2_PROGRESS.md). For the Burmese version of this guide, see [ASYNC_LSP_MM.md](ASYNC_LSP_MM.md).
