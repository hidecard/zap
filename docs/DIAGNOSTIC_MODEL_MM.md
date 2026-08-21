# Zap Structured Diagnostic Model

Zap သည် CLI project validator နှင့် language server နှစ်ခုလုံးတွင် တူညီသော structured diagnostic contract ကို အသုံးပြုသည်။ Machine-readable error များကို တည်ငြိမ်စေပြီး လူဖတ်ရန် message ကို မလိုအပ်ဘဲ မပြောင်းလဲစေရန် ဒီ contract ကို သတ်မှတ်ထားသည်။

## Fields

| Field | အဓိပ္ပါယ် |
| --- | --- |
| `kind` / `code` | `SyntaxError`၊ `NameError` သို့မဟုတ် `TypeError` ကဲ့သို့ တည်ငြိမ်သော error အမျိုးအစား။ |
| `severity` | လက်ရှိတန်ဖိုးမှာ `error` ဖြစ်ပြီး နောက်ပိုင်း warning နှင့် information diagnostic များအတွက် ကြိုတင်ထားသည်။ |
| `file` | ရရှိနိုင်ပါက diagnostic နှင့်သက်ဆိုင်သော source file။ |
| `line` / `column` | CLI JSON output အတွက် one-based source position။ |
| `message` | ပုံမှန်ပြုလုပ်ထားသော user-facing diagnostic message။ |
| `notes` | ဖြစ်နိုင်သောအကြောင်းရင်းကို ရှင်းပြသော deterministic follow-up အချက်များ။ |
| `help` | ရွေးချယ်နိုင်သော deterministic ပြင်ဆင်ရန်အကြံပြုချက်။ |

CLI JSON mode သည် `notes` ကို array အဖြစ် ထုတ်ပြီး `help` ကို string သို့မဟုတ် `null` အဖြစ် ထုတ်သည်။ LSP diagnostic သည် standard `severity`၊ `source`၊ `code`၊ `range` နှင့် `message` fields များကို ထိန်းသိမ်းပြီး Zap-specific metadata ကို `data` object အတွင်း ထည့်သည်။

## Compatibility rules

Diagnostic code၊ field name၊ severity value နှင့် message normalization တို့သည် tooling contract ၏ အစိတ်အပိုင်းများဖြစ်သည်။ ရှိပြီးသား fields များကို မဖယ်ရှားဘဲ field အသစ်များကိုသာ ထပ်တိုးနိုင်သည်။ လူဖတ်ရန် rendering သည် ပြောင်းလဲနိုင်သော်လည်း CLI JSON နှင့် LSP snapshot များသည် deterministic ဖြစ်ရမည်။ Secret များ သို့မဟုတ် source ထဲတွင် မပါသော environment-specific path များကို မထည့်ရ။

လက်ရှိ type diagnostic များတွင် `Check the expression type and the expected annotation.` note နှင့် `Use a compatible value or update the type annotation.` help text ပါဝင်သည်။ Syntax နှင့် name diagnostics များတွင်လည်း အလားတူ deterministic guidance ပါဝင်သည်။

## Verification

Native test suite တွင် conditional-expression type error အတွက် CLI/LSP parity coverage ပါဝင်ပြီး code၊ range၊ severity၊ notes၊ help နှင့် normalized message အားလုံးကို စစ်ဆေးသည်။ Run command မှာ:

```text
cargo test --manifest-path native/Cargo.toml
```
