# Zap Language Specification

**ရည်ရွယ်ချက်:** Zap ၏ syntax၊ typing၊ runtime behavior၊ diagnostics၊ compatibility နှင့် version decision များအတွက် canonical normative owner ဖြစ်သည်။
**လမ်းညွှန်:** [Documentation hub](DOCUMENTATION_NAVIGATION_MM.md) · [လေ့လာရေး guide](LEARN_ZAP_MM.md) · [Syntax reference](SYNTAX_GUIDE.md) · [Stdlib reference](STDLIB_INDEX_MM.md) · [Package author guide](PACKAGE.md) · [Runtime state](RUNTIME_STATE_MM.md) · [Deployment boundaries](DEPLOYMENT_MM.md)

**Specification အခြေအနေ:** Zap v2.2.7 အတွက် normative foundation

ဤစာတမ်းသည် language semantics များအတွက် canonical index ဖြစ်သည်။ အဟောင်း guide တစ်ခုခုသည် ဤစာတမ်းနှင့် မကိုက်ညီပါက specification နှင့် ကိုက်ညီအောင် implementation/test များကို ပြင်ရမည်။ Legacy behavior ကို အလိုအလျောက် normative အဖြစ် မယူဆဘဲ compatibility exception အဖြစ် အတိအကျ မှတ်တမ်းတင်ရမည်။

## ၁။ Source နှင့် execution model

Zap program သည် `.zp` extension ပါသော UTF-8 source file ဖြစ်သည်။ Canonical native pipeline သည် **source → lexer → AST parser → evaluator** ဖြစ်သည်။ Evaluator သည် parse လုပ်ပြီးသား AST ကို တိုက်ရိုက် execute လုပ်ပြီး function/method body များကို source line ပြန်တည်ဆောက်ခြင်း မပြုပါ။ Source file ကို `zap <file.zp>` သို့မဟုတ် `zap run <file.zp>` ဖြင့် run နိုင်သည်။

Indentation သည် block များကို သတ်မှတ်သည်။ Statement များတွင် declaration၊ assignment၊ expression statement၊ `say`၊ `return`၊ conditional၊ loop၊ function၊ class နှင့် explicit module/import form များ ပါဝင်သည်။ Parser သည် malformed structure ကို panic မဖြစ်စေဘဲ structured diagnostic ဖြင့် reject ရမည်။

## ၂။ Expression နှင့် precedence

Operator များသည် အောက်ပါအတိုင်း အင်အားအကြီးဆုံးမှ အနည်းဆုံးသို့ bind လုပ်သည်။

| အဆင့် | Operator | Associativity |
|---|---|---|
| ၁ | grouping `(...)`၊ call၊ indexing၊ member access | ရေးထားသည့် chain အတိုင်း left-to-right evaluation |
| ၂ | unary `-`၊ `not` | right-to-left |
| ၃ | exponentiation နှင့် multiplicative arithmetic | အဆင့်တစ်ခုအတွင်း left-to-right |
| ၄ | additive arithmetic နှင့် concatenation | left-to-right |
| ၅ | comparison: `<`၊ `<=`၊ `>`၊ `>=`၊ `==`၊ `!=` | left-to-right |
| ၆ | `and` | short-circuit ဖြင့် left-to-right |
| ၇ | `or` | short-circuit ဖြင့် left-to-right |

မရှင်းလင်းနိုင်သည့် ရည်ရွယ်ချက်အတွက် parentheses သည် normative escape ဖြစ်သည်။ Boolean operator များသည် short-circuit လုပ်ပြီး မရောက်နိုင်သော right-hand operand ကို မ evaluate လုပ်ရ။

## ၃။ Values နှင့် typing

Core value category များမှာ `text`၊ `number`၊ `bool`၊ `list`၊ `map`၊ `object`၊ `function`၊ `none` နှင့် runtime က ဖော်ပြသည့် typed `result`/`option` form များ ဖြစ်သည်။ Annotation များတွင် primitive name များနှင့် `list<number>`၊ `map<text, number>`၊ `option<text>`၊ `result<text>` ကဲ့သို့ bounded generic form များကို သုံးနိုင်သည်။ `function` annotation သည် first-class callable value များကို လက်ခံသည်။ `any` သည် အတိအလင်း ခွင့်ပြုထားသည့် escape hatch ဖြစ်ပြီး runtime coercion ကို မဆိုလိုပါ။

Static check များသည် declared annotation၊ collection element expectation၊ function argument၊ return value နှင့် implementation က သိနိုင်သည့် control-flow narrowing များကို စစ်ဆေးသည်။ Dynamic boundary များတွင် runtime check သည် အဆုံးသတ်အာဏာရှိသည်။ Mismatch ကို undocumented Rust panic သို့မဟုတ် မတည်ငြိမ်သော string မဟုတ်ဘဲ structured diagnostic ဖြင့် ဖော်ပြရမည်။

## ၄။ Functions၊ calls နှင့် closures

Function တစ်ခုတွင် name၊ ordered parameter များ၊ optional annotation များ၊ optional default expression များ၊ optional return annotation နှင့် AST body ပါဝင်သည်။ Declared function name ကို first-class callable value အဖြစ် evaluate လုပ်ပြီး ထို value ကို assign လုပ်ခြင်း၊ argument အဖြစ်ပေးခြင်း၊ return ပြန်ခြင်းနှင့် callable expression တစ်ခုခုမှ invoke လုပ်ခြင်းတို့ ပြုလုပ်နိုင်သည်။ Call contract အရ positional သို့မဟုတ် named argument သုံးနိုင်သည်။ Argument မပေးထားသည့်အခါ default ကို evaluate လုပ်သည်။ Duplicate၊ unknown၊ တစ်ကြိမ်ထက်ပိုပေးသော argument သို့မဟုတ် callable မဟုတ်သည့် value ကို invoke လုပ်ခြင်းတို့သည် error ဖြစ်ပြီး arity/type failure များသည် deterministic runtime message သုံးသည်။ Closure သည် implementation က သတ်မှတ်ထားသော lexical environment ကို capture လုပ်ပြီး ownership/cycle behavior သည် memory contract အတိုင်း ဖြစ်ရမည်။ Callable value ကို `<callable>` ဟု ပြသပြီး deterministic `{"__zap_variant":"callable"}` marker အဖြစ် serialize လုပ်သည်။ Executable code မပါသောကြောင့် ထို marker ကို deserialize ပြန်မလုပ်နိုင်ပါ။

## ၅။ Control flow နှင့် modules

`if`/`else`၊ `while` နှင့် `for` များသည် source order အတိုင်း execute လုပ်သည်။ Loop condition ကို iteration တစ်ကြိမ်စီမတိုင်မီ evaluate လုပ်သည်။ `return` သည် လက်ရှိ function မှသာ ထွက်သည်။ Explicit `module` နှင့် `import` declaration များသည် relative၊ bounded path များကို deterministic source order ဖြင့် resolve လုပ်သည်။ Absolute path၊ traversal component၊ malformed entry နှင့် circular module graph များကို stable diagnostic ဖြင့် reject ရမည်။ Legacy `use` import သည် သတ်မှတ်ထားသည့် compatibility syntax အဖြစ် ဆက်လက်ရှိနိုင်သည်။

## ၆။ Runtime ownership နှင့် asynchronous boundary

Object field များသည် documented single-threaded `Rc<RefCell>` ownership model ကို သုံးသည်။ Cyclic object graph ကို discard မလုပ်မီ explicit cycle-breaking operation လုပ်ရမည်။ Runtime သည် default အားဖြင့် thread-safe မဟုတ်ဘဲ ဤ boundary သည် ရည်ရွယ်ချက်ရှိရှိ သတ်မှတ်ထားခြင်း ဖြစ်သည်။

လက်ရှိ async executor သည် deterministic ဖြစ်ပြီး poll-budget သုံးသည်။ Language `async fn` call များသည် caller ၏ `RuntimeState` မှတစ်ဆင့် completed value ကို schedule လုပ်ပြီး context ပိုင် `ScheduledFuture` ပြန်ပေးသည်။ `await` နှင့် `task_join` သည် result ကို consume မလုပ်မီ executor ကို drive လုပ်ပြီး `task_is_ready` သည် poll မလုပ်ဘဲ readiness ကို စောင့်ကြည့်သည်။ Runtime သည် joinable task၊ cancellation-aware join၊ timeout propagation နှင့် typed task failure များကိုလည်း ပေးသည်။ ၎င်းသည် production I/O reactor မဟုတ်ပါ။ Blocking call၊ socket readiness၊ worker scheduling၊ shutdown နှင့် foreign blocking work ကို forced cancellation ပြုလုပ်ခြင်းတို့အတွက် `ASYNC_BOUNDARIES_MM.md` ထဲရှိ သီးခြား production boundary contract ကို လိုက်နာရမည်။

## ၇။ Diagnostics နှင့် compatibility

User-facing diagnostic တိုင်းတွင် severity၊ stable code၊ message၊ ရရှိနိုင်ပါက source location၊ notes နှင့် help တို့ကို ထိန်းသိမ်းရမည်။ CLI နှင့် LSP consumer များသည် semantic diagnostic field တစ်စုံတစ်ရာတည်းကို မျှဝေရမည်။ Compatibility behavior ကို **normative**၊ **compatibility**၊ **deprecated** သို့မဟုတ် **rejected** အဖြစ် အမည်တပ်ရမည်။ Fixture အဟောင်းတစ်ခုက လက်ခံသောကြောင့်သာ behavior တစ်ခုကို normative မသတ်မှတ်ရ။

လက်ရှိ release line သည် v2.2.7 ဖြစ်သည်။ Semantics ပြောင်းလဲမှုတစ်ခုအတွက် specification update၊ bilingual documentation parity၊ conformance test၊ changelog entry နှင့် explicit version decision လိုအပ်သည်။ Release artifact များသည် pinned Rust toolchain၊ formatting၊ strict Clippy၊ native tests၊ provenance နှင့် signature gate များကို ဆက်လက်အောင်မြင်ရမည်။ အနာဂတ် change များအတွက် bilingual [`COMPATIBILITY_CHANGE_TEMPLATE_EN.md`](COMPATIBILITY_CHANGE_TEMPLATE_EN.md) နှင့် [`COMPATIBILITY_CHANGE_TEMPLATE_MM.md`](COMPATIBILITY_CHANGE_TEMPLATE_MM.md) records များကို အသုံးပြုရမည်။

## ၈။ Conformance ownership

Parser သည် syntax နှင့် AST construction ကို ပိုင်ဆိုင်သည်။ Evaluator သည် runtime expression နှင့် statement behavior ကို ပိုင်ဆိုင်သည်။ Diagnostics module သည် stable error contract ကို ပိုင်ဆိုင်သည်။ Registry module သည် package transport၊ authentication၊ checksum၊ signature နှင့် cache policy ကို ပိုင်ဆိုင်သည်။ CI သည် repository တွင် သတ်မှတ်ထားသည့် gate များကို enforce လုပ်သည်။ Subsystem တစ်ခုသည် အခြား subsystem ၏ contract ကို တိတ်တဆိတ် ပြန်လည်သတ်မှတ်ခွင့် မရှိပါ။

## Specification ownership index

Machine-readable rule-to-section-to-fixture map သည် [`SPEC_OWNERSHIP_INDEX.tsv`](SPEC_OWNERSHIP_INDEX.tsv) ဖြစ်ပြီး field နှင့် migration contract ကို [`SPEC_OWNERSHIP_EN.md`](SPEC_OWNERSHIP_EN.md) နှင့် [`SPEC_OWNERSHIP_MM.md`](SPEC_OWNERSHIP_MM.md) တွင် မှတ်တမ်းတင်ထားသည်။ CI သည် index ထဲရှိ English section၊ Burmese section နှင့် fixture owner တစ်ခုချင်းစီ ရှိကြောင်း စစ်ဆေးပြီး unique rule ID နှင့် required domain coverage ကိုလည်း enforce လုပ်သည်။

## ဆက်စပ် normative contract များ

အောက်ပါစာတမ်းများသည် အသေးစိတ် subcontract များကို ပေးပြီး ဤ specification နှင့်အတူ bilingual ဖြစ်နေရမည်။

| Contract | English | Burmese |
|---|---|---|
| Diagnostics | `DIAGNOSTIC_MODEL_EN.md` | `DIAGNOSTIC_MODEL_MM.md` |
| Memory | `MEMORY_MODEL_EN.md` | `MEMORY_MODEL_MM.md` |
| Async boundary | `ASYNC_BOUNDARIES_EN.md` | `ASYNC_BOUNDARIES_MM.md` |
| Syntax reference | `SYNTAX_GUIDE_EN.md` | `SYNTAX_GUIDE_MM.md` |
| Standard library | `STDLIB_TEXT_MATH_COLLECTION_EN.md` | `STDLIB_TEXT_MATH_COLLECTION_MM.md` |

**လက်ရှိကန့်သတ်ချက်:** ဤစာတမ်းသည် canonical semantic foundation နှင့် navigation point ဖြစ်သည်။ ချဲ့ထွင်ထားသော ownership index တွင် post-review LSP၊ standard-library determinism၊ memory-budget၊ registry-transport၊ benchmark-provenance နှင့် release-version contract များအပါအဝင် stable rule ၃၆ ခု ပါဝင်သည်။ ကျန်ရှိသည့်အလုပ်မှာ အခြား fragmented rule တစ်ခုချင်းစီကို ဤစာတမ်း သို့မဟုတ် အတိအလင်း link ချိတ်ထားသည့် normative subcontract ထဲသို့ ရွှေ့ရန်၊ rule တစ်ခုချင်းစီအတွက် parser/evaluator conformance fixture ထည့်ရန်နှင့် မဖြေရှင်းရသေးသော legacy behavior ၏ version ownership ကို မှတ်တမ်းတင်ရန် ဖြစ်သည်။
