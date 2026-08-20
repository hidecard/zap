# Zap VS Code Extension

ဤ extension သည် Zap ၏ `.zp` source file များကို VS Code တွင် တိုက်ရိုက်ရေးသားနိုင်ရန် အထောက်အကူပြုပါသည်။ Zap CLI ကို အသုံးပြုပြီး syntax highlighting၊ autocomplete၊ diagnostics နှင့် run command များကို ပေါင်းစပ်ထားပါသည်။

## ပါဝင်သောအင်္ဂါရပ်များ

| အင်္ဂါရပ် | ရှင်းလင်းချက် |
|---|---|
| `.zp` support | `.zp` file များကို Zap language အဖြစ် အလိုအလျောက်သိရှိခြင်း |
| Syntax highlighting | keyword၊ function၊ type၊ string၊ number၊ comment နှင့် builtin များကို highlight ပြခြင်း |
| Autocomplete | Zap keyword၊ type နှင့် standard-library builtin များကို အကြံပြုခြင်း |
| Snippets | function၊ loop၊ condition၊ `try/catch`၊ import၊ `main` နှင့် `raise` snippets များ |
| Error diagnostics | `zap check --json` ရလဒ်ကို VS Code Problems panel တွင် ပြခြင်း |
| Run | လက်ရှိ `.zp` file ကို integrated terminal တွင် `zap run` ဖြင့် run ခြင်း |
| Workspace check | Zap project တစ်ခုလုံးကို command palette မှ စစ်ဆေးခြင်း |

## အသုံးပြုရန်

Zap CLI ကို `PATH` ထဲတွင် ထည့်ထားပါ သို့မဟုတ် VS Code Settings တွင် `zap.executable` ကို Zap executable လမ်းကြောင်းအဖြစ် သတ်မှတ်ပါ။ ထို့နောက် `vscode-extension` folder ကို VS Code ဖြင့် ဖွင့်ပြီး **Developer: Install Extension from Location...** ကို ရွေးချယ်ပါ။

## Commands

Command Palette မှ **Zap: Run Current File**၊ **Zap: Check Workspace** နှင့် **Zap: Restart Diagnostics** တို့ကို အသုံးပြုနိုင်ပါသည်။ `.zp` editor အတွင်းတွင် play button နှင့် right-click context menu entry ကိုလည်း ထည့်သွင်းထားပါသည်။

## Settings

```json
{
  "zap.executable": "zap",
  "zap.enableDiagnostics": true,
  "zap.diagnosticDelay": 350,
  "zap.runInTerminal": true
}
```

Extension သည် Zap parser ကို သီးခြားပြန်ရေးမထားဘဲ CLI ၏ stable JSON diagnostic boundary ကို အသုံးပြုသောကြောင့် command line နှင့် editor diagnostics များ အတူတကွ တူညီစွာ အလုပ်လုပ်ပါသည်။
