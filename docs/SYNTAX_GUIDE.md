# Zap Syntax Guide

ဤ guide သည် Zap `0.8.0` native runtime တွင် လက်ရှိအသုံးပြုနိုင်သော syntax၊ standard built-ins နှင့် developer tooling များကို အခြေခံထားသည်။ Zap source file များသည် `.zp` extension ကို အသုံးပြုရပြီး ဥပမာ `main.zp` ဖြစ်သည်။

> Zap သည် ရိုးရှင်းသော indentation-based block syntax၊ ရှင်းလင်းသော keywords နှင့် standalone CLI workflow ကို အဓိကထားသော programming language ဖြစ်သည်။

## 1. Program တစ်ခုကို Run လုပ်ခြင်း

`main.zp` ဖိုင်တစ်ခုဖန်တီးပြီး အောက်ပါ command ဖြင့် run နိုင်သည်။

```bash
zap main.zp
```

အကူအညီနှင့် version ကို ကြည့်ရန်—

```bash
zap --help
zap --version
```

Project အသစ်စတင်ရန် `zap init hello-zap`၊ manifest စစ်ဆေးရန် `zap check`၊ machine-readable diagnostics အတွက် `zap check --json .`၊ test files များ run ရန် `zap test`၊ style စစ်ရန် `zap lint main.zp` နှင့် source ကို whitespace အတိုင်းပြန်ညှိရန် `zap fmt main.zp` ကို အသုံးပြုနိုင်သည်။ `zap init` သည် starter `tests/smoke_test.zp` ကိုလည်း ဖန်တီးပေးသည်။

```bash
zap init hello-zap
cd hello-zap
zap check
zap check --json .
zap lint main.zp
zap test
zap run main.zp
```

## 2. Comments

Comment များသည် `#` ဖြင့် စတင်သည်။ Runtime သည် comment တစ်ကြောင်းလုံးကို မလုပ်ဆောင်ပါ။

```zp
# This is a Zap comment
say "Hello"

let count = 3  # Inline comment
```

## 3. Output နှင့် Literals

တန်ဖိုးတစ်ခုကို terminal သို့ ထုတ်ရန် `say` ကို အသုံးပြုသည်။

```zp
say "Hello from Zap"
say 42
say true
say none
```

လက်ရှိ core တွင် အောက်ပါ value အမျိုးအစားများကို အသုံးပြုနိုင်သည်။

| Value type | နမူနာ | အဓိပ္ပာယ် |
|---|---|---|
| Text | `"Zap"` | စာသားတန်ဖိုး |
| Integer | `42` | ကိန်းပြည့် |
| Boolean | `true`, `false` | မှန်/မမှန် |
| None | `none` | တန်ဖိုးမရှိခြင်း |
| List | `[1, 2, 3]` | အစဉ်လိုက် collection |
| Map | `{"name": "Zap"}` | key/value collection |

String များကို double quotes ဖြင့် ရေးသည်။ String များကို `+` ဖြင့် ဆက်နိုင်သည်။

```zp
let language = "Zap"
let message = "Learning " + language
say message
```

## 4. Variables နှင့် Assignment

Variable အသစ်ကြေညာရန် `let name = value` ကို အသုံးပြုသည်။ ရှိပြီးသား variable ၏ တန်ဖိုးကို ပြန်ပြောင်းရန် `name = value` ကို အသုံးပြုနိုင်သည်။

```zp
let name = "Zap"
let version = 3
say name
say version

version = 4
say version
```

Variable များသည် လက်ရှိ block/function scope အတွင်း အသုံးပြုနိုင်သည်။ Variable name များကို စာလုံး၊ ဂဏန်းနှင့် underscore ဖြင့် ဖွဲ့စည်းပြီး စာလုံးဖြင့် စတင်ရန် အကြံပြုသည်။

## 5. Arithmetic Operators

Zap တွင် အခြေခံ arithmetic operator များ ပါဝင်သည်။

```zp
let a = 10
let b = 3

say a + b   # 13
say a - b   # 7
say a * b   # 30
say a / b   # 3.333...
say a % b   # 1
```

Operator precedence သည် အများအားဖြင့် multiplication၊ division နှင့် modulus ကို addition/subtraction ထက် ဦးစားပေးသည်။ အစီအစဉ်ကို ရှင်းလင်းစေရန် parentheses သုံးနိုင်သည်။

```zp
let result = (2 + 3) * 4
say result
```

## 6. Comparison နှင့် Boolean Logic

Comparison operator များမှာ `==`၊ `!=`၊ `<`၊ `>`၊ `<=` နှင့် `>=` ဖြစ်သည်။ ရလဒ်သည် boolean value ဖြစ်သည်။

```zp
let score = 75
say score >= 50
say score == 100
say score != 0
```

Boolean expression များကို `and`၊ `or` နှင့် `not` ဖြင့် ပေါင်းစပ်နိုင်သည်။

```zp
let logged_in = true
let is_admin = false

say logged_in and not is_admin
say logged_in or is_admin
say not false
```

## 7. If/Else Conditional

Block စတင်ရာတွင် colon (`:`) ထည့်ပြီး အတွင်းရှိ statement များကို indentation ဖြင့် ရေးသည်။ `else` သည် `if` နှင့် တူညီသော indentation အဆင့်တွင် ရှိရမည်။

```zp
let temperature = 30

if temperature > 35:
    say "Hot"
else:
    say "Comfortable"
```

Nested conditional ကိုလည်း ရေးနိုင်သည်။

```zp
let score = 82

if score >= 80:
    if score == 100:
        say "Perfect"
    else:
        say "Excellent"
else:
    say "Keep practicing"
```

## 8. Lists နှင့် Indexing

List သည် `[]` အတွင်း comma ဖြင့် ခွဲထားသော value များဖြစ်သည်။ Index သည် `0` မှ စတင်သည်။

```zp
let tools = ["web", "ai", "iot"]
say tools[0]
say tools[2]
say len(tools)
```

List ထဲရှိ value များသည် number၊ text၊ boolean၊ list သို့မဟုတ် map ဖြစ်နိုင်သည်။

```zp
let project = ["Zap", 3, true]
say project[0]
say project[1]
say project[2]
```

## 9. Maps နှင့် Key Access

Map သည် `{key: value}` ပုံစံဖြစ်သည်။ String key များကို double quotes ဖြင့် ရေးနိုင်သည်။ Value ကို `map["key"]` ပုံစံဖြင့် ဖတ်နိုင်သည်။

```zp
let user = {"name": "Ada", "language": "Zap", "active": true}

say user["name"]
say user["language"]
say user["active"]
```

Nested map နှင့် list များကို ပေါင်းစပ်အသုံးပြုနိုင်သည်။

```zp
let app = {"name": "ZapBoard", "features": ["web", "ai"]}

say app["name"]
say app["features"][0]
```

## 10. For Loops

List သို့မဟုတ် `range()` မှ ရလာသော collection ကို လှည့်ပတ်ရန် `for item in values:` ကို အသုံးပြုသည်။

```zp
let languages = ["Zap", "Web", "AI"]

for item in languages:
    say item
```

`range(n)` သည် `0` မှ `n - 1` အထိ integer list တစ်ခုကို ပြန်ပေးသည်။

```zp
for number in range(5):
    say number
```

Loop အတွင်း `continue` ဖြင့် လက်ရှိ iteration ကို ကျော်နိုင်ပြီး `break` ဖြင့် loop ကို ရပ်နိုင်သည်။

```zp
for number in range(6):
    if number == 2:
        continue
    if number == 5:
        break
    say number
```

## 11. While Loops

အခြေအနေမှန်နေသရွေ့ code ကို ပြန်လုပ်ရန် `while condition:` ကို အသုံးပြုသည်။ Infinite loop မဖြစ်စေရန် condition ကို ပြောင်းလဲပေးရမည်။

```zp
let count = 0

while count < 3:
    say count
    count = count + 1
```

## 12. Functions

Function ကြေညာရန် `fn name(parameters):` ကို အသုံးပြုသည်။ Function ထဲမှ value ပြန်ပေးရန် `return` ကို အသုံးပြုနိုင်သည်။

```zp
fn greet(name):
    return "Hello, " + name

let message = greet("Developer")
say message
```

Parameter များစွာပါသော function—

```zp
fn add(a, b):
    return a + b

say add(7, 8)
```

Function သည် return မပြုလုပ်လျှင် အလုပ်လုပ်ပြီးနောက် တန်ဖိုးမရှိသော result ရှိနိုင်သည်။

## 13. Nested Functions နှင့် Closures

Function တစ်ခုအတွင်း function တစ်ခု ကြေညာနိုင်သည်။ အတွင်း function သည် အပြင် function ၏ variable ကို capture လုပ်နိုင်သည်။

```zp
fn make_adder(base):
    fn add(value):
        return base + value
    return add(10)

let result = make_adder(5)
say result
```

အထက်ပါ program သည် `15` ကို ထုတ်ပေးမည်။ ဤပုံစံသည် reusable calculation နှင့် callback-style logic များအတွက် အခြေခံဖြစ်သည်။

## 14. OOP: Classes၊ Objects နှင့် Methods

Zap v0.7.0 တွင် class-based OOP foundation ပါဝင်သည်။ Class declaration သည် indentation-based ဖြစ်ပြီး method များကို `fn` ဖြင့် ရေးရသည်။ Object method ၏ ပထမ parameter သည် `self` ဖြစ်သည်။

```zp
class User:
    fn init(self, name):
        self.name = name

    fn greet(self):
        return "Hello, " + self.name

let user = new("User", "Zap")
say user.greet()
say user.name
```

`new("ClassName", arguments...)` သည် class name မှန်ကန်မှုကို စစ်ပြီး object အသစ်ဖန်တီးသည်။ `init(self, arguments...)` ရှိပါက constructor အဖြစ် အလိုအလျောက်ခေါ်သည်။ Initial property များကို map ဖြင့်လည်း ပေးနိုင်သည်။ မရှိသော class ကို ဖန်တီးရန် ကြိုးစားပါက `unknown class` error ပြန်ပေးသည်။ Child class တွင် ကိုယ်ပိုင် `init` ရှိပါက parent ၏ `init` ကို အရင်ခေါ်ပြီးမှ child `init` ကို ခေါ်သည်။

```zp
class Device:
    fn status(self):
        return self.name + " is " + self.state

let device = new("Device", {"name": "sensor", "state": "ready"})
say device.status()
```

Property ဖတ်ခြင်းနှင့် ပြင်ခြင်း—

```zp
class Counter:
    fn increment(self):
        self.value = self.value + 1
        return self.value

let counter = new("Counter", {"value": 0})
say counter.increment()
say counter.value
```

Inheritance နှင့် method override ကို `extends` ဖြင့် ရေးနိုင်သည်။ Child class သည် parent method များကို အသုံးပြုနိုင်ပြီး အမည်တူ method ရှိပါက child method ကို ဦးစားပေးသည်။

```zp
class Animal:
    fn speak(self):
        return "sound"

class Dog extends Animal:
    fn speak(self):
        return "woof"

let dog = new("Dog")
say dog.speak()
```

OOP boundary အနေဖြင့် v0.7.0 တွင် class registry validation၊ object၊ mutable properties၊ methods၊ constructor၊ parent constructor initialization၊ method override နှင့် single inheritance ကို support လုပ်သည်။ မရှိသော parent class ကို `extends` လုပ်ပါက class declaration အဆင့်တွင် error ပြန်ပေးသည်။ Interfaces၊ abstract classes၊ generics၊ private modifiers၊ explicit `super` method calls နှင့် multiple inheritance များကို မထည့်သွင်းသေးပါ။

## 15. Structured Errors: `raise`, `try`, နှင့် `catch`

Zap တွင် runtime error flow အတွက် deterministic structured control flow ကို အသုံးပြုနိုင်ပါသည်။ `raise <expression>` သည် expression ကို evaluate ပြုလုပ်ပြီး ၎င်း value ကို လက်ရှိ function၊ loop နှင့် module boundary များမှတစ်ဆင့် `try`/`catch` က handle မလုပ်မချင်း propagate လုပ်ပါသည်။ Expression မပါသော `raise` ကို parse အဆင့်တွင် `raise expects an expression` diagnostic ဖြင့် ပယ်ချပါသည်။

```zp
fn load_config():
    raise "configuration unavailable"

try:
    load_config()
catch error:
    say "handled: " + error
```

`try` block ၏နောက်တွင် same-level `catch <binding>:` clause နှင့် indent လုပ်ထားသော body ရှိရမည်။ `raise` လုပ်ထားသော value ကို catch binding ထဲသို့ ထည့်ပေးပြီး catch body အတွင်းတွင်သာ ရှိပါသည်။ ရှိပြီးသား variable အမည်ကို shadow လုပ်ပါက catch ပြီးနောက် မူလ value ကို ပြန်လည် restore လုပ်ပါသည်။ Catch body အတွင်း `raise` ထပ်လုပ်ပါက value အသစ် သို့မဟုတ် မူလ value သည် အပြင်ဘက်သို့ ဆက်လက် propagate ဖြစ်ပါသည်။

```zp
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

Try body သည် ပုံမှန်ပြီးဆုံးပါက catch body ကို မလုပ်ဆောင်ပါ။ Catch body အတွင်း `return`၊ `break` နှင့် `continue` များသည် မိမိတို့၏ enclosing function သို့မဟုတ် loop semantics အတိုင်း ဆက်လက်အလုပ်လုပ်ပါသည်။ Process boundary အထိ uncaught ဖြစ်နေသော value ကို `raised error: <value>` ဟု deterministic diagnostic ဖြင့် ဖော်ပြပါသည်။

## 16. Standard Utility Built-ins

Zap native runtime တွင် type စစ်ဆေးခြင်း၊ collection ရှာဖွေခြင်း၊ text ပြောင်းလဲခြင်းနှင့် numeric calculation အတွက် built-ins များ ပါဝင်သည်။

```zp
let user = {"name": "Zap", "version": 3}

say type(user)                 # map
say keys(user)                 # list of map keys
say contains(user, "name")    # true
say contains([1, 2, 3], 2)     # true
say join(["web", "ai"], ",") # web,ai

say abs(-8)                    # 8
say min(4, 9)                  # 4
say max(4, 9)                  # 9

say upper("zap")               # ZAP
say lower("ZAP")               # zap
say trim("  core  ")           # core
say split("a,b,c", ",")[1]    # b

say is_empty([])             # true
say sum([1, 2, 3])           # 6
say reverse("Zap")           # paZ
say join(sort([3, 1, 2]), ",") # 1,2
say get({"name": "Zap"}, "missing", "default") # default
```

`assert(condition, message)` သည် condition မှားလျှင် Zap error ဖြင့် program ကို ရပ်စေသည်။ Test နှင့် configuration validation များအတွက် အသုံးဝင်သည်။

```zp
let version = 3
assert(version >= 1, "version must be positive")
```

## 16. JSON

`json(value)` သည် Zap value ကို JSON text အဖြစ် encode လုပ်ပြီး `from_json(text)` သည် JSON text ကို Zap value အဖြစ် decode လုပ်သည်။

```zp
let user = {"name": "Ada", "skills": ["web", "ai"]}

let encoded = json(user)
say encoded

let decoded = from_json(encoded)
say decoded["name"]
```

JSON data ကို application configuration သို့မဟုတ် API response များအတွက် အသုံးပြုရန် ရည်ရွယ်ထားသည်။

## 17. File I/O

စာသားဖိုင်ရေးရန် `write_text(path, text)` နှင့် ဖတ်ရန် `read_text(path)` ကို အသုံးပြုသည်။

```zp
write_text("message.txt", "Hello from Zap")

let message = read_text("message.txt")
say message
```

Line-based file helper များကိုလည်း အသုံးပြုနိုင်သည်။ `read_lines(path)` သည် text list ကို ပြန်ပေးပြီး `write_lines(path, lines)` သည် list ထဲရှိ text များကို newline ဖြင့်ရေးသည်။ Empty list ရေးလျှင် empty file ဖြစ်သည်။

```zp
write_lines("items.txt", ["one", "two"])
let items = read_lines("items.txt")
say join(items, "|")
```

Relative path များသည် program run လုပ်သော current working directory ကို အခြေခံသည်။ File permission နှင့် path validation များကို production အသုံးပြုမှုမတိုင်မီ စစ်ဆေးသင့်သည်။

## 18. Modules နှင့် Workspaces

Zap တွင် source file ၏ logical name ကို သတ်မှတ်ရန် `module` နှင့် module ကို alias ဖြင့် import လုပ်ရန် `import ... as ...` ကို အသုံးပြုနိုင်သည်။

```text
hello-app/
├── main.zp
└── modules/
    └── app/
        └── core.zp
```

`modules/app/core.zp`—

```zp
module app.core

fn version():
    return "2.0"
```

`main.zp`—

```zp
module app.main
import app.core as core

say core
```

Dotted import path သည် module root အောက်ရှိ `.zp` file သို့ map လုပ်သည်။ ဥပမာ `import app.core as core` သည် `modules/app/core.zp` သို့ resolve ဖြစ်ရန် `zap.toml` တွင်—

```toml
[package]
name = "workspace-demo"
version = "0.1.0"
main = "main.zp"

[module]
root = "modules"
entries = ["app/core.zp"]
```

ဟု ရေးနိုင်သည်။ Module `root` နှင့် `entries` များသည် relative path ဖြစ်ရမည်၊ entry တိုင်းသည် `.zp` ဖြင့်ဆုံးရမည်၊ ဖိုင်တကယ်ရှိရမည်။ Explicit import များတွင် absolute path၊ separator၊ empty component နှင့် traversal များကို လက်မခံပါ။ Resolver သည် import များကို deterministic source order ဖြင့် လိုက်စစ်ပြီး ပြီးစီးပြီးသော node များကို cache လုပ်သည်။ Circular dependency ဖြစ်ပါက cycle တစ်ခုလုံးပါသော `circular module dependency` diagnostic ကို ထုတ်ပေးသည်။ Compatibility အတွက် `use "file.zp"` ကို ဆက်လက်အသုံးပြုနိုင်သော်လည်း workspace အသစ်များတွင် `module` နှင့် `import ... as ...` ကို ဦးစားပေးသင့်သည်။

## 19. Complete Example: CLI Counter

အောက်ပါ program သည် variable၊ function၊ loop၊ conditional နှင့် `continue`/`break` တို့ကို ပေါင်းစပ်အသုံးပြုထားသည်။

```zp
fn label(number):
    if number % 2 == 0:
        return "even"
    else:
        return "odd"

for number in range(8):
    if number == 6:
        break
    if number == 1:
        continue
    say str(number) + " is " + label(number)
```

## 20. Complete Example: JSON File

```zp
let settings = {"app": "Zap Notes", "version": 1, "features": ["files", "json"]}

let text = json(settings)
write_text("settings.json", text)

let loaded_text = read_text("settings.json")
let loaded = from_json(loaded_text)

say loaded["app"]
say loaded["features"][0]
```

## 21. Complete Example: Small Project

`zap.toml`—

```toml
[package]
name = "hello-zap"
version = "0.1.0"
main = "main.zp"
```

`main.zp`—

```zp
fn welcome(name):
    return "Welcome to " + name

let user = "Zap developer"
say welcome(user)
```

Commands—

```bash
zap check
zap fmt main.zp
zap main.zp
```

## 22. လက်ရှိအခြေအနေ နှင့် နောက်တစ်ဆင့်

အထက်ပါ core syntax များသည် Zap native runtime ၏ လက်ရှိအခြေအနေကို ကိုယ်စားပြုသည်။ Web response helpers နှင့် `ai.ask` ကဲ့သို့သော Web/AI foundation API များသည် လက်ရှိတွင် foundation သို့မဟုတ် placeholder အဆင့်ဖြစ်နိုင်ပြီး production networking၊ provider integration၊ async runtime၊ type checking နှင့် package registry တို့ကို roadmap အဖြစ် ဆက်လက်တည်ဆောက်မည်။

## Async functions နှင့် await

Function declaration ရှေ့တွင် `async` ထည့်ပါက deterministic `Future` value ပြန်ပေးသော function ဖြစ်လာပါသည်။ ပြီးဆုံးသော result ကို ရယူရန် `await` ကို အသုံးပြုပါ။

```zap
async fn load() -> number:
    return 7

let pending = load()
let value: number = await pending
say value
```

`await` သည် expression ဖြစ်သောကြောင့် call ပေါ်တွင် တိုက်ရိုက်အသုံးပြုနိုင်ပါသည်။

```zap
async fn answer() -> number:
    return 42

say (await answer()) + 1
```

လက်ရှိ runtime သည် async body များကို deterministic အတိုင်း run လုပ်ပြီး background thread မဖန်တီးသေးပါ။ Timers၊ cancellation နှင့် ပိုမိုပြည့်စုံသော suspension behavior များမှာ နောက်ထပ် roadmap အလုပ်များဖြစ်ပါသည်။ Executor နှင့် editor protocol အသေးစိတ်အတွက် [Async/LSP Burmese guide](ASYNC_LSP_MM.md) ကို ဖတ်ရှုနိုင်ပါသည်။

Named arguments များကို function နှင့် method call များတွင် အသုံးပြုနိုင်ပါသည်။ Default parameter validation နှင့် runnable examples များအတွက် [Default Parameters Burmese guide](DEFAULT_PARAMETERS_MM.md) ကို ဖတ်ရှုနိုင်ပါသည်။
