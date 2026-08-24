# Zap Programming Language

![Zap Programming Language banner](assets/branding/zap-banner.png)

[![Zap CI](https://github.com/hidecard/zap/actions/workflows/ci.yml/badge.svg)](https://github.com/hidecard/zap/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/hidecard/zap?display_name=tag&sort=semver&color=2ea44f)](https://github.com/hidecard/zap/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20Windows%20%7C%20macOS-lightgrey.svg)](https://github.com/hidecard/zap/actions)
[![Source: .zp](https://img.shields.io/badge/source-.zp-8A2BE2.svg)](README.md)

**ဘာသာစကားရွေးချယ်ရန်:** [English README](README.md) · [မြန်မာ README](README_MM.md) · [မြန်မာ Documentation hub](docs/DOCUMENTATION_NAVIGATION_MM.md) · [Zap Language Guide](docs/LEARN_ZAP_MM.md)

> **Zap** သည် `.zp` source file၊ indentation-based block၊ optional type check၊ explicit module၊ structured error နှင့် standalone native runtime ပါဝင်သော ဖတ်ရလွယ်ကူသည့် general-purpose programming language ဖြစ်ပါသည်။

Zap သည် native executable အဖြစ် ဖြန့်ချိထားသောကြောင့် install ပြီးနောက် application runtime အတွက် Python၊ Node.js၊ Java သို့မဟုတ် Rust ထပ်မလိုပါ။ HTML၊ CSS၊ plain JavaScript သို့မဟုတ် React၊ Vue၊ Svelte စသည့် frontend tool များ၏ build output ကို project `public/` directory ထဲ ထည့်ပြီး Zap ဖြင့် serve လုပ်နိုင်ပါသည်။

## လက်ရှိ release

| အချက် | အခြေအနေ |
|---|---|
| လက်ရှိ release line | `v2.11.0` |
| Source file | `.zp`၊ အများအားဖြင့် `main.zp` |
| Project manifest | `zap.toml` |
| Lockfile | `zap.lock` |
| Runtime | Standalone native executable |
| Platforms | Linux x86_64၊ Windows x86_64၊ macOS ARM64 |
| License | MIT |
| Repository | [github.com/hidecard/zap](https://github.com/hidecard/zap) |
| Releases | [GitHub Releases](https://github.com/hidecard/zap/releases) |

## Install လုပ်ခြင်း

သင့် operating system နှင့် CPU architecture ကိုက်ညီသော archive ကို [v2.11.0 release page](https://github.com/hidecard/zap/releases/tag/v2.11.0) မှ download လုပ်ပြီး checksum/signature verify လုပ်ကာ extract လုပ်ပါ။

### Linux

```bash
tar -xzf zap-2.11.0-linux-x86_64.tar.gz
cd zap
bash install.sh
zap --version
```

### macOS ARM64

```bash
tar -xzf zap-2.11.0-macos-arm64.tar.gz
cd zap
chmod +x install.sh
./install.sh
zap --version
```

### Windows

မျှော်မှန်းထားသော archive သည် `zap-2.11.0-windows-x86_64.zip` ဖြစ်ပါသည်။

```bat
cd C:\Zap
install_windows.bat
zap.exe --version
```

Administrator access မရှိပါက executable ကို user-owned directory ထဲတွင်ထားပြီး ထို directory ကို `PATH` ထဲထည့်နိုင်ပါသည်။ Platform-specific installation အပြည့်အစုံအတွက် [မြန်မာ Language Guide](docs/LEARN_ZAP_MM.md) ကို ဖတ်ပါ။

## Command တစ်ကြောင်းဖြင့် project ဆောက်ခြင်း

Zap တွင် Django-style `startapp` command နှင့် hidden app registry မရှိပါ။ Project တစ်ခုလုံးကို command တစ်ကြောင်းဖြင့် စတင်ဖန်တီးပြီး နောက်ပိုင်း file/module များကို User ကိုယ်တိုင် manage လုပ်နိုင်ပါသည်။

```bash
zap new my_app
cd my_app
zap check
zap build --locked
zap test tests
zap dev
```

Generator က အောက်ပါ structure ကို ထုတ်ပေးပါသည်။

```text
my_app/
├── zap.toml
├── zap.lock
├── main.zp
├── web.zp
├── server.zp
├── models/
├── functions/
├── ui/
├── routes/
├── middleware/
├── migrations/
├── admin/
├── public/
└── tests/
```

ဤ directory များသည် user-owned ordinary files ဖြစ်ပါသည်။ `models/` တွင် data shape/validation၊ `functions/` တွင် business logic၊ `ui/` တွင် browser UI metadata၊ `routes/` တွင် HTTP route၊ `middleware/` တွင် request/response policy၊ `migrations/` တွင် schema change၊ `admin/` တွင် optional administration registration၊ `public/` တွင် browser assets နှင့် `tests/` တွင် executable checks များကို ထားပါ။

## ပထမဆုံး Zap program

`hello.zp` ဖန်တီးပါ။

```zap
fn greet(name: text) -> text:
    return "Hello, " + name

say greet("Zap")
```

```bash
zap hello.zp
# သို့မဟုတ်
zap run hello.zp
```

## Language Guide နှင့် Documentation

| လိုအပ်ချက် | စတင်ဖတ်ရန် |
|---|---|
| Install မှ Advanced အထိ လေ့လာရန် | [မြန်မာ Language Guide](docs/LEARN_ZAP_MM.md) · [English Language Guide](docs/LEARN_ZAP_EN.md) |
| Searchable syntax reference | [မြန်မာ syntax](docs/SYNTAX_GUIDE.md) · [English syntax](docs/SYNTAX_GUIDE_EN.md) |
| Normative language behavior | [မြန်မာ specification](docs/LANGUAGE_SPEC_MM.md) · [English specification](docs/LANGUAGE_SPEC_EN.md) |
| Standard library | [မြန်မာ index](docs/STDLIB_INDEX_MM.md) · [English index](docs/STDLIB_INDEX_EN.md) |
| Package/lockfile workflow | [မြန်မာ package guide](docs/PACKAGE.md) · [English package guide](docs/PACKAGE_EN.md) |
| Web နှင့် frontend | [မြန်မာ Web guide](docs/ZAP_WEB_NATIVE_MM.md) · [မြန်မာ frontend guide](docs/FRONTEND_INTEGRATION_MM.md) |
| Runtime၊ memory၊ async | [Runtime state](docs/RUNTIME_STATE_MM.md) · [Memory contract](docs/MEMORY_BUDGET_OBJECT_STORE_MM.md) · [Async boundary](docs/ASYNC_BOUNDARIES_MM.md) |
| Host နှင့် deployment | [Host guide](docs/ZAP_HOST_MM.md) · [Deployment guide](docs/DEPLOYMENT_MM.md) |
| English navigation | [Documentation hub](docs/DOCUMENTATION_NAVIGATION_EN.md) |

## CLI အခြေခံ commands

```bash
zap file.zp                 # source file run
zap new my_app               # user-managed Web project ဖန်တီး
zap check .                  # Zap project directory ကို validate လုပ်ရန်
zap check --json .           # structured diagnostic
zap build --locked .         # reproducible build input စစ်
zap test tests               # Zap test run
zap fmt main.zp              # source format
zap lint main.zp             # style issue ပြ
zap lock                    # canonical lock data ထုတ်
zap install                 # locked dependency validate
zap update                  # manifest ပြောင်းပြီး lock ပြန်ထုတ်
zap web check               # Web configuration validate
zap dev                     # bounded development server စတင်
zap --help                  # command list ပြ
```

## Frontend integration

Plain HTML၊ CSS နှင့် JavaScript သည် production တွင် JavaScript runtime မလိုဘဲ အလုပ်လုပ်နိုင်ပါသည်။

```html
<script type="module" src="/assets/app.js"></script>
```

React၊ Vue၊ Svelte သို့မဟုတ် အခြား frontend project ကို သီးခြား build လုပ်ပြီး output ကို `public/` ထဲ copy လုပ်နိုင်ပါသည်။ Deployment အချိန်တွင် Zap သည် ထွက်လာသော file များကို serve လုပ်ပြီး npm/Node.js မလိုပါ။

## Implemented နှင့် deferred scope

လက်ရှိ stable direction တွင် `.zp` language core၊ native CLI၊ manifest/lockfile၊ typed check၊ modules၊ classes၊ Result/Option၊ JSON၊ tests၊ formatter/linter၊ structured diagnostics၊ LSP foundation၊ user-managed Web scaffold၊ bounded native Web serving နှင့် SQLite-first migration contract များ ပါဝင်ပါသည်။

Complete ORM၊ provider-neutral production migration platform၊ user-defined trait syntax၊ production async I/O reactor၊ cross-file semantic rename၊ template compiler နှင့် hidden app registry တို့ကို complete ဟု မဆိုထားသေးပါ။ Status များကို [language specification](docs/LANGUAGE_SPEC_MM.md)၊ contract၊ test နှင့် release note များတွင် ကြည့်ပါ။

## Development

Zap runtime ကို Rust ဖြင့် တည်ဆောက်ထားပါသည်။ Source မှ build လုပ်ရန် `rust-toolchain.toml` တွင် သတ်မှတ်ထားသော toolchain ကို install ပြီး—

```bash
cargo test --manifest-path native/Cargo.toml --all-targets
cargo build --release --manifest-path native/Cargo.toml
```

Documentation၊ Web scaffold၊ release-version၊ VS Code asset နှင့် LSP parity validation များကို [မြန်မာ documentation hub](docs/DOCUMENTATION_NAVIGATION_MM.md) တွင် ဖော်ပြထားသည့်အတိုင်း run ပါ။

## Release provenance

လက်ရှိ source baseline သည် v2.11.0 ဖြစ်ပါသည်။ v2.3.0၊ v2.2.7 နှင့် အစောပိုင်း release record များကို [GitHub Releases](https://github.com/hidecard/zap/releases) နှင့် bilingual `CHANGELOG` file များတွင် ဆက်လက်ကြည့်ရှုနိုင်ပါသည်။ Release artifact များကို version consistency၊ native test၊ cross-platform build၊ security check၊ documentation check နှင့် installer verification များ pass ပြီးမှသာ publish လုပ်ပါသည်။

## License

Zap ကို [MIT License](LICENSE) အောက်တွင် ဖြန့်ချိထားပါသည်။
