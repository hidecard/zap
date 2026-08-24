# Zap v2.11.4 Release Notes

**Release line:** v2.11.4
**အကျယ်အဝန်း:** B2 collection-element conformance increment
**အခြေအနေ:** Incremental bootstrap နှင့် type-checking release

## အနှစ်ချုပ်

Zap v2.11.4 တွင် provisional Zap-owned B2 type-checker candidate အတွက် collection-element inference increment တစ်ခု ထည့်ထားပါသည်။ လက်ရှိ support လုပ်ထားသော corpus အတွင်း numeric literal ဖြင့် index လုပ်ထားသော `list<number>` variable သည် element type ကို ပြန်ပေးနိုင်ပြီး၊ မကိုက်ညီသော annotated assignment အတွက် deterministic structured `TypeError` diagnostic ထုတ်ပေးပါသည်။ Native Rust checker နှင့် Zap candidate နှစ်ခုစလုံးကို positive/negative evidence များဖြင့် အမြဲတမ်း စစ်ဆေးထားပါသည်။

ဤ release သည် **B0** အဖြစ်သာ ဆက်ရှိပါသည်။ Complete parsing၊ type checking၊ typed IR၊ package/build behavior၊ VM execution၊ diagnostics နှင့် platform boundary များအတွက် Rust သည် reference owner ဖြစ်နေဆဲပါသည်။ Zap implementation အသစ်သည် corpus-limited evidence သာဖြစ်ပြီး fully Zap-only compiler၊ self-hosting သို့မဟုတ် B4 ဖြစ်ပြီဟု မဆိုလိုပါ။

## ပြောင်းလဲမှုများ

| နယ်ပယ် | ပြောင်းလဲမှု | Boundary |
|---|---|---|
| B2 candidate | Tracked `list<T>` value ကို numeric indexing လုပ်ပြီး support ပြုထားသော fixture path အတွက် `T` ကို propagate လုပ်ပါသည်။ | Provisional နှင့် corpus-limited |
| Negative conformance | `collection_incompatible.zp` ထည့်ထားပါသည်။ `list<number>` မှ `values[0]` ကို `text` သို့ assign လုပ်ပါက line 2/column 1 တွင် stable diagnostic ဖြင့် reject လုပ်ပါသည်။ | Rust နှင့် Zap candidate gates |
| Native regression evidence | Existing positive TC-008 collection-element case အနားတွင် သက်ဆိုင်ရာ native negative case ထည့်ထားပါသည်။ | Native Rust reference က အတည်ပြုချက်ပေးနေဆဲ |
| Ownership | Candidate fixture အသစ်အတွက် provisional `BOOT-022` ownership metadata ထည့်ထားပါသည်။ | Stage advancement မရှိ |
| Documentation | Bilingual bootstrap contract၊ type-checking baseline၊ README/version surfaces၊ changelog နှင့် release notes များ synchronize လုပ်ထားပါသည်။ | B0 wording ဆက်လက်ထိန်းသိမ်းထား |

## တိကျသောကန့်သတ်ချက်များ

ဤ increment တွင် general collection inference၊ Zap candidate အတွင်း map-element inference၊ arbitrary index expression၊ nested collection propagation၊ control-flow-sensitive element facts သို့မဟုတ် user-defined generic declarations မပါဝင်ပါ။ Candidate typed-IR producer သည် ယခင် annotated declaration fixture တစ်ခုတည်းကိုသာ ဆက်လက် cover လုပ်ပါသည်။ Broader collection inference၊ deeper nested inference နှင့် generic declaration များအတွက် သီးခြား design နှင့် evidence gate များ လိုအပ်ပါသည်။

ရှိပြီးသား v2.11.3 tag နှင့် release history များကို မပြောင်းလဲထားပါ။ v2.11.4 သည် incremental release အသစ်ဖြစ်ပြီး ယခင် release များကို rewrite သို့မဟုတ် retag မလုပ်ပါ။

## Verification

Publish မလုပ်မီ release candidate သည် version၊ documentation၊ link၊ ownership၊ formatting၊ bootstrap၊ native-test၊ dependency နှင့် security gates အားလုံးကို pass ရမည်။ Final preflight total များကို အစောပိုင်း release မှ မကူးယူဘဲ exact committed v2.11.4 candidate မှသာ မှတ်တမ်းတင်ပါမည်။

## References

[1]: ../docs/BOOTSTRAP_CONTRACT_MM.md
[2]: ../bootstrap/b2/typecheck.zp
[3]: ../bootstrap/fixtures/typecheck/collection_incompatible.zp
[4]: ../scripts/bootstrap/verify_b2_typecheck.sh
[5]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[6]: ../native/tests/core.rs
