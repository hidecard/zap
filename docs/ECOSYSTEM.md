# Zap Ecosystem Architecture

Zap သည် language core တစ်ခုအပေါ်တွင် domain-specific packages နှင့် platform runtimes များကို တည်ဆောက်မည့် ecosystem ဖြစ်သည်။ v2.2.3 အပြီးတွင် Framework Foundation v0.1 သည် Web၊ App၊ AI နှင့် IoT အတွက် current Zap syntax ဖြင့် run လို့ရသော contract starter များကို ပေးထားပြီ။ Web အတွက် Zap-native loopback server နှင့် SQLite-first migration adapter ကို စတင်ထားပြီး production-grade platform adapters များကို သီးခြား capability boundary၊ security policy နှင့် deployment evidence ဖြင့် ဆက်တိုးမည်။

## Layers

| Layer | တာဝန် | အခြေအနေ |
|---|---|---|
| Zap Core | syntax၊ parser၊ values၊ functions၊ modules၊ runtime | လက်ရှိတည်ဆောက်နေသည် |
| Standard Library | collections၊ JSON၊ file၊ time၊ networking၊ process နှင့် testing APIs | တစ်စိတ်တစ်ပိုင်းရှိသည် |
| Package Tooling | `zap.toml`၊ local modules၊ dependencies၊ lockfile၊ registry | manifest၊ deterministic lockfile၊ nested local graph/cycle validation နှင့် registry-ready metadata ရှိသည်; remote registry roadmap |
| Domain Frameworks | Web၊ App/Mobile၊ AI နှင့် IoT contract starters | Web native runtime/asset/API slice implemented; production adapters deferred |
| Platform Runtimes | native OS၊ browser/WASM၊ Android၊ GPU နှင့် microcontroller | Host adapters and target runtimes are roadmap work |

## Future frameworks

### Zap Web

Zap Web starter သည် routing/request/response contract ကို deterministic အဖြစ် သတ်မှတ်ထားပြီး Zap-native loopback development server၊ confined UTF-8 HTML/CSS/JavaScript asset serving၊ browser JSON API boundary၊ SQLite-first database adapter၊ structured migration workflow နှင့် provider-neutral parameterized query/DTO contract ကို Framework branch တွင် စတင်ပေးထားသည်။ Zap executable နှင့် project asset files သာဖြင့် runtime ကို run နိုင်ရန် ရည်ရွယ်ပြီး Node.js/အခြား language runtime များသည် optional build-time tool များသာ ဖြစ်သည်။ TLS၊ production concurrency၊ middleware execution၊ binary asset streaming၊ WebSocket၊ PostgreSQL/MySQL adapters နှင့် production database operations များသည် နောက်အဆင့်အလုပ်များ ဖြစ်သည်။ `zap-host` သည် application framework မဟုတ်ဘဲ operational Axum/Tower platform adapter boundary အဖြစ် ဆက်ရှိသည်။

### Zap Mobile

Zap App starter သည် app-state/navigation contract ကို deterministic အဖြစ် သတ်မှတ်ထားသည်။ Native renderer၊ lifecycle၊ permissions၊ storage၊ notifications နှင့် bridge APIs များကို Tauri၊ Flutter သို့မဟုတ် React Native/Expo shell များအပေါ် သီးခြား adapter အဖြစ် တည်ဆောက်ရမည်။

### Zap AI

Zap AI starter သည် provider request/response contract ကိုသာ model လုပ်သည်။ Provider client၊ structured output၊ embeddings၊ vector store၊ tool calling၊ streaming နှင့် secret/quota handling များသည် provider adapter အဖြစ် သီးခြားတည်ဆောက်ရမည်။ API key ကို source ထဲ မရေးရ။

### Zap IoT

Zap IoT starter သည် bounded telemetry/device-state contract ကို simulate လုပ်သည်။ ပထမဆုံး real target ကို Linux/SBC gateway အဖြစ် စတင်ပြီး MQTT/Paho၊ device identity၊ payload limit၊ reconnect၊ duplicate handling နှင့် offline replay ကို adapter contract အဖြစ် သတ်မှတ်ရမည်။ Firmware အတွက် ESP-IDF/Zephyr/Embassy ကို host ecosystem အဖြစ် အသုံးပြုမည်။ Low-RAM MCU တွင် Zap interpreter အပြည့်ထည့်ခြင်း မစတင်သေးပါ။

## Current commands versus roadmap

လက်ရှိ native CLI တွင် အောက်ပါ commands များကို အသုံးပြုနိုင်သည်။ `zap test` သည် `tests/` အောက်ရှိ subdirectories များအပါအဝင် `*_test.zp` files အားလုံးကို run လုပ်ပြီး `zap init` သည် starter smoke test ပါဝင်သော project ကို ဖန်တီးပေးသည်။ `zap add` သည် local manifest ထဲသို့ dependency ထည့်ပြီး lockfile ကို invalidate လုပ်သည်။ `zap lock` သည် canonical lockfile ကို ပြန်လည် generate လုပ်သည်။

```text
zap main.zp
zap check
zap test
zap init my-app
zap fmt main.zp
zap add package-name 1.0 [project-dir]
zap lock [project-dir]
zap --version
zap --help
zap async-check
zap lsp
zap new shop
zap dev shop
zap db check shop
zap db inspect --json shop
zap db plan shop
zap db migrate --dry-run shop
zap db migrate --check shop
zap db migrate shop
```

Zap Web အတွက် `zap new <dir>` သည် generator command အဖြစ် ရှိပြီး `zap check`, `zap web check`, `zap db check`, `zap db inspect`, `zap db plan`, `zap db migrate --check`, `zap db migrate`, `zap test` နှင့် `zap dev` workflow ကို အသုံးပြုနိုင်သည်။ အခြား Framework starter project များကို repository ၏ `frameworks/` directory မှ copy/clone လုပ်ပြီး `zap lock`, `zap check`, `zap build`, `zap run main.zp` ဖြင့် စစ်နိုင်သည်။ Registry publishing သည် `zap registry publish` command ဖြင့် ရရှိပြီးဖြစ်ပါသည်။

```text
zap new my-app
zap new shop
zap dev shop
zap web check shop
zap db inspect --json shop
zap db plan shop
zap db migrate --dry-run shop
zap db migrate --check shop
zap db migrate shop
zap mobile new android-app
zap ai new assistant
zap iot new sensor-node
zap registry publish
```

P2 package-manager foundation အသေးစိတ်အခြေအနေကို [`P2_PROGRESS.md`](P2_PROGRESS.md) နှင့် [`P2_PROGRESS_MM.md`](P2_PROGRESS_MM.md) တွင် ဖတ်ရှုနိုင်သည်။ လက်ရှိ foundation တွင် deterministic manifest/lockfile၊ local path graph traversal၊ cycle detection၊ registry metadata validation၊ local/HTTPS registry transport၊ content-addressed cache၊ checksum enforcement၊ validated remote publishing၊ signed-index verification၊ deterministic cache pruning၊ authenticated local registry persistence၊ exact နှင့် version-range solving၊ `async fn`/`await` language syntax၊ deterministic single-thread async runtime၊ delay/cancellation၊ task limits၊ poll budgets နှင့် stdio LSP/editor features ပါဝင်သည်။ Framework Foundation v0.1 တွင် executable Web route/response၊ App state/navigation၊ AI provider request/response နှင့် IoT telemetry/device-state starters၊ manifests၊ lockfiles၊ bilingual guide နှင့် starter validator ပါဝင်သည်။ ဆက်လက်တိုးချဲ့နိုင်သော နယ်ပယ်များမှာ `zap-host` capability/DTO contract၊ external asynchronous I/O၊ multi-thread scheduling၊ OS deployment security နှင့် real platform adapters ဖြစ်သည်။ Framework packages များသည် Zap core syntax ကို မပြောင်းရပါ။ Domain APIs များကို module/package အဖြစ် ပေးပြီး platform-specific implementation များကို runtime adapters ဖြင့် ခွဲထားရမည်။

## Recommended implementation order

Framework implementation order သည် ပထမဦးစွာ current syntax ဖြင့် contract starters နှင့် smoke validation ကို တည်ငြိမ်စေရမည်။ ထို့နောက် `zap-host` capability၊ DTO၊ typed errors၊ limits၊ cancellation၊ idempotency၊ tracing၊ redaction နှင့် replay boundary ကို freeze လုပ်မည်။ ပြီးလျှင် Web သို့မဟုတ် Linux/SBC Edge adapter တစ်ခုကို စမ်းသပ်မည်။ App native shell နှင့် IoT firmware bindings များကို host adapter contract နှင့် target-specific tests အောင်မြင်ပြီးမှ တိုးချဲ့မည်။
