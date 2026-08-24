# Zap v2.11.11 Release Notes

**Release အခြေအနေ:** Incremental bootstrap-evidence release ဖြစ်ပြီး repository သည် B0 အဖြစ်သာ ဆက်ရှိသည်။

## အနှစ်ချုပ်

Zap v2.11.11 တွင် loop-local option narrowing အတွက် အကျယ်အဝန်းကန့်သတ်ထားသော provisional B2 candidate slice ကို ထည့်သွင်းထားသည်။ Zap-written candidate သည် tracked `option<number>` variable အတွက် direct `while is_some(value):` guard ကိုသာ သိရှိပြီး indented loop body တစ်ခုအတွင်း numeric payload ကို အသုံးပြုခွင့်ပေးသည်။ Loop boundary ရောက်သောအခါ မူလ option-wrapper type ကို ပြန်လည်ရရှိစေသည်။ Positive fixture သည် body အတွင်း payload use ကို စစ်ဆေးပြီး paired negative fixture သည် loop ပြီးနောက် `option<number>` mismatch ကို stable structured diagnostic ဖြင့် အတည်ပြုသည်။

Native Rust checker သည် သက်ဆိုင်ရာ TC-006 loop-boundary behavior ကို ရှိပြီးသား reference အဖြစ် ပိုင်ဆိုင်သည်။ ဤ release သည် candidate-side evidence နှင့် deterministic differential verification ကိုသာ ထည့်သွင်းခြင်းဖြစ်ပြီး compiler ownership ကို Zap သို့ မလွှဲပြောင်းပါ။ Candidate ကို general type checker ဖြစ်စေခြင်း သို့မဟုတ် bootstrap stage တိုးမြှင့်ခြင်း မရှိပါ။

## ပြောင်းလဲမှုများ

| အပိုင်း | ပြောင်းလဲမှု | Boundary |
|---|---|---|
| B2 candidate | tracked `option<number>` အတွက် direct `while is_some` narrowing ထည့်ထားသည်။ | Variable တစ်ခု၊ indented loop body တစ်ခုသာ |
| Loop boundary | Loop ပြီးနောက် မူလ option wrapper ကို ပြန်ထားသည်။ | General data-flow သို့မဟုတ် mutation analysis မပါ |
| Conformance | Exact rejection shape ပါသော positive/negative loop fixture များ ထည့်ထားသည်။ | Corpus-limited evidence |
| Native gate | ရှိပြီးသား TC-006 regression နှင့်အတူ Rust reference verifier ကို တိုးချဲ့ထားသည်။ | Rust သည် reference owner ဖြစ်နေဆဲ |
| Candidate gate | Deterministic two-run candidate parity ကို corpus output ၁၂ ခုမှ ၁၄ ခုသို့ တိုးချဲ့ထားသည်။ | Unsupported syntax သည် fail-closed ဖြစ်နေဆဲ |
| Ownership | Provisional loop-narrowing slice အတွက် `BOOT-026` ထည့်ထားသည်။ | Provisional သာဖြစ်သည် |
| Documentation | English/Burmese contract၊ matrix၊ current status၊ roadmap နှင့် release notes များ synchronize လုပ်ထားသည်။ | B0 language မပြောင်းပါ |

## Verification contract

Candidate သည် native reference conformance၊ candidate differential determinism၊ malformed-source no-panic safety၊ matrix parity၊ specification ownership၊ Markdown link၊ formatting၊ version consistency၊ Cargo check နှင့် exact committed release preflight များကို အောင်မြင်ရမည်။ Public release workflow တွင် source validation၊ Linux x86_64၊ macOS ARM64၊ Windows x86_64 နှင့် Publish jobs အားလုံး သီးခြားစီ အောင်မြင်ရမည်။ Published artifact များသည် checksum၊ manifest၊ provenance နှင့် signature verification ကို ဖြတ်ရမည်။

## Deferred scope

Compound guard၊ candidate `is_option_none` else-branch support၊ loop mutation၊ reassignment invalidation၊ nested loop၊ arbitrary control-flow expression၊ broader collection/map inference၊ nested map၊ generic declaration၊ complete typed-IR ownership၊ package/build ownership၊ VM ownership နှင့် B4 self-rebuild acceptance များသည် သီးခြား design/evidence gate များနောက်တွင် ဆက်လက် deferred ဖြစ်သည်။

## Bootstrap boundary

Zap သည် **B0** အဖြစ်သာ ဆက်ရှိသည်။ Rust သည် complete/reference compiler နှင့် runtime owner ဖြစ်နေဆဲဖြစ်သည်။ `bootstrap/` အောက်ရှိ Zap lexer၊ parser၊ type-checker နှင့် typed-IR implementation များသည် provisional၊ corpus-limited ဖြစ်ပြီး ဤ release သည် fully Zap-only၊ self-hosted၊ B1၊ B2၊ B3 သို့မဟုတ် B4 compiler ဖြစ်ကြောင်း မဆိုလိုပါ။

Publish လုပ်ထားသော tag များသည် immutable ဖြစ်သည်။ v2.11.10 နှင့် ယခင် release history များကို rewrite မလုပ်ရ။ v2.11.11 သည် annotated tag အသစ်ကို အသုံးပြုပြီး release gate များ pass ပြီးမှသာ publish လုပ်ရမည်။

## References

[1]: ../bootstrap/contracts/OWNERS.tsv
[2]: ../bootstrap/fixtures/typecheck/loop_narrowing.zp
[3]: ../bootstrap/fixtures/typecheck/loop_narrowing_incompatible.zp
[4]: ../scripts/bootstrap/verify_b2_typecheck.sh
[5]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[6]: ../docs/TYPECHECK_CONFORMANCE_MATRIX_MM.md
