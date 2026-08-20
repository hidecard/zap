# Zap P3 Roadmap — Production Language နှင့် Tooling

## ရည်ရွယ်ချက်

P3 သည် P3.1 module/workspace architecture ပါဝင်သော စစ်ဆေးအတည်ပြုပြီးသော `v2.0.1` maintenance release နောက်ပိုင်း ဆက်လက်လုပ်ဆောင်နေသော အဆင့်ဖြစ်ပါသည်။ ရည်ရွယ်ချက်မှာ Zap ကို web, AI, automation နှင့် systems-oriented project များအတွက် production-ready development platform တစ်ခုအဖြစ် တိုးချဲ့ရန် ဖြစ်ပြီး deterministic behavior နှင့် Rust 1.75 compatibility ကို မပျက်စေရန် ဖြစ်ပါသည်။

## လက်ရှိအခြေခံအခြေအနေ

P2 တွင် native runtime၊ deterministic registry transport နှင့် resolution၊ signed-index verification၊ cache lifecycle controls၊ async/Future foundation၊ cancellation နှင့် suspension primitives၊ diagnostics၊ hover၊ completion၊ formatting၊ definition နှင့် workspace symbols ပါသော LSP foundation တို့ ပြီးစီးထားပါသည်။

## ဦးစားပေး milestone များ

| Milestone | အကြောင်းအရာ | အောင်မြင်မှုစံနှုန်း | အခြေအနေ |
|---|---|---|---|
| P3.1 | Module နှင့် workspace architecture | ရှင်းလင်းသော module/import syntax၊ deterministic search paths၊ duplicate/cycle diagnostics နှင့် cross-platform workspace tests | ပြီးစီး |
| P3.2 | Structured error model | Native `raise`/`try`/`catch` propagation၊ တည်ငြိမ်သော diagnostics၊ catch binding restoration၊ re-raise support နှင့် deterministic runtime behavior | ပြီးစီး |
| P3.3 | Production standard library | HTTP client/server primitives၊ URL handling၊ process execution boundaries နှင့် လုံခြုံသော environment/configuration APIs | ပြီးစီး |
| P3.4 | Async I/O integration | timers၊ sockets၊ files၊ cancellation၊ backpressure နှင့် resource budgets အတွက် deterministic runtime interfaces | စီစဉ်ထား |
| P3.5 | Type-system productivity | Generic functions/collections၊ ပိုမိုကောင်းမွန်သော inference၊ pattern matching နှင့် exhaustiveness diagnostics | စီစဉ်ထား |
| P3.6 | Tooling နှင့် language server | Full formatter၊ workspace indexing၊ rename/references၊ import assistance၊ semantic tokens နှင့် project-aware diagnostics | စီစဉ်ထား |
| P3.7 | Quality နှင့် release engineering | Benchmarks၊ fuzz/property tests၊ security audit၊ reproducible artifacts နှင့် cross-platform install verification | စီစဉ်ထား |

## P3.2 အကောင်အထည်ဖော်ပြီးစီးမှု

P3.2 တွင် `raise <expression>` နှင့် same-level `try`/`catch <binding>:` syntax များကို AST runtime ထဲသို့ ထည့်သွင်းထားပါသည်။ Raised value များသည် function၊ loop၊ nested block နှင့် module execution များအတွင်းမှ catch မလုပ်မချင်း propagate ဖြစ်ပါသည်။ Catch ပြီးဆုံးပါက catch binding သည် မူလ variable value ကို ပြန်လည် restore လုပ်ပြီး catch အတွင်း re-raise လုပ်သည့်အခါတွင်လည်း ဤစည်းမျဉ်းကို ထိန်းသိမ်းထားပါသည်။ Bare `raise`၊ catch clause မရှိခြင်း၊ binding မမှန်ခြင်းနှင့် catch body မရှိခြင်းတို့အတွက် stable parser diagnostics များ ထုတ်ပေးပါသည်။ Uncaught value သည် process boundary တွင် `raised error: <value>` ဟူသော deterministic diagnostic အဖြစ် ရောက်ရှိပါသည်။ Rust 1.75 compatibility နှင့် AST/legacy execution parity ကိုလည်း ထိန်းသိမ်းထားပါသည်။

Parser/evaluator focused coverage များတွင် expression လိုအပ်သော raise syntax၊ uncaught flow၊ nested catch shadow restoration၊ ပုံမှန်ပြီးဆုံးမှု၊ text မဟုတ်သော payload များနှင့် re-raise behavior တို့ကို စစ်ဆေးထားပါသည်။ နောက် release artifact မထုတ်မီ complete native suite နှင့် strict release checks များကို ဆက်လက်အောင်မြင်ရန် လိုအပ်ပါသည်။

## ပထမဆုံးအကောင်အထည်ဖော်မည့် P3.1

P3.3 အကောင်အထည်ဖော်မှုတွင် ကန့်သတ်ထားသော `url_parse`၊ `url_encode`၊ `url_decode`၊ `http_get`၊ `http_request` နှင့် shell မသုံးသော `process_run` builtin များကို ထည့်သွင်းထားပါသည်။ Safe configuration အပိုင်းတွင် `env_get`၊ `config_dir` နှင့် `config_path` ကိုလည်း ထည့်သွင်းထားပြီး default environment access၊ platform-aware configuration directory နှင့် traversal မဖြစ်စေရန် ကန့်သတ်ထားသော file path များကို ပေးနိုင်ပါသည်။ Local server အပိုင်းတွင် `http_serve_once` ကို ထည့်သွင်းထားပြီး loopback တွင် bind လုပ်ကာ request တစ်ခုတည်းကိုသာ serve ပြီး request၊ response နှင့် wait time limit များကို ထိန်းချုပ်ထားပါသည်။ ဤ API များကို deterministic standard-library catalog တွင် မှတ်ပုံတင်ပြီး URL၊ process၊ HTTP၊ local server နှင့် configuration safety ကန့်သတ်ချက်များကို documentation တွင် ဖော်ပြထားပါသည်။ Cross-platform hardening တွင် native path separator များနှင့် Windows JSON file fixture escaping ကို ပြင်ဆင်ပြီးဖြစ်ပါသည်။ Linux native suite tests 235 ခု pass ဖြစ်ပြီး Windows နှင့် macOS အတွက် GitHub Actions matrix သည် အဓိက verification gate အဖြစ် ဆက်လက်ရှိပါသည်။

P3.1 ကို ပထမဦးစားပေးအဖြစ် ရွေးချယ်ထားခြင်းမှာ modules နှင့် workspaces များသည် reusable web, AI နှင့် standard-library packages များအတွက် အခြေခံလိုအပ်ချက်ဖြစ်သောကြောင့် ဖြစ်ပါသည်။ P2 တွင်ရှိပြီးသား parser-owned spans နှင့် project resolver ကို အသုံးပြုပြီး module model အသစ်နှစ်ခု မဖြစ်စေရန် တိုးချဲ့မည်ဖြစ်ပါသည်။

P3.1 implementation တွင် optional `[module]` manifest section ကို သတ်မှတ်ပြီး relative `root` နှင့် explicit `.zp` `entries` များကို validate ပြုလုပ်ထားပါသည်။ Absolute path၊ traversal၊ မရှိသော file၊ duplicate entry နှင့် unknown module field များကို CLI stable diagnostics ဖြင့် ပယ်ချပါသည်။ Explicit `module` declaration နှင့် `import ... as ...` path များကို module root အောက်တွင် deterministic resolve လုပ်ပြီး imported files များကို source order အတိုင်း traverse ပြုလုပ်ပါသည်။ Circular dependency ဖြစ်ပါက cycle တစ်ခုလုံးကို diagnostic ထဲတွင် ဖော်ပြပါသည်။ LSP သည် module declaration နှင့် import alias များကို hover၊ definition၊ completion နှင့် workspace symbol အတွက် index လုပ်ပါသည်။ Cross-platform workspace integration tests များသည် valid graph၊ nested import၊ stable cycle diagnostic နှင့် legacy compatibility များကို စစ်ဆေးပြီး P3.1 acceptance ပြီးစီးပါသည်။ P2 မှ local path dependencies နှင့် lockfile behavior များကို မပြောင်းလဲပါ။

## Engineering စည်းမျဉ်းများ

Zap P3 သည် stable Rust 1.75 နှင့် compatible ဖြစ်ရမည်၊ Edition 2024-only dependencies မသုံးရမည်၊ ordering နှင့် diagnostics များကို deterministic ဖြစ်အောင် ထိန်းသိမ်းရမည်၊ security-sensitive operations များကို explicit bounds ဖြင့် ကာကွယ်ရမည်။ Milestone တိုင်းတွင် focused regression tests၊ complete native tests၊ formatting၊ GitHub strict Clippy နှင့် Linux/Windows/macOS verification များ အောင်မြင်ရမည်ဖြစ်ပြီး English/Burmese documentation ကို update လုပ်ရမည်။

## Release policy

ရွေးချယ်ထားသော milestone အတွက် acceptance checklist၊ clean working tree၊ native tests၊ strict Clippy၊ Linux၊ Windows နှင့် macOS verification များ အားလုံး အောင်မြင်ပြီးမှသာ P3 release tag တင်မည်ဖြစ်ပါသည်။ ပထမ P3 release version ကို P3.1 scope အကောင်အထည်ဖော်ပြီး review ပြီးနောက် သတ်မှတ်မည်ဖြစ်ပါသည်။
