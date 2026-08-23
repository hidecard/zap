# Framework IoT Starter

ဤ project သည် current Zap v2.2.6 ဖြင့် run လို့ရသော **telemetry/device-state contract** ဖြစ်သည်။ Sensor reading ကို simulate လုပ်ခြင်းသာ ဖြစ်ပြီး GPIO၊ serial၊ Bluetooth၊ Wi-Fi၊ MQTT၊ OTA သို့မဟုတ် hardware ကို မထိပါ။

## Run

```bash
zap lock
zap check
zap build
zap run main.zp
```

`main.zp` သည် bounded temperature readings နှစ်ခုကို record လုပ်ပြီး accepted count နှင့် device state ကို deterministic JSON ဖြင့် ပြသည်။

## Host adapter boundary

ပထမဆုံး real target ကို Linux/SBC gateway အဖြစ် စတင်ပါ။ MQTT/Paho adapter တွင် device identity၊ topic allowlist၊ payload limit၊ QoS၊ reconnect၊ duplicate command handling၊ offline replay၊ timeout၊ cancellation နှင့် trace ID ကို သတ်မှတ်ပါ။

Firmware အတွက် ESP-IDF၊ Zephyr သို့မဟုတ် Embassy တို့၏ board/HAL/async boundary ကို reuse လုပ်ပါ။ Low-RAM MCU တွင် Zap interpreter အပြည့်ထည့်ခြင်း၊ interrupt/DMA/watchdog/flash budget ကို မသတ်မှတ်ဘဲ direct device runtime ပြောခြင်းတို့ကို မလုပ်ပါနှင့်။ `Framework` branch သည် simulator/contract အဆင့်သာ ဖြစ်သည်။ အသေးစိတ်ကို [`docs/FRAMEWORK_MM.md`](../../docs/FRAMEWORK_MM.md) တွင် ဖတ်ပါ။
