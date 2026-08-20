# Zap Packages and Module Tooling

Zap package tooling သည် local project manifest၊ deterministic `zap.lock` validation နှင့် local module resolution အဆင့်တွင် ရှိပါသည်။ `zap add`၊ `zap lock`၊ `zap install` နှင့် `zap update` commands များကို native CLI တွင် အကောင်အထည်ဖော်ထားပြီး remote registry၊ dependency download နှင့် publish workflow များသည် future roadmap ဖြစ်သည်။

## Current manifest

Project root တွင် `zap.toml` ထည့်နိုင်သည်။

```toml
[package]
name = "my-app"
version = "0.1.0"
main = "main.zp"
```

`main` မရေးထားပါက `main.zp` ကို default entry file အဖြစ် အသုံးပြုသည်။ Manifest နှင့် entry file ကို validate လုပ်ရန်—

```bash
zap check
zap check path/to/project
```

## Local module layout

```text
my-app/
├── zap.toml
├── main.zp
├── tests/
│   └── app_test.zp
├── modules/
│   └── math.zp
└── lib/
    └── text.zp
```

Source file မှ local module ကို import လုပ်ရန်—

```zp
use "math.zp"
```

Runtime သည် main source directory၊ `modules/` နှင့် `lib/` directories များကို ရှာဖွေသည်။ `use "math"` သည် `math.zp` အဖြစ်လည်း ရှာဖွေနိုင်သည်။

## Current CLI

```text
zap main.zp
zap check
zap test
zap fmt main.zp
zap add package-name 1.0 [project-dir]
zap lock [project-dir]
zap install [project-dir]
zap update [project-dir]
zap --version
zap --help
```

`zap test` သည် `tests/` directory နှင့် ၎င်းအောက်ရှိ subdirectories များထဲမှ `*_test.zp` files အားလုံးကို path အလိုက် sort လုပ်ပြီး run လုပ်သည်။ `zap init` သည် starter `tests/smoke_test.zp` ကိုလည်း ဖန်တီးပေးသည်။ `zap add` သည် manifest ထဲသို့ string-valued dependency ကို canonical order ဖြင့် ထည့်ပြီး ရှိပြီးသား lockfile ကို invalidate လုပ်သည်။ `zap lock` ဖြင့် lockfile ကို ပြန်လည် generate လုပ်နိုင်သည်။ `zap install` သည် ရှိပြီးသား manifest နှင့် lockfile ကို ပြောင်းလဲခြင်းမရှိဘဲ validate လုပ်သည်။ `zap update` သည် current manifest မှ canonical `zap.lock` ကို deterministic အတိုင်း ပြန်လည် generate လုပ်သည်။ နှစ်ခုလုံးသည် remote registry သို့ ချိတ်ဆက်ခြင်း သို့မဟုတ် package download ပြုလုပ်ခြင်း မရှိသေးပါ။ `zap new`၊ `zap publish` နှင့် framework-specific commands များသည် လက်ရှိ release တွင် မပါဝင်သေးပါ။

## Future package manager

နောက် version များတွင် local path packages၊ Git repositories နှင့် signed registry packages များကို support လုပ်ခြင်း၊ checksum verification၊ dependency graph resolution နှင့် reproducible builds များကို ထည့်သွင်းမည်။ လက်ရှိ `zap.lock` သည် manifest dependency requirements များကို deterministic အတိုင်း validate လုပ်ပေးနေပြီဖြစ်သည်။

Framework packages ဖြစ်သော `zap-web`၊ `zap-mobile`၊ `zap-ai` နှင့် `zap-iot` များသည် Zap core တည်ငြိမ်ပြီးနောက် သီးခြား domain packages အဖြစ် တည်ဆောက်မည်။
