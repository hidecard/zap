# Zap v2.11.15

**Release အခြေအနေ:** Candidate preparation ဖြစ်ပြီး complete validation အောင်မြင်မှသာ publish လုပ်မည်။ Zap သည် B0 အဖြစ်သာ ဆက်ရှိသည်။

## အကျဉ်းချုပ်

Zap v2.11.15 တွင် provisional၊ corpus-limited B2 type-checker increment အဖြစ် exact direct map-literal annotation shape တစ်ခုကို ပြင်ဆင်ထားသည်။ Candidate သည် exact literal `{"score": 7}` ကို `map<text,number>` အဖြစ် သတ်မှတ်ပြီး `let scores: map<text,number> = {"score": 7}` ကို လက်ခံသည်။ Paired negative fixture သည် ထို direct map literal ကို `text` သို့ assign လုပ်ပါက line 1၊ column 1 တွင် `variable 'wrong' expects text, got map<text,number>` diagnostic ဖြင့် reject လုပ်သည်။

ဤအရာသည် deterministic fixture pair တစ်ခုအတွက် evidence သာဖြစ်သည်။ General map-literal inference၊ arbitrary map key/value၊ nested map၊ alias၊ collection expression inference သို့မဟုတ် static type checking အပြည့်အစုံကို မဆိုလိုပါ။

## ပြောင်းလဲမှုများ

| အပိုင်း | ပြောင်းလဲမှု | ကန့်သတ်ချက် |
|---|---|---|
| B2 candidate | Exact direct `{"score": 7}` ကို `map<text,number>` အဖြစ် infer လုပ်ခြင်း | Literal spelling တစ်မျိုး၊ key တစ်ခုနှင့် value type တစ်မျိုးသာ |
| Diagnostics | Map literal ကို `text` သို့ assign လုပ်ပါက paired rejection ထည့်ခြင်း | Stable line 1၊ column 1 diagnostic |
| Native reference | Rust checker သည် positive fixture ကို လက်ခံပြီး negative fixture တွင် `variable 'wrong' expects text, got map<text,number>` ကို ထုတ်ကြောင်း အတည်ပြုခြင်း | Rust သည် authoritative ဖြစ်နေဆဲ |
| Evidence gates | Native နှင့် candidate B2 verifier များတွင် map-literal pair ထည့်ခြင်း | Provisional corpus evidence သာ |
| Ownership | Bootstrap ledger တွင် `BOOT-031` ထည့်ခြင်း | Candidate evidence ဖြစ်ပြီး compiler ownership မဟုတ် |
| Test reliability | Test HTTP response ကို socket EOF စောင့်မည့်အစား declared `Content-Length` အတိုင်း ဖတ်ခြင်း | Test harness သာ၊ production networking behavior မပြောင်း |
| Documentation | English/Burmese contract၊ matrix၊ current status၊ roadmap နှင့် release notes update လုပ်ခြင်း | Broader inference နှင့် self-hosting ဆက်လက် deferred |

## Verification contract

Candidate သည် native နှင့် Zap candidate B2 verifier၊ malformed-source safety၊ native tests၊ typecheck matrix parity၊ specification ownership၊ Markdown links၊ VS Code packaging၊ formatting၊ release-version validation၊ documentation consistency နှင့် exact committed release preflight အားလုံးကို အောင်မြင်ရမည်။ Public workflow တွင် source validation၊ Linux x86_64၊ macOS ARM64၊ Windows x86_64 နှင့် Publish jobs အားလုံး အောင်မြင်ရမည်။ Publish ပြီးသော artifact များသည် checksum၊ manifest၊ provenance နှင့် detached-signature verification များကို အောင်မြင်ရမည်။

## Deferred scope

General map-literal inference၊ arbitrary map key/value၊ nested map၊ ရှိပြီးသား bounded corpus ပြင်ပ collection/map inference၊ compound guard၊ loop mutation၊ reassignment invalidation၊ alias၊ arbitrary control flow၊ generic declaration၊ complete typed-IR ownership၊ package/build ownership၊ VM ownership နှင့် B4 self-rebuild acceptance များသည် သီးခြား design နှင့် evidence gate များနောက်တွင် ဆက်လက် deferred ဖြစ်သည်။

## Bootstrap boundary

Zap သည် **B0** အဖြစ်သာ ဆက်ရှိသည်။ Rust သည် complete/reference compiler နှင့် runtime owner ဖြစ်နေဆဲဖြစ်သည်။ `bootstrap/` အောက်ရှိ Zap lexer၊ parser၊ type-checker နှင့် typed-IR အလုပ်များသည် provisional၊ corpus-limited ဖြစ်သည်။ ဤ candidate သည် fully Zap-only၊ self-hosted၊ B1၊ B2၊ B3 သို့မဟုတ် B4 compiler ဖြစ်ကြောင်း မဆိုလိုပါ။

## References

[1]: RELEASE_ROLLBACK_RUNBOOK_MM.md
[2]: ../bootstrap/contracts/OWNERS.tsv
[3]: ../bootstrap/fixtures/typecheck/map_annotation.zp
[4]: ../bootstrap/fixtures/typecheck/map_annotation_incompatible.zp
[5]: ../scripts/bootstrap/verify_b2_typecheck.sh
[6]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[7]: ../native/src/evaluator.rs
