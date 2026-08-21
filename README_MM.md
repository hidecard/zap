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

Zap ကို စတင်လေ့လာသူများအတွက် ရိုးရှင်းပြီး နားလည်ရလွယ်ကူစေရန် ရည်ရွယ်ထားပါသည်။ Indentation-based blocks၊ variables၊ functions၊ collections၊ conditions၊ loops၊ classes၊ modules၊ type annotations နှင့် Result/Option values များကို language core ထဲတွင် ထည့်သွင်းထားပါသည်။

## Project Status

Zap သည် production-ready language ecosystem တစ်ခုအဖြစ် တိုးတက်နေပါသည်။ Native Rust runtime၊ direct AST execution၊ structured diagnostics၊ control-flow type narrowing၊ module visibility၊ OOP rules၊ deterministic dependency lockfiles၊ registry resolution၊ checksum verification၊ offline package reuse၊ async runtime နှင့် stdio LSP/editor integration များကို ထည့်သွင်းထားပါသည်။

| အချက် | လက်ရှိအခြေအနေ |
|---|---|
| လက်ရှိ release line | `v2.1.7` |
| Runtime | Native Rust runtime |
| Source file | `.zp`၊ အများအားဖြင့် `main.zp` |
| Project manifest | `zap.toml` |
| CLI | `zap` |
| Platforms | Linux၊ Windows၊ macOS ARM64 |
| Release version policy | [Single-source-of-truth policy](docs/RELEASE_VERSION_POLICY_MM.md) |
| Repository | [github.com/hidecard/zap](https://github.com/hidecard/zap) |
| Releases | [GitHub Releases](https://github.com/hidecard/zap/releases) |

## Learning Guide

စတင်လေ့လာရန် [မြန်မာ learning guide](docs/LEARN_ZAP_MM.md) ကို ဖတ်ရှုပါ။ Syntax အသေးစိတ်အတွက် [မြန်မာ syntax guide](docs/SYNTAX_GUIDE.md) ကို အသုံးပြုနိုင်ပြီး English lessons အတွက် [English learning guide](docs/LEARN_ZAP_EN.md) နှင့် [English syntax guide](docs/SYNTAX_GUIDE_EN.md) ကို ဖတ်ရှုနိုင်ပါသည်။

## Why Zap?

Zap သည် language core ကို သေးငယ်၊ ရှင်းလင်းပြီး လေ့လာရလွယ်ကူအောင် တည်ဆောက်ထားပါသည်။ `.zp` file များကို native executable ဖြင့် တိုက်ရိုက် run နိုင်ပြီး နောင်တွင် web၊ AI၊ mobile နှင့် IoT libraries များ တည်ဆောက်ရန် foundation အဖြစ် အသုံးပြုနိုင်ပါသည်။

## Installation

Zap သည် သီးခြား language runtime မလိုအပ်သော standalone native executable အဖြစ် ဖြန့်ချိပါသည်။ v2.1.7 အတွက် မိမိအသုံးပြုသည့် operating system နှင့် CPU architecture ကိုက်ညီသော archive ကို [v2.1.7 GitHub Release](https://github.com/hidecard/zap/releases/tag/v2.1.7) မှ download လုပ်ပြီး checksum ကို verify လုပ်ကာ extract လုပ်ပါ။

v2.1.7 official archive များမှာ `zap-2.1.7-linux-x86_64.tar.gz`၊ `zap-2.1.7-macos-arm64.tar.gz` နှင့် `zap-2.1.7-windows-x86_64.zip` ဖြစ်ပါသည်။

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

## VS Code Extension

Official **Zap Language Support v0.5.0** extension သည် syntax highlighting၊ snippets၊ diagnostics၊ autocomplete၊ signature help၊ hover၊ go-to-definition၊ formatting၊ workspace symbols နှင့် run support များကို ပေးပါသည်။

```bash
code --install-extension ArkarYan.zap-language-support
```

[Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=ArkarYan.zap-language-support) မှလည်း install လုပ်နိုင်ပါသည်။

## Community နှင့် Contribution

မေးခွန်းများ၊ အကြံပြုချက်များနှင့် Zap development ဆွေးနွေးမှုများအတွက် [Discord Community](https://discord.gg/j9DHdCtJE) သို့မဟုတ် [Telegram Group](https://t.me/+fySFCXwMt8U3Y2Y1) သို့ ဝင်ရောက်နိုင်ပါသည်။ Source code နှင့် issue များကို [GitHub repository](https://github.com/hidecard/zap) တွင် ကြည့်ရှုနိုင်ပါသည်။

အဓိက project information ကို ဤ README နှစ်ခုတွင် ထိန်းသိမ်းထားပြီး lesson အပြည့်အစုံကို အထက်ပါ learning guides များတွင် ဆက်လက်ဖတ်ရှုနိုင်ပါသည်။ English version အတွက် [README.md](README.md) ကို ဖတ်ရှုပါ။
