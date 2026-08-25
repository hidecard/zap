# Zap v2.11.16

**Release အခြေအနေ:** Complete validation နှင့် public artifact/signature verification ပြီးစီးပြီး publish လုပ်ထားသည်။ Zap သည် B0 အဖြစ်သာ ဆက်ရှိသည်။

## အကျဉ်းချုပ်

Zap v2.11.16 တွင် exact direct option-constructor annotation shape တစ်ခုအတွက် provisional၊ corpus-limited B2 type-checker increment ကို publish လုပ်ထားသည်။ Published candidate သည် exact `some(1)` expression ကို `option<number>` အဖြစ် သတ်မှတ်ပြီး `let selected: option<number> = some(1)` ကို လက်ခံသည်။ Paired negative fixture သည် ထို direct expression ကို `text` သို့ assign လုပ်ပါက line 1၊ column 1 တွင် `variable 'wrong' expects text, got option<number>` diagnostic ဖြင့် reject လုပ်သည်။

ဤအရာသည် deterministic fixture pair တစ်ခုအတွက် evidence သာဖြစ်သည်။ General option-constructor inference၊ arbitrary constructor payload၊ result constructor၊ alias၊ variant narrowing၊ collection expression inference သို့မဟုတ် static type checking အပြည့်အစုံကို မဆိုလိုပါ။

## ပြောင်းလဲမှုများ

| နယ်ပယ် | ပြောင်းလဲမှု | Boundary |
|---|---|---|
| B2 candidate | Exact `some(1)` → `option<number>` recognition နှင့် direct mismatch evidence ထည့်ထားသည်။ | General option/result constructor inference မပါဝင် |
| Fixtures | Positive နှင့် incompatible assignment fixture အတွဲ ထည့်ထားသည်။ | Exact expression နှင့် direct annotation shape တစ်ခုတည်း |
| Differential gates | Native နှင့် candidate B2 verifier များကို deterministic output case 26 ခုအထိ တိုးချဲ့ထားသည်။ | Rust သည် reference owner ဖြစ်နေဆဲ |
| Ownership | Provisional `BOOT-032` မှတ်တမ်းတင်ထားသည်။ | Published evidence သည် corpus-limited ဖြစ်သည် |
| Documentation | English/Burmese contract၊ matrix၊ current status၊ roadmap နှင့် release notes update လုပ်ထားသည်။ | Broader inference နှင့် self-hosting ဆက်လက် deferred |

## Verification contract

Published source သည် native နှင့် Zap candidate B2 verifier၊ malformed-source safety၊ native tests၊ typecheck matrix parity၊ specification ownership၊ Markdown links၊ VS Code packaging၊ formatting၊ release-version validation၊ documentation consistency နှင့် exact committed release preflight အားလုံးကို အောင်မြင်ခဲ့သည်။ Public workflow တွင် source validation၊ Linux x86_64၊ macOS ARM64၊ Windows x86_64 နှင့် Publish jobs အားလုံး အောင်မြင်ခဲ့သည်။ Publish ပြီးသော artifact များသည် checksum၊ manifest၊ provenance နှင့် detached-signature verification များကို အောင်မြင်ခဲ့သည်။

## Deferred scope

General option-constructor inference၊ arbitrary payload expression၊ `ok`/`err` result constructor၊ option/result alias၊ variant narrowing၊ nested map၊ ရှိပြီးသား bounded corpus ပြင်ပ collection inference၊ compound guard၊ loop mutation၊ reassignment invalidation၊ arbitrary control flow၊ generic declaration၊ complete typed-IR ownership၊ package/build ownership၊ VM ownership နှင့် B4 self-rebuild acceptance များသည် သီးခြား design နှင့် evidence gate များနောက်တွင် ဆက်လက် deferred ဖြစ်သည်။
