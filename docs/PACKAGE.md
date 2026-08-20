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

`[dependencies]` section ထဲတွင် package name နှင့် version requirement သို့မဟုတ် local path specification များကို ရေးနိုင်သည်။ Zap သည် dependency name များကို alphabetic order ဖြင့် စီပြီး byte-for-byte တူညီသော canonical `zap.lock` ကို generate လုပ်သည်။

```toml
[package]
name = "hello-app"
version = "0.1.0"
main = "main.zp"

[dependencies]
web = "0.3"
json-tools = "1.2"
local-lib = { path = "../local-lib" }
```

Lockfile generate လုပ်ရန်—

```bash
zap lock
zap lock path/to/project
```

`zap.lock` တွင် lockfile version၊ package identity နှင့် sorted dependencies များပါဝင်သည်။ Local path သည် သုံးစွဲသည့် project directory အပေါ်မူတည်၍ resolve လုပ်ပြီး ရည်ညွှန်းသော directory တွင် package `name` နှင့် `version` ပါသော `zap.toml` ရှိရမည်။ Local dependency များကို lexicographic depth-first order ဖြင့် recursively စစ်ဆေးသောကြောင့် nested local package များအားလုံး မှန်ကန်မှသာ command အောင်မြင်သည်။ Active traversal stack ထဲတွင် canonical package path ထပ်ပေါ်လာပါက `dependency cycle detected: left -> right -> left` ကဲ့သို့ deterministic error ပြန်ပေးသည်။ Lockfile တွင် `local-lib = { path = "../local-lib" }` ပုံစံဖြင့် direct path ကို canonical ရေးသားသည်။ Project နှင့်အတူ lockfile ကို commit တင်ထားသင့်သည်။ Dependency ထည့်/ဖယ်ခြင်း သို့မဟုတ် package version ပြောင်းခြင်း ပြုလုပ်ပြီးတိုင်း `zap lock` ဖြင့် regenerate လုပ်ပါ။

`zap check` နှင့် `zap build` များသည် dependency ရှိသော project များတွင် `zap.lock` မရှိခြင်း၊ stale ဖြစ်ခြင်း သို့မဟုတ် canonical format မဟုတ်ခြင်းကို error ပြန်ပေးသည်။

## Dependency ထည့်ခြင်း

Manifest ကို တိုက်ရိုက်မပြင်ဘဲ dependency ထည့်ရန် `zap add` ကို အသုံးပြုနိုင်သည်။

```bash
zap add json-tools 1.2
zap add web 0.3 path/to/project
```

`zap add` သည် duplicate dependency name ကို reject လုပ်ပြီး dependency များကို alphabetic order ဖြင့် စီပေးသည်။ Manifest ပြောင်းလဲသွားသောကြောင့် ရှိပြီးသား `zap.lock` ကို ဖယ်ရှားပေးပြီး `zap lock` ဖြင့် canonical lockfile ကို ပြန်လည် generate လုပ်ရမည်။

## `zap install` နှင့် `zap update`

`zap install` သည် လက်ရှိ `zap.toml` နှင့် canonical `zap.lock` ကို ပြောင်းလဲခြင်းမရှိဘဲ စစ်ဆေးပေးသည်။ Dependency ရှိပြီး lockfile မရှိခြင်း၊ stale ဖြစ်ခြင်း သို့မဟုတ် canonical format မဟုတ်ခြင်းများကို reject လုပ်သောကြောင့် CI နှင့် reproducible project validation အတွက် အသုံးပြုနိုင်သည်။

```bash
zap install
zap install path/to/project
```

Manifest ကို ရည်ရွယ်ချက်ရှိရှိ ပြောင်းလဲပြီးနောက် lockfile ကို ပြန်လည် generate လုပ်ရန် `zap update` ကို အသုံးပြုသည်။ လက်ရှိ local package-manager foundation တွင် `zap update` သည် manifest မှ canonical `zap.lock` ကို deterministic အတိုင်း ပြန်ရေးပေးပြီး nested local path dependency graph ကိုလည်း စစ်ဆေးသည်။ Remote registry သို့ ချိတ်ဆက်ခြင်း သို့မဟုတ် package download ပြုလုပ်ခြင်း မရှိသေးပါ။ Version requirements များသည် remote resolution မတည်ဆောက်ရသေးသည့်အချိန်အထိ registry-ready leaf များအဖြစ်သာ ရှိနေသည်။

```bash
zap update
zap update path/to/project
```

`zap install` သည် validation-only command ဖြစ်ပြီး `zap update` သည် lockfile regeneration command ဖြစ်သည်။ နှစ်ခုလုံးသည် project directory အတွင်းတွင်သာ အလုပ်လုပ်ပြီး manifest နှင့် dependency ordering rules များကို မပြောင်းလဲစေပါ။

## Commands

```bash
zap check
zap check path/to/project
zap lock
zap lock path/to/project
zap add package-name 1.0
zap add package-name 1.0 path/to/project
zap install
zap install path/to/project
zap update
zap update path/to/project
zap build path/to/project
zap main.zp
zap fmt main.zp
```

`zap check` သည် manifest ဖတ်နိုင်ခြင်း၊ `name` နှင့် `version` ရှိခြင်း၊ dependency lockfile မှန်ကန်ခြင်း၊ entry file ရှိခြင်းနှင့် static source checks များကို စစ်ဆေးသည်။ လက်ရှိ package manager သည် deterministic local manifest/lockfile validation နှင့် local path package metadata စစ်ဆေးခြင်းအဆင့်တွင်ရှိသည်။ `zap install` သည် lockfile နှင့် local dependency graph ကို validate လုပ်ပြီး `zap update` သည် lockfile ကို ပြန်လည် generate လုပ်ကာ local graph ကို validate လုပ်သည်။ Local path graph များသည် cycle ရှိ/မရှိ စစ်ဆေးပြီး remote registry download၊ registry dependency solving နှင့် publishing များမှာ နောက်ပိုင်း ecosystem milestone များ ဖြစ်သည်။
