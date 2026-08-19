# Zap Packages and Module Tooling

Zap package tooling သည် လက်ရှိတွင် local project manifest နှင့် local module resolution အဆင့်တွင် ရှိပါသည်။ Remote registry၊ lockfile၊ dependency download နှင့် publish workflow များသည် future roadmap ဖြစ်ပြီး လက်ရှိ native CLI တွင် မအကောင်အထည်ဖော်ရသေးပါ။

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
zap --version
zap --help
```

`zap test` သည် `tests/` directory နှင့် ၎င်းအောက်ရှိ subdirectories များထဲမှ `*_test.zp` files အားလုံးကို path အလိုက် sort လုပ်ပြီး run လုပ်သည်။ `zap init` သည် starter `tests/smoke_test.zp` ကိုလည်း ဖန်တီးပေးသည်။ `zap new`၊ `zap add`၊ `zap install`၊ `zap update`၊ `zap publish` နှင့် framework-specific commands များသည် လက်ရှိ release တွင် မပါဝင်သေးပါ။ အဆိုပါ command များသည် manifest နှင့် local module system တည်ငြိမ်ပြီးနောက် ဆက်လက်တည်ဆောက်မည့် tooling roadmap ဖြစ်သည်။

## Future package manager

နောက် version များတွင် `zap.lock` ဖြင့် dependency versions များကို lock လုပ်ခြင်း၊ local path packages၊ Git repositories နှင့် signed registry packages များကို support လုပ်ခြင်း၊ checksum verification နှင့် reproducible builds များကို ထည့်သွင်းမည်။

Framework packages ဖြစ်သော `zap-web`၊ `zap-mobile`၊ `zap-ai` နှင့် `zap-iot` များသည် Zap core တည်ငြိမ်ပြီးနောက် သီးခြား domain packages အဖြစ် တည်ဆောက်မည်။
