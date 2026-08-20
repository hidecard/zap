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

Runtime သည် deterministic အတိုင်း ဆက်လက်အလုပ်လုပ်သည်။ Task များကို submit လုပ်သည့်အစဉ်အတိုင်း သိမ်းဆည်းပြီး လက်ရှိ budget သတ်မှတ်ထားသော executor ဖြင့် poll လုပ်သည်။ `spawn_joinable` သည် task လက်ခံမည့်အချိန် error များကို တိတ်တဆိတ်ဖယ်ရှားခြင်းမပြုဘဲ caller ထံ ပြန်ပေးသည်။ Implementation သည် Rust 1.75 နှင့် ကိုက်ညီသော standard-library primitives များကို အသုံးပြုပြီး worker thread များ မဖန်တီးပါ။

## ဥပမာ

```rust
let mut runtime = AsyncRuntime::new();
let handle = runtime.spawn_joinable(async { 42 }).unwrap();
runtime.run_until_idle();
let value = block_on(handle).unwrap();
```

Join မလုပ်မီ runtime ကို drive လုပ်ထားရမည်။ Task က pending ဖြစ်နေသေးလျှင် handle သည် pending အဖြစ် ဆက်ရှိပြီး သတ်မှတ်ထားသော poll budget ကို ဆက်လက်လိုက်နာသည်။

## Structured cancellation

`spawn_joinable_cancellable(future)` သည် join handle နှင့် clone လုပ်နိုင်သော `CancellationToken` တစ်ခုကို ပြန်ပေးသည်။ `cancel()` ကို ခေါ်လျှင် token သည် atomically cancelled ဖြစ်သွားသည်။ Task wrapper သည် inner future ကို poll မလုပ်မီ token ကို စစ်ဆေးသဖြင့် cancelled task ၏ inner future ကို မ poll လုပ်ဘဲ handle မှ `Err(JoinError::Cancelled)` ပြန်ပေးသည်။ ထို့ကြောင့် caller သည် completion အတွက် handle နှင့် cancellation အတွက် token ကို သီးခြားထိန်းချုပ်နိုင်သည်။

## Timeout propagation

`timeout_ticks(future, ticks)` သည် deadline ကို wall-clock time မဟုတ်ဘဲ executor poll အရေအတွက်ဖြင့် တိုင်းတာသည်။ Inner future ကို အရင် poll လုပ်ပြီး pending ဖြစ်သော poll တစ်ကြိမ်စီတွင် tick တစ်ခု လျော့သည်။ Tick မကျန်တော့သည့်အချိန်တွင် inner future က pending ဖြစ်နေသေးပါက wrapper သည် `Err(TimeoutError)` ပြန်ပေးသည်။ Inner future ပြီးမြောက်ပါက `Ok(value)` ကို propagate လုပ်ပြီး thread သို့မဟုတ် system sleep call မသုံးပါ။

## လုံခြုံရေးနှင့် ကျန်ရှိသော scope

`RuntimeLimits::max_tasks` နှင့် `RuntimeLimits::max_polls_per_run` မှတစ်ဆင့် runtime limits များကို တိတိကျကျ သတ်မှတ်နိုင်သည်။ Structured cancellation နှင့် poll-based timeout propagation များသည် ဤ slice တွင် ပါဝင်ပြီးဖြစ်သည်။ Cancellation ထက်ပိုသော task error values၊ language-level task builtins နှင့် formatter/LSP/VS Code synchronization များသည် v2.1-D ၏ နောက်ပိုင်းအဆင့်များအဖြစ် ကျန်ရှိနေသည်။

Regression coverage သည် output join အောင်မြင်မှု၊ deterministic readiness၊ task-limit error propagation၊ inner future ကို မ poll မလုပ်မီ cancellation နှင့် timeout/completion လမ်းကြောင်းနှစ်ခုလုံးကို စစ်ဆေးထားသည်။ Cross-platform behavior သည် standard-library executor semantics အတွင်းသာ လောလောဆယ် အကျုံးဝင်ပြီး release verification matrix ဖြင့် ဆက်လက်စစ်ဆေးရန် လိုအပ်သည်။
