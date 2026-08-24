# Zap v2.11.1 Release Notes

**Release line:** v2.11.1
**စစ်ဆေးထားသော baseline:** Published Zap v2.11.0
**အခြေအနေ:** Bootstrap type-checker နှင့် typed-IR candidate increment

## အနှစ်ချုပ်

Zap v2.11.1 တွင် bootstrap roadmap အတွက် ပထမဆုံး provisional Zap-owned type-checker candidate နှင့် ကိုက်ညီသော candidate-only typed-IR producer ကို ထည့်သွင်းထားပါသည်။ Candidate သည် annotated number declaration၊ compatible conditional expression နှင့် incompatible number annotation များကို cover လုပ်ပြီး typed-IR producer သည် annotated declaration node ကို ထုတ်ပေးကာ native reference artifact နှင့် owned field များကို နှိုင်းယှဉ်ပါသည်။

ဤ release သည် **B0** အဖြစ်သာ ဆက်ရှိပါသည်။ Complete type checking၊ typed IR၊ parser semantics၊ diagnostics၊ package/build behavior၊ VM execution နှင့် platform boundary များအတွက် Rust သည် reference owner ဖြစ်နေဆဲပါသည်။ Zap code အသစ်များသည် corpus-limited transition evidence သာဖြစ်ပြီး fully Zap-only သို့မဟုတ် self-hosted compiler ဖြစ်ပြီဟု မဆိုလိုပါ။

## အကောင်အထည်ဖော်ပြီးသော ပြောင်းလဲမှုများ

| နယ်ပယ် | ပြောင်းလဲမှု | အထောက်အထား |
|---|---|---|
| Zap type-checker candidate | B2 fixture သုံးခုဖြစ်သော annotated number၊ compatible conditional နှင့် incompatible number annotation အတွက် `bootstrap/b2/typecheck.zp` ကို ထည့်ထားသည်။ | Deterministic candidate gate |
| Type diagnostics | Source location၊ notes၊ help နှင့် mismatch message ပါသော stable `ZAP-TYPE-001` candidate diagnostic ကို ထုတ်ပေးသည်။ | Candidate acceptance/rejection gate |
| Zap typed-IR candidate | Annotated declaration fixture အတွက် `candidate_only` metadata ပါသော `bootstrap/b2/typed_ir.zp` ကို ထည့်ထားသည်။ | Candidate typed-IR differential gate |
| Reference parity | Native reference artifact နှင့် owned typed-IR node field များကို နှိုင်းယှဉ်ပြီး native schema ownership ကို တိကျစွာ ထိန်းသိမ်းသည်။ | B2 reference နှင့် candidate gate များ |
| Release contracts | CI နှင့် release-preflight enforcement၊ ownership row များ၊ bilingual bootstrap documentation၊ changelog နှင့် v2.11.1 version surface များကို ထည့်ထားသည်။ | Repository validation suite |

## Bootstrap boundary

Type-checker candidate သည် general expression inference၊ function parameter/return checking၊ generic/variant narrowing၊ diagnostic parity အပြည့်အစုံ သို့မဟုတ် arbitrary source program များကို မလုပ်သေးပါ။ Typed-IR producer သည် annotated declaration fixture တစ်ခုတည်းအတွက်သာ ရည်ရွယ်ထားပြီး native typed-IR emitter ကို မအစားထိုးပါ။ Artifact နှစ်ခုလုံးသည် provisional နှင့် candidate-only ဖြစ်ပါသည်။

ထို့ကြောင့် ဤ release ကို fully Zap-only၊ fully self-hosted သို့မဟုတ် B4 ဟု မဖော်ပြရ။ နောက် stage advancement အတွက် owned corpus ပိုမိုကျယ်ပြန့်ခြင်း၊ independent analysis၊ differential evidence၊ compatibility decision နှင့် documented platform-seed boundary လိုအပ်ပါသည်။

## Verification

v2.11.0 အပေါ်အခြေခံသော clean release preflight တွင် `passed=204`၊ `warnings=1`၊ `failures=0` ရရှိပါသည်။ Native formatting၊ clippy၊ cargo check၊ RustSec audit၊ native/host test၊ documentation/link check၊ parser/lexer/bootstrap gate၊ B2 type-checker နှင့် typed-IR candidate gate၊ package/build foundation နှင့် VM/platform foundation များ အားလုံး pass ဖြစ်ပါသည်။ Warning တစ်ခုမှာ development preflight တွင် `RELEASE_TAG` မသတ်မှတ်ထားခြင်းကြောင့် ဖြစ်ပြီး tagged CI တွင် tag/version identity နှင့် platform archive များကို ထပ်မံစစ်ဆေးပါသည်။

## References

[1]: ../docs/BOOTSTRAP_CONTRACT_MM.md
[2]: ../bootstrap/b2/typecheck.zp
[3]: ../bootstrap/b2/typed_ir.zp
[4]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[5]: ../scripts/bootstrap/verify_b2_typed_ir_candidate.sh
