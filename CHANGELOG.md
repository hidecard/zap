# Zap Change Log

Zap ၏ version အလိုက် ပြောင်းလဲမှုများနှင့် verified development changes များကို ဤဖိုင်တွင် မှတ်တမ်းတင်ထားသည်။ Stable release မဟုတ်သေးသော အလုပ်များကို သီးခြားဖော်ပြထားသည်။

## [Unreleased]

### Package reliability
- Registry-backed lockfile များကို version 2 သို့ တိုးချဲ့ပြီး resolved package version၊ source နှင့် SHA-256 checksum များကို deterministic `[resolved]` section ဖြင့် pin လုပ်ထားပါသည်။
- `zap lock` နှင့် `zap update` သည် resolved transitive packages များကို မှတ်တမ်းတင်ပြီး `zap install` သည် pinned graph တစ်ခုလုံးကို ပြန်လည် resolve/verify လုပ်ပါသည်။
- ရှိပြီးသား v1 lockfile များနှင့် compatibility ထိန်းသိမ်းထားပြီး offline cache reuse နှင့် checksum-pinned install integration coverage ထည့်သွင်းထားပါသည်။

## [2.0.3] - 2026-08-20

Zap 2.0.3 သည် P3.3 Production Standard Library milestone ကို ပြီးစီးစေသော release ဖြစ်ပါသည်။

### P3.3 နှင့် cross-platform hardening
- Bounded URL၊ HTTP client/server၊ direct non-shell process၊ environment/configuration APIs များကို ထည့်သွင်းထားပါသည်။
- VS Code extension နှင့် LSP signature help/formatting integration ကို ထည့်သွင်းထားပါသည်။
- Windows native path separator expectation၊ JSON-escaped file fixture portability နှင့် Windows smoke gate ရှိ option-aware URL port assertion ပြဿနာများကို ပြင်ဆင်ထားပါသည်။

### Verification
Native suite tests 235 ခု pass ဖြစ်ပြီး P3.3 smoke fixture အောင်မြင်ပါသည်။ Windows smoke gate တွင် option-aware URL port assertion နှင့် CRLF-safe process output normalization ကို ပြင်ဆင်ပြီးနောက် Linux၊ Windows၊ macOS နှင့် strict Clippy CI matrix အားလုံး အောင်မြင်ပါသည်။ Release packaging gates များကိုလည်း GitHub Actions workflow တွင် enforce လုပ်ထားပါသည်။

## [2.0.1] - 2026-08-20
### Bug fixes and release engineering
Zap v2.0.1 သည် P3.1 module/workspace architecture၊ v2 bug-audit ပြင်ဆင်ချက်များနှင့် cross-platform release engineering ကို ပေါင်းစပ်ထားသော maintenance release ဖြစ်သည်။
- Explicit module/import resolution၊ recursive cycle diagnostics နှင့် LSP module/import indexing။
- Stable `Error`/`KeyError` diagnostics၊ annotation validation၊ canonical CLI help နှင့် JSON-RPC `-32601 Method not found` handling။
- Async collection parsing၊ `join` နှင့် map-key `contains` parity ပြင်ဆင်ချက်များ။
- Annotation၊ CLI နှင့် framed LSP end-to-end tests များ။
- Linux၊ Windows နှင့် macOS release workflow တွင် smoke tests၊ versioned archives နှင့် SHA-256 verification။
### Verification
Native tests 229 ခု၊ end-to-end tests 3 ခု၊ formatting၊ whitespace၊ release build နှင့် package checksum checks များ အောင်မြင်သည်။

## [2.0.0] - 2026-08-20

### P2 Ecosystem Release

Zap P2 သည် native runtime၊ deterministic package registry၊ async foundation နှင့် LSP/editor integration များကို Ecosystem milestone အဖြစ် ပြီးစီးစေသည်။

- Exact၊ caret၊ tilde နှင့် comparator version ranges များကို deterministic ရွေးချယ်ခြင်း။
- HTTPS registry transport၊ signed-index verification၊ checksum enforcement၊ deterministic cache pruning၊ offline reuse နှင့် authenticated local persistence။
- Atomic artifact publishing နှင့် signed index rewriting။
- `async fn`၊ `Future`၊ `await`၊ timers၊ cancellation၊ task/poll budgets နှင့် deterministic suspension။
- LSP diagnostics၊ hover၊ completion၊ formatting၊ go-to-definition နှင့် workspace symbols။
- English/Burmese documentation နှင့် release guides များ synchronize ပြုလုပ်ခြင်း။

### Verification

Native tests 223 ခု၊ formatting၊ `cargo check`၊ strict Clippy၊ whitespace နှင့် Linux/macOS/Windows cross-platform release checks များ အောင်မြင်သည်။

## [2.0.2] - 2026-08-20

### P3.2 Structured Error Model

- `raise <expression>` နှင့် same-level `try`/`catch <binding>:` syntax များကို ထည့်သွင်းပြီး bare `raise`၊ malformed binding၊ missing catch နှင့် missing catch body diagnostics များကို deterministic ပြုလုပ်ထားသည်။
- Function၊ loop၊ nested block နှင့် module boundary များကို ဖြတ်သန်းသော structured raise propagation၊ catch binding restoration နှင့် re-raise behavior ကို အကောင်အထည်ဖော်ထားသည်။
- Uncaught raised value များကို process boundary တွင် stable `raised error: <value>` diagnostic အဖြစ် ထုတ်ပေးထားသည်။
- Rust 1.75 compatibility ကို ထိန်းသိမ်းကာ native suite **229 tests passed** ဖြစ်ကြောင်း အတည်ပြုထားသည်။

### P2 Ecosystem progress

- Remote registry index transport၊ HTTPS package downloads၊ content-addressed cache နှင့် SHA-256 integrity enforcement ကို ထည့်သွင်းထားသည်။
- Metadata-validated remote package publishing နှင့် deterministic nested dependency graph traversal/cycle detection ကို ထည့်သွင်းထားသည်။
- Stable-Rust compatible single-threaded async runtime foundation၊ `async fn`၊ deterministic `Future` values နှင့် `await` expressions ကို ထည့်သွင်းထားသည်။
- Stdio JSON-RPC LSP တွင် text synchronization၊ deterministic diagnostics၊ parser-span hover နှင့် context-aware completion ကို ထည့်သွင်းထားသည်။
- English/Burmese P2 roadmap၊ async/LSP guides နှင့် syntax references များကို synchronize ပြုလုပ်ထားသည်။

### Verification

- Native test suite: **223 tests passed**.
- Formatting၊ `cargo check` နှင့် `git diff --check` အောင်မြင်သည်။
- Strict Clippy နှင့် Linux၊ Windows၊ macOS ARM64 cross-platform checks များကို GitHub Actions တွင် အောင်မြင်စွာ verify ပြုလုပ်ထားသည်။
- P2 အားလုံး green မဖြစ်မချင်း release tag မတင်ရသေးပါ။

## [1.0.0] - 2026-08-20

### P1 Language Core Release

Zap P1 သည် standalone native runtime အတွက် Language Core milestone ကို ပြည့်စုံစွာ သတ်မှတ်ပေးသည်။ Direct AST execution၊ default parameters၊ named arguments၊ complex Option/Result type narrowing၊ OOP visibility နှင့် constructor rules၊ module-aware access၊ stabilized standard-library APIs၊ deterministic `zap.lock` package behavior နှင့် cross-platform CI smoke checks များကို ထည့်သွင်းထားသည်။

### Verification

Native tests 109 ခု၊ release build၊ formatting၊ whitespace၊ CLI version/help နှင့် runnable example checks များ အောင်မြင်သည်။ GitHub Actions release workflow သည် Linux x86_64၊ macOS ARM64 နှင့် Windows x86_64 artifact များအတွက် quality gates နှင့် packaging ကို လုပ်ဆောင်သည်။ အသေးစိတ်ကို [English changelog](CHANGELOG_EN.md) နှင့် [Burmese changelog](CHANGELOG_MM.md) တွင် ကြည့်ရှုနိုင်သည်။

## [0.9.3] - 2026-08-19

### Fixed

- Native AST function နှင့် class method body များကို runtime `Function` object ထဲတွင် တိုက်ရိုက်သိမ်းပြီး source-line reconstruction မပြုဘဲ execute လုပ်နိုင်သည်။
- AST execution routing နှင့် legacy OOP/function/module compatibility အတွက် regression coverage တိုးချဲ့ထားသည်။
- CI Rust quality gate တွင် stable Clippy နှင့် local toolchain နှစ်ခုလုံးကို ကိုက်ညီစေရန် indentation validation နှင့် AST compatibility checks များကို ပြင်ဆင်ထားသည်။

### Verification

- Native unit tests 25 ခုနှင့် integration tests 47 ခု စုစုပေါင်း 72 ခု အားလုံးအောင်မြင်သည်။
- OOP class၊ inheritance၊ constructor၊ override၊ property assignment၊ function၊ method၊ module နှင့် Unicode regression tests များ အားလုံးအောင်မြင်သည်။
- Linux x86_64၊ Windows x86_64 နှင့် macOS ARM64 release workflow matrix ကို အသုံးပြုသည်။


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
- Module refactor နှင့် P0 hardening အပြီး native unit tests 13 ခုနှင့် integration tests 47 ခု အောင်မြင်နေသည်။
- CLI command orchestration ကို `cli.rs` သို့ ခွဲထုတ်ပြီး `0` success၊ `1` program/check failure နှင့် `2` usage error exit-code policy ကို သတ်မှတ်ထားသည်။
- `zap test` တွင် `--filter`၊ `--fail-fast` နှင့် `--json` options များ ထည့်သွင်းပြီး unknown options များအတွက် usage error ပြန်ပေးသည်။
- Expression parser သည် expression အပြီးတွင် မသုံးရသေးသော token များကျန်ရှိပါက diagnostic error ထုတ်ပေးပြီး malformed source tests များတွင် line/column diagnostics ကို စစ်ဆေးထားသည်။
- Mixed indentation၊ nested blank/comment blocks၊ Windows-style path၊ permission failure နှင့် module parent-directory traversal regression tests များ ထည့်သွင်းထားသည်။
- User-facing source reads/writes၊ module imports နှင့် project test reads များအတွက် 8 MiB bounded file I/O limit ထည့်သွင်းထားသည်။ Oversized input ကို file content မဖတ်မီ typed error ဖြင့် reject လုပ်သည်။
- Source-span-aware `ast.rs` foundation ကို expression၊ literal၊ call၊ index၊ statement နှင့် program nodes များဖြင့် ထည့်သွင်းထားသည်။ Existing line-based evaluator ကို မပြတ်တောက်စေရန် compatibility foundation အဖြစ် စတင်ထားသည်။
- `run_checked` typed runtime boundary နှင့် regression coverage ထည့်သွင်းပြီး runtime String error များကို CLI အဆင့်မတိုင်မီ `ZapError` model သို့ ပြောင်းပေးသည်။
- AST compatibility parser တွင် assignment၊ return၊ break၊ continue နှင့် expression statements များကို source-span-aware statement nodes အဖြစ် parse လုပ်နိုင်အောင် ထည့်သွင်းထားသည်။ Existing line-based evaluator ကို မပြတ်တောက်စေရန် parser migration ကို အဆင့်လိုက် ဆက်လက်လုပ်ဆောင်နေသည်။
- AST statement parser regression tests ထည့်သွင်းပြီး native unit tests စုစုပေါင်း 18 ခုနှင့် integration tests 47 ခု အောင်မြင်ထားသည်။
- Indentation-aware AST program/block parser ဖြင့် `if`/`else`၊ `while` နှင့် `for ... in ...` control-flow statements များ၊ nested blocks နှင့် malformed/unmatched `else` diagnostics များကို ထည့်သွင်းထားသည်။ Existing line-based evaluator ကို မပြတ်တောက်စေရန် compatibility foundation အဖြစ် ဆက်လက်ထားရှိသည်။
- Control-flow AST regression tests ပြီးနောက် native unit tests စုစုပေါင်း 20 ခုနှင့် integration tests 47 ခု အားလုံး အောင်မြင်ထားသည်။

### Planned

- `ast.rs` redesign၊ remaining filesystem/JSON/collection standard-library extraction၊ runtime String errors ၏ အပြည့်အဝ ZapError conversion၊ deeper control-flow type narrowing၊ HTTP/URL/Regex modules၊ package lockfiles နှင့် editor tooling remain planned.

## [0.9.2] - 2026-08-19

### Added

- Release workflow တွင် Rust formatting၊ Clippy၊ Cargo check နှင့် native unit/integration tests များကို cross-platform build မစတင်မီ quality gate အဖြစ် ထည့်သွင်းထားသည်။
- `v0.9.2` tag သည် `native/Cargo.toml` version နှင့် ကိုက်ညီမှုကို CI တွင် အလိုအလျောက်စစ်ဆေးသည်။
- Native package နှင့် CLI version ကို `0.9.2` သို့ update လုပ်ထားသည်။

### Verification

- Release build matrix သည် Linux x86_64၊ Windows x86_64 နှင့် macOS ARM64 အတွက် quality gate အောင်မြင်ပြီးမှသာ artifact packaging နှင့် GitHub Release publishing ပြုလုပ်မည်။

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

[Unreleased]: docs/P2_PROGRESS.md
[0.9.1]: docs/RELEASE_0.9.1.md
[0.9.0]: docs/RELEASE_0.9.0.md
[0.8.0]: docs/RELEASE_0.8.0.md
[0.7.1]: docs/RELEASE_0.7.1.md
[0.7.0]: docs/RELEASE_0.7.0.md
[0.6.0]: docs/RELEASE_0.6.0.md
