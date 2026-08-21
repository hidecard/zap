# Zap Native Benchmark Harness

**အခြေအနေ:** Zap v2.1.6 အတွက် ပထမဆုံး repeatable baseline

Repository တွင် native interpreter အတွက် dependency-free benchmark runner ဖြစ်သော `scripts/benchmark_native.sh` ကို ထည့်သွင်းထားပါသည်။ လိုအပ်ပါက locked dependency graph ဖြင့် `native/target/release/zap` ကို build လုပ်ပြီး temporary source fixture များ ဖန်တီးကာ fixture တစ်ခုချင်းစီကို သတ်မှတ်ထားသည့်အကြိမ်ရေဖြင့် run ပါသည်။ CSV output တွင် `suite`၊ `iteration` နှင့် `elapsed_seconds` ဟူသော column များကို တည်ငြိမ်စွာ ထုတ်ပေးပါသည်။

## လက်ရှိ benchmark suite များ

| Suite | Workload | ရည်ရွယ်ချက် |
|---|---|---|
| `loops` | Integer accumulation ပါသော ကန့်သတ်ထားသည့် `while` loop။ | Loop နှင့် arithmetic dispatch baseline တည်ဆောက်ရန်။ |
| `calls` | ကန့်သတ်ထားသည့် loop အတွင်း user-defined function call များကို ထပ်ခါတလဲလဲ လုပ်ခြင်း။ | Call frame နှင့် return value baseline တည်ဆောက်ရန်။ |
| `closures` | Captured state ကို ထပ်ခါတလဲလဲ ပြောင်းလဲသော nested function။ | Closure environment နှင့် captured-state dispatch baseline တည်ဆောက်ရန်။ |
| `allocations` | `range(10000)` ပြီးနောက် `enumerate` လုပ်ခြင်း။ | List allocation နှင့် collection transformation baseline တည်ဆောက်ရန်။ |
| `json` | Deterministic numeric list ကို JSON encode/decode လုပ်ခြင်း။ | Conversion နှင့် nested-value traversal baseline တည်ဆောက်ရန်။ |
| `async` | Deterministic async task ကို spawn၊ readiness စစ်ဆေးခြင်းနှင့် join လုပ်ခြင်း။ | Task scheduling နှင့် completion baseline တည်ဆောက်ရန်။ |
| `imports` | External dependency မလိုသော explicit module/import dispatch fixture နှင့် deterministic helper call။ | Module loading နှင့် dispatch coverage ကို တည်ဆောက်ရန်။ |

Fixture များကို temporary directory ထဲတွင် ဖန်တီးသောကြောင့် repository ကို မပြောင်းလဲပါ။ Default output ကို `benchmark-results/native.csv` တွင် ရေးသားပြီး `ZAP_BENCH_REPEATS` ဖြင့် repetition အရေအတွက်ကို positive integer အဖြစ် သတ်မှတ်နိုင်ပါသည်။ `ZAP_BENCH_OUTPUT` ဖြင့် အခြား CSV path ကို ရွေးနိုင်ပါသည်။ ဥပမာ:

```sh
ZAP_BENCH_REPEATS=10 scripts/benchmark_native.sh
```

Minimal CI environment တွင်လည်း အလုပ်လုပ်စေရန် optional external timing package မသုံးဘဲ Bash ၏ built-in `time` ကို အသုံးပြုထားပါသည်။ Measurement များသည် wall-clock process time ဖြစ်ပြီး တူညီသော machine နှင့် toolchain ပေါ်ရှိ regression comparison အတွက်သာ ရည်ရွယ်ပါသည်။ Machine မတူသည့် performance claim အတွက် မသုံးရပါ။

## အဓိပ္ပါယ်ဖော်ခြင်းနှင့် ကန့်သတ်ချက်များ

Binary၊ compiler profile၊ operating system၊ CPU condition၊ repetition count နှင့် fixture source တို့ကို တစ်ပြိုင်တည်း မှတ်တမ်းမတင်လျှင် baseline run ၏ အသုံးဝင်မှု လျော့နည်းပါသည်။ မူရင်း measurement များကို audit လုပ်နိုင်စေရန် CSV တွင် raw observation များကိုသာ ထားရှိပြီး aggregation နှင့် plotting ကို သီးခြားဆောင်ရွက်သင့်ပါသည်။ လက်ရှိ suite တွင် external registry access မလိုသော explicit module/import dispatch ပါဝင်ပါသည်။ `scripts/aggregate_benchmark.sh` သည် CSV ကိုဖတ်ပြီး raw observation များကို ထိန်းသိမ်းကာ suite တစ်ခုချင်းစီအတွက် deterministic min/mean/max summary ထုတ်ပေးပါသည်။ Closure environment၊ JSON conversion နှင့် deterministic async scheduling ကို လက်ရှိ suite ထဲသို့ ထည့်ပြီးဖြစ်ပါသည်။

ဤ harness သည် statistically rigorous microbenchmark framework ဖြစ်သည်ဟု မဆိုပါ။ CPU frequency ကို isolate မလုပ်ပါ၊ process ကို core တစ်ခုသို့ pin မလုပ်ပါ၊ allocator-level allocation ကိုလည်း မတိုင်းတာပါ။ ထို့ကြောင့် performance claim တိုင်းတွင် environment ကို ဖော်ပြပြီး commit တစ်ခုတည်း၏ repeated run များကိုသာ နှိုင်းယှဉ်သင့်ပါသည်။ Benchmark သည် CI တွင် မြင်နိုင်သော baseline gate ဖြစ်ပြီး timing-threshold အပေါ်မူတည်သော release gate မဟုတ်ပါ။ CI သည် suite ခုနစ်ခု smoke run၊ CSV aggregation နှင့် raw/summary artifact upload ကို လုပ်ဆောင်သော်လည်း machine-dependent elapsed-time threshold ကြောင့် မအောင်မြင်အောင် မသတ်မှတ်ထားပါ။

## Verification

Harness နှင့် aggregator ကို suite တစ်ခုချင်းစီ repetition တစ်ကြိမ်ဖြင့် စမ်းသပ်ထားပါသည်။ လက်ရှိ suite ခုနစ်ခုလုံး အောင်မြင်ကာ raw CSV observation ခုနစ်ခုနှင့် deterministic summary output ထွက်ရှိပါသည်။ Benchmark ပြောင်းလဲမှုများကို commit မလုပ်မီ native formatter၊ full native tests နှင့် `git diff --check` တို့ကို မဖြစ်မနေ အောင်မြင်စေရမည်။
