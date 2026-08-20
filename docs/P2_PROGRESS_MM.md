# Zap P2 Progress — Ecosystem Foundation

## လက်ရှိအခြေအနေ

Zap P1 Language Core ကို `v1.0.0` အဖြစ် release ပြုလုပ်ပြီးပါပြီ။ ယခု P2 ကို remote registry မလိုအပ်သေးသော deterministic local package-manager foundation ဖြင့် စတင်ထားပါသည်။

| Milestone | အခြေအနေ | မှတ်ချက် |
|---|---|---|
| Manifest dependency declarations | ပြီးစီး | `[dependencies]` entries များနှင့် local path specification များကို parse နှင့် validate လုပ်သည်။ |
| Canonical lockfile | ပြီးစီး | `zap.lock` ကို package/dependency order တည်ငြိမ်စွာ generate လုပ်ပြီး local path များကို canonical ပုံစံဖြင့်ရေးသည်။ |
| `zap add` command | ပြီးစီး | String-valued dependency ထည့်ခြင်း၊ dependency section sort လုပ်ခြင်း၊ duplicate reject လုပ်ခြင်းနှင့် lockfile invalidate လုပ်ခြင်းတို့ ပါဝင်သည်။ |
| Registry-ready package metadata | ပြီးစီး | `description`၊ `authors`၊ `license`၊ `repository` နှင့် 64-character hexadecimal SHA-256 `checksum` fields များကို deterministic validate လုပ်သည်။ |
| Remote registry resolution | Roadmap | Network policy၊ cache၊ index resolution၊ download နှင့် integrity enforcement များ လိုအပ်သည်။ |
| `zap install` | ပြီးစီး | လက်ရှိ manifest နှင့် canonical lockfile ကို project files မပြောင်းလဲဘဲ validate လုပ်သည်။ Registry သို့ မချိတ်ဆက်ပါ။ |
| `zap update` | ပြီးစီး | လက်ရှိ manifest မှ canonical lockfile ကို deterministic အတိုင်း ပြန်လည် generate လုပ်ပြီး local dependency graph အပြည့်အစုံကို validate လုပ်သည်။ |
| Async runtime | Roadmap | သီးခြား P2 track ဖြစ်သည်။ |
| LSP/editor integration | Roadmap | Tooling track သီးခြားဖြစ်သည်။ |

## Local install/update contract

```bash
zap install [project-dir]
zap update [project-dir]
```

`zap install` သည် validation-only command ဖြစ်သည်။ Dependency ပါသော project များတွင် လက်ရှိဖြစ်ပြီး canonical ဖြစ်သော `zap.lock` ရှိရမည်။ Manifest သို့မဟုတ် lockfile ကို မပြောင်းလဲဘဲ network/registry access မပြုလုပ်ပါ။ `zap update` သည် `zap lock` နှင့်တူညီသော canonical ordering ကို အသုံးပြုပြီး current manifest မှ `zap.lock` ကို deterministic အတိုင်း ပြန်လည် generate လုပ်သည်။ Package download သို့မဟုတ် registry dependency graph solving မပြုလုပ်သေးပါ။ Local path dependency များအတွက် nested manifest များကို lexicographic order ဖြင့် recursively validate လုပ်ပြီး lockfile မရေးမီ cycle များကို reject လုပ်သည်။

## `zap add` contract

```bash
zap add <name> <version> [project-dir]
```

Command သည် `zap.toml` ကို deterministic အတိုင်း update လုပ်သည်။ Empty/whitespace ပါသော name၊ duplicate dependency name နှင့် မမှန်ကန်သော single-line requirement များကို reject လုပ်သည်။ `zap.lock` ရှိပြီးသားဖြစ်ပါက manifest ပြောင်းလဲသွားသောကြောင့် lockfile ကို ဖျက်သည်။ ထို့နောက် `zap lock` ဖြင့် canonical lockfile ကို ပြန်လည် generate လုပ်နိုင်သည်။

## Verification

Native test suite တွင် dependency ထည့်ခြင်း၊ lexicographic ordering၊ duplicate rejection၊ lockfile invalidation၊ install validation၊ update regeneration၊ idempotence၊ stale-lock rejection၊ CLI help exposure၊ valid local package၊ missing local manifest၊ nested local package နှင့် deterministic cycle diagnostics များကို test coverage ထည့်ထားပါသည်။ `name = { path = "../local-lib" }` ပုံစံကို support လုပ်ပြီး path ကို သုံးစွဲသည့် project အပေါ်မူတည်၍ resolve လုပ်သည်။ Local package တွင် package name နှင့် version ပါသော `zap.toml` ရှိရမည်ဖြစ်ပြီး `zap.lock` တွင် canonical ပုံစံဖြင့် သိမ်းဆည်းသည်။ Local path dependency များကို sorted order ဖြင့် depth-first traverse လုပ်သည်။ Active traversal stack ထဲတွင် canonical path ထပ်ပေါ်လာပါက `dependency cycle detected: left -> right -> left` ကဲ့သို့ error ပြန်ပေးသည်။ Manifest metadata contract သည် registry-ready အဖြစ် ပြီးစီးပြီးဖြစ်သည်။ Remote fetching သည် နောက်ထပ် P2 milestone ဖြစ်နေဆဲဖြစ်သည်။ Lockfile generate/update မလုပ်မီ root နှင့် nested local package များအားလုံးတွင် metadata validation ပြုလုပ်သည်။

[English package guide](PACKAGE_EN.md)၊ [Burmese package guide](PACKAGE.md) နှင့် [ecosystem roadmap](ECOSYSTEM.md) ကို ဆက်လက်ဖတ်ရှုနိုင်ပါသည်။
