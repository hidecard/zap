# Learn Zap — Burmese Beginner Course

ဒီစာအုပ်သည် Zap programming language ကို ပထမဆုံးစတင်လေ့လာမည့်သူများအတွက် ရေးထားသော lesson-based guide ဖြစ်သည်။ Lesson တစ်ခုစီတွင် အယူအဆ၊ code နမူနာ၊ run လုပ်ပုံ၊ expected output နှင့် လေ့ကျင့်ခန်း ပါဝင်သည်။ Code များကို ကိုယ်တိုင် `.zp` file ထဲရေးပြီး `zap file.zp` ဖြင့် စမ်းသပ်ပါ။

## သင်ကြားရေးအစီအစဉ်

| အပိုင်း | Lesson | ရည်ရွယ်ချက် |
|---|---|---|
| အခြေခံ | 1–4 | Program run၊ output၊ variables၊ operators |
| Control flow | 5–8 | Condition၊ list၊ map၊ loops |
| Reusable code | 9–10 | Functions၊ return၊ closures |
| Practical programming | 11–13 | File၊ path၊ JSON၊ modules၊ tests |
| Project | 14 | Mini task tracker တည်ဆောက်ခြင်း |
| OOP | 15 | Class၊ object၊ constructor၊ method နှင့် inheritance |
| Standard library | 16 | Collection helpers နှင့် line-based file I/O |

## Lesson 1 — Installation နှင့် Hello World

### ရည်ရွယ်ချက်

Zap ကို install လုပ်ပြီး ပထမဆုံး source file တစ်ခု run တတ်ရန် ဖြစ်သည်။ Zap source file များသည် `.zp` extension အသုံးပြုသည်။

### ပထမဆုံး file

`hello.zp` ဖန်တီးပြီး အောက်ပါ code ရေးပါ။

```zap
say "Hello from Zap"
say "မင်္ဂလာပါ Zap"
```

Run လုပ်ပါ။

```bash
zap hello.zp
```

Expected output—

```text
Hello from Zap
မင်္ဂလာပါ Zap
```

### မှတ်သားရန်

`say` သည် value တစ်ခုကို terminal တွင် ထုတ်ပြသသည့် Zap command ဖြစ်သည်။ Program တစ်ခုသည် အပေါ်မှအောက်သို့ အစဉ်လိုက် run သည်။

### လေ့ကျင့်ခန်း

သင့်အမည်၊ သင်လေ့လာရသည့်အကြောင်းအရာနှင့် ရည်မှန်းချက်ကို `say` သုံးပြီး သုံးကြောင်းရေးပါ။

## Lesson 2 — Comments နှင့် Program Structure

### Comments

`#` ဖြင့် စသည့်စာကြောင်းကို runtime က မလုပ်ဆောင်ပါ။

```zap
# ဒီစာကြောင်းသည် မှတ်ချက်ဖြစ်သည်
say "ဒီစာကြောင်းပဲ run မည်"
```

### Code ကို စနစ်တကျရေးခြင်း

```zap
# User information
say "Name: Zap Learner"
say "Level: Beginner"

# Course information
say "Course: Learn Zap"
```

Blank lines နှင့် comments များသည် code ကို ဖတ်ရလွယ်စေသည်။

### လေ့ကျင့်ခန်း

သင့် program ထဲတွင် comment သုံးကြောင်းထည့်ပြီး output နှစ်ကြောင်းပြပါ။

## Lesson 3 — Variables နှင့် Value Types

### Variable ဆိုတာဘာလဲ

Variable သည် value တစ်ခုကို နာမည်ပေးပြီး သိမ်းထားသည့်နေရာဖြစ်သည်။ Zap တွင် `let` ဖြင့် variable အသစ်ဖန်တီးနိုင်သည်။

```zap
let name = "Aye Aye"
let age = 18
let active = true
let nothing = none

say name
say age
say active
say nothing
```

Zap ၏ အခြေခံ value types များမှာ—

| Type | နမူနာ |
|---|---|
| `text` | `"Hello"` |
| `number` | `42` |
| `bool` | `true`၊ `false` |
| `list` | `[1, 2, 3]` |
| `map` | `{"name": "Zap"}` |
| `none` | `none` |

### Reassignment

```zap
let score = 10
score = score + 5
say score
```

Expected output—

```text
15
```

### Type annotation

Type ကို optional အနေဖြင့် ရေးနိုင်သည်။

```zap
let title: text = "Zap"
let count: number = 3
let published: bool = false
```

Type မကိုက်ပါက—

```zap
let count: number = "three"
```

runtime error ပြန်ပေးမည်။

### လေ့ကျင့်ခန်း

`student_name`၊ `lesson_count` နှင့် `completed` variables သုံးခုဖန်တီးပြီး type annotation ထည့်ပါ။

## Lesson 4 — Operators နှင့် Calculations

### Arithmetic

```zap
let a = 20
let b = 6

say a + b
say a - b
say a * b
say a / b
say a % b
```

`%` သည် remainder ကို ပြန်ပေးသည်။

### Comparison

```zap
let score = 75

say score == 75
say score != 50
say score > 60
say score >= 75
say score < 100
say score <= 75
```

Comparison ၏ result သည် `true` သို့မဟုတ် `false` ဖြစ်သည်။

### Boolean logic

```zap
let logged_in = true
let verified = false

say logged_in and verified
say logged_in or verified
say not verified
```

### လေ့ကျင့်ခန်း

အမှတ်နှစ်ခု၏ စုစုပေါင်း၊ ပျမ်းမျှနှင့် အောင်/မအောင် boolean result ကို တွက်ချက်ပါ။

## Lesson 5 — If နှင့် Else

Condition မှန်လျှင် if block ကို run ပြီး မမှန်လျှင် else block ကို run သည်။

```zap
let age = 20

if age >= 18:
    say "Adult"
else:
    say "Minor"
```

Indentation သည် အရေးကြီးသည်။ `if` အောက်ရှိ code ကို indentation တစ်ဆင့်ထားရမည်။

### Nested condition

```zap
let score = 82

if score >= 80:
    say "A"
else:
    if score >= 60:
        say "B"
    else:
        say "C"
```

### လေ့ကျင့်ခန်း

Temperature value တစ်ခုထားပြီး `hot`၊ `warm`၊ `cold` အဖြစ် condition သုံးမျိုးခွဲပြပါ။

## Lesson 6 — Lists နှင့် Indexing

List သည် value များကို အစဉ်လိုက် သိမ်းသည်။ Index သည် `0` မှ စသည်။

```zap
let fruits = ["apple", "banana", "orange"]

say fruits[0]
say fruits[2]
say len(fruits)
```

Expected output—

```text
apple
orange
3
```

List ထဲတွင် number၊ text၊ bool နှင့် map များ ပေါင်းစပ်ထားနိုင်သည်။

```zap
let mixed = ["Zap", 6, true, none]
say type(mixed[1])
```

### `range`

```zap
let numbers = range(5)
say numbers

let selected = range(2, 5)
say selected
```

`range(5)` သည် `0` မှ `4` အထိ ထုတ်ပေးသည်။

### လေ့ကျင့်ခန်း

သင်ကြိုက်နှစ်သက်သည့် language သုံးခုကို list ထဲထည့်ပြီး ဒုတိယတစ်ခုကို output ပြပါ။

## Lesson 7 — Maps နှင့် JSON

Map သည် key နှင့် value အတွဲများကို သိမ်းသည်။

```zap
let user = {
    "name": "Mya",
    "age": 22,
    "active": true
}

say user["name"]
say user["age"]
say keys(user)
say contains(user, "name")
```

### Nested data

```zap
let profile = {
    "name": "Zap User",
    "skills": ["web", "ai"],
    "address": {"city": "Yangon"}
}

say profile["skills"][0]
say profile["address"]["city"]
```

### JSON encode/decode

```zap
let data = {"ok": true, "count": 3}
let text = json(data)
say text

let decoded = from_json("{\"name\": \"Zap\"}")
say decoded["name"]
```

### လေ့ကျင့်ခန်း

Product တစ်ခု၏ `name`၊ `price`၊ `in_stock` နှင့် `tags` ပါသော map ဖန်တီးပြီး JSON အဖြစ် output ပြပါ။

## Lesson 8 — For နှင့် While Loops

### For loop

```zap
let items = ["read", "code", "test"]

for item in items:
    say item
```

`for` သည် list ထဲရှိ item တစ်ခုစီကို အလိုအလျောက် လှည့်ပတ်သည်။

### Range ဖြင့် loop

```zap
for number in range(5):
    say number
```

### While loop

```zap
let count = 1

while count <= 3:
    say count
    count = count + 1
```

Loop ထဲတွင် variable ကို update မလုပ်ပါက `while` သည် မရပ်နိုင်ပါ။

### Break နှင့် Continue

```zap
for number in range(5):
    if number == 1:
        continue
    if number == 4:
        break
    say number
```

`continue` သည် လက်ရှိ iteration ကို ကျော်ပြီး `break` သည် loop တစ်ခုလုံးကို ရပ်သည်။

### လေ့ကျင့်ခန်း

`range(10)` ထဲမှ 5 မရောက်မီ number များကိုသာ output ပြပါ။ 2 ကို skip လုပ်ပါ။

## Lesson 9 — Functions နှင့် Return

Function သည် code ကို ပြန်လည်အသုံးပြုနိုင်ရန် နာမည်ပေးထားသော block ဖြစ်သည်။

```zap
fn greet(name):
    return "Hello, " + name

say greet("Zap")
say greet("Learner")
```

### Parameters နှင့် calculation

```zap
fn multiply(a, b):
    return a * b

let result = multiply(6, 7)
say result
```

### Function ထဲတွင် assert

```zap
fn divide(a, b):
    assert(b != 0, "cannot divide by zero")
    return a / b

say divide(10, 2)
```

### လေ့ကျင့်ခန်း

Number တစ်ခုသည် even ဖြစ်မဖြစ် စစ်ပြီး boolean ပြန်ပေးသည့် `is_even(number)` function ရေးပါ။

## Lesson 10 — Scope နှင့် Closures

Function ထဲက variable သည် local scope ဖြစ်သည်။ အပြင် function ၏ variable ကို nested function က အသုံးပြုနိုင်သည်။

```zap
fn make_message(prefix):
    fn message(name):
        return prefix + ", " + name
    return message("Developer")

say make_message("Welcome")
```

`prefix` သည် nested function ထဲတွင် တိုက်ရိုက်မဖန်တီးဘဲ အသုံးပြုနိုင်သောကြောင့် closure ဖြစ်သည်။

### လေ့ကျင့်ခန်း

`make_multiplier(factor)` function ရေးပြီး factor နှင့် number ကို မြှောက်ပေးသည့် nested function တစ်ခု ဖန်တီးပါ။

## Lesson 11 — File I/O၊ Path နှင့် Time

### File ရေးခြင်းနှင့်ဖတ်ခြင်း

```zap
let path = "notes.txt"
write_text(path, "Zap is simple")
let content = read_text(path)
say content
```

### Path helper

```zap
let path = path_join("data", "users", "list.json")
say path
say basename(path)
say dirname(path)
say exists(path)
```

`path_join` သည် operating system ၏ path separator ကို အသုံးပြုသောကြောင့် platform မတူသည့်နေရာများတွင် သင့်တော်သည်။

### Time နှင့် delay

```zap
let timestamp: number = now()
say timestamp
sleep(100)
say "100 milliseconds later"
```

### Environment variable

```zap
if has_env("PATH"):
    say "PATH is available"
    say env("PATH")
```

### လေ့ကျင့်ခန်း

`data` directory path ကို `path_join` ဖြင့် တည်ဆောက်ပြီး file ရှိပါက ဖတ်၊ မရှိပါက create လုပ်သည့် program ရေးပါ။

## Lesson 12 — Modules နှင့် Project Structure

Project root တွင် `modules/greetings.zp` ဖိုင်ဖန်တီးပါ။

```zap
export fn greet(name):
    return "Hello, " + name
```

`main.zp` တွင် explicit import လုပ်ပါ။ `export` မပါသော function/variable များသည် module အပြင်မှ မမြင်ရပါ။

```zap
import "greetings"
say greet("Zap")
```

Project structure—

```text
my-project/
├── zap.toml
├── main.zp
├── modules/
│   └── greetings.zp
└── tests/
    └── greetings_test.zp
```

`zap.toml`—

```toml
[package]
name = "my-project"
version = "0.6.0"
main = "main.zp"
```

Run—

```bash
zap check .
zap build .
zap main.zp
```

### Module rules

`import` ဖြင့် module တစ်ခုကို အကြိမ်များစွာ ခေါ်လျှင် runtime သည် canonical path အပေါ်အခြေခံသော cache ကို အသုံးပြုသဖြင့် module top-level code ကို တစ်ကြိမ်သာ run သည်။ Module နှစ်ခု အပြန်အလှန် import လုပ်ပါက circular import error ပြမည်။ Absolute filesystem path import များကို လုံခြုံရေးအရ ခွင့်မပြုပါ။ အဟောင်း code များအတွက် `use "greetings"` သည် legacy import အဖြစ် ဆက်လက်အလုပ်လုပ်နိုင်သော်လည်း library အသစ်များတွင် explicit `import`/`export` ကို အသုံးပြုသင့်သည်။

### လေ့ကျင့်ခန်း

`math.zp` module ထဲတွင် `square(number)` function ရေးပြီး main file မှ import လုပ်ပါ။

## Lesson 13 — Testing နှင့် Formatter

Test file name သည် `_test.zp` ဖြင့်ဆုံးရမည်။

```zap
fn add(a, b):
    return a + b

assert(add(2, 3) == 5, "addition failed")
assert(type(add(2, 3)) == "number", "type failed")
say "all checks passed"
```

Run—

```bash
zap test
```

Test များများရှိသော project များတွင် filter ဖြင့် သက်ဆိုင်ရာ test file များကိုသာ ရွေးနိုင်သည်။ ပထမဆုံး failure ဖြစ်သည်နှင့် ရပ်ရန် `--fail-fast`၊ CI tool များအတွက် machine-readable result ထုတ်ရန် `--json` ကို အသုံးပြုနိုင်သည်။

```bash
zap test tests --filter arithmetic
zap test tests --fail-fast
zap test tests --json
```

မသိသော test option သည် usage error ဖြစ်ပြီး exit code `2` ပြန်ပေးသည်။ Test failure ဖြစ်ပါက exit code `1` ပြန်ပေးသည်။

Code ကို format လုပ်ရန်—

```bash
zap fmt main.zp
```

Project အခြေအနေစစ်ရန်—

```bash
zap check .
zap build .
```

### Test ရေးရာတွင် အကြံပြုချက်

Test တစ်ခုတွင် behavior တစ်ခုကို အဓိကထားပါ။ `assert` message ကို အဓိပ္ပာယ်ရှိအောင် ရေးပါ။ Error case ကိုလည်း စမ်းသပ်ပါ။

### လေ့ကျင့်ခန်း

`is_even` function အတွက် even၊ odd နှင့် zero cases သုံးခုကို test ရေးပါ။

## Lesson 14 — Complete Mini Project: Task Tracker

အောက်ပါ project သည် list၊ map၊ loop၊ function၊ JSON နှင့် assert တို့ကို ပေါင်းစပ်ထားသည်။ `task_tracker.zp` အဖြစ် သိမ်းပါ။

```zap
fn completed_count(tasks):
    let total = 0
    for task in tasks:
        if task["done"]:
            total = total + 1
    return total

let tasks = [
    {"title": "Learn variables", "done": true},
    {"title": "Practise loops", "done": false},
    {"title": "Build a project", "done": false}
]

let completed = completed_count(tasks)
let summary = {
    "total": len(tasks),
    "completed": completed,
    "remaining": len(tasks) - completed
}

assert(summary["total"] == 3, "task count failed")
say json(summary)
```

Expected result သည် JSON object တစ်ခုဖြစ်ပြီး total၊ completed နှင့် remaining ပါဝင်မည်။ Map order သည် runtime implementation ပေါ်မူတည်၍ ပြောင်းနိုင်သောကြောင့် output string တစ်ခုလုံးနှင့် မနှိုင်းဘဲ key value များကို စစ်ပါ။

### Project ကို တိုးချဲ့ရန်

Task အသစ်ထည့်ရန် function ရေးပါ။ File ထဲသို့ task data သိမ်းပါ။ Due date field ထည့်ပါ။ Completed task များကိုသာ filter လုပ်ပါ။ ထို့နောက် `tests/` folder ထဲတွင် test file ဖန်တီးပါ။

## ပြဿနာဖြေရှင်းခြင်း

| ပြဿနာ | စစ်ဆေးရန် |
|---|---|
| `zap` command မတွေ့ | PATH နှင့် installer အခြေအနေ စစ်ပါ |
| `main.zp` မတွေ့ | လက်ရှိ directory နှင့် file name စစ်ပါ |
| `expected =` | `let name = value` ပုံစံကို စစ်ပါ |
| `undefined variable` | Variable ကို အသုံးမပြုမီ ကြေညာထားပါသလား စစ်ပါ |
| `undefined function` | Function name နှင့် module import စစ်ပါ |
| `invalid operation` | Text နှင့် number ကို မမှန်ကန်စွာ မပေါင်းထားပါသလား စစ်ပါ |
| `index out of range` | List index သည် `0` မှ စကြောင်း မှတ်ပါ |
| Type mismatch | Annotation နှင့် assigned value type ကို တူအောင်ထားပါ |
| File error | Path၊ permission နှင့် parent directory ရှိမရှိ စစ်ပါ |

Version ကို စစ်ရန်—

```bash
zap --version
```

Help ကို စစ်ရန်—

```bash
zap --help
```

## နောက်တစ်ဆင့် လေ့လာရန်

ဒီ course ပြီးပါက [`docs/SYNTAX_GUIDE.md`](SYNTAX_GUIDE.md) တွင် syntax အပြည့်အစုံကို ပြန်လည်ကြည့်ပါ။ [`docs/LANGUAGE_GUIDE.md`](LANGUAGE_GUIDE.md) တွင် workflow နှင့် complete examples များကို လေ့လာပါ။ v0.6.0 ၏ future features များကို [`docs/ROADMAP_0.6.0.md`](ROADMAP_0.6.0.md) တွင် ဖတ်ပါ။

လက်ရှိ roadmap တွင် structured error model၊ HTTP client၊ async/await၊ tasks၊ channels၊ linting၊ JSON diagnostics နှင့် package management တို့ ပါဝင်သော်လည်း ၎င်းတို့အားလုံးသည် လက်ရှိ stable runtime feature မဟုတ်သေးပါ။

## သင်တန်းပြီးဆုံးမှု စစ်ဆေးရန်

အောက်ပါအရာများကို ကိုယ်တိုင်လုပ်နိုင်လျှင် beginner foundation ပြည့်စုံပြီဟု သတ်မှတ်နိုင်သည်။

1. `.zp` file ဖန်တီးပြီး `zap` ဖြင့် run လုပ်နိုင်ခြင်း။
2. Variables၊ lists၊ maps နှင့် JSON data ကို အသုံးပြုနိုင်ခြင်း။
3. `if`၊ `for` နှင့် `while` ဖြင့် logic ရေးနိုင်ခြင်း။
4. Function နှင့် closure ဖြင့် code ပြန်လည်အသုံးပြုနိုင်ခြင်း။
5. File၊ path၊ environment နှင့် time APIs များ အသုံးပြုနိုင်ခြင်း။
6. Module နှင့် `zap.toml` project တည်ဆောက်နိုင်ခြင်း။
7. `zap fmt`၊ `zap check`၊ `zap build` နှင့် `zap test` workflow အသုံးပြုနိုင်ခြင်း။
8. Mini project တစ်ခုကို ကိုယ်တိုင်ရေး၊ စမ်းသပ်ပြီး ပြင်ဆင်နိုင်ခြင်း။


## Lesson 15: OOP — Class၊ Object၊ Method နှင့် Inheritance

OOP ဆိုသည်မှာ data နှင့် ထို data ကို ကိုင်တွယ်သည့် behavior များကို object တစ်ခုအတွင်း စုစည်းရေးသားသည့် programming ပုံစံဖြစ်သည်။ Zap တွင် class သည် object များဖန်တီးရန် template ဖြစ်ပြီး object သည် class မှ ဖန်တီးထားသော အသုံးပြုနိုင်သည့် value ဖြစ်သည်။

### 15.1 Class နှင့် Object

```zp
class User:
    fn greet(self):
        return "Hello from Zap"

let user = new("User")
say user.greet()
```

`class User:` သည် `User` class ကို ကြေညာသည်။ `new("User")` သည် object တစ်ခု ဖန်တီးပြီး `user.greet()` သည် method ကို ခေါ်သည်။ Method ၏ ပထမ parameter ဖြစ်သော `self` သည် လက်ရှိ object ကို ကိုယ်စားပြုသည်။

### 15.2 Properties နှင့် `self`

```zp
class User:
    fn show_name(self):
        return self.name

let user = new("User", {"name": "Zap Developer"})
say user.show_name()
```

Object ဖန်တီးရာတွင် map ဖြင့် initial properties ထည့်နိုင်သည်။ Property အသစ် သတ်မှတ်ခြင်း သို့မဟုတ် ပြင်ဆင်ခြင်းကို method ထဲတွင် `self.property = value` ဖြင့် ရေးသည်။

```zp
class Counter:
    fn increment(self):
        self.value = self.value + 1
        return self.value

let counter = new("Counter", {"value": 0})
say counter.increment()
say counter.value
```

Expected output—

```text
1
1
```

### 15.3 Constructor — `init`

Class ထဲတွင် `init(self, ...)` method ရှိပါက object ဖန်တီးသောအခါ Zap runtime က အလိုအလျောက်ခေါ်ပေးသည်။

```zp
class Product:
    fn init(self, name, price):
        self.name = name
        self.price = price

    fn label(self):
        return self.name + " - " + str(self.price)

let product = new("Product", "Keyboard", 50)
say product.label()
```

Constructor ၏ ရည်ရွယ်ချက်မှာ object ၏ initial state ကို တစ်နေရာတည်းတွင် သတ်မှတ်ရန်ဖြစ်သည်။ Constructor မရှိလျှင် map arguments ဖြင့် fields များကို တိုက်ရိုက်စတင်နိုင်သည်။

### 15.4 Method Parameters နှင့် Return Values

```zp
class Calculator:
    fn add(self, left, right):
        return left + right + self.offset

let calculator = new("Calculator", {"offset": 1})
say calculator.add(2, 3)
```

Expected output သည် `6` ဖြစ်သည်။ `self` သည် implicit မဟုတ်ဘဲ method parameter list ထဲတွင် ရေးထားရမည်။

### 15.5 Inheritance နှင့် Override

Parent class ၏ behavior ကို child class က `extends` ဖြင့် ရယူနိုင်သည်။

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

Child class တွင် အမည်တူ method ရှိပါက child method ကို အသုံးပြုသည်။ Parent method များကို မ override လုပ်ထားပါက child object မှ တိုက်ရိုက်ခေါ်နိုင်သည်။

### 15.6 Objects နှင့် Collections

Object များကို list နှင့် map အတွင်း ထည့်သိမ်းနိုင်သည်။

```zp
class User:
    fn name_value(self):
        return self.name

let first = new("User", {"name": "A"})
let second = new("User", {"name": "B"})
let users = [first, second]
let directory = {"primary": first}

say users[1].name_value()
say directory["primary"].name_value()
```

### 15.7 OOP Exercise များ

**Exercise 1:** `Book` class တစ်ခုရေးပါ။ `title` နှင့် `author` properties ထည့်ပြီး `description()` method ဖြင့် `title + " by " + author` ပြန်ပေးပါ။

**Exercise 2:** `BankAccount` class တစ်ခုရေးပါ။ `balance` property ထည့်ပြီး `deposit(amount)` method ဖြင့် balance တိုးပါ။

**Exercise 3:** `Animal` parent class နှင့် `Cat`၊ `Dog` child classes ရေးပါ။ Child တစ်ခုစီတွင် `speak()` ကို override လုပ်ပါ။

**Exercise 4:** `Task` class ဖြင့် mini task tracker ကို ပြန်ရေးပါ။ `complete()` method သည် `done` property ကို `true` ပြောင်းရမည်။

### 15.8 OOP အတွက် သတိပြုရန်

v0.7.0 တွင် class registry validation၊ class၊ object၊ constructor၊ methods၊ mutable properties၊ `self`၊ parent constructor initialization၊ method override နှင့် single inheritance ကို အသုံးပြုနိုင်သည်။ မရှိသော class သို့မဟုတ် parent class ကို အသုံးပြုပါက runtime/class declaration error ပြန်ပေးသည်။ Interfaces၊ abstract classes၊ private fields၊ generics၊ explicit `super` method calls နှင့် multiple inheritance များကို မထည့်သွင်းသေးပါ။ Class name များကို စာလုံးကြီးဖြင့် စတင်ရေးခြင်းသည် ဖတ်ရလွယ်ကူစေသည်။

## OOP Learning Checkpoint

OOP lesson ပြီးဆုံးပါက အောက်ပါအရာများကို ကိုယ်တိုင်လုပ်နိုင်ရမည်။

1. Class တစ်ခု ကြေညာပြီး `new()` ဖြင့် object ဖန်တီးနိုင်ခြင်း။
2. `self` ဖြင့် property ဖတ်ခြင်းနှင့် ပြင်ခြင်း။
3. `init()` constructor ရေးနိုင်ခြင်း။
4. Method parameters နှင့် return values အသုံးပြုနိုင်ခြင်း။
5. `extends` ဖြင့် inheritance နှင့် method override ရေးနိုင်ခြင်း။
6. Object များကို list/map အတွင်း ထည့်ပြီး method ခေါ်နိုင်ခြင်း။
7. OOP code အတွက် test file တစ်ခုရေးနိုင်ခြင်း။

သင်ခန်းစာ၏ syntax reference ကို [`SYNTAX_GUIDE.md`](SYNTAX_GUIDE.md) တွင် ဆက်လက်ဖတ်ရှုနိုင်သည်။

## Updated Course Completion Checklist

OOP lesson အပါအဝင် beginner foundation ပြည့်စုံရန်—

1. `.zp` file ဖန်တီးပြီး `zap` ဖြင့် run လုပ်နိုင်ခြင်း။
2. Variables၊ lists၊ maps နှင့် JSON data ကို အသုံးပြုနိုင်ခြင်း။
3. `if`၊ `for` နှင့် `while` ဖြင့် logic ရေးနိုင်ခြင်း။
4. Function နှင့် closure ဖြင့် code ပြန်လည်အသုံးပြုနိုင်ခြင်း။
5. File၊ path၊ environment နှင့် time APIs များ အသုံးပြုနိုင်ခြင်း။
6. Module နှင့် `zap.toml` project တည်ဆောက်နိုင်ခြင်း။
7. `zap fmt`၊ `zap check`၊ `zap build` နှင့် `zap test` workflow အသုံးပြုနိုင်ခြင်း။
8. Class၊ object၊ constructor၊ method၊ property state နှင့် inheritance အသုံးပြုနိုင်ခြင်း။
9. Unknown class/parent error များကို နားလည်ပြီး စစ်ဆေးနိုင်ခြင်း။
10. OOP mini project တစ်ခုကို ကိုယ်တိုင်ရေး၊ စမ်းသပ်ပြီး ပြင်ဆင်နိုင်ခြင်း။

## OOP Lesson အတွက် Test Example

```zp
class User:
    fn init(self, name):
        self.name = name

    fn greet(self):
        return "Hello, " + self.name

let user = new("User", "Tester")
assert(user.greet() == "Hello, Tester", "greeting failed")
say "OOP test passed"
```

ဤ example ကို `oop_test.zp` အဖြစ် သိမ်းပြီး `zap test` workflow ထဲတွင် အသုံးပြုနိုင်သည်။

## Current Stable Boundary

OOP feature သည် v0.7.0 native runtime တွင် class validation၊ parent constructor initialization၊ method override နှင့် mutable object state အပါအဝင် implementation အဖြစ် ပါဝင်နေပြီဖြစ်သည်။ v0.7.0 တွင် collection နှင့် line-based file helpers များလည်း ပါဝင်လာသည်။ `async/await`၊ channels၊ HTTP client၊ package registry နှင့် advanced type system များသည် roadmap အဖြစ်သာ ရှိသေးပြီး stable API ၏ အစိတ်အပိုင်းမဟုတ်သေးပါ။

သင်ခန်းစာအားလုံးပြီးနောက် [`SYNTAX_GUIDE.md`](SYNTAX_GUIDE.md)၊ [`LANGUAGE_GUIDE.md`](LANGUAGE_GUIDE.md) နှင့် [`ROADMAP_0.6.0.md`](ROADMAP_0.6.0.md) တို့ကို ဆက်လက်ဖတ်ရှုပါ။

## Lesson 16 — Collection Helpers နှင့် Line-based File I/O

v0.7.0 တွင် list နှင့် map data များကို ပိုမိုလွယ်ကူစွာ ကိုင်တွယ်ရန် helper functions များ ထပ်တိုးထားသည်။ File ကို line တစ်ကြောင်းစီ ဖတ်ခြင်းနှင့် ရေးခြင်းကိုလည်း တိုက်ရိုက်လုပ်နိုင်သည်။

### Collection helpers

```zap
let numbers = [4, 1, 8, 2]

say is_empty(numbers)
say sum(numbers)
say min(4, 1)
say max(8, 2)
say reverse(numbers)
say sort(numbers)
say join(sort(numbers), ",")
```

Expected output—

```text
false
15
1
8
[2, 8, 1, 4]
[1, 2, 4, 8]
1,2,4,8
```

`sort` သည် number list သို့မဟုတ် text list ကို sort လုပ်ပေးသည်။ `reverse` သည် list အသစ်ကို ပြန်ပေးပြီး မူလ list ကို မပြောင်းပါ။ `sum` သည် number list အတွက်သာ အသုံးပြုရမည်။

### Map မှ default value ရယူခြင်း

```zap
let user = {"name": "Zap", "role": "developer"}
say get(user, "name", "unknown")
say get(user, "email", "unknown")
```

Key မရှိပါက `get` ၏ တတိယ argument ဖြစ်သော default value ကို ပြန်ပေးသည်။ ထို့ကြောင့် optional configuration သို့မဟုတ် JSON data များကို စစ်ဆေးရာတွင် အသုံးဝင်သည်။

### Line-based file I/O

```zap
let lines = ["first", "second", "third"]
write_lines("items.txt", lines)

let loaded = read_lines("items.txt")
say len(loaded)
say join(loaded, "|")
```

Expected output—

```text
3
first|second|third
```

`write_lines` သည် list ထဲရှိ text တစ်ခုစီကို newline ဖြင့်ရေးပြီး `read_lines` သည် newline များကို ဖယ်ရှားကာ text list အဖြစ် ပြန်ပေးသည်။ Empty file သည် empty list အဖြစ် ပြန်ရနိုင်သည်။

### Mini exercise

1. `scores` number list တစ်ခုဖန်တီးပြီး `sum`၊ `min` နှင့် `max` ဖြင့် report ထုတ်ပါ။
2. User names list ကို sort လုပ်ပြီး `write_lines` ဖြင့် file ထဲသိမ်းပါ။
3. `read_lines` ဖြင့် ပြန်ဖတ်ပြီး `join` ဖြင့် terminal တွင် ပြပါ။
4. Map ထဲမှ မရှိနိုင်သော key များကို `get` ဖြင့် default value သုံးပြီး ဖတ်ပါ။

### Lesson 16 checkpoint

Collection helper များကို မှန်ကန်သော value type နှင့် အသုံးပြုနိုင်ခြင်း၊ မူလ list မပြောင်းဘဲ sorted/reversed result ရယူနိုင်ခြင်း၊ line-based file read/write လုပ်နိုင်ခြင်းနှင့် missing map key အတွက် default value သတ်မှတ်နိုင်ခြင်းတို့ကို လုပ်နိုင်ရမည်။

စာအုပ်၏ နောက်ဆုံး feature status ကို [`README.md`](../README.md) နှင့် [`docs/SYNTAX_GUIDE.md`](SYNTAX_GUIDE.md) တွင် ဆက်လက်စစ်ဆေးပါ။

## Updated Course Completion Checklist

v0.7.0 foundation ပြည့်စုံရန် OOP နှင့် standard library lesson များအပြင်—

1. `is_empty`၊ `sum`၊ `min`၊ `max`၊ `reverse`၊ `sort` နှင့် `join` ကို အသုံးပြုနိုင်ခြင်း။
2. `get(map, key, default)` ဖြင့် missing map key ကို လုံခြုံစွာ ကိုင်တွယ်နိုင်ခြင်း။
3. `read_lines` နှင့် `write_lines` ဖြင့် line-based text file workflow တည်ဆောက်နိုင်ခြင်း။
4. Implemented features နှင့် roadmap-only features ကို documentation မှ ခွဲခြားနားလည်နိုင်ခြင်း။

## Lesson 17 — Lint နှင့် JSON Project Diagnostics

v0.8.0 တွင် source style နှင့် project manifest ကို editor/CI workflow များအတွက် စစ်ဆေးနိုင်သော tooling နှစ်မျိုး ပါဝင်လာသည်။

### `zap lint`

`zap lint` သည် tabs၊ trailing whitespace နှင့် အလွန်ရှည်သော line များကို ရှာဖွေပေးသည်။

```bash
zap lint main.zp
```

အမှားမရှိပါက—

```text
lint ok: main.zp
```

ဟု ပြသည်။ Issue ရှိပါက line number ပါသော warning ပြပြီး command သည် non-zero exit code ဖြင့် ပြန်ထွက်သည်။

### `zap check --json`

Project တစ်ခုတွင် `zap.toml`၊ package name၊ version နှင့် main entry file မှန်ကန်မှုကို JSON အဖြစ် စစ်ဆေးနိုင်သည်။

```bash
zap check --json .
```

အောင်မြင်ပါက—

```json
{"ok":true,"project":"hello-zap 0.1.0 (main: main.zp)"}
```

မအောင်မြင်ပါက `ok` သည် `false` ဖြစ်ပြီး `error` field ထဲတွင် အကြောင်းပြချက် ပါဝင်သည်။ ဤ output သည် shell script၊ CI pipeline နှင့် future editor integration များအတွက် အသုံးဝင်သည်။

### Mini exercise

1. `main.zp` ထဲတွင် tab တစ်ခု ထည့်ပြီး `zap lint main.zp` ဖြင့် စစ်ပါ။
2. Line အဆုံးတွင် မလိုအပ်သော spaces ထည့်ပြီး lint warning ကို ကြည့်ပါ။
3. `zap.toml` မှ `main` file name ကို မမှန်အောင်ပြင်ပြီး `zap check --json .` ဖြင့် JSON error ကို ကြည့်ပါ။
4. Manifest ကို ပြန်ပြင်ပြီး `ok: true` output ပြန်ရအောင် စမ်းပါ။

### Lesson 17 checkpoint

`zap lint` ဖြင့် source style issue ရှာနိုင်ခြင်း၊ `zap check --json` ဖြင့် machine-readable project result ရယူနိုင်ခြင်း၊ success/error JSON နှစ်မျိုးကို ခွဲခြားဖတ်နိုင်ခြင်းနှင့် CI အတွက် command exit status အရေးကြီးကြောင်း နားလည်ရမည်။

## v0.8.0 Course Completion Checklist

1. OOP class၊ constructor၊ property၊ method နှင့် inheritance ကို အသုံးပြုနိုင်ခြင်း။
2. Collection helpers၊ map default lookup နှင့် line-based file I/O ကို အသုံးပြုနိုင်ခြင်း။
3. `zap run` ဖြင့် explicit source execution ပြုလုပ်နိုင်ခြင်း။
4. `zap lint` ဖြင့် tabs၊ trailing whitespace နှင့် long lines စစ်နိုင်ခြင်း။
5. `zap check --json` ဖြင့် project diagnostics ကို automation-friendly JSON အဖြစ် ရယူနိုင်ခြင်း။
6. Implemented features နှင့် roadmap-only features ကို documentation မှ ခွဲခြားနားလည်နိုင်ခြင်း။
7. Integer overflow၊ division by zero နှင့် modulo by zero များသည် runtime error အဖြစ် ပြန်လာပြီး process မပျက်စီးစေရန် နားလည်နိုင်ခြင်း။
8. Function parameter နှင့် return type annotation များကို အသုံးပြုနိုင်ခြင်း။

### Function Type Annotation အသစ်

Zap တွင် function signature ကို အောက်ပါအတိုင်း ရေးနိုင်သည်။ `number`၊ `text`၊ `bool`၊ `list`၊ `map`၊ `none` နှင့် `any` တို့ကို လက်ရှိ runtime မှ စစ်ဆေးပေးသည်။

```zap
fn add(a: number, b: number) -> number:
    return a + b

say add(2, 3)
```

`add("wrong", 3)` ကဲ့သို့ annotation နှင့် မကိုက်ညီသော argument ကို ပေးပါက function မလုပ်ဆောင်ဘဲ `type mismatch` runtime error ပြန်ပေးမည်။ `zap check` သည် function signature ထဲရှိ မသိသော annotation များကို static စစ်ဆေးပေးပြီး `zap check --json` သည် အောက်ပါပုံစံဖြင့် machine-readable diagnostic ပြန်ပေးနိုင်သည်။

```json
{
  "ok": false,
  "kind": "TypeError",
  "message": "TypeError at main.zp:1: unknown type annotation 'unknown_type'",
  "error": "TypeError at main.zp:1: unknown type annotation 'unknown_type'"
}
```

လက်ရှိ `zap check` သည် function definition ၏ annotation syntax/allowed type names အပြင် သိရှိထားသော function call များ၏ argument အရေအတွက်၊ literal argument type၊ variable မှ inferred type နှင့် ရိုးရိုး nested expression type များကိုပါ static စစ်ဆေးပေးသည်။ ဥပမာ `add(1)`၊ `greet(42)`၊ `let first = "wrong"` ပြီး `add(first, 2)` သို့မဟုတ် `add("a" + "b", 2)` ကဲ့သို့ မကိုက်ညီသော call များကို program မ run မီ ဖော်ထုတ်နိုင်သည်။ Variable inference သည် လက်ရှိ literal၊ arithmetic/text expression နှင့် annotated function return အခြေခံအဆင့်ဖြစ်ပြီး complex control-flow inference ကို နောက်အဆင့်တွင် တိုးချဲ့မည်။

```bash
zap check --json .
```

```json
{
  "ok": false,
  "kind": "TypeError",
  "file": "main.zp",
  "line": 3,
  "column": 1,
  "message": "function 'add' expects 2 arguments, got 1"
}
```

ဤ structured fields များကို CI၊ editor နှင့် automation tools များက file၊ line၊ column အလိုက် တိုက်ရိုက်အသုံးပြုနိုင်သည်။

### Result နှင့် Option အသုံးပြုပုံ
v0.9.0 တွင် recoverable value အဖြစ် `ok(value)`၊ `err(value)`၊ `some(value)` နှင့် `option_none()` ကို အသုံးပြုနိုင်သည်။ `is_ok`၊ `is_err`၊ `is_some` နှင့် `is_option_none` ဖြင့် value အမျိုးအစားကို စစ်ဆေးနိုင်ပြီး `unwrap` သို့မဟုတ် `unwrap_or` ဖြင့် value ကို ရယူနိုင်သည်။

```zap
let success = ok(42)
let failure = err("network error")
let name = some("Zap")
let missing = option_none()

say is_ok(success)
say unwrap(success)
say unwrap_or(failure, 0)
say unwrap_or(missing, "unknown")
```

`unwrap(err(...))` သို့မဟုတ် `unwrap(option_none())` ကို စစ်ဆေးခြင်းမရှိဘဲ ခေါ်ပါက runtime error ပြန်ပေးသည်။ `Result` နှင့် `Option` တန်ဖိုးများကို JSON အဖြစ်လည်း serialize လုပ်နိုင်သည်။ လက်ရှိအဆင့်တွင် Result/Option payload static type validation မပြီးသေးသော်လည်း Result error အတွက် `?` automatic propagation ကို အသုံးပြုနိုင်ပြီဖြစ်သည်။

### v0.9.0 Audit Note
v0.9.0 တွင် function parameter/return annotation runtime checks၊ static signature validation၊ known function-call argument count/literal type checking၊ variable/nested-expression inference၊ `Result`/`Option` foundation နှင့် `zap check --json` structured diagnostics ကို ထည့်သွင်းထားသည်။ Generic type inference၊ control-flow narrowing၊ Result/Option payload static validation၊ `async/await`၊ HTTP client၊ package lockfile/registry နှင့် language server များမှာ မပြီးသေးသော roadmap features များ ဖြစ်သည်။ Result error အတွက် `?` automatic propagation ကို ထည့်သွင်းပြီးဖြစ်သည်။ Python-style typing၊ JavaScript-style modules၊ Go-style package/testing workflow နှင့် Dart-style asynchronous Futures/Streams/isolate concepts များကို နှိုင်းယှဉ်လေ့လာပြီး Zap တွင် လွယ်ကူမှုနှင့် safety ကို ဦးစားပေး၍ design လုပ်မည်။

နောက်ထပ် roadmap ကို [`ROADMAP_0.8.0.md`](ROADMAP_0.8.0.md)၊ comparative audit ကို [`AUDIT_LANGUAGE_COMPARISON_2026-08.md`](AUDIT_LANGUAGE_COMPARISON_2026-08.md) နှင့် release details ကို [`RELEASE_0.8.0.md`](RELEASE_0.8.0.md) တွင် ဖတ်ရှုပါ။

## Lesson 18 — Result နှင့် `?` Automatic Propagation

Zap တွင် `ok(value)` နှင့် `err(value)` ဖြင့် Result value များကို ဖန်တီးနိုင်သည်။ `is_ok(result)` နှင့် `is_err(result)` ဖြင့် အခြေအနေစစ်နိုင်ပြီး `unwrap(result)` သို့မဟုတ် `unwrap_or(result, fallback)` ဖြင့် value ကို ရယူနိုင်သည်။

Function တစ်ခုအတွင်း Result error ကို အပေါ်သို့ ပြန်ပို့လိုပါက expression နောက်တွင် `?` ကို ထည့်နိုင်သည်။ `ok(value)?` သည် value ကို ဖြည်ပေးပြီး `err(error)?` သည် လက်ရှိ function မှ `err(error)` ကို ချက်ချင်း return ပြန်ပေးသည်။ ထို့ကြောင့် nested function များတွင် error ကို တစ်ဆင့်ချင်း စစ်ဆေးရေးသားစရာ မလိုတော့ပါ။

```zap
fn read_user():
    return err("user not found")

fn load_profile():
    let user = read_user()?
    return ok(user)

let result = load_profile()
say is_err(result)
```

အထက်ပါ program ၏ output သည် `true` ဖြစ်သည်။ `read_user()` က error ပြန်ပေးသောကြောင့် `load_profile()` သည် `return ok(user)` သို့ မရောက်ဘဲ error Result ကို အပေါ် function သို့ ပြန်ပို့သည်။ Success case တွင်—

```zap
fn read_number():
    return ok(41)

fn calculate():
    let number = read_number()?
    return ok(number + 1)

say unwrap(calculate())
```

ဟုရေးနိုင်ပြီး output သည် `42` ဖြစ်မည်။ `?` ကို Result မဟုတ်သော value ပေါ်တွင် သုံးပါက runtime error ပြန်ရမည်။ Option value များအတွက် လက်ရှိတွင် `is_some`၊ `is_option_none`၊ `unwrap` နှင့် `unwrap_or` ကို အသုံးပြုနိုင်သည်။

### Lesson 18 လေ့ကျင့်ခန်း

1. `find_task(id)` function တစ်ခုရေးပြီး task မတွေ့ပါက `err("not found")` ပြန်ပေးပါ။
2. အခြား function တစ်ခုတွင် `find_task(id)?` ဖြင့် error ကို အပေါ်သို့ propagate လုပ်ပါ။
3. Success နှင့် error နှစ်မျိုးလုံးအတွက် `is_ok`၊ `is_err` နှင့် `unwrap_or` ကို စမ်းသပ်ပါ။

### Lesson 18 checkpoint

Result value တစ်ခုကို ဖန်တီးခြင်း၊ စစ်ဆေးခြင်း၊ ဖြည်ခြင်းနှင့် `?` ဖြင့် error ကို function အပေါ်သို့ အလိုအလျောက် ပြန်ပို့ခြင်းတို့ကို နားလည်ပြီး အသုံးပြုနိုင်ရမည်။


## Lesson 19 — Structured ZapError Diagnostic

Zap runtime တွင် error များကို `ZapError` ဟုခေါ်သော structured diagnostic boundary ဖြင့် ခွဲခြားပေးထားသည်။ လက်ရှိ error kind များမှာ `SyntaxError`၊ `NameError`၊ `TypeError`၊ `ValueError`၊ `IOError`၊ `FileNotFound`၊ `PermissionError`၊ `OverflowError` နှင့် `ProjectError` တို့ ဖြစ်သည်။ Error message နှင့်အတူ source file၊ line နှင့် column information ရှိပါက ထည့်သွင်းဖော်ပြပေးသည်။

Automation သို့မဟုတ် editor integration အတွက် JSON diagnostic ကို အသုံးပြုနိုင်သည်။

```bash
zap check --json .
```

Check မအောင်မြင်ပါက အောက်ပါပုံစံမျိုး output ရနိုင်သည်။

```json
{"ok":false,"kind":"TypeError","file":"main.zp","line":4,"column":12,"message":"expected number, got text","error":"TypeError at main.zp:4:12: expected number, got text"}
```

Human-readable command error နှင့် JSON check error နှစ်မျိုးစလုံးသည် တူညီသော diagnostic classification ကို အသုံးပြုသည်။ Runtime evaluator အတွင်းရှိ legacy Rust `String` error path အချို့ကို `ZapError` သို့ အပြည့်အဝပြောင်းရန်မှာ နောက်ထပ် architecture refactor ဖြစ်ပြီး လက်ရှိ `.zp` syntax ကို မပြောင်းလဲပါ။


## Lesson 20 — Typed Result နှင့် Option Payload

Zap တွင် Result သို့မဟုတ် Option အတွင်းသယ်ဆောင်ထားသည့် value ၏ type ကို static checker ဖြင့် စစ်နိုင်သည်။ Annotation ထဲတွင် angle bracket အသုံးပြုပြီး payload type ကို ရေးပါ။

```zap
let answer: result<number> = ok(42)
let failure: result<text> = err("not found")
let user: option<text> = some("Zap")
let missing: option<number> = option_none()
```

Payload type မကိုက်ညီပါက program မ run မီ checker က reject လုပ်မည်။

```zap
let invalid: result<number> = ok("wrong")
```

Machine-readable `TypeError` ရရန် `zap check --json .` ကို အသုံးပြုနိုင်သည်။ `option_none()` သည် concrete payload မပါသောကြောင့် `option<any>` အဖြစ် သတ်မှတ်ပြီး typed Option များတွင် assign လုပ်နိုင်သည်။
