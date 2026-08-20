# Zap v0.6.0

**Release status:** OOP foundation release

Zap v0.6.0 သည် v0.5.0 native foundation ကို ဆက်လက်တိုးချဲ့ပြီး standard library၊ optional type annotations၊ build validation၊ class-based object-oriented programming နှင့် Burmese learning documentation များကို ထည့်သွင်းထားသော release ဖြစ်သည်။

## အဓိက Feature များ

### Class-based OOP

- `class` ဖြင့် class ကြေညာခြင်း။
- `new("ClassName", arguments...)` ဖြင့် object ဖန်တီးခြင်း။
- `init(self, ...)` constructor အလိုအလျောက်ခေါ်ခြင်း။
- `self` ဖြင့် object properties ဖတ်ခြင်း၊ ပြင်ခြင်း။
- Method arguments နှင့် return values။
- `object.method()` နှင့် `object.property` dot syntax။
- `extends` ဖြင့် single inheritance။
- Child method override။
- Object value ကို list/map/JSON/type inspection နှင့် ပေါင်းစပ်အသုံးပြုခြင်း။

```zap
class User:
    fn init(self, name):
        self.name = name

    fn greet(self):
        return "Hello, " + self.name

class Admin extends User:
    fn role(self):
        return "admin"

let admin = new("Admin", "Root")
say admin.greet()
say admin.role()
```

### Standard Library နှင့် Tooling

- `now()`၊ `sleep()`
- `env()`၊ `has_env()`
- `path_join()`၊ `basename()`၊ `dirname()`၊ `exists()`
- `pow()`၊ `sqrt()`
- Optional variable annotations။
- `zap build [dir]` project validation။
- Recursive `zap test` workflow။

## Installation

GitHub [Releases](https://github.com/hidecard/zap/releases) မှ သင့် operating system နှင့် architecture ကိုက်ညီသော archive ကို download လုပ်ပြီး extract လုပ်ပါ။

Linux သို့မဟုတ် macOS တွင်—

```bash
tar -xzf zap-linux-x86_64.tar.gz
cd zap-0.6.0
bash install.sh
zap --version
```

Windows တွင် `zap-windows-x86_64.zip` ကို extract ပြီး `install_windows.bat` ကို run လုပ်ပါ။ Direct run လုပ်လိုပါက—

```bat
bin\zap.exe --version
bin\zap.exe main.zp
```

## Verification

Release မတင်မီ native test suite ကို run ပါ။

```bash
cd native
cargo test
```

v0.6.0 OOP regression tests တွင် class creation၊ constructor arguments၊ inherited methods၊ method override၊ property assignment နှင့် object state persistence တို့ကို စစ်ဆေးထားသည်။

## Documentation

- [`README.md`](../README.md) — Complete project documentation။
- [`LEARN_ZAP_MM.md`](LEARN_ZAP_MM.md) — Burmese beginner lessons၊ OOP Lesson 15 နှင့် exercises။
- [`SYNTAX_GUIDE.md`](SYNTAX_GUIDE.md) — Full syntax reference နှင့် OOP syntax။
- [`LANGUAGE_GUIDE.md`](LANGUAGE_GUIDE.md) — Language workflow နှင့် practical examples။
- [`USAGE.md`](USAGE.md) — Installation နှင့် CLI usage။
- [`DESIGN.md`](DESIGN.md) — Runtime/language design။
- [`ROADMAP_0.6.0.md`](ROADMAP_0.6.0.md) — Implemented နှင့် proposed features။

## မပါဝင်သေးသော Feature များ

`async/await`၊ channels၊ HTTP client၊ package registry၊ interfaces၊ abstract classes၊ private modifiers၊ generics နှင့် multiple inheritance များသည် v0.6.0 တွင် မပါဝင်သေးပါ။ ၎င်းတို့ကို နောက် release roadmap တွင် ဆက်လက်တည်ဆောက်မည်။

## Checksums

Release archive တစ်ခုစီအတွက် `.sha256` file ကို package script က အလိုအလျောက်ထုတ်ပေးသည်။ Download ပြီးနောက်—

```bash
sha256sum -c zap-linux-x86_64.tar.gz.sha256
```

Windows တွင်—

```powershell
Get-FileHash .\zap-windows-x86_64.zip -Algorithm SHA256
```

## Release Scope

ဤ release သည် OOP ကို အလွယ်တကူလေ့လာနိုင်သော foundation အဖြစ် ထည့်သွင်းထားခြင်းဖြစ်ပြီး production framework၊ package ecosystem နှင့် static type system အပြည့်အစုံအဖြစ် မသတ်မှတ်သေးပါ။ Runtime behavior များကို regression tests ဖြင့် ကာကွယ်ထားပြီး cross-platform release archives များကို package workflow ဖြင့် ထုတ်ပေးနိုင်သည်။
