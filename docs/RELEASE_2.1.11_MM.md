# Zap v2.1.11 Release Notes

**ထုတ်ဝေသည့်ရက်:** 2026-08-21

## Release summary

Zap v2.1.11 တွင် native runtime အတွက် ပထမဆုံး explicit per-run `RuntimeState` နှင့် `ExecutionContext` boundary ကို ထည့်သွင်းထားပါသည်။ ဤ release သည် ရှိပြီးသား language behavior ကို ထိန်းသိမ်းထားပြီး module-cache၊ import-cycle နှင့် execution-depth state များ process-global ownership မှတစ်ဆင့် run များအကြား မပေါက်ကြားစေရန် ပြင်ဆင်ထားပါသည်။

## Highlights

Native entrypoint သည် source run တစ်ကြိမ်စီအတွက် `ExecutionContext` ဖန်တီးပြီး AST evaluator၊ legacy evaluator၊ expression parser၊ function call၊ method call၊ object-field initialization နှင့် module loading များတစ်လျှောက် ဖြန့်ဝေပါသည်။ `RuntimeState` သည် လက်ရှိ module cache၊ active import-cycle stack နှင့် bounded execution-depth counter တို့ကို ပိုင်ဆိုင်ပါသည်။ Context reset behavior ကို explicit ပြုလုပ်ပြီး independent test လုပ်နိုင်ပါသည်။

Repository တွင် bilingual runtime-state contract၊ navigation links၊ README architecture/status update၊ roadmap acceptance evidence နှင့် English/Burmese pair အသစ်အတွက် documentation-consistency coverage များကိုလည်း ထည့်သွင်းထားပါသည်။

## Compatibility and deferred scope

ဤပြောင်းလဲမှုသည် internal runtime-boundary improvement ဖြစ်ပါသည်။ Broad async syntax၊ executor-backed language scheduling၊ weak references၊ tracing collection၊ cumulative per-run byte accounting သို့မဟုတ် `Send`/`Sync` guarantee အသစ်များကို မထည့်သွင်းပါ။ Workspace confinement နှင့် ရှိပြီးသား memory contract များသည် ဤ slice တွင် သီးခြား boundary များအဖြစ် ဆက်ရှိပါသည်။ ရှိပြီးသား AST/legacy compatibility behavior ကို native suite ဖြင့် ဆက်လက်စစ်ဆေးထားပါသည်။

## Verification

Release ကို Rust 1.75.0 ဖြင့် `cargo fmt --check`၊ strict `cargo clippy --all-targets --all-features -- -D warnings`၊ full native all-target/all-feature test suite၊ documentation consistency validation၊ documentation regression harness များ၊ benchmark regression checks နှင့် `git diff --check` တို့ဖြင့် စစ်ဆေးထားပါသည်။ Native integration suite တွင် test 254 ခု pass ဖြစ်ပြီး runtime-state isolation နှင့် reset regression များလည်း ထည့်သွင်းထားပါသည်။

## Upgrade guidance

အသုံးပြုသူများသည် မိမိ operating system နှင့် architecture ကိုက်ညီသော archive ကို [v2.1.11 GitHub Release](https://github.com/hidecard/zap/releases/tag/v2.1.11) မှ download လုပ်၍ upgrade လုပ်နိုင်ပါသည်။ Install မလုပ်မီ published checksum နှင့် signature ကို verify လုပ်ပါ။ ဤ release အတွက် source-language migration မလိုအပ်ပါ။

## Documentation

[English runtime-state contract](RUNTIME_STATE_EN.md)၊ [Burmese runtime-state contract](RUNTIME_STATE_MM.md)၊ [English documentation navigation](DOCUMENTATION_NAVIGATION_EN.md) နှင့် [Burmese documentation navigation](DOCUMENTATION_NAVIGATION_MM.md) ကို ဖတ်ရှုနိုင်ပါသည်။ ကျန်ရှိသော memory၊ async၊ conformance၊ specification၊ tooling နှင့် traits work များကို bilingual TODO register နှင့် next-step plan များတွင် မှတ်တမ်းတင်ထားပါသည်။
