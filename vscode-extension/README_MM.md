# Zap VS Code Extension

ဤ extension သည် Zap ၏ `.zp` source file များကို VS Code တွင် တိုက်ရိုက်ရေးသားနိုင်ရန် အထောက်အကူပြုပါသည်။ Zap CLI ကို အသုံးပြုပြီး syntax highlighting၊ autocomplete၊ diagnostics နှင့် run command များကို ပေါင်းစပ်ထားပါသည်။

## ပါဝင်သောအင်္ဂါရပ်များ

| အင်္ဂါရပ် | ရှင်းလင်းချက် |
|---|---|
| `.zp` support | `.zp` file များကို Zap language အဖြစ် အလိုအလျောက်သိရှိခြင်း |
| Syntax highlighting | keyword၊ function၊ type၊ string၊ number၊ comment နှင့် builtin များကို highlight ပြခြင်း |
| Autocomplete | Zap LSP မှ workspace-aware symbols များနှင့် keyword၊ type၊ builtin များကို အကြံပြုခြင်း |
| Signature help | Function call ရေးနေစဉ် parameter label နှင့် လက်ရှိ active argument ကို ပြခြင်း |
| Formatting | LSP မှတစ်ဆင့် newline ကို normalize လုပ်ခြင်း၊ tab ကို space လေးခုအဖြစ် ပြောင်းခြင်းနှင့် trailing whitespace ဖြုတ်ခြင်း |
| Hover | function၊ class၊ module၊ import နှင့် binding အချက်အလက်များကို mouse hover ဖြင့် ပြခြင်း |
| Go to definition | Zap LSP မှ top-level declaration များသို့ သွားရောက်နိုင်ခြင်း |
| Workspace symbols | VS Code symbol search မှ Zap declaration များကို ရှာဖွေနိုင်ခြင်း |
| Snippets | function၊ loop၊ condition၊ `try/catch`၊ import၊ `main` နှင့် `raise` snippets များ |
| Error diagnostics | Zap LSP `publishDiagnostics` နှင့် CLI fallback ရလဒ်များကို Problems panel တွင် ပြခြင်း |
| Run | လက်ရှိ `.zp` file ကို integrated terminal တွင် `zap run` ဖြင့် run ခြင်း |
| Workspace check | Zap project တစ်ခုလုံးကို command palette မှ စစ်ဆေးခြင်း |

## အသုံးပြုရန်

Zap CLI ကို `PATH` ထဲတွင် ထည့်ထားပါ သို့မဟုတ် VS Code Settings တွင် `zap.executable` ကို Zap executable လမ်းကြောင်းအဖြစ် သတ်မှတ်ပါ။ Extension သည် `zap lsp` ကို stdio JSON-RPC server အဖြစ် စတင်ပြီး ဖွင့်ထားသော `.zp` document များကို server နှင့် synchronize လုပ်ပါသည်။ ထို့နောက် `vscode-extension` folder ကို VS Code ဖြင့် ဖွင့်ပြီး **Developer: Install Extension from Location...** ကို ရွေးချယ်ပါ။

## Commands

Command Palette မှ **Zap: Run Current File**၊ **Zap: Check Workspace** နှင့် **Zap: Restart Diagnostics** တို့ကို အသုံးပြုနိုင်ပါသည်။ `.zp` editor အတွင်းတွင် play button နှင့် right-click context menu entry ကိုလည်း ထည့်သွင်းထားပါသည်။

## Settings

```json
{
  "zap.executable": "zap",
  "zap.enableLsp": true,
  "zap.enableDiagnostics": true,
  "zap.diagnosticDelay": 350,
  "zap.runInTerminal": true
}
```

LSP client သည် `Content-Length` JSON-RPC framing ကို အသုံးပြုပြီး initialize၊ document open/change/close၊ completion၊ signature help၊ hover၊ definition၊ document formatting၊ workspace symbols နှင့် publish-diagnostics notification များကို ထောက်ပံ့ပါသည်။ Signature help သည် `(` နှင့် `,` ရိုက်ပြီးနောက် အလုပ်လုပ်ပြီး document formatting ကို VS Code ၏ **Format Document** မှ အသုံးပြုနိုင်ပါသည်။ Extension သည် Zap parser ကို သီးခြားပြန်ရေးမထားဘဲ native LSP နှင့် CLI diagnostic boundary များကို အသုံးပြုသောကြောင့် command line နှင့် editor diagnostics များ တူညီစွာ အလုပ်လုပ်ပါသည်။
