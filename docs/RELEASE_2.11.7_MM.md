# Zap v2.11.7 Release Notes

**Release line:** v2.11.7
**အကျယ်အဝန်း:** Malformed-source no-panic safety regression gate
**အခြေအနေ:** Incremental release-engineering နှင့် safety-evidence release

## အနှစ်ချုပ်

Zap v2.11.7 တွင် native CLI အတွက် deterministic malformed-source safety harness အသစ် ထည့်ထားပါသည်။ Harness သည် malformed generic annotation၊ unknown annotation နှင့် incompatible annotation ပါသော invalid-source corpus အသေးတစ်ခုကို စစ်ဆေးသည်။ Case တစ်ခုစီသည် nonzero status ဖြင့် ရပ်တန့်ရမည်ဖြစ်ပြီး panic၊ unchecked-`unwrap`၊ unchecked-`expect` သို့မဟုတ် stack-backtrace signature များ မထွက်ရပါ။

ဤ regression သည် CI နှင့် release preflight နှစ်ခုစလုံးတွင် required ဖြစ်ပါသည်။ ရှိပြီးသား parser၊ adversarial-input နှင့် malformed-program corpus checks များကို ဖြည့်စွက်ခြင်းသာဖြစ်ပြီး ၎င်းတို့ကို မအစားထိုးပါ။ ဖြစ်နိုင်သမျှ malformed program အားလုံးကို fuzz စစ်ဆေးပြီးပြီဟုလည်း မဆိုပါ။

Bootstrap stage သည် **B0** အဖြစ်သာ ဆက်ရှိပါသည်။ Complete compiler/runtime semantics အတွက် Rust သည် reference owner ဖြစ်နေဆဲပါသည်။ ဤ release သည် fully Zap-only compiler၊ self-hosting၊ B4၊ complete panic-freedom သို့မဟုတ် complete fuzz coverage claim မဟုတ်ပါ။

## ပြောင်းလဲမှုများ

| နယ်ပယ် | ပြောင်းလဲမှု | Boundary |
|---|---|---|
| Source safety | Timeout နှင့် panic-signature check ပါသော `scripts/test_malformed_source_safety.sh` ထည့်ထားပါသည်။ | Deterministic corpus အသေးတစ်ခုသာ |
| CI | Malformed-source regression ကို required quality job ထဲသို့ ထည့်ထားပါသည်။ | Failure သည် fail-closed ဖြစ်သည် |
| Release preflight | Script ကို required release files နှင့် preflight gates ထဲသို့ ထည့်ထားပါသည်။ | နောက် release candidate များအတွက် required |
| Documentation | Current-status page၊ TODO checkpoint နှင့် bilingual release metadata များ update လုပ်ထားပါသည်။ | B0 boundary မပြောင်းပါ |

## Verification contract

Safety harness သည် ရှိပါက release binary ကို အသုံးပြုပြီး မရှိပါက locked native binary ကို build လုပ်သည်။ Fixture တစ်ခုစီသည် nonzero exit status ဖြင့် fail ရမည်၊ timeout မဖြစ်ရမည်၊ panic သို့မဟုတ် unchecked-failure signature မပါရပါ။ ပိုမိုကျယ်ပြန့်သော parser နှင့် malformed-program suite များသည် သီးခြား required evidence အဖြစ် ဆက်ရှိပါသည်။

## Historical နှင့် release policy

Publish လုပ်ထားသော v2.11.6 release သည် immutable ဖြစ်ပါသည်။ v2.11.7 သည် annotated tag အသစ်ကို အသုံးပြုမည်ဖြစ်ပြီး v2.11.4၊ v2.11.5 သို့မဟုတ် v2.11.6 ကို rewrite/retag မလုပ်ပါ။ Historical changelog entry နှင့် ယခင် release note များကို မပြောင်းလဲပါ။

## References

[1]: ../scripts/test_malformed_source_safety.sh
[2]: ../scripts/release_preflight.sh
[3]: ../.github/workflows/ci.yml
[4]: ../docs/CURRENT_STATUS_MM.md
[5]: ../docs/BOOTSTRAP_CONTRACT_MM.md
