# Zap P2 Progress — Ecosystem Foundation

## လက်ရှိအခြေအနေ

Zap P1 Language Core ကို `v1.0.0` အဖြစ် release ပြုလုပ်ပြီးပါပြီ။ ယခု P2 ကို remote registry မလိုအပ်သေးသော deterministic local package-manager foundation ဖြင့် စတင်ထားပါသည်။

| Milestone | အခြေအနေ | မှတ်ချက် |
|---|---|---|
| Manifest dependency declarations | ပြီးစီး | `[dependencies]` entries များနှင့် local path specification များကို parse နှင့် validate လုပ်သည်။ |
| Canonical lockfile | ပြီးစီး | `zap.lock` ကို package/dependency order တည်ငြိမ်စွာ generate လုပ်ပြီး local path များကို canonical ပုံစံဖြင့်ရေးသည်။ |
| `zap add` command | ပြီးစီး | String-valued dependency ထည့်ခြင်း၊ dependency section sort လုပ်ခြင်း၊ duplicate reject လုပ်ခြင်းနှင့် lockfile invalidate လုပ်ခြင်းတို့ ပါဝင်သည်။ |
| Registry-ready package metadata | ပြီးစီး | `description`၊ `authors`၊ `license`၊ `repository` နှင့် 64-character hexadecimal SHA-256 `checksum` fields များကို deterministic validate လုပ်သည်။ |
| Registry index, HTTPS transport နှင့် cache foundation | Foundation အဆင့် ပြီးစီး | JSON index validation၊ exact နှင့် version-range selection၊ local/HTTPS index နှင့် artifact transport၊ deterministic content-addressed cache၊ SHA-256 enforcement နှင့် offline reuse ကို support လုပ်သည်။ Remote publishing validation နှင့် checksum-verified archive publishing ကိုလည်း ထည့်သွင်းထားသည်။ Signed indexes၊ cache garbage collection၊ server-side persistence နှင့် full upload authentication များမှာ နောက်ထပ်အလုပ်များ ဖြစ်သည်။ |
| `zap install` | ပြီးစီး | လက်ရှိ manifest နှင့် canonical lockfile ကို validate လုပ်ပြီး registry သတ်မှတ်ထားပါက registry entry နှင့် checksum-verified cache ကို စစ်ဆေး/ဖြည့်ပေးသည်။ |
| `zap update` | ပြီးစီး | Canonical lockfile ကို deterministic ပြန်လည် generate လုပ်ပြီး local graph နှင့် configured registry/cache integrity ကို validate လုပ်သည်။ |
| Async runtime နှင့် language syntax | Foundation အဆင့် ပြီးစီး | Deterministic single-thread executor၊ `spawn`၊ `run_until_idle` နှင့် `block_on` ကို internal အနေဖြင့် အသုံးပြုနိုင်သည်။ `async fn`၊ `await`၊ deterministic `Future` values နှင့် evaluator integration ကို ထည့်သွင်းပြီးဖြစ်သည်။ Timers၊ cancellation နှင့် multi-thread scheduling များမှာ နောက်ထပ်အလုပ်များဖြစ်သည်။ |
| LSP/editor integration | Foundation အဆင့် ပြီးစီး | `zap lsp` မှ stdio JSON-RPC framing၊ initialize/shutdown၊ text synchronization၊ parser-backed diagnostics၊ top-level declaration များအတွက် parser-span hover နှင့် source-prefix/context-aware completion ကို ပေးသည်။ Formatting၊ go-to-definition နှင့် workspace indexing များမှာ roadmap ဖြစ်သည်။ |

## Local install/update contract

```bash
zap install [project-dir]
zap update [project-dir]
```

`zap install` သည် project manifest နှင့် lockfile အတွက် validation-only command ဖြစ်သည်။ `ZAP_REGISTRY_INDEX` သတ်မှတ်ထားပါက exact registry entry နှင့် checksum-verified cache ကိုပါ စစ်ဆေးသည်။ `ZAP_OFFLINE=1` သတ်မှတ်ထားပါက cache ထဲရှိပြီး checksum မှန်သော package များကိုသာ အသုံးပြုကာ download အသစ် မလုပ်ပါ။ `zap update` သည် canonical ordering ဖြင့် `zap.lock` ကို ပြန်လည် generate လုပ်ပြီး registry/cache checks များကိုလည်း ပြုလုပ်သည်။ Local path dependency များအတွက် nested manifest များကို lexicographic order ဖြင့် recursively validate လုပ်ပြီး lockfile မရေးမီ cycle များကို reject လုပ်သည်။

## `zap add` contract

```bash
zap add <name> <version> [project-dir]
```

Command သည် `zap.toml` ကို deterministic အတိုင်း update လုပ်သည်။ Empty/whitespace ပါသော name၊ duplicate dependency name နှင့် မမှန်ကန်သော single-line requirement များကို reject လုပ်သည်။ `zap.lock` ရှိပြီးသားဖြစ်ပါက manifest ပြောင်းလဲသွားသောကြောင့် lockfile ကို ဖျက်သည်။ ထို့နောက် `zap lock` ဖြင့် canonical lockfile ကို ပြန်လည် generate လုပ်နိုင်သည်။

## Verification

Native test suite တွင် dependency ထည့်ခြင်း၊ lexicographic ordering၊ duplicate rejection၊ lockfile invalidation၊ install validation၊ update regeneration၊ idempotence၊ stale-lock rejection၊ CLI help exposure၊ valid local package၊ missing local manifest၊ nested local package၊ deterministic cycle diagnostics၊ async parsing/evaluation၊ Future unwrap၊ LSP capability negotiation၊ context-filtered completion နှင့် parser-backed hover များကို test coverage ထည့်ထားပါသည်။ `name = { path = "../local-lib" }` ပုံစံကို support လုပ်ပြီး path ကို သုံးစွဲသည့် project အပေါ်မူတည်၍ resolve လုပ်သည်။ Local package တွင် package name နှင့် version ပါသော `zap.toml` ရှိရမည်ဖြစ်ပြီး `zap.lock` တွင် canonical ပုံစံဖြင့် သိမ်းဆည်းသည်။ Local path dependency များကို sorted order ဖြင့် depth-first traverse လုပ်သည်။ Active traversal stack ထဲတွင် canonical path ထပ်ပေါ်လာပါက `dependency cycle detected: left -> right -> left` ကဲ့သို့ error ပြန်ပေးသည်။ Manifest metadata contract၊ local/HTTPS registry transport၊ content-addressed cache၊ checksum enforcement၊ validated remote publishing နှင့် deterministic version-range solving foundation များသည် ပြီးစီးပြီးဖြစ်သည်။ Signed index၊ cache garbage collection၊ registry server-side persistence နှင့် full package upload authentication များသည် နောက်ထပ် P2 အလုပ်များ ဖြစ်သည်။ Lockfile generate/update မလုပ်မီ root နှင့် nested local package များအားလုံးတွင် metadata validation ပြုလုပ်သည်။ Async language layer သည် async call များကို deterministic `Future` value အဖြစ် ပြုလုပ်ပြီး `await` ဖြင့် resolve လုပ်သည်။ Timers၊ cancellation နှင့် ပိုမိုပြည့်စုံသော suspension semantics များမှာ နောက်ထပ်အလုပ်များဖြစ်သည်။ LSP သည် stdio Content-Length JSON-RPC ကို အသုံးပြုပြီး Zap lint diagnostics၊ parser source spans နှင့် active document မှ completion candidates များကို အသုံးပြုသည်။ Complete workspace language server မဟုတ်သေးပါ။

[English package guide](PACKAGE_EN.md)၊ [Burmese package guide](PACKAGE.md) နှင့် [ecosystem roadmap](ECOSYSTEM.md) ကို ဆက်လက်ဖတ်ရှုနိုင်ပါသည်။
