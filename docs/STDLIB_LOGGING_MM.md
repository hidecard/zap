# Zap Structured Logging Standard Library

`logging` domain သည် machine-readable event များအတွက် deterministic structured record builder များကို ပေးပါသည်။ Runtime သည် validation ပြီးသော record ကိုသာ ပြန်ပေးပြီး stdout သို့မဟုတ် stderr သို့ တိုက်ရိုက်မရေးပါ။ ထို့ကြောင့် application သည် မိမိလိုအပ်သော output sink ကို ရွေးချယ်နိုင်ပါသည်။

## `log_record(level, message, fields)`

`level`၊ `message` နှင့် `fields` key သုံးခုပါသော map ကို ပြန်ပေးပါသည်။ Level သည် `trace`၊ `debug`၊ `info`၊ `warn` သို့မဟုတ် `error` တစ်ခုဖြစ်ရပါမည်။ Message သည် ဗလာမဖြစ်သော text ဖြစ်ရပြီး `fields` သည် map ဖြစ်ရပါမည်။

```zap
let event = log_record("info", "server started", {"port": 8080, "mode": "dev"})
say event["level"]
say event["fields"]["port"]
```

## `log_json(level, message, fields)`

တူညီသော logical record ကို JSON line တစ်ကြောင်းအဖြစ် ပြန်ပေးပါသည်။ Top-level key များကို `fields`၊ `level`၊ `message` အစီအစဉ်ဖြင့် ထုတ်ပေးပြီး field name များကို အက္ခရာစဉ်အလိုက် စီပေးပါသည်။ ထို ordering သည် deterministic API contract ၏ အစိတ်အပိုင်းဖြစ်ပါသည်။

```zap
let line = log_json("warn", "slow request", {"path": "/health", "duration_ms": 250})
say line
```

## Safety limits နှင့် errors

| အမျိုးအစား | Limit |
|---|---:|
| Message size | 8 KiB |
| Field အရေအတွက် | 64 ခု |
| Field-name size | 256 bytes |
| Encoded JSON output | 64 KiB |

မမှန်ကန်သော level၊ ဗလာ message၊ map မဟုတ်သော fields၊ အလွန်ရှည်သော message သို့မဟုတ် field name နှင့် fields 64 ခုကျော်ခြင်းတို့သည် stable runtime error ပြန်ပေးပါသည်။ JSON output ကို ကန့်သတ်ထားပြီး data ကို တိတ်တဆိတ် truncate မလုပ်ပါ။

Structured logging သည် timestamp ကို အလိုအလျောက် မထည့်ပါ။ Event time လိုအပ်ပါက `utc_now()` ကို အသုံးပြုပြီး fields map ထဲသို့ explicit ထည့်နိုင်ပါသည်။

```zap
let current = utc_now()
let event = log_record("debug", "poll completed", {
    "unix_millis": current["unix_millis"],
    "items": 12
})
say log_json(event["level"], event["message"], event["fields"])
```

> **Determinism guarantee:** `log_json` သည် field name များကို အက္ခရာစဉ်အလိုက် စီပေးပြီး fixed size limits များကို အသုံးပြုပါသည်။ Hash-map iteration order ပေါ်တွင် မမှီခိုဘဲ oversized data ကို wrap သို့မဟုတ် truncate မလုပ်ပါ။

## ဆက်စပ် API များ

Explicit UTC event timestamp အတွက် `utc_now()`၊ အထွေထွေ JSON serialization အတွက် `json()` နှင့် log snapshot ကို file ထဲသို့ လုံခြုံစွာသိမ်းရန် `atomic_write()` ကို အသုံးပြုနိုင်ပါသည်။

[Standard-library index သို့ ပြန်သွားရန်](STDLIB_INDEX_MM.md)

[v2.1 roadmap သို့ ပြန်သွားရန်](V2.1_ROADMAP_MM.md)

---

Author: **Zap project maintainers**
_Last updated: 2026-08-21._

## References

[1]: ../native/src/evaluator.rs "Zap evaluator structured logging implementation and tests"
[2]: ../native/src/stdlib_catalog.rs "Zap public standard-library catalog"

[1] [2]

ဤ guide သည် Zap v2.1-C structured logging slice ကို မှတ်တမ်းတင်ထားပါသည်။
