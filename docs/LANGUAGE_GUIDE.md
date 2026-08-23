# Zap Language Guide

ဤဖိုင်သည် compatibility link အဖြစ်သာ ထားရှိထားသော legacy entry point ဖြစ်ပါသည်။ လက်ရှိ maintained beginner-to-advanced guide ကို အောက်ပါ bilingual documents တွင် ဖတ်ရှုပါ။

- [English Language Guide](LEARN_ZAP_EN.md)
- [မြန်မာ Language Guide](LEARN_ZAP_MM.md)
- [Documentation Navigation](DOCUMENTATION_NAVIGATION_MM.md)

လက်ရှိ project workflow သည် Django-style `startapp` သို့မဟုတ် အဟောင်း `zap init` မဟုတ်ပါ။ User-managed Web project တစ်ခုလုံးကို command တစ်ကြောင်းတည်းဖြင့် ဖန်တီးရန် `zap new <directory>` ကို အသုံးပြုပါ။

```bash
zap new my_app
cd my_app
zap check
zap build --locked
zap test tests
zap dev
```

Legacy links များ မပျက်စေရန် ဤဖိုင်ကို ချန်ထားခြင်းဖြစ်ပြီး language semantics နှင့် command contract အတွက် canonical source သည် Language Guide၊ Language Specification နှင့် CLI help တို့ ဖြစ်ပါသည်။
