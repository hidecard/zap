# Zap Framework Foundation

ဤ directory သည် Web, App, IoT နှင့် AI အတွက် **run လို့ရသော contract starter များ** ပါဝင်သော Framework Foundation ဖြစ်ပါသည်။ Web ကို ယခု Framework ၏ ဦးစားပေး direction အဖြစ် Zap ကိုယ်တိုင်ပေါ်တွင် project/app structure၊ route metadata၊ models၊ services၊ middleware၊ migrations၊ admin registration နှင့် tests များ တည်ဆောက်နိုင်ရန် ပြင်ဆင်ထားပါသည်။ Starter များသည် current Zap v2.2.3 syntax ဖြင့်ရေးထားပြီး domain model၊ validation နှင့် deterministic output ကို ပြသပါသည်။ Zap-native loopback development server နှင့် SQLite-first database adapter/migration workflow ကို စတင်ထားပြီး production concurrency၊ identity provider နှင့် external platform runtime များသည် explicit capability နှင့် security gates မပြည့်မီ မဖွင့်သေးပါ။

```text
frameworks/
  web/       route/request/response contract
  mobile/    app-state/navigation contract
  ai/        provider request/response contract
  iot/       telemetry/device-state contract
```

## Create a Zap-native Web project

Django-like Web workflow ကို စတင်ရန် repository root မှ အောက်ပါအတိုင်း run ပါ။

```bash
zap new shop
cd shop
zap check
zap web check
zap db check
zap db plan
zap db migrate --dry-run
zap db migrate
zap test tests
zap dev
```

`zap new` သည် `zap.toml`၊ `main.zp`၊ `server.zp`၊ `routes.zp`၊ `models/`၊ `services/`၊ `views/`၊ `public/`၊ `migrations/`၊ `middleware.zp`၊ `admin.zp` နှင့် nested `tests/` ကို ဖန်တီးပေးပါသည်။ `zap web check` က `[web]` manifest path များကို စစ်ပြီး `zap db check` က structured migration declaration နှင့် deterministic SQL plan ကို စစ်ပါသည်။ `zap db plan` သည် pending SQL ကို read-only ပြပြီး `zap db migrate --dry-run` သည် apply မလုပ်ဘဲ ထို plan ကို ပြပါသည်။ `zap db migrate` သည် SQLite database ကို transaction အတွင်း apply လုပ်ကာ checksum ledger မှတ်တမ်းတင်ပါသည်။ `zap dev` သည် bounded loopback development/reference server ကို run ပါသည်။ အသေးစိတ်ကို [Burmese Zap-first Web guide](../docs/ZAP_WEB_NATIVE_MM.md) နှင့် [English Zap-first Web guide](../docs/ZAP_WEB_NATIVE_EN.md) တွင် ဖတ်နိုင်ပါသည်။

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

Framework syntax အသစ်ကို language core ထဲသို့ အလျင်မထည့်ရပါ။ Real Web/App/IoT integration မစခင် `docs/FRAMEWORK_EN.md` နှင့် `docs/FRAMEWORK_MM.md` ထဲက capability, error, limit, replay နှင့် security contract များကို အရင်အတည်ပြုရမည်။ Zap-first Web အတွက် လက်ရှိ project scaffold နှင့် contract validation ကို ဦးစားပေးပြီး persistent native server၊ ORM၊ session/admin UI နှင့် provider integration များကို executable contract မရှိဘဲ မဆိုရပါ။

Web contract ကို [English Web guide](../docs/WEB_FRAMEWORK_EN.md) နှင့် [Burmese Web guide](../docs/WEB_FRAMEWORK_MM.md) တွင် ဖတ်ပါ။ Zap ကိုယ်တိုင်အပေါ် Web project တည်ဆောက်ပုံကို [English Zap-first Web guide](../docs/ZAP_WEB_NATIVE_EN.md) နှင့် [Burmese Zap-first Web guide](../docs/ZAP_WEB_NATIVE_MM.md) တွင် ဖတ်ပါ။ လက်ရှိ operational reference host ကို စတင်အသုံးပြုရန် [English zap-host quickstart](../docs/ZAP_HOST_QUICKSTART_EN.md) နှင့် [Burmese zap-host quickstart](../docs/ZAP_HOST_QUICKSTART_MM.md) ကို ကြည့်ပါ။ Framework အကျဉ်းချုပ်ကို [English Framework Guide](../docs/FRAMEWORK_EN.md) နှင့် [Burmese Framework Guide](../docs/FRAMEWORK_MM.md) တွင် ဖတ်ပါ။
