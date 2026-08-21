# Compatibility နှင့် Deprecation Change Template

Language၊ runtime၊ standard-library၊ package သို့မဟုတ် diagnostic behavior ပြောင်းလဲသည့်အခါ ဤ template ကို အသုံးပြုရမည်။

## Change identity

| Field | Value |
|---|---|
| Change ID | `SPEC-NNN` |
| Target release | `vX.Y.Z` |
| Compatibility class | `normative` / `compatibility` / `deprecated` / `rejected` |
| Canonical specification section | `docs/LANGUAGE_SPEC_EN.md#...` နှင့် `docs/LANGUAGE_SPEC_MM.md#...` |
| Fixture owner | Repository path နှင့် လိုအပ်ပါက `#fragment` |

## Existing behavior

ယခင် behavior ကို native နှင့် ထိန်းသိမ်းထားသော legacy result ရှိပါက ထည့်သွင်း၍ ရှင်းပြရမည်။ ယခင် behavior သည် documentation တွင် ပါခဲ့ခြင်း၊ မရည်ရွယ်ဘဲ ဖြစ်နေခြင်း သို့မဟုတ် ownership index တွင် classification ရှိပြီးသားဖြစ်ခြင်းကို ဖော်ပြရမည်။

## New behavior

Normative behavior အသစ်၊ လက်ခံမည့် input၊ reject မည့် input၊ diagnostic၊ limit၊ determinism expectation နှင့် supported platform များကို ရှင်းပြရမည်။ Explicit decision မရှိဘဲ legacy acceptance ကို normative အဖြစ် မမြှင့်တင်ရပါ။

## Migration နှင့် deprecation

`deprecated` behavior ဖြစ်ပါက warning သို့မဟုတ် diagnostic code၊ notice ပါဝင်သည့် ပထမ release၊ အနည်းဆုံး compatibility ကာလ၊ replacement behavior နှင့် removal release decision ကို ဖော်ပြရမည်။ `compatibility` ဖြစ်ပါက ယခင် behavior ကို ဆက်လက်ထားရသည့်အကြောင်းနှင့် semantics တိတ်တဆိတ် မပြောင်းလဲစေရန် boundary ကို ဖော်ပြရမည်။ `rejected` ဖြစ်ပါက fail-closed example ထည့်ရမည်။

## Evidence နှင့် release gates

Regression သို့မဟုတ် corpus fixture၊ ownership-index row၊ bilingual documentation pair၊ changelog entry နှင့် verification command များကို စာရင်းပြုရမည်။ Formatter၊ strict Clippy၊ full native tests၊ focused parity/replay/ownership tests၊ deployment-policy validation နှင့် target-native CI အားလုံး green မဖြစ်မချင်း change ကို release-ready ဟု မသတ်မှတ်ရပါ။
