# Zap Framework လမ်းညွှန်

**အတည်ပြုထားသော baseline:** Zap v2.2.7
**Framework branch:** `Framework`
**အခြေအနေ:** Framework Foundation v0.1 — Zap-native Web project scaffold နှင့် contract starter များ၊ full native runtime integration များသည် gate ချထားသော milestone များအဖြစ် ဆက်လက်ရှိနေသေး

## ရည်ရွယ်ချက်

`frameworks/` directory သည် Web, App, IoT နှင့် AI integration များအတွက် domain contract ကို သတ်မှတ်ပေးသော run လို့ရသည့် Zap program အသေးစားများကို ပေးပါသည်။ Starter များကို လက်ရှိ stable Zap syntax ဖြင့် ရေးထားပြီး request/response model၊ application state၊ telemetry record နှင့် bounded validation များကို ပြသပါသည်။ လက်ရှိ interpreter သည် HTTP server၊ native mobile UI runtime၊ MCU firmware runtime သို့မဟုတ် AI provider client ဖြစ်ပြီးသားဟု မဆိုပါ။

Framework layer သည် **Zap-first application boundary** ဖြစ်ပါသည်။ Zap က Web project model၊ routing metadata၊ request/response policy၊ DTO validation၊ service composition၊ model/migration intent၊ authentication policy၊ admin registration နှင့် test workflow ကို ပိုင်ဆိုင်သင့်ပါသည်။ External adapter များသည် socket၊ TLS၊ database driver၊ identity verification၊ process supervision၊ credential နှင့် platform-specific scheduling ကဲ့သို့ OS/provider capability များကိုသာ ပိုင်ဆိုင်ရမည်။ ရည်ရွယ်ချက်မှာ unsupported syntax သို့မဟုတ် ambient global state မဖုံးကွယ်ဘဲ Django ကဲ့သို့ coherent developer experience ရရှိစေရန် ဖြစ်ပါသည်။

> Framework starter တစ်ခုသည် published Zap runtime ဖြင့် run လို့ရခြင်း၊ valid manifest နှင့် lockfile ရှိခြင်း၊ host boundary ကို ရှင်းလင်းစွာရေးထားခြင်းနှင့် invalid input အတွက် negative case ရှိခြင်းတို့ ပြည့်စုံမှ complete ဖြစ်သည်။ Aspirational DSL စာသားများ ပါရုံဖြင့် complete မဖြစ်ပါ။

## Zap-native Web direction

Web roadmap ကို အခြား starter domain များထက် ဦးစားပေးထားပါသည်။ Web project အသစ်ကို `zap new <dir>` ဖြင့် ဖန်တီးပြီး `zap check`၊ `zap web check`၊ `zap db check`၊ `zap db plan` နှင့် `zap test tests` ဖြင့် စစ်နိုင်ပါသည်။ Generated project သည် routes၊ models၊ services၊ middleware၊ migrations၊ admin registration နှင့် tests များအတွက် ရိုးရိုး Zap module များကို အသုံးပြုပါသည်။ `zap db migrate --dry-run` သည် read-only SQLite plan ကို ပြပြီး `zap db migrate` သည် additive migration များကို transaction အတွင်း apply လုပ်ကာ checksum ledger မှတ်တမ်းတင်ပါသည်။ `zap dev` သည် manifest ထဲက native server ကို run ပါသည်။ Nested test များ၏ import ကိုလည်း `zap.toml` ရှိသော အနီးဆုံး project root မှ resolve လုပ်ပေးပါသည်။

ဤအရာသည် Django ကဲ့သို့ framework အတွက် ပထမအဆင့် ဖြစ်ပါသည်။ လက်ရှိ parser တွင် first-class route/model declaration၊ concurrent production server၊ SQLite-first adapter ထက်ကျော်သော provider-neutral database driver၊ session system သို့မဟုတ် built-in admin UI မရှိသေးပါ။ ထို feature များအတွက် explicit language/runtime contract နှင့် security test လိုအပ်သဖြင့် scaffold က ရှိပြီးသားဟု မဆိုရပါ။

ယခင် `host/zap-host` package သည် လက်ရှိ contract layer အတွက် reference HTTP adapter အဖြစ် ဆက်ရှိနေပါမည်။ ၎င်းသည် primary application model မဟုတ်ပါ။ Zap-native server capability တိုးလာသည်နှင့် adapter သည် platform boundary အဖြစ် သေးသွားပြီး route၊ DTO၊ auth၊ migration၊ admin နှင့် application policy များကို Zap ထဲတွင် ရေးသားသင့်ပါသည်။

## Self-contained runtime နှင့် browser boundary

ရည်ရွယ်သော developer experience သည် installed Zap runtime တစ်ခုရှိရုံဖြင့် Python၊ Node.js၊ Rust၊ Java သို့မဟုတ် အခြား language runtime ကို deployment host တွင် ထပ်မလိုဘဲ Zap project ကို validate၊ build၊ test နှင့် run လုပ်နိုင်ခြင်း ဖြစ်ပါသည်။ Native Zap executable သည် execution boundary ဖြစ်ပြီး Rust သည် source/build implementation detail သာဖြစ်ကာ project user အတွက် prerequisite မဟုတ်ပါ။ ထို့ကြောင့် distribution တစ်ခုသည် support လုပ်သော operating system တစ်ခုချင်းစီအတွက် pinned Zap binary သို့မဟုတ် installer ကို ပေးရမည်။ Server execution ကို ဒုတိယ language runtime ထံ မသိမသာ လွှဲမပေးရပါ။

Browser code သည် server dependency မဟုတ်ဘဲ interoperability boundary ဖြစ်ပါသည်။ HTML၊ CSS နှင့် JavaScript သည် သတ်မှတ်ထားသော `public` asset root အောက်ရှိ ရိုးရိုး file များဖြစ်ပါသည်။ `frontend_contract.zp` နှင့် `web_static` က browser asset အတွက် confined၊ typed response boundary ပေးပြီး `/api/tasks` ကဲ့သို့ route များက JSON data ပေးပါသည်။ React၊ Vue၊ Svelte၊ Alpine သို့မဟုတ် အခြား JavaScript framework ကို build-time တွင် optional အဖြစ် သုံးနိုင်ပြီး ထွက်လာသော file များကို Zap က serve လုပ်ပါသည်။ Runtime တွင် Node မလိုပါ။ Zap သည် အဆိုပါ JavaScript framework များကို implement လုပ်သည် သို့မဟုတ် ၎င်းတို့၏ build tool ကို အစားထိုးသည်ဟု မဆိုပါ။

ဤ policy ကြောင့် framework သည် Zap-first ဖြစ်နေသော်လည်း Web ecosystem နှင့် မခွဲထွက်ပါ။ Zap သည် project structure၊ route၊ validation၊ application contract နှင့် server-side execution ကို ပိုင်ဆိုင်ပြီး browser framework က project ရွေးချယ်ပါက client rendering ကို ပိုင်ဆိုင်ပါသည်။ Integration သည် file၊ HTTP route နှင့် JSON DTO များမှတစ်ဆင့် explicit ဖြစ်ပါသည်။

## လက်ရှိ starter matrix

| Starter | လက်ရှိ deliverable | ထပ်လိုအပ်မည့် host integration | Production အခြေအနေ |
|---|---|---|---|
| `frameworks/web` | Deterministic route/request/response contract၊ Zap-native loopback dev server၊ bounded HTML/CSS/JS asset၊ JSON API boundary နှင့် SQLite-first migration path | TLS၊ concurrent production listener၊ full middleware pipeline၊ provider-neutral database driver၊ binary asset delivery၊ deployment supervision | Development/reference slice |
| `frameworks/mobile` | Portable app model, screen နှင့် action contract | Tauri, Flutter, React Native/Expo သို့မဟုတ် native shell | Contract prototype |
| `frameworks/iot` | Bounded sensor event နှင့် device-state contract | MQTT/Paho, gateway transport, ESP-IDF, Zephyr သို့မဟုတ် Embassy host | Contract prototype |
| `frameworks/ai` | Prompt/response boundary example | Provider SDK, local model, credential နှင့် quota adapter | Contract prototype |

Starter များသည် parser syntax အသစ် မထည့်ပါ။ Function၊ map၊ list၊ loop၊ `assert` နှင့် `json()` ကို အသုံးပြုသဖြင့် ယနေ့ `zap main.zp` ဖြင့် run နိုင်ပါသည်။

## Quick start

Repository root မှ starter တစ်ခုချင်းစီကို အောက်ပါအတိုင်း run ပါ။

```bash
cd frameworks/web
zap lock
zap check
zap run main.zp
```

`frameworks/mobile`, `frameworks/iot` နှင့် `frameworks/ai` အတွက်လည်း အတူတူ command ကို သုံးနိုင်ပါသည်။ `zap.lock` သည် generated output ဖြစ်သော်လည်း project နှင့်အတူ commit လုပ်သင့်ပါသည်။ နောက်ပိုင်း adapter dependency ထည့်ပါက lockfile ပြန်ထုတ်ပြီး `zap install --locked` ဖြင့် စစ်ဆေးပါ။

Starter program များသည် deterministic JSON သို့မဟုတ် text output ပြီးနောက် socket၊ native window၊ device connection သို့မဟုတ် external model session မဖွင့်ဘဲ ပြီးဆုံးပါသည်။ ထို့ကြောင့် CI smoke test နှင့် learning example အတွက် သင့်တော်ပါသည်။

## Package boundaries

Framework layer ၏ dependency direction သည် အောက်ပါအတိုင်း ဖြစ်ရမည်။

```text
Zap source နှင့် domain contract
          ↓
framework starter package
          ↓
zap-host capability နှင့် DTO boundary
          ↓
platform adapter
          ↓
OS, network, native UI, device SDK သို့မဟုတ် provider
```

Starter တစ်ခုသည် undeclared provider package ကို import မလုပ်ရ၊ credential ကို မသိမသာ မဖတ်ရ၊ unrestricted process/socket မဖွင့်ရ၊ operating system တစ်ခုတည်းကို မယူဆရပါ။ Platform behavior သည် သီးခြား versioned adapter package ထဲတွင်သာ ရှိရမည်။

| Layer | ပိုင်ဆိုင်ရမည့်အရာ | မပိုင်ဆိုင်ရမည့်အရာ |
|---|---|---|
| Zap core | Parsing, evaluation, diagnostics, deterministic values | HTTP, mobile renderer, MCU driver |
| Framework contract | Domain record, validation, route/state/telemetry policy | OS handle, credential, native thread |
| `zap-host` boundary | Capability name, typed DTO, limit, error, trace | Hidden global state, unrestricted authority |
| Platform adapter | HTTP/TLS, native UI lifecycle, MQTT, board SDK, provider API | RFC မရှိဘဲ language semantic အသစ် |
| Deployment | Identity, sandbox, egress, quota, supervision, secret | Runtime limit = OS isolation ဟူသောယူဆချက် |

## Web starter

`frameworks/web/web_contract.zp` သည် reusable Web contract module ဖြစ်ပြီး `normalize_request`, `security_headers`, `response` နှင့် `route` functions များကို export လုပ်ထားပါသည်။ Request contract သည် `GET`/`POST` ကို normalize လုပ်၊ traversal ပုံစံ path များကို reject လုပ်၊ path ကို 2,048 bytes၊ body ကို 65,536 bytes အထိ bounded ထားပြီး request ID ကို 1–128 bytes လိုအပ်စေပါသည်။ Response contract တွင် `status`, `content_type`, `headers`, `body` fields များ ပါဝင်ပါသည်။

`frameworks/web/main.zp` သည် root၊ health၊ echo၊ not-found၊ traversal-rejection နှင့် unsupported-method cases များကို ပြသပါသည်။ Web API layer တွင် reusable `api_contract.zp`, `dto_contract.zp`, `database_contract.zp`, `auth_contract.zp`, `rate_limit_contract.zp` modules များ ပါဝင်ပြီး `api_contract_test.zp` သည် 200/201/400/401/403/404/429 behavior၊ DTO mapping၊ quota transition နှင့် policy failure များကို cover လုပ်ပါသည်။ Schema၊ threat control၊ database boundary၊ authentication policy၊ rate-limit semantics၊ adapter pipeline နှင့် Web-specific definition of done အသေးစိတ်ကို [`WEB_FRAMEWORK_MM.md`](WEB_FRAMEWORK_MM.md) တွင် ဖတ်ရှုပါ။

Web starter နှင့် Zap-native scaffold တို့က request၊ response၊ DTO၊ database၊ authentication၊ rate-limit၊ migration၊ admin နှင့် browser asset contract များကို သတ်မှတ်ပေးပါသည်။ လက်ရှိ `host/zap-host` package သည် ဤ contract များကို operational Axum/Tower boundary သို့ ပြောင်းပေးနိုင်သော်လည်း adapter အဖြစ်သာ ရှိရမည်။ Long-term direction တွင် native Zap Web runtime က project lifecycle ကို ပိုင်ဆိုင်ပြီး provider-neutral capability interface များမှတစ်ဆင့် external service များကို ခေါ်ရမည်။ လက်ရှိ `web_static` slice သည် bounded UTF-8 text asset များကိုသာ serve လုပ်နိုင်ပြီး binary media၊ cache manifest၊ server-side rendering နှင့် production asset fingerprinting များသည် သီးခြားအလုပ်များ ဖြစ်နေသေးပါသည်။

Production native server တစ်ခုတွင် method/path normalization၊ maximum header/body bytes၊ timeout၊ cancellation၊ error mapping၊ log redaction၊ connection shutdown၊ readiness နှင့် backpressure ကို သတ်မှတ်ရမည်။ Production claim မပြုမီ ထို runtime responsibility များအတွက် သီးခြား implementation နှင့် test gate များ လိုအပ်ပါသည်။

## App starter

`frameworks/mobile/main.zp` သည် application name၊ initial route၊ screen နှင့် action ပါသော app manifest ကို model လုပ်ပါသည်။ Native shell တစ်ခုရွေးချယ်ခြင်းမပြုမီ navigation နှင့် action policy ကို data အဖြစ် သတ်မှတ်နိုင်ကြောင်း ပြပါသည်။

ပထမဆုံး App implementation သည် custom renderer မရေးဘဲ shell တစ်ခုကို generate/consume လုပ်သင့်ပါသည်။ Rust/native-web shell အတွက် [Tauri](https://v2.tauri.app/), widget-based multiplatform UI အတွက် [Flutter](https://docs.flutter.dev/), JavaScript/native ecosystem လိုအပ်ပါက [React Native with Expo](https://reactnative.dev/docs/environment-setup) တို့ကို host option အဖြစ် အသုံးပြုနိုင်ပါသည်။ Zap contract သည် renderer ရွေးချယ်မှုနှင့် မချိတ်ထားသင့်ပါ။

App adapter တွင် lifecycle events၊ foreground/background behavior၊ offline storage၊ IPC authentication၊ permission prompt၊ deep link၊ update/rollback နှင့် crash reporting တို့ကို သတ်မှတ်ရမည်။ Screen map တစ်ခုရှိရုံဖြင့် mobile runtime မဖြစ်ပါ။

## IoT starter

`frameworks/iot/main.zp` သည် device identity၊ bounded sensor sample၊ accepted-reading count နှင့် device state record ကို model လုပ်ပါသည်။ GPIO၊ serial၊ Bluetooth၊ Wi-Fi သို့မဟုတ် real broker ကို မထိဘဲ reading များကို simulate လုပ်ပါသည်။

ပထမဆုံး IoT implementation ကို Linux/SBC gateway သို့မဟုတ် host process အဖြစ် စတင်ရန် အကြံပြုပါသည်။ MQTT အတွက် [Eclipse Paho](https://eclipse.dev/paho/) ကဲ့သို့ established client ကို သုံးနိုင်ပြီး topic policy၊ payload size၊ QoS၊ retained message၊ reconnect၊ duplicate handling နှင့် offline replay ကို ရှင်းလင်းစွာ သတ်မှတ်ရမည်။ Firmware အတွက် Zap သည် [ESP-IDF](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/) သို့မဟုတ် [Zephyr](https://docs.zephyrproject.org/latest/) component နှင့် binding ထုတ်ခြင်း သို့မဟုတ် ဆက်သွယ်ခြင်းကို ဦးစားပေးသင့်ပါသည်။ MCU သေးသေးပေါ်တွင် Zap interpreter အပြည့်ထည့်ခြင်းသည် interrupt၊ DMA၊ `no_std`၊ flash/RAM budget၊ watchdog နှင့် board support လိုအပ်ချက်များကြောင့် ယခု scope အပြင်တွင် ရှိပါသည်။ [Embassy](https://embassy.dev/book/) ကို embedded constraint reference အဖြစ် သုံးနိုင်ပါသည်။

IoT adapter သည် malformed/oversized telemetry ကို reject လုပ်ရမည်၊ device identity authenticate လုပ်ရမည်၊ command များကို idempotent ဖြစ်စေရမည်၊ correlation identifier ထည့်ရမည်၊ reconnect ကို safe ဖြစ်စေရမည်၊ reset/brownout ပြီးနောက် behavior ကို သတ်မှတ်ရမည်။ Simulated loop အောင်မြင်ခြင်းသည် hardware-in-the-loop evidence မဟုတ်ပါ။

## AI starter

AI starter သည် contract example သာဖြစ်ပါသည်။ Prompt နှင့် response record ကို model လုပ်ပြီး provider သို့ မဆက်သွယ်ပါ၊ credential မသိမ်းပါ။ နောက်ပိုင်း provider adapter တွင် model selection၊ timeout၊ request/response size၊ retry၊ quota error၊ prompt/response redaction နှင့် audit retention ကို သတ်မှတ်ရမည်။

## Capability နှင့် security contract

Real adapter တစ်ခုချင်းစီသည် ambient access မသုံးဘဲ explicit host capability object ရရှိရမည်။ အနည်းဆုံး အောက်ပါ field များ ပါဝင်သင့်ပါသည်။

| Field | လိုအပ်ချက် |
|---|---|
| `capability` | `web.listen`, `iot.publish`, `app.storage.read` ကဲ့သို့ stable name |
| `identity` | Authenticated caller/device/app identity; string field တစ်ခုတည်းမှ မယူဆရ |
| `limits` | Input, output, task, timeout, queue, payload bounds |
| `deadline` | Monotonic deadline သို့မဟုတ် documented poll budget |
| `cancellation` | Cooperative cancellation နှင့် resource-close behavior |
| `idempotency_key` | Retry ဖြစ်နိုင်သော command အတွက် လိုအပ် |
| `trace_id` | Domain result, host operation နှင့် audit record ကို ချိတ်ရန် |
| `redaction` | Secret/token/password field များ diagnostics ထဲ မပါစေရ |
| `error` | Unstructured provider string မဟုတ်ဘဲ stable typed category |
| `replay_class` | Pure, input-deterministic, runtime-dependent, external I/O ခွဲခြားချက် |

Host သည် default deny ဖြစ်ရမည်။ Denied capability သည် external side effect မဖြစ်မီ deterministic typed error ပြန်ပေးရမည်။ Runtime logical budget သည် OS sandbox၊ network egress policy၊ process identity သို့မဟုတ် secret manager ကို အစားမထိုးပါ။

## Testing နှင့် acceptance

Framework starter တစ်ခုချင်းစီတွင် evidence လေးမျိုး ရှိရမည်။

1. **Executable smoke:** Clean directory နှင့် committed lockfile ဖြင့် `zap check`, `zap build`, `zap run main.zp` အောင်မြင်ရမည်။
2. **Contract assertions:** Valid output shape၊ deterministic ordering နှင့် representative edge/error behavior ကို Zap source သို့မဟုတ် host tests ထဲတွင် assert လုပ်ရမည်။
3. **Negative security cases:** Oversized input၊ unsupported capability၊ malformed route/topic/action နှင့် missing identity များကို side effect မဖြစ်မီ reject လုပ်ရမည်။
4. **Adapter parity:** Fixture တစ်ခုတည်းအတွက် fake host နှင့် real host သည် normalized domain result တူရမည်။ External error များသည် typed နှင့် traceable ဖြစ်ရမည်။

CI gate သည် undeclared dependency၊ missing lockfile၊ unresolved placeholder import၊ unsupported aspirational syntax သို့မဟုတ် contract prototype ကို production runtime ဟု ခေါ်သော documentation claim ရှိပါက fail ဖြစ်ရမည်။

## v0.1 တွင် မပါသေးသောအရာများ

Framework branch တွင် Zap-native Web project scaffold နှင့် CLI validation command များ ပါဝင်လာပြီဖြစ်သော်လည်း persistent native HTTP server၊ custom mobile renderer၊ MCU interpreter၊ MQTT client၊ OTA manager၊ cloud deployment command၊ real ORM/database driver၊ session store၊ built-in admin UI သို့မဟုတ် provider-specific AI client မပါသေးပါ။ ထို feature များသည် aspirational syntax မဟုတ်ဘဲ explicit contract နှင့် security evidence လိုအပ်ပါသည်။

`zap-host` adapter ကို `host/zap-host` အောက်တွင် operational reference boundary အဖြစ် ဆက်အသုံးပြုနိုင်ပါသည်။ ၎င်းတွင် Axum/Tower HTTP handling၊ capability-facing trait များ၊ typed DTO mapping၊ bounded request/response handling၊ deterministic test၊ structured error၊ sensitive-header redaction နှင့် graceful shutdown ပါဝင်ပါသည်။ အသေးစိတ် setup ကို [`ZAP_HOST_MM.md`](ZAP_HOST_MM.md) နှင့် Zap-first project workflow ကို [`ZAP_WEB_NATIVE_MM.md`](ZAP_WEB_NATIVE_MM.md) တွင် ဖတ်နိုင်ပါသည်။ Real native runtime embedding၊ database/authentication provider၊ shared quota storage၊ TLS နှင့် deployment-specific evidence များသည် နောက်ဆက်တွဲအလုပ်များ ဖြစ်နေသေးပါသည်။

## Framework Foundation v0.1 Definition of Done

Starter လေးခုလုံးတွင် valid manifest နှင့် lockfile ရှိရမည်၊ current Zap syntax သာ သုံးရမည်၊ clean smoke validation pass ဖြစ်ရမည်၊ non-production boundary ကို ရှင်းလင်းစွာ ဖော်ပြရမည်၊ deterministic domain record များ ရှိရမည်၊ secret/unrestricted host access မရှိရမည်၊ bilingual documentation navigation မှ link ချိတ်ထားရမည်။ Real platform adapter များသည် သီးခြား milestone များဖြစ်ပြီး starter directory တစ်ခုတည်းကြောင့် ရှိပြီးသားဟု မယူဆရပါ။

## ကိုးကားချက်များ

[1]: https://docs.rs/axum/latest/axum/ — Axum HTTP routing နှင့် request handling documentation
[2]: https://v2.tauri.app/ — Tauri desktop/mobile application shell documentation
[3]: https://docs.flutter.dev/ — Flutter multiplatform UI toolkit documentation
[4]: https://reactnative.dev/docs/environment-setup — React Native နှင့် Expo environment guidance
[5]: https://eclipse.dev/paho/ — Eclipse Paho MQTT client project
[6]: https://docs.zephyrproject.org/latest/ — Zephyr RTOS နှင့် embedded platform documentation
[7]: https://docs.espressif.com/projects/esp-idf/en/latest/esp32/ — Espressif ESP-IDF documentation
[8]: https://embassy.dev/book/ — Embassy embedded async framework documentation
[9]: https://github.com/hidecard/zap/blob/master/docs/ASYNC_BOUNDARIES_MM.md — Zap async boundary contract
[10]: https://github.com/hidecard/zap/blob/master/SECURITY.md — Zap security policy နှင့် untrusted execution boundary
