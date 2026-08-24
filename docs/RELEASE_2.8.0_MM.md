# Zap v2.8.0 Release Notes

**Release line:** v2.8.0
**စစ်ဆေးထားသော baseline:** merged `master` ရှိ Zap v2.7.0
**အခြေအနေ:** Web developer-experience increment release

## အနှစ်ချုပ်

Zap v2.8.0 တွင် user-managed Web project ၏ export လုပ်ထားသော route table ကို listener မဖွင့်ဘဲ စစ်ဆေးနိုင်သည့် deterministic၊ read-only `zap web routes` command ကို ထည့်သွင်းထားပါသည်။ Human-readable output နှင့် editor tooling/script များအတွက် `--json` output နှစ်မျိုးလုံးကို support လုပ်ပါသည်။

ထို့အပြင် generated `main.zp` ထဲတွင် placeholder `APP_NAME` အစား project name အမှန်ကို ထည့်ပေးရန် Web scaffold ကို ပြင်ဆင်ထားပါသည်။ Scaffold validator သည် output mode နှစ်မျိုးလုံးဖြင့် route inspection ကို စမ်းသပ်ပြီး generated project metadata သည် project name အလိုက် ဖြစ်ကြောင်း verify လုပ်ပါသည်။

## အကောင်အထည်ဖော်ပြီးသော ပြောင်းလဲမှုများ

| နယ်ပယ် | ပြောင်းလဲမှု | အထောက်အထား |
|---|---|---|
| Route inspection | Export လုပ်ထားသော `routes()` factory ကို execute လုပ်ပြီး method၊ path၊ handler နှင့် ရှိပါက scope ကို ပြသည်။ | CLI smoke test |
| Route safety | ပြသမီ route entry shape၊ safe absolute path၊ method token နှင့် handler shape ကို validate လုပ်သည်။ | Shared evaluator contract |
| Server safety | Live `web_serve` boundary အတွင်း strict handler resolution ကို ဆက်လက်ထိန်းသိမ်းထားသည်။ | Native Web tests |
| Scaffold clarity | Generated `APP_NAME` placeholder ကို sanitized project directory name ဖြင့် အစားထိုးသည်။ | Framework starter regression |
| Automation | Starter validation ထဲတွင် human-readable နှင့် JSON route inspection နှစ်မျိုး ထည့်ထားသည်။ | Framework validator |
| Documentation | English/Burmese Web၊ framework၊ learner နှင့် CLI reference များကို synchronize လုပ်ထားသည်။ | Documentation/link gates |

## အသုံးပြုပုံ

```bash
zap new shop
cd shop
zap web check
zap web routes
zap web routes --json
zap dev
```

Normal output သည် project ကို developer က ဖတ်ရှုရန် ဖြစ်ပြီး JSON output သည် editor integration နှင့် automation အတွက် ဖြစ်ပါသည်။ Inspection သည် server မစတင်ပါ၊ route ကို execute လုပ်ခွင့်လည်း မပေးပါ။ Development server သည် traffic လက်မခံမီ ပိုမိုတင်းကျပ်သော handler-resolution စစ်ဆေးမှုကို ဆက်လုပ်ပါသည်။

## Compatibility နှင့် boundaries

ဤ command သည် additive ဖြစ်ပြီး ရှိပြီးသား `zap new`၊ `zap web check`၊ `zap dev` နှင့် full-document project workflow များကို မပြောင်းလဲပါ။ User-managed `models/`၊ `functions/`၊ `ui/`၊ `routes/`၊ `middleware/`၊ `migrations/`၊ `admin/`၊ `public/` နှင့် `tests/` directory များသည် ပြင်ဆင်နိုင်သော ရိုးရိုး Zap module များအဖြစ် ဆက်ရှိပါသည်။ Frontend build tool များသည် build-time တွင်သာ optional ဖြစ်ပြီး deployed process အတွက် Zap runtime နှင့် ထုတ်ပြီးသား browser asset များသာ လိုအပ်ပါသည်။

ဤ release သည် first-class route syntax၊ automatic JSON validation schema၊ centralized `Result` error middleware၊ production async I/O reactor၊ provider-neutral production ORM/database platform၊ cross-file refactoring၊ incremental compilation၊ debugger/profiler integration၊ SSR/template compilation၊ WebSocket/streaming upload၊ built-in admin UI သို့မဟုတ် real mobile/AI/IoT provider adapter များ ပြီးစီးပြီဟု မဆိုထားပါ။ ၎င်းတို့သည် implementation နှင့် platform evidence လိုအပ်သည့် သီးခြား milestone များအဖြစ် ဆက်ရှိပါသည်။

## Verification

Release branch သည် native formatting၊ full native test suite၊ framework starter validation၊ documentation consistency၊ Markdown link validation၊ VS Code asset validation၊ deployment checks နှင့် clean-tree release preflight များ pass ဖြစ်ရမည်။ Tagged workflow တွင် Linux x86_64၊ macOS ARM64 နှင့် Windows x86_64 archive၊ checksum၊ signature၊ provenance၊ manifest၊ installer behavior နှင့် published asset များကိုလည်း verify လုပ်ရမည်။

## References

[1]: ../docs/ZAP_WEB_NATIVE_MM.md
[2]: ../docs/WEB_FRAMEWORK_MM.md
[3]: ../docs/LEARN_ZAP_MM.md
