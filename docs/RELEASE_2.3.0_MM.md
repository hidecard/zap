# Zap v2.3.0 Release Notes

**အခြေအနေ:** Source integration baseline နှင့် release-contract record
**ရက်စွဲ:** 2026-08-23

## အကျဉ်းချုပ်

Zap v2.3.0 တွင် လက်ရှိ master runtime နှင့် Framework branch၊ security-maintenance branch များမှ review ပြီးသောအလုပ်များကို ပေါင်းစည်းထားပါသည်။ Native executable သည် project execution boundary အဖြစ် ဆက်လက်ရှိနေပြီး Web scaffold တွင် model၊ business function၊ browser UI၊ route၊ middleware၊ migration၊ admin နှင့် test များကို ပိုမိုရှင်းလင်းစွာ ခွဲရေးနိုင်ပါသည်။

## ထည့်သွင်းထားသောအလုပ်များ

- Framework runtime၊ frontend interoperability boundary၊ authentication/deployment contract၊ production-operation documentation နှင့် host-adapter update များကို ပေါင်းစည်းထားပါသည်။
- JSON cycle protection၊ collection-producing builtin များအတွက် bound၊ DNS validation ပြီးနောက် restricted HTTP connection pinning၊ registry-operation hardening၊ RustSec evidence နှင့် macOS native web-request test fix များကို ထည့်သွင်းထားပါသည်။
- `zap new` သည် `ui/ui.zp` module ကို ထုတ်ပေးပါသည်။ ထို module တွင် browser entrypoint၊ asset root၊ frontend mode နှင့် runtime တွင် Node.js မလိုကြောင်း မှတ်တမ်းတင်ထားပါသည်။
- English/Burmese Web guide များတွင် Model/Function/UI ခွဲခြားပုံနှင့် React၊ Vue၊ Svelte၊ Alpine သို့မဟုတ် ရိုးရိုး HTML/CSS/JavaScript ကို build-time တွင် optional အသုံးပြုနိုင်ပုံကို update လုပ်ထားပါသည်။

## Runtime နှင့် deployment boundary

Zap project ကို deploy လုပ်ရာတွင် installed Zap executable နှင့် project ၏ declared asset များ လိုအပ်ပါသည်။ Deployment host တွင် Python၊ Node.js၊ Rust၊ Java သို့မဟုတ် အခြား application runtime မလိုအပ်ပါ။ JavaScript framework toolchain များသည် build-time အတွက် optional ဖြစ်ပြီး ထွက်လာသော static file များကို declared public asset root မှ serve လုပ်နိုင်ပါသည်။

Web server နှင့် database/authentication integration များသည် development/reference နှင့် adapter boundary စာတမ်းများအတိုင်းသာ သတ်မှတ်ထားပါသည်။ Release asset များ ဖြန့်ချိမီ release preflight၊ checksum၊ signature၊ provenance၊ deployment နှင့် security gate များကို ပြီးစီးအောင် စစ်ဆေးရမည်။

## Validation evidence

Integrated branch သည် Rust formatting၊ native test suite 258 tests၊ release compilation၊ Framework starter validator 193 checks နှင့် generated Web scaffold အတွက် `zap check`၊ `zap run` နှင့် `zap test tests` များကို development environment တွင် အောင်မြင်ခဲ့ပါသည်။
