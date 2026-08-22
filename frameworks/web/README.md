# Framework Web Starter

ဤ project သည် current Zap v2.2.3 ဖြင့် run လို့ရသော **Web API contract** ဖြစ်သည်။ Reusable logic ကို `web_contract.zp`, `api_contract.zp`, `dto_contract.zp`, `database_contract.zp`, `auth_contract.zp` နှင့် `rate_limit_contract.zp` modules တွင် ခွဲထားပြီး HTTP server မဖွင့်ပါ၊ TCP socket မဖတ်ပါ၊ TLS နှင့် raw credentials မသုံးပါ။

## Run

```bash
zap lock
zap check
zap build
zap run main.zp
zap test .
```

`main.zp` သည် `/` နှင့် `/health` ၏ `200`၊ `/echo` ၏ `POST 200`၊ မသိသော route ၏ `404`၊ traversal path ၏ `400` နှင့် unsupported method ၏ `405` response map များကို စစ်ဆေးပြီး `GET/POST user API` တွင် DTO mapping၊ repository lookup/insert၊ `401/403/404/429` policy များကို ပြသသည်။ `web_contract_test.zp` နှင့် `api_contract_test.zp` သည် exported modules ကို တိုက်ရိုက် regression test လုပ်သည်။ Output သည် deterministic JSON ဖြစ်သည်။

## API modules

`api_contract.zp` သည် application orchestration ကို ကိုင်တွယ်ပြီး `dto_contract.zp` သည် public input/output fields များကို validate/map လုပ်သည်။ `database_contract.zp` သည် driver မချိတ်ထားသော repository DTO boundary ဖြစ်သည်။ `auth_contract.zp` သည် host မှ verify လုပ်ပြီးသား identity/scopes ပေါ်တွင် `401/403` authorization ပြုလုပ်ပြီး `rate_limit_contract.zp` သည် atomic host storage ဖြင့် အသုံးပြုရမည့် deterministic fixed-window state ကို ပြန်ပေးသည်။

Production သို့ မတင်မီ real driver adapter တွင် parameterized queries၊ transaction/pool timeout၊ credential redaction၊ subject binding၊ monotonic clock၊ atomic quota store၊ timeout/cancellation နှင့် duplicate-insert policy များကို ထည့်သွင်းစစ်ဆေးရမည်။

## Host adapter boundary

နောက်ပိုင်း Web adapter သည် incoming HTTP request ကို bounded Zap map အဖြစ် ပြောင်းပြီး returned response map ကို HTTP response အဖြစ် ပြောင်းရမည်။ Method/path normalization၊ header/body limits၊ timeout၊ cancellation၊ error mapping၊ redaction နှင့် shutdown ကို adapter က ပိုင်ဆိုင်ရမည်။

ပထမဆုံး adapter prototype ကို existing HTTP stack အပေါ် တည်ဆောက်ထားသည်။ `Framework` branch ၏ Zap contract scope သည် dependency-free အတိုင်း ရှိနေပြီး host implementation ကို [`host/zap-host`](../../host/zap-host) တွင် သီးခြားထားသည်။ Run စစ်ဆေးရန် `cd host/zap-host && cargo test --all-targets` ကို သုံးပါ။ အသေးစိတ်ကို [`docs/FRAMEWORK_MM.md`](../../docs/FRAMEWORK_MM.md) နှင့် [`docs/ZAP_HOST_MM.md`](../../docs/ZAP_HOST_MM.md) တွင် ဖတ်ပါ။
