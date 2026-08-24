# Zap v2.11.9 Release Notes

**Release အခြေအနေ:** Incremental B2 evidence release ဖြစ်ပြီး bootstrap stage သည် B0 အဖြစ်သာ ဆက်ရှိသည်။

## အနှစ်ချုပ်

Zap v2.11.9 တွင် Zap-written B2 type-checker candidate အတွက် ကျဉ်းမြောင်းသော provisional branch-local option-narrowing slice တစ်ခု ထည့်သွင်းထားသည်။ Owned corpus သည် `option<number>` variable တစ်ခုကို track လုပ်ပြီး indented `if` body တစ်ခုအတွင်း direct `is_some` guard ဖြင့်သာ narrow လုပ်ကာ ရရှိသော numeric payload ကို annotated function call မှတစ်ဆင့် အသုံးပြုသည်။ Numeric payload ကို `text` သို့ assign လုပ်သည့် paired negative fixture ကို reject လုပ်သည်။

Rust reference checker တွင် positive နှင့် negative case များအတွက် permanent native regression coverage ထည့်ထားသည်။ Native B2 verifier နှင့် Zap candidate verifier တို့သည် deterministic acceptance/rejection behavior နှင့် stable `ZAP-TYPE-001` mismatch shape ကို မဖြစ်မနေ စစ်ဆေးသည်။ ဤ evidence သည် ရည်ရွယ်ချက်ရှိရှိ ကန့်သတ်ထားခြင်းဖြစ်ပြီး general branch analysis၊ compound-guard reasoning၊ loop narrowing၊ reassignment invalidation၊ generic inference သို့မဟုတ် type-checker parity အပြည့်အစုံ ရှိပြီဟု မဆိုလိုပါ။

Bootstrap stage သည် **B0** အဖြစ်သာ ဆက်ရှိသည်။ Rust သည် complete reference compiler နှင့် runtime owner ဖြစ်နေဆဲဖြစ်သည်။ ဤ release သည် fully Zap-only compiler၊ self-hosting၊ B4 သို့မဟုတ် general Zap-owned type-checking semantics ရှိပြီဟု မဆိုလိုပါ။

## ပြောင်းလဲမှုများ

| အပိုင်း | ပြောင်းလဲမှု | Boundary |
|---|---|---|
| B2 candidate | Tracked `option<number>` variable အတွက် direct `is_some` branch-local narrowing path ထည့်ထားသည်။ | Direct guard တစ်ခုနှင့် indented `if` body တစ်ခုတည်း |
| Native conformance | Branch-local payload use အတွက် paired native acceptance/rejection coverage ထည့်ထားသည်။ | Rust သည် reference owner အဖြစ် ဆက်ရှိသည် |
| Candidate gate | Deterministic candidate verification ကို JSON case ၁၀ ခုမှ ၁၂ ခုသို့ တိုးချဲ့ထားသည်။ | Corpus evidence သည် general compiler correctness မဟုတ်ပါ |
| Diagnostics | Narrowed numeric payload ကို `text` သို့ assign လုပ်ရာတွင် `ZAP-TYPE-001` ကို ထိန်းသိမ်းထားသည်။ | Assert လုပ်ထားသော mismatch သည် line 5၊ column 1 ဖြစ်သည် |
| Ownership | Provisional `BOOT-025` metadata နှင့် paired branch fixture များ ထည့်ထားသည်။ | Ownership သည် provisional အဖြစ်သာ ရှိသည် |
| Documentation | Bilingual bootstrap contract၊ conformance matrix၊ current-status scope၊ roadmap checkpoint နှင့် release metadata များ update လုပ်ထားသည်။ | B0 boundary မပြောင်းပါ |

## Verification contract

Release candidate သည် native B2 verifier၊ twice-run Zap-written candidate verifier မှ byte-identical JSON output၊ permanent native TC-001 regression၊ documentation နှင့် ownership consistency check၊ malformed-source safety နှင့် exact committed release preflight များကို အောင်မြင်ရမည်။ Incompatible branch fixture တွင် `kind=TypeError`၊ `code=ZAP-TYPE-001`၊ line 5၊ column 1 နှင့် `variable 'inside' expects text, got number` message များကို ထိန်းသိမ်းရမည်။

## Deferred scope

Compound boolean guard၊ လက်ရှိ Rust baseline ထက်ကျော်လွန်သော loop-boundary narrowing၊ candidate အတွင်း reassignment invalidation၊ ရှိပြီးသား native evidence ထက်ကျော်လွန်သော alias propagation၊ nested map၊ arbitrary nested expression၊ user-defined generic declaration၊ complete typed-IR ownership၊ package/build ownership၊ VM ownership နှင့် B4 self-rebuild acceptance များကို သီးခြား design/evidence gate များနောက်တွင် ဆက်လက် deferred ထားသည်။

## Historical နှင့် release policy

Publish လုပ်ထားသော v2.11.8 နှင့် ယခင် tag အားလုံးသည် immutable ဖြစ်သည်။ v2.11.9 သည် annotated tag အသစ်ကိုသာ အသုံးပြုရမည်ဖြစ်ပြီး ယခင် release history ကို rewrite သို့မဟုတ် retag မလုပ်ရ။ Public workflow နှင့် artifact verification အောင်မြင်ပြီးမှသာ current-status စာမျက်နှာများတွင် v2.11.9 ကို latest အဖြစ် ဖော်ပြရမည်။

## References

[1]: ../bootstrap/b2/typecheck.zp
[2]: ../bootstrap/fixtures/typecheck/branch_narrowing.zp
[3]: ../bootstrap/fixtures/typecheck/branch_narrowing_incompatible.zp
[4]: ../scripts/bootstrap/verify_b2_typecheck.sh
[5]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[6]: ../native/tests/core.rs
[7]: ../bootstrap/contracts/OWNERS.tsv
[8]: ../docs/BOOTSTRAP_CONTRACT_MM.md
