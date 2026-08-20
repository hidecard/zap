# Zap v0.6.0 Roadmap

**Status:** In development  
**Target:** Reliable native foundation for Web, AI, Mobile and IoT applications

Zap v0.6.0 သည် feature အများကြီးကို တစ်ခါတည်း ထည့်သွင်းသည့် release မဟုတ်ဘဲ v0.5.0 native runtime ကို real-world application များအတွက် ပိုမိုအသုံးဝင်၊ စမ်းသပ်ရလွယ်ကူပြီး cross-platform ဖြစ်စေမည့် foundation release ဖြစ်သည်။

## v0.6.0 တွင် အကောင်အထည်ဖော်ပြီးသောအရာများ

Native runtime တွင် `now()`၊ `sleep(milliseconds)`၊ `env(name)`၊ `has_env(name)`၊ `exists(path)`၊ `path_join()`၊ `basename()`၊ `dirname()`၊ `pow()` နှင့် `sqrt()` built-ins များ ထည့်သွင်းထားသည်။ ထို့အပြင် `zap build [dir]` command သည် project manifest နှင့် entry file ကို စစ်ဆေးပြီး build validation result ပြသနိုင်သည်။ `zap --version` သည် `0.6.0` ကို ပြသမည်ဖြစ်သည်။

Optional type annotation ၏ ပထမအဆင့်အနေဖြင့် variable declaration တွင် အောက်ပါ syntax ကို အသုံးပြုနိုင်သည်။

```zap
let name: text = "Zap"
let count: number = 3
let enabled: bool = true
```

လက်ရှိ validation သည် `text`၊ `number`၊ `bool`၊ `list`၊ `map`၊ `none` နှင့် `any` တို့ကို စစ်ဆေးပေးသည်။ Type မကိုက်ညီပါက program မလုပ်ဆောင်မီ error ပြန်ပေးမည်။

## OOP Foundation — Implemented

v0.6.0 native runtime တွင် class-based OOP foundation ကို အကောင်အထည်ဖော်ပြီးဖြစ်သည်။ လက်ရှိ support အတွင်း `class` declaration၊ `new()` object creation၊ `init()` constructor၊ `self` receiver၊ property read/write၊ method arguments၊ single inheritance နှင့် method override ပါဝင်သည်။

```zap
class Animal:
    fn speak(self):
        return "sound"

class Dog extends Animal:
    fn speak(self):
        return "woof"

let dog = new("Dog")
say dog.speak()
```

OOP boundary တွင် interfaces၊ abstract classes၊ private modifiers၊ generics နှင့် multiple inheritance များ မပါဝင်သေးပါ။ OOP integration tests နှင့် beginner lesson ကို release documentation ထဲတွင် ထည့်သွင်းထားသည်။

## Standard Library ဦးစားပေးအစီအစဉ်

| အဆင့် | Module/API | ရည်ရွယ်ချက် |
|---|---|---|
| P0 | `path_join`, `basename`, `dirname`, `exists` | Cross-platform file path နှင့် file existence စစ်ဆေးမှု |
| P0 | `now`, `sleep` | Timestamp နှင့် basic timing |
| P0 | `env`, `has_env` | Environment configuration |
| P0 | `pow`, `sqrt` | Basic numeric utilities |
| P1 | `http.get` | Web နှင့် AI API client အခြေခံ |
| P1 | structured file errors | File operation failure များကို error kind ဖြင့် ပြသခြင်း |
| P2 | HTTP server | Web framework များအတွက် နောက် release foundation |

## Concurrency Roadmap

v0.6.0 တွင် concurrency API ကို တစ်ခါတည်း အပြည့်အစုံထည့်မည့်အစား design နှင့် runtime boundary ကို အရင်သတ်မှတ်မည်။ ပထမဆုံး အကောင်အထည်ဖော်မည့် model သည် shared mutable memory မဟုတ်ဘဲ `async`/`await`၊ task နှင့် channel အခြေခံ model ဖြစ်မည်။

```zap
async fn fetch_data(url: text) -> text:
    let response = await http.get(url)
    return response.body
```

အထက်ပါ syntax သည် roadmap proposal ဖြစ်ပြီး လက်ရှိ stable v0.6.0 native runtime တွင် `async`၊ `await` နှင့် HTTP client ကို မထည့်သွင်းသေးပါ။ Runtime API တည်ငြိမ်ပြီး test coverage ပြည့်စုံမှသာ အသုံးပြုနိုင်သော syntax အဖြစ် ပြောင်းလဲမည်။

### မထည့်သွင်းသေးသော concurrency အရာများ

Threads၊ locks၊ shared mutable state၊ actor framework နှင့် parallel CPU execution တို့ကို v0.6.0 ၏ ပထမအဆင့်တွင် မထည့်သွင်းသေးပါ။ ဤအရာများသည် memory model၊ cancellation နှင့် failure propagation သတ်မှတ်ချက်များ တည်ငြိမ်ပြီးနောက် v0.7.0 သို့မဟုတ် နောက်ပိုင်းတွင် ဆက်လက်လုပ်ဆောင်မည်။

## CLI နှင့် Quality Workflow

```text
zap init <dir>       project scaffold ဖန်တီးခြင်း
zap check [dir]      zap.toml နှင့် main file စစ်ဆေးခြင်း
zap build [dir]      build-ready project validation
zap fmt <file.zp>    source format ပြုလုပ်ခြင်း
zap test [dir]       *_test.zp များကို recursive run ပြုလုပ်ခြင်း
zap <file.zp>        Zap program run ပြုလုပ်ခြင်း
```

နောက်ထပ် quality tooling အဖြစ် `zap lint`၊ `zap check --json`၊ `zap test --watch` နှင့် `zap doc` တို့ကို v0.6.x အတွင်း ထည့်သွင်းရန် စီစဉ်ထားသည်။

## Release Acceptance Criteria

| Area | လက်ခံစံနှုန်း |
|---|---|
| Runtime | Linux၊ Windows နှင့် macOS ARM64 တွင် native binary run ရမည် |
| Standard library | Path separator၊ environment နှင့် timing behavior များ cross-platform ဖြစ်ရမည် |
| Type checking | မှန်ကန်သော annotation နှင့် မမှန်ကန်သော annotation နှစ်မျိုးစလုံး test ရှိရမည် |
| OOP | Class၊ object၊ constructor၊ property၊ method နှင့် inheritance tests များ pass ဖြစ်ရမည် |
| CLI | `init`၊ `check`၊ `build`၊ `fmt` နှင့် `test` commands များ documented ဖြစ်ရမည် |
| Tests | Native integration tests အားလုံး pass ဖြစ်ရမည် |
| Docs | Implemented feature နှင့် roadmap proposal ကို မရောထွေးဘဲ Burmese guides တွင် ဖော်ပြရမည် |
| Packaging | Binary archive နှင့် SHA-256 checksum များ ထုတ်ပေးနိုင်ရမည် |

## v0.7.0 သို့ ရွှေ့ထားသောအရာများ

Package registry၊ lockfile၊ generics၊ full static type checker၊ HTTP server framework၊ database modules၊ FFI၊ bytecode compiler နှင့် native mobile/device bindings များကို v0.7.0 သို့မဟုတ် နောက်ပိုင်း release များတွင် ဆက်လက်လုပ်ဆောင်မည်။

Zap ၏ ရည်ရွယ်ချက်မှာ syntax ကို ရိုးရှင်းစွာ ထိန်းသိမ်းပြီး standard library၊ tooling နှင့် native runtime ကို တဖြည်းဖြည်း ခိုင်မာစေရန် ဖြစ်သည်။
