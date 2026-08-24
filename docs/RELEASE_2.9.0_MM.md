# Zap v2.9.0 Release Notes

**Release line:** v2.9.0
**စစ်ဆေးထားသော baseline:** merged `master` ရှိ Zap v2.8.0
**အခြေအနေ:** Web framework safety increment release

## အနှစ်ချုပ်

Zap v2.9.0 တွင် Web route validation ကို inspection၊ project check နှင့် native development server အကြား တူညီအောင် ပြုလုပ်ထားပါသည်။ Duplicate method/path registration များကို route table မပြမီနှင့် listener က traffic လက်မခံမီ reject လုပ်ပါသည်။

တူညီသော shared route-table validator ကို `zap web check` နှင့် `zap web routes` နှစ်ခုလုံးတွင် အသုံးပြုထားပါသည်။ ထို့ကြောင့် route conflict များသည် declaration order ပေါ်မူတည်သော runtime behavior မဖြစ်တော့ဘဲ အစောပိုင်း deterministic project error အဖြစ် ပြသပါသည်။

## အကောင်အထည်ဖော်ပြီးသော ပြောင်းလဲမှုများ

| နယ်ပယ် | ပြောင်းလဲမှု | အထောက်အထား |
|---|---|---|
| Route table | Export လုပ်ထားသော route list တစ်ခုလုံးအတွင်း method/path registration unique ဖြစ်/မဖြစ် validate လုပ်သည်။ | Native unit regression |
| `zap web check` | Export လုပ်ထားသော `routes()` factory ကို execute လုပ်ပြီး malformed သို့မဟုတ် conflict ဖြစ်သော route entry များကို project validation အတွင်း reject လုပ်သည်။ | Generated-project smoke test |
| `zap web routes` | Text နှင့် JSON inspection နှစ်မျိုးလုံးအတွက် တူညီသော route-table validator ကို ပြန်သုံးသည်။ | CLI smoke test |
| Live server | Shared conflict check နှင့် named-handler များကို serve မလုပ်မီ strict resolution ကို ဆက်ထိန်းသိမ်းသည်။ | Native Web contract tests |
| Documentation | English/Burmese Web နှင့် framework guidance များကို executable behavior နှင့် synchronize လုပ်ထားသည်။ | Documentation/link gates |

## အသုံးပြုပုံ

```bash
zap new shop
cd shop
zap web check
zap web routes
zap web routes --json
zap dev
```

Route table တွင် `GET /users` နှင့် `POST /users` ကဲ့သို့ path တူပြီး method မတူသော registration များကို ခွင့်ပြုပါသည်။ သို့သော် method/path pair တစ်ခုတည်းကို နှစ်ကြိမ်ထက်ပို၍ register မလုပ်ရပါ။ `zap web check` နှင့် `zap web routes` သည် network listener မစတင်ဘဲ conflict ကို report လုပ်ပါသည်။ `zap dev` သည် အတူတူ conflict check လုပ်ပြီး serve မလုပ်မီ named handler များကိုလည်း resolve လုပ်ပါသည်။

## Compatibility နှင့် boundaries

ဤသည်မှာ ရှိပြီးသား user-managed project structure အတွက် additive safety improvement ဖြစ်ပါသည်။ Hidden app registration သို့မဟုတ် Django-style `startapp` command အသစ် မထည့်ထားပါ။ `models/`၊ `functions/`၊ `ui/`၊ `routes/`၊ `middleware/`၊ `migrations/`၊ `admin/`၊ `public/` နှင့် `tests/` directory များသည် ပြင်ဆင်နိုင်သော Zap module များအဖြစ် ဆက်ရှိပြီး browser build output ကို runtime တွင် Node.js မလိုဘဲ deploy လုပ်နိုင်ပါသည်။

ဤ release သည် first-class route syntax၊ automatic JSON validation schema၊ centralized `Result` error middleware၊ production async I/O reactor၊ provider-neutral ORM/database support၊ cross-file refactoring၊ incremental compilation၊ debugger/profiler integration၊ SSR/template compilation၊ WebSocket/streaming upload၊ built-in admin UI သို့မဟုတ် real mobile/AI/IoT provider adapter များ ပြီးစီးပြီဟု မဆိုထားပါ။ ၎င်းတို့သည် implementation နှင့် evidence လိုအပ်သည့် သီးခြား milestone များအဖြစ် ဆက်ရှိပါသည်။

## Verification

Release branch သည် native formatting၊ full native test suite၊ framework starter validation၊ documentation consistency၊ Markdown link validation၊ VS Code asset validation၊ deployment checks နှင့် clean-tree release preflight များ pass ဖြစ်ရမည်။ Tagged workflow တွင် Linux x86_64၊ macOS ARM64 နှင့် Windows x86_64 archive၊ checksum၊ signature၊ provenance၊ manifest၊ installer နှင့် published asset များကို verify လုပ်ရမည်။

## References

[1]: ../docs/ZAP_WEB_NATIVE_MM.md
[2]: ../docs/WEB_FRAMEWORK_MM.md
[3]: ../docs/LEARN_ZAP_MM.md
