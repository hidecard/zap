# Zap v2.11.12 Release Notes

**Release အခြေအနေ:** Incremental bootstrap-evidence release ဖြစ်ပြီး repository သည် B0 အဖြစ်သာ ဆက်ရှိသည်။

## အနှစ်ချုပ်

Zap v2.11.12 တွင် direct negative option predicate form `if is_option_none(name): ... else:` အတွက် အကျယ်အဝန်းကန့်သတ်ထားသော provisional B2 candidate slice ကို ထည့်သွင်းထားသည်။ Tracked `option<number>` variable တစ်ခုအတွက် true body သည် option wrapper ကို ဆက်ထိန်းထားပြီး indented `else` body တစ်ခုအတွင်း payload ကို `number` အဖြစ် အသုံးပြုနိုင်သည်။ Paired fixture များသည် accepted form နှင့် line 5, column 1 ရှိ exact incompatible diagnostic `variable 'payload' expects text, got number` ကို စစ်ဆေးသည်။

Native Rust checker သည် complete/reference owner အဖြစ် ဆက်ရှိသည်။ ဤ release သည် native reference တွင် ရှိပြီးသား behavior အတွက် candidate-side evidence နှင့် deterministic differential verification ကို ထည့်ခြင်းသာဖြစ်ပြီး compiler ownership သို့မဟုတ် bootstrap stage ကို မတိုးမြှင့်ပါ။

## ပြောင်းလဲမှုများ

| အပိုင်း | ပြောင်းလဲမှု | Boundary |
|---|---|---|
| B2 candidate | Tracked `option<number>` တစ်ခုအတွက် direct `is_option_none` else-body narrowing ထည့်ထားသည်။ | Variable တစ်ခု၊ direct conditional shape တစ်ခုသာ |
| True branch | မူလ `option<number>` wrapper ကို ဆက်ထိန်းထားသည်။ | Payload widening အလိုအလျောက် မလုပ် |
| Else branch | Indented body တစ်ခုအတွင်း tracked value ကို `number` အဖြစ် narrow လုပ်သည်။ | Compound သို့မဟုတ် arbitrary predicate မပါ |
| Conformance | Exact line/column diagnostic ပါသော positive/negative fixture များ ထည့်ထားသည်။ | Corpus-limited evidence |
| Native gate | Paired fixture များအတွက် Rust reference verifier ကို တိုးချဲ့ထားသည်။ | Rust သည် reference owner ဖြစ်နေဆဲ |
| Candidate gate | Deterministic candidate parity ကို JSON output ၁၄ ခုမှ ၁၆ ခုသို့ တိုးချဲ့ထားသည်။ | Unsupported syntax သည် fail-closed ဖြစ်နေဆဲ |
| Ownership | Provisional else-branch slice အတွက် `BOOT-027` ထည့်ထားသည်။ | Provisional သာဖြစ်သည် |
| Documentation | English/Burmese contract၊ matrix၊ narrowing guide၊ TODO roadmap နှင့် release notes များ synchronize လုပ်ထားသည်။ | B0 language မပြောင်းပါ |

## Verification contract

Candidate သည် native reference conformance၊ candidate differential determinism၊ malformed-source no-panic safety၊ matrix parity၊ specification ownership၊ Markdown link၊ formatting၊ version consistency၊ Cargo check နှင့် exact committed release preflight များကို အောင်မြင်ရမည်။ Public release workflow တွင် source validation၊ Linux x86_64၊ macOS ARM64၊ Windows x86_64 နှင့် Publish jobs အားလုံး သီးခြားစီ အောင်မြင်ရမည်။ Published artifact များသည် checksum၊ manifest၊ provenance နှင့် signature verification ကို ဖြတ်ရမည်။

## Deferred scope

Option variable အများအပြား၊ compound guard၊ nested/compound control flow၊ loop mutation၊ reassignment invalidation၊ alias propagation၊ arbitrary user-defined predicate၊ broader collection/map inference၊ nested map၊ generic declaration၊ complete typed-IR ownership၊ package/build ownership၊ VM ownership နှင့် B4 self-rebuild acceptance များသည် သီးခြား design/evidence gate များနောက်တွင် ဆက်လက် deferred ဖြစ်သည်။

## Bootstrap boundary

Zap သည် **B0** အဖြစ်သာ ဆက်ရှိသည်။ Rust သည် complete/reference compiler နှင့် runtime owner ဖြစ်နေဆဲဖြစ်သည်။ `bootstrap/` အောက်ရှိ Zap lexer၊ parser၊ type-checker နှင့် typed-IR implementation များသည် provisional၊ corpus-limited ဖြစ်ပြီး ဤ release သည် fully Zap-only၊ self-hosted၊ B1၊ B2၊ B3 သို့မဟုတ် B4 compiler ဖြစ်ကြောင်း မဆိုလိုပါ။

Publish လုပ်ထားသော tag များသည် immutable ဖြစ်သည်။ v2.11.11 နှင့် ယခင် release history များကို rewrite မလုပ်ရ။ v2.11.12 သည် annotated tag အသစ်ကို အသုံးပြုပြီး release gate များ pass ပြီးမှသာ publish လုပ်ရမည်။

## References

[1]: ../bootstrap/contracts/OWNERS.tsv
[2]: ../bootstrap/fixtures/typecheck/else_narrowing.zp
[3]: ../bootstrap/fixtures/typecheck/else_narrowing_incompatible.zp
[4]: ../scripts/bootstrap/verify_b2_typecheck.sh
[5]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[6]: ../docs/TYPE_NARROWING_MM.md
