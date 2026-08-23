# Zap Type-Checking နှင့် Conformance Acceptance Matrix

**အခြေအနေ:** PDF-driven follow-up roadmap အတွက် baseline  
**နောက်ဆုံးစစ်ဆေးထားသော version:** v2.2.7
**အကျယ်အဝန်း:** Static checking၊ control-flow narrowing၊ diagnostics နှင့် conformance fixtures

ဤစာတမ်းသည် နောက်ထပ် type-system workstream အတွက် acceptance boundary ကို သတ်မှတ်ထားသည်။ လက်ရှိအကောင်အထည်ဖော်ပြီးသားအရာများ၊ ဒီဇိုင်းဆုံးဖြတ်ချက်လိုအပ်သည့်အရာများနှင့် implementation ကျန်နေသည့်အရာများကို ခွဲခြားထားသည်။ နောက်ဆုံးစစ်ဆေးထားသော release metadata သည် v2.2.7 ဖြစ်ပြီး ဤ release သည် ပြီးစီးပြီးသား async runtime၊ registry နှင့် release-engineering အလုပ်များကို ပြန်လည်ဖွင့်မည်မဟုတ်ပါ။

## လက်ရှိ baseline

| အပိုင်း | လက်ရှိအပြုအမူ | အခြေအနေ | အထောက်အထား |
|---|---|---:|---|
| Primitive annotations | `text`၊ `number`၊ `bool`၊ `list`၊ `map`၊ `object` နှင့် `none` annotation များကို parse/check လုပ်နိုင်သည် | Baseline ပြီး | Parser/evaluator tests နှင့် `zap check` |
| Function argument count | ပျောက်နေသော သို့မဟုတ် ပိုနေသော argument များအတွက် static diagnostic ထုတ်သည် | Baseline ပြီး | Human နှင့် JSON diagnostics |
| Literal argument mismatch | သိရှိထားသော parameter annotation နှင့် literal argument type ကို စစ်သည် | Baseline ပြီး | `zap check --json` |
| Return annotation | Function return annotation နှင့် return expression type ကို စစ်သည် | Baseline ပြီး | Function annotation tests |
| Result/Option payloads | `result<T>` နှင့် `option<T>` payload annotation များသည် `ok`၊ `err` နှင့် `some` payload များကို စစ်သည် | Baseline ပြီး | Type annotation tests |
| JSON diagnostics | Diagnostic တွင် `kind`၊ `message`၊ `error`၊ `file`၊ `line` နှင့် `column` fields ပါသည် | Baseline ပြီး | CLI/LSP fixtures |
| Simple narrowing | Support လုပ်ထားသော `option<T>` နှင့် `result<T>` guard များအတွက် branch-local narrowing ရှိသည် | Baseline ပြီး | `TYPE_NARROWING_EN.md` |
| Complex narrowing | Nested boolean expression၊ loop၊ reassignment နှင့် incompatible alias များ | Implemented baseline | TC-001–TC-006 နှင့် TC-010 fixtures၊ advanced inference ကို deferred ထားသည် |
| Complex inference | Nested call၊ collection element နှင့် control-flow expression များ | Implemented baseline | TC-007–TC-009 fixtures၊ advanced generic inference ကို deferred ထားသည် |
| Generic design | Generic list/map/function syntax နှင့် inference contract | Implemented baseline | `TYPECHECK_GENERIC_DESIGN_MM.md`၊ generic declaration နှင့် advanced inference ကို deferred ထားသည် |

## လက်ရှိ conformance အထောက်အထား

TC-006 အတွက် guarded `while` body အတွင်း narrowed payload ကို အသုံးပြုနိုင်ပြီး loop ပြီးနောက် မူလ wrapper type ကို ပြန်လည်ရရှိကြောင်း အတည်ပြုသည့် permanent loop-boundary fixtures များ ရှိပါသည်။ Else-branch narrowing သည် explicit `is_option_none(value)` guard ကို ထောက်ပံ့ပြီး true branch တွင် option wrapper ကို ထိန်းသိမ်းကာ else branch တွင် payload type ကို အသုံးပြုစေပါသည်။ TC-009 အတွက် compatible branches၊ မကိုက်ညီသော branch result types နှင့် bool မဟုတ်သော condition များကို `zap check --json` ဖြင့် စစ်ဆေးသည့် permanent positive/negative fixtures များ ထည့်သွင်းပြီးဖြစ်သည်။ ထို့အပြင် L3 regression သည် incompatible conditional expression အတွက် JSON `ok`၊ `kind`၊ `file`၊ `line`၊ `column`၊ `message` နှင့် `error` fields များ မပြောင်းလဲဘဲ ထွက်ရှိကြောင်း အတည်ပြုပါသည်။ TC-010 အတွက် `option<T>` နှင့် `result<T>` wrapper identity သည် alias assignment ဖြတ်သန်းပြီးနောက် ထိန်းသိမ်းထားကြောင်းနှင့် reassignment ပြုလုပ်ပါက narrowed alias fact ကို invalidate လုပ်ကြောင်း အတည်ပြုသည့် permanent fixtures များ ထည့်သွင်းပြီးဖြစ်သည်။ TC-012 ကို `list<T>`၊ `map<K, V>`၊ `option<T>` နှင့် `result<T>` annotation များအတွက် implemented baseline အဖြစ် မှတ်တမ်းတင်ထားပြီး malformed generic form များကို reject လုပ်ကာ user-defined generic declaration နှင့် advanced inference များကို explicit deferred scope အဖြစ် ထားရှိထားပါသည်။ ဤ fixtures များသည် L2 behavior နှင့် TC-009 L3 diagnostic contract ကို တည်ဆောက်ပေးပါသည်။ `lsp_diagnostics_match_cli_type_error_contract` L4 regression သည် LSP diagnostics များက shared static checker ကို ပြန်လည်အသုံးပြုကြောင်း၊ `TypeError` code ကို ထိန်းသိမ်းကြောင်း၊ source location တူညီမှုကို LSP ၏ zero-based range သို့ မှန်ကန်စွာ ပြောင်းပေးကြောင်းနှင့် normalized message ကို ထုတ်ပေးကြောင်း အတည်ပြုပါသည်။ Lint diagnostics များကိုလည်း အဆိုပါ source-diagnostic bridge မှတစ်ဆင့် ဆက်လက်ရရှိနိုင်ပါသည်။

## Acceptance levels

Feature တစ်ခုကို **proposed** မှ **implemented** သို့ တိုးမြှင့်ရန် syntax၊ positive behavior၊ negative behavior၊ diagnostic shape နှင့် bilingual documentation အားလုံး ပါဝင်ရမည်။ Runtime behavior တစ်ခုတည်းဖြင့် static-checking release gate ကို မဖြတ်နိုင်ပါ။

| Level | အဓိပ္ပါယ် | လိုအပ်သောအထောက်အထား |
|---|---|---|
| L0 | မသတ်မှတ်ရသေး | Specification မပြီးမချင်း implementation မစရ |
| L1 | Syntax/design အတည်ပြု | Specification နှင့် parser accept/reject cases |
| L2 | Static behavior အကောင်အထည်ဖော်ပြီး | Positive/negative `zap check` fixtures |
| L3 | Diagnostic contract တည်ငြိမ် | JSON schema၊ location၊ error kind နှင့် message assertions |
| L4 | Conformance-ready | Runtime agreement၊ formatter/LSP agreement၊ bilingual docs နှင့် CI gate |

## Conformance scenario matrix

| ID | စစ်ဆေးရမည့်အခြေအနေ | မျှော်မှန်း static result | Diagnostic လိုအပ်ချက် | ဦးစားပေး |
|---|---|---|---|---:|
| TC-001 | `if is_some(value)` ဖြင့် `option<number>` ကို branch အတွင်း သုံးခြင်း | Branch အတွင်း numeric use ကို လက်ခံရန် | Valid use အတွက် diagnostic မရှိရ၊ branch ပြီးနောက် wrapper type ပြန်ဖြစ်ရမည် | P0 |
| TC-002 | `if is_some(value) and value > 0` ဖြင့် guard နှင့် numeric comparison ပေါင်းခြင်း | Facts နှစ်ခုလုံးမှန်မှသာ လက်ခံရန် | Unsafe comparison နေရာကို တိတိကျကျ ပြရမည် | P0 |
| TC-003 | `if is_some(a) or is_some(a)` ဖြင့် တူညီသော fact ထပ်သုံးခြင်း | တူညီသော variable narrowing ကို deterministic လက်ခံရန် | Duplicate/contradictory diagnostic မထုတ်ရ | P0 |
| TC-004 | `if is_some(a) or is_some(b)` ဖြင့် variable မတူသော guard သုံးခြင်း | Variable နှစ်ခုလုံးကို မသက်သေပြဘဲ narrowing မလုပ်ရ | သက်သေမလုံလောက်သည့် use တွင် TypeError ပြရမည် | P0 |
| TC-005 | Narrowed variable ကို branch အတွင်း ပြန် assign လုပ်ခြင်း | Assignment ပြီးနောက် narrowing fact ကို invalidate လုပ်ရန် | မလုံခြုံသည့် post-assignment use ကို ပြရမည် | P0 |
| TC-006 | Loop အတွင်း narrowed variable ကို ပြောင်းလဲခြင်း | Loop boundary တိုင်းတွင် facts ပြန်တွက်ရန် | Loop ပြင်ပသို့ stale narrowing မထွက်ရ | P1 |
| TC-007 | Annotated value ပြန်ပေးသော nested function call | Call မှတစ်ဆင့် return type ကို propagate လုပ်ရန် | Call သို့မဟုတ် assignment နေရာတွင် mismatch location တည်ငြိမ်ရမည် | P0 |
| TC-008 | Collection element ကို annotation နှင့် အသုံးပြုခြင်း | Statically သိနိုင်သော element type ကို infer/check လုပ်ရန် | JSON diagnostic တွင် element expression location ပါရမည် | P1 |
| TC-009 | မကိုက်ညီသော branch type များပြန်ပေးသည့် control-flow expression | မကိုက်ညီသော expression result ကို reject လုပ်ရန် | TypeError တွင် branch context နှင့် location ပါရမည် | P1 |
| TC-010 | Alias သည် branch များမှ `option<T>` သို့မဟုတ် `result<T>` ကို သယ်ဆောင်ခြင်း | Wrapper identity နှင့် narrowing facts ကို ထိန်းသိမ်းရန် | Unsound alias widening မဖြစ်ရ | P1 |
| TC-011 | Unknown annotation သို့မဟုတ် malformed generic annotation | Parse/check အဆင့်တွင် reject လုပ်ရန် | Exact span နှင့် `kind=TypeError` သို့မဟုတ် syntax diagnostic | Baseline ပြီး |
| TC-012 | `list<number>` ကဲ့သို့ generic syntax သုံးခြင်း | Support လုပ်ထားသော generic annotation form များကို လက်ခံပြီး malformed သို့မဟုတ် unsupported form များကို reject လုပ်ရန် | မသိသော generic form များကို type diagnostic ဖြင့် reject လုပ်ရမည် | Implemented baseline |

## Diagnostic contract

Static-checking failure အသစ်တိုင်းသည် structured diagnostic boundary ကို ထိန်းသိမ်းရမည်။

```json
{
  "kind": "TypeError",
  "message": "...",
  "error": "...",
  "file": "main.zp",
  "line": 1,
  "column": 1
}
```

Message wording သည် ပြောင်းလဲနိုင်သော်လည်း `kind`၊ source location နှင့် user-actionable message များ မဖြစ်မနေပါရမည်။ Internal Rust error များကို public JSON contract ထဲသို့ မပေါက်ကြားစေရ။ Human-readable diagnostic နှင့် JSON diagnostic သည် failure တစ်ခုတည်းကို ဖော်ပြရမည်။

## အကောင်အထည်ဖော်ရန် အစီအစဉ်

TC-001 မှ TC-012 အထိကို သတ်မှတ်ထားသော supported syntax boundary အတွင်း အကောင်အထည်ဖော်ပြီးဖြစ်သည်။ နောက်ထပ်လုပ်ငန်းများတွင် negative collection-element cases၊ ပိုမိုနက်ရှိုင်းသော nested inference နှင့် user-defined generic declarations များကို design record နှင့် release gate အသစ်အောက်တွင်သာ တိုးချဲ့သင့်သည်။ Fixture တစ်ခုအတွက်သာ generic syntax ကို အလျင်စလို မတိုးချဲ့သင့်ပါ။

## ထပ်မလုပ်သင့်သည့် အပိုင်း

ဤ matrix ၏ အကျယ်အဝန်းပြင်ပတွင် v2.1-D/E တွင် အကောင်အထည်ဖော်ပြီး validation ပြီးသား structured task API၊ cancellation/deadline၊ threaded file/TCP/process adapter၊ registry authentication/service deployment၊ release preflight၊ artifact manifest၊ signing၊ provenance နှင့် post-publish release verification များ ရှိသည်။

## ဤ workstream ပြီးစီးရန် သတ်မှတ်ချက်

v2.2.7 type-checking baseline သည် supported syntax boundary အတွင်း TC-001 မှ TC-012 အထိ ပြီးစီးသည်ဟု သတ်မှတ်နိုင်သည်။ P0 rows များတွင် L3 evidence ရှိပြီး TC-006၊ TC-009 နှင့် TC-010 တွင်လည်း L3 evidence ရှိသည်။ Diagnostic location များသည် `file`၊ `line` နှင့် `column` အနေဖြင့် တည်ငြိမ်သည်။ Accepted program များတွင် runtime နှင့် static behavior ကိုက်ညီပြီး negative fixtures များသည် သတ်မှတ်ထားသော failure reason ဖြင့် fail သည်။ LSP သည် shared diagnostic vocabulary ကို အသုံးပြုပြီး English/Burmese documentation pair သည် synchronized ဖြစ်သည်။ Advanced generic declarations နှင့် inference များသည် ဤ release boundary ပြင်ပတွင် ရှိသည်။
