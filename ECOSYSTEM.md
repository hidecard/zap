# Zap Ecosystem Architecture

Zap သည် language core တစ်ခုအပေါ်တွင် domain-specific packages နှင့် platform runtimes များကို တည်ဆောက်မည့် ecosystem ဖြစ်သည်။ လက်ရှိ project ဦးစားပေးသည် **Zap native language core** ဖြစ်ပြီး Web၊ Mobile၊ AI နှင့် IoT frameworks များကို core နှင့် package tooling တည်ငြိမ်ပြီးနောက် စတင်မည်။

## Layers

| Layer | တာဝန် | အခြေအနေ |
|---|---|---|
| Zap Core | syntax၊ parser၊ values၊ functions၊ modules၊ runtime | လက်ရှိတည်ဆောက်နေသည် |
| Standard Library | collections၊ JSON၊ file၊ time၊ networking၊ process နှင့် testing APIs | တစ်စိတ်တစ်ပိုင်းရှိသည် |
| Package Tooling | `zap.toml`၊ local modules၊ dependencies၊ lockfile၊ registry | manifest နှင့် local search paths ရှိသည် |
| Domain Frameworks | Web၊ Android/Mobile၊ AI နှင့် IoT APIs | Roadmap |
| Platform Runtimes | native OS၊ browser/WASM၊ Android၊ GPU နှင့် microcontroller | Roadmap |

## Future frameworks

### Zap Web

Zap Web သည် routing၊ middleware၊ JSON API၊ HTML/template rendering၊ static files၊ WebSocket နှင့် database adapters များကို package အဖြစ် ပေးရန် ရည်ရွယ်သည်။ Browser-side target အတွက် Zap-to-JavaScript သို့မဟုတ် WASM backend ကို နောက်ပိုင်းတွင် လေ့လာမည်။

### Zap Mobile

Zap Mobile သည် Android/iOS UI components၊ navigation၊ state၊ device permissions၊ storage၊ notifications နှင့် native bridge APIs များကို ပေးရန် ရည်ရွယ်သည်။ Android အတွက် Kotlin/Java bridge သို့မဟုတ် WebView/WASM backend၊ iOS အတွက် Swift bridge တို့ကို platform adapter အဖြစ် ခွဲထားမည်။

### Zap AI

Zap AI သည် model client၊ prompt၊ structured output၊ embeddings၊ vector store၊ tool calling၊ streaming နှင့် local model adapters များကို ပေးရန် ရည်ရွယ်သည်။ API keys များကို source code ထဲ မရေးဘဲ environment/config secret အဖြစ်သာ စီမံမည်။

### Zap IoT

Zap IoT သည် GPIO၊ I2C၊ SPI၊ UART၊ sensors၊ actuators၊ MQTT၊ device configuration၊ OTA update နှင့် offline queue APIs များကို ပေးရန် ရည်ရွယ်သည်။ Microcontroller target အတွက် resource-constrained `zap-embedded` runtime profile လိုအပ်မည်။

## Current commands versus roadmap

လက်ရှိ native CLI တွင် အောက်ပါ commands များကို အသုံးပြုနိုင်သည်။

```text
zap main.zp
zap check
zap fmt main.zp
zap --version
zap --help
```

အောက်ပါ commands များသည် roadmap ဖြစ်ပြီး လက်ရှိ release တွင် မပါဝင်သေးပါ။

```text
zap new my-app
zap web new website
zap mobile new android-app
zap ai new assistant
zap iot new sensor-node
zap publish
```

Framework packages များသည် Zap core syntax ကို မပြောင်းရပါ။ Domain APIs များကို module/package အဖြစ် ပေးပြီး platform-specific implementation များကို runtime adapters ဖြင့် ခွဲထားရမည်။

## Recommended implementation order

ပထမဦးစွာ parser၊ runtime values၊ functions၊ lexical scopes၊ module system၊ diagnostics၊ formatter နှင့် package manager ကို တည်ငြိမ်အောင်လုပ်မည်။ ထို့နောက် desktop/server ပေါ်တွင် Zap Web နှင့် Zap AI ကို စတင်မည်။ Mobile နှင့် IoT frameworks များကို platform bridges နှင့် cross-compilation strategy တည်ငြိမ်လာသောအခါ တိုးချဲ့မည်။
