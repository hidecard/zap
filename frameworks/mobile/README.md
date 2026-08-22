# Framework App Starter

ဤ project သည် current Zap v2.2.3 ဖြင့် run လို့ရသော **app-state/navigation contract** ဖြစ်သည်။ Native window၊ Android/iOS lifecycle၊ permission prompt၊ IPC သို့မဟုတ် device API မဖွင့်ပါ။

## Run

```bash
zap lock
zap check
zap build
zap run main.zp
```

`main.zp` သည် `Home` နှင့် `Settings` screen များ၊ action များနှင့် route validation ကို model လုပ်သည်။ ရှိသော route သို့ navigation အောင်မြင်ပြီး မရှိသော route သို့ navigation ကို initial route သို့ ပြန်ထားကာ reject လုပ်သည်။

## Host adapter boundary

နောက်ပိုင်း App adapter သည် state/navigation record ကို native shell နှင့် ချိတ်ရမည်။ Lifecycle၊ foreground/background၊ offline storage၊ IPC authentication၊ permission၊ deep link၊ update/rollback နှင့် crash reporting ကို host adapter က ပိုင်ဆိုင်ရမည်။

Custom renderer မရေးဘဲ Tauri၊ Flutter သို့မဟုတ် React Native/Expo shell တစ်ခုကို ရွေးချယ်ရန် အကြံပြုသည်။ `Framework` branch ၏ scope သည် portable contract starter အထိသာ ဖြစ်သည်။ အသေးစိတ်ကို [`docs/FRAMEWORK_MM.md`](../../docs/FRAMEWORK_MM.md) တွင် ဖတ်ပါ။
