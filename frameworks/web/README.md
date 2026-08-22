# Framework Web Starter

ဤ project သည် current Zap v2.2.3 ဖြင့် run လို့ရသော **route/request/response contract** ဖြစ်သည်။ HTTP server မဖွင့်ပါ၊ TCP socket မဖတ်ပါ၊ TLS နှင့် credentials မသုံးပါ။

## Run

```bash
zap lock
zap check
zap build
zap run main.zp
```

`main.zp` သည် `/` အတွက် `200`၊ `/health` အတွက် `200` နှင့် မသိသော route အတွက် `404` response map များကို စစ်ဆေးသည်။ Output သည် deterministic JSON ဖြစ်သည်။

## Host adapter boundary

နောက်ပိုင်း Web adapter သည် incoming HTTP request ကို bounded Zap map အဖြစ် ပြောင်းပြီး returned response map ကို HTTP response အဖြစ် ပြောင်းရမည်။ Method/path normalization၊ header/body limits၊ timeout၊ cancellation၊ error mapping၊ redaction နှင့် shutdown ကို adapter က ပိုင်ဆိုင်ရမည်။

ပထမဆုံး real adapter ကို existing HTTP stack အပေါ် တည်ဆောက်ရန် အကြံပြုသည်။ `Framework` branch ၏ scope သည် contract starter အထိသာ ဖြစ်ပြီး production listener မဟုတ်ပါ။ အသေးစိတ်ကို [`docs/FRAMEWORK_MM.md`](../../docs/FRAMEWORK_MM.md) တွင် ဖတ်ပါ။
