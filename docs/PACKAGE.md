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

## Registry-ready package metadata

`[package]` table ထဲတွင် နောက်ပိုင်း registry publishing အတွက် optional metadata fields များ ထည့်နိုင်သည်။ `zap lock` သို့မဟုတ် `zap update` မလုပ်မီ root package နှင့် nested local package များအားလုံးတွင် အဆိုပါ fields များကို locally validate လုပ်သည်။

```toml
[package]
name = "hello-app"
version = "0.1.0"
description = "A small Zap application"
authors = ["Zap Team"]
license = "MIT"
repository = "https://github.com/hidecard/zap"
checksum = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
```

`description`၊ `license` နှင့် `repository` တို့သည် အလွတ်မဖြစ်သော quoted string များ ဖြစ်ရမည်။ `authors` သည် အလွတ်မဖြစ်သော quoted string သို့မဟုတ် array ဖြစ်နိုင်သည်။ `checksum` ထည့်ထားပါက hexadecimal character 64 လုံးပါသော SHA-256 digest ဖြစ်ရမည်။

## Registry index နှင့် package cache

Zap တွင် deterministic JSON registry index နှင့် content-addressed package cache foundation ပါဝင်လာပါပြီ။ Index ပုံစံမှာ—

```json
{
  "packages": [
    {
      "name": "demo",
      "version": "1.0.0",
      "source": "file://demo.pkg",
      "checksum": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    }
  ]
}
```

Index ကို `zap registry check path/to/index.json` ဖြင့် validate လုပ်နိုင်သည်။ Package ကို `name` နှင့် `version` တိတိကျကျကိုက်ညီမှသာ ရွေးချယ်ပြီး missing သို့မဟုတ် duplicate entry များကို reject လုပ်သည်။ `file://` source များကို index file ၏ directory အပေါ်မူတည်၍ resolve လုပ်ကာ cache ထဲသို့ copy ပြီး SHA-256 checksum ကိုစစ်ဆေးသည်။ Default cache layout သည် `.zap/cache/<name>/<version>/<checksum>.pkg` ဖြစ်ပြီး `ZAP_CACHE_DIR` ဖြင့် အခြား cache root သတ်မှတ်နိုင်သည်။

Project dependency များအတွက် `ZAP_REGISTRY_INDEX` ကို local path၊ `file://` URL သို့မဟုတ် HTTPS URL ဖြင့် သတ်မှတ်နိုင်သည်။ Remote index ကို project resolution မလုပ်မီ `zap registry fetch <index-url>` ဖြင့် validate လုပ်နိုင်သည်။ `zap install` သည် cache entry ရှိ/မရှိနှင့် checksum မှန်/မမှန် စစ်ဆေးပြီး local သို့မဟုတ် HTTPS source မှ artifact မရှိသေးပါက cache ထဲသို့ ထည့်ပေးသည်။ `zap update` သည် registry validation ပြီးမှ canonical lockfile ကို ပြန်ရေးသည်။ `ZAP_OFFLINE=1` သတ်မှတ်ထားပါက download အသစ် မပြုလုပ်ဘဲ cache ထဲတွင်ရှိပြီး checksum မှန်သော package များဖြင့်သာ အောင်မြင်နိုင်သည်။ Plain HTTP ကို ပုံမှန်အားဖြင့် ပိတ်ထားပြီး local fixture အတွက်သာ `ZAP_ALLOW_INSECURE_HTTP=1` ဖြင့် ခွင့်ပြုနိုင်သည်။

Remote publishing အတွက် archive ကို အရင် SHA-256 စစ်ပြီး HTTPS endpoint သို့ `X-Zap-Package-*` headers များနှင့် ပို့ပေးသည်။ Bearer authentication အတွက် `ZAP_REGISTRY_TOKEN` ကို သတ်မှတ်နိုင်သည်။

```bash
zap registry publish https://registry.example/publish ./demo.pkg demo 1.0.0 <sha256>
```

Checksum မကိုက်ညီပါက network request မလုပ်မီ publish ကို reject လုပ်သည်။ လက်ရှိ contract သည် package archive ကို opaque bytes အဖြစ် upload လုပ်ခြင်းဖြစ်ပြီး registry server-side persistence၊ signed index နှင့် authentication policy များသည် နောက်ထပ်အလုပ်များ ဖြစ်သည်။

## Dependency ထည့်ခြင်း

Manifest ကို တိုက်ရိုက်မပြင်ဘဲ dependency ထည့်ရန် `zap add` ကို အသုံးပြုနိုင်သည်။

```bash
zap add json-tools 1.2
zap add web 0.3 path/to/project
```

`zap add` သည် duplicate dependency name ကို reject လုပ်ပြီး dependency များကို alphabetic order ဖြင့် စီပေးသည်။ Manifest ပြောင်းလဲသွားသောကြောင့် ရှိပြီးသား `zap.lock` ကို ဖယ်ရှားပေးပြီး `zap lock` ဖြင့် canonical lockfile ကို ပြန်လည် generate လုပ်ရမည်။

## `zap install` နှင့် `zap update`

`zap install` သည် လက်ရှိ `zap.toml`၊ canonical `zap.lock` နှင့် `ZAP_REGISTRY_INDEX` သတ်မှတ်ထားပါက registry cache ကို ပြောင်းလဲခြင်းမရှိဘဲ စစ်ဆေးပေးသည်။ Missing/stale lockfile၊ missing registry entry နှင့် checksum mismatch များကို reject လုပ်သောကြောင့် CI နှင့် reproducible project validation အတွက် အသုံးပြုနိုင်သည်။

```bash
zap install
zap install path/to/project
```

Manifest ကို ရည်ရွယ်ချက်ရှိရှိ ပြောင်းလဲပြီးနောက် lockfile ကို ပြန်လည် generate လုပ်ရန် `zap update` ကို အသုံးပြုသည်။ `zap update` သည် manifest မှ canonical `zap.lock` ကို deterministic အတိုင်း ပြန်ရေးပေးပြီး nested local path graph ကိုစစ်ဆေးသည်။ `ZAP_REGISTRY_INDEX` သတ်မှတ်ထားပါက exact registry version ကိုရွေးပြီး cache/download checksum ကို စစ်ဆေးသည်။ Version range solving မပါသေးဘဲ exact version selection foundation အဖြစ်သာ လုပ်ဆောင်သည်။

```bash
zap update
zap update path/to/project
```

`zap install` သည် validation-only command ဖြစ်ပြီး `zap update` သည် lockfile regeneration command ဖြစ်သည်။ နှစ်ခုလုံးသည် project directory အတွင်းတွင်သာ အလုပ်လုပ်ပြီး manifest နှင့် dependency ordering rules များကို မပြောင်းလဲစေပါ။

## Commands

```bash
zap check
zap registry fetch https://registry.example/index.json
zap registry publish https://registry.example/publish ./demo.pkg demo 1.0.0 <sha256>
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

`zap check` သည် manifest ဖတ်နိုင်ခြင်း၊ `name` နှင့် `version` ရှိခြင်း၊ dependency lockfile မှန်ကန်ခြင်း၊ entry file ရှိခြင်းနှင့် static source checks များကို စစ်ဆေးသည်။ လက်ရှိ package manager သည် deterministic local/HTTPS registry index validation၊ content-addressed cache၊ SHA-256 integrity enforcement၊ offline reuse နှင့် checksum-verified archive publishing foundation များကို ထောက်ပံ့သည်။ `zap install` သည် lockfile၊ local dependency graph နှင့် configured registry cache ကို validate လုပ်ပြီး `zap update` သည် lockfile ကို ပြန်လည် generate လုပ်ကာ registry source များကို validate လုပ်သည်။ Signed index၊ version range solving၊ cache garbage collection နှင့် registry server-side persistence များမှာ ဆက်လက်လုပ်ဆောင်ရန် ကျန်ရှိသည်။
