# Zap v2.1.14 Release Notes

**အတည်ပြုထားသော version:** v2.1.14
**Release date:** 2026-08-21

## အကျဉ်းချုပ်

Zap v2.1.14 သည် cross-platform CI မှတွေ့ရှိသော Windows-only line-helper compatibility regression အပြီး v2.1.13 tag ကို အစားထိုးသော release ဖြစ်ပါသည်။ Explicit workspace နှင့် LSP state migration ကို ဆက်လက်ထိန်းသိမ်းထားပြီး absolute-path `read_lines` နှင့် `write_lines` program များလိုအပ်သော ရှိပြီးသား behavior ကို ပြန်လည်ထိန်းသိမ်းထားပါသည်။

## Runtime-state ပြောင်းလဲမှုများ

Workspace confinement ကို per-run `RuntimeState` က ဆက်လက်ပိုင်ဆိုင်ပြီး LSP document map ကို explicit per-session `LspState` က ဆက်လက်ပိုင်ဆိုင်ပါသည်။ Native evaluator သည် canonical workspace root တစ်ခုကို မှတ်တမ်းတင်ပြီး nested AST execution၊ module loading နှင့် context-aware filesystem operation များအတွင်း ပြန်လည်အသုံးပြုပါသည်။ LSP response များသည် server ပိုင် document map မှ ဖတ်ရှုသဖြင့် independent `LspState` instance များသည် တစ်ခုနှင့်တစ်ခု၏ open document များကို မမြင်နိုင်ပါ။

## Cross-platform compatibility fix

Compatibility-only line-helper behavior ဖြစ်သော absolute-path `read_lines` နှင့် `write_lines` ကို ဆက်လက်ထိန်းသိမ်းထားပါသည်။ ထို့ကြောင့် migrated filesystem operation များအတွက် context-aware confinement ကို ထိန်းသိမ်းထားစဉ် Windows၊ Linux နှင့် macOS program များ၏ ရှိပြီးသား behavior များ တည်ငြိမ်နေပါသည်။ Failed v2.1.13 Windows CI run ကို diagnosis ပြုလုပ်ပြီး focused commit ဖြင့် ပြင်ဆင်ကာ failed tag ကို ပြန်အသုံးမပြုဘဲ ဤ release version ဖြင့် supersede လုပ်ထားပါသည်။

## Compatibility boundary

Parser ပိုင် source နှင့် local module များအတွက် canonical AST execution သည် normative ဖြစ်နေဆဲ ဖြစ်ပါသည်။ Legacy line interpreter ကို older line-bodied function record များအတွက် explicit compatibility-only path အဖြစ်သာ ထားရှိပါသည်။ First-class callable value၊ parent-linked `EnvFrame` binding cell၊ cumulative memory budget၊ broad language async syntax နှင့် traits/interfaces semantics များသည် deferred ဖြစ်နေဆဲ ဖြစ်ပါသည်။

## Verification

Release candidate သည် Rust formatting၊ `-D warnings` ပါ strict Clippy၊ full native all-target/all-feature suite (integration test 254 ခု)၊ workspace နှင့် LSP isolation regression များ၊ documentation consistency validation၊ documentation regression harness၊ release preflight နှင့် `git diff --check` များကို အောင်မြင်ခဲ့ပါသည်။ Corrected commit ကို v2.1.14 tag မဖန်တီးမီ `master` သို့ push လုပ်ပြီးဖြစ်ပါသည်။

## ကိုးကားရန်

* [Runtime-state contract](RUNTIME_STATE_MM.md)
* [AST foundation status](P0_FOUNDATION_STATUS_MM.md)
* [Documentation navigation](DOCUMENTATION_NAVIGATION_MM.md)
* [Full changelog](../CHANGELOG_MM.md)
