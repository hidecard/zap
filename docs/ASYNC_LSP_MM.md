# Zap Async Runtime နှင့် LSP Integration

## လက်ရှိအခြေအနေ

Zap P2 တွင် deterministic async language layer၊ bounded threaded I/O adapter နှင့် editor protocol foundation ကို ထည့်သွင်းပြီးဖြစ်ပါသည်။ Deterministic executor သည် stable Rust နှင့် ကိုက်ညီပြီး Language အနေဖြင့် `async fn`၊ deferred `Future` value နှင့် `await` expression များကို support လုပ်ပါသည်။ LSP server သည် JSON-RPC initialization၊ document synchronization၊ diagnostics၊ parser-backed hover နှင့် context-aware completion များကို ပေးပါသည်။

Deterministic language scheduling နှင့် production-oriented blocking adapter များကို သီးခြားခွဲထားပါသည်။ လက်ရှိတွင် async call သည် function body ကို run ပြီး completed `Future` value အဖြစ် ပြန်ပေးသည်။ `await` သည် ထို value ကို evaluation အတွင်း unwrap လုပ်သည်။ `delay_ticks`၊ `yield_now`၊ poll budget နှင့် runtime task limit များသည် deterministic scheduling control များကို ပေးပြီး `CancellationToken` နှင့် `Cancellable` သည် cooperative cancellation ပေးပါသည်။ `ThreadedRuntime` သည် explicitly submit လုပ်ထားသော blocking work နှင့် asynchronous file read များအတွက် bounded fixed worker set ကို ပေးပြီး deterministic language executor ကို အစားမထိုးပါ။

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
| `ThreadedRuntime::spawn_blocking(task)` | `Send + 'static` blocking adapter ကို bounded worker set ပေါ်တွင် run ပြီး wakeable join handle ပြန်ပေးသည်။ |
| `ThreadedRuntime::read_file_async(path)` | Regular file တစ်ခုကို သတ်မှတ်ထားသော byte limit အတွင်း asynchronous ဖတ်သည်။ |
| `spawn(future)` | Language-level facade အဖြစ် async expression ၏ completed `Future` value ကို ပြန်ပေးသည်။ |
| `task_join(value)` | Language-level `Future` value ကို စစ်ဆေးပြီး unwrap လုပ်သည်။ |
| `task_is_ready(value)` | Language-level task value ကို မစားသုံးဘဲ ready ဖြစ်/မဖြစ် စစ်ဆေးသည်။ |

Deterministic executor သည် external runtime dependency မရှိပါ။ `RuntimeLimits` သည် task count နှင့် poll count ကို ကန့်သတ်ပြီး `RunReport` သည် poll အရေအတွက်နှင့် ကျန် task အရေအတွက်ကို ဖော်ပြသည်။ သီးခြား `ThreadedRuntime` သည် Rust standard library ကိုသာ အသုံးပြုပြီး `ThreadRuntimeLimits` ဖြင့် worker count၊ admitted task count နှင့် maximum file-read bytes များကို ကန့်သတ်သည်။ Worker panic များကို `ThreadJoinError::WorkerPanicked` အဖြစ် ပြောင်းပေးပြီး task limit ကျော်လွန်သော admission ကို ငြင်းပယ်ကာ worker ပြီးဆုံးချိန်တွင် joiner ကို wake လုပ်သည်။ File read သည် regular file များကိုသာ ခွင့်ပြုပြီး directory သို့မဟုတ် အခြား non-file များကို ငြင်းပယ်ကာ byte limit ထက် မပိုဘဲ ဖတ်သည်။ Deterministic task များ၏ cancellation သည် cooperative ဖြစ်ပြီး blocking system call များကို admission နှင့် read-size control များဖြင့်သာ ကန့်သတ်ပါသည်။

## Production I/O နှင့် Multi-thread Scheduling

Deterministic language executor ပေါ်တွင် မ run သင့်သော blocking operation များအတွက် native runtime တွင် bounded threaded adapter ပါရှိပါသည်။ `ThreadedRuntime::new(ThreadRuntimeLimits { max_workers, max_tasks, max_read_bytes })` သည် fixed worker set တစ်ခုကို စတင်ပေးသည်။ `spawn_blocking` သည် `max_tasks` အထိသာ active job များကို လက်ခံပြီး worker ပြီးဆုံးချိန်တွင် wake လုပ်ပေးသော `ThreadJoinHandle` ကို ပြန်ပေးသည်။ Worker အတွင်း panic ဖြစ်ပါက runtime boundary ကို ဖြတ်မသွားဘဲ `WorkerPanicked` အဖြစ် ပြောင်းပေးသည်။

`read_file_async` သည် ပထမဆုံး production I/O facade ဖြစ်ပါသည်။ Metadata ကို စစ်ဆေးပြီး regular file ဖြစ်ရမည်၊ `max_read_bytes` အတွင်း ရှိရမည်ဟု သတ်မှတ်ထားသည်။ Outer result သည် scheduler failure နှင့် inner result သည် file-operation failure ကို ခွဲခြားပေးသည်။ ဤ adapter သည် explicit ဖြစ်ပြီး Zap file operation အားလုံးကို အလိုအလျောက် thread ပြောင်းမပေးသလို OS-level sandboxing သို့မဟုတ် လည်ပတ်နေပြီးသား system call ကို အတင်းအကျပ် cancel လုပ်နိုင်သည်ဟုလည်း မဆိုလိုပါ။

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

လက်ရှိ deterministic model တွင် background thread မရှိပါ။ `Future` သည် completed result ကို ထိန်းသိမ်းထားသော stable runtime value ဖြစ်ပြီး `await` သည် ၎င်းကို unwrap လုပ်ပါသည်။ ဤ slice တွင် language-level task facade သည် eager ဖြစ်ပါသည်။ `spawn(async_call())` သည် အရင် evaluation လုပ်ပြီးသား async result မှ task ပုံစံ `Future` ကို ဖန်တီးပေးသည်။ `task_join` သည် ၎င်းကို unwrap လုပ်ပြီး `task_is_ready` သည် value ကို မစားသုံးဘဲ ready အခြေအနေကို စစ်ဆေးသည်။ `Future` မဟုတ်သော value ကို await သို့မဟုတ် join လုပ်ပါက value ကို တိတ်တဆိတ် ပြောင်းလဲမည့်အစား runtime error ပြန်ပေးပါသည်။

```zap
async fn answer() -> number:
    return 42

let task = spawn(answer())
let ready = task_is_ready(task)
let value = task_join(task)
say value
```

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
| `workspace/symbol` | In-memory indexed documents များထဲမှ deterministic symbol များကို ရှာဖွေပြီး editor တွင် မဖွင့်ထားသော package module များကို explicit local import အတိုင်း လုံခြုံစွာ လိုက်လံရှာဖွေသည်။ |
| `textDocument/formatting` | Line ending၊ tab၊ trailing space နှင့် နောက်ဆုံး newline များကို normalize လုပ်သော full-document edit တစ်ခု ပြန်ပေးသည်။ |

Completion သည် fixed unfiltered list မဟုတ်တော့ဘဲ context-aware ဖြစ်ပါသည်။ ဥပမာ `async fn load():` ပါသော document ထဲတွင် `lo` ရိုက်ထားပါက completion response တွင် `load` ကို function item အဖြစ် ပြန်ပေးပါသည်။ Hover သည် source position မှ active word ကို ရှာပြီး parser ၏ `SourceSpan` ပါသော AST မှ declaration အချက်အလက်ကို ပြန်ထုတ်ပါသည်။

Workspace symbol indexing သည် ဖွင့်ထားသော file ၏ directory မှ `import app.util as util` ကဲ့သို့သော explicit local import များကို လိုက်လံရှာဖွေပြီး dotted path ကို `app/util.zp` အဖြစ် ပြောင်းလဲပါသည်။ Imported file များကို indexing မပြုမီ canonicalize လုပ်ပြီး importing directory အတွင်းတွင်သာ ရှိရမည်ဖြစ်ကာ 8 MiB အထိသာ ခွင့်ပြုပါသည်။ Invalid၊ မတွေ့ရှိသော၊ အရွယ်အစားကျော်လွန်သော၊ ဖတ်မရသော သို့မဟုတ် traversal ဆန်သော module များကို editor သို့မဟုတ် filesystem escape မဖြစ်စေရန် deterministic အတိုင်း ကျော်လွှားပါသည်။ ရှာဖွေတွေ့ရှိသော module URI များကို open document များနှင့်အတူ sorted index တစ်ခုတည်းထဲ ထည့်သွင်းသဖြင့် nested import များကို တစ်ကြိမ်သာ လိုက်လံပြီး ရလဒ်များ တည်ငြိမ်နေပါသည်။

Diagnostics များကို Zap ၏ ရှိပြီးသား lint implementation မှ ထုတ်ယူပါသည်။ ထို့ကြောင့် CLI နှင့် editor diagnostics များ၏ rules များ တူညီနေပါသည်။ Lint message တွင် source line ပါရှိပါက server သည် ၎င်းကို zero-based LSP range အဖြစ် ပြောင်းပြီး line ၏ character width အတိုင်း သတ်မှတ်ပါသည်။ Line မဖတ်နိုင်သော diagnostic များအတွက် ပထမ line ကို deterministic fallback အဖြစ် အသုံးပြုပါသည်။

## Tooling Synchronization

Formatter နှင့် LSP တို့သည် finalized async vocabulary တစ်ခုတည်းကို အသုံးပြုပါသည်။ Completion တွင် `spawn`၊ `task_join` နှင့် `task_is_ready` ကို stable description များဖြင့် ပြသပြီး VS Code TextMate grammar တွင်လည်း ထို builtins များကို callable Zap function များအဖြစ် highlight လုပ်ပါသည်။ Extension validation script သည် grammar ကို parse လုပ်ပြီး async builtin တစ်ခုခုပျောက်ဆုံးပါက package ကို reject လုပ်သဖြင့် language facade နှင့် editor asset များကြား drift မဖြစ်စေရန် ကာကွယ်ပေးပါသည်။

## လက်ကျန် P2 နယ်ပယ်

Bounded production I/O adapter နှင့် multi-thread scheduler ကို regular-file read နှင့် explicitly submitted blocking task များအတွက် အကောင်အထည်ဖော်ပြီးဖြစ်ပါသည်။ ဆက်လက်ကျန်ရှိသော နယ်ပယ်များမှာ ပိုမိုကျယ်ပြန့်သော non-blocking socket/process adapter များ၊ blocking system call များ၏ forced cancellation၊ OS-level sandboxing နှင့် network registry service deployment တို့ ဖြစ်ပါသည်။ Signed-index verification၊ deterministic cache garbage collection၊ authenticated local registry persistence၊ runtime resource limits၊ one-poll suspension၊ formatting၊ definition၊ workspace symbols နှင့် VS Code grammar/tooling synchronization အပိုင်းများကို implementation နှင့် tests ဖြင့် ပြီးစီးထားပါသည်။

Package workflow အတွက် [Burmese package guide](PACKAGE.md) နှင့် [P2 progress](P2_PROGRESS_MM.md) ကို ဖတ်ရှုနိုင်ပါသည်။ English version အတွက် [ASYNC_LSP_EN.md](ASYNC_LSP_EN.md) ကို ကြည့်ပါ။
