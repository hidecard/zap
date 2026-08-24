# Zap v2.10.0 Release Notes

**Release line:** v2.10.0
**စစ်ဆေးထားသော baseline:** v2.10.0 tag မတိုင်မီ latest master ရှိ Zap v2.9.2
**အခြေအနေ:** Web validation နှင့် Result middleware increment release

## အနှစ်ချုပ်

Zap v2.10.0 တွင် bounded native Web request-validation contract နှင့် centralized Result-aware response boundary ကို ထည့်သွင်းထားပါသည်။ Web handler များသည် parse လုပ်ပြီးသား map သို့မဟုတ် raw JSON text ကို `web_validate_request(body, schema)` ဖြင့် စစ်ဆေးပြီး typed `ResultOk` သို့မဟုတ် `ResultErr` value များကို error protocol အသစ်တစ်ခု မဖန်တီးဘဲ ပြန်ပေးနိုင်ပါသည်။

Native Web server သည် safe handler `ResultErr` value များကို JSON HTTP error အဖြစ် centrally ပြောင်းပေးပြီး ရှိပြီးသား direct response map များကို ဆက်လက်ထောက်ပံ့ထားပါသည်။ ဤ feature သည် bounded ဖြစ်သောကြောင့် validator က ထုတ်သော malformed-input နှင့် field-validation failure များအတွက် HTTP `400` ကို အသုံးပြုပြီး semantic အရ invalid ဖြစ်သော payload အတွက် handler က `422` ကို ရွေးချယ်နိုင်ပါသည်။

## အကောင်အထည်ဖော်ပြီးသော ပြောင်းလဲမှုများ

| နယ်ပယ် | ပြောင်းလဲမှု | အထောက်အထား |
|---|---|---|
| Typed validation | Schema field အများဆုံး 64 ခုနှင့် raw JSON/map body 64 KiB ကို bounded `text`၊ `number`၊ `bool`၊ `map`၊ `list` နှင့် `none` type များဖြင့် validate လုပ်သည်။ | Native unit regression များ |
| Validation results | Validated map အတွက် `ResultOk` ပြန်ပေးပြီး invalid JSON၊ body shape၊ schema၊ unknown/missing field၊ type mismatch နှင့် text-length violation များအတွက် bounded `ResultErr` map ပြန်ပေးသည်။ | Native unit regression နှင့် scaffold test |
| Central middleware | Status `400..599`၊ bounded token code နှင့် bounded message ပါသော safe `ResultErr` map များကို `error`၊ `message` နှင့် `request_id` ပါသော JSON သို့ ပြောင်းပေးသည်။ | Native Web loopback test |
| Compatibility | Direct response map များကို ဆက်လက်အလုပ်လုပ်စေပြီး malformed Result shape နှင့် handler raise များကို `500 handler_error` အဖြစ် fail closed လုပ်သည်။ | Native Web loopback test |
| `zap new` scaffold | Generated `create_user` function နှင့် test ထဲတွင် validation၊ `is_err`၊ `unwrap` နှင့် `ok` ကို ပြသပြီး valid/invalid JSON ဖြင့် စမ်းသပ်ထားသည်။ | Generated-project smoke test |
| Public contract | `web` standard-library domain နှင့် `web_validate_request` ကို catalog၊ policy table၊ bilingual guide နှင့် VS Code grammar များတွင် register လုပ်ထားသည်။ | Catalog၊ policy၊ documentation၊ asset နှင့် link gate များ |

## အသုံးပြုပုံ

```zap
export fn create_user(request):
    let schema = {"name": {"type": "text", "max_len": 120}, "email": {"type": "text", "max_len": 254}}
    let checked = web_validate_request(request["body"], schema)
    if is_err(checked):
        return checked
    let payload = unwrap(checked)
    return ok({"status": 201, "body": json({"created": true, "body": payload})})
```

အောင်မြင်သော validation သည် `ResultOk(map)` ကို ပြန်ပေးပါသည်။ မျှော်မှန်းထားသော request သို့မဟုတ် schema failure များသည် `ResultErr({"status": 400, "code": "...", "message": "...", "field": "..."})` အဖြစ် ပြန်လာပါသည်။ `field` entry ကို Zap code ထဲတွင် အသုံးပြုနိုင်သော်လည်း native HTTP boundary သည် bounded public field ဖြစ်သော `error`၊ `message` နှင့် `request_id` ကိုသာ ထုတ်ပေးပါသည်။ Syntax မှန်ကန်သော်လည်း semantic အရ လက်မခံနိုင်သော payload အတွက် handler code က `422` ကဲ့သို့ အခြား safe error status ကို ပြန်ပေးနိုင်ပါသည်။

## Compatibility နှင့် boundaries

ဤသည်မှာ ရှိပြီးသား user-managed project structure အတွက် additive native Web capability ဖြစ်ပါသည်။ Hidden app registration သို့မဟုတ် Django-style `startapp` command မထည့်ထားပါ။ `models/`၊ `functions/`၊ `ui/`၊ `routes/`၊ `middleware/`၊ `migrations/`၊ `admin/`၊ `public/` နှင့် `tests/` directory များသည် ပြင်ဆင်နိုင်သော Zap module များအဖြစ် ဆက်ရှိပြီး browser build output ကို runtime တွင် Node.js မလိုဘဲ deploy လုပ်နိုင်ပါသည်။

Validator သည် complete schema compiler မဟုတ်ပါ။ Bounded type set အသေးစားကိုသာ support လုပ်ပြီး coercion၊ nested schema compilation၊ database-backed uniqueness check၊ authentication သို့မဟုတ် business-rule validation မလုပ်ပါ။ Result adapter သည် complete middleware graph မဟုတ်သကဲ့သို့ production TLS၊ graceful shutdown၊ backpressure၊ observability၊ async I/O၊ provider-neutral ORM/database support၊ cross-file refactoring၊ SSR/template compilation၊ WebSocket/streaming upload၊ built-in admin UI သို့မဟုတ် real mobile/AI/IoT provider adapter များ ပြီးစီးပြီဟုလည်း မဆိုထားပါ။ ၎င်းတို့သည် implementation နှင့် evidence လိုအပ်သည့် သီးခြား milestone များအဖြစ် ဆက်ရှိပါသည်။

## Verification

Release candidate သည် native formatting၊ full native test suite၊ release compilation၊ framework starter validation၊ standard-library policy checks၊ documentation consistency၊ Markdown link validation၊ VS Code asset validation နှင့် clean-tree release preflight များကို pass ဖြစ်ခဲ့ပါသည်။ Generated project smoke path တွင် `zap new`၊ `zap check`၊ `zap web check`၊ `zap web routes`၊ database migration command များနှင့် `zap test` တို့ pass ဖြစ်ခဲ့ပါသည်။ Tagged workflow တွင် platform archive၊ checksum၊ signature၊ provenance၊ manifest၊ installer နှင့် published asset များကို ထပ်မံ verify လုပ်ရမည်။

## References

[1]: ../docs/ZAP_WEB_NATIVE_MM.md
[2]: ../docs/WEB_FRAMEWORK_MM.md
[3]: ../docs/LEARN_ZAP_MM.md
[4]: ../docs/STDLIB_POLICY_MM.md
