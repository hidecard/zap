# Zap Async Runtime နှင့် LSP Integration

## လက်ရှိအခြေအနေ

Zap P2 တွင် deterministic async language layer၊ bounded threaded I/O adapter နှင့် editor protocol foundation ကို ထည့်သွင်းပြီးဖြစ်ပါသည်။ Deterministic executor သည် stable Rust နှင့် ကိုက်ညီပြီး Language အနေဖြင့် `async fn`၊ context-owned `ScheduledFuture` value နှင့် `await` expression များကို support လုပ်ပါသည်။ LSP server သည် JSON-RPC initialization၊ document synchronization၊ diagnostics၊ parser-backed hover၊ semantic rename edit နှင့် context-aware completion များကို ပေးပါသည်။ M3-LSP-01 သည် editor surface များကို canonical AST၊ lexer span၊ async facade နှင့် standard-library catalog တို့နှင့် တစ်ပြေးညီ ထိန်းသိမ်းပါသည်။

Deterministic language scheduling နှင့် production-oriented blocking adapter များကို သီးခြားခွဲထားပါသည်။ Async call သည် caller ၏ `RuntimeState` ထဲတွင် result ကို schedule လုပ်ပြီး context-owned `ScheduledFuture` ပြန်ပေးသည်။ `await` သည် executor ကို drive လုပ်ပြီး ထို value ကို unwrap လုပ်သည်။ `delay_ticks`၊ `yield_now`၊ poll budget နှင့် runtime task limit များသည် deterministic scheduling control များကို ပေးပြီး `CancellationToken`၊ `Cancellable` နှင့် language `task_cancel` API သည် cooperative cancellation ပေးပါသည်။ `ThreadedRuntime` သည် explicitly submit လုပ်ထားသော blocking work နှင့် asynchronous file read များအတွက် bounded fixed worker set ကို ပေးပြီး deterministic language executor ကို အစားမထိုးပါ။

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
| `ThreadedRuntime::tcp_exchange(address, request)` | Response byte cap နှင့် deadline ပါသော bounded non-blocking TCP request/response exchange ကို လုပ်ဆောင်သည်။ |
| `ThreadedRuntime::process_async(command, arguments)` | stdout/stderr capture limit၊ hard deadline နှင့် structured output ပါသော process ကို asynchronous run လုပ်သည်။ |
| `spawn(future)` | Language-level facade အဖြစ် context-owned `ScheduledFuture` ကို ထိန်းသိမ်း သို့မဟုတ် schedule လုပ်သည်။ |
| `task_join(value)` | Context executor ကို drive လုပ်ပြီး language task result ကို consume လုပ်သည်။ |
| `task_is_ready(value)` | Language task value ကို မစားသုံးဘဲ၊ poll မလုပ်ဘဲ ready ဖြစ်/မဖြစ် စစ်ဆေးသည်။ |
| `task_cancel(value)` | Pending language task အတွက် cooperative cancellation request ပြုလုပ်ပြီး လက်ခံ/မလက်ခံ ပြန်ပေးသည်။ |
| `task_join_timeout(value, poll_budget)` | Poll budget အများဆုံးအထိ drive လုပ်ပြီး task pending ဖြစ်နေသေးပါက `TimedOut` ပြန်ပေးသည်။ |

Deterministic executor သည် external runtime dependency မရှိပါ။ `RuntimeLimits` သည် task count နှင့် poll count ကို ကန့်သတ်ပြီး `RunReport` သည် poll အရေအတွက်နှင့် ကျန် task အရေအတွက်ကို ဖော်ပြသည်။ သီးခြား `ThreadedRuntime` သည် Rust standard library ကိုသာ အသုံးပြုပြီး `ThreadRuntimeLimits` ဖြင့် worker count၊ admitted task count နှင့် maximum file-read bytes များကို ကန့်သတ်သည်။ Worker panic များကို `ThreadJoinError::WorkerPanicked` အဖြစ် ပြောင်းပေးပြီး task limit ကျော်လွန်သော admission ကို ငြင်းပယ်ကာ worker ပြီးဆုံးချိန်တွင် joiner ကို wake လုပ်သည်။ File read သည် regular file များကိုသာ ခွင့်ပြုပြီး directory သို့မဟုတ် အခြား non-file များကို ငြင်းပယ်ကာ byte limit ထက် မပိုဘဲ ဖတ်သည်။ Deterministic task များအတွက် cooperative cancellation ကို default အဖြစ်ထားရှိပြီး process adapter များတွင် cancellation သို့မဟုတ် deadline ရောက်ပါက child process ကို explicit terminate လုပ်နိုင်သည်။ Zap မပိုင်သော arbitrary blocking call များအတွက် safe forced cancellation မပေးပါ။

## Production I/O နှင့် Multi-thread Scheduling

Deterministic language executor ပေါ်တွင် မ run သင့်သော blocking operation များအတွက် native runtime တွင် bounded threaded adapter ပါရှိပါသည်။ `ThreadedRuntime::new(ThreadRuntimeLimits { max_workers, max_tasks, max_read_bytes })` သည် fixed worker set တစ်ခုကို စတင်ပေးသည်။ `spawn_blocking` သည် `max_tasks` အထိသာ active job များကို လက်ခံပြီး worker ပြီးဆုံးချိန်တွင် wake လုပ်ပေးသော `ThreadJoinHandle` ကို ပြန်ပေးသည်။ Worker အတွင်း panic ဖြစ်ပါက runtime boundary ကို ဖြတ်မသွားဘဲ `WorkerPanicked` အဖြစ် ပြောင်းပေးသည်။

`read_file_async` သည် regular-file production I/O facade ဖြစ်ပါသည်။ `tcp_exchange(address, request)` သည် bounded TCP request/response adapter ဖြစ်ပြီး address resolution နှင့် connection ကို deadline ဖြင့် ကန့်သတ်ထားသည်။ Stream ကို non-blocking mode သို့ ပြောင်းပြီး socket သည် `WouldBlock` ပြန်ပေးသောအခါ yield လုပ်ကာ ဆက်လက်စောင့်သည်။ Response သည် `max_socket_bytes` ထက်ကျော်ပါက reject လုပ်သည်။ `process_async(command, arguments)` သည် worker set ပေါ်တွင် child process ကို run လုပ်ပြီး stdin ကို null ထားကာ stdout/stderr ကို သီးခြား drain လုပ်သည်၊ hard deadline နှင့် output cap များကို enforce လုပ်သည်။ `process_async_cancellable(command, arguments, token)` သည် cancellation token trigger ဖြစ်ပါက platform process API မှတစ်ဆင့် child ကို terminate လုပ်ပြီး reaped child နှင့် bounded output များကို အပြီးသတ် drain လုပ်ကာ resolve လုပ်သည်။ API နှစ်ခုလုံးတွင် outer result သည် scheduler/admission failure နှင့် inner result သည် I/O သို့မဟုတ် process failure ကို ခွဲခြားပေးသည်။ Forced cancellation သည် Zap က စတင်ထားသော child process များအတွက်သာ သက်ရောက်ပြီး arbitrary foreign blocking call များနှင့် OS-level sandboxing တို့သည် ဤ adapter contract အပြင်ဘက်တွင် ရှိသည်။

## Async Language Syntax

`fn` ရှေ့တွင် `async` ထည့်ပြီး asynchronous function ကြေညာနိုင်ပါသည်။ Function call သည် ပုံမှန် result အစား context-owned `ScheduledFuture` value ပြန်ပေးပြီး deterministic executor ကို drive လုပ်ကာ completed result ရယူရန် `await` ကို အသုံးပြုရပါသည်။

```zap
async fn load_version() -> number:
    return 7

let pending = load_version()
let version: number = await pending
say version
```

`async` function သည် ပုံမှန် function ကဲ့သို့ parameter နှင့် return-type annotation များကို အသုံးပြုနိုင်ပါသည်။ Evaluator သည် runtime function ပေါ်တွင် async declaration flag ကို ထိန်းသိမ်းပြီး declared result ကို validate လုပ်ကာ caller ၏ `RuntimeState` မှတစ်ဆင့် schedule လုပ်ပါသည်။

`await` သည် expression ဖြစ်သောကြောင့် declaration၊ assignment၊ return expression သို့မဟုတ် nested call များထဲတွင် အသုံးပြုနိုင်ပါသည်။

```zap
async fn answer() -> number:
    return 42

let value = await answer()
say value + 1
```

လက်ရှိ deterministic model တွင် background thread မရှိပါ။ `ScheduledFuture` သည် per-run task ID ပါသော stable runtime value ဖြစ်ပြီး `await` သို့မဟုတ် `task_join` သည် result ကို consume မလုပ်မီ context executor ကို drive လုပ်ပါသည်။ `spawn(async_call())` သည် scheduled handle ကို ထိန်းသိမ်းပြီး `task_is_ready` သည် consume သို့မဟုတ် poll မလုပ်ဘဲ readiness ကို စစ်ဆေးသည်။ `task_cancel` သည် cooperative cancellation request ပြုလုပ်ပြီး `task_join_timeout` သည် executor poll ကို ကန့်သတ်ကာ deterministic `TimedOut` failure ပြန်ပေးသည်။ `Future` မဟုတ်သော value ကို await သို့မဟုတ် join လုပ်ပါက value ကို တိတ်တဆိတ် ပြောင်းလဲမည့်အစား runtime error ပြန်ပေးပါသည်။

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

Server သည် standard input/output မှတစ်ဆင့် `Content-Length` header ဖြင့် frame လုပ်ထားသော JSON-RPC message များကို ဆက်သွယ်ပါသည်။ `initialize` သည် `textDocumentSync` တွင် `openClose: true` နှင့် `change: 1` ကို ကြေညာသဖြင့် client များသည် full-document change ကို `params.contentChanges` မှတစ်ဆင့် ပေးရမည်ဖြစ်သည်။ `didChange` ထဲရှိ non-standard `textDocument.text` field ကို server မဖတ်ပါ။

| Message | အပြုအမူ |
|---|---|
| `initialize` | Zap server information ကို ပြန်ပေးပြီး text synchronization၊ completion၊ diagnostics၊ hover၊ definition၊ rename နှင့် workspace-symbol capabilities များကို ကြေညာသည်။ |
| `shutdown` | အောင်မြင်သော null result ကို ပြန်ပေးသည်။ |
| `textDocument/didOpen` | Document text နှင့် optional monotonically increasing version ကို သိမ်းဆည်းပြီး deterministic source range ပါသော lint diagnostics ထုတ်ပေးသည်။ |
| `textDocument/didChange` | ကြေညာထားသော full-sync mode အတွက် standard `params.contentChanges` payload ကို အသုံးပြုကာ နောက်ဆုံး full-text change မှ stored document ကို အစားထိုးသည်။ Versioned open ပြီးနောက် stale/unversioned update များကို reject လုပ်ပြီး လက်ခံထားသော text အပေါ် diagnostics ထုတ်ပေးသည်။ Position-aware incremental mode မတည်ဆောက်မချင်း range-based incremental change များကို လုံခြုံစွာ reject လုပ်သည်။ |
| `textDocument/didClose` | Per-session workspace index မှ document ကို ဖယ်ရှားပြီး အခြား LSP session များကို မထိခိုက်စေပါ။ |
| `textDocument/completion` | လက်ရှိ source prefix အပေါ်မူတည်၍ language keyword၊ catalog ထဲရှိ standard-library builtin အားလုံးနှင့် document ထဲမှ top-level `let`/function declaration များကို filter လုပ်သည်။ |
| `textDocument/hover` | သိမ်းဆည်းထားသော document ကို parse လုပ်ပြီး top-level function၊ class နှင့် declaration များအတွက် parser-owned metadata ပြသည်။ Async builtin များအတွက် stable scheduling documentation ကို ပြသည်။ |
| `textDocument/definition` | Referenced top-level declaration ကို parser-span source range သို့ resolve လုပ်သည်။ |
| `textDocument/rename` | ရွေးချယ်ထားသော lexical declaration နှင့် ၎င်းနှင့် binding ဖြစ်သော reference များအတွက် deterministic file-local `WorkspaceEdit` ထုတ်ပေးသည်။ Parameter၊ closure၊ shadowed scope နှင့် import alias များကို လွှမ်းခြုံပြီး string/comment များကို ကာကွယ်ကာ invalid name၊ keyword နှင့် standard-library builtin များကို reject လုပ်သည်။ Cross-file rename ကို လက်ရှိ contract တွင် support မလုပ်သေးပါ။ |
| `workspace/symbol` | In-memory indexed documents များထဲမှ deterministic symbol များကို ရှာဖွေပြီး editor တွင် မဖွင့်ထားသော package module များကို explicit local import အတိုင်း လုံခြုံစွာ လိုက်လံရှာဖွေသည်။ |
| `textDocument/formatting` | Line ending၊ tab၊ trailing space နှင့် နောက်ဆုံး newline များကို normalize လုပ်သော full-document edit တစ်ခု ပြန်ပေးသည်။ |

Completion သည် fixed unfiltered list မဟုတ်တော့ဘဲ context-aware ဖြစ်ပါသည်။ Language keyword၊ machine-readable standard-library catalog နှင့် active document ထဲမှ declaration များကို ပေါင်းစပ်ပေးပါသည်။ ဥပမာ `async fn load():` ပါသော document ထဲတွင် `lo` ရိုက်ထားပါက completion response တွင် `load` ကို function item အဖြစ် ပြန်ပေးပါသည်။ Hover သည် source position မှ active word ကို ရှာပြီး parser ၏ `SourceSpan` ပါသော AST မှ declaration အချက်အလက်ကို ပြန်ထုတ်ပါသည်။ `spawn`၊ `task_join`၊ `task_is_ready`၊ `task_cancel`၊ `task_join_timeout` နှင့် `async_capabilities` တို့သည် သက်ဆိုင်ရာ async-boundary text ကို ပြသပြီး signature help တွင် တည်ငြိမ်သော parameter label များကို ပေးပါသည်။

Workspace symbol indexing သည် ဖွင့်ထားသော file ၏ directory မှ `import app.util as util` ကဲ့သို့သော explicit local import များကို လိုက်လံရှာဖွေပြီး dotted path ကို `app/util.zp` အဖြစ် ပြောင်းလဲပါသည်။ Imported file များကို indexing မပြုမီ canonicalize လုပ်ပြီး importing directory အတွင်းတွင်သာ ရှိရမည်ဖြစ်ကာ 8 MiB အထိသာ ခွင့်ပြုပါသည်။ Invalid၊ မတွေ့ရှိသော၊ အရွယ်အစားကျော်လွန်သော၊ ဖတ်မရသော သို့မဟုတ် traversal ဆန်သော module များကို editor သို့မဟုတ် filesystem escape မဖြစ်စေရန် deterministic အတိုင်း ကျော်လွှားပါသည်။ ရှာဖွေတွေ့ရှိသော module URI များကို open document များနှင့်အတူ sorted index တစ်ခုတည်းထဲ ထည့်သွင်းသဖြင့် nested import များကို တစ်ကြိမ်သာ လိုက်လံပြီး ရလဒ်များ တည်ငြိမ်နေပါသည်။ Indexing ကို document ၂၅၆ ခု၊ import depth ၃၂ အဆင့်နှင့် in-memory source text စုစုပေါင်း ၃၂ MiB အထိ ကန့်သတ်ထားပြီး limit ကျော်သော document/module များကို လက်ခံထားပြီးသော open buffer များကို မဖယ်ရှားဘဲ ကျော်လွှားပါသည်။

Rename သည် same-spelling token အားလုံးကို အစားထိုးခြင်းမပြုဘဲ file-local lexical binding model ကို အသုံးပြုသည်။ Function၊ class၊ module၊ `let`၊ `for`၊ `catch`၊ parameter နှင့် `import ... as alias` declaration များကို binding identity သတ်မှတ်ပြီး nested scope များအတွင်း nearest visible declaration သို့ reference များကို resolve လုပ်သည်။ Outer binding ကို ရွေးချယ်ပါက inner shadow ကို မပြောင်းလဲစေပါ။ String၊ comment၊ module-path segment၊ keyword နှင့် builtin များကို ဖယ်ထားသည်။ Cross-file edit သည် လက်ရှိ contract အပြင်တွင်ရှိပြီး active URI အတွက် edit များသာ ပြန်ပေးသည်။

Diagnostics များကို Zap ၏ ရှိပြီးသား lint implementation မှ ထုတ်ယူပါသည်။ ထို့ကြောင့် CLI နှင့် editor diagnostics များ၏ rules များ တူညီနေပါသည်။ LSP session တစ်ခုအတွင်း document version များသည် monotonic ဖြစ်ရမည်။ Versioned document အတွက် stale သို့မဟုတ် unversioned change များကို လျစ်လျူရှုပြီး နောက်ဆုံးမှန်ကန်သော buffer ကို မအစားထိုးပါ။ Full-sync change လက်ခံသောအခါ `contentChanges` ထဲမှ text အသစ်အပေါ် diagnostics ထုတ်ပေးသဖြင့် completion၊ hover၊ definition၊ symbol၊ formatting နှင့် rename အားလုံးသည် တစ်ခုတည်းသော accepted in-memory document ကို အသုံးပြုသည်။ Incremental range edit များကို မမှန်ကန်စွာ ခန့်မှန်း apply မလုပ်ဘဲ reject လုပ်ထားပါသည်။ Server သည် `initialize.params.capabilities.general.positionEncodings` မှ `utf-8`၊ `utf-16` သို့မဟုတ် `utf-32` ကို negotiate လုပ်ပြီး ပထမဦးဆုံး support လုပ်နိုင်သော encoding ကို ရွေးသည်၊ မပါရှိပါက UTF-16 ကို default သတ်မှတ်သည်။ Inbound cursor position နှင့် outbound diagnostic၊ symbol၊ formatting၊ rename range များအားလုံးတွင် ထို encoding ကို တစ်ပြေးညီ အသုံးပြုသည်။ URI handling သည် percent-decode ကို လုံခြုံစွာလုပ်ပြီး malformed escape၊ URI host၊ NUL byte နှင့် decode ပြီးနောက် traversal segment များကို reject လုပ်ကာ import containment check မတိုင်မီ local path ကို canonicalize လုပ်သည်။ Lint message တွင် source line ပါရှိပါက server သည် ၎င်းကို zero-based LSP range အဖြစ် ပြောင်းပြီး line ၏ encoded width အတိုင်း သတ်မှတ်ပါသည်။ Line မဖတ်နိုင်သော diagnostic များအတွက် ပထမ line ကို deterministic fallback အဖြစ် အသုံးပြုပါသည်။

## Tooling Synchronization

Formatter နှင့် LSP တို့သည် finalized async vocabulary တစ်ခုတည်းကို အသုံးပြုပါသည်။ Completion တွင် catalog ထဲရှိ public builtin အားလုံးကို domain ကို deterministic detail အဖြစ် ပြသပြီး `spawn`၊ `task_join`၊ `task_is_ready`၊ `task_cancel` နှင့် `task_join_timeout` တို့လည်း ပါဝင်ပါသည်။ VS Code TextMate grammar တွင် catalog vocabulary တစ်ခုလုံးကို callable Zap function များအဖြစ် highlight လုပ်ပါသည်။ Editor parity validation script သည် grammar တွင် catalog builtin နှင့် async keyword များ အားလုံးပါဝင်မှုကို စစ်ဆေးပြီး language facade၊ catalog နှင့် editor asset များကြား drift မဖြစ်စေရန် ကာကွယ်ပေးပါသည်။

## Production Deployment Boundaries

Repository အတွင်း authenticated registry service အတွက် reproducible deployment reference layer ကို ထည့်သွင်းပြီးဖြစ်ပါသည်။ `deploy/zap-registry.service` တွင် dynamic least-privilege user၊ protected filesystem path များ၊ သတ်မှတ်ထားသော writable storage၊ memory/CPU/task/file quota များနှင့် loopback-only network access ပါသော Linux supervision ကို သတ်မှတ်ထားပါသည်။ `deploy/zap-registry.nginx.conf` တွင် TLS 1.2/1.3 termination၊ HTTP-to-HTTPS redirect၊ bounded request body နှင့် proxy timeout များ၊ restricted method များနှင့် loopback upstream ကို သတ်မှတ်ထားပါသည်။ `deploy/registry.env.example` တွင် placeholder များသာ ပါပြီး `deploy/registry-deployment-policy.toml` တွင် credential၊ sandbox၊ quota နှင့် egress contract ကို မှတ်တမ်းတင်ထားပါသည်။ Install မလုပ်မီ `scripts/validate_registry_deployment.sh` ကို run လုပ်ရမည်ဖြစ်ပြီး CI တွင်လည်း အလားတူ gate ကို run လုပ်ပါသည်။

Bounded production I/O adapter နှင့် multi-thread scheduler သည် regular-file read၊ bounded non-blocking TCP exchange၊ bounded asynchronous process execution၊ cancellation-aware child termination နှင့် explicitly submitted blocking task များအထိ လွှမ်းခြုံပြီးဖြစ်ပါသည်။ Authenticated loopback registry service သည် bounded request များ၊ bearer authentication၊ safe in-root GET path များ၊ signed-index persistence၊ managed shutdown နှင့် deterministic failure response များကို ထောက်ပံ့ပါသည်။ Repository-side production boundary များကို implementation နှင့် tests ဖြင့် ပြီးစီးထားပါသည်။ Public deployment သည် platform-specific ဖြစ်နေသေးသဖြင့် operator များသည် real TLS certificate၊ DNS၊ ingress/WAF/rate limiting၊ external service supervision၊ OS-native sandbox equivalent၊ monitoring နှင့် review ပြုလုပ်ထားသော egress allowlist များကို သီးခြား provision လုပ်ရမည်။ Zap မပိုင်သော arbitrary blocking call များ၏ cancellation သည် safe runtime contract အပြင်တွင် ရှိနေပါသည်။

Package workflow အတွက် [Burmese package guide](PACKAGE.md) နှင့် [P2 progress](P2_PROGRESS_MM.md) ကို ဖတ်ရှုနိုင်ပါသည်။ English version အတွက် [ASYNC_LSP_EN.md](ASYNC_LSP_EN.md) ကို ကြည့်ပါ။
