# Zap Programming Language

> **Zap** သည် ရိုးရှင်းသော indentation-based syntax ဖြင့် လေ့လာရလွယ်ကူပြီး standalone အဖြစ် တိုက်ရိုက် run နိုင်သော programming language ဖြစ်သည်။ `.zp` source files များကို Linux၊ macOS နှင့် Windows တွင် `zap` CLI ဖြင့် အသုံးပြုနိုင်ရန် ရည်ရွယ်ထားသည်။

Zap ၏ အဓိကရည်ရွယ်ချက်မှာ beginner များအတွက် ရိုးရှင်းသော syntax နှင့် developer များအတွက် တဖြည်းဖြည်းချဲ့ထွင်နိုင်သော native runtime တစ်ခုကို ပေါင်းစပ်ပေးရန်ဖြစ်သည်။ Web၊ Mobile၊ AI နှင့် IoT frameworks များသည် language core တည်ငြိမ်ပြီးနောက် သီးခြား ecosystem အဖြစ် တည်ဆောက်မည်ဖြစ်သည်။

## လက်ရှိအခြေအနေ

Zap သည် **native core prototype / early development release** အဆင့်တွင် ရှိပါသည်။ Native Rust runtime သည် အောက်ပါ core features များကို လက်ရှိထောက်ပံ့ထားသည်။

| အပိုင်း | လက်ရှိ support |
|---|---|
| Source extension | `.zp`၊ ဥပမာ `main.zp` |
| Values | text၊ integer၊ boolean၊ list၊ map၊ none |
| Variables | declaration နှင့် reassignment |
| Operators | arithmetic၊ `%` modulus၊ comparison၊ `and`၊ `or`၊ `not` |
| Control flow | `if/else`၊ `for`၊ `while`၊ `break`၊ `continue` |
| Functions | parameters၊ calls၊ `return`၊ local scope၊ nested lexical closures |
| Modules | source-relative `use "module.zp"` imports၊ `modules/` နှင့် `lib/` search paths |
| Built-ins | `say`၊ `len`၊ `range`၊ `str`၊ `json`၊ `from_json`၊ `read_text`၊ `write_text` |
| Tooling | `zap --help`၊ `zap --version`၊ `zap check`၊ `zap fmt` |
| Installation | Prebuilt native binary package |

## Zap ကို Install လုပ်ခြင်း

End users များအတွက် development toolchain မလိုပါ။ GitHub Releases မှ သင့် operating system နှင့် CPU architecture ကိုက်ညီသော archive ကို download လုပ်ပြီး extract လုပ်ပါ။ Linux/macOS တွင် archive directory ထဲမှ installer ကို run လုပ်ပါ။

```bash
tar -xzf zap-linux-x86_64.tar.gz
cd zap-0.3.0
bash install.sh
```

Terminal အသစ်ဖွင့်ပြီး installation ကို စစ်ဆေးပါ။

```bash
zap --version
zap --help
```

ထို့နောက် မည်သည့် folder မှာမဆို Zap source file ကို run နိုင်ပါသည်။

```bash
mkdir hello-zap
cd hello-zap
printf 'say "Hello from Zap"\n' > main.zp
zap main.zp
```

Windows တွင် release archive ကို extract ပြီး `install_windows.bat` ကို run လုပ်ပါ။ Command Prompt အသစ်ဖွင့်ပြီး အောက်ပါ command များကို အသုံးပြုပါ။

```bat
zap --version
zap main.zp
```

Installer သည် user-level directory ထဲသို့ binary ထည့်ပြီး PATH ကို update လုပ်ရန် ရည်ရွယ်ထားသောကြောင့် administrator permission မလိုသော installation flow ကို ဦးစားပေးထားသည်။

### Source မှ build လုပ်ခြင်း

Developer များသည် Rust toolchain ရှိပါက source မှ native binary build လုပ်နိုင်သည်။

```bash
make native
./bin/zap --version
./bin/zap main.zp
```

သို့မဟုတ်—

```bash
cargo build --release --manifest-path native/Cargo.toml
```

Source build သည် development အတွက်သာ ဖြစ်သည်။ ပုံမှန် user installation အတွက် prebuilt release archive ကို အသုံးပြုခြင်းဖြင့် အပို development toolchain မလိုတော့ပါ။

## CLI အသုံးပြုနည်း

```bash
zap main.zp                 # program run
zap --version               # runtime version
zap --help                  # command help
zap check                   # current project manifest validate
zap check path/to/project   # specific project validate
zap fmt main.zp             # source formatting
```

`zap` သည် file path ကို တိုက်ရိုက်လက်ခံသော native runtime ဖြစ်သည်။ Repository ထဲရှိ reference tooling သည် language behavior စမ်းသပ်ရန်အတွက်သာ ဖြစ်ပြီး ပုံမှန် Zap အသုံးပြုမှုအတွက် မလိုအပ်ပါ။

## ပထမဆုံး Zap Program

```zp
let name = "Zap"
let numbers = range(5)

say "Hello, " + name

for number in numbers:
    if number == 2:
        continue
    if number == 4:
        break
    say number
```

Zap သည် indentation-based block syntax ကို အသုံးပြုသည်။ Block စတင်ရန် colon (`:`) သုံးပြီး နောက်လိုက် lines များကို indentation ဖြင့် ရေးသားသည်။

## Functions နှင့် Closures

```zp
fn make_adder(base):
    fn add(value):
        return base + value
    return add(10)

let result = make_adder(5)
say result
```

Function တစ်ခုအတွင်းရှိ variable များသည် local scope ရှိပြီး nested function သည် outer function ၏ variable များကို capture လုပ်နိုင်သည်။

## Lists၊ Maps နှင့် JSON

```zp
let user = {
    "name": "Ada",
    "language": "Zap"
}

let encoded = json(user)
say encoded

let decoded = from_json(encoded)
say decoded["name"]
```

`json(value)` သည် Zap value ကို JSON text အဖြစ် encode လုပ်ပြီး `from_json(text)` သည် JSON text ကို Zap value အဖြစ် decode လုပ်သည်။

## File I/O

```zp
write_text("message.txt", "Zap file I/O")
let message = read_text("message.txt")
say message
```

File path များကို program ၏ current working directory အပေါ် အခြေခံ၍ အသုံးပြုသည်။ Production sandbox နှင့် permission policy များကို runtime အဆင့်တွင် ဆက်လက်တိုးချဲ့မည်။

## Modules နှင့် Project Manifest

Project တစ်ခုတွင် `zap.toml` manifest ထည့်နိုင်သည်။

```toml
[package]
name = "hello-app"
version = "0.1.0"
main = "main.zp"
```

အကြံပြု project layout သည်—

```text
hello-app/
├── zap.toml
├── main.zp
├── modules/
│   └── math.zp
└── lib/
    └── text.zp
```

Module ကို အောက်ပါအတိုင်း import လုပ်နိုင်သည်။

```zp
use "math.zp"
```

Runtime သည် main source file ရှိသော directory၊ project ၏ `modules/` နှင့် `lib/` directories များအတွင်း module ကို ရှာဖွေသည်။ Manifest နှင့် main entry file ကို validate လုပ်ရန်—

```bash
zap check
```

အသေးစိတ် package manifest specification ကို [`PACKAGE.md`](PACKAGE.md) တွင် ဖတ်နိုင်သည်။

## Formatter

Zap source ကို canonical whitespace format သို့ ပြောင်းရန်—

```bash
zap fmt main.zp
```

Formatter သည် indentation ကို normalize လုပ်ပြီး trailing whitespace များကို ဖယ်ရှားသည်။ လက်ရှိ formatter သည် syntax-changing formatter မဟုတ်သေးဘဲ safe whitespace normalization ကို ဦးစားပေးထားသည်။

## Native Architecture

Zap native runtime ၏ လုပ်ဆောင်ပုံမှာ—

```text
.zp source
    │
    ▼
Lexer / tokenizer
    │
    ▼
Expression parser
    │
    ▼
Block and statement executor
    │
    ▼
Native Rust runtime
```

Runtime သည် source ကို native execution pipeline ဖြင့် တိုက်ရိုက် run လုပ်သည်။ Native package metadata နှင့် source code များသည် [`native/`](native/) directory ထဲတွင် ရှိသည်။ အသေးစိတ် native implementation notes ကို [`NATIVE.md`](NATIVE.md) တွင် ဖတ်နိုင်သည်။

## Release Package တည်ဆောက်ခြင်း

Rust toolchain ရှိသော maintainer များသည် local binary archive ထုတ်ရန်—

```bash
make package
```

ဟု run လုပ်နိုင်သည်။ Archive ကို `dist/` directory ထဲတွင် ထုတ်ပေးပြီး `bin/zap`၊ installer နှင့် documentation များ ပါဝင်သည်။ SHA-256 checksum file ကိုလည်း အတူထုတ်ပေးသည်။ Release workflow သည် Linux၊ macOS Apple Silicon၊ macOS Intel နှင့် Windows targets များအတွက် binary-only archives ထုတ်ရန် ပြင်ဆင်ထားသည်။

## Tests

Native runtime နှင့် integration tests များကို run လုပ်ရန်—

```bash
make native-test
```

Reference compatibility tests များသည် optional ဖြစ်ပြီး project development အတွင်းသာ အသုံးပြုနိုင်သည်။ End-user installation နှင့် native CLI အသုံးပြုမှုအတွက် အပို runtime မလိုပါ။

## Repository ဖိုင်များ

| ဖိုင်/Directory | ရည်ရွယ်ချက် |
|---|---|
| `native/` | Rust native runtime၊ Cargo metadata နှင့် integration tests |
| `bin/zap` | Local release build ထုတ်ထားသော native CLI binary |
| `zap.py` | Optional reference tooling |
| `hello.zp`, `advanced.zp` | Language examples |
| `native_hello.zp` | Native runtime smoke-test example |
| `install.sh` | Linux/macOS binary installer |
| `install_windows.bat` | Windows binary installer |
| `build_native.sh`, `build_native.bat` | Native binary build helpers |
| `package_release.sh` | Binary-only release archive builder |
| `.github/workflows/release.yml` | Cross-platform release automation |
| `CORE_SPEC.md` | Language core specification |
| `DESIGN.md` | Language design notes |
| `NATIVE.md` | Native runtime documentation |
| `PACKAGE.md` | `zap.toml` manifest နှင့် package layout specification |
| `USAGE.md` | Detailed usage guide |
| `ECOSYSTEM.md` | Future Web/Mobile/AI/IoT ecosystem plan |
| `Makefile` | Build၊ test နှင့် package shortcuts |

## Roadmap

Zap core ၏ လက်ရှိဦးစားပေးများမှာ ပိုမိုတိကျသော line/column diagnostics၊ formatter တိုးတက်မှု၊ package lockfile၊ dependency registry နှင့် bytecode/optimized execution ဖြစ်သည်။ Language core တည်ငြိမ်ပြီး tooling ပြည့်စုံလာသောအခါ Web၊ Android/Mobile၊ AI နှင့် IoT frameworks များကို Zap packages အဖြစ် စတင်တည်ဆောက်မည်။

ယခု release သည် production compiler မဟုတ်သေးသော early native runtime ဖြစ်သည်။ Syntax နှင့် runtime behavior များသည် development အတွင်း ပြောင်းလဲနိုင်သောကြောင့် project examples၊ tests နှင့် specification files များကို အမြဲတမ်းအတူတကွ စစ်ဆေးသင့်သည်။

## License

Zap သည် MIT License အောက်တွင် ဖြန့်ချိထားသည်။ အသေးစိတ်ကို [`LICENSE`](LICENSE) တွင် ဖတ်နိုင်သည်။
