# Zap Bootstrap Contract

**အခြေအနေ:** Zap v2.9.0 အတွက် self-hosting foundation

## ရည်ရွယ်ချက်

ဤ contract သည် လက်ရှိ Rust implementation နှင့် အနာဂတ် Zap implementation တို့ self-hosting အဆင့်များအတွင်း ဘယ်လိုအတူလုပ်မည်ကို သတ်မှတ်သည်။ လက်ရှိ Rust implementation သည် **B0 reference compiler/runtime** ဖြစ်သည်။ Zap ဖြင့်ရေးထားသော implementation သည် owned fixture များအတွက် canonical token stream၊ AST၊ diagnostics နှင့် accept/reject ဆုံးဖြတ်ချက်များကို B0 နှင့် တူညီစွာ ထုတ်ပေးနိုင်မှသာ **B1** ဟု သတ်မှတ်မည်။

## Bootstrap အဆင့်များ

| အဆင့် | Implementation | လိုအပ်သောအထောက်အထား |
|---|---|---|
| B0 | လက်ရှိ native Rust parser/type checker/evaluator | ရှိပြီးသား conformance နှင့် corpus tests များ အောင်မြင်ရမည် |
| B1 | Zap lexer/parser နှင့် AST producer | Token/AST JSON သည် B0 fixture နှင့် တူရမည် |
| B2 | Zap type checker | Typecheck decision နှင့် diagnostic JSON သည် B0 နှင့် တူရမည် |
| B3 | Zap pure standard library နှင့် typed IR bridge | Network သို့မဟုတ် ambient environment မပါဘဲ compiler run နိုင်ရမည် |
| B4 | Self-rebuild | B3 သည် compiler source ကို build လုပ်ပြီး မိမိ artifact ကို ပြန်ထုတ်နိုင်ရမည် |

## Canonical artifacts

တူညီသော UTF-8 source အတွက် bootstrap compiler တိုင်းက deterministic artifact များ ထုတ်ပေးရမည်။

1. `tokens.json` — token kind၊ normalized value နှင့် source span။
2. `ast.json` — AST node kind၊ fields နှင့် source span။
3. `diagnostics.json` — severity၊ stable code၊ message၊ location၊ notes နှင့် help။
4. `typecheck.json` — accept/reject decision နှင့် inferred/declared type information။
5. `manifest.json` — compiler version၊ language-spec version၊ stdlib version နှင့် schema versions။

Artifact များထဲတွင် memory address၊ timestamp၊ random identifier၊ host path သို့မဟုတ် environment အပေါ်မူတည်သော ordering မပါရ။ Map/object serialization order ကို lexical သို့မဟုတ် specification သတ်မှတ်ချက်အတိုင်းထားရမည်။

## Ownership boundaries

Lexer သည် UTF-8 tokenization နှင့် source span ကို ပိုင်သည်။ Parser သည် AST construction နှင့် precedence ကို ပိုင်သည်။ Type checker သည် static type decision ကို ပိုင်သည်။ Evaluator သည် runtime behavior ကို ပိုင်သည်။ Standard library သည် pure text၊ collection၊ math၊ JSON၊ option နှင့် result helpers ကို ပိုင်သည်။ Filesystem၊ process၊ network နှင့် Web operation များသည် explicit host capability boundary ထဲတွင်သာ ရှိရမည်။ Pure bootstrap compiler ထဲတွင် ထို capability များကို တိုက်ရိုက်မထည့်ရ။

## Differential test စည်းမျဉ်း

Fixture တစ်ခုချင်းစီအတွက် B0 နှင့် candidate bootstrap compiler ကို source bytes တူတူ၊ schema version တူတူဖြင့် ခေါ်ရမည်။ ခွင့်ပြုထားသော normalization ပြီးနောက် canonical JSON artifact များ တူမှသာ pass ဟု သတ်မှတ်ရမည်။ Mismatch ဖြစ်လျှင် compiler bug သို့မဟုတ် explicit compatibility decision ဖြစ်ပြီး specification update မလုပ်ဘဲ fixture ပြောင်း၍ ဖုံးကွယ်ခြင်း မလုပ်ရ။

## Reproducibility စည်းမျဉ်း

Bootstrap command သည် clean checkout မှ network မရှိဘဲ၊ project-specific ambient environment မရှိဘဲ အလုပ်လုပ်ရမည်။ Input source hash၊ compiler hash၊ schema versions နှင့် output artifact hash များကို မှတ်တမ်းတင်ရမည်။ Clean run နှစ်ကြိမ်ဆက်တိုက်တွင် artifact hash တူညီမှသာ B4 ပြီးစီးသည်ဟု သတ်မှတ်ရမည်။

## ပထမအဆင့် fixture များ

ပထမဆုံး fixture set ကို `bootstrap/fixtures/` အောက်တွင် ထားသည်။

- `lexer/hello.zp` — name၊ text၊ number နှင့် `say`။
- `parser/precedence.zp` — grouping၊ arithmetic၊ comparison နှင့် boolean precedence။
- `typecheck/list_number.zp` — မှန်ကန်သော typed collection။
- `typecheck/type_error.zp` — deterministic reject ဖြစ်ရမည့် annotation။
- `stdlib/pure_values.zp` — host access မပါသော pure collection/text helper များ။

Syntax သို့မဟုတ် standard-library feature တစ်ခု migrate လုပ်တိုင်း fixture set ကို တိုးချဲ့ရမည်။ ရှိပြီးသား `conformance/` နှင့် `corpus/` suite များသည် ပိုကျယ်သော compatibility source အဖြစ် ဆက်ရှိမည်။

## Acceptance gate

B1 change ကို merge မလုပ်မီ bootstrap validator အောင်မြင်ရမည်၊ pinned toolchain ပေါ်ရှိ native suite အောင်မြင်ရမည်၊ English/Burmese contract pair တူညီရမည်၊ ထို့အပြင် bootstrap stage၊ schema၊ fixture owners နှင့် known limitations များကို change record တွင် ဖော်ပြရမည်။
