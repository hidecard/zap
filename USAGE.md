# Zap 0.2 — Complete Usage Guide

## 1. Zap ဆိုတာဘာလဲ

Zap သည် Python နှင့် JavaScript ကဲ့သို့ လေ့လာရလွယ်ကူရန် ရည်ရွယ်ထားသော Web နှင့် AI application programming language ဖြစ်သည်။ ယခု install လုပ်ရန်အတွက် recommended runtime သည် Python မလိုသော standalone Rust native binary ဖြစ်သည်။

> `zap` native runtime သည် `.zp` source files များကို Python မလိုဘဲ တိုက်ရိုက် execute လုပ်သည်။ Python implementation သည် language features စမ်းသပ်ရန် optional reference prototype သာဖြစ်သည်။

## 2. One-click installation

### Linux နှင့် macOS

Project folder ကို download/extract လုပ်ပြီး Terminal ဖွင့်ပါ။ ထို့နောက်—

```bash
cd zap
bash install.sh
```

Installer သည် standalone `zap` binary ကို user account ၏ `~/.local/bin` သို့ install လုပ်ပြီး `.bashrc` သို့မဟုတ် `.zshrc` ထဲတွင် PATH ကို update လုပ်ပါမည်။ Python၊ pip သို့မဟုတ် virtual environment မလိုပါ။ Terminal အသစ်တစ်ခု ပြန်ဖွင့်ပြီး installation ကို စစ်ပါ။

```bash
zap --version
```

### Windows

`install_windows.bat` ကို double-click လုပ်ပါ။ သို့မဟုတ် Command Prompt တွင်—

```bat
cd zap
install_windows.bat
```

ထို့နောက် Command Prompt အသစ်တစ်ခုဖွင့်ပြီး—

```bat
zap --version
```

### Release archive မှ direct installation

Python မရှိသောစက်တွင် GitHub Releases မှ သင့် operating system နှင့်ကိုက်ညီသော archive ကို download/extract လုပ်ပြီး binary ကို PATH ထဲသို့ ထည့်ပါ။ Source checkout မှ install လုပ်လျှင် `install.sh` သည် Rust toolchain ရှိပါက binary ကို build လုပ်ပေးနိုင်သည်။ End users များအတွက် prebuilt release archive ကို အသုံးပြုရန် အကြံပြုသည်။

## 3. CLI commands

| Command | ရည်ရွယ်ချက် |
|---|---|
| `zap --version` | Native Zap version ကို ပြသည် |
| `zap file.zp` | Zap source file ကို Python မလိုဘဲ execute လုပ်သည် |
| `zap --help` | Native CLI usage ကို ပြသည် |
| `zap fmt file.zp` | `.zp` source file ကို canonical whitespace ဖြင့် format လုပ်သည် |
| `zap check [dir]` | `zap.toml` နှင့် project entry file ကို validate လုပ်သည် |

Project အသစ်တစ်ခုကို မည်သည့် directory တွင်မဆို ဖန်တီးပြီး `.zp` file ကို run လုပ်နိုင်သည်။ Source file ကို format ပြင်ဆင်ရန် `zap fmt main.zp` ကို အသုံးပြုနိုင်သည်။ ဥပမာ—

```bash
mkdir hello-app
cd hello-app
printf 'say "Hello from Zap"\n' > main.zp
zap main.zp
```

Native CLI သည် source file path ကို တိုက်ရိုက်လက်ခံသောကြောင့် project တစ်ခုချင်းစီတွင် Python သို့မဟုတ် အခြား runtime dependency မလိုအပ်ပါ။

## 4. Project manifest နှင့် modules

Project root တွင် `zap.toml` ဖိုင်ထည့်နိုင်သည်။

```toml
[package]
name = "hello-app"
version = "0.1.0"
main = "main.zp"
```

စစ်ဆေးရန် `zap check` ကို အသုံးပြုပါ။ `use "math"` သို့မဟုတ် `use "math.zp"` module များကို main file directory၊ `modules/` နှင့် `lib/` အောက်တွင် ရှာဖွေပါသည်။ အသေးစိတ်ကို `PACKAGE.md` တွင် ဖတ်ရှုနိုင်သည်။

## 5. Zap language examples

### Output နှင့် variables

```zap
name = "Zap"
version = 2
say name
say version
```

### Arithmetic နှင့် conditional

```zap
score = 80
if score >= 50:
    say "Pass"
else:
    say "Try again"
```

### Function

```zap
fn greet(name):
    return "Hello, " + name

say greet("Developer")
```

### List နှင့် for loop

```zap
for item in ["web", "ai", "data"]:
    say item
```

### Map/object နှင့် indexing

```zap
map user = {"name": "Zap", "version": 2}
say user["name"]
say user["version"]
```

### While loop

```zap
count = 0
while count < 3:
    say count
    count = count + 1
```

## 5. JSON, Web နှင့် AI

JSON ပြောင်းလဲရန်—

```zap
map user = {"name": "Zap"}
text_data = json(user)
say text_data
say from_json(text_data)["name"]
```

HTTP GET foundation ကို အသုံးပြုရန်—

```zap
response = web.get "https://example.com"
say response["status"]
say response["text"]
```

Web response object တည်ဆောက်ရန်—

```zap
response = web.text "Hello from Zap"
json_response = web.json {"ok": true}
```

AI interface ကို အသုံးပြုရန်—

```zap
answer = ai.ask "Explain HTTP in one sentence"
say answer["text"]
```

လက်ရှိ `ai.ask` သည် placeholder provider ဖြစ်သည်။ API key၊ real model provider နှင့် production network integration များကို နောက် version တွင် ထည့်သွင်းရမည်။ API keys များကို Zap source code ထဲ မရေးသင့်ပါ။

## 6. Development နှင့် tests

Native source code ကို ပြင်ပြီး test suite ကို run လုပ်ရန်—

```bash
cargo test --manifest-path native/Cargo.toml
make native-test
```

Local binary package ထုတ်ရန်—

```bash
make package
```

Python reference prototype tests များသည် optional ဖြစ်ပြီး compatibility စမ်းသပ်မှုအတွက်သာ အသုံးပြုသည်။

## 7. Uninstall

Linux/macOS တွင် installer ထည့်ထားသော user-level `zap` binary ကို ဖယ်ရှားပြီး shell profile ထဲရှိ Zap PATH line ကို ဖယ်ရှားပါ။ Windows တွင် `%USERPROFILE%\\.zap\\bin\\zap.exe` ကို ဖယ်ရှားပြီး user PATH ထဲရှိ Zap entry ကို ဖယ်ရှားပါ။

Zap binary ကို ဖယ်ရှားခြင်းသည် Python package သို့မဟုတ် system Python ကို မထိခိုက်ပါ။

## 8. Current limitations

Zap 0.3 သည် production compiler မဟုတ်သေးသော early native runtime ဖြစ်ပါသည်။ Static type checking၊ remote package registry၊ lockfile၊ async runtime၊ full web server၊ streaming AI၊ security sandbox နှင့် native bytecode VM များကို ဆက်လက်တည်ဆောက်နေပါသည်။ မယုံကြည်ရသော source code ကို production တွင် တိုက်ရိုက် run မလုပ်သင့်ပါ။

## 9. Project files

| File | Purpose |
|---|---|
| `native/` | Rust native runtime နှင့် integration tests |
| `bin/zap` | Local native CLI binary |
| `zap.py` | Optional Python reference prototype |
| `setup.py` | Optional reference-package metadata |
| `install.sh` | Linux/macOS global user installer |
| `install_windows.bat` | Windows installer |
| `README.md` | Project overview |
| `USAGE.md` | Complete usage guide |
| `DESIGN.md` | Language design specification |
| `hello.zp` | Basic example |
| `advanced.zp` | Function/map/loop/JSON/AI example |
| `test_zap.py` | Automated tests |
| `LICENSE` | MIT License |

## References

ဤ guide သည် Zap project ၏ source code နှင့် local installation behavior ကို အခြေခံထားသော project documentation ဖြစ်သည်။
