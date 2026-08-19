# Zap Native Runtime

## ရည်ရွယ်ချက်

Zap native runtime သည် `.zp` source files များကို standalone executable တစ်ခုဖြင့် တိုက်ရိုက် run ပေးသော language runtime ဖြစ်သည်။ Runtime ၏ အဓိကရည်ရွယ်ချက်မှာ installation လွယ်ကူခြင်း၊ platform များအကြား တူညီသော behavior ရှိခြင်းနှင့် Zap language core ကို အနာဂတ် Web၊ Mobile၊ AI နှင့် IoT ecosystem များအတွက် ခိုင်မာသောအခြေခံအဖြစ် တည်ဆောက်ပေးခြင်း ဖြစ်သည်။

လက်ရှိ native implementation သည် Rust ဖြင့်ရေးသားထားပြီး end users များသည် prebuilt release archive မှ binary ကို install လုပ်ကာ မည်သည့် folder မှာမဆို `zap main.zp` ဖြင့် အသုံးပြုနိုင်သည်။

## Supported platforms

Source project တွင် Windows x86_64၊ macOS Apple Silicon၊ macOS Intel နှင့် Linux x86_64 targets များအတွက် build configuration ပါဝင်သည်။ Release tag တင်သောအခါ GitHub Actions သည် platform တစ်ခုချင်းစီအတွက် standalone archive များကို အလိုအလျောက် ထုတ်ပေးနိုင်သည်။

## Source မှ build လုပ်ရန်

Source build သည် runtime ကို တိုးချဲ့မည့် developer များအတွက် ဖြစ်ပြီး Rust toolchain လိုအပ်သည်။

```bash
cd native
cargo build --release
```

ထွက်လာသော binary သည်—

```text
native/target/release/zap
```

Project root မှလည်း build လုပ်နိုင်သည်။

```bash
cargo build --release --manifest-path native/Cargo.toml
```

## Run လုပ်ရန်

```bash
native/target/release/zap native_hello.zp
```

Runtime သည် lexer၊ expression parser နှင့် block executor pipeline မှတစ်ဆင့် `say`၊ variable assignment၊ text၊ integer၊ boolean၊ list၊ map၊ indexing၊ `len()`၊ `range()`၊ `str()`၊ `json()`၊ `from_json()`၊ `read_text()`၊ `write_text()`၊ arithmetic၊ comparison၊ `and/or/not`၊ `if/else`၊ `for`/`while`၊ function definition/call၊ local scope၊ lexical closures၊ `return` နှင့် module loading တို့ကို ဆောင်ရွက်ပေးသည်။

## Architecture

| အပိုင်း | လက်ရှိအကောင်အထည်ဖော်မှု |
|---|---|
| Lexer/parser | Native tokenizer နှင့် expression parser |
| Runtime | Standalone native executable |
| Data values | text၊ integer၊ boolean၊ list၊ map၊ none |
| Control flow | `if/else`၊ `for`၊ `while`၊ `break`၊ `continue` |
| Functions | `fn`၊ parameters၊ calls၊ local scope၊ lexical closures၊ `return` |
| Operators | arithmetic၊ modulus၊ comparison၊ `and/or/not` |
| Built-ins | `len()`၊ `range()`၊ `str()`၊ `json()`၊ `from_json()`၊ `read_text()`၊ `write_text()` |
| Modules | `use "module.zp"` source-relative local module loading |
| Distribution | Platform-specific binary archives နှင့် installers |
| Targets | Linux x86_64၊ macOS arm64/x86_64 နှင့် Windows x86_64 |

## Binary package installation

End users များသည် source build မလုပ်ဘဲ GitHub Releases မှ သက်ဆိုင်ရာ archive ကို download လုပ်ပြီး extract လုပ်နိုင်သည်။ Linux/macOS တွင်—

```bash
bash install.sh
zap --version
```

ဟု run လုပ်ပါ။ Installer သည် `zap` binary ကို user-level PATH directory ထဲသို့ ထည့်ပြီး မည်သည့် folder မှာမဆို `zap main.zp` ဖြင့် အသုံးပြုနိုင်စေသည်။ Windows တွင် `install_windows.bat` ကို run လုပ်ပြီး Command Prompt အသစ်ဖွင့်ပါ။

Source checkout မှ local binary တည်ဆောက်လိုပါက—

```bash
./build_native.sh
```

Linux/macOS တွင် run လုပ်နိုင်ပြီး Windows တွင် `build_native.bat` ကို အသုံးပြုနိုင်သည်။ GitHub release workflow သည် version tag တင်သောအခါ Windows၊ macOS နှင့် Linux archives များကို build လုပ်ရန် ပြင်ဆင်ထားသည်။

## Runtime design principles

Zap runtime သည် language behavior ကို တစ်နေရာတည်းတွင် စုစည်းထားသော native execution path ဖြင့် ထိန်းသိမ်းသည်။ ဤပုံစံသည် release binary ၏ installation ကို ရိုးရှင်းစေပြီး source file များကို platform မတူညီသော်လည်း တူညီသော CLI workflow ဖြင့် run နိုင်စေသည်။ Error diagnostics၊ module resolution နှင့် project manifest validation တို့ကို runtime နှင့် tooling အဆင့်တွင် တဖြည်းဖြည်း တိုးတက်အောင် ပြုလုပ်မည်။

## နောက်တစ်ဆင့်

Native project တွင် AST parser တိုးတက်မှု၊ ပိုမိုတိကျသော line/column error locations၊ formatter တိုးချဲ့မှု၊ package lockfile၊ dependency registry၊ bytecode execution နှင့် security sandbox တို့ကို ဆက်လက်တည်ဆောက်မည်။ Language core တည်ငြိမ်လာသောအခါ Web၊ Android/Mobile၊ AI နှင့် IoT frameworks များကို Zap packages အဖြစ် တည်ဆောက်ရန် ရည်ရွယ်ထားသည်။

> လက်ရှိ native runtime သည် early development release ဖြစ်ပြီး production compiler အဖြစ် မသတ်မှတ်ရသေးပါ။ Syntax နှင့် runtime behavior များသည် development အတွင်း ပြောင်းလဲနိုင်သောကြောင့် release notes နှင့် project specification များကို အမြဲစစ်ဆေးသင့်သည်။

အသေးစိတ် user workflow ကို [`USAGE.md`](USAGE.md) နှင့် project overview ကို [`README.md`](../README.md) တွင် ဖတ်ရှုနိုင်သည်။
