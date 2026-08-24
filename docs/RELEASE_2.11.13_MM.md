# Zap v2.11.13 Release Notes

**Release အခြေအနေ:** v2.11.12 failed-tag incident အတွက် publish လုပ်ပြီးသော corrective release ဖြစ်ပြီး Zap သည် B0 အဖြစ်သာ ဆက်ရှိသည်။

## အနှစ်ချုပ်

Zap v2.11.13 သည် tracked `option<number>` variable တစ်ခုအတွက် ထည့်သွင်းထားသော provisional၊ corpus-limited direct `is_option_none` else-body narrowing evidence ကို ဆက်လက်သယ်ဆောင်ထားပြီး macOS ARM64 CI အတွက် အကျယ်အဝန်းအနည်းဆုံး cross-platform correction ကို ထည့်ထားသည်။ Complete CRLF-terminated request header များပို့ပြီးနောက် native web-server regression test ၏ client write side ကို မလိုအပ်ဘဲ half-close မလုပ်တော့ပါ။ Request parser သည် EOF မလိုအပ်သောကြောင့် ထိုမလိုအပ်သော half-close ကို ဖယ်ရှားခြင်းက CI တွင် တွေ့ရှိခဲ့သော macOS ARM64 local-socket reset behavior ကို ရှောင်ရှားစေသည်။

v2.11.12 annotated tag ကို macOS ARM64 target-native test တွင် release workflow မအောင်မြင်ခဲ့သည့် immutable evidence အဖြစ် ထိန်းသိမ်းထားပြီး v2.11.12 GitHub Release ကို publish မလုပ်ခဲ့ပါ။ v2.11.13 သည် tag အသစ်ဖြစ်ပြီး ကိုယ်ပိုင် complete release workflow နှင့် public artifact verification များ pass ပြီးမှသာ publish လုပ်ခဲ့သည်။

## ပြောင်းလဲမှုများ

| အပိုင်း | ပြောင်းလဲမှု | Boundary |
|---|---|---|
| B2 candidate | Tracked `option<number>` တစ်ခုအတွက် direct `is_option_none` else-body narrowing ကို ဆက်လက်ထားသည်။ | Direct conditional shape တစ်ခုနှင့် indented else body တစ်ခုသာ |
| CI reliability | Complete request header ပြီးနောက် test client ၏ မလိုအပ်သော half-close ကို ဖယ်ရှားထားသည်။ | Test-harness portability fix သာဖြစ်ပြီး runtime ownership claim မဟုတ် |
| macOS ARM64 | Post-fix CI matrix တွင် native target tests အားလုံး အောင်မြင်သည်။ | Verified commit/workflow နှင့်သာ သက်ဆိုင်သော evidence |
| Release incident handling | Immutable v2.11.12 tag/history ကို ထိန်းသိမ်းပြီး corrective version အသစ် အသုံးပြုသည်။ | Tag move၊ deletion သို့မဟုတ် fabricated release မလုပ် |
| Documentation | Active bilingual version metadata နှင့် failed-tag boundary ကို update လုပ်ထားသည်။ | Historical record များ မပြောင်း |

## Verification contract

Candidate သည် native/candidate B2 verification၊ malformed-source no-panic safety၊ matrix parity၊ specification ownership၊ Markdown link၊ VS Code packaging၊ Cargo formatting/check၊ RustSec audit နှင့် exact committed release preflight များကို အောင်မြင်ရမည်။ Public workflow တွင် source validation၊ Linux x86_64၊ macOS ARM64၊ Windows x86_64 နှင့် Publish job များအားလုံး သီးခြားစီ အောင်မြင်ရမည်။ Published artifact များသည် checksum၊ manifest၊ provenance နှင့် detached signature verification ကို ဖြတ်ရမည်။

## Incident record

v2.11.12 tag ကို ၎င်း၏ မူလ release-preparation commit တွင် immutable အဖြစ် ဆက်လက်ထားသည်။ ထို release workflow သည် source validation နှင့် Linux/Windows build job များကို အောင်မြင်ခဲ့သော်လည်း macOS ARM64 target-native test `evaluator::tests::native_web_server_handles_requests_and_isolates_handler_errors` တွင် 265 passed နှင့် 1 failed ဖြစ်ခဲ့သည်။ ထို့ကြောင့် Publish job ကို skip လုပ်ခဲ့ပြီး v2.11.12 သည် public release မဖြစ်ခဲ့ပါ။ ထို့နောက် ပြင်ဆင်ထားသော master commit သည် macOS ARM64 အပါအဝင် Zap CI matrix အပြည့်အစုံကို အောင်မြင်ခဲ့သည်။

## Deferred scope

Compound guard၊ loop mutation၊ reassignment invalidation၊ alias၊ nested/arbitrary control flow၊ broader collection/map inference၊ nested map၊ generic declaration၊ complete typed-IR ownership၊ package/build ownership၊ VM ownership၊ production async reactor ownership နှင့် B4 self-rebuild acceptance များသည် သီးခြား design/evidence gate များနောက်တွင် ဆက်လက် deferred ဖြစ်သည်။

## Bootstrap boundary

Zap သည် **B0** အဖြစ်သာ ဆက်ရှိသည်။ Rust သည် complete/reference compiler နှင့် runtime owner ဖြစ်နေဆဲဖြစ်သည်။ `bootstrap/` အောက်ရှိ Zap lexer၊ parser၊ type-checker နှင့် typed-IR အလုပ်များသည် provisional၊ corpus-limited ဖြစ်ပြီး ဤ release သည် fully Zap-only၊ self-hosted၊ B1၊ B2၊ B3 သို့မဟုတ် B4 compiler ဖြစ်ကြောင်း မဆိုလိုပါ။

## References

[1]: RELEASE_ROLLBACK_RUNBOOK_MM.md
[2]: ../bootstrap/contracts/OWNERS.tsv
[3]: ../bootstrap/fixtures/typecheck/else_narrowing.zp
[4]: ../bootstrap/fixtures/typecheck/else_narrowing_incompatible.zp
[5]: ../scripts/bootstrap/verify_b2_typecheck.sh
[6]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[7]: ../native/src/evaluator.rs
