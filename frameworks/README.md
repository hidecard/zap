# Zap Framework Foundation

ဤ directory သည် Web, App, IoT နှင့် AI အတွက် **run လို့ရသော contract starter များ** ပါဝင်သော Framework Foundation ဖြစ်ပါသည်။ Starter များသည် current Zap v2.2.3 syntax ဖြင့်ရေးထားပြီး domain model၊ validation နှင့် deterministic output ကို ပြသပါသည်။ HTTP server၊ native mobile UI၊ real MQTT/device connection သို့မဟုတ် external AI provider ကို ယခု branch တွင် မဖွင့်သေးပါ။

```text
frameworks/
  web/       route/request/response contract
  mobile/    app-state/navigation contract
  ai/        provider request/response contract
  iot/       telemetry/device-state contract
```

## Run a starter

Starter တစ်ခုချင်းစီသည် `zap.toml`, `zap.lock` နှင့် `main.zp` ပါသော standalone project ဖြစ်ပါသည်။ Repository root မှ—

```bash
cd frameworks/web
zap lock
zap check
zap build
zap run main.zp
```

`mobile`, `iot`, `ai` directories များအတွက်လည်း command တူညီပါသည်။ Starter လေးခုလုံးသည် dependency-free ဖြစ်သောကြောင့် generated lockfile တွင် dependency သုညခုရှိပါသည်။

## Contract boundary

Framework starter သည် domain contract ကိုသာ ပိုင်ဆိုင်ပြီး platform adapter သည် အောက်ပါအရာများကို ပိုင်ဆိုင်ရမည်။

```text
Zap core → Framework contract → zap-host capability/DTO → platform adapter → OS/device/network/provider
```

Real adapter များတွင် capability ကို default-deny ထားရမည်။ Identity၊ input/output limit၊ deadline၊ cancellation၊ idempotency key၊ trace ID၊ redaction နှင့် typed error ကို မသိမသာ မချန်ထားရပါ။ Runtime logical budget သည် OS sandbox သို့မဟုတ် network egress policy ကို အစားမထိုးပါ။

## Starter status

| Starter | ယခုလုပ်နိုင်သည့်အရာ | မလုပ်သေးသည့်အရာ |
|---|---|---|
| Web | Route table နှင့် response map ကို deterministic စစ်နိုင်; Axum/Tower host adapter ကို `host/zap-host` တွင် run/test နိုင် | TLS, real database/auth provider, shared rate-limit store, production deployment |
| App | Screen/action/state model နှင့် navigation validation | Native renderer, lifecycle, IPC, store |
| IoT | Bounded sensor records နှင့် device-state summary | MQTT, GPIO, serial, OTA, HIL |
| AI | Prompt/request/response schema နှင့် usage record | Provider API, credential, quota, model runtime |

## Development rule

Framework syntax အသစ်ကို language core ထဲသို့ အလျင်မထည့်ရပါ။ Real Web/App/IoT integration မစခင် `docs/FRAMEWORK_EN.md` နှင့် `docs/FRAMEWORK_MM.md` ထဲက capability, error, limit, replay နှင့် security contract များကို အရင်အတည်ပြုရမည်။ Existing host ecosystem များကို အသုံးပြုရန် အကြံပြုပါသည်။ Web အတွက် Axum/Tower, App အတွက် Tauri/Flutter/React Native/Expo, IoT အတွက် MQTT/Paho နှင့် ESP-IDF/Zephyr စသည့် adapter boundary များကို သီးခြား package အဖြစ် ထားရမည်။

Web contract ကို [English Web guide](../docs/WEB_FRAMEWORK_EN.md) နှင့် [Burmese Web guide](../docs/WEB_FRAMEWORK_MM.md) တွင် ဖတ်ပါ။ Host ကို စတင်အသုံးပြုရန် [English zap-host quickstart](../docs/ZAP_HOST_QUICKSTART_EN.md) နှင့် [Burmese zap-host quickstart](../docs/ZAP_HOST_QUICKSTART_MM.md) ကို ကြည့်ပါ။ Framework အကျဉ်းချုပ်ကို [English Framework Guide](../docs/FRAMEWORK_EN.md) နှင့် [Burmese Framework Guide](../docs/FRAMEWORK_MM.md) တွင် ဖတ်ပါ။
