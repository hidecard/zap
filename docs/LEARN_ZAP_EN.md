# Learn Zap — English Guide

**Verified baseline:** Zap v2.2.6
**Purpose:** A learner-first path from installation and core syntax to modules, typed results, testing, tooling, and small projects.
**Navigation:** [Documentation hub](DOCUMENTATION_NAVIGATION_EN.md) · [Syntax reference](SYNTAX_GUIDE_EN.md) · [Language specification](LANGUAGE_SPEC_EN.md) · [Stdlib reference](STDLIB_INDEX_EN.md) · [Package author guide](PACKAGE_EN.md) · [Runtime state](RUNTIME_STATE_EN.md) · [Deployment boundaries](DEPLOYMENT_EN.md)

Zap is a small, readable, general-purpose programming language using `.zp` source files. This guide is designed for a first-time learner. It moves from running a program to values, control flow, functions, modules, error handling, testing, and small projects.

> **Language choice:** If you prefer Burmese, use [`LEARN_ZAP_MM.md`](LEARN_ZAP_MM.md). The Burmese guide follows this same core learning path and includes additional localized advanced lessons for OOP, tooling, diagnostics, and typed Result/Option payloads.

## Before You Begin

Download a release archive from the [GitHub Releases page](https://github.com/hidecard/zap/releases), extract it, and place the `zap` executable on your `PATH`. Verify the installation with:

```bash
zap --version
zap --help
```

On Windows, the executable can also be run directly:

```bat
bin\zap.exe --version
bin\zap.exe main.zp
```

## Lesson 1 — Hello World

Create a file named `hello.zp`:

```zap
say "Hello from Zap"
```

Run it with:

```bash
zap hello.zp
```

`say` writes a value to the terminal. A Zap program is normally a sequence of statements separated by new lines.

## Lesson 2 — Comments and Indentation

Comments begin with `#` and continue to the end of the line. Blocks are defined by a colon and indentation. Four spaces per level are recommended.

```zap
# This line is ignored by the runtime.
if true:
    say "This statement is inside the block"
```

Keep indentation consistent inside every `if`, loop, function, and class body.

## Lesson 3 — Values and Variables

Zap supports text, integer numbers, booleans, lists, maps, and `none`.

```zap
let language = "Zap"
let version = 9
let ready = true
let empty = none
let tools = ["compiler", "formatter", "tester"]
let user = {"name": "Developer", "active": true}

say language
say type(tools)
```

Use `let` for a new variable and ordinary assignment for a value that already exists:

```zap
let count = 1
count = count + 1
say count
```

## Lesson 4 — Type Annotations

Annotations are optional. The current built-in annotation names include `text`, `number`, `bool`, `list`, `map`, `none`, and `any`.

```zap
let name: text = "Zap"
let port: number = 8080
let enabled: bool = true
```

A mismatch is reported by the checker and runtime:

```zap
let port: number = "8080"  # invalid: text is not number
```

Use the checker before running a project:

```bash
zap check .
zap check --json .
```

## Lesson 5 — Text and Operators

Text values can be joined with `+`. Arithmetic operators are `+`, `-`, `*`, `/`, and `%`. Comparisons produce booleans.

```zap
let first = "Hello"
let second = "Zap"
let message = first + ", " + second + "!"
let total = 10 + 5 * 2
let remainder = 17 % 4
let valid = total >= 20

say message
say total
say remainder
say valid
```

Use parentheses when they make precedence clearer. Integer overflow and division by zero are checked runtime errors.

## Lesson 6 — Conditions

Use `if` and `else` with a colon and indentation:

```zap
let score = 85

if score >= 80:
    say "Excellent"
else:
    say "Keep practising"
```

Logical operators are `and`, `or`, and `not`:

```zap
let account_exists = true
let verified = false

if account_exists and not verified:
    say "Verification is required"
```

## Lesson 7 — Lists, Maps, and JSON

List indexes start at zero. Maps use keys and bracket lookup.

```zap
let languages = ["Zap", "Rust", "Go"]
say languages[0]
say len(languages)

let profile = {"name": "Zap User", "age": 20}
say profile["name"]
say keys(profile)

let encoded = json(profile)
let decoded = from_json(encoded)
say decoded["age"]
```

Use `contains`, `get`, `join`, `sort`, `reverse`, `sum`, and `is_empty` when working with collections.

## Lesson 8 — Loops

`for` iterates over a list or range. `while` repeats while its condition is true.

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

Use `break` to stop a loop and `continue` to skip to the next iteration.

## Lesson 9 — Functions and Return Values

Declare a function with `fn`. Parameters and return types can be annotated.

```zap
fn add(a: number, b: number) -> number:
    return a + b

say add(4, 6)
```

A function without an explicit return produces `none`. The checker validates known function signatures and reports wrong argument counts or incompatible literal types.

## Lesson 10 — Closures and Scope

A nested function can read values from its surrounding function.

```zap
fn make_greeting(prefix: text) -> text:
    fn greet(name: text) -> text:
        return prefix + ", " + name
    return greet("Developer")

say make_greeting("Hello")
```

Keep functions small and give each function one clear responsibility.

## Lesson 11 — Classes and Objects

Zap provides a beginner-friendly class syntax. A class can define an initializer and methods.

```zap
class User:
    fn init(self, name):
        self.name = name

    fn greet(self):
        return "Hello, " + self.name

let user = new("User", "Zap")
say user.greet()
```

Inheritance uses `extends`:

```zap
class Animal:
    fn speak(self):
        return "sound"

class Dog extends Animal:
    fn speak(self):
        return "woof"

let dog = new("Dog")
say dog.speak()
```

See [`SYNTAX_GUIDE.md`](SYNTAX_GUIDE.md) for the full current syntax reference.

## Lesson 12 — Files, Paths, and Environment

The standard runtime includes basic text-file, path, time, and environment helpers.

```zap
let path = path_join("data", "note.txt")

if exists(path):
    say read_text(path)
else:
    write_text(path, "Created by Zap")

if has_env("PATH"):
    say env("PATH")
```

Validate user-provided paths in production programs and handle file errors explicitly.

## Lesson 13 — Modules and Exports

A project can contain local modules below the manifest’s relative module root. Zap uses explicit module declarations and `import ... as ...` paths so that private symbols are not accidentally exposed and module resolution remains deterministic. Absolute paths, traversal-like paths, missing entries, duplicate entries, and circular imports are rejected.

`modules/app/core.zp`:

```zap
module app.core

export fn greet(name):
    return "Hello, " + name

fn private_helper():
    return "internal"
```

`main.zp`:

```zap
module app.main
import app.core as core

say core.greet("Zap")
```

Only exported symbols are available to importers. Modules are cached during one runtime execution, imported files are traversed in deterministic source order, and absolute module paths are not accepted. Legacy `use`/path-style imports may remain available for compatibility, but new libraries should use explicit `module` and `import`/`export` syntax.

## Lesson 14 — Result and Option Values

`Result` represents success or failure. `Option` represents a value that may be present or absent.

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

`unwrap` raises a runtime error for an error or missing value. Prefer `unwrap_or` when a safe fallback is appropriate.

## Lesson 15 — Automatic Result Propagation

The `?` operator unwraps a successful Result and returns an error Result from the current function when the value is an error.

```zap
fn load_user() -> result<any>:
    return err("user not found")

fn profile() -> result<any>:
    let user = load_user()?
    return ok(user)

say profile()
```

`ok(value)?` continues with `value`; `err(error)?` returns the error immediately. Applying `?` to a non-Result value is invalid.

## Lesson 16 — Tests and Assertions

Test files conventionally end in `_test.zp`. Use `assert` to express expected behavior.

```zap
fn add(a, b):
    return a + b

assert(add(2, 3) == 5, "addition failed")
assert(type(add(2, 3)) == "number", "wrong result type")
say "test passed"
```

Run project tests with:

```bash
zap test
zap test tests
```

The native runtime integration suite can be run from the repository with `cargo test --manifest-path native/Cargo.toml`.

For larger projects, narrow the run with `--filter`, stop after the first failure with `--fail-fast`, or produce machine-readable output with `--json`:

```bash
zap test tests --filter arithmetic
zap test tests --fail-fast
zap test tests --json
```

Unknown test options are usage errors and return exit code `2`; a failing test returns exit code `1`.

## Lesson 17 — CLI Workflow and Diagnostics

The main commands are:

| Command | Purpose |
|---|---|
| `zap file.zp` | Run a source file |
| `zap init project` | Create a project scaffold |
| `zap check` | Validate a project |
| `zap check --json` | Emit structured diagnostics |
| `zap build` | Validate build readiness |
| `zap test` | Run Zap tests |
| `zap fmt file.zp` | Format source code |
| `zap lint file.zp` | Report style issues |
| `zap --version` | Print the runtime version |

For machine-readable diagnostics, use `zap check --json`. Diagnostics contain structured fields such as the kind, message, file, line, and column when source information is available.

## Lesson 18 — Mini Project

A practical first project can combine a function, a list, a map, and JSON:

```zap
fn describe(name: text, score: number) -> map:
    return {"name": name, "score": score, "passed": score >= 50}

let students = [describe("Aye", 80), describe("Min", 45)]
let report = {"language": "Zap", "students": students}

say json(report)
```

After writing the program, run `zap fmt`, `zap check`, and `zap test`. Keep examples small while learning, then split reusable code into exported modules.

## Quick Reference

| Area | Current Zap syntax or tools |
|---|---|
| Source files | `.zp`, commonly `main.zp` |
| Blocks | Colon followed by indentation |
| Values | text, number, bool, list, map, none |
| Types | `text`, `number`, `bool`, `list`, `map`, `none`, `any` |
| Functions | `fn name(args) -> type:` |
| Modules | explicit `import` and `export` |
| Errors | `ok`, `err`, `?`, `unwrap`, `unwrap_or` |
| Tooling | `check`, `check --json`, `build`, `test`, `fmt`, `lint`, `run` |

For a complete syntax inventory, continue with [`SYNTAX_GUIDE.md`](SYNTAX_GUIDE.md). For the Burmese course, open [`LEARN_ZAP_MM.md`](LEARN_ZAP_MM.md).

## Documentation Feedback

Zap is still evolving. If an example differs from the installed runtime, report it with the Zap version, operating system, source file, and command used. This makes documentation corrections reproducible.


## Lesson 19 — Structured ZapError Diagnostics

Zap reports failures through a structured diagnostic boundary called `ZapError`. The current variants include `SyntaxError`, `NameError`, `TypeError`, `ValueError`, `IOError`, `FileNotFound`, `PermissionError`, `OverflowError`, `Error`, and `ProjectError`. The `Error` category is reserved for stable runtime failures such as uncaught typed `Result` errors. The runtime keeps the original message and, when available, the source file, line, and column.

For automation and editor integration, use JSON diagnostics:

```bash
zap check --json .
```

A failed check can produce output such as:

```json
{"ok":false,"kind":"TypeError","file":"main.zp","line":4,"column":12,"message":"expected number, got text","error":"TypeError at main.zp:4:12: expected number, got text"}
```

Human-readable command failures and JSON check failures now share the same diagnostic classification. The evaluator still contains some legacy internal `String` error paths; replacing those internal return types with `ZapError` is a later architecture refactor and does not change the current `.zp` syntax.


## Lesson 20 — Typed Result and Option Payloads

Zap can statically check the value carried by a Result or Option. Use angle brackets in an annotation to describe the payload type:

```zap
let answer: result<number> = ok(42)
let failure: result<text> = err("not found")
let user: option<text> = some("Zap")
let missing: option<number> = option_none()
```

The checker rejects mismatched payloads before execution:

```zap
let invalid: result<number> = ok("wrong")
```

Run `zap check --json .` to receive a machine-readable `TypeError`. `option_none()` is represented as `option<any>` and can be assigned to a typed Option because it carries no concrete payload.


For the current roadmap and release details, see [`DOCUMENTATION_NAVIGATION_EN.md`](DOCUMENTATION_NAVIGATION_EN.md), [`TYPECHECK_CONFORMANCE_MATRIX_EN.md`](TYPECHECK_CONFORMANCE_MATRIX_EN.md), [`NEXT_TODO_PLAN_EN.md`](NEXT_TODO_PLAN_EN.md), and [`RELEASE_2.2.5_EN.md`](RELEASE_2.2.5_EN.md).
