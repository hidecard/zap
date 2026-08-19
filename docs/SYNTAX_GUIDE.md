# Zap Syntax Guide

ဤ guide သည် Zap `0.4.x` native runtime တွင် လက်ရှိအသုံးပြုနိုင်သော syntax နှင့် standard built-ins များကို အခြေခံထားသည်။ Zap source file များသည် `.zp` extension ကို အသုံးပြုရပြီး ဥပမာ `main.zp` ဖြစ်သည်။

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

Project အသစ်စတင်ရန် `zap init hello-zap`၊ manifest စစ်ဆေးရန် `zap check` နှင့် source ကို whitespace အတိုင်းပြန်ညှိရန် `zap fmt main.zp` ကို အသုံးပြုနိုင်သည်။

```bash
zap init hello-zap
cd hello-zap
zap check
zap main.zp
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

## 14. Standard Utility Built-ins

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
```

`assert(condition, message)` သည် condition မှားလျှင် Zap error ဖြင့် program ကို ရပ်စေသည်။ Test နှင့် configuration validation များအတွက် အသုံးဝင်သည်။

```zp
let version = 3
assert(version >= 1, "version must be positive")
```

## 15. JSON

`json(value)` သည် Zap value ကို JSON text အဖြစ် encode လုပ်ပြီး `from_json(text)` သည် JSON text ကို Zap value အဖြစ် decode လုပ်သည်။

```zp
let user = {"name": "Ada", "skills": ["web", "ai"]}

let encoded = json(user)
say encoded

let decoded = from_json(encoded)
say decoded["name"]
```

JSON data ကို application configuration သို့မဟုတ် API response များအတွက် အသုံးပြုရန် ရည်ရွယ်ထားသည်။

## 16. File I/O

စာသားဖိုင်ရေးရန် `write_text(path, text)` နှင့် ဖတ်ရန် `read_text(path)` ကို အသုံးပြုသည်။

```zp
write_text("message.txt", "Hello from Zap")

let message = read_text("message.txt")
say message
```

Relative path များသည် program run လုပ်သော current working directory ကို အခြေခံသည်။ File permission နှင့် path validation များကို production အသုံးပြုမှုမတိုင်မီ စစ်ဆေးသင့်သည်။

## 17. Modules

အခြား `.zp` file ထဲရှိ function များကို `use` ဖြင့် load လုပ်နိုင်သည်။ ဥပမာ project structure—

```text
hello-app/
├── main.zp
└── modules/
    └── math.zp
```

`modules/math.zp`—

```zp
fn triple(value):
    return value * 3
```

`main.zp`—

```zp
use "math.zp"

say triple(4)
```

Runtime သည် main source file ရှိသော directory၊ `modules/` နှင့် `lib/` directories များအတွင်း module ကို ရှာဖွေသည်။ Project manifest ရှိလျှင် `zap check` ဖြင့် entry file နှင့် project structure ကို စစ်ဆေးနိုင်သည်။

## 18. Complete Example: CLI Counter

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

## 19. Complete Example: JSON File

```zp
let settings = {"app": "Zap Notes", "version": 1, "features": ["files", "json"]}

let text = json(settings)
write_text("settings.json", text)

let loaded_text = read_text("settings.json")
let loaded = from_json(loaded_text)

say loaded["app"]
say loaded["features"][0]
```

## 20. Complete Example: Small Project

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

## 21. လက်ရှိအခြေအနေ နှင့် နောက်တစ်ဆင့်

အထက်ပါ core syntax များသည် Zap native runtime ၏ လက်ရှိအခြေအနေကို ကိုယ်စားပြုသည်။ Web response helpers နှင့် `ai.ask` ကဲ့သို့သော Web/AI foundation API များသည် လက်ရှိတွင် foundation သို့မဟုတ် placeholder အဆင့်ဖြစ်နိုင်ပြီး production networking၊ provider integration၊ async runtime၊ type checking နှင့် package registry တို့ကို roadmap အဖြစ် ဆက်လက်တည်ဆောက်မည်။
