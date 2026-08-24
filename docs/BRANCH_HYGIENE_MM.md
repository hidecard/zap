# Zap Branch Hygiene နှင့် Merge မှတ်တမ်း

**Audit baseline:** v2.11.9
**Repository:** [github.com/hidecard/zap](https://github.com/hidecard/zap)
**ဆုံးဖြတ်ချက်:** ဤ maintenance cycle အတွင်း branch များကို merge သို့မဟုတ် delete မလုပ်ခဲ့ပါ။

## Audit ရလဒ်

Integrated `master` branch သည် လက်ရှိ release baseline ဖြစ်သည်။ Audit ပြုလုပ်ချိန်တွင် local `master` နှင့် `origin/master` တို့သည် တူညီပြီး merge လုပ်ရန် သင့်လျော်သော open pull request မရှိပါ။ Remote တွင် ထပ်မံရှိသော branch တစ်ခုမှာ `fix/json-cycle-guard` ဖြစ်သည်။

ထို branch သည် [closed pull request #1](https://github.com/hidecard/zap/pull/1) နှင့် ဆက်နွယ်ပြီး merge မလုပ်ထားပါ။ လက်ရှိ master line နှင့် နှိုင်းယှဉ်ပါက ထို branch တွင် unique commit ၆ ခုရှိပြီး current master ထက်လည်း များစွာ နောက်ကျနေသည်။ ၎င်း၏ပြောင်းလဲမှုများသည် လက်ရှိ release အတွက် clean၊ reviewable delta မဟုတ်ဘဲ superseded production-hardening work ဖြစ်သည်။ Blind merge လုပ်ပါက obsolete history ကို ပြန်လည်ထည့်သွင်းခြင်းနှင့် နောက်ပိုင်း integrated changes များနှင့် conflict ဖြစ်ခြင်းတို့ကို ဖြစ်စေနိုင်သည်။

ထို branch ကို continuity နှင့် auditability အတွက် ရည်ရွယ်ချက်ရှိရှိ ထိန်းသိမ်းထားသည်။ ၎င်း၏ historical reference မလိုအပ်တော့ကြောင်း သီးခြား review ဖြင့် သက်သေမပြနိုင်သရွေ့ explicit authorization မရှိဘဲ delete မလုပ်ရ။ Local stale reference များကို `git fetch --prune origin` ဖြင့် prune လုပ်ပြီးဖြစ်သည်။ Release tag မည်သည့်ခုကိုမျှ ဖယ်ရှားခြင်း၊ ရွှေ့ခြင်း သို့မဟုတ် rewrite လုပ်ခြင်း မရှိပါ။

## လုပ်ဆောင်ရမည့် policy

| အခြေအနေ | လုပ်ဆောင်ရမည့်အရာ |
|---|---|
| Review လုပ်နိုင်သောပြောင်းလဲမှုနှင့် passing merge path ရှိသည့် open branch | Review နှင့် validation ပြီးမှ ပုံမှန် pull-request လမ်းကြောင်းဖြင့် merge လုပ်ရန်။ |
| ပြောင်းလဲမှုများ superseded ဖြစ်ပြီး closed ဖြစ်သည့် branch | Blind merge မလုပ်ရန်၊ historical reference ရည်ရွယ်ချက်ရှိမှသာ ထိန်းသိမ်းရန်။ |
| Provenance မရှင်းလင်းခြင်း သို့မဟုတ် အလွန်ကွဲပြားသော delta ရှိသည့် branch | ထိန်းသိမ်းရသည့်အကြောင်းရင်းကို မှတ်တမ်းတင်ပြီး ခန့်မှန်း၍ delete သို့မဟုတ် merge မလုပ်ရန်။ |
| Publish လုပ်ပြီးသော release tag | Immutable အဖြစ် သတ်မှတ်ရန်၊ နောက် release တစ်ခုစီအတွက် tag အသစ်သုံးရန်။ |

ဤမှတ်တမ်းသည် maintenance evidence ဖြစ်ပြီး historical branch အားလုံးကို semantic re-audit ပြီးပြီဟု မဆိုလိုပါ။ နောက်ထပ် branch cleanup တစ်ခုစီတွင် ancestry၊ patch-equivalence၊ pull-request နှင့် release-history check များကို ထပ်မံပြုလုပ်ရမည်။

## Bootstrap boundary

Branch hygiene သည် Zap ၏ maturity claim ကို မပြောင်းလဲစေပါ။ Zap သည် **B0** အဖြစ်သာ ရှိနေဆဲဖြစ်သည်။ Rust သည် complete/reference compiler နှင့် runtime owner ဖြစ်နေဆဲဖြစ်ပြီး `bootstrap/` အောက်ရှိ Zap lexer၊ parser၊ type-checker နှင့် typed-IR အလုပ်များသည် provisional၊ corpus-limited evidence အဖြစ်သာ ရှိသည်။

## References

[1]: https://github.com/hidecard/zap/pull/1
[2]: https://github.com/hidecard/zap/tree/master
[3]: ../docs/CURRENT_STATUS_MM.md
