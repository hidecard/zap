# Zap Syntax Reference — English

This reference summarizes the syntax supported by the current Zap runtime. Zap source files use the `.zp` extension and blocks are defined by indentation.

## Running Programs

```bash
zap main.zp
zap init hello-zap
zap check .
zap check --json .
zap build .
zap test .
zap fmt main.zp
zap lint main.zp
zap run main.zp
```

## Comments and Values

```zap
# comment
say "text"
say 42
say true
say none
say [1, 2, 3]
say {"name": "Zap"}
```

Supported core values are `text`, integer `number`, `bool`, `list`, `map`, and `none`.

## Variables and Annotations

```zap
let name = "Zap"
let port: number = 8080
let enabled: bool = true
port = 9090
```

Available annotation names include `text`, `number`, `bool`, `list`, `map`, `none`, and `any`. `zap check` reports known annotation mismatches.

## Operators

| Operator | Meaning |
|---|---|
| `+`, `-`, `*`, `/`, `%` | Arithmetic and text concatenation where applicable |
| `==`, `!=`, `<`, `<=`, `>`, `>=` | Comparison |
| `and`, `or`, `not` | Boolean logic |

```zap
let total = (10 + 5) * 2
let allowed = total >= 20 and not false
```

Integer overflow and division by zero are checked runtime errors.

## Blocks and Control Flow

```zap
if score >= 80:
    say "Excellent"
else:
    say "Keep practising"

for item in ["web", "ai", "iot"]:
    say item

let index = 0
while index < 3:
    say index
    index = index + 1
```

Use `break` and `continue` inside loops.

## Functions

```zap
fn add(a: number, b: number) -> number:
    return a + b

fn greet(name):
    return "Hello, " + name
```

Function annotations use the form `parameter: type` and `-> return_type`. Nested functions can capture values from their enclosing scope.

### Default parameters

Parameters may provide a default value with `=`. Zap currently binds arguments positionally: omitted parameters use their defaults, while supplied arguments override them.

```zap
fn greet(name: text = "World", punctuation: text = "!"):
    return "Hello, " + name + punctuation

say greet()
say greet("Zap", ".")
```

A function may mix required and defaulted parameters, but every required parameter must be supplied:

```zap
fn create_user(username: text, role: text = "member"):
    return username + " (" + role + ")"

say create_user("may")
say create_user("may", "admin")
```

Named arguments are supported for function and method calls. See the complete [Default Function Parameters guide](DEFAULT_PARAMETERS_EN.md) for validation rules, method examples, and runnable samples.

### Async functions and await

Prefix a function declaration with `async` to return a deterministic `Future`. Use `await` to unwrap the completed result:

```zap
async fn load() -> number:
    return 7

let pending = load()
let value: number = await pending
say value
```

`await` is an expression and may also be applied directly to a call:

```zap
async fn answer() -> number:
    return 42

say (await answer()) + 1
```

The current runtime executes async bodies deterministically and does not create background threads. Timers, cancellation, and richer suspension behavior remain runtime roadmap items. See the [Async/LSP guide](ASYNC_LSP_EN.md) for the executor and editor protocol details.

## Classes

```zap
class User:
    fn init(self, name):
        self.name = name

    fn greet(self):
        return "Hello, " + self.name

let user = new("User", "Zap")
say user.greet()
```

Inheritance uses `extends` and methods can be overridden:

```zap
class Animal:
    fn speak(self):
        return "sound"

class Dog extends Animal:
    fn speak(self):
        return "woof"
```

## Lists, Maps, and JSON

```zap
let items = ["a", "b", "c"]
say items[0]
say len(items)
say join(items, ",")

let user = {"name": "Zap", "active": true}
say user["name"]
say keys(user)
say json(user)
say from_json("{\"ok\": true}")
```

Collection helpers include `contains`, `get`, `is_empty`, `sum`, `reverse`, and `sort`.

## Modules and Workspaces

Zap supports explicit module declarations and deterministic imports. A module declaration records the logical name of a source file, while an import may provide a local alias:

```zap
# modules/app/core.zp
module app.core

fn version():
    return "2.0"
```

```zap
# main.zp
module app.main
import app.core as core

say core
```

The dotted import path maps to a `.zp` file below the module root. For example, `import app.core as core` resolves to `modules/app/core.zp` when the project manifest contains:

```toml
[package]
name = "workspace-demo"
version = "0.1.0"
main = "main.zp"

[module]
root = "modules"
entries = ["app/core.zp"]
```

Module roots and entries must be relative, entries must end in `.zp`, and each listed file must exist. Explicit imports reject absolute paths, separators, empty path components, and traversal. The resolver visits imported files in deterministic source order, caches completed nodes, and reports a stable `circular module dependency` diagnostic containing the complete cycle when a dependency loop is found. Legacy `use "file.zp"` imports remain available for compatibility; new workspace code should prefer `module` and `import ... as ...`.

## Result and Option

```zap
let success = ok(42)
let failure = err("failed")
let value = some("Zap")
let missing = option_none()

say is_ok(success)
say is_err(failure)
say is_some(value)
say unwrap_or(failure, 0)
say unwrap_or(missing, "default")
```

The `?` operator propagates an error Result from the current function:

```zap
fn read_value() -> Result:
    return err("not available")

fn use_value() -> Result:
    let value = read_value()?
    return ok(value)
```

## Structured Errors: `raise`, `try`, and `catch`

Zap also supports deterministic structured control flow for exceptional runtime paths. `raise <expression>` evaluates the expression and immediately propagates its value through the current function, loop, and module boundary until a matching `try`/`catch` handles it. A bare `raise` is rejected during parsing with `raise expects an expression`.

```zap
fn load_config():
    raise "configuration unavailable"

try:
    load_config()
catch error:
    say "handled: " + error
```

A `try` block must be followed by a same-level `catch <binding>:` clause with an indented body. The raised value is bound to the catch name, which may shadow an existing variable only for the catch body; the previous value is restored afterward. If the catch body executes `raise` again, the new or original value continues outward.

```zap
let error = "outer"
try:
    try:
        raise {"code": 503, "message": "offline"}
    catch error:
        say error["code"]
        raise error
catch error:
    say error["message"]

say error # outer
```

Catch blocks also preserve normal control flow. They do not execute when the try body completes normally, and `return`, `break`, and `continue` from a catch body retain their usual enclosing-function or enclosing-loop behavior. At the process boundary, an uncaught value is reported deterministically as `raised error: <value>`.

## Files, Paths, Time, and Environment

```zap
let path = path_join("data", "note.txt")
if exists(path):
    say read_text(path)
else:
    write_text(path, "Hello from Zap")

say now()
if has_env("PATH"):
    say env("PATH")
```

Available helpers include `read_text`, `write_text`, `read_lines`, `write_lines`, `basename`, `dirname`, `exists`, `sleep`, `env`, `has_env`, `abs`, `min`, `max`, `pow`, and `sqrt`.

## Diagnostics and Tests

```zap
assert(1 + 1 == 2, "arithmetic failed")
```

Test files conventionally use the `_test.zp` suffix. `zap check --json` emits structured diagnostics with fields such as `kind`, `message`, `file`, `line`, and `column`.

For the complete beginner course, see [`LEARN_ZAP_EN.md`](LEARN_ZAP_EN.md). For the Burmese course, see [`LEARN_ZAP_MM.md`](LEARN_ZAP_MM.md).
