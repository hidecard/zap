# Zap Language Guide

> **Audience:** This is the complete path from installing Zap for the first time to writing structured, typed, modular, asynchronous, tested, and Web-enabled programs.

**Verified baseline:** Zap v2.11.16
**Source extension:** `.zp`
**Runtime:** standalone native `zap` executable
**Navigation:** [Documentation hub](DOCUMENTATION_NAVIGATION_EN.md) · [Language specification](LANGUAGE_SPEC_EN.md) · [Syntax reference](SYNTAX_GUIDE_EN.md) · [Standard-library index](STDLIB_INDEX_EN.md) · [Burmese guide](LEARN_ZAP_MM.md)

## 1. What Zap Is

Zap is a small, readable, general-purpose language with indentation-based blocks, explicit modules, optional type annotations, structured `Result` and `Option` values, a native command-line runtime, deterministic project validation, and a practical path from scripts to Web applications. The normal execution pipeline is **source → lexer → AST parser → evaluator**. A Zap project does not need Python, Node.js, Java, or Rust installed on the machine where the compiled Zap executable runs.[1]

The Rust toolchain is used to build Zap itself. It is not a runtime dependency of a `.zp` application. JavaScript frameworks such as React, Vue, Svelte, or Alpine may be used as optional build-time tools; their emitted HTML, CSS, JavaScript, and other assets can be placed under `public/` and served by Zap.[2]

This guide distinguishes three kinds of information. **Normative** behavior is defined by the language specification and executable tests. **Compatibility** behavior exists for older projects, such as legacy `use` imports. **Deferred** behavior is designed but not enabled, such as user-defined traits and a production asynchronous I/O reactor. When this guide says that a feature is deferred, do not write a program that depends on it yet.

## 2. Install Zap

### 2.1 Linux and macOS

Download the archive for your operating system and architecture from the [GitHub Releases page](https://github.com/hidecard/zap/releases). Extract it, place the executable in a directory on `PATH`, and make it executable on Unix-like systems.

```bash
tar -xzf zap-2.11.16-linux-x86_64.tar.gz
sudo install -m 0755 zap/bin/zap /usr/local/bin/zap
zap --version
zap --help
```

For macOS ARM64, use the macOS ARM64 archive and install its `bin/zap` executable in the same way. If you do not have administrator access, place the executable in `~/.local/bin` and add that directory to `PATH`.

```bash
mkdir -p "$HOME/.local/bin"
install -m 0755 zap/bin/zap "$HOME/.local/bin/zap"
export PATH="$HOME/.local/bin:$PATH"
zap --version
```

### 2.2 Windows

Download the Windows archive, extract it, and either run `bin\zap.exe` directly or add the directory containing `zap.exe` to `PATH`.

```bat
bin\zap.exe --version
bin\zap.exe --help
```

### 2.3 Verify the installation

A successful installation prints the installed runtime version and the supported command list. The first diagnostic command to remember is:

```bash
zap --version
zap --help
```

If the shell reports that `zap` cannot be found, the executable is not on `PATH`. If the version is older than the project documentation, update the executable before relying on newer commands or language behavior.

## 3. Your First Program

Create `hello.zp`:

```zap
say "Hello from Zap"
```

Run it directly:

```bash
zap hello.zp
```

The explicit form is equivalent:

```bash
zap run hello.zp
```

Expected output:

```text
Hello from Zap
```

`say` writes a value to the terminal. Statements are normally separated by new lines. Zap does not require semicolons for ordinary statements.

Comments begin with `#` and continue to the end of the line:

```zap
# This comment is ignored.
say "This statement runs"
```

Blocks begin after a colon and are delimited by indentation. Four spaces per level are recommended:

```zap
if true:
    say "inside the block"
say "outside the block"
```

Do not mix indentation styles within one block. Parser diagnostics identify malformed indentation and missing block bodies instead of allowing an uncontrolled runtime failure.

## 4. Create a Project with One Command

For a structured Web project, use Zap’s single canonical generator:

```bash
zap new my_app
cd my_app
```

The command creates a complete user-managed project. There is intentionally no Django-style `startapp` command and no hidden application registry. After generation, you own the source files and may add, remove, rename, or reorganize modules directly.

```text
my_app/
├── zap.toml
├── zap.lock
├── main.zp
├── web.zp
├── server.zp
├── models/
│   └── user.zp
├── functions/
│   └── user_functions.zp
├── ui/
│   └── ui.zp
├── routes/
│   └── routes.zp
├── middleware/
│   └── middleware.zp
├── migrations/
│   └── 0001_initial.zp
├── admin/
│   └── admin.zp
├── public/
│   ├── index.html
│   └── assets/
│       ├── app.css
│       └── app.js
└── tests/
    └── web_test.zp
```

The directories are conventions backed by the Web manifest and validators. `models/` is for data shape and validation metadata, `functions/` is for business logic and use cases, `ui/` is for browser-facing UI metadata, `routes/` owns HTTP route declarations, `middleware/` owns request/response policy, `migrations/` owns schema changes, `admin/` owns explicit administration registrations, and `public/` owns browser assets. You can create additional files under these directories whenever your application needs them.

Run the generated project without installing another language runtime:

```bash
zap check
zap build --locked
zap test tests
zap dev
```

`zap dev` starts the bounded native development server declared by `server.zp`. It is a development/reference server, not a claim that all production Web concerns are solved.

## 5. Values and Variables

Zap’s core values are text, integer numbers, booleans, lists, maps, objects, functions, and `none`. Typed `result<T>` and `option<T>` values are available at the checked boundary.

```zap
let language = "Zap"
let version = 2
let ready = true
let empty = none
let tools = ["parser", "runtime", "lsp"]
let user = {"name": "Developer", "active": true}

say language
say version
say ready
say tools[0]
say user["name"]
```

Use `let` for a new binding and ordinary assignment for an existing binding:

```zap
let count = 1
count = count + 1
say count
```

A binding annotation documents and checks the expected type:

```zap
let name: text = "Zap"
let port: number = 8080
let enabled: bool = true
let tags: list<text> = ["language", "runtime"]
```

This is invalid because the value is text, not a number:

```zap
let port: number = "8080"
```

Use `zap check` before running a project. The checker catches known annotation, argument, return-value, and collection-element mismatches before execution where sufficient information is available.[3]

## 6. Operators and Expressions

The main operators are:

| Operators | Meaning |
|---|---|
| `+`, `-`, `*`, `/`, `%` | Arithmetic; `+` also joins text where applicable |
| `==`, `!=`, `<`, `<=`, `>`, `>=` | Comparison |
| `and`, `or`, `not` | Boolean logic with short-circuiting |
| `(...)` | Grouping |
| `[]` | List/map indexing |
| `.` | Member access and method calls |

```zap
let total = (10 + 5) * 2
let remainder = 17 % 4
let message = "total=" + str(total)
let allowed = total >= 20 and not false

say total
say remainder
say message
say allowed
```

Calls, indexing, and member access bind most strongly. Unary `-` and `not` bind before arithmetic. Multiplication and division bind before addition. Comparisons bind before `and`, and `and` binds before `or`. Use parentheses when a reader might have to guess your intent.

Boolean operators short-circuit:

```zap
let enabled = false
if enabled and expensive_check():
    say "this branch is not reached"
```

Integer overflow, division by zero, invalid indexing, and invalid member access are checked runtime failures. They are not silently converted to unrelated values.

## 7. Conditions and Loops

Use `if` and `else`:

```zap
let score = 85

if score >= 80:
    say "Excellent"
else:
    say "Keep practising"
```

`for` iterates over a list or range. `while` repeats while its condition is true:

```zap
for item in ["web", "ai", "iot"]:
    say item

for number in range(3):
    say number

let count = 0
while count < 3:
    say count
    count = count + 1
```

Use `break` to stop a loop and `continue` to skip the rest of the current iteration:

```zap
for number in range(10):
    if number == 2:
        continue
    if number == 5:
        break
    say number
```

Zap applies bounded loop and execution limits. A loop that never terminates is not a valid production strategy; redesign it around a finite collection, a bounded counter, or an explicit external service boundary.[4]

## 8. Functions

Declare functions with `fn`. Parameters and return values may be annotated:

```zap
fn add(a: number, b: number) -> number:
    return a + b

fn greet(name: text) -> text:
    return "Hello, " + name

say add(4, 6)
say greet("Zap")
```

A function without an explicit return produces `none`. A function name is a first-class callable value, so it may be assigned, passed, returned, and invoked through an alias:

```zap
fn double(value: number) -> number:
    return value * 2

let operation = double
say operation(7)
```

### 8.1 Default and named arguments

A parameter can provide a default value:

```zap
fn greet(name: text = "World", punctuation: text = "!") -> text:
    return "Hello, " + name + punctuation

say greet()
say greet("Zap", ".")
```

Named arguments are useful when a function has several optional parameters:

```zap
fn connect(host: text, port: number = 8080, secure: bool = true):
    return {"host": host, "port": port, "secure": secure}

let local = connect("localhost", secure = false)
say local["port"]
```

Required parameters must be supplied. Duplicate, unknown, or multiply supplied arguments are errors. Defaults are evaluated when the corresponding argument is omitted.[5]

### 8.2 Closures

A nested function can read a binding from its enclosing function:

```zap
fn make_greeting(prefix: text):
    fn greet(name: text) -> text:
        return prefix + ", " + name
    return greet

let say_hello = make_greeting("Hello")
say say_hello("Developer")
```

Closures use parent-linked lexical frames. Keep captured state small and explicit. Object and callable cycles are subject to the runtime’s bounded memory and explicit cycle policy rather than an automatic garbage collector.[6]

## 9. Lists, Maps, and JSON

List indexes start at zero. Maps use string or compatible keys and bracket lookup:

```zap
let languages = ["Zap", "Rust", "Go"]
say languages[0]
say len(languages)

let profile = {"name": "Zap User", "age": 20, "active": true}
say profile["name"]
say keys(profile)
```

Common collection helpers include `len`, `contains`, `get`, `is_empty`, `sum`, `reverse`, `sort`, `join`, `keys`, `entries`, `enumerate`, and `count`.

JSON conversion is explicit:

```zap
let profile = {"name": "Zap", "active": true}
let encoded = json(profile)
let decoded = from_json(encoded)

say encoded
say decoded["name"]
```

For typed payload checks, use a generic annotation:

```zap
let scores: list<number> = [10, 20, 30]
let response: map<text, number> = {"status": 200}
```

JSON serialization is bounded and cycle-safe. A cyclic object graph cannot be serialized as an infinite structure. Callable values serialize to a deterministic marker but are intentionally not deserializable as executable code.[7]

## 10. Classes and Objects

Zap supports classes, fields, methods, initialization, and inheritance:

```zap
class User:
    fn init(self, name: text):
        self.name = name

    fn greet(self) -> text:
        return "Hello, " + self.name

let user = new("User", "Zap")
say user.greet()
```

Inheritance uses `extends`:

```zap
class Animal:
    fn speak(self) -> text:
        return "sound"

class Dog extends Animal:
    fn speak(self) -> text:
        return "woof"

let dog = new("Dog")
say dog.speak()
```

Constructors and inherited initialization must follow the current constructor contract. Use explicit fields and methods instead of relying on undocumented dynamic properties. Object fields are managed through the runtime’s checked object boundary; invalid field access is a typed error rather than a Rust panic.

Traits and composition have a reviewed design direction but are not enabled as a complete user-defined syntax in the current release. Prefer classes, modules, functions, and explicit composition until the language specification enables that feature.[8]

## 11. Modules, Imports, and Workspaces

The simplest compatibility import loads a local file by path:

```zap
# modules/math.zp
export fn square(value: number) -> number:
    return value * value
```

```zap
# main.zp
import "modules/math"

say square(5)
```

For workspace-oriented code, use explicit module declarations and aliases:

```zap
# modules/app/core.zp
module app.core

export fn version() -> text:
    return "2.0"
```

```zap
# main.zp
module app.main
import app.core as core

say core.version()
```

Only exported symbols should be part of another module’s public surface. The resolver searches bounded project locations, rejects absolute and traversal paths, detects circular imports, and visits imported modules deterministically. New libraries should prefer explicit `module`, `import`, and `export` forms; legacy `use` imports remain for compatibility.[9]

A module root can be declared in `zap.toml`:

```toml
[package]
name = "workspace-demo"
version = "0.1.0"
main = "main.zp"

[module]
root = "modules"
entries = ["app/core.zp"]
```

Module entries must be relative, must end in `.zp`, and must exist. Treat module boundaries as API boundaries: export the smallest useful surface and keep helpers private.

## 12. Result and Option

`Result` represents success or failure. `Option` represents presence or absence:

```zap
let success = ok(42)
let failure = err("not found")
let present = some("Zap")
let missing = option_none()

say is_ok(success)
say is_err(failure)
say is_some(present)
say unwrap_or(failure, 0)
say unwrap_or(missing, "unknown")
```

Use `unwrap` only when failure is impossible or intentionally fatal. Prefer `unwrap_or` when a safe fallback exists.

Typed payload annotations make the expected value explicit:

```zap
let answer: result<number> = ok(42)
let failure: result<text> = err("not found")
let user: option<text> = some("Zap")
let missing: option<number> = option_none()
```

The `?` operator propagates an error Result from the current function:

```zap
fn load_user() -> result<map>:
    return err("user not found")

fn profile() -> result<map>:
    let user = load_user()?
    return ok(user)

say profile()
```

`ok(value)?` continues with `value`. `err(error)?` returns the error immediately. Applying `?` to a non-Result is invalid. Use Result for expected operational failure and reserve `raise` for exceptional control flow at a clear boundary.

## 13. Errors and Diagnostics

Zap reports failures through a structured diagnostic boundary. Depending on the failure, a diagnostic may classify the problem as `SyntaxError`, `NameError`, `TypeError`, `ValueError`, `IOError`, `FileNotFound`, `PermissionError`, `OverflowError`, `Error`, or `ProjectError`.

Human-readable diagnostics are useful during development:

```bash
zap check .
zap build .
```

Automation and editors should use JSON diagnostics:

```bash
zap check --json .
```

A JSON diagnostic contains structured fields such as `ok`, `kind`, `file`, `line`, `column`, `message`, and the formatted `error` string when source information is available:

```json
{"ok":false,"kind":"TypeError","file":"main.zp","line":4,"column":12,"message":"expected number, got text"}
```

The command-line checker and LSP share the same semantic diagnostic categories. Preserve the original message when handling errors, and include a stable context such as the operation, file, route, or package name.

## 14. Standard Library Essentials

The public standard library is organized into deterministic domains including `text`, `math`, `collections`, `filesystem`, `json`, `system`, `time`, `logging`, `runtime`, `async`, `network`, and `process`.[10]

### 14.1 Text and math

```zap
let raw = "  Zap Language  "
let clean = trim(raw)
let words = split(clean, " ")
say upper(clean)
say join(words, "-")
say abs(-7)
say min(3, 8)
say max(3, 8)
say pow(2, 3)
```

### 14.2 Files and paths

```zap
let path = path_join("data", "note.txt")

if exists(path):
    say read_text(path)
else:
    write_text(path, "Created by Zap")

let lines = read_lines(path)
write_lines(path, lines)
say file_metadata(path)
```

Keep file paths inside a known project root when writing application code. Validate user-provided names before combining them with `path_join`. The Web asset builtins apply their own root confinement and extension policy.

### 14.3 Environment and time

```zap
if has_env("ZAP_ENV"):
    say env("ZAP_ENV")
else:
    say "development"

let started = utc_now()
sleep(1)
let elapsed = duration_between(started, utc_now())
say elapsed
```

Environment variables are external input. Do not expose secrets in diagnostics, JSON responses, logs, or source-control files.

### 14.4 HTTP and processes

Network and process operations are bounded capabilities. Validate destinations, set timeouts, handle non-success responses, and avoid building shell commands from untrusted strings.

```zap
let response = http_get("https://example.com")
say response
```

The exact response map and environment policy are defined by the current standard-library and deployment contracts. Do not assume that the development/reference server is a production reverse proxy, TLS terminator, or process supervisor.

## 15. Test, Format, Lint, and Build

Test files conventionally end in `_test.zp` and use `assert`:

```zap
fn add(a: number, b: number) -> number:
    return a + b

assert(add(2, 3) == 5, "addition failed")
assert(type(add(2, 3)) == "number", "wrong result type")
say "test passed"
```

Run tests from a project root:

```bash
zap test
zap test tests
zap test tests --filter arithmetic
zap test tests --fail-fast
zap test tests --json
```

A failing test returns exit code `1`. A command-usage error returns exit code `2`. Use `--json` when a CI system needs machine-readable test results.

Format and lint source files:

```bash
zap fmt main.zp
zap lint main.zp
```

Validate project structure and build readiness:

```bash
zap check .
zap check --json .
zap build .
zap build --locked .
```

`zap build --locked` requires a valid canonical lockfile. Use it in CI when you want the dependency graph to be reproducible.

## 16. Project Manifest and Dependencies

A minimal project manifest is:

```toml
[package]
name = "hello-app"
version = "0.1.0"
main = "main.zp"
```

The Web scaffold adds constrained `[web]`, `[frontend]`, and `[database]` sections:

```toml
[web]
routes = "routes/routes.zp"
models = "models"
middleware = "middleware/middleware.zp"
migrations = "migrations"
assets = "public"
admin = "admin/admin.zp"
server = "server.zp"
serialization = "json-by-default"

[frontend]
framework = "plain"
output = "public"
spa_fallback = "index.html"

[database]
driver = "sqlite"
url = "data/zap.sqlite3"
```

For a dependency-free project, `zap.lock` records the package identity and an empty dependency section. Manage dependencies with the CLI rather than editing generated lockfile output by hand:

```bash
zap add json-tools 1.2
zap lock
zap install
zap update
```

`zap add` changes the manifest and invalidates an old lockfile. `zap lock` generates canonical lock data. `zap install` validates the manifest, lockfile, and available registry cache without changing project intent. `zap update` regenerates the lockfile after a deliberate manifest change. Commit `zap.toml` and `zap.lock` together.

Registry-backed projects may use `ZAP_REGISTRY_INDEX`, checksum-verified cache artifacts, signed indexes, and offline mode:

```bash
ZAP_OFFLINE=1 zap install
zap registry check path/to/index.json
zap registry fetch https://registry.example/index.json
```

Package publishing and registry serving have separate authentication, checksum, path, and deployment contracts. Read the [package author guide](PACKAGE_EN.md) and [registry authentication contract](REGISTRY_AUTH_EN.md) before operating a registry.

## 17. Web Development with Zap

The generated Web project separates concerns without hidden magic:

| Directory/file | Responsibility |
|---|---|
| `models/` | Data shape, field metadata, and validation-oriented definitions |
| `functions/` | Business logic, use cases, and request handlers |
| `ui/` | Browser UI metadata and entrypoint contract |
| `routes/` | HTTP route declarations |
| `middleware/` | Ordered request/response policy |
| `migrations/` | Versioned schema intent |
| `admin/` | Explicit administration registrations |
| `public/` | HTML, CSS, JavaScript, images, fonts, and other browser assets |
| `tests/` | Project and HTTP contract tests |

The generated route file can contain API and browser routes:

```zap
export fn routes():
    return [
        {"method": "GET", "path": "/", "handler": "home", "scope": ""},
        {"method": "GET", "path": "/api/tasks", "handler": "tasks", "scope": "tasks:read"},
        {"method": "GET", "path": "/assets/*path", "handler": "asset", "scope": ""},
        {"method": "GET", "path": "/*path", "handler": "spa", "scope": ""}
    ]
```

A handler can return a JSON response:

```zap
export fn tasks(request):
    return {"status": 200, "body": json({"tasks": [], "request_id": request["request_id"]})}
```

For JSON request bodies, `web_validate_request(body, schema)` accepts raw JSON text or a parsed map and returns `ResultOk` with only declared fields. Missing fields, unknown fields, wrong types, invalid JSON, and declared length violations return `ResultErr`; returning that error directly lets the native Web boundary map it to a stable JSON response:

```zap
export fn create_user(request):
    let checked = web_validate_request(request["body"], {"name": {"type": "text", "max_len": 120}, "email": {"type": "text", "max_len": 254}})
    if is_err(checked):
        return checked
    return ok({"status": 201, "body": json({"created": true, "body": unwrap(checked)})})
```

A static asset handler can use the bounded asset builtin:

```zap
export fn asset(request):
    return web_static("assets/" + request["params"]["path"], "public")

export fn spa(request):
    return web_static_spa(request["params"]["path"], "public", "index.html")
```

Keep API and asset routes before the final SPA wildcard. `web_static_spa` serves an existing asset or the validated fallback entry document for a client-side route. Cache fingerprinting, TLS termination, horizontal scaling, and CDN behavior remain deployment concerns.[11]

### 17.1 Plain JavaScript and other frontend frameworks

Plain HTML/CSS/JavaScript requires no frontend package manager:

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <link rel="stylesheet" href="/assets/app.css">
  </head>
  <body>
    <main id="app"></main>
    <script type="module" src="/assets/app.js"></script>
  </body>
</html>
```

React, Vue, Svelte, Alpine, or another frontend system may be used as an optional build-time toolchain. Build its output, copy the output into `public/`, and deploy the generated files with Zap. The deployed runtime does not execute npm, Node.js, a bundler, or the framework compiler.

## 18. Database and Migrations

The Web scaffold is SQLite-first. A migration is an explicit `.zp` declaration:

```zap
export fn migration():
    return {
        "id": "0001_initial",
        "depends_on": [],
        "operations": [
            {
                "kind": "create_table",
                "table": "users",
                "columns": {
                    "id": "integer primary key",
                    "name": "text not null"
                }
            }
        ]
    }
```

Inspect and apply migrations with:

```bash
zap db check
zap db plan
zap db inspect --json
zap db migrate --dry-run
zap db migrate
zap db migrate --check
```

The current implementation is designed for deterministic, additive SQLite operations. Do not treat the present migration command as a provider-neutral production migration platform. Back up production data, review SQL plans, and use the deployment runbook for operational changes.[12]

## 19. Async Programming

Prefix a function declaration with `async` to schedule a deterministic language future:

```zap
async fn load() -> number:
    return 7

let pending = load()
let value: number = await pending
say value
```

The executor is poll-budgeted and owned by the current runtime context:

```zap
async fn answer() -> number:
    return 42

let handle = answer()
say task_is_ready(handle)
say task_join_timeout(handle, 1)
```

Use `task_join` to consume a completed task, `task_cancel` for cooperative cancellation, and `task_join_timeout` to enforce a deterministic poll budget. The current language scheduler does not create a production worker pool or act as a socket-readiness reactor. Blocking I/O, graceful shutdown, foreign worker cancellation, and production concurrency belong to the separate async-boundary and host-adapter contracts.[13]

## 20. LSP and Editor Workflow

Zap includes a stdio Language Server Protocol implementation and a maintained VS Code asset set. The server supports diagnostics, hover, completion, signature help, definitions, document symbols, workspace symbols, formatting, and scope-aware rename within its documented boundaries.

Use the extension assets from `vscode-extension/` or the canonical editor tree under `editors/vscode/`. The LSP advertises bounded incremental synchronization: each change notification may contain up to 128 sequential full-document or range edits, negotiated UTF-8/UTF-16/UTF-32 positions are validated at character boundaries, versions must increase monotonically, and the 32 MiB workspace byte cap is enforced after every edit. Malformed, stale, oversized, out-of-range, or unknown-document range edits are rejected without replacing stored text. Cross-file rename is not yet a complete refactoring feature.

For repository development, validate editor parity with:

```bash
python3 scripts/validate_vscode_assets.py
scripts/test_lsp_semantic_parity.sh
scripts/test_lsp_protocol_sync.sh
```

When an editor reports an error, reproduce it with `zap check --json` first. This separates a language diagnostic from an editor transport or presentation problem.

## 21. Runtime Safety and Advanced Practices

Zap applies bounds to source size, loop iterations, execution depth, collection production, text values, HTTP requests, response bodies, registry transport, and selected task operations. These limits are part of the reliability boundary, not optional performance settings.

The runtime owns a per-run execution context containing module cache, import-cycle tracking, workspace confinement, logical memory accounting, object storage, and async scheduling state. Separate runs should not share user objects, modules, tasks, or diagnostics. Object fields are checked through the runtime borrow boundary. Cyclic object graphs require explicit cycle-breaking policy; there is no public weak-reference API or automatic cycle collector in the current surface.[6]

Advanced code should follow these rules:

1. Validate at every external boundary: files, environment variables, JSON, network destinations, registry metadata, and HTTP requests.
2. Keep modules small and export only stable public functions.
3. Prefer `Result` for expected failure and `Option` for absence.
4. Use explicit bounds and timeouts instead of unbounded loops or blocking operations.
5. Keep secrets in environment or managed credentials, never in source or diagnostics.
6. Run `zap fmt`, `zap check`, `zap test`, and `zap build --locked` before committing.
7. Treat a development server as a reference implementation until host, TLS, observability, shutdown, and deployment evidence are present.

## 22. A Complete Small Example

The following program combines typed functions, collections, JSON, conditions, and assertions:

```zap
fn describe(name: text, score: number) -> map:
    return {
        "name": name,
        "score": score,
        "passed": score >= 50
    }

let students = [
    describe("Aye", 80),
    describe("Min", 45)
]

let report = {
    "language": "Zap",
    "students": students,
    "count": len(students)
}

assert(report["count"] == 2, "student count is wrong")
say json(report)
```

Save it as `main.zp`, then run:

```bash
zap fmt main.zp
zap check .
zap run main.zp
```

When the program grows, move reusable declarations into a module, put tests in `tests/`, and create a `zap.toml` project instead of keeping everything in one file.

## 23. Command Reference

| Command | Purpose |
|---|---|
| `zap file.zp` | Run a source file |
| `zap run file.zp` | Run a source file explicitly |
| `zap new directory` | Create the complete user-managed Web scaffold |
| `zap init directory` | Create a minimal generic Zap project |
| `zap check [directory]` | Validate a Zap project directory |
| `zap check --json [directory]` | Emit structured diagnostics |
| `zap build [directory]` | Validate build readiness |
| `zap build --locked [directory]` | Require a canonical lockfile |
| `zap test [directory]` | Run `*_test.zp` files |
| `zap fmt file.zp` | Format source |
| `zap lint file.zp` | Report style issues |
| `zap lock [directory]` | Generate canonical lock data |
| `zap install [directory]` | Validate/install locked dependencies |
| `zap update [directory]` | Regenerate lock data after manifest changes |
| `zap web check [directory]` | Validate Web manifest and scaffold |
| `zap web routes [directory] [--json]` | Inspect the validated route table without opening a listener |
| `zap explain route <path> [directory] [--json]` | Explain matching route declarations and extracted parameters without executing handlers |
| `zap dev [directory]` | Run the declared Web development server |
| `zap db check [directory]` | Validate migration layout |
| `zap db plan [directory]` | Show the deterministic migration plan |
| `zap db inspect [directory]` | Inspect adapter and migration state |
| `zap db migrate [directory]` | Apply SQLite migrations |
| `zap lsp` | Run the LSP over stdio |
| `zap async-check` | Validate the async runtime boundary |
| `zap --version` | Print the installed version |
| `zap --help` | Print full command help |

## 24. Troubleshooting

| Symptom | Likely cause | Action |
|---|---|---|
| `zap: command not found` | Executable is not on `PATH` | Add the Zap `bin` directory to `PATH` and reopen the shell |
| `unknown command` | Installed binary is older than this guide | Run `zap --version` and install the matching release |
| `zap check` rejects a type | Annotation and value disagree | Correct the value or annotation; do not rely on coercion |
| `module not found` | Import path or module root is wrong | Check relative paths, entries, and `.zp` file names |
| `circular module dependency` | Import graph contains a cycle | Split shared declarations into a lower-level module |
| `zap build --locked` rejects the project | Lockfile is missing or stale | Run `zap lock` after reviewing manifest changes |
| `zap dev` rejects the project | Web manifest path is unsafe or missing | Run `zap web check` and inspect `[web]` fields |
| `db migrate --check` reports pending work | Migrations have not been applied | Review `zap db plan`, back up data, then migrate intentionally |
| LSP shows stale diagnostics | Editor sent an unsupported/stale update | Reopen the document and reproduce with `zap check --json` |
| frontend works in development but not deployment | Build output was not copied to `public/` | Copy the final HTML/CSS/JS assets and serve them through Zap |

## 25. What Is Stable and What Is Still Developing

The stable direction includes the `.zp` source format, native CLI execution, indentation blocks, core values, functions, classes, modules, typed checks, Result/Option foundations, deterministic project validation, lockfiles, JSON diagnostics, tests, LSP foundations, the one-command Web scaffold, and bounded native Web serving.

The following areas remain active or deliberately deferred: a complete user-defined trait system, a production asynchronous I/O reactor, provider-neutral database migrations, a full ORM, a template/component compiler, cross-file semantic rename, a hidden app registry, and a replacement for every JavaScript build tool. The absence of a feature is intentional when its contract and executable evidence are not complete. Track future changes in the language specification, compatibility template, roadmap, and release notes rather than inferring behavior from an old example.[1]

## References

[1]: LANGUAGE_SPEC_EN.md "Zap Language Specification"
[2]: FRONTEND_INTEGRATION_EN.md "Zap Frontend Integration Guide"
[3]: TYPECHECK_CONFORMANCE_MATRIX_EN.md "Zap Type-Check Conformance Matrix"
[4]: RUNTIME_STATE_EN.md "Zap Runtime State and Execution Context"
[5]: DEFAULT_PARAMETERS_EN.md "Zap Default Function Parameters"
[6]: MEMORY_BUDGET_OBJECT_STORE_EN.md "Zap Memory Budget and Object Store Contract"
[7]: STDLIB_POLICY_EN.md "Zap Standard Library Policy"
[8]: TRAITS_RFC_EN.md "Zap Traits and Composition RFC"
[9]: PACKAGE_EN.md "Zap Package Author Guide"
[10]: STDLIB_INDEX_EN.md "Zap Standard Library Index"
[11]: ZAP_WEB_NATIVE_EN.md "Zap Native Web Guide"
[12]: DATABASE_PRODUCTION_EN.md "Zap Database Production Guide"
[13]: ASYNC_BOUNDARIES_EN.md "Zap Async Boundary Contract"
