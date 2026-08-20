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
├── zap.lock
├── main.zp
├── modules/
│   └── math.zp
└── lib/
    └── text.zp
```

`use "math"` သို့မဟုတ် `use "math.zp"` ကို run လုပ်သောအခါ Zap သည် main file ၏ directory၊ `modules/` directory နှင့် `lib/` directory များကို အစဉ်လိုက်ရှာသည်။

## Dependencies နှင့် Lockfile

`[dependencies]` section ထဲတွင် package name နှင့် version requirement များကို ရေးနိုင်သည်။ Zap သည် dependency name များကို alphabetic order ဖြင့် စီပြီး byte-for-byte တူညီသော canonical `zap.lock` ကို generate လုပ်သည်။

```toml
[package]
name = "hello-app"
version = "0.1.0"
main = "main.zp"

[dependencies]
web = "0.3"
json-tools = "1.2"
```

Lockfile generate လုပ်ရန်—

```bash
zap lock
zap lock path/to/project
```

`zap.lock` တွင် lockfile version၊ package identity နှင့် sorted dependencies များပါဝင်သည်။ Project နှင့်အတူ lockfile ကို commit တင်ထားသင့်သည်။ Dependency ထည့်/ဖယ်ခြင်း သို့မဟုတ် package version ပြောင်းခြင်း ပြုလုပ်ပြီးတိုင်း `zap lock` ဖြင့် regenerate လုပ်ပါ။

`zap check` နှင့် `zap build` များသည် dependency ရှိသော project များတွင် `zap.lock` မရှိခြင်း၊ stale ဖြစ်ခြင်း သို့မဟုတ် canonical format မဟုတ်ခြင်းကို error ပြန်ပေးသည်။

## Dependency ထည့်ခြင်း

Manifest ကို တိုက်ရိုက်မပြင်ဘဲ dependency ထည့်ရန် `zap add` ကို အသုံးပြုနိုင်သည်။

```bash
zap add json-tools 1.2
zap add web 0.3 path/to/project
```

`zap add` သည် duplicate dependency name ကို reject လုပ်ပြီး dependency များကို alphabetic order ဖြင့် စီပေးသည်။ Manifest ပြောင်းလဲသွားသောကြောင့် ရှိပြီးသား `zap.lock` ကို ဖယ်ရှားပေးပြီး `zap lock` ဖြင့် canonical lockfile ကို ပြန်လည် generate လုပ်ရမည်။

## Commands

```bash
zap check
zap check path/to/project
zap lock
zap lock path/to/project
zap add package-name 1.0
zap add package-name 1.0 path/to/project
zap build path/to/project
zap main.zp
zap fmt main.zp
```

`zap check` သည် manifest ဖတ်နိုင်ခြင်း၊ `name` နှင့် `version` ရှိခြင်း၊ dependency lockfile မှန်ကန်ခြင်း၊ entry file ရှိခြင်းနှင့် static source checks များကို စစ်ဆေးသည်။ လက်ရှိ package manager သည် deterministic local manifest/lockfile validation နှင့် local module resolution အဆင့်တွင်ရှိပြီး remote registry download နှင့် publishing များမှာ နောက်ပိုင်း ecosystem milestone များ ဖြစ်သည်။
