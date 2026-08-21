# Zap Async Runtime

## v2.1-D ပထမအဆင့်

Zap ၏ async runtime တွင် deterministic ဖြစ်သော single-threaded executor foundation ရှိပြီးဖြစ်သည်။ v2.1-D ၏ ပထမအဆင့်တွင် worker thread များ မဖန်တီးဘဲ၊ synchronous language surface ကို မပြောင်းလဲဘဲ **joinable task handle** များကို ထည့်သွင်းထားသည်။

> Joinable task ဆိုသည်မှာ runtime သို့ future တစ်ခုကို တင်သွင်းပြီး runtime က task ကို ပြီးဆုံးသည်အထိ poll လုပ်ပြီးနောက် ထွက်လာသော output ကို handle မှတစ်ဆင့် ရယူနိုင်သော task ဖြစ်သည်။

## API contract

| API | Contract |
|---|---|
| `AsyncRuntime::spawn_joinable(future)` | Future ကို submit လုပ်ပြီး `Result<JoinHandle<T>, SpawnError>` ပြန်ပေးသည်။ |
| `JoinHandle<T>::is_ready()` | Task က output ထုတ်ပြီးပြီလားကို ပြသည်။ |
| `JoinHandle<T>` ကို future အဖြစ်အသုံးပြုခြင်း | `Result<T, JoinError>` အဖြစ် resolve လုပ်သည်။ မပြီးသေးမီ poll လုပ်လျှင် pending ဖြစ်ပြီး၊ output ရယူပြီးနောက် ထပ် poll လုပ်လျှင် `AlreadyJoined` ပြန်ပေးသည်။ |
| `SpawnError::TaskLimitReached` | `max_tasks` limit ကျော်မည့်အခါ ပြန်ပေးသည်။ |
| `AsyncRuntime::spawn_joinable_cancellable(future)` | `(JoinHandle<T>, CancellationToken)` ပြန်ပေးပြီး cancellation ဖြစ်ပါက `JoinError::Cancelled` ဖြင့် resolve လုပ်သည်။ |
| `timeout_ticks(future, ticks)` | Inner future သည် သတ်မှတ်ထားသော deterministic poll deadline မတိုင်မီ ပြီးပါက `Ok(output)`၊ မပြီးပါက `Err(TimeoutError)` ပြန်ပေးသည်။ |
| `AsyncRuntime::spawn_joinable_result(future)` | `Future<Output = Result<T, E>>` ကို submit လုပ်ပြီး task ၏ typed failure ကို ထိန်းသိမ်းထားသော `TaskJoinHandle<T, E>` ပြန်ပေးသည်။ |
| `TaskJoinHandle<T, E>` ကို future အဖြစ်အသုံးပြုခြင်း | `Ok(T)`၊ `Err(TaskJoinError::Failed(E))` သို့မဟုတ် `Err(TaskJoinError::AlreadyJoined)` အဖြစ် resolve လုပ်သည်။ |
| `AsyncRuntime::spawn_joinable_result_cancellable(future)` | `(TaskJoinHandle<T, E>, CancellationToken)` ပြန်ပေးသည်။ Cancellation ကို inner future မ poll မီ စစ်ဆေးသဖြင့် task result ထက် cancellation ကို ဦးစားပေးသည်။ |
| `spawn(future)` | Language-level facade အဖြစ် `Future` ကို လက်ခံပြီး task future တစ်ခု ပြန်ပေးသည်။ |
| `task_is_ready(task)` | လက်ရှိ eager language-level future representation အတွက် `true` ပြန်ပေးပြီး future မဟုတ်သော value များကို ငြင်းပယ်သည်။ |
| `task_join(task)` | Language-level task future ကို consume လုပ်ပြီး ပြီးမြောက်ထားသော value ကို ပြန်ပေးသည်။ Future မဟုတ်သော value များကို ငြင်းပယ်သည်။ |
| `async_capabilities()` | Deterministic executor၊ worker၊ network၊ process၊ cancellation၊ limit နှင့် deferred language-level boundary များကို ဖော်ပြသော stable map ကို ပြန်ပေးသည်။ |

Runtime သည် deterministic အတိုင်း ဆက်လက်အလုပ်လုပ်သည်။ Task များကို submit လုပ်သည့်အစဉ်အတိုင်း သိမ်းဆည်းပြီး လက်ရှိ budget သတ်မှတ်ထားသော executor ဖြင့် poll လုပ်သည်။ `spawn_joinable` သည် task လက်ခံမည့်အချိန် error များကို တိတ်တဆိတ်ဖယ်ရှားခြင်းမပြုဘဲ caller ထံ ပြန်ပေးသည်။ Implementation သည် Rust 1.75 နှင့် ကိုက်ညီသော standard-library primitives များကို အသုံးပြုပြီး worker thread များ မဖန်တီးပါ။

## ဥပမာများ

Runtime-level join ကို ဆက်လက်အသုံးပြုနိုင်သည်။

```rust
let mut runtime = AsyncRuntime::new();
let handle = runtime.spawn_joinable(async { 42 }).unwrap();
runtime.run_until_idle();
let value = block_on(handle).unwrap();
```

ပထမဆုံး language-level facade ကို `.zp` program များတွင် အသုံးပြုနိုင်သည်။

```zap
async fn load() -> number:
    return 42

let task = spawn(load())
let ready: bool = task_is_ready(task)
let value: number = task_join(task)
```

လက်ရှိ evaluator သည် async function body များကို eager အဖြစ်တွက်ချက်ပြီး ရလဒ်ကို ရှိပြီးသား `Future` value ထဲတွင် သိမ်းသည်။ ထို့ကြောင့် `spawn` သည် language-level task contract ကို သတ်မှတ်ပေးပြီး `task_join` နှင့် `task_is_ready` သည် တည်ငြိမ်သော API surface ကို ပေးသည်။ Executor-backed scheduling ကို နောက်ထပ် integration slice အဖြစ် ဆက်လက်လုပ်ဆောင်မည်။ Runtime-level handle များကို join မလုပ်မီ drive လုပ်ထားရမည်။ Runtime task သည် pending ဖြစ်နေသေးပါက handle သည် pending အဖြစ် ဆက်ရှိပြီး poll budget ကို လိုက်နာသည်။

## Structured cancellation

`spawn_joinable_cancellable(future)` သည် join handle နှင့် clone လုပ်နိုင်သော `CancellationToken` တစ်ခုကို ပြန်ပေးသည်။ `cancel()` ကို ခေါ်လျှင် token သည် atomically cancelled ဖြစ်သွားသည်။ Task wrapper သည် inner future ကို poll မလုပ်မီ token ကို စစ်ဆေးသဖြင့် cancelled task ၏ inner future ကို မ poll လုပ်ဘဲ handle မှ `Err(JoinError::Cancelled)` ပြန်ပေးသည်။ ထို့ကြောင့် caller သည် completion အတွက် handle နှင့် cancellation အတွက် token ကို သီးခြားထိန်းချုပ်နိုင်သည်။

## Timeout propagation

`timeout_ticks(future, ticks)` သည် deadline ကို wall-clock time မဟုတ်ဘဲ executor poll အရေအတွက်ဖြင့် တိုင်းတာသည်။ Inner future ကို အရင် poll လုပ်ပြီး pending ဖြစ်သော poll တစ်ကြိမ်စီတွင် tick တစ်ခု လျော့သည်။ Tick မကျန်တော့သည့်အချိန်တွင် inner future က pending ဖြစ်နေသေးပါက wrapper သည် `Err(TimeoutError)` ပြန်ပေးသည်။ Inner future ပြီးမြောက်ပါက `Ok(value)` ကို propagate လုပ်ပြီး thread သို့မဟုတ် system sleep call မသုံးပါ။

## Task error propagation

`spawn_joinable_result(future)` သည် output အဖြစ် `Result<T, E>` ပြန်ပေးသော future ကို လက်ခံသည်။ Runtime သည် အောင်မြင်သော value သို့မဟုတ် typed error အတိအကျကို သိမ်းဆည်းပြီး string ပြောင်းခြင်း သို့မဟုတ် panic အသုံးပြုခြင်းမရှိဘဲ caller ထံ `TaskJoinError::Failed(E)` ဖြင့် ပြန်ပေးသည်။ Cancellable variant သည် inner future ကို poll မလုပ်မီ token ကို စစ်ဆေးသည်။ Cancellation ကို ကြိုတင်တောင်းဆိုထားပါက handle သည် `TaskJoinError::Cancelled` ဖြင့် resolve လုပ်ပြီး task error ကို မထုတ်ပေးပါ။

## Boundary capability report

Argument မလိုသော `async_capabilities()` builtin သည် adapter တိုင်းကို language-level scheduler ၏ အစိတ်အပိုင်းဟု မဆိုဘဲ runtime boundary ကို observable ဖြစ်စေသည်။ Stable fields များသည် single-threaded poll-budget executor၊ fixed worker adapter၊ bounded non-blocking TCP adapter၊ bounded process adapter၊ terminate-then-drain process cancellation၊ eager language-level future များ၊ deferred language-level scheduling/cancellation/timeout နှင့် arbitrary foreign blocking call များကို interrupt မလုပ်နိုင်ခြင်းတို့ကို ခွဲခြားဖော်ပြသည်။ လက်ရှိ default worker၊ read၊ socket၊ process-output နှင့် timeout limits များကိုလည်း ဖော်ပြပြီး resource-limit preflight ကို `enforced`၊ invalid-limit error များကို `typed_deterministic` ဟု သတ်မှတ်ဖော်ပြသည်။

ဤ report သည် descriptive နှင့် deterministic သာ ဖြစ်သည်။ Worker မစတင်ပါ၊ socket/process မဖန်တီးပါ၊ task scheduling ကိုလည်း မပြောင်းလဲပါ။ Application သည် သင့်လျော်သော adapter ကို ကိုယ်တိုင်ရွေးချယ်ပြီး operating-system boundary တွင် deployment policy ကို သတ်မှတ်ရမည်။

## Cross-platform matrix

P0-05-C သည် `.github/workflows/ci.yml` ရှိ build job မှတစ်ဆင့် Linux x86_64၊ Windows x86_64 နှင့် macOS ARM64 များတွင် focused async matrix တစ်ခုတည်းကို run လုပ်သည်။ Commit ထဲတွင်ပါသော `scripts/test_p005c_async_matrix.sh` သည် target triple၊ runner OS၊ Rust/Cargo version နှင့် exact test filter များကို target အမည်ပါသော artifact ထဲတွင် မှတ်တမ်းတင်သည်။ Matrix သည် worker concurrency၊ invalid-limit preflight၊ loopback TCP round trip နှင့် response/request bounds၊ platform-native process output/cancellation နှင့် bounded regular-file read များကို လွှမ်းခြုံသည်။ Runner/toolchain limitation ရှိပါက တိတ်တဆိတ် skip မလုပ်ဘဲ versioned limitation artifact အဖြစ် မှတ်တမ်းတင်ရမည်။

## လုံခြုံရေးနှင့် ကျန်ရှိသော scope

`RuntimeLimits::max_tasks` နှင့် `RuntimeLimits::max_polls_per_run` မှတစ်ဆင့် runtime limits များကို တိတိကျကျ သတ်မှတ်နိုင်သည်။ Structured cancellation၊ poll-based timeout propagation၊ typed task error propagation၊ ပထမဆုံး language-level task facade (`spawn`၊ `task_join` နှင့် `task_is_ready`)၊ descriptive `async_capabilities()` report၊ typed limit validation၊ TCP request-size preflight နှင့် reproducible three-target focused matrix များသည် ဤ slice တွင် ပါဝင်ပြီးဖြစ်သည်။ Executor-backed language-level scheduling၊ language-level cancellation/timeout controls နှင့် formatter/LSP/VS Code synchronization များသည် v2.1-D ၏ နောက်ပိုင်းအဆင့်များအဖြစ် ကျန်ရှိနေသည်။

Regression coverage သည် output join အောင်မြင်မှု၊ deterministic readiness၊ task-limit နှင့် typed task failure propagation၊ inner future ကို မ poll မလုပ်မီ cancellation precedence၊ repeated join၊ timeout/completion လမ်းကြောင်းများ၊ zero/oversized resource-limit rejection၊ queue admission မတိုင်မီ TCP request-size rejection နှင့် P0-05-C matrix script မှ target-native process/file/socket behavior များကို စစ်ဆေးထားသည်။ Cross-platform evidence ကို target အမည်ပါသော CI artifact များအဖြစ် ထိန်းသိမ်းထားသည်။
