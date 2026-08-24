# Zap v2.11.8 Release Notes

**Release line:** v2.11.8  
**အကျယ်အဝန်း:** Bounded provisional map-element inference evidence  
**အခြေအနေ:** Incremental B2 evidence release; bootstrap stage သည် B0 အဖြစ်သာ ဆက်ရှိသည်

## အနှစ်ချုပ်

Zap v2.11.8 တွင် Zap-written B2 type-checker candidate အတွက် ကျဉ်းမြောင်းသော provisional map-element inference slice တစ်ခု ထည့်သွင်းထားသည်။ Owned corpus သည် text literal ဖြင့် index လုပ်သော tracked `map<text,number>` variable တစ်ခုကို cover လုပ်ပြီး inferred numeric element ကို `text` သို့ assign လုပ်ပါက reject လုပ်သည့် paired negative fixture တစ်ခုလည်း ပါဝင်သည်။

Native Rust reference checker တွင် အဆိုပါ positive/negative behavior အတွက် permanent TC-008 regression coverage ထည့်ထားသည်။ B2 reference နှင့် candidate verifier များသည် deterministic output နှင့် mismatch အတွက် stable `ZAP-TYPE-001` diagnostic fields များကို မဖြစ်မနေ စစ်ဆေးသည်။ ဤ evidence သည် ရည်ရွယ်ချက်ရှိရှိ ကန့်သတ်ထားခြင်းဖြစ်ပြီး arbitrary key-expression inference၊ nested-map inference၊ general collection inference သို့မဟုတ် type-checker parity အပြည့်အစုံ ရှိပြီဟု မဆိုလိုပါ။

Bootstrap stage သည် **B0** အဖြစ်သာ ဆက်ရှိသည်။ Rust သည် complete reference compiler နှင့် runtime owner ဖြစ်နေဆဲဖြစ်သည်။ ဤ release သည် fully Zap-only compiler၊ self-hosting၊ B4 သို့မဟုတ် general Zap-owned type-checking semantics ရှိပြီဟု မဆိုလိုပါ။

## ပြောင်းလဲမှုများ

| အပိုင်း | ပြောင်းလဲမှု | Boundary |
|---|---|---|
| B2 candidate | corpus-limited `map<text,number>` text-key element inference path ထည့်ထားသည်။ | Tracked map variable နှင့် text literal key သာ |
| Native conformance | Positive/negative TC-008 map-element regression case များ ထည့်ထားသည်။ | Rust သည် reference owner အဖြစ် ဆက်ရှိသည် |
| Candidate gate | Deterministic candidate verification ကို JSON case ၈ ခုမှ ၁၀ ခုသို့ တိုးချဲ့ထားသည်။ | Candidate evidence သည် general compiler correctness မဟုတ်ပါ |
| Ownership | Provisional `BOOT-024` metadata နှင့် paired map fixture များ ထည့်ထားသည်။ | Ownership သည် provisional အဖြစ်သာ ရှိသည် |
| Documentation | English/Burmese contract၊ matrix၊ current-status scope၊ roadmap checkpoint နှင့် release metadata များ update လုပ်ထားသည်။ | B0 boundary မပြောင်းပါ |

## Verification contract

Release candidate သည် native B2 verifier၊ byte-identical JSON output ရရှိကြောင်း နှစ်ကြိမ် run လုပ်သည့် Zap-written candidate verifier၊ TC-008 native regression၊ documentation နှင့် ownership consistency check၊ malformed-source safety နှင့် exact committed release preflight များကို အောင်မြင်ရမည်။ Map mismatch တွင် `kind=TypeError`၊ `code=ZAP-TYPE-001`၊ line 2၊ column 1 နှင့် `variable 'result' expects text, got number` message များကို ထိန်းသိမ်းရမည်။

## Historical နှင့် release policy

Publish လုပ်ထားသော v2.11.7 release နှင့် ယခင် tag အားလုံးသည် immutable ဖြစ်သည်။ v2.11.8 သည် annotated tag အသစ်ကိုသာ အသုံးပြုရမည်ဖြစ်ပြီး ယခင် release history ကို rewrite သို့မဟုတ် retag မလုပ်ရ။ Publication workflow နှင့် public artifact verification အောင်မြင်ပြီးမှသာ current-status စာမျက်နှာများတွင် v2.11.8 ကို latest release အဖြစ် ပြောင်းလဲဖော်ပြရမည်။

## References

[1]: ../bootstrap/b2/typecheck.zp
[2]: ../bootstrap/fixtures/typecheck/map_collection.zp
[3]: ../bootstrap/fixtures/typecheck/map_collection_incompatible.zp
[4]: ../scripts/bootstrap/verify_b2_typecheck.sh
[5]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[6]: ../native/tests/core.rs
[7]: ../bootstrap/contracts/OWNERS.tsv
[8]: ../docs/BOOTSTRAP_CONTRACT_MM.md
