# Zap Bootstrap နှင့် Self-Hosting Contract

**အခြေအနေ:** Zap v2.11.7 အတွက် B0 reference baseline

Zap ၏ self-hosting roadmap သည် အဆင့်လိုက်ဖြစ်သည်။ လက်ရှိ release သည် **Rust reference/native implementation** ဖြစ်ပြီး fully Zap-only compiler မဖြစ်သေးပါ။ Normative stage contract၊ သီးခြား version identity များနှင့် machine-readable ownership record များကို [`bootstrap/contracts`](../bootstrap/contracts/BOOTSTRAP_CONTRACT_MM.md) အောက်တွင် ထိန်းသိမ်းထားသည်။

## လက်ရှိ B0 boundary

Reference pipeline သည် အောက်ပါအတိုင်း ဖြစ်သည်။

```text
Zap source -> Rust lexer -> AST parser -> evaluator/runtime
```

ထို့ကြောင့် လက်ရှိ compiler ကို build လုပ်ရန် Rust/Cargo လိုအပ်နေသေးသည်။ Operating-system loader နှင့် explicit documented platform boundary များကို infrastructure boundary အဖြစ် လက်ခံထားပြီး အခြား language runtime နှင့် framework များကို လက်ရှိ Zap compiler path တွင် မလိုအပ်စေရ။

## B1 candidate status

ပထမဆုံး Zap-owned lexer candidate ကို [`bootstrap/b1/lexer.zp`](../bootstrap/b1/lexer.zp) တွင် ထည့်ထားသည်။ ၎င်းသည် initial owned corpus အတွက် identifier၊ number၊ text၊ comment၊ whitespace၊ operator၊ delimiter၊ Unicode နှင့် fail-closed diagnostic path များကို လက်ရှိအခြေအနေတွင် cover လုပ်သည်။ [`scripts/bootstrap/verify_b1_lexer.sh`](../scripts/bootstrap/verify_b1_lexer.sh) သည် candidate ကို run လုပ်ပြီး B0 token/diagnostic artifact များနှင့် output ကို နှိုင်းယှဉ်သည်။

ဤအရာသည် **corpus-limited B1 foundation** ဖြစ်ပြီး B1 compiler အပြည့်အစုံ မဟုတ်သေးပါ။ Candidate သည် reference owner မဟုတ်သေး၊ Rust lexer ကို မအစားထိုးသေးပါ။ Bootstrap stage claim ကို မြှင့်ရန် differential fixture များဖြင့် ဆက်လက်ချဲ့ထွင်ရမည်။

Native reference parser အတွက် additive differential corpus ကို [`bootstrap/fixtures/parser/compound.zp`](../bootstrap/fixtures/parser/compound.zp) တွင် ထည့်ထားပြီး canonical AST snapshot နှင့် syntax-rejection diagnostic fixture ပါရှိသည်။ [`scripts/bootstrap/verify_b1_parser.sh`](../scripts/bootstrap/verify_b1_parser.sh) သည် byte-for-byte reproducibility နှင့် reference output ကို စစ်ဆေးသည်။ ဤအရာသည် parser contract evidence သာဖြစ်ပြီး full Zap-owned parser ရှိပြီဟု မဆိုလိုပါ။

ပထမဆုံး Zap-written parser candidate ကို [`bootstrap/b1/parser.zp`](../bootstrap/b1/parser.zp) တွင် ထည့်ထားသည်။ ၎င်းသည် arithmetic declaration fixture နှင့် map၊ list၊ postfix indexing၊ binary operator၊ conditional/return ပါသော function နှင့် call ကို cover လုပ်သည့် compound corpus အပြင် missing-bracket rejection path တစ်ခုကို ရည်ရွယ်ချက်ရှိရှိ ပိုင်ဆိုင်သည်။ [`scripts/bootstrap/verify_b1_parser_candidate.sh`](../scripts/bootstrap/verify_b1_parser_candidate.sh) သည် output များကို B0 artifact များနှင့် byte-for-byte နှိုင်းယှဉ်သည်။ ဤသည်မှာ **provisional၊ corpus-limited candidate** ဖြစ်ပြီး Rust parser ကို မအစားထိုးသေး၊ fixture-scoped parsing assumption များ ကျန်ရှိနေသေးကာ B0 ထက် stage claim မမြှင့်သေးပါ။

## B2 conformance foundation

Repository တွင် reference-only B2 conformance gate ကို [`scripts/bootstrap/verify_b2_typecheck.sh`](../scripts/bootstrap/verify_b2_typecheck.sh) အဖြစ် ထည့်ထားသည်။ ၎င်းသည် annotated typed-IR artifact ကို native run များအကြား byte-for-byte နှိုင်းယှဉ်ပြီး annotated/conditional expression များ၏ type-check acceptance နှင့် incompatible annotation၊ function-call၊ collection-element များ၏ rejection ကို စစ်ဆေးသည်။ Typed-IR artifact သည် `reference_only` အဖြစ် ဆက်ရှိနေပြီး ဤအရာသည် conformance foundation သာဖြစ်ကာ Zap-owned type checker မဖြစ်သေးပါ။

ပထမဆုံး provisional Zap-owned type-checker candidate ကို [`bootstrap/b2/typecheck.zp`](../bootstrap/b2/typecheck.zp) တွင် ထည့်ထားပြီး [`scripts/bootstrap/verify_b2_typecheck_candidate.sh`](../scripts/bootstrap/verify_b2_typecheck_candidate.sh) သည် annotated နှင့် conditional fixture၊ return ပါသော annotated function တစ်ခု၊ incompatible-number၊ incompatible-call၊ negative collection-element diagnostic နှင့် bounded nested-list index inference slice များ၏ deterministic behavior ကို enforce လုပ်သည်။ Nested slice သည် `list<list<number>>` indexing ကို လက်ခံပြီး numeric result ကို `text` သို့ assign လုပ်ပါက paired fixture များဖြင့် reject လုပ်သည်။ ၎င်းနှင့် ကိုက်ညီသော candidate-only typed-IR producer ကို [`bootstrap/b2/typed_ir.zp`](../bootstrap/b2/typed_ir.zp) တွင် ထည့်ထားပြီး [`scripts/bootstrap/verify_b2_typed_ir_candidate.sh`](../scripts/bootstrap/verify_b2_typed_ir_candidate.sh) သည် owned node field များကို native reference artifact နှင့် နှိုင်းယှဉ်သည်။ နှစ်ခုလုံးသည် corpus-limited ဖြစ်ပြီး general expression inference၊ generic/variant narrowing၊ function checking အပြည့်အစုံနှင့် diagnostic parity အပြည့်အစုံ မလုပ်သေးပါ။ Native Rust သည် reference owner အဖြစ် ဆက်ရှိပြီး bootstrap stage သည် B0 အဖြစ်သာ ဆက်ရှိသည်။

## B3 foundation status

Repository တွင် reference-only B3 foundation gate ကို [`scripts/bootstrap/verify_b3_foundations.sh`](../scripts/bootstrap/verify_b3_foundations.sh) အဖြစ် ထည့်ထားသည်။ ၎င်းသည် catalog determinism taxonomy၊ dependency-free manifest ၏ canonical lockfile generation၊ lockfile reproducibility၊ offline locked build နှင့် Zap test fixture execution များကို စစ်ဆေးသည်။ ဤ check များသည် ရှိပြီးသား package/build/test-runner behavior ကို ပြသခြင်းသာဖြစ်ပြီး compiler pipeline တစ်ခုလုံး Zap-owned ဖြစ်နေပြီဟု မဆိုလိုပါ။

## Reference VM နှင့် platform-seed အခြေအနေ

ပထမဆုံး သီးခြား bytecode VM foundation ကို [`native/src/bytecode.rs`](../native/src/bytecode.rs) တွင် တည်ဆောက်ထားသည်။ `zap bootstrap vm-demo` သည် bounded arithmetic program ကို execute လုပ်ပြီး canonical [`bootstrap/fixtures/bytecode/vm_demo.json`](../bootstrap/fixtures/bytecode/vm_demo.json) artifact ကို ထုတ်ပေးသည်။ VM သည် unsupported schema version၊ မမှန်ကန်သော stack shape၊ Halt မရှိခြင်း၊ arithmetic failure နှင့် step-budget ကျော်လွန်ခြင်းတို့ကို panic မဖြစ်စေဘဲ reject လုပ်သည်။

Platform boundary သည် self-hosted မဟုတ်သေးဘဲ documented boundary အဖြစ်သာ ရှိသည်။ Compiler core တွင် network သို့မဟုတ် process capability မရှိပါ။ Console၊ bounded file access၊ memory နှင့် optional clock behavior များသည် explicit seed responsibility များ ဖြစ်သည်။ [`scripts/bootstrap/verify_vm_platform.sh`](../scripts/bootstrap/verify_vm_platform.sh) သည် ဤ boundary နှင့် deterministic VM smoke artifact ကို စစ်ဆေးသည်။

## Canonical inspection commands

Native CLI တွင် read-only B0 inspection command များ ထည့်ပြီးဖြစ်သည်။

```text
zap bootstrap status
zap bootstrap tokens <file.zp>
zap bootstrap ast <file.zp>
zap bootstrap typed-ir <file.zp>
zap bootstrap diagnostics <file.zp>
```

ပထမ batch တွင် token၊ AST၊ reference-only typed-IR၊ diagnostic၊ metadata၊ platform-boundary နှင့် standard-library fixture များကို [`bootstrap/fixtures`](../bootstrap/fixtures) အောက်တွင် freeze လုပ်ထားသည်။ [`scripts/bootstrap/verify_b0_artifacts.sh`](../scripts/bootstrap/verify_b0_artifacts.sh) ကို run လုပ်ပါက artifact များကို ပြန်တည်ဆောက်ပြီး committed corpus နှင့် byte-for-byte နှိုင်းယှဉ်ပေးမည်။

## အဆင့်ဆိုင်ရာ policy

| အဆင့် | အဓိပ္ပါယ် | ခွင့်ပြုသော release claim |
|---|---|---|
| B0 | Rust က reference behavior နှင့် fixture များကို ပိုင်ဆိုင် | Rust reference/native implementation |
| B1 | Zap lexer/parser က B0 artifact များကို ပြန်ထုတ် | Zap bootstrap compiler foundation |
| B2 | Zap diagnostics/type checker က B0 acceptance/rejection ကို ပြန်ထုတ် | Zap bootstrap compiler foundation |
| B3 | Zap stdlib၊ typed IR၊ package resolver နှင့် test runner က offline/deterministic အလုပ်လုပ် | Zap-owned compiler pipeline in transition |
| B4 | Zap compiler က documented platform seed ဖြင့် မိမိကိုယ်ကို ပြန် build | Fully Zap-only self-hosted compiler |

B4 bootstrap check မအောင်မြင်သေးသရွေ့ release တစ်ခုတွင် B4 wording မသုံးရ။ နောင် semantic သို့မဟုတ် artifact change များအတွက် bilingual contract update၊ fixture change၊ ownership record၊ လိုအပ်ပါက compatibility decision နှင့် regression evidence လိုအပ်သည်။

## နောက် gate

လက်ရှိ implementation gate သည် staged parity expansion ဖြစ်သည်။ Zap-owned lexer နှင့် parser candidate များသည် owned corpus ကို ဆက်လက်ချဲ့ထွင်ပြီး valid၊ Unicode၊ malformed၊ overflow နှင့် determinism case များတွင် Rust reference နှင့် output နှိုင်းယှဉ်ရမည်။ B2 typed-IR/type-check conformance foundation၊ ပထမ provisional Zap-owned checker candidate နှင့် candidate-only typed-IR producer တို့ကို enforce လုပ်ပြီးဖြစ်သော်လည်း type checking နှင့် typed IR အပြည့်အစုံသည် native-owned အဖြစ်သာ ရှိသေးသည်။ v2.11.7 increment တွင် provisional list-element inference path တစ်ခုနှင့် negative collection-element diagnostic fixture တစ်ခု ထည့်ထားပြီး နောက်ထပ် nested-list slice တွင် paired valid/incompatible fixture များကို ownership မပြောင်းဘဲ ထည့်ထားသည်။ Broader collection inference၊ arbitrary nested expression နှင့် user-defined generic declarations များသည် သီးခြား evidence/design gate များနောက်တွင် ဆက်လက်လုပ်ဆောင်ရန် ကျန်ရှိသည်။ VM နှင့် native backend အလုပ်များကို ထို gate များမကျော်မီ ပြီးစီးသည်ဟု မဆိုရ။
