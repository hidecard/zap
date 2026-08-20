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

Runtime သည် deterministic အတိုင်း ဆက်လက်အလုပ်လုပ်သည်။ Task များကို submit လုပ်သည့်အစဉ်အတိုင်း သိမ်းဆည်းပြီး လက်ရှိ budget သတ်မှတ်ထားသော executor ဖြင့် poll လုပ်သည်။ `spawn_joinable` သည် task လက်ခံမည့်အချိန် error များကို တိတ်တဆိတ်ဖယ်ရှားခြင်းမပြုဘဲ caller ထံ ပြန်ပေးသည်။ Implementation သည် Rust 1.75 နှင့် ကိုက်ညီသော standard-library primitives များကို အသုံးပြုပြီး worker thread များ မဖန်တီးပါ။

## ဥပမာ

```rust
let mut runtime = AsyncRuntime::new();
let handle = runtime.spawn_joinable(async { 42 }).unwrap();
runtime.run_until_idle();
let value = block_on(handle).unwrap();
```

Join မလုပ်မီ runtime ကို drive လုပ်ထားရမည်။ Task က pending ဖြစ်နေသေးလျှင် handle သည် pending အဖြစ် ဆက်ရှိပြီး သတ်မှတ်ထားသော poll budget ကို ဆက်လက်လိုက်နာသည်။

## လုံခြုံရေးနှင့် ကျန်ရှိသော scope

`RuntimeLimits::max_tasks` နှင့် `RuntimeLimits::max_polls_per_run` မှတစ်ဆင့် runtime limits များကို တိတိကျကျ သတ်မှတ်နိုင်သည်။ ရှိပြီးသား cancellation token နှင့် deterministic delay primitives များကိုလည်း runtime foundation တွင် ဆက်လက်အသုံးပြုနိုင်သည်။ Timeout propagation၊ task error values၊ language-level task builtins နှင့် formatter/LSP/VS Code synchronization များသည် v2.1-D ၏ နောက်ပိုင်းအဆင့်များဖြစ်ပြီး ဤ joinable-handle ပထမအဆင့်တွင် ပါဝင်သည်ဟု မယူဆရပါ။

Regression coverage သည် output join အောင်မြင်မှု၊ deterministic readiness နှင့် task-limit error propagation များကို စစ်ဆေးထားသည်။ Cross-platform behavior သည် standard-library executor semantics အတွင်းသာ လောလောဆယ် အကျုံးဝင်ပြီး release verification matrix ဖြင့် ဆက်လက်စစ်ဆေးရန် လိုအပ်သည်။
