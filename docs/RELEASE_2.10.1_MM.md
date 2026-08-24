# Zap v2.10.1 Release Notes

**Release line:** v2.10.1
**စစ်ဆေးထားသော baseline:** v2.10.1 tag မတိုင်မီ latest master ရှိ Zap v2.10.0
**အခြေအနေ:** Published bootstrap parser နှင့် diagnostics foundation increment

## အနှစ်ချုပ်

Zap v2.10.1 တွင် Zap-only bootstrap roadmap ၏ နောက်ထပ် စစ်ဆေးနိုင်သော increment ကို ထည့်သွင်းထားပါသည်။ Repository တွင် arithmetic နှင့် compound corpus အတွက် provisional Zap-written parser candidate၊ Zap lexer candidate ကို အသုံးပြုထားသော token-driven delimiter diagnostics နှင့် deterministic B2 typed-IR/type-check conformance foundation တို့ ပါဝင်လာပါသည်။

ဤ release သည် **B0** အဖြစ်သာ ဆက်ရှိပါသည်။ Compiler pipeline၊ diagnostics၊ type checking၊ typed IR၊ package/build behavior၊ VM နှင့် platform boundary များအတွက် Rust သည် reference owner ဖြစ်နေဆဲပါသည်။ Zap artifact အသစ်များသည် differential evidence နှင့် provisional ownership candidate များသာဖြစ်ပြီး fully Zap-only သို့မဟုတ် self-hosted compiler ဖြစ်ပြီဟု မဆိုလိုပါ။

## အကောင်အထည်ဖော်ပြီးသော ပြောင်းလဲမှုများ

| နယ်ပယ် | ပြောင်းလဲမှု | အထောက်အထား |
|---|---|---|
| Parser corpus | Map၊ list၊ postfix indexing၊ binary operator၊ conditional၊ return နှင့် call များပါသော canonical compound AST နှင့် malformed-input fixture များ ထည့်ထားသည်။ | Reference parser differential gate |
| Zap parser candidate | `bootstrap/b1/parser.zp` ကို arithmetic-only မှ owned compound corpus slice အထိ ချဲ့ထားသည်။ | Byte-for-byte parser candidate gate |
| Diagnostics | Source-substring bracket check အစား Zap lexer token stream ကို scan လုပ်၍ missing/unexpected closing delimiter များကို စစ်ဆေးသည်။ | Canonical syntax-diagnostic fixture များ |
| Typed IR | Schema နှင့် `reference_only` marker ပါသော deterministic annotated typed-IR artifact evidence ထည့်ထားသည်။ | B2 typed-IR reproducibility gate |
| Type checking | Valid annotation/conditional နှင့် incompatible annotation acceptance/rejection fixture များကို native checker အပေါ် စစ်ဆေးသည်။ | B2 type-check conformance gate |
| Contracts | Bilingual bootstrap contract၊ ownership record၊ current v2.10.1 identity နှင့် Unreleased/release documentation များ update လုပ်ထားသည်။ | Documentation၊ ownership နှင့် version gate များ |

## Bootstrap boundary

Parser candidate သည် ရည်ရွယ်ချက်ရှိရှိ corpus-limited ဖြစ်ပြီး fixture-scoped assumption များ ကျန်ရှိနေသေးသည်။ Rust lexer သို့မဟုတ် parser ကို မအစားထိုးသေးပါ၊ Zap grammar အပြည့်အစုံကိုလည်း မ cover သေးပါ။ Typed-IR artifact နှင့် type-check behavior သည် native-owned reference contract အဖြစ် ဆက်ရှိပါသည်။ B3 package/build/test-runner နှင့် VM/platform check များသည် foundation evidence သာဖြစ်ပြီး self-hosting claim မဟုတ်ပါ။

ထို့ကြောင့် ဤ release ကို fully Zap-only၊ fully self-hosted သို့မဟုတ် B4 ဟု မဖော်ပြရ။ နောက် stage advancement အတွက် owned corpus ပိုမိုကျယ်ပြန့်ခြင်း၊ independent Zap implementation များ၊ byte-for-byte သို့မဟုတ် semantic differential evidence၊ mismatch အတွက် compatibility decision နှင့် documented platform-seed boundary လိုအပ်ပါသည်။

## Verification

Clean release preflight တွင် `passed=202`၊ `warnings=1`၊ `failures=0` ရရှိပါသည်။ Native formatting၊ clippy၊ cargo check၊ RustSec audit၊ native test၊ host test၊ framework check၊ bilingual documentation check၊ Markdown link၊ parser/lexer/type-check bootstrap gate၊ package/build foundation နှင့် VM/platform foundation များ အားလုံး pass ဖြစ်ပါသည်။ Warning တစ်ခုမှာ development preflight တွင် `RELEASE_TAG` မသတ်မှတ်ထားခြင်းကြောင့် ဖြစ်ပြီး tagged CI တွင် tag/version match နှင့် supported archive များကို ထပ်မံ verify လုပ်ပါသည်။

## References

[1]: ../docs/BOOTSTRAP_CONTRACT_MM.md
[2]: ../bootstrap/contracts/BOOTSTRAP_CONTRACT_MM.md
[3]: ../bootstrap/b1/parser.zp
[4]: ../scripts/bootstrap/verify_b1_parser_candidate.sh
[5]: ../scripts/bootstrap/verify_b2_typecheck.sh
[6]: ../docs/TYPECHECK_CONFORMANCE_MATRIX_MM.md
