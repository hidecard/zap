# Zap Change Log

Zap ၏ version အလိုက် ပြောင်းလဲမှုများနှင့် verified development changes များကို ဤဖိုင်တွင် မှတ်တမ်းတင်ထားသည်။ Stable release မဟုတ်သေးသော အလုပ်များကို သီးခြားဖော်ပြထားသည်။

## [Unreleased]

### Added

- Dedicated `ZapError` diagnostic boundary ထည့်သွင်းထားသည်။ Human-readable runtime output နှင့် `zap check --json` output တို့သည် error kind၊ message နှင့် source location metadata ကို တူညီသော model ဖြင့် ထုတ်ပေးနိုင်သည်။
- `result<number>`၊ `result<text>`၊ `option<number>` နှင့် `option<text>` payload annotations နှင့် `ok`/`err`/`some` constructor inference ကို static checker တွင် ထည့်သွင်းထားသည်။
- `ZapError` အတွက် syntax၊ name၊ type၊ value၊ I/O၊ file-not-found၊ permission၊ overflow နှင့် project error variants များ ထည့်သွင်းထားသည်။
- ZapError classification နှင့် diagnostic location regression unit tests များ ထည့်သွင်းထားသည်။
- Native runtime ကို `diagnostics.rs`၊ `lexer.rs`၊ `parser.rs`၊ `value.rs`၊ `evaluator.rs` နှင့် `project.rs` modules များအဖြစ် ခွဲခြားထားသည်။
- `stdlib.rs` တွင် pure math/text built-in operation dispatch ၏ ပထမဆုံး extraction ကို ထည့်သွင်းထားသည်။
- `Token` များတွင် one-based file/line/column source locations ထည့်သွင်းပြီး lexer diagnostics ကို source-aware ပြုလုပ်ထားသည်။
- Diagnostic messages ထဲရှိ `password`၊ `secret`၊ `token` နှင့် `api_key` key/value များကို `<redacted>` အဖြစ် ဖုံးကွယ်ထားသည်။
- Source line count၊ loop iteration နှင့် execution depth အတွက် runtime resource limits ထည့်သွင်းထားသည်။
- Module refactor နှင့် P0 hardening အပြီး native unit tests 9 ခုနှင့် integration tests 35 ခု အောင်မြင်နေသည်။
- CLI command orchestration ကို `cli.rs` သို့ ခွဲထုတ်ပြီး `0` success၊ `1` program/check failure နှင့် `2` usage error exit-code policy ကို သတ်မှတ်ထားသည်။

### Planned

- `ast.rs` redesign၊ remaining filesystem/JSON/collection standard-library extraction၊ runtime String errors ၏ အပြည့်အဝ ZapError conversion၊ deeper control-flow type narrowing၊ HTTP/URL/Regex modules၊ package lockfiles နှင့် editor tooling remain planned.

## [0.9.1] - 2026-08-19

### Added

- Explicit `import`/`export` module visibility semantics နှင့် export မလုပ်ထားသော symbol isolation ထည့်သွင်းထားသည်။
- Canonical-path module cache ဖြင့် module top-level execution ကို တစ်ကြိမ်သာ run စေသည်။
- Active import stack ဖြင့် circular import detection နှင့် absolute module path rejection ထည့်သွင်းထားသည်။
- `zap check` တွင် annotated variable နှင့် inferred literal/expression type မကိုက်ညီမှုကို static diagnostic အဖြစ် စစ်ဆေးနိုင်သည်။
- Module cache၊ private export၊ cycle detection၊ absolute-path safety နှင့် static assignment mismatch regression tests များ ထည့်သွင်းထားသည်။
- Result-returning expression များအတွက် `?` automatic error propagation ထည့်သွင်းထားသည်။ `ok(value)?` သည် value ကို ဖြည်ပေးပြီး `err(error)?` သည် လက်ရှိ function မှ error Result ကို ပြန်ပို့သည်။
- Result propagation success/error နှင့် invalid non-Result operand အတွက် integration tests များ ထည့်သွင်းထားသည်။

### Verification

- Native integration test **34 ခုလုံး pass** ဖြစ်သည်။
- `cargo test --manifest-path native/Cargo.toml` အောင်မြင်သည်။
- Module documentation ကို Burmese learning guide နှင့် roadmap တွင် synchronize လုပ်ထားသည်။

### Remaining Work

- Runtime internals အားလုံး၏ `String` return types များကို `ZapError` သို့ တိုက်ရိုက်ပြောင်းလဲပြီး evaluator boundary ကို ပိုမိုခိုင်မာစေရန်။
- Branch/loop type narrowing၊ generic/nullable types နှင့် AST-based parser architecture တိုးရန်။
- `cli.rs` extraction၊ standard-library modules အပြည့်အစုံ၊ project-root-aware `../` traversal policy၊ HTTP/URL/Regex standard library ဆက်လက်လုပ်ရန်။

## [0.9.0]

### Added

- Function parameter annotation နှင့် return annotation metadata ကို native runtime တွင် ထည့်သွင်းထားသည်။
- `number`၊ `text`၊ `bool`၊ `list`၊ `map`၊ `object`၊ `none` နှင့် `any` type များအတွက် runtime type checking ထည့်သွင်းထားသည်။
- `zap check` တွင် main source ၏ function signature များကို static စစ်ဆေးနိုင်သည်။
- Unknown type annotation များအတွက် `SyntaxError` သို့မဟုတ် `TypeError` အမျိုးအစား diagnostic ထုတ်နိုင်သည်။
- `zap check --json` တွင် `ok`၊ `kind`၊ `message` နှင့် `error` fields ပါသော machine-readable output ထည့်သွင်းထားသည်။
- Integer addition၊ subtraction၊ multiplication overflow နှင့် division/modulo by zero များအတွက် checked runtime errors ထည့်သွင်းထားသည်။
- Typed function နှင့် structured diagnostic regression tests များ ထည့်သွင်းထားသည်။

### Verification

- Native integration test **34 ခုလုံး pass** ဖြစ်သည်။
- `cargo check` အောင်မြင်သည်။
- `git diff --check` အောင်မြင်သည်။
- Linux၊ macOS ARM64 နှင့် Windows x86_64 release workflow ကို မပြောင်းလဲဘဲ ဆက်လက်အသုံးပြုနိုင်သည်။

### Known Limitations

- Complex control-flow expression များနှင့် collection element များ၏ static inference ကို ဆက်လက်တိုးချဲ့ရန်လိုသည်။
- Runtime evaluator အတွင်းရှိ legacy `String` error return types များကို unified `ZapError` ဖြင့် အပြည့်အဝ အစားထိုးရန် ကျန်ရှိသည်။ CLI diagnostic boundary နှင့် JSON output တွင်တော့ `ZapError` ကို အသုံးပြုထားသည်။
- Structured `Result`၊ `async/await`၊ task cancellation၊ channels၊ HTTP client၊ package lockfile/registry နှင့် LSP မပါဝင်သေးပါ။
- `Result`/`Option` automatic propagation၊ async runtime၊ HTTP၊ package lockfile/registry နှင့် LSP များ မပါဝင်သေးပါ။

## [0.8.0]

- Native Rust runtime၊ OOP၊ collection/file/path/time/environment helpers၊ `zap run`၊ `zap lint`၊ `zap check --json` နှင့် cross-platform release workflow ကို ထည့်သွင်းခဲ့သည်။
- အသေးစိတ် release notes ကို [`docs/RELEASE_0.8.0.md`](docs/RELEASE_0.8.0.md) တွင် ဖတ်ရှုနိုင်သည်။

## [0.7.1]

- OOP audit patch၊ inheritance၊ constructors၊ method override နှင့် object property behavior များကို တည်ငြိမ်အောင် ပြင်ဆင်ခဲ့သည်။

## [0.7.0]

- Class-based OOP foundation နှင့် native runtime feature expansion ကို ထည့်သွင်းခဲ့သည်။

## [0.6.0]

- Standard library၊ modules၊ project manifest နှင့် CLI workflow များကို တိုးချဲ့ခဲ့သည်။

[Unreleased]: docs/RELEASE_0.9.1.md
[0.9.1]: docs/RELEASE_0.9.1.md
[0.9.0]: docs/RELEASE_0.9.0.md
[0.8.0]: docs/RELEASE_0.8.0.md
[0.7.1]: docs/RELEASE_0.7.1.md
[0.7.0]: docs/RELEASE_0.7.0.md
[0.6.0]: docs/RELEASE_0.6.0.md
