# A3 User-Defined Generic Declaration — Design နှင့် Acceptance Record

**အခြေအနေ:** Design gate ဖြစ်ပြီး A3 မပြီးစီးသေးပါ။ Zap သည် B0 အဖြစ်သာ ရှိနေပြီး Rust သည် reference compiler၊ type checker၊ runtime နှင့် diagnostic owner ဖြစ်နေဆဲပါ။

## ရည်ရွယ်ချက်

ဤ record သည် user-defined generic declaration အတွက် အန္တရာယ်နည်းသော A3 အစပိုင်း implementation target ကို သတ်မှတ်သည်။ ရှိပြီးသား `list<T>`၊ `map<K, V>`၊ `option<T>` နှင့် `result<T>` annotation များသည် generic declaration မဟုတ်သောကြောင့် ဤ gate ကို မဖြည့်ဆည်းပါ။ Declaration feature ကို Rust reference တွင် အရင် implement လုပ်ပြီးနောက် bootstrap candidate က differential evidence ဖြင့် ပြန်ထုတ်ရမည်။

## ကနဦး syntax ဆုံးဖြတ်ချက်

ကနဦး declaration ပုံစံမှာ အောက်ပါအတိုင်း ဖြစ်သည်။

```zap
fn identity<T>(value: T) -> T:
    return value
```

Declaration type-parameter list ကို function name နှင့် parameter list ကြားတွင် ထားမည်။ Type parameter သည် uppercase letter ဖြင့် စသော ASCII identifier ဖြစ်ရမည်။ Duplicate parameter၊ အလွတ် list၊ bracket မမှန်ခြင်းနှင့် မကြေညာထားသော type parameter များကို reject လုပ်ရမည်။ ပထမ implementation slice တွင် parameter နှင့် return annotation များအတွင်း type parameter သုံးခွင့်ရှိမည်ဖြစ်ပြီး call-site explicit type argument syntax မထည့်သေးပါ။

လက်ခံရမည့် ပထမ semantics များမှာ အောက်ပါအတိုင်း ဖြစ်သည်။

| Case | လိုအပ်သောရလဒ် |
|---|---|
| `identity<number>(1)` ကို call type inference ဖြင့် အသုံးပြုခြင်း | `number` အဖြစ် infer လုပ်ပြီး လက်ခံရန် |
| `identity<text>("x")` ကို call type inference ဖြင့် အသုံးပြုခြင်း | `text` အဖြစ် infer လုပ်ပြီး လက်ခံရန် |
| `same<T>(left: T, right: T)` တွင် static type တူသော argument နှစ်ခု | substitution တစ်ခုတည်းကို တည်ငြိမ်စွာ လက်ခံရန် |
| `same<T>(1, "x")` | stable incompatible-substitution diagnostic ဖြင့် reject လုပ်ရန် |
| `identity<T>` က substituted return type နှင့် မကိုက်ညီသော value ပြန်ပေးခြင်း | return location တွင် reject လုပ်ရန် |

Call column ထဲရှိ angle-bracket ဥပမာများသည် inferred type ကို ဖော်ပြခြင်းသာဖြစ်ပြီး explicit call syntax မဟုတ်ပါ။ Explicit generic call argument ကို သီးခြား grammar ဆုံးဖြတ်ချက်အထိ deferred ထားသည်။

## Substitution နှင့် safety rules

Call တစ်ခုစီတွင် checker သည် annotated argument များမှ declared type parameter တစ်ခုချင်းစီအတွက် substitution ကို စုဆောင်းပြီး ထပ်ခါတလဲလဲ အသုံးပြုသည့်နေရာများ၏ type များ တူညီကြောင်း စစ်ရမည်။ Substitution မရခြင်း သို့မဟုတ် ဆန့်ကျင်သော substitution ဖြစ်ခြင်းကို implicit `any` အဖြစ် မချဲ့ဘဲ error အဖြစ် သတ်မှတ်ရမည်။ Substitution သည် support လုပ်ထားသော primitive နှင့် wrapper annotation များအတွက် structural ဖြစ်ပြီး nested-annotation arity နှင့် map-key restriction များကို ရှိပြီးသား contract အတိုင်း ထိန်းသိမ်းရမည်။ Recursive substitution depth ကို 32 အထိ ကန့်သတ်ရမည်။ Depth ကျော်လွန်ပါက stable type diagnostic ဖြင့် fail closed လုပ်ရမည်။ Runtime call သည် generic annotation ကို မသိသော concrete type အဖြစ် လက်ခံမည့်အစား static checking နှင့် substitution result တူညီရမည်။

## ကနဦး slice အတွက် လိုအပ်သော evidence

ကနဦး A3 acceptance checkpoint အတွက် Rust parser acceptance/rejection fixture၊ declared type parameter များကို မှတ်တမ်းတင်သည့် canonical AST output၊ numeric/text substitution၊ multiple type parameter၊ structural `option<T>`/`result<T>` wrapper၊ conflicting substitution နှင့် generic arity အတွက် native static-check acceptance/rejection fixture၊ deterministic repeated run၊ substitution ပြီးနောက် runtime return checking၊ stable JSON diagnostic၊ bootstrap candidate differential output၊ malformed-source no-panic coverage၊ bilingual documentation နှင့် provisional ownership record လိုအပ်သည်။ Parser၊ static checker၊ evaluator၊ typed-IR၊ LSP နှင့် bootstrap candidate ပြောင်းလဲမှုများကို တစ်စုတစ်စည်းတည်း စစ်ဆေးရမည်။

## ရှင်းလင်းစွာ deferred ထားသော scope

ဤ design သည် A3 အပြည့်အစုံပြီးပြီဟု မဆိုပါ။ Constraint နှင့် trait bound၊ generic class နှင့် alias၊ explicit generic call argument၊ higher-kinded form၊ variance၊ overload resolution၊ cross-module instantiation၊ full collection inference၊ closure capture semantics နှင့် generic metadata အပြည့်အစုံပါသော typed-IR/LSP များကို သီးခြား acceptance မရမချင်း deferred ထားသည်။ `identity<T>` slice တစ်ခုတည်းဖြင့် A3 complete မဖြစ်နိုင်ပါ။ Declaration၊ scope၊ constraint၊ arity၊ substitution၊ recursion၊ diagnostic၊ runtime နှင့် cross-platform evidence အားလုံး pass ပြီးမှသာ ordered A3 gate ကို ပိတ်နိုင်မည်။

## Ownership နှင့် release rule

A3 gate မအောင်မြင်မချင်း Rust သည် authoritative ဖြစ်နေမည်။ Reference behavior freeze လုပ်ပြီးနောက်မှသာ bootstrap candidate သည် corpus-limited mirror ကို implement လုပ်နိုင်သည်။ ဤ design record တစ်ခုတည်းကြောင့် B1/B2/B3/B4 သို့မဟုတ် self-hosting claim မပြုရ၊ design gate မှတ်တမ်းတင်ထားခြင်းတစ်ခုတည်းကြောင့် release အသစ်လည်း မဖြတ်ရ။

**ရေးသားသူ:** Manus AI
**Baseline:** Zap v2.11.16
