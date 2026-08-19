# Zap Programming Language

> **Zap** သည် `.zp` file extension အသုံးပြုသည့်၊ ဖတ်ရလွယ်ကူသော indentation-based syntax နှင့် standalone native runtime ပါဝင်သည့် general-purpose programming language ဖြစ်သည်။ Web၊ AI၊ Mobile နှင့် IoT ecosystem များအတွက် language core ကို ရိုးရှင်းပြီး ချဲ့ထွင်နိုင်အောင် တည်ဆောက်နေသည်။

Zap သည် စတင်လေ့လာသူများအတွက် syntax ရိုးရှင်းစေရန် ရည်ရွယ်သော်လည်း functions၊ closures၊ JSON၊ file I/O၊ modules၊ project manifest နှင့် native CLI tooling များပါဝင်သည့် practical foundation တစ်ခုကို ပေးထားသည်။

## လက်ရှိ Version

| အချက် | အခြေအနေ |
|---|---|
| Current development line | `v0.6.0` |
| Runtime | Native Rust runtime |
| CLI | `zap` |
| Source file | `.zp`၊ ဥပမာ `main.zp` |
| Manifest | `zap.toml` |
| Current platforms | Linux၊ Windows၊ macOS ARM64 release packages |
| Repository | [github.com/hidecard/zap](https://github.com/hidecard/zap) |
| Beginner lessons | [`docs/LEARN_ZAP_MM.md`](docs/LEARN_ZAP_MM.md) |
| Full syntax reference | [`docs/SYNTAX_GUIDE.md`](docs/SYNTAX_GUIDE.md) |

Zap သည် ဆက်လက်တိုးချဲ့နေသော early development language ဖြစ်သောကြောင့် stable အဖြစ် သတ်မှတ်ထားသော feature နှင့် roadmap proposal ကို ခွဲခြားဖတ်ရှုပါ။

## အမြန်စတင်ခြင်း

### 1။ Installation

GitHub ရှိ [Releases](https://github.com/hidecard/zap/releases) မှ သင့် operating system နှင့် CPU architecture ကိုက်ညီသော archive ကို download လုပ်ပြီး extract လုပ်ပါ။ Native binary သည် runtime dependency မလိုဘဲ တိုက်ရိုက် run နိုင်သည်။

Linux သို့မဟုတ် macOS တွင်—

```bash
tar -xzf zap-0.6.0-linux-x86_64.tar.gz
cd zap-0.6.0
bash install.sh
zap --version
```

Windows တွင် archive ကို extract ပြီး `install_windows.bat` ကို run လုပ်ပါ။ ထို့နောက် Command Prompt အသစ်ဖွင့်ပြီး—

```bat
install_windows.bat
zap --version
```

Installer မသုံးဘဲ direct run လုပ်လိုပါက—

```bat
bin\zap.exe main.zp
```

### 2။ ပထမဆုံး Program

`hello.zp` file တစ်ခုဖန်တီးပါ။

```zap
say "Hello from Zap"
```

Run လုပ်ပါ။

```bash
zap hello.zp
```

### 3။ Project ဖန်တီးခြင်း

```bash
zap init hello-project
cd hello-project
zap check .
zap build .
zap test .
zap main.zp
```

`zap init` သည် `zap.toml`၊ `main.zp` နှင့် starter test ပါဝင်သော project structure ကို ဖန်တီးပေးသည်။

## Zap Language Feature Matrix

| အပိုင်း | လက်ရှိ support |
|---|---|
| Values | text၊ integer number၊ boolean၊ list၊ map၊ none |
| Variables | `let` declaration နှင့် reassignment |
| Optional annotations | `text`၊ `number`၊ `bool`၊ `list`၊ `map`၊ `none`၊ `any` |
| Operators | `+`၊ `-`၊ `*`၊ `/`၊ `%`၊ comparison၊ `and`၊ `or`၊ `not` |
| Control flow | `if/else`၊ `for`၊ `while`၊ `break`၊ `continue` |
| Functions | parameters၊ return values၊ local scope၊ nested closures |
| Collections | list indexing၊ map indexing၊ `keys`၊ `contains`၊ `join` |
| Text | `upper`၊ `lower`၊ `trim`၊ `split`၊ `str`၊ `len` |
| JSON | `json` နှင့် `from_json` |
| Files | `read_text` နှင့် `write_text` |
| Path/time/env | `path_join`၊ `basename`၊ `dirname`၊ `exists`၊ `now`၊ `sleep`၊ `env`၊ `has_env` |
| Math | `abs`၊ `min`၊ `max`၊ `pow`၊ `sqrt` |
| Modules | local `.zp` modules၊ `modules/` နှင့် `lib/` search paths |
| CLI | `init`၊ `check`၊ `build`၊ `test`၊ `fmt`၊ run၊ help၊ version |

## Syntax အခြေခံများ

### Variables နှင့် Types

```zap
let name: text = "Zap"
let version: number = 6
let ready: bool = true
let items: list = ["web", "ai", "iot"]
let settings: map = {"mode": "dev", "debug": true}

say name
say version
say type(items)
```

Type annotation သည် optional ဖြစ်သည်။ Annotation ရေးထားပါက assigned value နှင့် type ကို runtime က စစ်ဆေးပေးသည်။ Annotation မရေးထားလည်း dynamic value အဖြစ် အသုံးပြုနိုင်သည်။

### Comments

```zap
# ဒီစာကြောင်းသည် comment ဖြစ်သည်
say "Comments are ignored by the runtime"
```

### Operators

```zap
let total = 10 + 5 * 2
let remainder = 17 % 4
let same = total == 20
let allowed = same or not false

say total
say remainder
say allowed
```

### Conditions

```zap
let score = 85

if score >= 80:
    say "Excellent"
else:
    say "Keep practising"
```

Zap blocks များသည် indentation ဖြင့် သတ်မှတ်သည်။ Block အတွင်းရှိ code များကို indentation တစ်ဆင့်တည်း ထားပါ။

### Lists နှင့် Loops

```zap
let languages = ["Zap", "Rust", "Python"]

for language in languages:
    say language

let n = 0
while n < 3:
    say n
    n = n + 1
```

### Functions နှင့် Closures

```zap
fn add(a, b):
    return a + b

fn make_greeting(prefix):
    fn greet(name):
        return prefix + ", " + name
    return greet("Developer")

say add(4, 6)
say make_greeting("Hello")
```

### Maps နှင့် JSON

```zap
let user = {
    "name": "Zap User",
    "age": 20,
    "skills": ["web", "ai"]
}

say user["name"]
say keys(user)
say json(user)

let decoded = from_json("{\"ok\": true}")
say decoded["ok"]
```

### File I/O နှင့် Path

```zap
let file = path_join("data", "note.txt")

if exists(file):
    say read_text(file)
else:
    write_text(file, "Hello from Zap")
    say "File created"
```

### Time နှင့် Environment

```zap
let started_at: number = now()
say started_at

if has_env("PATH"):
    say env("PATH")

sleep(10)
say "finished"
```

## Built-in Reference

| Function | အသုံးပြုပုံ | ရည်ရွယ်ချက် |
|---|---|---|
| `say` | `say value` | Output ပြသခြင်း |
| `len` | `len(value)` | Text သို့မဟုတ် list အရှည် |
| `range` | `range(5)`၊ `range(2, 5)` | Number list ဖန်တီးခြင်း |
| `str` | `str(value)` | Value ကို text ပြောင်းခြင်း |
| `type` | `type(value)` | Value type သိခြင်း |
| `keys` | `keys(map)` | Map keys ရယူခြင်း |
| `contains` | `contains(collection, item)` | ပါဝင်မှုစစ်ခြင်း |
| `join` | `join(list, separator)` | List ကို text ပေါင်းခြင်း |
| `upper/lower` | `upper(text)` | Text case ပြောင်းခြင်း |
| `trim/split` | `trim(text)`၊ `split(text, sep)` | Text ပြင်ဆင်ခြင်း |
| `json/from_json` | `json(value)` | JSON encode/decode |
| `read_text/write_text` | `read_text(path)` | Text file I/O |
| `path_join` | `path_join(a, b)` | Platform-aware path ပေါင်းခြင်း |
| `basename/dirname` | `basename(path)` | Path အစိတ်အပိုင်းရယူခြင်း |
| `exists` | `exists(path)` | File/path ရှိမရှိစစ်ခြင်း |
| `now/sleep` | `now()`၊ `sleep(ms)` | Time နှင့် delay |
| `env/has_env` | `env(name)` | Environment ဖတ်ခြင်း |
| `abs/min/max` | `abs(-4)` | Numeric helpers |
| `pow/sqrt` | `pow(2, 3)` | Power နှင့် square root |
| `assert` | `assert(condition, message)` | Program/test condition စစ်ခြင်း |

## CLI Command Reference

```text
zap <file.zp>       Zap source file ကို run လုပ်သည်
zap init <dir>       Project အသစ် scaffold ဖန်တီးသည်
zap check [dir]      zap.toml နှင့် main file စစ်သည်
zap build [dir]      Build-ready project validation ပြုလုပ်သည်
zap test [dir]       *_test.zp files များကို recursive run သည်
zap fmt <file.zp>    Formatting ပြုလုပ်ပြီး file ကို update သည်
zap --version        Runtime version ပြသည်
zap --help           Command help ပြသည်
```

## Project Structure

```text
my-zap-project/
├── zap.toml
├── main.zp
├── modules/
│   └── helpers.zp
├── lib/
│   └── format.zp
└── tests/
    └── smoke_test.zp
```

Manifest နမူနာ—

```toml
[package]
name = "my-zap-project"
version = "0.6.0"
main = "main.zp"
```

Local module အသုံးပြုရန်—

```zap
use "helpers"
say greet("Zap")
```

Module resolution သည် source file ၏ directory၊ `modules/` နှင့် `lib/` တို့ကို ရှာဖွေသည်။

## Testing

Test file အမည်သည် `_test.zp` ဖြင့်ဆုံးရမည်။

```zap
fn add(a, b):
    return a + b

assert(add(2, 3) == 5, "add function failed")
assert(type(add(2, 3)) == "number", "result type failed")
say "test passed"
```

Run—

```bash
zap test
zap test tests
```

Native runtime integration tests များကို—

```bash
cd native
cargo test
```

ဖြင့် run နိုင်သည်။

## Formatting နှင့် Error Debugging

```bash
zap fmt main.zp
zap check .
```

Error တက်ပါက file path၊ line အနီးရှိ syntax၊ variable name၊ function arguments နှင့် data type များကို အရင်စစ်ဆေးပါ။ `assert` တွင် အဓိပ္ပာယ်ရှိသော message ရေးပါ။

```zap
assert(total >= 0, "total must not be negative")
```

## Beginner Learning Path

စတင်လေ့လာသူများသည် အောက်ပါအစီအစဉ်အတိုင်း လေ့လာသင့်သည်။

| Lesson | အကြောင်းအရာ |
|---|---|
| 1 | Installation နှင့် Hello World |
| 2 | Output၊ comments နှင့် program structure |
| 3 | Variables နှင့် value types |
| 4 | Operators နှင့် calculations |
| 5 | Conditions |
| 6 | Lists နှင့် indexing |
| 7 | Maps နှင့် JSON |
| 8 | `for` နှင့် `while` loops |
| 9 | Functions နှင့် return |
| 10 | Closures နှင့် scope |
| 11 | File I/O၊ path၊ time နှင့် environment |
| 12 | Modules နှင့် project structure |
| 13 | Tests၊ formatter နှင့် CLI workflow |
| 14 | Complete mini project |

Lesson တစ်ခုစီတွင် explanation၊ runnable code၊ expected output နှင့် exercise ပါဝင်သည်။ [`docs/LEARN_ZAP_MM.md`](docs/LEARN_ZAP_MM.md) ကို ဖွင့်ပြီး Lesson 1 မှ စတင်ပါ။

## Examples

Repository ၏ `examples/` folder တွင် runnable examples များ ပါဝင်သည်။

```bash
zap examples/hello.zp
zap examples/data.zp
zap examples/tasks.zp
zap examples/advanced.zp
```

## Native Architecture

Zap ၏ runtime သည် Rust ဖြင့်ရေးသားထားသော standalone binary ဖြစ်သည်။ Source code သည် `native/`၊ language documentation သည် `docs/`၊ runnable examples သည် `examples/`၊ အဟောင်း prototype များသည် `legacy/` တွင် ခွဲထားသည်။ Runtime သည် source ကို tokenize လုပ်ပြီး expression evaluation နှင့် line execution အဆင့်များဖြင့် run သည်။

Production compiler၊ bytecode execution၊ package registry နှင့် framework layers များသည် နောက်ပိုင်း roadmap အစိတ်အပိုင်းများ ဖြစ်သည်။

## v0.6.0 Status နှင့် Roadmap

### လက်ရှိအကောင်အထည်ဖော်ပြီးသောအရာများ

Native runtime version `0.6.0`၊ path/time/environment/math built-ins၊ optional variable annotations၊ `zap build`၊ updated documentation နှင့် regression tests များ ပါဝင်သည်။

### နောက်ထပ်တိုးချဲ့မည့်အရာများ

Structured `Result` error model၊ source line/column diagnostics၊ HTTP client၊ async/await၊ tasks၊ channels၊ `zap lint`၊ `zap check --json`၊ `zap test --watch`၊ package lockfile နှင့် package registry များကို အဆင့်ဆင့် ဆက်လက်လုပ်ဆောင်မည်။

`async`/`await`၊ HTTP client၊ channels နှင့် package registry များသည် ယခု stable runtime တွင် မပါဝင်သေးပါ။ အသေးစိတ် roadmap ကို [`docs/ROADMAP_0.6.0.md`](docs/ROADMAP_0.6.0.md) နှင့် design ကို [`docs/DESIGN.md`](docs/DESIGN.md) တွင် ဖတ်ရှုပါ။

## Documentation Map

| Document | ရည်ရွယ်ချက် |
|---|---|
| [`docs/LEARN_ZAP_MM.md`](docs/LEARN_ZAP_MM.md) | Beginner Burmese lessons နှင့် exercises |
| [`docs/LANGUAGE_GUIDE.md`](docs/LANGUAGE_GUIDE.md) | Complete language usage guide |
| [`docs/SYNTAX_GUIDE.md`](docs/SYNTAX_GUIDE.md) | Syntax နှင့် code reference |
| [`docs/USAGE.md`](docs/USAGE.md) | Installation၊ CLI နှင့် usage workflow |
| [`docs/ROADMAP_0.6.0.md`](docs/ROADMAP_0.6.0.md) | v0.6.0 implementation roadmap |
| [`docs/DESIGN.md`](docs/DESIGN.md) | Language design principles |
| [`docs/ECOSYSTEM.md`](docs/ECOSYSTEM.md) | Core၊ standard library နှင့် future frameworks |
| [`docs/PACKAGES.md`](docs/PACKAGES.md) | Manifest၊ modules နှင့် package future |
| [`docs/NATIVE.md`](docs/NATIVE.md) | Native runtime development |

## Contributing

Zap ကို စမ်းသပ်အသုံးပြုပြီး bug၊ syntax အခက်အခဲ၊ documentation ပြင်ဆင်ချက် သို့မဟုတ် feature proposal များကို GitHub repository ၏ Issues နှင့် Pull Requests မှတစ်ဆင့် တင်ပြနိုင်သည်။ Bug report တွင် operating system၊ `zap --version` output၊ source code အတိုနှင့် error output ကို ထည့်ပါ။

## License

Repository တွင် သတ်မှတ်ထားသော license စည်းမျဉ်းများကို လိုက်နာပါ။ License နှင့် release package အချက်အလက်များကို [GitHub repository](https://github.com/hidecard/zap) တွင် ကြည့်ရှုနိုင်သည်။
