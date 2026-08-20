# Zap Async Runtime နှင့် LSP Integration

## လက်ရှိအခြေအနေ

Zap P2 တွင် deterministic async language layer နှင့် editor protocol foundation ကို ထည့်သွင်းပြီးဖြစ်ပါသည်။ Runtime သည် single-threaded ဖြစ်ပြီး stable Rust နှင့် ကိုက်ညီပါသည်။ Language အနေဖြင့် `async fn`၊ deferred `Future` value နှင့် `await` expression များကို support လုပ်ပါသည်။ LSP server သည် JSON-RPC initialization၊ document synchronization၊ diagnostics၊ parser-backed hover နှင့် context-aware completion များကို ပေးပါသည်။

Scheduling ကို deterministic အဖြစ် ထိန်းသိမ်းထားပါသည်။ လက်ရှိတွင် async call သည် function body ကို run ပြီး completed `Future` value အဖြစ် ပြန်ပေးသည်။ `await` သည် ထို value ကို evaluation အတွင်း unwrap လုပ်သည်။ `delay_ticks`၊ `yield_now`၊ poll budget နှင့် runtime task limit များသည် deterministic scheduling control များကို ပေးပြီး `CancellationToken` နှင့် `Cancellable` သည် cooperative cancellation ပေးပါသည်။ Multi-thread scheduling နှင့် external I/O integration များမှာ foundation အပြင်ဘက်တွင် ရှိပါသည်။

## Async Runtime

Native runtime တွင် deterministic executor operation သုံးမျိုးရှိပါသည်။

| Operation | ရည်ရွယ်ချက် |
|---|---|
| `spawn(future)` | Async task ကို deterministic task queue ထဲ ထည့်သည်။ |
| `spawn_cancellable(future)` | `CancellationToken` ဖြင့် ထိန်းချုပ်နိုင်သော task ထည့်သည်။ |
| `run_until_idle()` | လက်ရှိ queue pass အတွင်း task များကို spawn order အတိုင်း poll လုပ်သည်။ |
| `block_on(future)` | Future တစ်ခုကို ပြီးဆုံးသည်အထိ synchronous အနေဖြင့် drive လုပ်သည်။ |
| `delay_ticks(n)` | Poll count အပေါ်အခြေခံသော deterministic delay future ပြန်ပေးသည်။ |
| `yield_now()` | တစ်ကြိမ် suspend လုပ်ပြီး နောက် poll တွင် ပြန်လည်လုပ်ဆောင်သည်။ |
| `spawn_limited(future)` | သတ်မှတ်ထားသော maximum task count ကို ကျော်လွန်မသွားအောင် ထိန်းသည်။ |
| `run_with_budget(n)` | Poll အကြိမ်ရေ `n` အထိသာ လုပ်ပြီး deterministic `RunReport` ပြန်ပေးသည်။ |

Executor သည် worker thread နှင့် external runtime dependency များကို မသုံးပါ။ `RuntimeLimits` သည် task count နှင့် poll count ကို ကန့်သတ်ပြီး `RunReport` သည် poll အရေအတွက်နှင့် ကျန် task အရေအတွက်ကို ဖော်ပြသည်။ ထို့ကြောင့် လက်ရှိ synchronous behavior ကို မပြောင်းလဲဘဲ နောက်ပိုင်း I/O integration များအတွက် တည်ငြိမ်သောအခြေခံ ရရှိပါသည်။ Cancellation သည် cooperative ဖြစ်ပြီး cancelled task သည် ၎င်း၏ inner future ကို ဆက်လက်မ poll ဘဲ ပြီးဆုံးပါသည်။

## Async Language Syntax

`fn` ရှေ့တွင် `async` ထည့်ပြီး asynchronous function ကြေညာနိုင်ပါသည်။ Function call သည် ပုံမှန် result အစား `Future` value ပြန်ပေးပြီး completed result ရယူရန် `await` ကို အသုံးပြုရပါသည်။

```zap
async fn load_version() -> number:
    return 7

let pending = load_version()
let version: number = await pending
say version
```

`async` function သည် ပုံမှန် function ကဲ့သို့ parameter နှင့် return-type annotation များကို အသုံးပြုနိုင်ပါသည်။ Evaluator သည် runtime function ပေါ်တွင် async declaration flag ကို ထိန်းသိမ်းပြီး declared result ကို validate လုပ်ကာ `Future` ထဲ wrap လုပ်ပါသည်။

`await` သည် expression ဖြစ်သောကြောင့် declaration၊ assignment၊ return expression သို့မဟုတ် nested call များထဲတွင် အသုံးပြုနိုင်ပါသည်။

```zap
async fn answer() -> number:
    return 42

let value = await answer()
say value + 1
```

လက်ရှိ deterministic model တွင် background thread မရှိပါ။ `Future` သည် completed result ကို ထိန်းသိမ်းထားသော stable runtime value ဖြစ်ပြီး `await` သည် ၎င်းကို unwrap လုပ်ပါသည်။ `Future` မဟုတ်သော value ကို await လုပ်ပါက value ကို တိတ်တဆိတ် ပြောင်းလဲမည့်အစား runtime error ပြန်ပေးပါသည်။

## LSP Server

Editor server ကို အောက်ပါ command ဖြင့် စတင်နိုင်ပါသည်။

```bash
zap lsp
```

Server သည် standard input/output မှတစ်ဆင့် `Content-Length` header ဖြင့် frame လုပ်ထားသော JSON-RPC message များကို ဆက်သွယ်ပါသည်။

| Message | အပြုအမူ |
|---|---|
| `initialize` | Zap server information ကို ပြန်ပေးပြီး text synchronization၊ completion၊ diagnostics၊ hover၊ definition နှင့် workspace-symbol capabilities များကို ကြေညာသည်။ |
| `shutdown` | အောင်မြင်သော null result ကို ပြန်ပေးသည်။ |
| `textDocument/didOpen` | Document ကို သိမ်းဆည်းပြီး deterministic source ranges ပါသော lint diagnostics ထုတ်ပေးသည်။ |
| `textDocument/didChange` | သိမ်းဆည်းထားသော document ကို အစားထိုးပြီး diagnostics အသစ် ထုတ်ပေးသည်။ |
| `textDocument/completion` | လက်ရှိ source prefix အပေါ်မူတည်၍ keyword များကို filter လုပ်ပြီး document ထဲမှ top-level `let` နှင့် function declaration များကို ထည့်ပေးသည်။ |
| `textDocument/hover` | သိမ်းဆည်းထားသော document ကို parse လုပ်ပြီး top-level function၊ class နှင့် declaration များအတွက် parser-owned metadata ပြသည်။ |
| `textDocument/definition` | Referenced top-level declaration ကို parser-span source range သို့ resolve လုပ်သည်။ |
| `workspace/symbol` | In-memory indexed documents အားလုံးထဲမှ top-level declaration symbols များကို deterministic ရှာဖွေသည်။ |
| `textDocument/formatting` | Line ending၊ tab၊ trailing space နှင့် နောက်ဆုံး newline များကို normalize လုပ်သော full-document edit တစ်ခု ပြန်ပေးသည်။ |

Completion သည် fixed unfiltered list မဟုတ်တော့ဘဲ context-aware ဖြစ်ပါသည်။ ဥပမာ `async fn load():` ပါသော document ထဲတွင် `lo` ရိုက်ထားပါက completion response တွင် `load` ကို function item အဖြစ် ပြန်ပေးပါသည်။ Hover သည် source position မှ active word ကို ရှာပြီး parser ၏ `SourceSpan` ပါသော AST မှ declaration အချက်အလက်ကို ပြန်ထုတ်ပါသည်။

Diagnostics များကို Zap ၏ ရှိပြီးသား lint implementation မှ ထုတ်ယူပါသည်။ ထို့ကြောင့် CLI နှင့် editor diagnostics များ၏ rules များ တူညီနေပါသည်။ Lint message တွင် source line ပါရှိပါက server သည် ၎င်းကို zero-based LSP range အဖြစ် ပြောင်းပြီး line ၏ character width အတိုင်း သတ်မှတ်ပါသည်။ Line မဖတ်နိုင်သော diagnostic များအတွက် ပထမ line ကို deterministic fallback အဖြစ် အသုံးပြုပါသည်။

## လက်ကျန် P2 နယ်ပယ်

ယခု foundation သည် production asynchronous I/O runtime သို့မဟုတ် multi-thread scheduler မဟုတ်သေးပါ။ External I/O integration၊ nested symbol indexing၊ module-aware package indexing နှင့် network registry service deployment များမှာ ဆက်လက်တိုးချဲ့နိုင်သော နယ်ပယ်များဖြစ်ပါသည်။ Signed-index verification၊ deterministic cache garbage collection၊ authenticated local registry persistence၊ runtime resource limits၊ one-poll suspension၊ formatting၊ definition နှင့် workspace symbols များကို implementation နှင့် tests ဖြင့် ပြီးစီးထားပါသည်။

Package workflow အတွက် [Burmese package guide](PACKAGE.md) နှင့် [P2 progress](P2_PROGRESS_MM.md) ကို ဖတ်ရှုနိုင်ပါသည်။ English version အတွက် [ASYNC_LSP_EN.md](ASYNC_LSP_EN.md) ကို ကြည့်ပါ။
