# Zap Async Runtime နှင့် LSP Integration

## လက်ရှိအခြေအနေ

Zap P2 တွင် deterministic async language layer နှင့် editor protocol foundation ကို ထည့်သွင်းပြီးဖြစ်ပါသည်။ Runtime သည် single-threaded ဖြစ်ပြီး stable Rust နှင့် ကိုက်ညီပါသည်။ Language အနေဖြင့် `async fn`၊ deferred `Future` value နှင့် `await` expression များကို support လုပ်ပါသည်။ LSP server သည် JSON-RPC initialization၊ document synchronization၊ diagnostics၊ parser-backed hover နှင့် context-aware completion များကို ပေးပါသည်။

Scheduling ကို deterministic အဖြစ် ထိန်းသိမ်းထားပါသည်။ လက်ရှိတွင် async call သည် function body ကို run ပြီး completed `Future` value အဖြစ် ပြန်ပေးသည်။ `await` သည် ထို value ကို evaluation အတွင်း unwrap လုပ်သည်။ Timers၊ cancellation၊ multi-thread scheduling နှင့် ပိုမိုပြည့်စုံသော suspension semantics များမှာ နောက်ထပ် runtime milestone များဖြစ်ပါသည်။

## Async Runtime

Native runtime တွင် deterministic executor operation သုံးမျိုးရှိပါသည်။

| Operation | ရည်ရွယ်ချက် |
|---|---|
| `spawn(future)` | Async task ကို deterministic task queue ထဲ ထည့်သည်။ |
| `run_until_idle()` | Task များကို spawn order အတိုင်း poll လုပ်ပြီး အားလုံးပြီးသည်အထိ run သည်။ |
| `block_on(future)` | Future တစ်ခုကို ပြီးဆုံးသည်အထိ synchronous အနေဖြင့် drive လုပ်သည်။ |

Executor သည် worker thread နှင့် external runtime dependency များကို မသုံးပါ။ ထို့ကြောင့် လက်ရှိ synchronous behavior ကို မပြောင်းလဲဘဲ နောက်ပိုင်း suspension၊ timers၊ cancellation နှင့် I/O integration များအတွက် တည်ငြိမ်သောအခြေခံ ရရှိပါသည်။

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
| `initialize` | Zap server information ကို ပြန်ပေးပြီး text synchronization၊ completion၊ diagnostics နှင့် hover capabilities များကို ကြေညာသည်။ |
| `shutdown` | အောင်မြင်သော null result ကို ပြန်ပေးသည်။ |
| `textDocument/didOpen` | Document ကို သိမ်းဆည်းပြီး deterministic source ranges ပါသော lint diagnostics ထုတ်ပေးသည်။ |
| `textDocument/didChange` | သိမ်းဆည်းထားသော document ကို အစားထိုးပြီး diagnostics အသစ် ထုတ်ပေးသည်။ |
| `textDocument/completion` | လက်ရှိ source prefix အပေါ်မူတည်၍ keyword များကို filter လုပ်ပြီး document ထဲမှ top-level `let` နှင့် function declaration များကို ထည့်ပေးသည်။ |
| `textDocument/hover` | သိမ်းဆည်းထားသော document ကို parse လုပ်ပြီး top-level function၊ class နှင့် declaration များအတွက် parser-owned metadata ပြသည်။ |

Completion သည် fixed unfiltered list မဟုတ်တော့ဘဲ context-aware ဖြစ်ပါသည်။ ဥပမာ `async fn load():` ပါသော document ထဲတွင် `lo` ရိုက်ထားပါက completion response တွင် `load` ကို function item အဖြစ် ပြန်ပေးပါသည်။ Hover သည် source position မှ active word ကို ရှာပြီး parser ၏ `SourceSpan` ပါသော AST မှ declaration အချက်အလက်ကို ပြန်ထုတ်ပါသည်။

Diagnostics များကို Zap ၏ ရှိပြီးသား lint implementation မှ ထုတ်ယူပါသည်။ ထို့ကြောင့် CLI နှင့် editor diagnostics များ၏ rules များ တူညီနေပါသည်။ Lint message တွင် source line ပါရှိပါက server သည် ၎င်းကို zero-based LSP range အဖြစ် ပြောင်းပြီး line ၏ character width အတိုင်း သတ်မှတ်ပါသည်။ Line မဖတ်နိုင်သော diagnostic များအတွက် ပထမ line ကို deterministic fallback အဖြစ် အသုံးပြုပါသည်။

## လက်ကျန် P2 နယ်ပယ်

ယခု foundation သည် full workspace language server သို့မဟုတ် production asynchronous I/O runtime မဟုတ်သေးပါ။ Timers၊ cancellation၊ resource limits၊ richer suspension points၊ formatting၊ go-to-definition၊ document symbols၊ workspace-aware package/module indexing နှင့် signed indexes၊ range solving၊ cache garbage collection၊ server-side persistence ကဲ့သို့ registry features များမှာ ကျန်ရှိနေပါသည်။

Package workflow အတွက် [Burmese package guide](PACKAGE.md) နှင့် [P2 progress](P2_PROGRESS_MM.md) ကို ဖတ်ရှုနိုင်ပါသည်။ English version အတွက် [ASYNC_LSP_EN.md](ASYNC_LSP_EN.md) ကို ကြည့်ပါ။
