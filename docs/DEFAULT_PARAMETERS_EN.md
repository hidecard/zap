# Default Function Parameters in Zap

Default function parameters let a function provide a value when the caller omits that argument. They are useful for optional configuration, friendly defaults, and APIs that should remain concise for common cases.

## Basic syntax

A default parameter is written with `=` in the parameter list:

```zap
fn greet(name = "World"):
    return "Hello, " + name

say greet()
say greet("Zap")
```

The output is:

```text
Hello, World
Hello, Zap
```

When `greet()` is called without an argument, Zap binds `name` to `"World"`. When `greet("Zap")` is called, the provided value takes precedence over the default.

## Typed default parameters

A parameter may include both a type annotation and a default expression. The annotation is written before the default value:

```zap
fn repeat_message(message: text = "Zap", times: number = 1):
    let index = 0
    while index < times:
        say message
        index = index + 1

repeat_message()
repeat_message("Learning", 2)
```

The default expression must produce a value compatible with the parameter annotation. The following declaration is invalid because the default is text while the parameter requires a number:

```zap
fn square(value: number = "one") -> number:
    return value * value
```

Use `zap check` to detect annotation mismatches before running a project.

## Positional and named binding

Zap supports both positional arguments and named arguments. Positional arguments bind from left to right. A named argument uses `parameter = expression` inside the call and binds directly to the parameter with that name. Omitted parameters use their declared defaults.

```zap
fn connect(host: text = "localhost", port: number = 8080, secure: bool = false):
    return host + ":" + str(port) + ":" + str(secure)

say connect()
say connect("api.example.com")
say connect(host = "api.example.com", secure = true)
say connect(port = 443, host = "api.example.com")
```

Named arguments are useful when overriding a later default without supplying every earlier default. Positional arguments may appear before named arguments, but a positional argument may not follow a named argument. Thus `f(10, c = 30)` is valid while `f(a = 10, 20)` is rejected.

## Required and defaulted parameters together

A function can mix required parameters with defaulted parameters. A call must always provide every required parameter, while defaulted parameters may be omitted.

```zap
fn create_user(username: text, role: text = "member", active: bool = true):
    return {
        "username": username,
        "role": role,
        "active": active
    }

say create_user("may")
say create_user("may", "admin", false)
```

The first call supplies the required `username` and receives defaults for `role` and `active`. The second call overrides both defaults.

## Default expressions

A default is stored as an expression and evaluated when the argument is omitted. This means the value is resolved at call time rather than copied as a fixed value at function declaration time.

```zap
fn welcome(prefix: text = "Hello", name: text = "World"):
    return prefix + ", " + name

say welcome()
say welcome("Mingalaba", "Zap")
```

Default expressions are evaluated in the function call's local environment. Defaults should therefore be kept simple and deterministic. A default can refer to values already available to the function's closure, but it should not depend on a later parameter that has not been bound yet. Omitted defaults are parsed and evaluated as canonical AST expressions, including nested built-in calls; they do not re-enter the legacy line-expression parser.

## Methods and constructors

The same positional default behavior applies to class methods and constructors. The implicit `self` parameter is supplied by the runtime and is not written by the caller as an ordinary method argument.

```zap
class User:
    fn init(self, name: text = "Guest"):
        self.name = name

    fn label(self, prefix: text = "User"):
        return prefix + ": " + self.name

let guest = new("User")
let developer = new("User", "Developer")
say guest.label()
say developer.label("Account")
```

For methods, the runtime checks the arguments after `self`. Constructor and method defaults follow the same omission and override rules as ordinary functions. The built-in `new(...)` call is a separate constructor boundary: it accepts a text class name, positional constructor arguments, and an optional positional map of explicit fields. Named arguments are intentionally rejected with a deterministic diagnostic; named binding remains supported for user-defined functions and methods.

## Return types and defaults

Default parameters work with return annotations. Runtime and static validation still apply to the supplied or defaulted value and to the returned value.

```zap
fn port_or_default(port: number = 8080) -> number:
    return port

say port_or_default()
say port_or_default(3000)
```

## Validation rules

| Rule | Example | Result |
|---|---|---|
| A default requires a non-empty expression | `fn f(value =):` | Rejected during parsing |
| Duplicate parameter names are rejected | `fn f(value, value):` | Rejected during parsing |
| A supplied argument must match its annotation | `fn f(n: number = 1):` then `f(n = "x")` | Runtime/static type error |
| A default must match its annotation | `fn f(n: number = "x"):` | Type-checking error when checked |
| Too few required arguments are rejected | `fn f(a, b = 2):` then `f()` | Missing-argument error |
| Too many arguments are rejected | `fn f(a = 1):` then `f(1, 2)` | Argument-count error |
| Unknown names are rejected | `fn f(a):` then `f(b = 1)` | Unknown named-argument error |
| Duplicate names are rejected | `f(a = 1, a = 2)` | Duplicate named-argument error |
| Positional-after-named is rejected | `f(a = 1, 2)` | Binding-order error |
| Named binding selects parameters directly | `f(second = 20, first = 10)` | Values bind by name |
| Built-in constructor names are rejected | `new("User", name = "Guest")` | Deterministic unsupported-named-argument error |

A typical missing-argument diagnostic for a function with two required parameters is similar to:

```text
function expects 2 to 2 arguments, got 1
```

For a function with one required and two defaulted parameters, the valid range is zero? No: the required parameter still must be supplied. The runtime reports the valid minimum and maximum number of arguments, for example `function expects 1 to 3 arguments, got 0`.

## Complete example

The following file can be run directly from the repository as `examples/default_parameters.zp`:

```zap
fn greet(name: text = "World", punctuation: text = "!"):
    return "Hello, " + name + punctuation

fn rectangle_area(width: number, height: number = 1) -> number:
    return width * height

fn describe_user(username: text, role: text = "member", active: bool = true):
    say "username=" + username
    say "role=" + role
    say "active=" + str(active)

say greet()
say greet("Zap", ".")
say rectangle_area(8)
say rectangle_area(8, 3)
describe_user("developer")
describe_user("admin", "administrator", false)
```

Run it with:

```bash
zap examples/default_parameters.zp
```

The current implementation supports **positional and named arguments together with default parameters**. Named calls such as `greet(name = "Zap")` are supported for user-defined functions and methods through the structured AST call path. Built-in calls, including `new(...)`, reject named arguments unless a built-in contract explicitly adds support. Native `new(...)` construction, default expressions, and unsupported-call diagnostics now stay on the canonical AST execution path without hidden legacy reparsing.

## Related references

The general syntax reference is available in [`SYNTAX_GUIDE_EN.md`](SYNTAX_GUIDE_EN.md). The beginner course is [`LEARN_ZAP_EN.md`](LEARN_ZAP_EN.md). The implementation is covered by the native regression test `applies_default_function_parameters` in [`native/tests/core.rs`](../native/tests/core.rs).
