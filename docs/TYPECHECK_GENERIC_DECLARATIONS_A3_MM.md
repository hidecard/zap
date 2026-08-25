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

## လက်ရှိ bounded parser-evidence checkpoint

လက်ရှိ provisional checkpoint တွင် Rust-reference-backed malformed-header fixture သုံးခု ထပ်တိုးထားသည်။ `generic_empty_params.zp` သည် `fn empty<>` ကို `generic type-parameter list cannot be empty` ဖြင့် reject လုပ်သည်။ `generic_duplicate_params.zp` သည် ထပ်နေသော `T` ကို `duplicate generic type parameter: T` ဖြင့် reject လုပ်သည်။ `generic_invalid_param.zp` သည် lowercase `t` ကို `invalid generic type parameter 't'` ဖြင့် reject လုပ်သည်။ သုံးခုလုံးသည် line 1, column 1 တွင် တည်ငြိမ်သော `ZAP-SYNTAX-001` / `SyntaxError` diagnostic ထုတ်ပေးပြီး candidate differential verifier ကလည်း ဤ corpus case များကို တိတိကျကျ ပြန်လည်ထုတ်ပေးသည်။ ထို checkpoint တွင် `generic_list_wrapper.zp` ကိုလည်း ထည့်ထားပြီး `list_keep<T>(list<T>) -> list<T>` ကို Rust reference က လက်ခံ၍ execute လုပ်သည်။ ထို့အပြင် `generic_list_wrapper_incompatible.zp` တွင် result ကို `text` သို့ assign လုပ်ပါက line 4, column 1 တွင် တည်ငြိမ်သော `variable 'wrong' expects text, got list<number>` diagnostic ကို ရရှိသည်။ ဤသည်မှာ bounded structural list evidence သာဖြစ်ပြီး arbitrary malformed-generic parsing၊ A3 အပြည့်အစုံ၊ candidate ownership၊ B4 သို့မဟုတ် self-hosting ကို မသက်သေပြပါ။ Rust parser တွင် လက်ခံထားသော `map<K, V>` parameter shape အတွက် nesting-aware signature splitter ကိုလည်း focused အနေဖြင့် ထည့်ထားသည်။ `generic_map_wrapper.zp` သည် `map_keep<K, V>(map<K, V>) -> map<K, V>` ကို လက်ခံ၍ execute လုပ်ပြီး `generic_map_wrapper_incompatible.zp` သည် line 4, column 1 တွင် `variable 'wrong' expects text, got map<text,number>` diagnostic ထုတ်ပေးသည်။ Candidate သည် ဤ exact fixture များကိုသာ mirror လုပ်သည်။ Rust-reference-only nested substitution probe တစ်ခုသည် `keep_nested<T>(option<list<T>>) -> option<list<T>>` ကို inferred `option<list<number>>` call ဖြင့် လက်ခံပြီး `text` သို့ assign လုပ်ပါက `variable 'wrong' expects text, got option<list<number>>` ဖြင့် reject လုပ်သည်။ ဤသည်မှာ deeper structural behavior နှင့် runtime substitution ကို မှတ်တမ်းတင်ခြင်းသာဖြစ်ပြီး provisional candidate သည် ဤ corpus ကို လက်ရှိ mirror မလုပ်သေးပါ။ Scope probe များအရ `identity<T>` အတွင်း `T` ကို အသုံးပြုနိုင်ပြီး inferred call များကို ဆက်လက်လက်ခံသည်။ သို့သော် `let leaked: T = 1` ကို line 4, column 1 တွင် reject လုပ်ပြီး `fn bad(value: T) -> T` ကို line 1, column 1 တွင် `unknown type annotation 'T'` ဖြင့် reject လုပ်သည်။ ဤသည်များသည် Rust-reference-only scope boundary များဖြစ်ပြီး candidate ownership မဟုတ်ပါ။ နောက်ထပ် bounded checkpoint တွင် `identity<T>` ကို export လုပ်သော `generic_cross_module_library.zp` နှင့် importing main fixture နှစ်ခုကို ထည့်ထားသည်။ `generic_cross_module.zp` သည် လက်ခံ၍ execute လုပ်ပြီး `generic_cross_module_incompatible.zp` သည် line 3, column 1 တွင် `variable 'wrong' expects text, got number` ဖြင့် reject လုပ်သည်။ Rust project checker သည် explicitly imported module များထဲမှ exported function signature များကို recursively စုစည်းပြီး importing main module ၏ generic call substitution check တွင် အသုံးပြုသည်။ Candidate ကတော့ ဤ exact imported `identity(…)` corpus shape ကိုသာ အသိအမှတ်ပြုသည်။ Imported function body checking၊ namespace collision policy၊ alias၊ non-generic cross-module inference နှင့် module-wide typed-IR/LSP propagation အပြည့်အစုံသည် ဆက်လက် open ဖြစ်သည်။ Generic constraint syntax ကိုလည်း သီးခြား deferred ထားသည်။ Rust သည် `fn bounded<T: number>(…)` ကို `invalid generic type parameter 'T: number'` ဖြင့် reject လုပ်ပြီး `fn bounded<T extends number>(…)` ကို `invalid generic type parameter 'T extends number'` ဖြင့် reject လုပ်သည်။ `where` form ကို `unknown return type annotation 'T where T: number'` ဖြင့် reject လုပ်သည်။ Candidate သည် ဤ diagnostic သုံးခုကို deferred corpus record အဖြစ်သာ တိတိကျကျ ပြန်ထုတ်ပေးသည်။ Trait-bound သို့မဟုတ် constraint semantics ကို implement လုပ်ထားသည်ဟု မဆိုပါ။ `identity<number>(1)` explicit-call probe ကို လက်ရှိ Rust project-check path က လက်ခံသော်လည်း runtime တွင် `undefined variable: number` ဖြင့် fail ဖြစ်သည်။ ထို့ကြောင့် explicit generic call syntax ကို language feature အဖြစ် implement လုပ်ထားခြင်း မဟုတ်ကြောင်းနှင့် အခြားနေရာများရှိ explanatory notation ကို language syntax ဟု မယူဆရကြောင်း မှတ်တမ်းတင်ထားသည်။ Candidate သည် ဤ exact deferred fixture ၏ static acceptance ကိုသာ mirror လုပ်သည်။ Generic class နှင့် alias probe များကိုလည်း deferred ထားသည်။ `generic_class_deferred.zp` ကို လက်ရှိ Rust project-check path က static လက်ခံသော်လည်း runtime တွင် `unexpected token after expression at 1:6` ဖြင့် fail ဖြစ်သည်။ `generic_alias_deferred.zp` ကို line 3, column 1 တွင် `unknown type annotation 'NumberBox<number>'` ဖြင့် reject လုပ်သည်။ Candidate သည် ဤ exact corpus boundary များကိုသာ mirror လုပ်ပြီး generic class instantiation၊ generic field metadata၊ alias expansion နှင့် alias diagnostic ကို implement လုပ်ထားသည်ဟု မဆိုပါ။ Bounded typed-IR checkpoint တစ်ခုတွင် Rust `generic_identity.zp` artifact ကို IR-node level အထိ တိတိကျကျ mirror လုပ်ထားသည်။ Generic function node တွင် `type_params: ["T"]` ပါဝင်ပြီး inferred call declaration နှစ်ခုတွင် `number` နှင့် `text` inferred type ပါဝင်သည်။ ဤသည်မှာ typed-IR metadata parity slice သာဖြစ်ပြီး generic multiple-parameter/wrapper IR၊ class/alias IR နှင့် imported-module IR များသည် ဆက်လက် open ဖြစ်သည်။ Rust LSP regression တစ်ခုသည် ထို declaration metadata ကို အသုံးပြု၍ generic function hover ကို `function `identity`<T> -> `T`` အဖြစ် render လုပ်ကြောင်း စစ်ဆေးထားသည်။ ထို့အပြင် သီးခြား document-symbol regression တစ်ခုသည် သက်ဆိုင်ရာ `identity` declaration အတွက် `function<T> in file:///generic-symbols.zp` detail ကို report လုပ်သည်။ ဤနှစ်ခုသည် bounded hover နှင့် document-symbol detail surface များဖြစ်ပြီး signature-help regression တစ်ခုက inferred `identity(1` call context ကို လက်ခံကာ `fn identity<T>(value: T) -> T` label နှင့် parameter metadata ကို ထိန်းသိမ်းကြောင်း စစ်ဆေးသည်။ Explicit generic call argument များကို deferred ထားဆဲဖြစ်ပြီး imported-module LSP propagation နှင့် candidate LSP implementation အပြည့်အစုံသည် ဆက်လက် open ဖြစ်သည်။

## ရှင်းလင်းစွာ deferred ထားသော scope

ဤ design သည် A3 အပြည့်အစုံပြီးပြီဟု မဆိုပါ။ Constraint နှင့် trait bound၊ generic class နှင့် alias၊ explicit generic call argument၊ higher-kinded form၊ variance၊ overload resolution၊ cross-module instantiation၊ full collection inference၊ closure capture semantics နှင့် generic metadata အပြည့်အစုံပါသော typed-IR/LSP များကို သီးခြား acceptance မရမချင်း deferred ထားသည်။ `identity<T>` slice တစ်ခုတည်းဖြင့် A3 complete မဖြစ်နိုင်ပါ။ Declaration၊ scope၊ constraint၊ arity၊ substitution၊ recursion၊ diagnostic၊ runtime နှင့် cross-platform evidence အားလုံး pass ပြီးမှသာ ordered A3 gate ကို ပိတ်နိုင်မည်။

## Ownership နှင့် release rule

A3 gate မအောင်မြင်မချင်း Rust သည် authoritative ဖြစ်နေမည်။ Reference behavior freeze လုပ်ပြီးနောက်မှသာ bootstrap candidate သည် corpus-limited mirror ကို implement လုပ်နိုင်သည်။ ဤ design record တစ်ခုတည်းကြောင့် B1/B2/B3/B4 သို့မဟုတ် self-hosting claim မပြုရ၊ design gate မှတ်တမ်းတင်ထားခြင်းတစ်ခုတည်းကြောင့် release အသစ်လည်း မဖြတ်ရ။

**ရေးသားသူ:** Manus AI
**Baseline:** Zap v2.11.16
