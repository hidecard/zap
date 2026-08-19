# Zap Change Log

Zap ၏ version အလိုက် ပြောင်းလဲမှုများနှင့် verified development changes များကို ဤဖိုင်တွင် မှတ်တမ်းတင်ထားသည်။ Stable release မဟုတ်သေးသော အလုပ်များကို သီးခြားဖော်ပြထားသည်။

## [Unreleased — v0.9.0 Development]

### Added

- Function parameter annotation နှင့် return annotation metadata ကို native runtime တွင် ထည့်သွင်းထားသည်။
- `number`၊ `text`၊ `bool`၊ `list`၊ `map`၊ `object`၊ `none` နှင့် `any` type များအတွက် runtime type checking ထည့်သွင်းထားသည်။
- `zap check` တွင် main source ၏ function signature များကို static စစ်ဆေးနိုင်သည်။
- Unknown type annotation များအတွက် `SyntaxError` သို့မဟုတ် `TypeError` အမျိုးအစား diagnostic ထုတ်နိုင်သည်။
- `zap check --json` တွင် `ok`၊ `kind`၊ `message` နှင့် `error` fields ပါသော machine-readable output ထည့်သွင်းထားသည်။
- Integer addition၊ subtraction၊ multiplication overflow နှင့် division/modulo by zero များအတွက် checked runtime errors ထည့်သွင်းထားသည်။
- Typed function နှင့် structured diagnostic regression tests များ ထည့်သွင်းထားသည်။

### Verification

- Native integration test **27 ခုလုံး pass** ဖြစ်သည်။
- `cargo check` အောင်မြင်သည်။
- `git diff --check` အောင်မြင်သည်။
- Linux၊ macOS ARM64 နှင့် Windows x86_64 release workflow ကို မပြောင်းလဲဘဲ ဆက်လက်အသုံးပြုနိုင်သည်။

### Known Limitations

- Function call တစ်ခုချင်းစီ၏ argument type/count ကို `zap check` မှ static inference ဖြင့် မစစ်ဆေးရသေးပါ။
- JSON diagnostic တွင် `file`၊ `line` နှင့် `column` ကို သီးခြား fields အဖြစ် မခွဲရသေးပါ။
- Structured `Result`၊ `async/await`၊ task cancellation၊ channels၊ HTTP client၊ package lockfile/registry နှင့် LSP မပါဝင်သေးပါ။
- ဤအပိုင်းသည် v0.9.0 development line ဖြစ်ပြီး stable v0.8.0 version/tag ကို မပြောင်းလဲသေးပါ။

## [0.8.0]

- Native Rust runtime၊ OOP၊ collection/file/path/time/environment helpers၊ `zap run`၊ `zap lint`၊ `zap check --json` နှင့် cross-platform release workflow ကို ထည့်သွင်းခဲ့သည်။
- အသေးစိတ် release notes ကို [`docs/RELEASE_0.8.0.md`](docs/RELEASE_0.8.0.md) တွင် ဖတ်ရှုနိုင်သည်။

## [0.7.1]

- OOP audit patch၊ inheritance၊ constructors၊ method override နှင့် object property behavior များကို တည်ငြိမ်အောင် ပြင်ဆင်ခဲ့သည်။

## [0.7.0]

- Class-based OOP foundation နှင့် native runtime feature expansion ကို ထည့်သွင်းခဲ့သည်။

## [0.6.0]

- Standard library၊ modules၊ project manifest နှင့် CLI workflow များကို တိုးချဲ့ခဲ့သည်။

[Unreleased — v0.9.0 Development]: docs/RELEASE_0.9.0.md
[0.8.0]: docs/RELEASE_0.8.0.md
[0.7.1]: docs/RELEASE_0.7.1.md
[0.7.0]: docs/RELEASE_0.7.0.md
[0.6.0]: docs/RELEASE_0.6.0.md
