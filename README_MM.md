# Zap Programming Language

![Zap Programming Language banner](assets/branding/zap-banner.png)

[![Zap CI](https://github.com/hidecard/zap/actions/workflows/ci.yml/badge.svg)](https://github.com/hidecard/zap/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/hidecard/zap?display_name=tag&sort=semver&color=2ea44f)](https://github.com/hidecard/zap/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Runtime: Rust](https://img.shields.io/badge/runtime-Rust-orange.svg)](native/)
[![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20Windows%20%7C%20macOS-lightgrey.svg)](https://github.com/hidecard/zap/actions)
[![Discord](https://img.shields.io/badge/Discord-Community-5865F2.svg?logo=discord&logoColor=white)](https://discord.gg/j9DHdCtJE)
[![Telegram](https://img.shields.io/badge/Telegram-Group-26A5E4.svg?logo=telegram&logoColor=white)](https://t.me/+fySFCXwMt8U3Y2Y1)

**ဘာသာစကားရွေးချယ်ရန်:** [English README](README.md) · [မြန်မာ README](README_MM.md)

**Community:** [Discord](https://discord.gg/j9DHdCtJE) · [Telegram](https://t.me/+fySFCXwMt8U3Y2Y1)

> **Zap** သည် `.zp` source file များကို အသုံးပြုသည့် ဖတ်ရလွယ်ကူပြီး standalone native runtime ပါဝင်သော general-purpose programming language ဖြစ်သည်။

Zap ကို စတင်လေ့လာသူများအတွက် ရိုးရှင်းပြီး နားလည်ရလွယ်ကူစေရန် ရည်ရွယ်ထားပါသည်။ Indentation-based blocks၊ variables၊ functions၊ collections၊ conditions၊ loops၊ classes၊ modules၊ type annotations နှင့် Result/Option values များကို language core ထဲတွင် ထည့်သွင်းထားပါသည်။ Native source run တစ်ကြိမ်စီတွင် module cache၊ import cycle၊ execution depth၊ workspace confinement၊ recursive value charging/rollback ပါသော logical budget၊ object store နှင့် parent-linked lexical closure frame များကို live binding cell များဖြင့် ထိန်းသိမ်းကာ မရောနှောစေရန် explicit `ExecutionContext` တစ်ခုကို အသုံးပြုပါသည်။ Object/capture cycle များသည် explicit `clear_object_fields()` policy အောက်တွင်သာ ရှိပြီး checked object/frame access များသည် panic မဖြစ်ဘဲ typed borrow failure ပြန်ပေးပါသည်။ LSP server သည် open document များကို per-session `LspState` ထဲတွင် ပိုင်ဆိုင်ပါသည်။ Normal source program နှင့် local module များသည် canonical AST boundary မှတစ်ဆင့်သာ execute လုပ်ပြီး native object construction၊ default expression နှင့် direct built-in dispatch များကိုလည်း ထို boundary အတွင်း ဆောင်ရွက်ပါသည်။ Support မလုပ်သော named built-in call များကို explicit error ဖြင့် ပြန်ပေးပြီး line interpreter ကို older line-bodied function record များအတွက် compatibility boundary အဖြစ်သာ ထားရှိပါသည်။ Post-v2.2.2 master hardening တွင် canonical equality ကို cycle-safe နှင့် bounded အဖြစ် ပြုလုပ်ခြင်း၊ logical accounting နှင့် AST member read များတွင် checked object/frame borrow error ပြန်ပေးခြင်း၊ task/frame invariant fallback များကို panic မဖြစ်အောင် ပြုလုပ်ခြင်းနှင့် LSP rename scope-stack panic path ဖယ်ရှားခြင်းတို့ ပါဝင်သည်။ ဤအလုပ်များကို v2.2.3 patch release တွင် ထည့်သွင်းထားပြီး public weak-reference API၊ automatic collector၊ traits implementation၊ parser syntax သို့မဟုတ် runtime syntax အသစ် မထည့်သွင်းပါ။

## Project Status

Zap သည် production-ready language ecosystem တစ်ခုအဖြစ် တိုးတက်နေပါသည်။ Native Rust runtime၊ direct AST execution၊ structured diagnostics၊ `ZAP-MEMORY-001` stable memory-limit diagnostic၊ control-flow type narrowing၊ module visibility၊ OOP rules၊ deterministic dependency lockfiles၊ registry resolution၊ checksum verification၊ offline package reuse၊ executor-backed context-owned language scheduling၊ `ScheduledFuture` handle၊ cooperative `task_cancel`၊ poll-budget `task_join_timeout` ပါသော async runtime နှင့် stdio LSP/editor integration များကို ထည့်သွင်းထားပါသည်။ M2-VERIFY-01 တွင် fixed-seed bounded replay၊ repeated semantic outcome digest နှင့် CI/release-preflight evidence များကို ထည့်သွင်းထားပါသည်။ M3-STDLIB-01 တွင် public standard-library domain နှင့် builtin တစ်ခုချင်းစီအတွက် stability၊ deprecation၊ semver၊ platform၊ limit၊ timeout/error နှင့် determinism metadata ပါသော catalog နှင့် bilingual policy ကို ထည့်သွင်းထားပါသည်။ M3-LSP-01 တွင် parser/lexer-backed rename၊ didClose cleanup၊ nested/module-aware indexing၊ catalog-driven completion နှင့် async builtin hover/signature metadata ပါသော LSP/editor semantic parity ကို ထည့်သွင်းထားပါသည်။ M4-RFC-01 သည် traits နှင့် composition အတွက် reviewed design direction ကို မှတ်တမ်းတင်ထားသော်လည်း proposed syntax ကို enable မလုပ်သေးပါ။ v2.2.0 နောက်ပိုင်း LSP hardening တွင် standard full-sync `didChange` ၏ `params.contentChanges` ကို အသုံးပြုကာ document version များကို track လုပ်ပြီး accepted buffer အပေါ် diagnostics ထုတ်ပေးသည်။ Stale သို့မဟုတ် unsupported range edit များကို လုံခြုံစွာ reject လုပ်သည်။ Scope-aware semantic rename သည် ယခု file-local binding၊ shadowing၊ closure၊ parameter နှင့် import alias များကို resolve လုပ်နိုင်ပြီး cross-file rename ကို support မလုပ်သေးပါ။ ဤ LSP/editor correction များသည် immutable v2.2.0 tag နောက်ပိုင်း `master` တွင် ပါဝင်လာပြီး v2.2.1 corrective release တွင် ထုတ်ဝေထားသည်။ ထို့နောက် runtime-safety၊ helper၊ grammar နှင့် documentation correction များကို v2.2.2 တွင် ထုတ်ဝေထားပြီး အထက်တွင်ဖော်ပြထားသော post-v2.2.2 hardening ကို v2.2.3 တွင် ထည့်သွင်းထားသည်။

| အချက် | လက်ရှိအခြေအနေ |
|---|---|
| လက်ရှိ release line | `v2.2.3` |
| Runtime | Native Rust runtime |
| Source file | `.zp`၊ အများအားဖြင့် `main.zp` |
| Project manifest | `zap.toml` |
| CLI | `zap` |
| Platforms | Linux၊ Windows၊ macOS ARM64 |
| Documentation hub | [မြန်မာ navigation](docs/DOCUMENTATION_NAVIGATION_MM.md) · [English navigation](docs/DOCUMENTATION_NAVIGATION_EN.md) |
| Runtime-state contract | [မြန်မာ](docs/RUNTIME_STATE_MM.md) · [English](docs/RUNTIME_STATE_EN.md) |
| Memory budget/object store contract | [မြန်မာ](docs/MEMORY_BUDGET_OBJECT_STORE_MM.md) · [English](docs/MEMORY_BUDGET_OBJECT_STORE_EN.md) |
| AST foundation status | [မြန်မာ](docs/P0_FOUNDATION_STATUS_MM.md) · [English](docs/P0_FOUNDATION_STATUS_EN.md) |
| Runtime architecture | `runtime_state.rs` နှင့် `value.rs` တွင် per-run `RuntimeState`၊ `MemoryBudget`၊ `ObjectStore`၊ workspace-root ownership၊ module-cache isolation၊ import-cycle tracking၊ execution-depth accounting၊ reset-detached lifecycle statistics နှင့် parent-linked `EnvFrame` closure များကို အကောင်အထည်ဖော်ထားပါသည် |
| Documentation source | [Zap documentation directory](https://github.com/hidecard/zap/tree/master/docs) |
| Verification status | M2-VERIFY-01 bounded replay၊ M2-VERIFY-02 native matrix၊ M2-BENCH-01 provenance/variance၊ M2-REG-01 transport၊ M3-STDLIB-01 policy evidence၊ M3-LSP-01 semantic-parity/editor validation နှင့် post-release LSP protocol synchronization evidence |
| Language design | [Traits/composition RFC](docs/TRAITS_RFC_MM.md) — design-only ဖြစ်ပြီး v2.2.3 အတွက် deferred |
| Release version policy | [Single-source-of-truth policy](docs/RELEASE_VERSION_POLICY_MM.md) |
| v2.2.0 နောက်ပိုင်း remediation provenance | [Corrective-release record](docs/POST_V2.2.0_REMEDIATION_MM.md) — v2.2.0၊ v2.2.1 နှင့် v2.2.2 သည် immutable ဖြစ်ပြီး LSP/editor correction များကို v2.2.1၊ နောက်ဆက်တွဲ runtime-safety/helper correction များကို v2.2.2၊ post-v2.2.2 runtime/equality/borrow/LSP hardening များကို v2.2.3 တွင် ထည့်သွင်းထားသည် |
| Repository | [github.com/hidecard/zap](https://github.com/hidecard/zap) |
| Releases | [GitHub Releases](https://github.com/hidecard/zap/releases) |

## Release provenance

ဤ README ထဲရှိ installation link နှင့် archive name များသည် published [v2.2.3 release](https://github.com/hidecard/zap/releases/tag/v2.2.3) ကို ရည်ညွှန်းထားခြင်း ဖြစ်ပါသည်။ အစောပိုင်း [v2.2.0 release](https://github.com/hidecard/zap/releases/tag/v2.2.0) နှင့် published [v2.2.1 release](https://github.com/hidecard/zap/releases/tag/v2.2.1) တို့၏ tag နှင့် signed asset များသည် immutable ဖြစ်သည်။ v2.2.0 နောက်ပိုင်း remediation history နှင့် v2.2.3 အထိ runtime-safety/helper correction များကို [remediation/provenance record](docs/POST_V2.2.0_REMEDIATION_MM.md) နှင့် v2.2.3 release note တွင် မှတ်တမ်းတင်ထားပါသည်။ post-v2.2.2 hardening ကို v2.2.3 တွင် ထည့်သွင်းထားသည်။

## Learning Guide

စတင်လေ့လာမည့်လမ်းကြောင်းကို ရွေးချယ်ရန် [မြန်မာ documentation navigation](docs/DOCUMENTATION_NAVIGATION_MM.md) ကို အရင်ဖတ်ရှုပါ။ ထို့နောက် [မြန်မာ learning guide](docs/LEARN_ZAP_MM.md) နှင့် [မြန်မာ syntax guide](docs/SYNTAX_GUIDE.md) ကို အသုံးပြုနိုင်ပါသည်။ English lessons အတွက် [English learning guide](docs/LEARN_ZAP_EN.md)၊ [English syntax guide](docs/SYNTAX_GUIDE_EN.md) နှင့် [English documentation navigation](docs/DOCUMENTATION_NAVIGATION_EN.md) ကို ဖတ်ရှုနိုင်ပါသည်။

### LSP editor hardening အတွင်း limitation

လက်ရှိ server သည် full document synchronization ကို ကြေညာပြီး လက်ခံထားသော newer document version များအတွက် standard `contentChanges` ကို မှန်ကန်စွာ အသုံးပြုသည်။ Position-aware application မတည်ဆောက်မချင်း range-based incremental change များကို reject လုပ်ထားသည်။ Server သည် UTF-8၊ UTF-16 သို့မဟုတ် UTF-32 position column များကို negotiate လုပ်နိုင်ပြီး malformed/host/traversal file URI များကို reject လုပ်ကာ session index ကို document ၂၅၆ ခု၊ import level ၃၂ နှင့် source text ၃၂ MiB အထိ ကန့်သတ်ထားသည်။ Rename သည် shadowing၊ closure၊ parameter နှင့် import alias အပါအဝင် file-local lexical binding များကို resolve လုပ်သည်။ Cross-file rename ကို support မလုပ်သေးသဖြင့် automated refactoring မလုပ်မီ ရလဒ်ကို ပြန်လည်စစ်ဆေးရမည်။ Protocol regression ကို `scripts/test_lsp_protocol_sync.sh` ဖြင့် စစ်ဆေးနိုင်သည်။

## Why Zap?

Zap သည် language core ကို သေးငယ်၊ ရှင်းလင်းပြီး လေ့လာရလွယ်ကူအောင် တည်ဆောက်ထားပါသည်။ `.zp` file များကို native executable ဖြင့် တိုက်ရိုက် run နိုင်ပြီး parser ပိုင် source များကို canonical AST ဖြင့် execute လုပ်ပါသည်။ Async task များအတွက် context-owned `ScheduledFuture`၊ `task_join`၊ `task_is_ready`၊ cooperative `task_cancel`၊ poll-budget `task_join_timeout` နှင့် explicit terminal-state/one-time admitted-task release semantics နှင့် eager scheduled-value contract များကို အသုံးပြုနိုင်ပါသည်။ Fixed-seed bounded replay verification သည် malformed input နှင့် repeated semantic outcome များကို CI တွင် ထပ်မံစစ်ဆေးနိုင်စေပါသည်။ Standard-library public surface ကို `text`၊ `math`၊ `collections`၊ `filesystem`၊ `json`၊ `system`၊ `time`၊ `logging`၊ `runtime`၊ `async`၊ `network` နှင့် `process` domain များအဖြစ် ဖွဲ့စည်းထားပြီး M3-STDLIB-01 catalog/policy သည် stability၊ deprecation၊ semver၊ platform၊ limit၊ timeout/error နှင့် determinism metadata များကို သတ်မှတ်ထားပါသည်။ Per-run workspace/module/execution state နှင့် logical budget/object counter များကို stable `memory_stats()` lifecycle fields၊ `cycle_policy=explicit_clear_object_fields` capability report၊ public weak-reference API သို့မဟုတ် automatic collector မရှိသည့် bounded cycle policy၊ AST value/callable/default/object charge နှင့် failed-operation rollback၊ checked object-field နှင့် canonical-AST EnvFrame borrow boundary၊ explicit `ScheduledFuture` terminal state နှင့် one-time admitted-task release၊ eager async scheduled-value semantics၊ first-class callable value များနှင့် parent-linked live-cell `EnvFrame` closure များအတူ explicit runtime context ဖြင့် ခွဲခြားထားပြီး LSP document state ကို per-session `LspState` ဖြင့် ပိုင်ဆိုင်ထားပါသည်။ Parser ပိုင် source များအတွက် canonical AST-only path ထဲတွင် native `new(...)` construction နှင့် AST ဖြင့် evaluate လုပ်သော default expression များလည်း ပါဝင်ပါသည်။ Legacy line execution ကို compatibility-only boundary အဖြစ် ကန့်သတ်ထားပါသည်။
နောင်တွင် web၊ AI၊ mobile နှင့် IoT libraries များ တည်ဆောက်ရန် foundation အဖြစ် အသုံးပြုနိုင်ပါသည်။

## Installation

Zap သည် သီးခြား language runtime မလိုအပ်သော standalone native executable အဖြစ် ဖြန့်ချိပါသည်။ v2.2.3 အတွက် မိမိအသုံးပြုသည့် operating system နှင့် CPU architecture ကိုက်ညီသော archive ကို [v2.2.3 GitHub Release](https://github.com/hidecard/zap/releases/tag/v2.2.3) မှ download လုပ်ပြီး checksum ကို verify လုပ်ကာ extract လုပ်ပါ။

v2.2.3 official archive များမှာ `zap-2.2.3-linux-x86_64.tar.gz`၊ `zap-2.2.3-macos-arm64.tar.gz` နှင့် `zap-2.2.3-windows-x86_64.zip` ဖြစ်ပါသည်။

| Platform | Architecture | လုပ်ဆောင်ရန် |
|---|---|---|
| Linux | x86_64 | Archive ကို extract လုပ်ပြီး `bash install.sh` run ပါ။ |
| Windows | x86_64 | Archive ကို extract လုပ်ပြီး Command Prompt မှ `install_windows.bat` run ပါ။ |
| macOS | ARM64 | `chmod +x install.sh` ပြီးနောက် `./install.sh` run ပါ။ |

Install ပြီးပါက version နှင့် help ကို စစ်ဆေးပါ။

```bash
zap --version
zap --help
```

Windows Command Prompt တွင်—

```bat
zap.exe --version
zap.exe --help
```

## ပထမဆုံး Zap Program

`main.zp` file တစ်ခုဖန်တီးပြီး Zap ဖြင့် run ပါ။

```text
message = "Hello from Zap"
print(message)
```

```bash
zap main.zp
```

Windows တွင်—

```bat
zap.exe main.zp
```

## Package Project

Package project များတွင် `zap.toml` manifest နှင့် canonical `zap.lock` lockfile ကို အသုံးပြုပါသည်။ Local path dependencies များကို deterministic order ဖြင့် recursive validation လုပ်ပြီး registry artifacts များကို SHA-256 checksum ဖြင့် စစ်ဆေးပါသည်။ Offline reuse အတွက် `ZAP_OFFLINE=1` ကို အသုံးပြုနိုင်ပါသည်။

## VS Code extension

Official **Zap Language Support v0.5.0** extension သည် syntax highlighting၊ snippets၊ diagnostics၊ autocomplete၊ signature help၊ hover၊ go-to-definition၊ formatting၊ workspace symbols၊ rename နှင့် run support များကို ပေးပါသည်။ Repository ထဲတွင် catalog နှင့်ကိုက်ညီသော TextMate grammar နှင့် language configuration ကို `editors/vscode/` အောက်တွင် ထည့်သွင်းထားပြီး `scripts/validate_vscode_assets.py` ဖြင့် စစ်ဆေးနိုင်ပါသည်။

```bash
code --install-extension ArkarYan.zap-language-support
```

[Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=ArkarYan.zap-language-support) မှလည်း install လုပ်နိုင်ပါသည်။

## Community နှင့် Contribution

မေးခွန်းများ၊ အကြံပြုချက်များနှင့် Zap development ဆွေးနွေးမှုများအတွက် [Discord Community](https://discord.gg/j9DHdCtJE) သို့မဟုတ် [Telegram Group](https://t.me/+fySFCXwMt8U3Y2Y1) သို့ ဝင်ရောက်နိုင်ပါသည်။ Source code နှင့် issue များကို [GitHub repository](https://github.com/hidecard/zap) တွင် ကြည့်ရှုနိုင်ပါသည်။

အဓိက project information ကို ဤ README နှစ်ခုတွင် ထိန်းသိမ်းထားပြီး lesson အပြည့်အစုံကို အထက်ပါ learning guides များတွင် ဆက်လက်ဖတ်ရှုနိုင်ပါသည်။ English version အတွက် [README.md](README.md) ကို ဖတ်ရှုပါ။
