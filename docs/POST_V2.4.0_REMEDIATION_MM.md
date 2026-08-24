# v2.4.0 နောက်ပိုင်း ပြင်ဆင်ချက်နှင့် Provenance မှတ်တမ်း

**စစ်ဆေးထားသော baseline:** Zap v2.11.5

**မှတ်တမ်းအခြေအနေ:** လက်ရှိ master ၏ follow-up record; v2.4.0 သည် immutable ဖြစ်သည်

**နယ်ပယ်:** ဤမှတ်တမ်းသည် publish လုပ်ပြီးသော v2.4.0 tag နောက်ပိုင်းတွင် ထည့်သွင်းနိုင်သည့် correction နှင့် documentation အလုပ်များကို သတ်မှတ်သည်။ v2.4.0 release ကို ပြန်ရေးခြင်း မဟုတ်သလို နောက်ပိုင်း commit များသည် v2.4.0 archive ထဲတွင် ပါဝင်သည်ဟုလည်း မဆိုလိုပါ။

## Provenance boundary

Publish လုပ်ပြီးသော [`v2.4.0` release](https://github.com/hidecard/zap/releases/tag/v2.4.0) နှင့် annotated tag များသည် immutable ဖြစ်သည်။ v2.4.0 ကို install လုပ်သော user သည် ထို tag နှင့် သက်ဆိုင်သော binary၊ checksum၊ signature၊ provenance metadata နှင့် documentation များကိုသာ ရရှိမည်ဖြစ်သည်။ နောက်ပိုင်း `master` commit များသည် သီးခြား version decision နှင့် release validation လိုအပ်သည်။

| Boundary | အဓိပ္ပာယ် |
|---|---|
| v2.4.0 tag | Publish ပြီး immutable ဖြစ်သည်။ နောက်ပိုင်း correction များကို tag ထဲသို့ backport မလုပ်ပါ။ |
| လက်ရှိ master | Follow-up fix နှင့် feature အသစ်များအတွက် development surface ဖြစ်သည်။ Release artifact အဖြစ် အလိုအလျောက် မသတ်မှတ်ရပါ။ |
| နောက် release | Changelog၊ bilingual notes၊ version metadata၊ tests နှင့် complete preflight gate ပါရမည်။ |

## ပြင်ဆင်ရန် queue

| အပိုင်း | လက်ရှိအခြေအနေ | Release မလုပ်မီ လိုအပ်သော evidence |
|---|---|---|
| Documentation links နှင့် baselines | Bilingual navigation၊ active usage material နှင့် release metadata များကို လက်ရှိ release line နှင့် ညှိနေသည်။ | Repository-relative link validation၊ bilingual parity နှင့် historical label များ မှန်ကန်ရမည်။ |
| Language semantics | Optional annotation၊ bounded generic form၊ module၊ class၊ Result/Option နှင့် structured diagnostic များ ရှိသည်။ Generic declaration၊ trait၊ pattern matching နှင့် typed intermediate representation များသည် deferred ဖြစ်သည်။ | Normative specification update၊ parser/evaluator conformance fixture၊ stable diagnostic၊ LSP parity နှင့် migration note လိုအပ်သည်။ |
| Async runtime | Deterministic single-thread scheduling၊ cancellation၊ timeout နှင့် poll budget များ ရှိသည်။ | Stream/channel semantics၊ structured cancellation၊ external-I/O lifecycle test နှင့် worker/isolate boundary များမရှိဘဲ production concurrency claim မပြုရ။ |
| Web runtime | User-managed scaffold၊ bounded development server၊ static/SPA asset၊ DTO/auth/rate-limit contract နှင့် SQLite-first migration များ ရှိသည်။ | Production listener behavior၊ shutdown/backpressure၊ auth/session persistence၊ database adapter test၊ deployment evidence နှင့် security review လိုအပ်သည်။ |
| LSP/editor | Full synchronization၊ diagnostic၊ hover၊ completion၊ definition၊ formatting၊ file-local rename နှင့် workspace symbol များ ရှိသည်။ | Incremental synchronization နှင့် cross-file semantic refactoring များကို implement/test မလုပ်မီ complete feature အဖြစ် မကြော်ငြာရ။ |

## Release policy

Branch သို့မဟုတ် pull request တစ်ခုတွင် ရှိနေခြင်းတစ်ခုတည်းဖြင့် follow-up change ကို v2.4.0 ၏ အစိတ်အပိုင်းဟု မရေးရပါ။ Annotated tag၊ cross-platform workflow၊ signed artifact၊ checksum၊ provenance နှင့် published-release verification များ ကိုက်ညီပြီးမှသာ release version ကို authoritative ဟု သတ်မှတ်ရမည်။ Historical release notes များသည် historical အဖြစ် ဆက်ရှိရမည်။ လက်ရှိ behavior ကို current specification၊ guide နှင့် release notes ထဲတွင် ထည့်ရမည်။

## ကိုးကားချက်များ

1. [Zap v2.4.0 release](https://github.com/hidecard/zap/releases/tag/v2.4.0)။
2. [Normative language specification](LANGUAGE_SPEC_MM.md)။
3. [Framework Foundation boundary](FRAMEWORK_MM.md)။
4. [Documentation navigation](DOCUMENTATION_NAVIGATION_MM.md)။
