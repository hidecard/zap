# P0-01 Legacy-to-Native Migration Guidance

## Overview

Native runtime သည် canonical implementation ဖြစ်ပြီး retained Python runtime သည် older line-based programs အတွက် compatibility reference သာ ဖြစ်ပါသည်။ Native-only matrix row တစ်ခုသည် policy နှင့် migration note ကို တစ်ပြိုင်တည်း review လုပ်ထားမှသာ intentional ဖြစ်ပါသည်။

## Native-only fixtures

| Fixture | Observed boundary | Migration guidance |
|---|---|---|
| `native-only/arithmetic.zp` | Modern `let` declaration ကို arithmetic မတိုင်မီ အသုံးပြုထားပြီး legacy translator က declaration boundary တွင် ပယ်သည်။ | Modern declaration နှင့် arithmetic အတွက် native runtime ကို သုံးပါ။ Legacy program များသည် documented legacy declaration form ကို သုံးရပါမည်။ |
| `native-only/while_loop.zp` | Loop fixture သည် modern declaration boundary ကို သုံးပြီး legacy translator က loop ကို မတိုင်မီ ပယ်သည်။ | Loop ပါ program များကို native သို့ ပြောင်းပါ သို့မဟုတ် compatibility runtime ရှိစဉ် legacy declaration form ကို ထားပါ။ |
| `native-only/variables.zp` | Modern declarations နှင့် assignment များသည် legacy translator boundary အပြင်တွင် ရှိသည်။ | Declaration နှင့် assignment semantics အတွက် native ကို သုံးပါ။ |
| `native-only/let_binding.zp` | `let` သည် retained legacy runtime တွင် intentional native-only ဖြစ်သည်။ | Temporary compatibility အတွက်သာ legacy form ကို သုံးပြီး new code ကို native တွင် ရေးပါ။ |
| `native-only/append_function.zp` | Modern declaration boundary နှင့် native list `append` builtin ကို သုံးသည်။ | Native list operations ကို သုံးပါ။ Legacy runtime ကို API-compatible standard library အဖြစ် မယူဆရပါ။ |
| `native-only/assert_statement.zp` | Modern declaration boundary နှင့် native `assert` ကို သုံးသည်။ | Validation နှင့် tests အတွက် native assertions ကို သုံးပါ။ Legacy program များတွင် explicit replacement check လိုသည်။ |
| `native-only/join_function.zp` | Modern declaration boundary နှင့် native `join` ကို သုံးသည်။ | Native string/list API သို့မဟုတ် explicit compatibility helper ကို သုံးပါ။ |

Inventory တွင် runtime နှစ်ခုလုံး အောင်မြင်ပြီး output ကွဲသော `compatibility` rows သို့မဟုတ် legacy က လက်ခံပြီး native က ပယ်သော `deprecated` rows ပါနိုင်ပါသည်။ ထို rows များသည် review signals ဖြစ်ပြီး semantics ကို silent change လုပ်ခွင့် မဟုတ်ပါ။ Inventory ကို `scripts/inventory_legacy_parity.sh` ဖြင့် `target/legacy-parity-inventory.tsv` တွင် ထုတ်နိုင်ပါသည်။

## Migration strategy

1. `conformance/p0-01/matrix.tsv` သို့မဟုတ် inventory report ရှိ owning fixture နှင့် policy ကို ရှာပါ။
2. Runtime တစ်ခုခုကို ပြောင်းပြီးနောက် `scripts/test_p001_parity.sh` ကို run ပါ။
3. Native-only feature အတွက် application ကို native declaration နှင့် builtin boundary သို့ ပြောင်းပါ။
4. Compatibility သို့မဟုတ် deprecated row အတွက် owner၊ release impact နှင့် migration deadline ကို curated matrix ထဲမတင်မီ မှတ်တမ်းတင်ပါ။
5. Diagnostics၊ output normalization နှင့် migration notes ကို တစ်ခုတည်းသော reviewed change တွင် ထားပါ။

## CI integration

Curated matrix သည် GitHub Actions release gate ဖြစ်သည်။ Missing fixture၊ unknown policy၊ unexpected exit status သို့မဟုတ် unapproved common-output drift ရှိပါက job က fixture ID ဖြင့် fail ဖြစ်သည်။ Inventory command သည် deterministic ဖြစ်ပြီး release evidence အဖြစ် တွဲထားနိုင်သော်လည်း reviewed policy matrix ကို အစားထိုးမည် မဟုတ်ပါ။

## Support policy

- `common`: normalized output parity ဖြင့် runtime နှစ်ခုလုံးတွင် supported ဖြစ်သည်။
- `native-only`: canonical native runtime တွင် supported ဖြစ်ပြီး legacy compatibility ကို အာမမခံပါ။
- `rejected`: runtime နှစ်ခုလုံးတွင် intentionally unsupported သို့မဟုတ် malformed input ဖြစ်သည်။
- `compatibility`: successful output ကွဲနေသဖြင့် explicit compatibility decision လိုသည်။
- `deprecated`: legacy က လက်ခံပြီး native က ပယ်သည့် behavior ဖြစ်သဖြင့် migrate သို့မဟုတ် removal decision မှတ်တမ်းတင်ရမည်။
