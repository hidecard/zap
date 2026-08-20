# Zap Language Usage Guide

> **Zap** သည် `.zp` source file များကို native runtime ဖြင့် တိုက်ရိုက် run နိုင်သော general-purpose programming language ဖြစ်သည်။ ဤ guide သည် လက်ရှိ Zap runtime တွင် အသုံးပြုနိုင်သော syntax နှင့် developer workflow ကို beginner များအတွက် အဆင့်လိုက်ရှင်းပြထားသည်။

## 1. Zap Program တစ်ခုကို Run ခြင်း

Zap source file သည် `.zp` extension အသုံးပြုရသည်။ `main.zp` သည် project entry file အဖြစ် အသုံးများသည်။

```bash
zap --version
zap --help
zap main.zp
```

Project အသစ်စတင်ရန် `zap init` ကို အသုံးပြုနိုင်သည်။

```bash
zap init hello-zap
cd hello-zap
zap check
zap main.zp
```

Windows တွင် release archive ကို extract ပြီး installer မလိုဘဲ executable ကို တိုက်ရိုက် run နိုင်သည်။

```bat
bin\zap.exe --version
bin\zap.exe main.zp
```

## 2. Program Structure နှင့် Comments

Zap သည် indentation-based blocks ကို အသုံးပြုသည်။ Block စတင်သည့် statement နောက်တွင် colon (`:`) ထည့်ပြီး block အတွင်းရှိ lines များကို indentation ဖြင့် ရေးရသည်။ အများအားဖြင့် indentation တစ်ဆင့်ကို spaces လေးလုံး အသုံးပြုရန် အကြံပြုသည်။

```zp
# ဤသည်မှာ comment ဖြစ်သည်။

if true:
    say "ဒီ block ထဲက code ဖြစ်သည်"
```

Comment သည် `#` ဖြင့် စတင်ပြီး line အဆုံးအထိ runtime က မလုပ်ဆောင်ပါ။

## 3. Values နှင့် Variables

Zap တွင် အခြေခံ value အမျိုးအစားများမှာ text၊ number၊ bool၊ list၊ map နှင့် none ဖြစ်သည်။ Variable ကို assignment ဖြင့် ဖန်တီးနိုင်ပြီး နောက်ပိုင်းတွင် value ပြန်သတ်မှတ်နိုင်သည်။

```zp
let language = "Zap"
let version = 4
let stable = true
let empty = none

say language
say version
say stable
say empty

version = version + 1
say version
```

`let` ကို variable ဖန်တီးရန် အသုံးပြုနိုင်ပြီး ရိုးရိုး assignment ကိုလည်း runtime က လက်ခံသည်။ Variable name များတွင် အက္ခရာ၊ number နှင့် underscore ပါဝင်နိုင်သော်လည်း number ဖြင့် မစသင့်ပါ။

## 4. Text နှင့် Escape Characters

Text literal များကို double quote (`"`) ဖြင့် ရေးသည်။ `+` ဖြင့် text နှစ်ခုကို ပေါင်းနိုင်သည်။

```zp
let first = "Hello"
let second = "Zap"
say first + ", " + second + "!"

say "line one\nline two"
say "column\tvalue"
```

အသုံးများသော text functions များမှာ—

```zp
let message = "  Zap Language  "
say upper(message)
say lower(message)
say trim(message)
say len(message)
say split(trim(message), " ")
say join(["web", "ai", "iot"], " / ")
```

## 5. Numbers နှင့် Operators

Number များသည် integer value ဖြစ်ပြီး arithmetic operator များကို အသုံးပြုနိုင်သည်။

| Operator | အဓိပ္ပာယ် | ဥပမာ |
|---|---|---|
| `+` | ပေါင်းခြင်း သို့မဟုတ် text ပေါင်းခြင်း | `a + b` |
| `-` | နှုတ်ခြင်း | `a - b` |
| `*` | မြှောက်ခြင်း | `a * b` |
| `/` | စားခြင်း | `a / b` |
| `%` | အကြွင်း | `a % b` |
| `-` | unary negative | `-amount` |

```zp
let total = 10 + 5 * 2
let remainder = 17 % 4
let negative = -total

say total
say remainder
say negative
say abs(negative)
say min(10, 25)
say max(10, 25)
```

Operator precedence သည် multiplication၊ division နှင့် modulus ကို addition/subtraction ထက် ဦးစားပေးသည်။ မရှင်းလင်းပါက parentheses သုံးပါ။

```zp
let result = (10 + 5) * 2
```

## 6. Boolean Logic နှင့် Comparison

Boolean value များသည် `true` နှင့် `false` ဖြစ်သည်။ Comparison operator များသည် bool value ပြန်ပေးသည်။

```zp
let age = 21
let adult = age >= 18
let exact = age == 21
let different = age != 10

say adult
say exact
say different
```

Logical operator များမှာ `and`၊ `or` နှင့် unary `not` ဖြစ်သည်။

```zp
let has_account = true
let verified = false

if has_account and not verified:
    say "verification required"

if has_account or verified:
    say "account exists"
```

## 7. Conditional Statements

`if`၊ `else if` နှင့် `else` ကို colon နှင့် indentation ဖြင့် ရေးသည်။ လက်ရှိ syntax တွင် chained branch ကို nested `if` ဖြင့် ရေးနိုင်သည်။

```zp
let score = 78

if score >= 80:
    say "A"
else:
    if score >= 60:
        say "B"
    else:
        say "Needs improvement"
```

Number သုည၊ empty text၊ empty list/map နှင့် `none` သည် false-like ဖြစ်ပြီး အခြား value များသည် true-like ဖြစ်သည်။

```zp
if [1, 2, 3]:
    say "list is not empty"

if not "":
    say "empty text is false-like"
```

## 8. Lists နှင့် Indexing

List ကို square brackets ဖြင့် ဖန်တီးပြီး index သည် zero-based ဖြစ်သည်။

```zp
let tools = ["compiler", "formatter", "tester"]
say tools[0]
say tools[2]
say len(tools)

for tool in tools:
    say tool
```

List ထဲတွင် မတူညီသော value types များလည်း ထည့်နိုင်သည်။

```zp
let record = ["Zap", 4, true, none]
say record[0]
say type(record[1])
```

`range(end)` သည် `0` မှ `end - 1` အထိ list ပြန်ပေးပြီး `range(start, end)` သည် start မှ end မတိုင်မီအထိ list ပြန်ပေးသည်။

```zp
for number in range(5):
    say number

for number in range(2, 6):
    say number
```

## 9. Maps နှင့် Keys

Map သည် text key နှင့် value များကို သိမ်းဆည်းသည်။ Text key သို့မဟုတ် name key အသုံးပြုနိုင်ပြီး bracket indexing ဖြင့် value ကို ရယူနိုင်သည်။

```zp
let user = {"name": "Zap Developer", "role": "builder", "active": true}

say user["name"]
say user["role"]
say keys(user)
say contains(user, "name")
```

Map key order ကို program logic အတွက် မမှီခိုသင့်ပါ။ Display သို့မဟုတ် deterministic output လိုအပ်ပါက key များကို ကိုယ်တိုင်စီစဉ်ပြီး အသုံးပြုပါ။

## 10. Loops

`for` loop သည် list သို့မဟုတ် range value များကို တစ်ခုချင်းစီ iterate လုပ်သည်။ `while` loop သည် condition true ဖြစ်နေသရွေ့ run သည်။

```zp
for item in ["web", "mobile", "iot"]:
    say item

let count = 0
while count < 3:
    say count
    count = count + 1
```

Loop ကို `break` ဖြင့် ရပ်နိုင်ပြီး လက်ရှိ iteration ကို `continue` ဖြင့် ကျော်နိုင်သည်။

```zp
for number in range(10):
    if number == 2:
        continue
    if number == 6:
        break
    say number
```

## 11. Functions နှင့် Return Values

Function ကို `fn` ဖြင့် ကြေညာပြီး parameters များကို parentheses အတွင်း ရေးသည်။ Function body သည် indentation ဖြင့် သတ်မှတ်သည်။

```zp
fn greet(name):
    return "Hello, " + name

let message = greet("Developer")
say message
```

`return` မရေးထားသော function သည် `none` value ပြန်ပေးသည်။ Function တွင် required parameter များ၏ အရေအတွက် မကိုက်ညီပါက runtime error ပြမည်။

```zp
fn add(a, b):
    return a + b

say add(10, 20)
```

### Default parameters

Parameter တစ်ခုကို မပေးထားသည့်အခါ အသုံးပြုမည့် default value ကို `=` ဖြင့် သတ်မှတ်နိုင်သည်။ လက်ရှိ Zap တွင် argument များသည် positional binding ဖြစ်ပြီး မပေးထားသော parameter များသာ default value ကို အသုံးပြုသည်။

```zp
fn greet(name: text = "World", punctuation: text = "!"):
    return "Hello, " + name + punctuation

say greet()
say greet("Zap", ".")
```

Required parameter နှင့် default parameter ကို function တစ်ခုတည်းတွင် ရောစပ်နိုင်သော်လည်း required parameter များကို မဖြစ်မနေ ပေးရမည်။

```zp
fn create_user(username: text, role: text = "member", active: bool = true):
    return {
        "username": username,
        "role": role,
        "active": active
    }

say create_user("may")
say create_user("may", "admin", false)
```

Default value သည် parameter annotation နှင့် ကိုက်ညီရမည်။ ဥပမာ `number` parameter အတွက် text default ပေးထားခြင်းသည် မမှန်ကန်ပါ။ Named arguments များကို လက်ရှိ version တွင် function၊ method နှင့် closure များအတွက် ထောက်ပံ့ထားပါသည်။ အသေးစိတ် syntax၊ method/constructor examples နှင့် validation rules များအတွက် [`DEFAULT_PARAMETERS_MM.md`](DEFAULT_PARAMETERS_MM.md) ကိုဖတ်ပါ။ `option<T>` နှင့် `result<T>` အတွက် guard-based type narrowing ကို [`TYPE_NARROWING_MM.md`](TYPE_NARROWING_MM.md) တွင် လေ့လာနိုင်ပါသည်။

## 12. Closures နှင့် Scope

Function အတွင်းရှိ variable များသည် local scope ရှိသည်။ Nested function သည် outer function မှ variable ကို capture လုပ်နိုင်သည်။

```zp
fn make_adder(base):
    fn add(value):
        return base + value
    return add(10)

say make_adder(5)
```

Function အတွင်း variable name တူပါက local value က outer value ထက် ဦးစားပေးသည်။ Scope ကို ရှင်းလင်းစွာထားရန် function များကို တိုတောင်းပြီး တာဝန်ခွဲရေးသားပါ။

## 13. Runtime Built-ins Reference

| Function | အသုံးပြုပုံ | ရလဒ် |
|---|---|---|
| `say(value)` | `say "Hello"` | Console သို့ value ထုတ်သည် |
| `len(value)` | `len(items)` | text/list အရှည် |
| `range(end)` | `range(5)` | number list |
| `range(start, end)` | `range(2, 5)` | bounded number list |
| `str(value)` | `str(42)` | text ပြောင်းသည် |
| `type(value)` | `type(data)` | value type name |
| `keys(map)` | `keys(user)` | map key list |
| `contains(value, item)` | `contains(items, "web")` | bool |
| `join(list, separator)` | `join(parts, "/")` | text |
| `abs(number)` | `abs(-4)` | absolute number |
| `min(a, b)` | `min(3, 8)` | smaller number |
| `max(a, b)` | `max(3, 8)` | larger number |
| `upper(text)` | `upper(name)` | uppercase text |
| `lower(text)` | `lower(name)` | lowercase text |
| `trim(text)` | `trim(input)` | surrounding whitespace ဖယ်ထားသော text |
| `split(text, separator)` | `split(path, "/")` | text list |
| `assert(condition, message)` | `assert(ok, "invalid")` | မမှန်လျှင် runtime error |
| `json(value)` | `json(user)` | JSON text |
| `from_json(text)` | `from_json(raw)` | Zap value |
| `read_text(path)` | `read_text("data.txt")` | file text |
| `write_text(path, text)` | `write_text("out.txt", data)` | file ရေးပြီး `none` ပြန်သည် |

## 14. JSON Data

Map နှင့် list များကို JSON text အဖြစ် encode/decode လုပ်နိုင်သည်။

```zp
let payload = {
    "name": "Zap",
    "features": ["web", "ai", "iot"]
}

let raw = json(payload)
say raw

let restored = from_json(raw)
say restored["features"][1]
```

Invalid JSON သို့မဟုတ် မကိုက်ညီသော input ကို decode လုပ်ပါက runtime error ပြမည်။ External data ကို အသုံးမပြုမီ `assert` နှင့် `type` ဖြင့် validate လုပ်ရန် အကြံပြုသည်။

## 15. File Input/Output

`read_text` နှင့် `write_text` သည် plain text file များအတွက် အသုံးပြုသည်။ File path သည် program ကို run သည့် current working directory အပေါ် အခြေခံသည်။

```zp
let path = "notes.txt"
write_text(path, "Zap makes small tools easy to write.")
let notes = read_text(path)
say notes
```

File permission၊ path မရှိခြင်း သို့မဟုတ် read/write failure ဖြစ်ပါက runtime error ကို စစ်ဆေးပြီး ပြမည်။ User-provided path များကို production application တွင် validate လုပ်ပါ။

## 16. Modules နှင့် Project Layout

Project structure ကို အောက်ပါအတိုင်းထားရန် အကြံပြုသည်။

```text
hello-zap/
├── zap.toml
├── main.zp
├── modules/
│   └── greeting.zp
├── lib/
│   └── text_helpers.zp
└── tests/
    └── smoke.zp
```

`use` ဖြင့် module ကို import လုပ်နိုင်သည်။ Runtime သည် main file directory၊ `modules/` နှင့် `lib/` directories များတွင် module file ကို ရှာဖွေသည်။

```zp
use "greeting.zp"
```

Manifest သည် project name၊ version နှင့် entry file ကို သတ်မှတ်သည်။

```toml
[package]
name = "hello-zap"
version = "0.1.0"
main = "main.zp"
```

Manifest ကို စစ်ဆေးရန်—

```bash
zap check
zap check path/to/project
```

## 17. Formatter နှင့် Code Style

Source formatting ကို runtime မပြောင်းလဲစေဘဲ canonical whitespace အဖြစ် ပြင်ရန်—

```bash
zap fmt main.zp
```

Project များတွင် indentation တစ်ဆင့်ကို spaces လေးလုံး၊ function name များကို `snake_case`၊ constant-like value များကို ရှင်းလင်းသော name နှင့် comment များကို အတိုချုံးအသုံးပြုရန် အကြံပြုသည်။

## 18. Error Handling နှင့် Debugging

လက်ရှိ Zap runtime သည် recoverable failure များအတွက် typed `Result` value (`ok(...)` နှင့် `err(...)`) ကို အသုံးပြုနိုင်ပြီး၊ မကိုင်တွယ်ရသေးသော runtime failure များကို deterministic `Error` diagnostic အဖြစ် ပြသသည်။ အောက်ပါအမှားများကို အထူးစစ်ဆေးပါ။

| Error အမျိုးအစား | ဖြစ်နိုင်သောအကြောင်းရင်း |
|---|---|
| `undefined variable` | variable ကို မဖန်တီးမီ အသုံးပြုခြင်း |
| `undefined function` | function name မှားခြင်း သို့မဟုတ် module မတွေ့ခြင်း |
| `index out of range` | list index မမှန်ခြင်း |
| `key not found` | map key မရှိခြင်း |
| `division by zero` | သုညဖြင့် စားခြင်း |
| `invalid operation` | မကိုက်ညီသော value types များကို operator ဖြင့် ပေါင်းခြင်း |
| `assertion failed` | သတ်မှတ်ထားသော invariant မမှန်ခြင်း |
| `Error` | မကိုင်တွယ်ရသေးသော typed `Result` failure သို့မဟုတ် runtime failure |

`zap check --json` ကိုအသုံးပြုပါက diagnostic တွင် `kind`, `message`, `file`, `line`, နှင့် `column` fields များကို deterministic ပုံစံဖြင့် ရရှိနိုင်သည်။

Debug လုပ်ရာတွင် intermediate value များကို `say` ဖြင့်ထုတ်ပြီး `type`၊ `len` နှင့် `assert` ဖြင့် စစ်ဆေးပါ။

```zp
let result = from_json(raw)
say type(result)
assert(type(result) == "map", "payload must be a map")
say keys(result)
```

## 19. Complete Example: Task Summary

အောက်ပါ program သည် list၊ map၊ function၊ loop၊ filtering logic နှင့် JSON output ကို တစ်နေရာတည်းတွင် အသုံးပြုထားသည်။

```zp
fn completed_count(tasks):
    let total = 0
    for task in tasks:
        if task["done"]:
            total = total + 1
    return total

let tasks = [{"title": "learn syntax", "done": true}, {"title": "write a program", "done": true}, {"title": "build a project", "done": false}]

let completed = completed_count(tasks)
let summary = {"total": len(tasks), "completed": completed, "remaining": len(tasks) - completed}

assert(summary["total"] > 0, "task list must not be empty")
say json(summary)
```

## 20. Recommended Development Workflow

Zap project တစ်ခုကို စတင်သည့်အခါ `zap init` ဖြင့် scaffold ဖန်တီးပါ။ ၎င်းသည် `zap.toml`၊ `main.zp` နှင့် အခြေခံ `tests/smoke_test.zp` ကို ဖန်တီးပေးသည်။ ထို့နောက် `main.zp` ကို တဖြည်းဖြည်းရေးသားပြီး `zap check` ဖြင့် manifest ကိုစစ်ဆေးကာ `zap fmt` ဖြင့် source style ကိုညှိပါ။ အဓိက logic များကို function များအဖြစ်ခွဲပြီး input data များကို `type` နှင့် `assert` ဖြင့် စောစီးစွာစစ်ဆေးပါ။

```bash
zap init my-app
cd my-app
zap check
zap fmt main.zp
zap test
zap main.zp
```

Release မတင်မီ native test suite ကို run လုပ်ပါ။

```bash
cargo test --manifest-path native/Cargo.toml
```

## 21. လက်ရှိအခြေအနေ နှင့် နောက်ထပ်တိုးချဲ့မည့်အရာများ

လက်ရှိ native foundation တွင် variables၊ expressions၊ control flow၊ functions၊ closures၊ collections၊ JSON၊ file I/O၊ modules၊ formatter၊ project validation၊ project scaffolding နှင့် recursive project test runner ပါဝင်သည်။ Function/method/closure named arguments၊ OOP visibility၊ constructor delegation နှင့် `option<T>`/`result<T>` guard-based type narrowing များလည်း ပါဝင်သည်။ `zap test` သည် `tests/` အောက်ရှိ subdirectories များအပါအဝင် `*_test.zp` files များကို run လုပ်သည်။ နောက်ထပ် language evolution အတွက် static type checking၊ richer diagnostics၊ first-class module exports၊ package registry၊ async I/O နှင့် platform-specific libraries များကို အဆင့်လိုက် တိုးချဲ့သွားမည်။

လက်ရှိ runtime တွင် တကယ်အလုပ်လုပ်ပြီးသား syntax ကိုသာ production source တွင် အသုံးပြုပါ။ မပြီးသေးသော Web၊ Mobile၊ AI နှင့် IoT libraries များကို roadmap အဖြစ် သတ်မှတ်ထားပြီး core language stability ရရှိပြီးနောက် သီးခြား package များအဖြစ် တည်ဆောက်မည်။

## 22. Documentation Map

| ဖိုင် | ရည်ရွယ်ချက် |
|---|---|
| `../README.md` | Project overview နှင့် quick start |
| `LANGUAGE_GUIDE.md` | Beginner-to-intermediate language usage guide |
| `SYNTAX_GUIDE.md` | Syntax reference နှင့် runnable examples |
| `CORE_SPEC.md` | Core language behavior နှင့် implementation status |
| `PACKAGE.md` | `zap.toml` နှင့် project layout specification |
| `NATIVE.md` | Native runtime architecture နှင့် build information |
| `DESIGN.md` | Zap design principles နှင့် roadmap |
| `USAGE.md` | Installation နှင့် CLI usage |
| `ECOSYSTEM.md` | Web၊ Mobile၊ AI နှင့် IoT ecosystem plan |
| `../examples/` | Runnable Zap examples |
