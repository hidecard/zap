# Zap v2.11.3 Release Notes

**Release line:** v2.11.3
**စစ်ဆေးထားသော baseline:** Published Zap v2.11.1 နှင့် validated v2.11.2 roadmap checkpoint
**အခြေအနေ:** Bootstrap function type-checking corpus နှင့် release-pipeline resilience increment

## အနှစ်ချုပ်

Zap v2.11.3 တွင် provisional Zap-owned type-checker candidate ကို annotated function တစ်ခု၊ ထို corpus slice အတွက် parameter/return propagation၊ compatible numeric call နှင့် deterministic incompatible function-call diagnostic များအထိ ချဲ့ထားပါသည်။ ထို့အပြင် multi-platform release workflow တွင် target-native tests သည် ပထမအကြိမ် fail ဖြစ်ပါက တစ်ကြိမ် retry လုပ်မည်ဖြစ်ပြီး ဒုတိယအကြိမ်ပါ fail ဖြစ်လျှင် release ကို fail-closed အဖြစ် ရပ်တန့်မည်ဖြစ်ပါသည်။

ဤ release သည် **B0** အဖြစ်သာ ဆက်ရှိပါသည်။ Complete type checking၊ typed IR၊ parser semantics၊ diagnostics၊ package/build behavior၊ VM execution နှင့် platform boundary များအတွက် Rust သည် reference owner ဖြစ်နေဆဲပါသည်။ Zap code အသစ်များသည် corpus-limited transition evidence သာဖြစ်ပြီး fully Zap-only သို့မဟုတ် self-hosted compiler ဖြစ်ပြီဟု မဆိုလိုပါ။

## အကောင်အထည်ဖော်ပြီးသော ပြောင်းလဲမှုများ

| နယ်ပယ် | ပြောင်းလဲမှု | အထောက်အထား |
|---|---|---|
| Function corpus | Annotated `number` parameter၊ return propagation နှင့် numeric call အတွက် valid/invalid fixture များ ထည့်ထားပါသည်။ | Native B2 conformance gate |
| Call diagnostics | Call site တွင် text argument မကိုက်ညီမှုအတွက် stable `ZAP-TYPE-001`၊ location၊ notes၊ help နှင့် message field များကို candidate က handle လုပ်ပါသည်။ | Zap-owned B2 candidate gate |
| Candidate determinism | B2 case ငါးခုလုံးကို နှစ်ကြိမ် run လုပ်ပြီး JSON output byte-identical ဖြစ်ရမည်ဟု gate ချဲ့ထားပါသည်။ | Deterministic candidate gate |
| Release resilience | ပထမ target-native test failure နောက်တွင် တစ်ကြိမ် retry ထည့်ထားပြီး ဒုတိယ failure ဖြစ်ပါက release ရပ်တန့်မည်။ | Release workflow review နှင့် tagged CI |
| Documentation | Bilingual contract၊ ownership record၊ changelog၊ release note နှင့် current v2.11.3 version surface များ update လုပ်ထားပါသည်။ | Documentation၊ version နှင့် ownership gate များ |

## Bootstrap boundary

Type-checker candidate သည် general expression inference၊ parameter အများအပြား၊ default argument၊ function return annotation၊ generic/variant narrowing၊ control-flow facts၊ diagnostic parity အပြည့်အစုံ သို့မဟုတ် arbitrary source program များကို မလုပ်သေးပါ။ Typed-IR producer သည် candidate-only ဖြစ်ပြီး annotated declaration fixture တစ်ခုတည်းကိုသာ ကန့်သတ်ထားသည်။ Function corpus အသစ်အတွက် typed IR ကို မထုတ်ပေးသေးပါ။

ထို့ကြောင့် ဤ release ကို fully Zap-only၊ fully self-hosted သို့မဟုတ် B4 ဟု မဖော်ပြရ။ နောက် stage advancement အတွက် owned corpus ပိုမိုကျယ်ပြန့်ခြင်း၊ independent analysis၊ differential evidence၊ compatibility decision နှင့် documented platform-seed boundary လိုအပ်ပါသည်။

## Verification

v2.11.2 အပေါ်အခြေခံသော clean preflight တွင် `passed=204`၊ `warnings=1`၊ `failures=0` ရရှိပါသည်။ Expanded native/Zap-owned B2 function gate၊ parser/lexer gate၊ B0 artifact၊ B3 package/build foundation၊ VM/platform foundation၊ documentation consistency၊ ownership နှင့် formatting check များ pass ဖြစ်ပါသည်။ ယခင် v2.11.2 tagged run တွင် macOS ARM64 target-native test တစ်ခု သီးခြား fail ဖြစ်သောကြောင့် publish မလုပ်နိုင်ခဲ့ပါ။ ထို tag ကို rewrite မလုပ်ဘဲ v2.11.3 တွင် retry hardening ကို ထည့်ထားပါသည်။

## References

[1]: ../docs/BOOTSTRAP_CONTRACT_MM.md
[2]: ../bootstrap/b2/typecheck.zp
[3]: ../scripts/bootstrap/verify_b2_typecheck.sh
[4]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[5]: ../.github/workflows/release.yml
[6]: ../bootstrap/fixtures/typecheck/function.zp
[7]: ../bootstrap/fixtures/typecheck/function_incompatible.zp
