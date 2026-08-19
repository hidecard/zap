# Zap Package Manifest

Zap project တစ်ခု၏ root directory တွင် `zap.toml` ဖိုင်ကို ထားနိုင်သည်။ Native runtime သည် manifest ကို `zap check` command ဖြင့် validate လုပ်ပြီး main source file ရှိ/မရှိ စစ်ဆေးသည်။

## Minimal manifest

```toml
[package]
name = "hello-app"
version = "0.1.0"
main = "main.zp"
```

`main` မရေးထားပါက `main.zp` ကို default entry file အဖြစ် အသုံးပြုသည်။

## Recommended layout

```text
hello-app/
├── zap.toml
├── main.zp
├── modules/
│   └── math.zp
└── lib/
    └── text.zp
```

`use "math"` သို့မဟုတ် `use "math.zp"` ကို run လုပ်သောအခါ Zap သည် main file ၏ directory၊ `modules/` directory နှင့် `lib/` directory များကို အစဉ်လိုက်ရှာသည်။

## Commands

```bash
zap check
zap check path/to/project
zap main.zp
zap fmt main.zp
```

`zap check` သည် manifest ဖတ်နိုင်ခြင်း၊ `name` နှင့် `version` ရှိခြင်း၊ entry file ရှိခြင်းတို့ကို စစ်ဆေးသည်။ လက်ရှိ package manager သည် local project validation နှင့် local module resolution အဆင့်တွင်ရှိပြီး remote registry၊ lockfile နှင့် dependency download များကို နောက်ပိုင်းတွင် ထည့်သွင်းမည်။
