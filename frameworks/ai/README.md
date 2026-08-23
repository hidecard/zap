# Framework AI Starter

ဤ project သည် current Zap v2.2.6 ဖြင့် run လို့ရသော **provider request/response contract** ဖြစ်သည်။ External model provider ကို မခေါ်ပါ၊ API key မဖတ်ပါ၊ network request မလုပ်ပါ။

## Run

```bash
zap lock
zap check
zap build
zap run main.zp
```

`main.zp` သည် model name၊ prompt၊ maximum output limit၊ response text နှင့် prompt usage record ကို deterministic JSON အဖြစ် model လုပ်သည်။

## Host adapter boundary

နောက်ပိုင်း provider adapter တွင် model selection၊ request/response size limit၊ timeout၊ retry၊ quota error၊ redaction၊ audit retention နှင့် provider-specific failure mapping ကို သတ်မှတ်ရမည်။ API key/secret များကို source code ထဲ မရေးရ၊ diagnostic/log ထဲ မထည့်ရ။

`Framework` branch ၏ scope သည် provider-neutral contract starter အထိသာ ဖြစ်သည်။ အသေးစိတ်ကို [`docs/FRAMEWORK_MM.md`](../../docs/FRAMEWORK_MM.md) တွင် ဖတ်ပါ။
