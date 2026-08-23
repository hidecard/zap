# Zap Language Guide

> **ရည်ရွယ်သူ:** Zap ကို ပထမဆုံး install လုပ်ခြင်းမှစ၍ structured၊ typed၊ modular၊ asynchronous၊ tested နှင့် Web-enabled program များ ရေးသားနိုင်သည့် Advanced အဆင့်အထိ လေ့လာလိုသူများအတွက် ဖြစ်ပါသည်။

**စစ်ဆေးထားသော baseline:** Zap v2.7.0
**Source extension:** `.zp`
**Runtime:** standalone native `zap` executable
**လမ်းညွှန်:** [Documentation hub](DOCUMENTATION_NAVIGATION_MM.md) · [Language specification](LANGUAGE_SPEC_MM.md) · [Syntax reference](SYNTAX_GUIDE.md) · [Standard-library index](STDLIB_INDEX_MM.md) · [English guide](LEARN_ZAP_EN.md)

## 1. Zap ဆိုတာဘာလဲ

Zap သည် indentation-based block၊ explicit module၊ optional type annotation၊ structured `Result` နှင့် `Option` value၊ native command-line runtime၊ deterministic project validation နှင့် script မှ Web application အထိ တိုးချဲ့ရေးသားနိုင်သော general-purpose programming language ဖြစ်ပါသည်။ ပုံမှန် execution pipeline သည် **source → lexer → AST parser → evaluator** ဖြစ်ပြီး `.zp` application run လုပ်သည့်စက်တွင် Python၊ Node.js၊ Java သို့မဟုတ် Rust ကို runtime dependency အဖြစ် မလိုပါ။[1]

Rust toolchain သည် Zap executable ကို build လုပ်ရန်သာ အသုံးပြုပါသည်။ Zap project run လုပ်ရန် မလိုအပ်ပါ။ React၊ Vue၊ Svelte၊ Alpine စသည့် JavaScript framework များကို build-time toolchain အဖြစ် optional အသုံးပြုနိုင်ပြီး ထွက်လာသော HTML၊ CSS၊ JavaScript နှင့် asset များကို `public/` အောက်တွင် ထား၍ Zap ဖြင့် serve လုပ်နိုင်ပါသည်။[2]

ဤ guide တွင် behavior သုံးမျိုး ခွဲခြားဖတ်ရပါမည်။ **Normative** ဆိုသည်မှာ language specification နှင့် executable test များက သတ်မှတ်ထားသော behavior ဖြစ်သည်။ **Compatibility** ဆိုသည်မှာ အဟောင်း project များအတွက် ဆက်လက်ထားရှိသော behavior ဖြစ်သည်။ **Deferred** ဆိုသည်မှာ design ရှိသော်လည်း လက်ရှိ release တွင် enable မလုပ်သေးသော feature ဖြစ်သည်။ ဥပမာ user-defined traits နှင့် production asynchronous I/O reactor တို့သည် deferred ဖြစ်နေသေးပါသည်။

## 2. Zap ကို Install လုပ်ခြင်း

### 2.1 Linux နှင့် macOS

သင့် operating system နှင့် architecture ကိုက်ညီသော archive ကို [GitHub Releases](https://github.com/hidecard/zap/releases) မှ download လုပ်ပါ။ Extract လုပ်ပြီး executable ကို `PATH` ထဲရှိ directory သို့ ထည့်ကာ Unix စနစ်များတွင် executable permission ပေးပါ။

```bash
tar -xzf zap-2.7.0-linux-x86_64.tar.gz
sudo install -m 0755 zap/bin/zap /usr/local/bin/zap
zap --version
zap --help
```

Administrator access မရှိပါက `~/.local/bin` ထဲသို့ ထည့်နိုင်ပါသည်။

```bash
mkdir -p "$HOME/.local/bin"
install -m 0755 zap/bin/zap "$HOME/.local/bin/zap"
export PATH="$HOME/.local/bin:$PATH"
zap --version
```

macOS ARM64 အတွက် သက်ဆိုင်ရာ macOS ARM64 archive ကို အသုံးပြုပါ။

### 2.2 Windows

Windows archive ကို download လုပ်ပြီး extract လုပ်ပါ။ `bin\zap.exe` ကို တိုက်ရိုက် run လုပ်နိုင်သလို `zap.exe` ရှိသော directory ကို Windows `PATH` ထဲသို့ ထည့်နိုင်ပါသည်။

```bat
bin\zap.exe --version
bin\zap.exe --help
```

### 2.3 Installation စစ်ဆေးခြင်း

```bash
zap --version
zap --help
```

`zap` မတွေ့ဟု shell က ပြပါက executable သည် `PATH` ထဲ မရှိခြင်း ဖြစ်ပါသည်။ Documentation ထက် version နိမ့်နေပါက command အသစ်များကို အသုံးမပြုမီ သက်ဆိုင်ရာ release executable ကို update လုပ်ပါ။

## 3. ပထမဆုံး Program

`hello.zp` ဖိုင် ဖန်တီးပါ။

```zap
say "Hello from Zap"
```

```bash
zap hello.zp
zap run hello.zp
```

ရလဒ်မှာ—

```text
Hello from Zap
```

`say` သည် value ကို terminal သို့ ရေးပေးပါသည်။ ပုံမှန် statement များအတွက် semicolon မလိုပါ။ Comment သည် `#` ဖြင့် စပြီး line အဆုံးအထိ ဖြစ်ပါသည်။

```zap
# ဤစာကြောင်းကို runtime က မလုပ်ဆောင်ပါ။
if true:
    say "block အတွင်း"
say "block အပြင်"
```

Block တစ်ခုအတွင်း indentation style မရောပါနှင့်။ Parser သည် malformed indentation နှင့် body မပါသော block များကို structured diagnostic ဖြင့် ပြပါမည်။

## 4. Command တစ်ကြောင်းဖြင့် Project ဆောက်ခြင်း

Structured Web project တစ်ခုအတွက်—

```bash
zap new my_app
cd my_app
```

ဒီ command တစ်ကြောင်းတည်းဖြင့် user ကိုယ်တိုင် စီမံနိုင်သော project တစ်ခုလုံးကို ဖန်တီးပေးပါသည်။ Django-style `startapp` command နှင့် hidden application registry မရှိပါ။ Generate ပြီးနောက် source file များကို User ကိုယ်တိုင် ထည့်၊ ဖျက်၊ အမည်ပြောင်း၊ စီစဉ်နိုင်ပါသည်။

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

`models/` သည် data shape နှင့် validation metadata၊ `functions/` သည် business logic နှင့် use case၊ `ui/` သည် browser-facing UI metadata၊ `routes/` သည် HTTP route declaration၊ `middleware/` သည် request/response policy၊ `migrations/` သည် schema change၊ `admin/` သည် administration registration နှင့် `public/` သည် browser asset များကို ပိုင်ဆိုင်ပါသည်။ Directory များသည် user-managed ordinary files ဖြစ်သောကြောင့် application တိုးလာသည်နှင့်အမျှ file အသစ်များကို ကိုယ်တိုင် ထည့်နိုင်ပါသည်။

```bash
zap check
zap build --locked
zap test tests
zap dev
```

`zap dev` သည် bounded native development server ဖြစ်ပြီး production Web platform အားလုံး ပြီးစီးပြီဟု မဆိုလိုပါ။

## 5. Value နှင့် Variable

Zap ၏ core value များမှာ text၊ integer number၊ boolean၊ list၊ map၊ object၊ function နှင့် `none` ဖြစ်ပါသည်။

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

Variable အသစ်အတွက် `let` သုံးပြီး ရှိပြီးသား variable ကို assignment ဖြင့် ပြင်ပါ။

```zap
let count = 1
count = count + 1
say count
```

Type annotation ဖြင့် မျှော်မှန်းထားသော type ကို ရေးနိုင်ပါသည်။

```zap
let name: text = "Zap"
let port: number = 8080
let enabled: bool = true
let tags: list<text> = ["language", "runtime"]
```

အောက်ပါ code သည် text ကို number အဖြစ် သတ်မှတ်ထားသောကြောင့် မမှန်ပါ။

```zap
let port: number = "8080"
```

Project run မလုပ်မီ `zap check` ဖြင့် စစ်ပါ။[3]

## 6. Operator နှင့် Expression

| Operator | အဓိပ္ပာယ် |
|---|---|
| `+`, `-`, `*`, `/`, `%` | Arithmetic; `+` သည် သင့်တော်သည့်အခါ text join လည်း လုပ်သည် |
| `==`, `!=`, `<`, `<=`, `>`, `>=` | နှိုင်းယှဉ်ခြင်း |
| `and`, `or`, `not` | Boolean logic နှင့် short-circuit |
| `(...)` | Grouping |
| `[]` | List/map indexing |
| `.` | Member access နှင့် method call |

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

Call၊ indexing နှင့် member access သည် precedence အမြင့်ဆုံး ဖြစ်ပါသည်။ Parentheses သုံးခြင်းဖြင့် ရည်ရွယ်ချက်ရှင်းလင်းစေပါသည်။ Integer overflow၊ division by zero၊ invalid indexing နှင့် invalid member access တို့သည် checked runtime failure ဖြစ်ပါသည်။

## 7. Condition နှင့် Loop

```zap
let score = 85

if score >= 80:
    say "Excellent"
else:
    say "Keep practising"
```

`for` သည် list သို့မဟုတ် range ကို iterate လုပ်ပြီး `while` သည် condition မှန်နေသရွေ့ run ပါသည်။

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

Loop ရပ်ရန် `break` နှင့် လက်ရှိ iteration ကို ကျော်ရန် `continue` သုံးပါ။

```zap
for number in range(10):
    if number == 2:
        continue
    if number == 5:
        break
    say number
```

Zap တွင် loop iteration နှင့် execution depth limit များ ရှိပါသည်။ မပြီးဆုံးနိုင်သော loop ကို production strategy အဖြစ် မသုံးပါနှင့်။[4]

## 8. Function

`fn` ဖြင့် function ကြေညာပါ။

```zap
fn add(a: number, b: number) -> number:
    return a + b

fn greet(name: text) -> text:
    return "Hello, " + name

say add(4, 6)
say greet("Zap")
```

Function name သည် first-class callable ဖြစ်သောကြောင့် assign၊ pass၊ return နှင့် alias ဖြင့် invoke လုပ်နိုင်ပါသည်။

```zap
fn double(value: number) -> number:
    return value * 2

let operation = double
say operation(7)
```

### 8.1 Default နှင့် Named Argument

```zap
fn greet(name: text = "World", punctuation: text = "!") -> text:
    return "Hello, " + name + punctuation

say greet()
say greet("Zap", ".")
```

Named argument သုံးခြင်း—

```zap
fn connect(host: text, port: number = 8080, secure: bool = true):
    return {"host": host, "port": port, "secure": secure}

let local = connect("localhost", secure = false)
say local["port"]
```

Required parameter များကို ထည့်ပေးရမည်။ Duplicate၊ unknown သို့မဟုတ် ထပ်ပေးသော argument များသည် error ဖြစ်ပါသည်။[5]

### 8.2 Closure

```zap
fn make_greeting(prefix: text):
    fn greet(name: text) -> text:
        return prefix + ", " + name
    return greet

let say_hello = make_greeting("Hello")
say say_hello("Developer")
```

Closure များသည် parent-linked lexical frame ကို အသုံးပြုပါသည်။ Captured state ကို သေးငယ်ပြီး explicit ထားပါ။[6]

## 9. List၊ Map နှင့် JSON

```zap
let languages = ["Zap", "Rust", "Go"]
say languages[0]
say len(languages)

let profile = {"name": "Zap User", "age": 20, "active": true}
say profile["name"]
say keys(profile)
```

အသုံးများသော helper များမှာ `len`၊ `contains`၊ `get`၊ `is_empty`၊ `sum`၊ `reverse`၊ `sort`၊ `join`၊ `keys`၊ `entries`၊ `enumerate` နှင့် `count` ဖြစ်ပါသည်။

```zap
let profile = {"name": "Zap", "active": true}
let encoded = json(profile)
let decoded = from_json(encoded)

say encoded
say decoded["name"]
```

```zap
let scores: list<number> = [10, 20, 30]
let response: map<text, number> = {"status": 200}
```

JSON serialization သည် bounded နှင့် cycle-safe ဖြစ်ပြီး callable value ကို executable code အဖြစ် ပြန် deserialize မလုပ်ပါ။[7]

## 10. Class နှင့် Object

```zap
class User:
    fn init(self, name: text):
        self.name = name

    fn greet(self) -> text:
        return "Hello, " + self.name

let user = new("User", "Zap")
say user.greet()
```

Inheritance ကို `extends` ဖြင့် ရေးပါ။

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

Explicit field နှင့် method ကို အသုံးပြုပြီး undocumented dynamic property များကို မမှီခိုပါနှင့်။ Traits နှင့် composition အတွက် design direction ရှိသော်လည်း complete user-defined syntax အဖြစ် မ enable လုပ်သေးသောကြောင့် classes၊ modules၊ functions နှင့် explicit composition ကို အသုံးပြုပါ။[8]

## 11. Module၊ Import နှင့် Workspace

```zap
# modules/math.zp
module app.math

export fn square(value: number) -> number:
    return value * value
```

```zap
# main.zp
module app.main
import app.math as math

say math.square(5)
```

အခြား module မှ အသုံးပြုစေလိုသော symbol များကိုသာ export လုပ်သင့်ပါသည်။ Resolver သည် absolute path၊ traversal path၊ missing entry နှင့် circular import များကို reject လုပ်ပါသည်။ Library အသစ်များတွင် `module`၊ `import` နှင့် `export` ကို ဦးစားပေးပြီး legacy `use` ကို compatibility အတွက်သာ သုံးပါ။[9]

```toml
[package]
name = "workspace-demo"
version = "0.1.0"
main = "main.zp"

[module]
root = "modules"
entries = ["app/math.zp"]
```

Module entry များသည် relative ဖြစ်ရမည်၊ `.zp` ဖြင့် အဆုံးသတ်ရမည်၊ file တကယ်ရှိရမည်။

## 12. Result နှင့် Option

`Result` သည် success/failure ကို ကိုယ်စားပြုပြီး `Option` သည် value ရှိ/မရှိကို ကိုယ်စားပြုပါသည်။

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

Typed payload annotation—

```zap
let answer: result<number> = ok(42)
let failure: result<text> = err("not found")
let user: option<text> = some("Zap")
let missing: option<number> = option_none()
```

`?` operator သည် error Result ကို လက်ရှိ function မှ အပေါ်သို့ propagate လုပ်ပေးပါသည်။

```zap
fn load_user() -> result<map>:
    return err("user not found")

fn profile() -> result<map>:
    let user = load_user()?
    return ok(user)

say profile()
```

`ok(value)?` သည် value ဖြင့် ဆက်လုပ်ပြီး `err(error)?` သည် error ကို ချက်ချင်းပြန်ပေးပါသည်။ Result မဟုတ်သော value ပေါ် `?` သုံးခြင်းသည် invalid ဖြစ်ပါသည်။

## 13. Error နှင့် Diagnostic

Zap သည် `SyntaxError`၊ `NameError`၊ `TypeError`၊ `ValueError`၊ `IOError`၊ `FileNotFound`၊ `PermissionError`၊ `OverflowError`၊ `Error` နှင့် `ProjectError` ကဲ့သို့ structured diagnostic kind များကို အသုံးပြုပါသည်။

```bash
zap check .
zap build .
zap check --json .
```

```json
{"ok":false,"kind":"TypeError","file":"main.zp","line":4,"column":12,"message":"expected number, got text"}
```

CLI checker နှင့် LSP သည် semantic diagnostic category တူညီစွာ အသုံးပြုပါသည်။ Error ကို handle လုပ်ရာတွင် original message နှင့် operation/file/route/package context ကို ထိန်းသိမ်းပါ။

## 14. Standard Library အခြေခံများ

Public standard library ကို `text`၊ `math`၊ `collections`၊ `filesystem`၊ `json`၊ `system`၊ `time`၊ `logging`၊ `runtime`၊ `async`၊ `network` နှင့် `process` domain များအဖြစ် ဖွဲ့စည်းထားပါသည်။[10]

### 14.1 Text နှင့် Math

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

### 14.2 File နှင့် Path

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

Application code တွင် file path များကို project root တစ်ခုအတွင်း ထားပြီး user ထည့်သော name ကို validate လုပ်ပါ။

### 14.3 Environment နှင့် Time

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

Environment variable သည် external input ဖြစ်သောကြောင့် secret များကို log သို့မဟုတ် source control ထဲ မထည့်ပါနှင့်။

### 14.4 HTTP နှင့် Process

```zap
let response = http_get("https://example.com")
say response
```

Network နှင့် process operation များတွင် destination၊ timeout နှင့် non-success response ကို handle လုပ်ပါ။ Untrusted string ဖြင့် shell command မတည်ဆောက်ပါနှင့်။

## 15. Test၊ Format၊ Lint နှင့် Build

```zap
fn add(a: number, b: number) -> number:
    return a + b

assert(add(2, 3) == 5, "addition failed")
assert(type(add(2, 3)) == "number", "type failed")
say "test passed"
```

```bash
zap test
zap test tests
zap test tests --filter arithmetic
zap test tests --fail-fast
zap test tests --json
zap fmt main.zp
zap lint main.zp
zap check .
zap build --locked .
```

Test failure သည် exit code `1`၊ command usage error သည် exit code `2` ဖြစ်ပါသည်။ `zap build --locked` သည် canonical lockfile လိုအပ်ပါသည်။

## 16. Project Manifest နှင့် Dependency

```toml
[package]
name = "hello-app"
version = "0.1.0"
main = "main.zp"
```

Web project အတွက်—

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

Dependency ကို CLI ဖြင့် manage လုပ်ပါ။

```bash
zap add json-tools 1.2
zap lock
zap install
zap update
```

`zap add` သည် manifest ပြောင်းပြီး lockfile ကို invalidate လုပ်ပါသည်။ `zap lock` သည် canonical lock data ထုတ်ပေးပြီး `zap install` သည် validation-only အဖြစ် အလုပ်လုပ်ပါသည်။ `zap update` သည် manifest ပြောင်းပြီးနောက် lockfile ကို ပြန်ထုတ်ပါသည်။ `zap.toml` နှင့် `zap.lock` ကို အတူ commit တင်ပါ။

Offline registry validation—

```bash
ZAP_OFFLINE=1 zap install
zap registry check path/to/index.json
zap registry fetch https://registry.example/index.json
```

Package publish နှင့် registry serve အတွက် [Burmese package guide](PACKAGE.md) နှင့် [registry authentication contract](REGISTRY_AUTH_MM.md) ကို ဖတ်ပါ။

## 17. Zap ဖြင့် Web Development

| Directory/file | တာဝန် |
|---|---|
| `models/` | Data shape၊ field metadata နှင့် validation |
| `functions/` | Business logic၊ use case နှင့် request handler |
| `ui/` | Browser UI metadata နှင့် entrypoint contract |
| `routes/` | HTTP route declaration |
| `middleware/` | Ordered request/response policy |
| `migrations/` | Versioned schema intent |
| `admin/` | Administration registration |
| `public/` | HTML၊ CSS၊ JavaScript နှင့် browser asset များ |
| `tests/` | Project နှင့် HTTP contract tests |

Generated route file—

```zap
export fn routes():
    return [
        {"method": "GET", "path": "/", "handler": "home", "scope": ""},
        {"method": "GET", "path": "/api/tasks", "handler": "tasks", "scope": "tasks:read"},
        {"method": "GET", "path": "/assets/*path", "handler": "asset", "scope": ""},
        {"method": "GET", "path": "/*path", "handler": "spa", "scope": ""}
    ]
```

JSON response handler—

```zap
export fn tasks(request):
    return {"status": 200, "body": json({"tasks": [], "request_id": request["request_id"]})}
```

Static asset handler—

```zap
export fn asset(request):
    return web_static("assets/" + request["params"]["path"], "public")

export fn spa(request):
    return web_static_spa(request["params"]["path"], "public", "index.html")
```

API နှင့် asset route များကို SPA wildcard မတိုင်မီ ထားပါ။ Cache fingerprint၊ TLS termination၊ horizontal scaling နှင့် CDN behavior များသည် deployment concern ဖြစ်ပါသည်။[11]

### 17.1 Plain JavaScript နှင့် အခြား frontend framework

```html
<!doctype html>
<html lang="my">
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

React၊ Vue၊ Svelte၊ Alpine သို့မဟုတ် အခြား frontend system ကို optional build-time toolchain အဖြစ် သုံးနိုင်ပါသည်။ Build output ကို `public/` ထဲ ထည့်ပြီး Zap နှင့် deploy လုပ်ပါ။ Deployed runtime သည် npm၊ Node.js၊ bundler သို့မဟုတ် framework compiler ကို run မလုပ်ပါ။[2]

## 18. Database နှင့် Migration

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

```bash
zap db check
zap db plan
zap db inspect --json
zap db migrate --dry-run
zap db migrate
zap db migrate --check
```

လက်ရှိ implementation သည် deterministic additive SQLite operation များအတွက် ရည်ရွယ်ထားပြီး provider-neutral production migration platform မဟုတ်သေးပါ။ Data backup၊ SQL plan review နှင့် deployment runbook ကို အမြဲအသုံးပြုပါ။[12]

## 19. Async Programming

```zap
async fn load() -> number:
    return 7

let pending = load()
let value: number = await pending
say value
```

```zap
async fn answer() -> number:
    return 42

let handle = answer()
say task_is_ready(handle)
say task_join_timeout(handle, 1)
```

`task_join` သည် task result ကိုစားသုံးပြီး `task_cancel` သည် cooperative cancellation ပြုလုပ်ကာ `task_join_timeout` သည် poll budget သတ်မှတ်ပါသည်။ Current scheduler သည် production worker pool သို့မဟုတ် socket-readiness reactor မဟုတ်ပါ။[13]

## 20. LSP နှင့် Editor Workflow

Zap တွင် stdio Language Server Protocol implementation ပါဝင်ပြီး diagnostics၊ hover၊ completion၊ signature help၊ definition၊ document/workspace symbols၊ formatting နှင့် documented boundary အတွင်း rename ကို support လုပ်ပါသည်။ LSP သည် bounded incremental synchronization ကို advertise လုပ်ပါသည်။ Change notification တစ်ခုတွင် sequential full-document သို့မဟုတ် range edit အများဆုံး 128 ခု ပါနိုင်ပြီး negotiated UTF-8/UTF-16/UTF-32 position များကို character boundary အတိုင်း စစ်ဆေးပါသည်။ Version များသည် အစဉ်တိုးရမည်ဖြစ်ပြီး 32 MiB workspace byte cap ကို edit တစ်ခုချင်းစီပြီးတိုင်း enforce လုပ်ပါသည်။ Malformed၊ stale၊ oversized၊ out-of-range သို့မဟုတ် မဖွင့်ထားသော document အတွက် range edit များကို stored text မပြောင်းဘဲ reject လုပ်ပါသည်။ Cross-file rename သည် complete refactoring feature မဟုတ်သေးပါ။

```bash
python3 scripts/validate_vscode_assets.py
scripts/test_lsp_semantic_parity.sh
scripts/test_lsp_protocol_sync.sh
```

Editor error တွေ့ပါက `zap check --json` ဖြင့် အရင် reproduce လုပ်ပါ။

## 21. Runtime Safety နှင့် Advanced Practice

Zap သည် source size၊ loop iteration၊ execution depth၊ collection production၊ text value၊ HTTP request၊ response body၊ registry transport နှင့် task operation အချို့ကို bounds ထားပါသည်။ Runtime သည် per-run context ထဲတွင် module cache၊ import-cycle tracking၊ workspace confinement၊ logical memory accounting၊ object storage နှင့် async state ကို ပိုင်ဆိုင်ပါသည်။

Advanced code တွင် external boundary တိုင်းကို validate လုပ်ပါ၊ public function အနည်းဆုံးသာ export လုပ်ပါ၊ expected failure အတွက် `Result` နှင့် absence အတွက် `Option` သုံးပါ၊ unbounded loop/blocking operation မသုံးဘဲ bound/timeout သတ်မှတ်ပါ၊ secret ကို source/log ထဲမထည့်ပါနှင့်။ Commit မတင်မီ အောက်ပါ workflow ကို run ပါ။[6]

```bash
zap fmt main.zp
zap check .
zap test
zap build --locked
```

## 22. ပြည့်စုံသော Small Example

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

```bash
zap fmt main.zp
zap check .
zap run main.zp
```

Program ကြီးလာပါက reusable declaration များကို module ထဲ ရွှေ့ပြီး test များကို `tests/` ထဲ ထားပါ။

## 23. Command Reference

| Command | ရည်ရွယ်ချက် |
|---|---|
| `zap file.zp` | Source file run လုပ်ခြင်း |
| `zap run file.zp` | Explicit source execution |
| `zap new directory` | User-managed Web scaffold အပြည့်အစုံ ဖန်တီးခြင်း |
| `zap init directory` | Minimal generic project ဖန်တီးခြင်း |
| `zap check [directory]` | Zap project directory ကို စစ်ခြင်း |
| `zap check --json [directory]` | Structured diagnostic ထုတ်ခြင်း |
| `zap build [directory]` | Build readiness စစ်ခြင်း |
| `zap build --locked [directory]` | Canonical lockfile မဖြစ်မနေလိုအပ်ခြင်း |
| `zap test [directory]` | Test file များ run လုပ်ခြင်း |
| `zap fmt file.zp` | Source format ပြုလုပ်ခြင်း |
| `zap lint file.zp` | Style issue ပြခြင်း |
| `zap lock [directory]` | Canonical lock data ထုတ်ခြင်း |
| `zap install [directory]` | Locked dependency validate/install လုပ်ခြင်း |
| `zap update [directory]` | Lock data ပြန်ထုတ်ခြင်း |
| `zap web check [directory]` | Web manifest/scaffold စစ်ခြင်း |
| `zap dev [directory]` | Web development server run လုပ်ခြင်း |
| `zap db check/plan/inspect/migrate` | Database migration workflow |
| `zap lsp` | LSP ကို stdio ဖြင့် run လုပ်ခြင်း |
| `zap async-check` | Async boundary စစ်ခြင်း |
| `zap --version` | Installed version ပြခြင်း |
| `zap --help` | Command help ပြခြင်း |

## 24. ပြဿနာဖြေရှင်းခြင်း

| ပြဿနာ | ဖြေရှင်းနည်း |
|---|---|
| `zap: command not found` | Zap `bin` directory ကို `PATH` ထဲထည့်ပြီး shell ပြန်ဖွင့်ပါ |
| `unknown command` | `zap --version` စစ်ပြီး matching release update လုပ်ပါ |
| Type reject | Annotation နှင့် value ကို ကိုက်ညီအောင် ပြင်ပါ |
| `module not found` | Relative path၊ module root နှင့် `.zp` file name စစ်ပါ |
| Circular dependency | Shared declaration ကို lower-level module သို့ ခွဲပါ |
| Locked build reject | Manifest review လုပ်ပြီး `zap lock` run ပါ |
| `zap dev` reject | `zap web check` နှင့် `[web]` path များ စစ်ပါ |
| Migration pending | `zap db plan` review လုပ်ပြီး backup ပြီးမှ migrate လုပ်ပါ |
| Stale LSP diagnostic | Document reopen လုပ်ပြီး `zap check --json` ဖြင့် reproduce လုပ်ပါ |
| Frontend deployment မအောင်မြင် | Final HTML/CSS/JS output ကို `public/` ထဲ copy လုပ်ပါ |

## 25. Stable နှင့် ဆက်လက်ဖွံ့ဖြိုးနေသောအပိုင်းများ

Stable direction ထဲတွင် `.zp` source၊ native CLI execution၊ indentation block၊ core value၊ function၊ class၊ module၊ typed check၊ Result/Option foundation၊ deterministic project validation၊ lockfile၊ JSON diagnostic၊ test၊ LSP foundation၊ one-command Web scaffold နှင့် bounded native Web serving ပါဝင်ပါသည်။

Active/deferred အပိုင်းများမှာ complete user-defined trait system၊ production asynchronous I/O reactor၊ provider-neutral database migration၊ full ORM၊ template/component compiler၊ cross-file semantic rename၊ hidden app registry နှင့် JavaScript build tool အားလုံးကို အစားထိုးခြင်းတို့ ဖြစ်ပါသည်။ Contract နှင့် executable evidence မပြည့်စုံသေးသော feature ကို မထည့်ထားခြင်းသည် ရည်ရွယ်ချက်ရှိသော limitation ဖြစ်ပါသည်။[1]

## References

[1]: LANGUAGE_SPEC_MM.md "Zap Language Specification"
[2]: FRONTEND_INTEGRATION_MM.md "Zap Frontend Integration Guide"
[3]: TYPECHECK_CONFORMANCE_MATRIX_MM.md "Zap Type-Check Conformance Matrix"
[4]: RUNTIME_STATE_MM.md "Zap Runtime State and Execution Context"
[5]: DEFAULT_PARAMETERS_MM.md "Zap Default Function Parameters"
[6]: MEMORY_BUDGET_OBJECT_STORE_MM.md "Zap Memory Budget and Object Store Contract"
[7]: STDLIB_POLICY_MM.md "Zap Standard Library Policy"
[8]: TRAITS_RFC_MM.md "Zap Traits and Composition RFC"
[9]: PACKAGE.md "Zap Package Author Guide"
[10]: STDLIB_INDEX_MM.md "Zap Standard Library Index"
[11]: ZAP_WEB_NATIVE_MM.md "Zap Native Web Guide"
[12]: DATABASE_PRODUCTION_MM.md "Zap Database Production Guide"
[13]: ASYNC_BOUNDARIES_MM.md "Zap Async Boundary Contract"
