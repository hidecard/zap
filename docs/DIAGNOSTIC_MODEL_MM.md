# Zap Structured Diagnostic Model

Zap သည် CLI project validator နှင့် language server နှစ်ခုလုံးတွင် တူညီသော structured diagnostic contract ကို အသုံးပြုသည်။ Machine-readable error များကို တည်ငြိမ်စေပြီး လူဖတ်ရန် message ကို မလိုအပ်ဘဲ မပြောင်းလဲစေရန် ဒီ contract ကို သတ်မှတ်ထားသည်။

## Fields

| Field | အဓိပ္ပါယ် |
| --- | --- |
| `code` | Editor နှင့် CI အတွက် compatibility key ဖြစ်သော `ZAP-TYPE-001` ကဲ့သို့သော stable machine-readable identifier။ |
| `kind` | `SyntaxError`၊ `NameError` သို့မဟုတ် `TypeError` ကဲ့သို့ တည်ငြိမ်သော user-facing error အမျိုးအစား။ |
| `severity` | လက်ရှိတန်ဖိုးမှာ `error` ဖြစ်ပြီး နောက်ပိုင်း warning နှင့် information diagnostic များအတွက် ကြိုတင်ထားသည်။ |
| `file` | ရရှိနိုင်ပါက diagnostic နှင့်သက်ဆိုင်သော source file။ |
| `line` / `column` | CLI JSON output အတွက် one-based source position။ |
| `message` | ပုံမှန်ပြုလုပ်ထားသော user-facing diagnostic message။ |
| `notes` | ဖြစ်နိုင်သောအကြောင်းရင်းကို ရှင်းပြသော deterministic follow-up အချက်များ။ |
| `help` | ရွေးချယ်နိုင်သော deterministic ပြင်ဆင်ရန်အကြံပြုချက်။ |

CLI JSON mode သည် `notes` ကို array အဖြစ် ထုတ်ပြီး `help` ကို string သို့မဟုတ် `null` အဖြစ် ထုတ်သည်။ LSP diagnostic သည် standard `severity`၊ `source`၊ `code`၊ `range` နှင့် `message` fields များကို ထိန်းသိမ်းပြီး Zap-specific metadata ကို `data` object အတွင်း ထည့်သည်။

## Stable code registry

| Code | Kind | အဓိပ္ပာယ် |
| --- | --- | --- |
| `ZAP-SYNTAX-001` | `SyntaxError` | Source syntax သို့မဟုတ် parsing အမှား။ |
| `ZAP-NAME-001` | `NameError` | မသိသော သို့မဟုတ် မသတ်မှတ်ရသေးသော name။ |
| `ZAP-TYPE-001` | `TypeError` | Value သို့မဟုတ် expression type မကိုက်ညီမှု။ |
| `ZAP-VALUE-001` | `ValueError` | မမှန်ကန်သော value သို့မဟုတ် operation။ |
| `ZAP-IO-001` | `IOError` | အထွေထွေ input/output အမှား။ |
| `ZAP-FILE-001` | `FileNotFound` | လိုအပ်သော file မရှိခြင်း။ |
| `ZAP-KEY-001` | `KeyError` | Object သို့မဟုတ် map key မတွေ့ခြင်း။ |
| `ZAP-PERM-001` | `PermissionError` | Operation ကို ခွင့်မပြုခြင်း။ |
| `ZAP-OVERFLOW-001` | `OverflowError` | ကန့်သတ်ထားသော numeric သို့မဟုတ် resource operation overflow ဖြစ်ခြင်း။ |
| `ZAP-RUNTIME-001` | `Error` | Stable uncaught runtime failure။ |
| `ZAP-BORROW-001` | `BorrowError` | Checked object-field သို့မဟုတ် lexical-EnvFrame borrow conflict ဖြစ်သောအခါ panic မဖြစ်ဘဲ runtime error ပြန်ပေးခြင်း။ |
| `ZAP-MEMORY-001` | `MemoryError` | Run-owned logical byte၊ object၊ task၊ output သို့မဟုတ် bounded value-lifecycle limit ကျော်လွန်ခြင်း။ |
| `ZAP-PROJECT-001` | `ProjectError` | Project၊ manifest သို့မဟုတ် dependency validation အမှား။ |

Code များသည် additive compatibility identifier များ ဖြစ်သည်။ နောင် release တွင် diagnostic kind သို့မဟုတ် message ပိုမိုတိကျလာနိုင်သော်လည်း ရှိပြီးသား code ကို မတူညီသော failure category အတွက် တိတ်တဆိတ် ပြန်မသုံးရပါ။

## Compatibility rules

Diagnostic code၊ field name၊ severity value နှင့် message normalization တို့သည် tooling contract ၏ အစိတ်အပိုင်းများဖြစ်သည်။ ရှိပြီးသား fields များကို မဖယ်ရှားဘဲ field အသစ်များကိုသာ ထပ်တိုးနိုင်သည်။ လူဖတ်ရန် rendering သည် ပြောင်းလဲနိုင်သော်လည်း CLI JSON နှင့် LSP snapshot များသည် deterministic ဖြစ်ရမည်။ Secret များ သို့မဟုတ် source ထဲတွင် မပါသော environment-specific path များကို မထည့်ရ။ Canonical equality traversal များကို `max_value_nodes` ဖြင့် ကန့်သတ်ပြီး ယခင်က စစ်ဆေးပြီးသား object pair များကို short-circuit လုပ်သည်။ Callable များကို handle identity ဖြင့် နှိုင်းယှဉ်သောကြောင့် cyclic value များသည် unbounded recursion မဖြစ်စေပါ။

လက်ရှိ type diagnostic များတွင် `Check the expression type and the expected annotation.` note နှင့် `Use a compatible value or update the type annotation.` help text ပါဝင်သည်။ Borrow diagnostic သည် active object-field သို့မဟုတ် lexical-frame access ပြီးဆုံးစေရန် stable guidance ကို အသုံးပြုပြီး competing read သို့မဟုတ် mutation မလုပ်မီ စောင့်စေသည်။ Object-field အတွက် `Avoid reading and mutating the same object fields at the same time.` note နှင့် `Finish the active object-field access before mutating the object.` help text ကို ဆက်လက်အသုံးပြုသည်။ Canonical `==` နှင့် `!=` operation များတွင် object field borrow ဖြစ်နေပါက တူညီသော `ZAP-BORROW-001` boundary ကို ပြန်ပေးသည်။ Memory diagnostic တွင် value၊ task သို့မဟုတ် output admission ကို လျှော့ရန်၊ သို့မဟုတ် retry မလုပ်မီ cyclic object field များကို ရှင်းရန် deterministic guidance ပါဝင်သည်။ Syntax နှင့် name diagnostics များတွင်လည်း အလားတူ deterministic guidance ပါဝင်သည်။

## Verification

Native test suite တွင် conditional-expression type error အတွက် CLI/LSP parity coverage ပါဝင်ပြီး code၊ range၊ severity၊ notes၊ help နှင့် normalized message အားလုံးကို စစ်ဆေးသည်။ Run command မှာ:

```text
cargo test --manifest-path native/Cargo.toml
```
