# Zap Native Benchmark Harness

**အခြေအနေ:** Zap v2.2.7 အတွက် repeatable benchmark baseline နှင့် regression gate

Repository တွင် native interpreter အတွက် dependency-free benchmark runner ဖြစ်သော `scripts/benchmark_native.sh` ကို ထည့်သွင်းထားပါသည်။ လိုအပ်ပါက locked dependency graph ဖြင့် `native/target/release/zap` ကို build လုပ်ပြီး temporary source fixture များ ဖန်တီးကာ fixture တစ်ခုချင်းစီကို သတ်မှတ်ထားသည့်အကြိမ်ရေဖြင့် run ပါသည်။ Raw CSV output တွင် `suite`၊ `iteration` နှင့် `elapsed_seconds` ဟူသော column များကို တည်ငြိမ်စွာ ထုတ်ပေးပါသည်။ M2-BENCH-01 တွင် `ZAP_BENCH_PROVENANCE` ဖြင့် သတ်မှတ်နိုင်သော provenance sidecar (default အနေဖြင့် raw CSV basename ၏ `.provenance.tsv`) ကိုလည်း ထုတ်ပေးပြီး run status၊ UTC timestamp၊ commit၊ target triple၊ operating-system/kernel/architecture၊ ရရှိနိုင်ပါက CPU description၊ Rust/Cargo version၊ binary နှင့် benchmark-script SHA-256 digest၊ repeat/warm-up count၊ suite list နှင့် raw-observation path များကို မှတ်တမ်းတင်ပါသည်။

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

Fixture များကို temporary directory ထဲတွင် ဖန်တီးသောကြောင့် repository ကို မပြောင်းလဲပါ။ Default output ကို `benchmark-results/native.csv` တွင် ရေးသားပြီး `ZAP_BENCH_REPEATS` ဖြင့် measured repetition အရေအတွက်ကို positive integer အဖြစ် သတ်မှတ်နိုင်ပြီး `ZAP_BENCH_WARMUPS` ဖြင့် suite တစ်ခုချင်းစီအတွက် warm-up အကြိမ်ရေကို သတ်မှတ်နိုင်ပါသည်။ `ZAP_BENCH_OUTPUT` ဖြင့် အခြား CSV path ကို ရွေးနိုင်ပါသည်။ ဥပမာ:

```sh
ZAP_BENCH_REPEATS=10 scripts/benchmark_native.sh
```

Minimal CI environment တွင်လည်း အလုပ်လုပ်စေရန် optional external timing package မသုံးဘဲ Bash ၏ built-in `time` ကို အသုံးပြုထားပါသည်။ `ZAP_BENCH_REPEATS` သည် 1 မှ 64 အတွင်း၊ `ZAP_BENCH_WARMUPS` သည် 0 မှ 16 အတွင်းသာ ရှိရမည်ဖြစ်ပြီး CI နှင့် release-preflight အလုပ်ကို bounded ထားပါသည်။ Measurement များသည် wall-clock process time ဖြစ်ပြီး တူညီသော machine နှင့် toolchain ပေါ်ရှိ regression comparison အတွက်သာ ရည်ရွယ်ပါသည်။ Machine မတူသည့် performance claim အတွက် မသုံးရပါ။

## အဓိပ္ပါယ်ဖော်ခြင်းနှင့် ကန့်သတ်ချက်များ

Binary၊ compiler profile၊ operating system၊ CPU condition၊ repetition count နှင့် fixture source တို့ကို တစ်ပြိုင်တည်း မှတ်တမ်းမတင်လျှင် baseline run ၏ အသုံးဝင်မှု လျော့နည်းပါသည်။ M2-BENCH-01 သည် ထို run condition များကို provenance sidecar ထဲတွင် မှတ်တမ်းတင်ပြီး မူရင်း measurement များကို audit လုပ်နိုင်စေရန် raw observation CSV ကို သီးခြားထားရှိပါသည်။ လက်ရှိ suite တွင် external registry access မလိုသော explicit module/import dispatch ပါဝင်ပါသည်။ `scripts/aggregate_benchmark.sh` သည် CSV ကိုဖတ်ပြီး raw observation များကို ထိန်းသိမ်းကာ suite တစ်ခုချင်းစီအတွက် deterministic min/mean/p95/max၊ population standard deviation၊ population variance နှင့် coefficient of variation (`cv_percent`) summary ထုတ်ပေးပါသည်။ Closure environment၊ JSON conversion နှင့် deterministic async scheduling ကို လက်ရှိ suite ထဲသို့ ထည့်ပြီးဖြစ်ပါသည်။

ဤ harness သည် statistically rigorous microbenchmark framework ဖြစ်သည်ဟု မဆိုပါ။ CPU frequency ကို isolate မလုပ်ပါ၊ process ကို core တစ်ခုသို့ pin မလုပ်ပါ၊ allocator-level allocation ကိုလည်း မတိုင်းတာပါ။ ထို့ကြောင့် performance claim တိုင်းတွင် environment ကို ဖော်ပြပြီး commit တစ်ခုတည်း၏ repeated run များကိုသာ နှိုင်းယှဉ်သင့်ပါသည်။ Benchmark သည် CI တွင် မြင်နိုင်သော regression gate ဖြစ်ပါသည်။ CI သည် suite ခုနစ်ခု smoke run၊ CSV aggregation၊ checked-in `benchmark-results/native-summary.csv` နှင့် mean/p95 comparison၊ raw CSV၊ provenance TSV၊ summary နှင့် comparison artifact upload များကို လုပ်ဆောင်ပါသည်။ Default threshold သည် checked-in baseline ထက် 200% တိုးလာမှု ဖြစ်ပြီး ထို threshold ကျော်လွန်ပါက quality job fail ဖြစ်ပါသည်။ Measurement များသည် machine-dependent ဖြစ်သောကြောင့် baseline update သည် explicit reviewed change ဖြင့်သာ ပြုလုပ်ရမည်။

## Verification

Harness နှင့် aggregator ကို positive/zero warm-up setting၊ bounded repeat validation၊ repeated measurement၊ p95 နှင့် variance aggregation၊ malformed input rejection နှင့် expected slow-run failure များဖြင့် စမ်းသပ်ထားပါသည်။ `scripts/test_benchmark_regression.sh` သည် expanded summary schema ကို စစ်ဆေးပြီး malformed variance field များကို reject လုပ်သည်။ CI နှင့် release preflight သည် provenance field များကို require လုပ်ကာ checked-in baseline နှင့် mean/p95 ကို နှိုင်းယှဉ်ပြီး raw CSV၊ provenance TSV၊ summary နှင့် comparison log များကို upload လုပ်သည်။ လက်ရှိ suite ခုနစ်ခုလုံး အောင်မြင်ကာ သတ်မှတ်ထားသော raw observation များနှင့် deterministic summary output ထွက်ရှိပါသည်။ Benchmark ပြောင်းလဲမှုများကို commit မလုပ်မီ native formatter၊ full native tests နှင့် `git diff --check` တို့ကို မဖြစ်မနေ အောင်မြင်စေရမည်။

## P1-05 deterministic test-layer runner

ပိုမိုကျယ်ပြန့်သော conformance နှင့် property layer အတွက် dependency-free CI-visible runner သည် `scripts/test_p105_layers.sh` ဖြစ်ပါသည်။ ၎င်းသည် deterministic parser နှင့် lexer corpus၊ malformed-program နှင့် JSON security corpus၊ malformed-lockfile case များ၊ standard-library security input များ၊ registry provenance/property mutation များ၊ collection/filesystem regression များနှင့် async cancellation/scheduler determinism case များကို run လုပ်ပါသည်။ Invocation တစ်ခုချင်းစီသည် တည်ငြိမ်သော Cargo test filter တစ်ခုကို အသုံးပြုပြီး non-zero result ဖြစ်ပါက ချက်ချင်း fail လုပ်ပါသည်။

Quality job သည် Linux corpus gate ကို run လုပ်ပြီး build matrix သည် Linux၊ Windows နှင့် macOS target များကို သီးခြား compile/test လုပ်ပါသည်။ ထိုခွဲခြားမှုကြောင့် corpus diagnostic များ deterministic ဖြစ်နေစေပြီး cross-platform compilation နှင့် test coverage ကို မလျော့စေပါ။ P1-05 ၏ ကျန်ရှိသော gap များမှာ dedicated fuzz target များနှင့် allocator/heap-level regression counter များ ဖြစ်ပြီး M2-VERIFY-02 platform-specific input slice ကို native build matrix ထဲတွင် အကောင်အထည်ဖော်ပြီးဖြစ်သည်။

လက်ရှိ validation command သည်:

```sh
scripts/test_p105_layers.sh
```

ဤ runner သည် deterministic regression gate ဖြစ်ပြီး timing benchmark မဟုတ်သကဲ့သို့ ရေရှည် fuzz campaign များကို အစားထိုးရန်လည်း မဟုတ်ပါ။

## References

[1]: ../scripts/test_p105_layers.sh "P1-05 deterministic test-layer runner"
[2]: ../.github/workflows/ci.yml "Zap CI quality နှင့် cross-platform build matrix"
[3]: ../scripts/benchmark_native.sh "Native benchmark runner နှင့် provenance sidecar"
[4]: ../scripts/aggregate_benchmark.sh "Variance field များပါသော deterministic benchmark aggregation"
[5]: ../scripts/check_benchmark_regression.sh "Mean နှင့် p95 benchmark regression comparator"
[6]: ../scripts/test_benchmark_regression.sh "Benchmark schema နှင့် regression contract harness"
