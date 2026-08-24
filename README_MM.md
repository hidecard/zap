# Zap Programming Language

![Zap Programming Language banner](assets/branding/zap-banner.png)

[![Zap CI](https://github.com/hidecard/zap/actions/workflows/ci.yml/badge.svg)](https://github.com/hidecard/zap/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/hidecard/zap?display_name=tag&sort=semver&color=2ea44f)](https://github.com/hidecard/zap/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20Windows%20%7C%20macOS-lightgrey.svg)](https://github.com/hidecard/zap/actions)
[![Source: .zp](https://img.shields.io/badge/source-.zp-8A2BE2.svg)](README.md)

**ဘာသာစကားရွေးချယ်ရန်:** [English README](README.md) · [မြန်မာ README](README_MM.md) · [မြန်မာ Documentation hub](docs/DOCUMENTATION_NAVIGATION_MM.md) · [မြန်မာ Language Guide](docs/LEARN_ZAP_MM.md)

> **Zap သည် ဖတ်ရလွယ်ကူပြီး native-first ဖြစ်သော general-purpose programming language တစ်ခုဖြစ်သည်။** Structured type၊ diagnostics၊ explicit project workflow နှင့် cross-platform standalone executable တို့ကို ရိုးရှင်းသော developer experience တစ်ခုထဲတွင် ပေါင်းစပ်ထားသည်။

Zap runtime ကို Rust ဖြင့် တည်ဆောက်ထားပြီး standalone native executable အဖြစ် ဖြန့်ချိပါသည်။ လက်ရှိ release line `v2.9.0` တွင် `.zp` language core၊ project manifest/lockfile၊ check/build/test/fmt/lint CLI tooling၊ user-managed Web scaffold နှင့် bounded native development server တို့ ပါဝင်ပါသည်။ အောက်တွင်ဖော်ပြထားသော advanced production feature များကို complete ဟု မဆိုထားသေးပါ။

## Zap ကို ဘာကြောင့်သုံးမလဲ

Zap သည် readable syntax၊ project file များကို developer ကိုယ်တိုင် စီမံနိုင်မှုနှင့် compact native workflow ကို အလေးထားသူများအတွက် ရည်ရွယ်ပါသည်။ သင်ယူရန်၊ experiment လုပ်ရန်၊ focused CLI tool သို့မဟုတ် service အသေးစားများရေးရန်နှင့် ရိုးရှင်းသော project model ကို စမ်းသပ်ရန် သင့်တော်ပါသည်။

Zap သည် Rust၊ Go၊ Python နှင့် TypeScript တို့ကို အစားထိုးရန် အမြဲတမ်းအကောင်းဆုံးရွေးချယ်မှု မဟုတ်ပါ။ Systems work အတွက် Rust၊ cloud service team ကြီးများအတွက် Go၊ automation/data အတွက် Python နှင့် browser-first product အတွက် TypeScript တို့၏ ecosystem များသည် ပိုမိုရင့်ကျက်ပါသည်။ Zap ၏ အခွင့်အလမ်းမှာ beginner-friendly native-first workflow ဖြင့် အသုံးဝင်မှုကို သက်သေပြနိုင်ခြင်း ဖြစ်ပါသည်။

| Zap ၏ အားသာချက် | အဓိပ္ပာယ် |
|---|---|
| **Native-first ဖြန့်ချိမှု** | Application runtime အဖြစ် Python၊ Node.js၊ Java သို့မဟုတ် Rust ကို ထပ်မံ install လုပ်စရာမလိုသော standalone executable workflow ကို ရည်ရွယ်သည်။ |
| **ဖတ်ရလွယ်သော syntax** | Indentation-based block၊ optional type annotation၊ explicit module နှင့် structured diagnostics များကို အသုံးပြုသည်။ |
| **Explicit project structure** | `routes/`၊ `models/`၊ `functions/`၊ `tests/` နှင့် `public/` တို့သည် hidden app registry မဟုတ်ဘဲ user-managed directory များဖြစ်သည်။ |
| **CLI workflow တစ်ခုတည်း** | Check၊ build၊ test၊ format၊ lint၊ lock နှင့် Web validation များကို `zap` command ဖြင့် လုပ်ဆောင်နိုင်သည်။ |

## လက်ရှိအခြေအနေ

| အချက် | အခြေအနေ |
|---|---|
| လက်ရှိ release line | `v2.9.0` |
| Source file | `.zp`၊ အများအားဖြင့် `main.zp` |
| Project manifest | `zap.toml` |
| Lockfile | `zap.lock` |
| Runtime | Standalone native executable |
| Platforms | Linux x86_64၊ Windows x86_64၊ macOS ARM64 |
| License | MIT |
| Repository | [github.com/hidecard/zap](https://github.com/hidecard/zap) |
| Releases | [GitHub Releases](https://github.com/hidecard/zap/releases) |

> **Security boundary:** `ZAP_UNTRUSTED=1` သည် capability denial နှင့် request policy အတွက် defensive runtime mode ဖြစ်ပြီး kernel-enforced sandbox မဟုတ်ပါ။ Untrusted source၊ downloaded plugin သို့မဟုတ် multi-tenant workload များအတွက် OS-level isolation၊ least-privilege filesystem permission၊ network egress control၊ resource quota နှင့် audit logging များ ထပ်မံအသုံးပြုပါ။

## Quickstart

### ၁။ Zap install လုပ်ခြင်း

သင့် operating system နှင့် CPU architecture ကိုက်ညီသော archive ကို [`v2.9.0` release page](https://github.com/hidecard/zap/releases/tag/v2.9.0) မှ download လုပ်ပါ။ Extract မလုပ်မီ checksum နှင့် signature ကို verify လုပ်ပြီး `zap` executable ကို `PATH` ထဲသို့ ထည့်ပါ။

#### Linux x86_64

```bash
tar -xzf zap-2.9.0-linux-x86_64.tar.gz
cd zap
bash install.sh
zap --version
```

#### macOS ARM64

```bash
tar -xzf zap-2.9.0-macos-arm64.tar.gz
cd zap
chmod +x install.sh
./install.sh
zap --version
```

#### Windows x86_64

`zap-2.9.0-windows-x86_64.zip` archive ကို အသုံးပြုပါ။

```bat
cd C:\Zap
install_windows.bat
zap.exe --version
```

Administrator access မရှိပါက executable ကို user-owned directory ထဲတွင်ထားပြီး ထို directory ကို `PATH` ထဲသို့ ထည့်နိုင်ပါသည်။ အသေးစိတ် verification အတွက် [မြန်မာ Language Guide](docs/LEARN_ZAP_MM.md) ကို ဖတ်ပါ။

### ၂။ Project ဖန်တီး၊ စစ်၊ build၊ test နှင့် run လုပ်ခြင်း

```bash
zap new hello_zap
cd hello_zap
zap check .
zap build --locked .
zap test tests
zap run main.zp
```

`zap dev` သည် local development အတွက် bounded development server ကို စတင်ပါသည်။ Production hosting platform မဟုတ်ပါ။

Generator မှ ထွက်လာသော project သည် developer ကိုယ်တိုင် စီမံနိုင်သော ordinary structure ဖြစ်ပါသည်။

```text
hello_zap/
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

`models/` တွင် data shape နှင့် validation၊ `functions/` တွင် business logic၊ `ui/` တွင် browser UI metadata၊ `routes/` တွင် HTTP route၊ `middleware/` တွင် request/response policy၊ `migrations/` တွင် schema change၊ `admin/` တွင် optional administration registration၊ `public/` တွင် browser asset နှင့် `tests/` တွင် executable check များကို ထားပါ။

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

မျှော်မှန်း output သည် အောက်ပါအတိုင်း ဖြစ်ပါသည်။

```text
Hello, Zap
```

## Language အကျဉ်းချုပ်

```zap
let scores: list<number> = [80, 45, 90]

fn passed(score: number) -> bool:
    return score >= 50

for score in scores:
    if passed(score):
        say "passed: " + str(score)
```

Language တွင် text၊ number၊ boolean၊ list၊ map၊ object၊ function၊ class၊ inheritance၊ optional annotation၊ closure၊ explicit module၊ JSON၊ `Result`/`Option`၊ default/named argument၊ bounded asynchronous task နှင့် deterministic diagnostic များ ပါဝင်ပါသည်။

## CLI အခြေခံ commands

| Command | ရည်ရွယ်ချက် | ပုံမှန်အသုံးပြုချိန် |
|---|---|---|
| `zap new my_app` | User-managed project scaffold ဖန်တီးရန် | Project စတင်ချိန် |
| `zap check .` | Project validation လုပ်ရန် | Commit မတင်မီ |
| `zap check --json .` | Structured diagnostics ထုတ်ရန် | CI နှင့် editor tooling |
| `zap build --locked .` | Lockfile အတိုင်း reproducible build စစ်ရန် | Release မတိုင်မီ |
| `zap test tests` | Test suite run ရန် | Code ပြောင်းပြီးတိုင်း |
| `zap fmt main.zp` | Source format လုပ်ရန် | Review မတိုင်မီ |
| `zap lint main.zp` | Style issue ရှာရန် | Commit မတင်မီ |
| `zap lock` | Canonical lock data ထုတ်ရန် | Manifest ပြောင်းပြီးနောက် |
| `zap install` | Locked dependency validate လုပ်ရန် | Clean checkout တွင် |
| `zap update` | Manifest ပြောင်းပြီး lock data ပြန်ထုတ်ရန် | Dependency update လုပ်ချိန် |
| `zap web check` | Web configuration နှင့် route conflict စစ်ရန် | Web project serve မလုပ်မီ |
| `zap web routes --json` | Route table ကို machine-readable ကြည့်ရန် | CI/integration တွင် |
| `zap dev` | Bounded development server စရန် | Local development သာ |
| `zap --help` | Command အားလုံးကြည့်ရန် | CLI လေ့လာချိန် |

## Web development model

Zap သည် plain HTML၊ CSS၊ JavaScript သို့မဟုတ် သီးခြား build လုပ်ထားသော React၊ Vue၊ Svelte frontend output ကို `public/` မှ serve လုပ်နိုင်ပါသည်။

```html
<script type="module" src="/assets/app.js"></script>
```

Frontend source ကို သက်ဆိုင်ရာ toolchain ဖြင့် သီးခြား build လုပ်ပြီး output ကို `public/` ထဲသို့ copy လုပ်နိုင်ပါသည်။ Deployment runtime တွင် npm သို့မဟုတ် Node.js မလိုသော်လည်း frontend source build အတွက် လိုနိုင်ပါသည်။ `routes/` တွင် server-side route declaration နှင့် `public/` တွင် browser asset များကို ထားပါ။

ဤ Web foundation တွင် complete ORM၊ built-in authentication၊ provider-neutral database abstraction၊ production async I/O reactor၊ WebSocket၊ streaming upload၊ SSR/template compiler သို့မဟုတ် built-in admin UI များ ပါဝင်သည်ဟု မဆိုထားသေးပါ။ အသေးစိတ်ကို [မြန်မာ Zap Web guide](docs/ZAP_WEB_NATIVE_MM.md) တွင် ကြည့်ပါ။

## Implemented နှင့် မပြီးသေးသော scope

| အပိုင်း | အခြေအနေ | နယ်နိမိတ် |
|---|---|---|
| `.zp` language core | Implemented direction | Core value၊ function၊ class၊ module၊ JSON နှင့် diagnostics |
| Native runtime | Implemented direction | Standalone executable နှင့် release platform များ |
| CLI tooling | Implemented direction | Check၊ build၊ test၊ format၊ lint၊ lock နှင့် Web check |
| Web scaffold | Implemented foundation | User-managed directory၊ route validation နှင့် static frontend output |
| Package/lock workflow | Available | Lockfile-based reproducibility |
| LSP/editor support | Foundation | Coverage နှင့် parity ကို documentation/fixture ဖြင့် စစ်ရန်လိုသည် |
| ORM | Complete ဟု မဆိုထားသေး | Production-ready database abstraction မဟုတ်သေး |
| Production migration | Complete ဟု မဆိုထားသေး | SQLite-first contract သည် provider-neutral platform မဟုတ်သေး |
| Advanced async I/O | Complete ဟု မဆိုထားသေး | Bounded async task သည် production I/O reactor မဟုတ်သေး |
| Debugger/profiler | Complete ဟု မဆိုထားသေး | Tooling roadmap item အဖြစ်သာ သတ်မှတ်ပါ |
| WebSocket/streaming upload | Complete ဟု မဆိုထားသေး | လက်ရှိ Web framework limitation ဖြစ်သည် |
| Hidden app registry | ရည်ရွယ်ချက်ရှိရှိ မထည့်ထား | Project များကို explicit/user-managed အဖြစ်ထားသည် |

## Documentation

| လိုအပ်ချက် | စတင်ဖတ်ရန် |
|---|---|
| Install မှ advanced အထိ လေ့လာရန် | [မြန်မာ Language Guide](docs/LEARN_ZAP_MM.md) · [English Language Guide](docs/LEARN_ZAP_EN.md) |
| Syntax reference | [မြန်မာ syntax](docs/SYNTAX_GUIDE.md) · [English syntax](docs/SYNTAX_GUIDE_EN.md) |
| Language behavior | [မြန်မာ specification](docs/LANGUAGE_SPEC_MM.md) · [English specification](docs/LANGUAGE_SPEC_EN.md) |
| Standard library | [မြန်မာ index](docs/STDLIB_INDEX_MM.md) · [English index](docs/STDLIB_INDEX_EN.md) |
| Package/lockfile | [မြန်မာ package guide](docs/PACKAGE.md) · [English package guide](docs/PACKAGE_EN.md) |
| Web နှင့် frontend | [မြန်မာ Web guide](docs/ZAP_WEB_NATIVE_MM.md) · [မြန်မာ frontend guide](docs/FRONTEND_INTEGRATION_MM.md) |
| Runtime၊ memory၊ async | [Runtime state](docs/RUNTIME_STATE_MM.md) · [Memory contract](docs/MEMORY_BUDGET_OBJECT_STORE_MM.md) · [Async boundary](docs/ASYNC_BOUNDARIES_MM.md) |
| Host နှင့် deployment | [Host guide](docs/ZAP_HOST_MM.md) · [Deployment guide](docs/DEPLOYMENT_MM.md) |
| Documentation navigation | [မြန်မာ hub](docs/DOCUMENTATION_NAVIGATION_MM.md) · [English hub](docs/DOCUMENTATION_NAVIGATION_EN.md) |
| Release history | [မြန်မာ changelog](CHANGELOG_MM.md) · [English changelog](CHANGELOG_EN.md) |

## Source မှ build လုပ်ခြင်း

Zap runtime ကို Rust ဖြင့် တည်ဆောက်ထားပါသည်။ [`rust-toolchain.toml`](rust-toolchain.toml) တွင် သတ်မှတ်ထားသော pinned toolchain ကို install ပြီး အောက်ပါ command များ run ပါ။

```bash
cargo test --manifest-path native/Cargo.toml --all-targets
cargo build --release --manifest-path native/Cargo.toml
```

Pull Request မတင်မီ မြန်မာ documentation hub တွင် ဖော်ပြထားသော documentation၊ Web scaffold၊ release-version၊ VS Code asset နှင့် LSP parity validators များကိုလည်း run ပါ။

## Contribution

Contribution ပြုလုပ်လိုပါက [CONTRIBUTING.md](CONTRIBUTING.md)၊ သက်ဆိုင်ရာ specification/contract document နှင့် release note များကို အရင်ဖတ်ပါ။ ပြောင်းလဲမှုကို သေးငယ်ပြီး ရည်ရွယ်ချက်တိကျအောင်ထားပါ၊ regression test ထည့်ပါ၊ behavior ပြောင်းလဲပါက English နှင့် မြန်မာ documentation နှစ်ခုလုံး update လုပ်ပါ၊ native/documentation/Web/release validation များ run ပါ။

Documentation correction၊ executable example၊ conformance fixture နှင့် focused regression test များသည် ပထမဆုံး contribution အတွက် သင့်တော်ပါသည်။ Syntax၊ runtime၊ package သို့မဟုတ် Web framework အပြောင်းအလဲကြီးများအတွက် implementation မစမီ issue ဖွင့်ပြီး contract ကို ဆွေးနွေးပါ။

## Bilingual documentation policy

`README.md` သည် release-facing canonical README ဖြစ်ပြီး `README_MM.md` သည် မြန်မာ companion ဖြစ်ပါသည်။ Command၊ feature status၊ security boundary သို့မဟုတ် supported platform ပြောင်းလဲပါက README နှစ်ခုကို တစ်ပြိုင်နက် update လုပ်ပြီး release facts နှင့် limitations တူညီကြောင်း စစ်ဆေးပါ။ ဘာသာစကားအလိုက် ရှင်းလင်းပုံကွာနိုင်သော်လည်း release facts မကွာရပါ။

## Security နှင့် responsible use

Restricted operation အတွက် capability restriction နှင့် bounded request/process policy များ ရှိသော်လည်း ၎င်းတို့သည် kernel-enforced sandbox မဟုတ်ပါ။ Untrusted Zap code execute လုပ်မည့် host များသည် worker isolation၊ filesystem/environment restriction၊ network egress filtering၊ CPU/memory/process quota၊ process-group cleanup နှင့် audit event များ ထည့်သွင်းပါ။

Security report များအတွက် public issue မဖွင့်ဘဲ [SECURITY.md](SECURITY.md) တွင် ဖော်ပြထားသော process ကို လိုက်နာပါ။

## Release provenance

လက်ရှိ source baseline သည် `v2.9.0` ဖြစ်ပါသည်။ Version consistency၊ native test၊ cross-platform build၊ security check၊ documentation check နှင့် installer verification များ pass ပြီးမှသာ release artifact များ publish လုပ်ပါသည်။ အစောပိုင်း release record များကို [GitHub Releases](https://github.com/hidecard/zap/releases) နှင့် changelog file များတွင် ကြည့်နိုင်ပါသည်။

## License

Zap ကို [MIT License](LICENSE) အောက်တွင် ဖြန့်ချိထားပါသည်။
