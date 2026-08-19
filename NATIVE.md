# Zap Native Runtime Prototype

## အဓိကအဖြေ

ရပါတယ်။ Zap ကို Python မမှီခိုသော language အဖြစ် တည်ဆောက်နိုင်သည်။ ယခု project ထဲတွင် Rust ဖြင့်ရေးသားထားသော native runtime prototype ပါဝင်ပြီး `zap` executable သည် Zap source ကို Python မသုံးဘဲ တိုက်ရိုက်ဖတ်ကာ execute လုပ်သည်။

## Supported platforms

Source project သည် Windows x86_64၊ macOS Apple Silicon၊ macOS Intel နှင့် Linux x86_64 targets များအတွက် build configuration ပါဝင်သည်။ Release tag တင်သောအခါ GitHub Actions သည် platform တစ်ခုချင်းစီအတွက် standalone archive များကို အလိုအလျောက် ထုတ်ပေးနိုင်သည်။

## Build လုပ်ရန်

Rust toolchain လိုအပ်သည်။ Native executable ကို တည်ဆောက်ရန်—

```bash
cd native
cargo build --release
```

ထွက်လာသော binary သည်—

```text
native/target/release/zap
```

## Run လုပ်ရန်

```bash
native/target/release/zap native_hello.zp
```

ဤ command ကို run ရန် Python မလိုအပ်ပါ။ Runtime သည် lexer၊ expression parser၊ `say`၊ variable assignment၊ string၊ integer၊ boolean၊ list၊ map၊ indexing၊ `len()`၊ `range()`၊ `str()`၊ `json()`၊ `from_json()`၊ `read_text()`၊ `write_text()`၊ arithmetic၊ modulus၊ comparison၊ `and/or/not`၊ `if/else`၊ `for`/`while`၊ function definition/call၊ local function scope၊ `return` နှင့် module declaration ကို native Rust code ဖြင့် ဆောင်ရွက်သည်။

## Architecture

| အပိုင်း | လက်ရှိအကောင်အထည်ဖော်မှု |
|---|---|
| Lexer/parser | Rust tokenizer နှင့် expression parser |
| Runtime | Rust native executable |
| Data values | text၊ integer၊ boolean၊ list၊ map၊ none |
| Control flow | `if/else`၊ `for`၊ `while` |
| Functions | `fn`/`def`၊ parameters၊ calls၊ local scope၊ lexical closures၊ `return` |
| Operators | arithmetic၊ modulus၊ comparison၊ `and/or/not` |
| Built-ins | `len()`၊ `range()`၊ `str()`၊ `json()`၊ `from_json()`၊ `read_text()`၊ `write_text()` |
| Modules | `use "module.zp"` source-relative local module loading |
| Python dependency | မရှိ |
| Target | Linux x86_64၊ macOS arm64/x86_64 နှင့် Windows x86_64 release archives |

## Python version နှင့် native version ကွာခြားချက်

Python runtime version သည် reference prototype အဖြစ်သာ ကျန်ရှိပြီး native Zap runtime သည် Python မလိုသော primary implementation ဖြစ်သည်။ Native core သည် language features များကို ဆက်လက်တိုးချဲ့နေဆဲဖြစ်ပြီး framework implementation များကို core တည်ငြိမ်ပြီးနောက် ဆက်လုပ်မည်။

## Binary package install

End users များသည် Rust သို့မဟုတ် Python မလိုဘဲ GitHub Releases မှ သက်ဆိုင်ရာ archive ကို download လုပ်ပြီး extract လုပ်နိုင်သည်။ Extract လုပ်ထားသော directory ထဲတွင်—

```bash
bash install.sh
zap --version
```

ဟု run လုပ်ပါ။ Installer သည် `zap` binary ကို user-level PATH directory ထဲသို့ ထည့်ပြီး မည်သည့် folder မှာမဆို `zap main.zp` ဖြင့် အသုံးပြုနိုင်စေသည်။ Windows တွင် `install_windows.bat` ကို run လုပ်ပြီး Command Prompt အသစ်ဖွင့်ပါ။

Source မှ build လုပ်မည့် developer များအတွက်—

Linux/macOS တွင်—

```bash
./build_native.sh
```

Windows တွင် `build_native.bat` ကို run လုပ်ပါ။ Rust ရှိပြီးသား developer များသည် project root မှ—

```bash
cargo build --release --manifest-path native/Cargo.toml
```

ကိုလည်း အသုံးပြုနိုင်သည်။ GitHub release workflow သည် `v0.3.0` ကဲ့သို့ tag တင်သောအခါ Windows၊ macOS နှင့် Linux archives များကို build လုပ်ရန် ပြင်ဆင်ထားသည်။

## နောက်တစ်ဆင့်

Native project တွင် AST parser တိုးတက်မှုနှင့် ပိုမိုတိကျသော line/column error locations များကို ဆက်လက်တိုးချဲ့မည်။ Nested lexical closures၊ JSON encode/decode၊ formatter၊ package manifest နှင့် installable standalone binary archives များကို ထည့်သွင်းပြီးဖြစ်သည်။

> Native prototype သည် proof-of-concept ဖြစ်ပြီး production compiler မဟုတ်သေးပါ။
