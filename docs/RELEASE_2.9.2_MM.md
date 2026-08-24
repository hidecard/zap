# Zap v2.9.2 Release Notes

**Release line:** v2.9.2
**အဓိကအကြောင်းအရာ:** scaffold correctness၊ installer reliability၊ bounded host behavior နှင့် release hardening။

## အဓိကပြောင်းလဲမှုများ

Zap v2.9.2 သည် ပထမဆုံးအသုံးပြုချိန် developer experience နှင့် production-facing default များကို ပြင်ဆင်သည့် maintenance release ဖြစ်ပါသည်။ Web project အသစ်ဖန်တီးရာတွင် သတ်မှတ်ထားသော project name ကို မှန်ကန်စွာသုံးပြီး linter-compatible source ထုတ်ပေးကာ documented validation workflow ကို ဖြတ်နိုင်ပါသည်။

Unix installer နှင့် portable uninstaller သည် `ZAP_INSTALL_DIR` သတ်မှတ်ထားပါက default `~/.local/bin` အစား သတ်မှတ်ထားသော directory ကို shell profile ထဲတွင် PATH အဖြစ် မှန်ကန်စွာ သိမ်းပေးပါသည်။ Makefile သည် actual legacy test suite ကို ညွှန်ပြပြီး locked native၊ host၊ legacy နှင့် aggregate test target များ ပေးထားပါသည်။

Standalone release archive များတွင် README navigation အတွက် လိုအပ်သော Markdown documentation များ ပါဝင်လာပါသည်။ Release workflow တွင်လည်း least-privilege job permission နှင့် reviewed immutable action reference များကို အသုံးပြုထားပါသည်။

## Safety နှင့် reliability

Host adapter တွင် production mode guard ထည့်ထားသောကြောင့် JWT configuration မပြည့်စုံပါက demo authenticator ကို တိတ်တဆိတ် မရွေးတော့ပါ။ User-list response များကို hard maximum နှင့် pagination contract ဖြင့် bounded လုပ်ထားပါသည်။ Demo repository သည် local development အတွက်သာဖြစ်ပြီး production-ready ဟု သတ်မှတ်ရန် real persistent repository adapter လိုအပ်ဆဲဖြစ်ပါသည်။

## Upgrade မှတ်ချက်

ရှိပြီးသား Zap source file နှင့် lockfile များသည် v2.7.0 language/runtime baseline နှင့် ဆက်လက်ကိုက်ညီပါသည်။ Upgrade ပြီးနောက် `zap check`၊ `zap build --locked` နှင့် `zap test` ကို ပြန် run ပါ။ Custom Unix installation directory အသုံးပြုထားပါက ပြင်ဆင်ပြီးသော PATH entry ရေးရန် `install.sh` ကို ပြန် run ပါ။

## Verification

Release သည် version consistency၊ formatting၊ Clippy၊ native နှင့် host tests၊ documentation/link checks၊ scaffold smoke tests၊ release preflight၊ deterministic packaging၊ checksum/signature verification နှင့် supported release target များအတွက် installer verification များကို ဖြတ်ရန် ရည်ရွယ်ထားပါသည်။
