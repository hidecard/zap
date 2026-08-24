# Zap v2.11.2 Release Notes

**Release line:** v2.11.2
**စစ်ဆေးထားသော baseline:** Published Zap v2.11.1
**အခြေအနေ:** Bootstrap function type-checking corpus increment

## အနှစ်ချုပ်

Zap v2.11.2 တွင် provisional Zap-owned type-checker candidate ကို ရိုးရှင်းသော declaration နှင့် conditional များထက် ပိုမိုကျယ်ပြန့်စွာ ချဲ့ထားပါသည်။ Candidate သည် annotated function တစ်ခု၊ ထို corpus slice အတွက် parameter/return propagation၊ compatible numeric call နှင့် deterministic incompatible function-call diagnostic ကို cover လုပ်ပါသည်။ Native reference checker gate သည်လည်း ထို acceptance/rejection case များကို စစ်ဆေးပါသည်။

ဤ release သည် **B0** အဖြစ်သာ ဆက်ရှိပါသည်။ Complete type checking၊ typed IR၊ parser semantics၊ diagnostics၊ package/build behavior၊ VM execution နှင့် platform boundary များအတွက် Rust သည် reference owner ဖြစ်နေဆဲပါသည်။ Zap code အသစ်များသည် corpus-limited transition evidence သာဖြစ်ပြီး fully Zap-only သို့မဟုတ် self-hosted compiler ဖြစ်ပြီဟု မဆိုလိုပါ။

## အကောင်အထည်ဖော်ပြီးသော ပြောင်းလဲမှုများ

| နယ်ပယ် | ပြောင်းလဲမှု | အထောက်အထား |
|---|---|---|
| Function corpus | Annotated `number` parameter၊ return propagation နှင့် numeric call အတွက် valid/invalid fixture များ ထည့်ထားပါသည်။ | Native B2 conformance gate |
| Call diagnostics | Call site တွင် text argument မကိုက်ညီမှုအတွက် stable `ZAP-TYPE-001`၊ line၊ column၊ notes၊ help နှင့် message field များကို candidate က handle လုပ်ပါသည်။ | Zap-owned B2 candidate gate |
| Candidate determinism | B2 case ငါးခုလုံးကို နှစ်ကြိမ် run လုပ်ပြီး JSON output byte-identical ဖြစ်ရမည်ဟု gate ချဲ့ထားပါသည်။ | Deterministic candidate gate |
| Documentation | Bilingual bootstrap contract၊ ownership record၊ Unreleased/release note နှင့် current v2.11.2 version surface များ update လုပ်ထားပါသည်။ | Documentation၊ version နှင့် ownership gate များ |

## Bootstrap boundary

Type-checker candidate သည် general expression inference၊ parameter အများအပြား၊ default argument၊ function return annotation၊ generic/variant narrowing၊ control-flow facts၊ diagnostic parity အပြည့်အစုံ သို့မဟုတ် arbitrary source program များကို မလုပ်သေးပါ။ Typed-IR producer သည် candidate-only ဖြစ်ပြီး annotated declaration fixture တစ်ခုတည်းကိုသာ ကန့်သတ်ထားသည်။ Function corpus အသစ်အတွက် typed IR ကို မထုတ်ပေးသေးပါ။

ထို့ကြောင့် ဤ release ကို fully Zap-only၊ fully self-hosted သို့မဟုတ် B4 ဟု မဖော်ပြရ။ နောက် stage advancement အတွက် owned corpus ပိုမိုကျယ်ပြန့်ခြင်း၊ independent analysis၊ differential evidence၊ compatibility decision နှင့် documented platform-seed boundary လိုအပ်ပါသည်။

## Verification

v2.11.2 version bump မတိုင်မီ clean release preflight တွင် `passed=204`၊ `warnings=1`၊ `failures=0` ရရှိပါသည်။ Version bump နောက်ပိုင်း expanded native/Zap-owned B2 function corpus၊ parser/lexer gate၊ B0 artifact၊ B3 package/build foundation၊ VM/platform foundation၊ specification ownership၊ documentation consistency နှင့် formatting gate များ pass ဖြစ်ပါသည်။ Warning တစ်ခုမှာ development preflight တွင် `RELEASE_TAG` မသတ်မှတ်ထားခြင်းကြောင့် ဖြစ်ပြီး tagged CI တွင် tag/version identity နှင့် platform archive များကို စစ်ဆေးပါသည်။

## References

[1]: ../docs/BOOTSTRAP_CONTRACT_MM.md
[2]: ../bootstrap/b2/typecheck.zp
[3]: ../scripts/bootstrap/verify_b2_typecheck.sh
[4]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[5]: ../bootstrap/fixtures/typecheck/function.zp
[6]: ../bootstrap/fixtures/typecheck/function_incompatible.zp
