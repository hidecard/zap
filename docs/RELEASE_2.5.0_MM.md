# Zap v2.5.0 Release Notes

**Release line:** v2.5.0

**အဓိကအကြောင်းအရာ:** Documentation integrity နှင့် safer project operations

## အနှစ်ချုပ်

Zap v2.5.0 သည် v2.4.0 တွင် သတ်မှတ်ထားသော learning နှင့် Web direction ကို user-facing surface ပိုမိုတည်ငြိမ်စေရန် စုစည်းထားသော release ဖြစ်ပါသည်။ One-command user-managed `zap new <directory>` workflow ကို ဆက်လက်ထိန်းသိမ်းထားပြီး repository-wide Markdown link validation ကို CI နှင့် release preflight ထဲသို့ ထည့်သွင်းထားပါသည်။ Operational metadata အဟောင်းများကို ပြင်ဆင်ထားပြီး development/reference Web slice နှင့် production deployment ကြား boundary ကို ပိုမိုရှင်းလင်းစွာ ဖော်ပြထားပါသည်။

## ပြောင်းလဲထားသောအရာများ

| အပိုင်း | ပြောင်းလဲမှု |
|---|---|
| Documentation integrity | Repository-wide relative Markdown link validator အသစ်ကို ထည့်ပြီး CI နှင့် release preflight တွင် ချိတ်ဆက်ထားပါသည်။ Tracked Markdown file များကို စစ်ပြီး external URL များကို ကျော်သွားကာ repository ပြင်ပသို့ ထွက်သည့် link နှင့် မရှိသော target များကို file/line number နှင့် report လုပ်ပါသည်။ |
| Canonical learning path | အဟောင်း Burmese-only `docs/LANGUAGE_GUIDE.md` course ကို maintained bilingual Language Guide များသို့ညွှန်ပြသော compatibility alias အဖြစ် ပြောင်းထားပါသည်။ Project အသစ်များအတွက် `zap new` ကို သုံးပြီး `zap init` ကို compatibility command အဖြစ်သာ မှတ်တမ်းတင်ထားပါသည်။ |
| Usage နှင့် operations | Bilingual usage guide များကို ပြန်ရေးပြီး deployment၊ production-operations၊ RustSec၊ standard-library၊ ecosystem နှင့် progress metadata များကို v2.5.0 development line နှင့် ညှိထားပါသည်။ Roadmap အဟောင်းများကို လိုအပ်သည့်နေရာတွင် historical ဟု label တပ်ထားပါသည်။ |
| Provenance | Immutable v2.4.0 tag ကို နောက်ပိုင်း master correction များနှင့် မရောထွေးစေရန် bilingual post-v2.4.0 remediation/provenance record များ ထည့်ထားပါသည်။ |
| User-managed Web | `models/`, `functions/`, `ui/`, `routes/`, `middleware/`, `migrations/`, `admin/`, `public/` နှင့် `tests/` တို့ပါသော explicit project layout ကို ဆက်လက်ထိန်းသိမ်းထားပြီး hidden app registry သို့မဟုတ် Django-style `startapp` မထည့်ထားပါ။ |

## အရေးကြီးသော boundary

ဤ release သည် complete ORM၊ provider-neutral production migration platform၊ user-defined trait သို့မဟုတ် generic declaration၊ production async I/O reactor၊ cross-file semantic rename၊ SSR/template compiler၊ WebSocket/streaming/upload stack၊ built-in admin UI သို့မဟုတ် real mobile/AI/IoT provider adapter များ ပြီးစီးပြီဟု မဆိုထားပါ။ ၎င်းတို့သည် language contract၊ security evidence နှင့် platform test လိုအပ်သော သီးခြား milestone များဖြစ်သည်။

## Verification

Release branch သည် pinned Rust formatting/test gate၊ strict Clippy၊ framework starter validation၊ documentation consistency နှင့် bilingual parity၊ repository-wide Markdown link validation၊ release-version check၊ VS Code asset validation၊ LSP semantic parity၊ registry deployment check နှင့် complete release preflight အားလုံးကို tagging မလုပ်မီ pass ရမည်။

## Upgrade guidance

v2.5.0 ကို install လုပ်မည့် user များသည် မိမိ platform နှင့် architecture ကိုက်ညီသော archive ကို download လုပ်ပြီး checksum နှင့် signature verify လုပ်ကာ `zap --version` ဖြင့် စစ်ဆေးပါ။ ရှိပြီးသား `.zp` project များသည် manifest နှင့် lockfile workflow အတိုင်း ဆက်လက်အသုံးပြုနိုင်ပါသည်။ Web project အသစ်များအတွက်—

```bash
zap new my_app
cd my_app
zap check
zap build --locked
zap test tests
zap dev
```

အသေးစိတ်လေ့လာရန် [မြန်မာ Language Guide](LEARN_ZAP_MM.md) သို့မဟုတ် [English Language Guide](LEARN_ZAP_EN.md) ကို ဖတ်ပါ။ Release provenance အတွက် [post-v2.4.0 remediation record](POST_V2.4.0_REMEDIATION_MM.md) ကို ဖတ်ပါ။
