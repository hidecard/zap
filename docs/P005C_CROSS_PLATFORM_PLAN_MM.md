# P0-05-C Cross-Platform Async Matrix Plan

## ရည်ရွယ်ချက်

Production-oriented async boundary ကို Linux၊ Windows နှင့် macOS များတွင် reproducible focused test script တစ်ခုနှင့် target-native CI evidence artifact တစ်ခုစီဖြင့် အတည်ပြုရန် ဖြစ်သည်။

## Matrix contract

| Target | လိုအပ်သော evidence | Platform-specific expectation |
|---|---|---|
| Linux x86_64 | Build၊ focused async tests၊ upload လုပ်ထားသော log | Regular-file read၊ loopback TCP exchange၊ bounded process output၊ deadline၊ cancellation နှင့် typed preflight errors များ native အတိုင်း pass ဖြစ်ရမည် |
| Windows x86_64 | Build၊ focused async tests၊ upload လုပ်ထားသော log | `cmd.exe` process adapter path၊ Windows path/regular-file behavior၊ loopback TCP exchange၊ bounded output၊ cancellation နှင့် typed preflight errors များ native အတိုင်း pass ဖြစ်ရမည် |
| macOS ARM64 | Build၊ focused async tests၊ upload လုပ်ထားသော log | Native ARM64 build၊ regular-file read၊ loopback TCP exchange၊ bounded process output၊ deadline၊ cancellation နှင့် typed preflight errors များ native အတိုင်း pass ဖြစ်ရမည် |

## Focused test set

Matrix script သည် target တိုင်းတွင် တူညီသော exact test names များကို run လုပ်မည်။ Worker concurrency၊ invalid-limit preflight၊ TCP round trip၊ oversized TCP response၊ admission မတိုင်မီ oversized TCP request၊ cross-platform process output၊ capped process output၊ forced process cancellation နှင့် bounded regular-file read များကို လွှမ်းခြုံမည်။ Full native suite သည် သီးခြား gate အဖြစ် ဆက်ရှိမည်။ Matrix သည် adapter boundary ကို target တိုင်းတွင် တကယ် exercise လုပ်ထားကြောင်း အထောက်အထားပေးမည်။

## Evidence နှင့် limitation policy

Target job တစ်ခုစီသည် target triple၊ runner OS၊ Rust version နှင့် exact test command များပါသော deterministic text log တစ်ခုရေးမည်။ Log ကို target အမည်ပါသော CI artifact အဖြစ် upload လုပ်မည်။ Target တစ်ခုသည် runner/toolchain limitation ကြောင့် မ run နိုင်ပါက တိတ်တဆိတ် skip မလုပ်ဘဲ versioned limitation record ထုတ်ရမည်။ ဤအဆင့်သည် arbitrary foreign blocking call များ interrupt လုပ်နိုင်သည် သို့မဟုတ် language-level future များ executor-backed ဖြစ်သည်ဟု မဆိုပါ။

## Release gates

Matrix script သည် local host တွင် pass ဖြစ်ရမည်။ GitHub Actions build matrix သည် target သုံးခုလုံးတွင် script ကို invoke လုပ်ရမည်။ Artifact upload configuration ရှိရမည်။ Async runtime contract တွင် matrix ကို document လုပ်ရမည်။ P0/P1 register သည် နောက် execution item ကို P1-05-A replayable verification layers သို့ ပြောင်းနိုင်မှသာ ဤအဆင့် ပြီးစီးသည်။
