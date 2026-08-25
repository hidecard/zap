# အပိုင်း A — Compiler နှင့် Self-Hosting Acceptance Contract

**အခြေအနေ:** Design နှင့် acceptance contract ဖြစ်ပြီး အပိုင်း A အတွင်း deferred ထားသော အရာများ ပြီးစီးပြီဟု မဆိုလိုပါ။ Zap သည် B0 ဖြစ်နေဆဲဖြစ်ပြီး Rust သည် reference/compiler/runtime owner ဖြစ်နေဆဲဖြစ်သည်။

## ရည်ရွယ်ချက်

အပိုင်း A သည် corpus-limited bootstrap evidence မှ self-hosted compiler အခြေအနေသို့ ရွှေ့ရန် လိုအပ်သောအလုပ်များကို ဖော်ပြသည်။ အလုပ်များကို သတ်မှတ်ထားသော gate အစဉ်အတိုင်း လုပ်ရမည်။ Fixture တစ်ခု သို့မဟုတ် syntax example တစ်ခုတည်းဖြင့် candidate ကို complete compiler ownership သို့ မတိုးမြှင့်ရ။

> အပိုင်း A item တစ်ခုသည် syntax၊ semantics၊ negative behavior၊ diagnostic၊ deterministic artifact၊ bilingual documentation၊ ownership record နှင့် cross-platform regression evidence အားလုံးကို gate တစ်ခုတည်းတွင် လက်ခံပြီးမှသာ ပြီးစီးသည်။

## Ownership စည်းမျဉ်း

သက်ဆိုင်ရာ acceptance gate မအောင်မြင်မချင်း semantics၊ diagnostics၊ typed IR၊ package/build behavior၊ VM execution နှင့် supported release target များအတွက် Rust သည် authoritative ဖြစ်သည်။ Bootstrap implementation များကို candidate အဖြစ် run နိုင်သော်လည်း owned corpus အပေါ် Rust reference နှင့် နှိုင်းယှဉ်ရမည်။ Reference pipeline ကို တိတ်တဆိတ် အစားထိုးခြင်း မပြုရ။

## အပိုင်း A gate အစဉ်

| Gate | Work package | အနည်းဆုံး လက်ခံရမည့် evidence | Ownership ရလဒ် |
|---|---|---|---|
| A1 | Complete type-inference contract | Type lattice၊ expression coverage၊ positive/negative fixture၊ inference determinism နှင့် Rust/candidate differential check | Full expression coverage မအောင်မြင်မချင်း candidate သည် provisional |
| A2 | Broader basic-type inference | `text`၊ `number`၊ `bool`၊ `list`၊ `map` နှင့် `none` cross-product matrix၊ direct value၊ expression၊ call၊ branch နှင့် invalid combination များ | Literal-only slice များဖြင့် broad claim မပြုရ |
| A3 | Generic declarations | Grammar၊ AST၊ scope၊ constraints၊ arity၊ substitution၊ recursion limit၊ diagnostic နှင့် runtime boundary test | အားလုံးမပြီးမချင်း user-defined generic syntax deferred |
| A4 | Collection inference | Homogeneous/heterogeneous list/map rule၊ nested value၊ empty collection၊ key/value constraint၊ mutation effect နှင့် alias | Exact-literal/element slice ရှိရုံဖြင့် gate မပြီး |
| A5 | Nested နှင့် compound inference | Recursive map/list၊ deeper expression၊ compound guard၊ branch join နှင့် short-circuit behavior | Recursive/compound rule တစ်ခုစီတွင် paired acceptance/rejection evidence လို |
| A6 | Flow နှင့် mutation analysis | Loop join၊ mutation၊ reassignment invalidation၊ alias fact၊ closure capture နှင့် post-branch restoration | Invalidating write နောက်တွင် narrowing fact မကျန်ရ |
| A7 | Parser coverage | Arbitrary valid program နှင့် Unicode၊ malformed၊ overflow၊ indentation၊ delimiter၊ determinism corpus | Owned-corpus parser evidence သည် arbitrary-program ownership မဟုတ် |
| A8 | Diagnostic parity | Error kind/code၊ message normalization၊ source line/column၊ JSON shape၊ LSP range conversion နှင့် failure exit behavior | Rust နှင့် bootstrap သည် failure တစ်ခုတည်းကို ဖော်ပြရမည် |
| A9 | General typed IR | Stable schema၊ supported AST form အားလုံး၊ inferred type၊ span၊ determinism နှင့် byte-for-byte/reference semantic comparison | လက်ရှိ annotated-declaration artifact သည် reference-only |
| A10 | Package/build ownership | Manifest၊ lockfile၊ resolver၊ dependency validation၊ offline build၊ test runner နှင့် reproducible package artifact ကို bootstrap က implement လုပ်ခြင်း | Foundation check သာဖြင့် ownership မလွှဲ |
| A11 | VM execution ownership | Bootstrap-produced IR ကို Rust နှင့် behavior၊ limit၊ error နှင့် security boundary တူညီစွာ execute လုပ်ခြင်း | Equivalence လက်ခံမချင်း native VM သည် authoritative |
| A12 | Platform-seed acceptance | Linux x86_64၊ macOS ARM64 နှင့် Windows x86_64 တွင် bootstrap build/run reproducible ဖြစ်ပြီး artifact verify လုပ်ခြင်း | Source/toolchain contract တူညီသော platform evidence လို |
| A13 | B4 self-rebuild | Bootstrap compiler က documented seed ကို build၊ rebuilt compiler က မိမိကိုယ်ကို ပြန် build၊ output deterministic ဖြစ်ပြီး gate အားလုံး pass | ဤ gate ပြီးမှသာ B4/self-hosted wording သုံးရမည် |

## Gate တိုင်းအတွက် လိုအပ်သော evidence

Gate တိုင်းတွင် Rust reference fixture set၊ bootstrap candidate fixture set၊ positive/negative case၊ deterministic repeated run၊ stable diagnostic၊ လိုအပ်ပါက malformed-input safety၊ machine-readable ownership record နှင့် English/Burmese synchronized documentation ပါရမည်။ Gate က သက်သေပြသော exact scope နှင့် ရည်ရွယ်ချက်ရှိရှိ deferred ထားသော scope ကို သီးခြားဖော်ပြရမည်။

## မပြုရမည့် claims

သက်ဆိုင်ရာ gate မအောင်မြင်မချင်း မည်သည့် section-A checkpoint မဆို Zap-only compiler၊ complete inference၊ complete parser coverage၊ full diagnostic parity၊ typed-IR ownership၊ package/build ownership၊ VM ownership၊ platform-seed ownership သို့မဟုတ် B4 self-hosting ဟု မဆိုရ။ ပိုကြီးသော gate များ open ဖြစ်နေသော်လည်း independently verified bounded increment အတွက် release ထုတ်နိုင်သည်။ သို့သော် release notes တွင် B0 boundary ကို မဖြုတ်ရ။

## Release စည်းမျဉ်း

Section-A program သည် independently verified bounded increment များအတွက် intermediate release များ ထုတ်နိုင်သည်။ A program ပြီးဆုံးမှုအတွက် release ကို A1 မှ A13 အားလုံး pass၊ bilingual contract နှင့် ownership ledger synchronized၊ exact committed preflight clean၊ cross-platform release job အားလုံး pass နှင့် public checksum၊ manifest၊ provenance၊ signature များ independently verify ပြီးမှသာ tag လုပ်ရမည်။

## နောက် implementation ဆုံးဖြတ်ချက်

A2 design gate တွင် arithmetic၊ text addition၊ boolean logic၊ comparison၊ result construction၊ list arithmetic နှင့် map arithmetic ကို cover လုပ်သော Rust-reference-backed exact-expression matrix အသေးတစ်ခုကို ယခု implement နှင့် verify လုပ်ပြီးဖြစ်သည်။ နောက် bounded A3 checkpoint တွင် AST type-parameter metadata၊ inferred identity call၊ multiple-parameter substitution၊ structural `option<T>` နှင့် `result<T>` wrapper substitution၊ generic arity diagnostic၊ conflicting-substitution diagnostic၊ generic return check နှင့် runtime substitution check ပါဝင်သော Rust-backed `identity<T>` နှင့် `same<T>` declaration များကို ထည့်ထားသည်။ A1 complete inference၊ A2 broader cross-product coverage နှင့် A3 declaration contract အပြည့်အစုံများသည် ဆက်လက် open ဖြစ်သည်။ ရှိပြီးသား literal၊ constructor၊ exact-expression သို့မဟုတ် bounded generic slice များကို complete inference သို့မဟုတ် self-hosting ဟု မယူဆဘဲ evidence ကို အဆင့်လိုက် တိုးချဲ့ရမည်။
