# Zap v2.1.13 Release Notes

**အတည်ပြုထားသော version:** v2.1.13
**Release date:** 2026-08-21

## အကျဉ်းချုပ်

Zap v2.1.13 သည် canonical AST execution ပြီးနောက် ဆက်လက်လုပ်ဆောင်သော hidden-state migration slice ကို ပြီးစီးစေပါသည်။ Workspace confinement ကို per-run `RuntimeState` က ပိုင်ဆိုင်ပြီး LSP document map ကို production process-global thread-local storage အစား explicit per-session `LspState` က ပိုင်ဆိုင်ပါသည်။

## Runtime-state ပြောင်းလဲမှုများ

Native evaluator သည် `ExecutionContext` ထဲတွင် canonical workspace root တစ်ခုကို မှတ်တမ်းတင်ပြီး nested function၊ block နှင့် module execution များအတွင်း ထို root ကို ဆက်လက်အသုံးပြုပါသည်။ Filesystem builtin များဖြစ်သော metadata၊ atomic write၊ text/line read/write နှင့် existence check များသည် တူညီသော context-aware boundary ကို အသုံးပြုပါသည်။ Context reset ပြုလုပ်သောအခါ workspace root ကို module cache၊ import-cycle နှင့် execution-depth state များနှင့်အတူ ရှင်းလင်းပါသည်။

LSP stdio server သည် server session တစ်ခုစီအတွက် `LspState` တစ်ခု ဖန်တီးပါသည်။ Completion၊ signature help၊ hover၊ definition၊ formatting၊ document symbols နှင့် workspace symbols များသည် ထို state မှ ဖတ်ရှုပါသည်။ Independent LSP state များသည် တစ်ခုနှင့်တစ်ခု၏ open document များကို မမြင်နိုင်ပါ။ Test-only compatibility wrapper ကို production server execution တွင် အသုံးမပြုပါ။

## Compatibility boundary

Parser ပိုင် source နှင့် local module များအတွက် canonical AST path သည် normative ဖြစ်နေဆဲ ဖြစ်ပါသည်။ Legacy line interpreter ကို older line-bodied function record များအတွက် explicit compatibility-only path အဖြစ်သာ ထားရှိပါသည်။ ဤ release တွင် first-class callable value၊ parent-linked `EnvFrame` binding cell၊ cumulative memory budget၊ broad language async syntax သို့မဟုတ် traits/interfaces semantics များ ပါဝင်သည်ဟု မဆိုလိုပါ။

## Verification

Release candidate သည် Rust formatting check၊ `-D warnings` ပါ strict Clippy၊ full native all-target/all-feature test suite (integration test 254 ခု)၊ workspace နှင့် LSP isolation regression များ၊ documentation consistency validation၊ documentation regression harness နှင့် `git diff --check` များကို အောင်မြင်ခဲ့ပါသည်။

## ကိုးကားရန်

* [Runtime-state contract](RUNTIME_STATE_MM.md)
* [AST foundation status](P0_FOUNDATION_STATUS_MM.md)
* [Documentation navigation](DOCUMENTATION_NAVIGATION_MM.md)
* [Full changelog](../CHANGELOG_MM.md)
