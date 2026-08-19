# Zap 0.7.1 — Complete Usage Guide

စတင်လေ့လာသူများအတွက် lesson-based Burmese course ကို [`LEARN_ZAP_MM.md`](LEARN_ZAP_MM.md) တွင် ဖတ်ရှုနိုင်သည်။

## 1. Zap ဆိုတာဘာလဲ

Zap သည် လေ့လာရလွယ်ကူသော syntax၊ native execution နှင့် Web/AI application များအတွက် တဖြည်းဖြည်းချဲ့ထွင်နိုင်သော general-purpose programming language ဖြစ်သည်။ ယခု install လုပ်ရန်အတွက် recommended runtime သည် standalone native binary ဖြစ်သည်။

> `zap` native runtime သည် `.zp` source files များကို အပို runtime မလိုဘဲ တိုက်ရိုက် execute လုပ်သည်။ ပုံမှန်အသုံးပြုသူများသည် release binary ကို တိုက်ရိုက်အသုံးပြုနိုင်သည်။

## 2. One-click installation

### Linux နှင့် macOS

Project folder ကို download/extract လုပ်ပြီး Terminal ဖွင့်ပါ။ ထို့နောက်—

```bash
cd zap
bash install.sh
```

Installer သည် standalone `zap` binary ကို user account ၏ `~/.local/bin` သို့ install လုပ်ပြီး `.bashrc` သို့မဟုတ် `.zshrc` ထဲတွင် PATH ကို update လုပ်ပါမည်။ အပို runtime သို့မဟုတ် package manager မလိုပါ။ Terminal အသစ်တစ်ခု ပြန်ဖွင့်ပြီး installation ကို စစ်ပါ။

```bash
zap --version
```

### Windows

Release archive ကို extract လုပ်ပြီး `bin\zap.exe` ရှိကြောင်း စစ်ပါ။ `.exe` ကို installer မလိုဘဲ တိုက်ရိုက် run နိုင်သည်။

```bat
cd zap-0.8.0
bin\zap.exe --version
bin\zap.exe main.zp
```

မည်သည့် folder မှာမဆို `zap` command သုံးလိုပါက `install_windows.bat` ကို **Command Prompt မှ Run as administrator မလိုဘဲ double-click သို့မဟုတ် command line ဖြင့်** run လုပ်ပါ။ Installer သည် `%USERPROFILE%\.zap\bin\zap.exe` သို့ copy လုပ်ပြီး user-level PATH ကို update လုပ်သည်။

```bat
cd zap-0.8.0
install_windows.bat
```

PATH update သည် လက်ရှိ Command Prompt အဟောင်းတွင် မပေါ်သေးပါက Command Prompt အသစ်ဖွင့်ပါ။ အမြဲတမ်း direct path ဖြင့်လည်း run နိုင်သည်။

```bat
"%USERPROFILE%\.zap\bin\zap.exe" --version
"%USERPROFILE%\.zap\bin\zap.exe" main.zp
```

### Release archive မှ direct installation

GitHub Releases မှ သင့် operating system နှင့်ကိုက်ညီသော archive ကို download/extract လုပ်ပြီး binary ကို PATH ထဲသို့ ထည့်ပါ။ Source checkout မှ install လုပ်လျှင် `install.sh` သည် Rust toolchain ရှိပါက binary ကို build လုပ်ပေးနိုင်သည်။ End users များအတွက် prebuilt release archive ကို အသုံးပြုရန် အကြံပြုသည်။

## 3. CLI commands

| Command | ရည်ရွယ်ချက် |
|---|---|
| `zap --version` | Native Zap version ကို ပြသည် |
| `zap file.zp` | Zap source file ကို standalone runtime ဖြင့် execute လုပ်သည် |
| `zap run file.zp` | Source file ကို explicit run command ဖြင့် execute လုပ်သည် |
| `zap --help` | Native CLI usage ကို ပြသည် |
| `zap fmt file.zp` | `.zp` source file ကို canonical whitespace ဖြင့် format လုပ်သည် |
| `zap check [dir]` | `zap.toml` နှင့် project entry file ကို validate လုပ်သည် |
| `zap test [dir]` | Directory နှင့် subdirectories များအောက်ရှိ `*_test.zp` files အားလုံးကို run လုပ်သည် |
| `zap init <dir>` | Zap project အသစ် scaffold လုပ်သည် |
| `zap build [dir]` | Build-ready project validation ပြုလုပ်သည် |
| `zap lint <file.zp>` | Tabs၊ trailing whitespace နှင့် long lines စစ်သည် |
| `zap check --json [dir]` | CI/editor အတွက် JSON project diagnostics ထုတ်သည် |

Project အသစ်တစ်ခုကို မည်သည့် directory တွင်မဆို ဖန်တီးပြီး `.zp` file ကို run လုပ်နိုင်သည်။ Source file ကို format ပြင်ဆင်ရန် `zap fmt main.zp` ကို အသုံးပြုနိုင်သည်။ ဥပမာ—

```bash
mkdir hello-app
cd hello-app
printf 'say "Hello from Zap"\n' > main.zp
zap run main.zp
```

Native CLI သည် source file path ကို တိုက်ရိုက်လက်ခံသောကြောင့် project တစ်ခုချင်းစီတွင် အခြား runtime dependency မလိုအပ်ပါ။

### Zap test runner

Project-level tests များကို `tests/` directory သို့မဟုတ် ၎င်းအောက်ရှိ subdirectories များတွင် `*_test.zp` naming ဖြင့် သိမ်းပါ။ `zap init` သည် `tests/smoke_test.zp` starter test ကို အလိုအလျောက် ဖန်တီးပေးသည်။ ထို့နောက်—

```bash
zap test
zap test path/to/tests
```

ဟု run လုပ်နိုင်သည်။ Test runner သည် test files များကို path အလိုက် sort လုပ်ပြီး run သည်။ Test file တစ်ခုအတွင်း `assert(condition, message)` ကို အသုံးပြုပြီး failure ဖြစ်ပါက command သည် non-zero exit code ဖြင့် ရပ်တန့်သည်။

## 4. Project manifest နှင့် modules

Project root တွင် `zap.toml` ဖိုင်ထည့်နိုင်သည်။

```toml
[package]
name = "hello-app"
version = "0.1.0"
main = "main.zp"
```

စစ်ဆေးရန် `zap check` ကို အသုံးပြုပါ။ CI သို့မဟုတ် editor integration အတွက် `zap check --json .` ကို အသုံးပြုပါ။ Source style စစ်ရန် `zap lint main.zp` ကို အသုံးပြုပါ။ `use "math"` သို့မဟုတ် `use "math.zp"` module များကို main file directory၊ `modules/` နှင့် `lib/` အောက်တွင် ရှာဖွေပါသည်။ အသေးစိတ်ကို `PACKAGE.md` တွင် ဖတ်ရှုနိုင်သည်။

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

## 6. JSON, Web နှင့် AI

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

## 7. Standard library helpers

```zap
let scores = [8, 3, 10, 5]
say sum(scores)
say join(sort(scores), ",")
write_lines("notes.txt", ["one", "two"])
say join(read_lines("notes.txt"), "|")
```

`get(map, key, default)` သည် မရှိသော map key အတွက် default value ပြန်ပေးသည်။ `read_lines` နှင့် `write_lines` သည် text file ကို line list အဖြစ် ကိုင်တွယ်သည်။

## 8. Development နှင့် tests

Native source code ကို ပြင်ပြီး test suite ကို run လုပ်ရန်—

```bash
cargo test --manifest-path native/Cargo.toml
make native-test
```

Local binary package ထုတ်ရန်—

```bash
make package
```

Language behavior အတွက် native integration tests များကို `native/tests/` အောက်တွင် ထိန်းသိမ်းထားပြီး `cargo test --manifest-path native/Cargo.toml` ဖြင့် run နိုင်သည်။

## 9. Uninstall

Linux/macOS တွင် installer ထည့်ထားသော user-level `zap` binary ကို ဖယ်ရှားပြီး shell profile ထဲရှိ Zap PATH line ကို ဖယ်ရှားပါ။ Windows တွင် `%USERPROFILE%\\.zap\\bin\\zap.exe` ကို ဖယ်ရှားပြီး user PATH ထဲရှိ Zap entry ကို ဖယ်ရှားပါ။

Zap binary ကို ဖယ်ရှားခြင်းသည် system ပေါ်ရှိ အခြား software များကို မထိခိုက်ပါ။

## 10. Current limitations

Zap 0.7.1 သည် production compiler မဟုတ်သေးသော early native runtime ဖြစ်ပါသည်။ Full static type checking၊ remote package registry၊ lockfile၊ async runtime၊ full web server၊ streaming AI၊ security sandbox နှင့် native bytecode VM များကို ဆက်လက်တည်ဆောက်နေပါသည်။ v0.7.1 တွင် `is_empty`၊ `sum`၊ `reverse`၊ `sort`၊ `get`၊ `read_lines` နှင့် `write_lines` ပါဝင်ပြီး OOP class validation၊ inherited constructors နှင့် method override behavior ကို audit ပြင်ဆင်ထားသည်။ မယုံကြည်ရသော source code ကို production တွင် တိုက်ရိုက် run မလုပ်သင့်ပါ။

## 11. Project files

| File | Purpose |
|---|---|
| `native/` | Rust native runtime နှင့် integration tests |
| `bin/zap` | Local native CLI binary |

| `install.sh` | Linux/macOS global user installer |
| `install_windows.bat` | Windows installer |
| `../README.md` | Project overview |
| `USAGE.md` | Complete usage guide |
| `DESIGN.md` | Language design specification |
| `../examples/hello.zp` | Basic example |
| `../examples/tasks.zp` | Function/map/loop/assert example |
| `../examples/data.zp` | JSON and file I/O example |
| `native/tests/` | Native runtime integration tests |
| `tests/` | User-facing `*_test.zp` test files |
| `LICENSE` | MIT License |

## References

ဤ guide သည် Zap project ၏ source code နှင့် local installation behavior ကို အခြေခံထားသော project documentation ဖြစ်သည်။
