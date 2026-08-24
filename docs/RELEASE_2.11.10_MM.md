# Zap v2.11.10 Release Notes

**Release အခြေအနေ:** Maintenance နှင့် release-governance documentation release ဖြစ်ပြီး bootstrap stage သည် B0 အဖြစ်သာ ဆက်ရှိသည်။

## အနှစ်ချုပ်

Zap v2.11.10 တွင် v2.11.9 release နောက်ပိုင်း အသုံးပြုခဲ့သော repository branch-hygiene နှင့် merge policy ကို မှတ်တမ်းတင်ထားသည်။ Audit အရ merge လုပ်ရန် သင့်လျော်သော open pull request သို့မဟုတ် clean branch မရှိပါ။ Remote `fix/json-cycle-guard` branch သည် closed pull request #1 နှင့် ဆက်နွယ်ပြီး လက်ရှိ master line နှင့် နှိုင်းယှဉ်ပါက unique commit ၆ ခုရှိကာ current master ထက် များစွာ နောက်ကျနေသည်။ ၎င်းသည် လက်ရှိအတွက် clean delta မဟုတ်ဘဲ superseded production-hardening history ဖြစ်သောကြောင့် continuity အတွက် ရည်ရွယ်ချက်ရှိရှိ ထိန်းသိမ်းထားပြီး blind merge သို့မဟုတ် delete မလုပ်ခဲ့ပါ။

ဤ release တွင် bilingual branch-hygiene record ထည့်သွင်းထားပြီး documentation navigation နှင့် top-level README နှစ်ခုမှ link ချိတ်ထားသည်။ ထို record တွင် အနာဂတ် branch merge သို့မဟုတ် deletion မပြုမီ ancestry၊ patch-equivalence၊ pull-request နှင့် release-history check များ လုပ်ဆောင်ရမည့် policy ကို သတ်မှတ်ထားသည်။ Local stale reference များကို prune လုပ်ပြီးဖြစ်သော်လည်း publish လုပ်ထားသော release tag မည်သည့်ခုကိုမျှ ရွှေ့ခြင်း၊ rewrite လုပ်ခြင်း သို့မဟုတ် ဖယ်ရှားခြင်း မရှိပါ။

ဤ release သည် compiler semantics အသစ် သို့မဟုတ် bootstrap ownership အသစ် ရှိပြီဟု မဆိုလိုပါ။ Zap သည် **B0** အဖြစ်သာ ရှိနေဆဲဖြစ်သည်။ Rust သည် complete reference compiler နှင့် runtime owner ဖြစ်နေဆဲဖြစ်ပြီး `bootstrap/` အောက်ရှိ Zap lexer၊ parser၊ type-checker နှင့် typed-IR အလုပ်များသည် provisional၊ corpus-limited evidence အဖြစ်သာ ရှိသည်။

## ပြောင်းလဲမှုများ

| အပိုင်း | ပြောင်းလဲမှု | Boundary |
|---|---|---|
| Branch audit | လက်ရှိ branch၊ PR၊ ancestry၊ divergence နှင့် retention finding များကို မှတ်တမ်းတင်ထားသည်။ | Blind merge သို့မဟုတ် ခန့်မှန်း၍ deletion မလုပ်ရ |
| Documentation | English/Burmese branch-hygiene နှင့် merge record များကို synchronized ထည့်ထားသည်။ | Policy documentation သာဖြစ်သည် |
| Navigation | Release-operator documentation hub နှစ်ခုမှ branch record ကို link ချိတ်ထားသည်။ | Relative link များ validate လုပ်ထားသည် |
| README guidance | English/Burmese contribution guidance မှ branch record ကို link ချိတ်ထားသည်။ | Branch permission မပြောင်းပါ |
| Cleanup | Local stale reference များကိုသာ prune လုပ်ထားသည်။ | `origin/fix/json-cycle-guard` ကို ရည်ရွယ်ချက်ရှိရှိ ထိန်းသိမ်းထားသည် |
| Release integrity | ရှိပြီးသား tag နှင့် release history အားလုံးကို ထိန်းသိမ်းထားသည်။ | v2.11.9 နှင့် ယခင် release များ immutable ဖြစ်သည် |

## Verification contract

Release candidate သည် version consistency၊ bilingual documentation consistency၊ Markdown link validation၊ type-check matrix consistency၊ specification ownership validation၊ formatting၊ native tests၊ malformed-source safety၊ bootstrap gate၊ package/build gate၊ security audit နှင့် exact committed release preflight များကို အောင်မြင်ရမည်။ Artifact verification မပြုမီ public workflow တွင် source validation၊ Linux x86_64၊ macOS ARM64၊ Windows x86_64 နှင့် Publish jobs အားလုံး အောင်မြင်ရမည်။

## Deferred scope

Explicit provenance check မရှိဘဲ နောက်ထပ် branch များကို merge သို့မဟုတ် delete လုပ်ခြင်း၊ broader branch/loop type inference၊ generic declaration၊ nested map၊ deeper nested expression၊ complete typed-IR ownership၊ package/build ownership၊ VM ownership နှင့် B4 self-rebuild acceptance များကို ဆက်လက် deferred ထားသည်။ Historical branch သို့မဟုတ် documentation record ထဲတွင် ပါဝင်နေခြင်းတစ်ခုတည်းဖြင့် feature တစ်ခုကို promote မလုပ်ရ။

## Historical နှင့် release policy

Publish လုပ်ထားသော v2.11.9 နှင့် ယခင် tag အားလုံးသည် immutable ဖြစ်သည်။ v2.11.10 သည် annotated tag အသစ်ကိုသာ အသုံးပြုရမည်ဖြစ်ပြီး ယခင် release history ကို rewrite မလုပ်ရ။ Public workflow နှင့် artifact verification အောင်မြင်ပြီးမှသာ current-status စာမျက်နှာများတွင် v2.11.10 ကို latest အဖြစ် ဖော်ပြရမည်။

## References

[1]: ../docs/BRANCH_HYGIENE_MM.md
[2]: https://github.com/hidecard/zap/pull/1
[3]: ../docs/DOCUMENTATION_NAVIGATION_MM.md
[4]: ../docs/CURRENT_STATUS_MM.md
