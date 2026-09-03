# Zap v2.11.18

**Release အခြေအနေ:** Complete validation နှင့် public artifact/signature verification ပြီးနောက် publish လုပ်ထားသည်။ Zap သည် Bootstrap stage B0 အဖြစ်သာ ဆက်ရှိပြီး ဤ release ထဲရှိ B4 အလုပ်သည် bounded/provisional ဖြစ်သည်။

## အကျဉ်းချုပ်

Zap v2.11.18 သည် canonical parser-AST → B3 lowerer → B4 VM လမ်းကြောင်းကို bounded closure execution ဖြင့် တိုးချဲ့ထားပါသည်။ Nested AST function များသည် outer value များကို capture လုပ်နိုင်ပြီး closure value အဖြစ် return လုပ်ကာ independent captured environment များဖြင့် ပြန်လည် invoke လုပ်နိုင်ပါသည်။ ယခင် canonical AST control-flow၊ exception၊ class၊ inheritance နှင့် C3 `super()` slice များကိုလည်း ဆက်လက်ထိန်းသိမ်းထားပါသည်။

## ပြောင်းလဲမှုများ

| နယ်ပယ် | ပြောင်းလဲမှု | အကန့်အသတ် |
|---|---|---|
| Canonical AST closures | Nested function များသည် အသုံးပြုထားသော outer value များကို capture လုပ်ပြီး return/invoke လုပ်နိုင်သည်။ | Bounded lexical capture သာဖြစ်ပြီး full heap-level shared-cell သို့မဟုတ် cycle collector မပါဝင် |
| B4 module boundary | AST control-flow lowering ကို `bootstrap/b4/ast_control.zp` တွင် သီးခြားခွဲထားသည်။ | Complete semantics အတွက် Rust/native runtime က owner ဖြစ်နေဆဲ |
| Typed-IR handoff | ရှိပြီးသား typed-IR payload-to-VM slice အတွက် `seed_compile_typed_ir` ကို ထည့်ထားသည်။ | Complete typed-IR production သို့မဟုတ် compiler ownership မဟုတ် |
| Verification | Closure၊ control-flow၊ try/catch၊ literal-list `for` နှင့် typed-IR-to-VM gates များ ထည့်ထားသည်။ | Literal-list `for` သာ support လုပ်ပြီး general iterator များ deferred |

## Verification contract

Release source သည် bootstrap verifier matrix၊ native Rust tests၊ formatting၊ whitespace၊ release preflight၊ cross-platform build jobs၊ artifact manifest/checksum/provenance validation နှင့် detached-signature verification များကို အောင်မြင်ရမည်။ Authoritative release version ကို `native/Cargo.toml` မှ ရယူပြီး lockfile၊ CLI၊ documentation နှင့် release surfaces များနှင့် တိုက်ဆိုင်စစ်ဆေးရမည်။

## ဆက်လက် deferred ဖြစ်သောအရာများ

ဤ release သည် full B4 self-hosting မဟုတ်ပါ။ Arbitrary-program parser coverage၊ complete type inference၊ complete typed-IR production၊ package/build ownership၊ platform-seed acceptance၊ Rust-independent self-rebuild၊ production garbage collection၊ production asynchronous I/O၊ general runtime iterator၊ full trait runtime နှင့် complete object/type semantics များ ကျန်ရှိနေပါသည်။ `native_independent:false` ကို ရည်ရွယ်ချက်အတိုင်း ဆက်လက်ထားရှိပါသည်။
