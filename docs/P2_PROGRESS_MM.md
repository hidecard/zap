# Zap P2 Progress — Ecosystem Foundation

## လက်ရှိအခြေအနေ

Zap P1 Language Core ကို `v1.0.0` အဖြစ် release ပြုလုပ်ပြီးပါပြီ။ ယခု P2 ကို remote registry မလိုအပ်သေးသော deterministic local package-manager foundation ဖြင့် စတင်ထားပါသည်။

| Milestone | အခြေအနေ | မှတ်ချက် |
|---|---|---|
| Manifest dependency declarations | ပြီးစီး | `[dependencies]` entries များကို parse နှင့် validate လုပ်သည်။ |
| Canonical lockfile | ပြီးစီး | `zap.lock` ကို package/dependency order တည်ငြိမ်စွာ generate လုပ်သည်။ |
| `zap add` command | ပြီးစီး | String-valued dependency ထည့်ခြင်း၊ dependency section sort လုပ်ခြင်း၊ duplicate reject လုပ်ခြင်းနှင့် lockfile invalidate လုပ်ခြင်းတို့ ပါဝင်သည်။ |
| Remote registry resolution | Roadmap | Package metadata၊ network policy၊ cache နှင့် integrity checks များ လိုအပ်သည်။ |
| `zap install` / `zap update` | Roadmap | Registry နှင့် dependency graph contract များသတ်မှတ်ပြီးနောက် အကောင်အထည်ဖော်မည်။ |
| Async runtime | Roadmap | သီးခြား P2 track ဖြစ်သည်။ |
| LSP/editor integration | Roadmap | Tooling track သီးခြားဖြစ်သည်။ |

## `zap add` contract

```bash
zap add <name> <version> [project-dir]
```

Command သည် `zap.toml` ကို deterministic အတိုင်း update လုပ်သည်။ Empty/whitespace ပါသော name၊ duplicate dependency name နှင့် မမှန်ကန်သော single-line requirement များကို reject လုပ်သည်။ `zap.lock` ရှိပြီးသားဖြစ်ပါက manifest ပြောင်းလဲသွားသောကြောင့် lockfile ကို ဖျက်သည်။ ထို့နောက် `zap lock` ဖြင့် canonical lockfile ကို ပြန်လည် generate လုပ်နိုင်သည်။

## Verification

Native test suite တွင် dependency ထည့်ခြင်း၊ lexicographic ordering၊ duplicate rejection၊ lockfile invalidation နှင့် CLI help exposure များကို test coverage ထည့်ထားပါသည်။ နောက်ထပ် package-manager milestone သည် reproducibility မပျက်စေဘဲ local path packages နှင့် registry-ready metadata အတွက် dependency source model တည်ဆောက်ခြင်း ဖြစ်သည်။

[English package guide](PACKAGE_EN.md)၊ [Burmese package guide](PACKAGE.md) နှင့် [ecosystem roadmap](ECOSYSTEM.md) ကို ဆက်လက်ဖတ်ရှုနိုင်ပါသည်။
