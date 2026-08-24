# Zap v2.11.6 Release Notes

**Release line:** v2.11.6
**အကျယ်အဝန်း:** B2 nested-list inference conformance slice
**အခြေအနေ:** Incremental bootstrap-evidence release

## အနှစ်ချုပ်

Zap v2.11.6 တွင် provisional Zap B2 type-checker candidate အတွက် ကန့်သတ်ထားသော nested-list inference slice ကို ထည့်ထားပါသည်။ Candidate သည် `list<list<number>>` အတွက် `rows[0][1]` expression ၏ numeric element type ကို ယခုသိရှိနိုင်ပါသည်။ ထို numeric result ကို `text` annotation သို့ assign လုပ်ပါက stable structured diagnostic ဖြင့် reject လုပ်သည့် negative fixture ကိုလည်း တွဲဖက်ထည့်ထားပါသည်။

Native Rust checker နှင့် provisional Zap candidate နှစ်ခုစလုံးကို deterministic release-gated check များဖြင့် စစ်ဆေးထားပါသည်။ Native conformance test တွင် positive/negative case နှစ်ခုလုံးကို မှတ်တမ်းတင်ထားပြီး ownership ledger တွင် candidate fixture အသစ်ကို provisional အဖြစ် မှတ်တမ်းတင်ထားပါသည်။

Bootstrap stage သည် **B0** အဖြစ်သာ ဆက်ရှိပါသည်။ Complete compiler/runtime semantics အတွက် Rust သည် reference owner ဖြစ်နေဆဲပါသည်။ ဤ release သည် general nested expression inference၊ broad collection inference၊ fully Zap-only compiler၊ self-hosting သို့မဟုတ် B4 ဖြစ်ပြီဟု မဆိုပါ။

## ပြောင်းလဲမှုများ

| နယ်ပယ် | ပြောင်းလဲမှု | Boundary |
|---|---|---|
| B2 candidate | `list<list<number>>` အတွက် bounded nested-list index inference ထည့်ထားပါသည်။ | Candidate-only နှင့် corpus-limited |
| Native conformance | Positive/negative `TC-008` nested collection case များ ထည့်ထားပါသည်။ | Rust သည် reference owner ဖြစ်နေဆဲ |
| Differential evidence | Native နှင့် Zap candidate gate များကို deterministic JSON diagnostic ပါသော paired nested fixture များအထိ ချဲ့ထားပါသည်။ | Stage မမြှင့်ပါ |
| Ownership | Nested collection candidate fixture အတွက် `BOOT-023` ထည့်ထားပါသည်။ | Provisional ownership record |
| Documentation | English/Burmese bootstrap contract၊ type-check matrix၊ TODO checkpoint နှင့် current-status page များ update လုပ်ထားပါသည်။ | B0 boundary မပြောင်းပါ |

## Verification

Exact committed v2.11.6 candidate သည် version consistency၊ bilingual documentation parity၊ Markdown links၊ ownership၊ formatting၊ native/host tests၊ bootstrap gates၊ dependency audit နှင့် release preflight အားလုံး pass ပြီးမှသာ publish လုပ်ရမည်။ Nested candidate scope သည် explicit paired fixture များအတွင်းသာ ကန့်သတ်ထားပြီး unsupported သို့မဟုတ် unknown expression များသည် candidate contract ပြင်ပတွင် ရှိပါသည်။

## Historical နှင့် release policy

Publish လုပ်ထားသော v2.11.5 release သည် immutable ဖြစ်ပါသည်။ v2.11.6 သည် annotated tag အသစ်ကိုသာ အသုံးပြုမည်ဖြစ်ပြီး v2.11.4 သို့မဟုတ် v2.11.5 ကို rewrite/retag မလုပ်ပါ။ Historical changelog entry နှင့် ယခင် release note များကို မပြောင်းလဲပါ။

## References

[1]: ../bootstrap/b2/typecheck.zp
[2]: ../bootstrap/fixtures/typecheck/nested_collection.zp
[3]: ../bootstrap/fixtures/typecheck/nested_collection_incompatible.zp
[4]: ../scripts/bootstrap/verify_b2_typecheck.sh
[5]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[6]: ../docs/BOOTSTRAP_CONTRACT_MM.md
