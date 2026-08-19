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

## Modules

Use explicit exports in a module:

```zap
# modules/greeting.zp
export fn greet(name):
    return "Hello, " + name

fn private_helper():
    return "internal"
```

Import it from another source file:

```zap
import "greeting.zp"
say greet("Zap")
```

Only exported symbols are visible to importing files. Modules are resolved from the source directory and supported local module directories. Canonical-path caching prevents duplicate top-level execution; circular imports and absolute module paths are rejected.

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
