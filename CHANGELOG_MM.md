# Zap ပြောင်းလဲမှုမှတ်တမ်း

## [2.0.3] — 2026-08-20

Zap 2.0.3 သည် P3.3 Production Standard Library milestone ကို ပြီးစီးစေသော release ဖြစ်ပါသည်။

### ထည့်သွင်းထားသော အချက်များ

- Deterministic validation ပါသော ကန့်သတ်ထားသည့် `url_parse`၊ `url_encode` နှင့် `url_decode` builtin များ။
- HTTP/HTTPS URL များကိုသာ လက်ခံပြီး timeout နှင့် response-size limit ပါသော `http_get` နှင့် `http_request` builtin များ။
- Shell မသုံးသော `process_run`၊ text argument များ၊ UTF-8 stdout/stderr capture၊ status report နှင့် output limit များ။
- Safe configuration helper များဖြစ်သော deterministic default ပါသည့် `env_get`၊ platform-aware `config_dir` နှင့် relative file name တစ်ခုတည်းကိုသာ လက်ခံသော traversal-resistant `config_path`။
- Loopback တွင် bind လုပ်ပြီး request တစ်ခုတည်းကို serve သည့် bounded `http_serve_once` local server၊ request၊ response နှင့် wait limit များပါဝင်ခြင်း။
- API အသစ်များအတွက် deterministic standard-library catalog နှင့် English/Burmese documentation update များ။
- `.zp` registration၊ TextMate syntax highlighting၊ snippets၊ autocomplete၊ CLI-backed diagnostics၊ workspace check နှင့် current-file run command များပါသော `vscode-extension` folder အသစ်။
- Zap function call များအတွက် native နှင့် VS Code LSP signature help၊ `(` နှင့် `,` နောက် active parameter tracking ပါဝင်ခြင်း။
- Line ending normalize၊ space လေးခု indentation နှင့် trailing whitespace cleanup ပါဝင်သော LSP document formatting။

### စစ်ဆေးမှု

- Native suite: **tests 235 ခု pass** ဖြစ်ပါသည်။
- P3.3 URL၊ process၊ HTTP validation၊ configuration၊ local-server argument နှင့် compatibility regression များ အောင်မြင်ပါသည်။
- Evaluator native path separator expectation၊ Windows JSON file fixture escaping၊ option-aware URL port assertion နှင့် Windows smoke gate ရှိ CRLF-safe process output normalization များအတွက် cross-platform test hardening ပြီးစီးပါသည်။
- Linux native suite tests **235 ခု pass** ဖြစ်ပြီး Windows/macOS target-native tests၊ strict Clippy နှင့် release packaging များကို GitHub Actions တွင် ဆက်လက် enforce လုပ်ထားပါသည်။

## [2.0.1] — 2026-08-20

Zap v2.0.1 သည် P2 Ecosystem foundation နောက်ပိုင်း production-quality maintenance release ဖြစ်ပြီး P3.1 module/workspace architecture အပိုင်းကို ပြီးစီးစေကာ v2 audit findings များကို ပြင်ဆင်ထားပါသည်။

### ထည့်သွင်းပြီး ပြင်ဆင်ထားသော အချက်များ

- `module <name>` declaration နှင့် `import <path> as <alias>` syntax များကို manifest-backed deterministic resolution ဖြင့် ထည့်သွင်းထားပါသည်။
- Recursive multi-module graph validation၊ traversal protection၊ missing-target diagnostics၊ repeated-import caching နှင့် circular-dependency chain အပြည့်အစုံကို ထည့်သွင်းထားပါသည်။
- Module declarations နှင့် import aliases များအတွက် LSP completion၊ hover၊ definition နှင့် workspace-symbol indexing ကို တိုးချဲ့ထားပါသည်။
- Stable runtime `Error` နှင့် `KeyError` diagnostic categories နှင့် structured human-readable/JSON output ကို ထည့်သွင်းထားပါသည်။
- Supported scalar နှင့် generic annotation များအတွက် declaration-time validation ကို ထည့်သွင်းထားပါသည်။
- Help၊ invalid arguments နှင့် invalid paths များအတွက် canonical CLI help/usage output ကို တစ်မျိုးတည်းဖြစ်အောင် ပြင်ဆင်ထားပါသည်။
- Unknown LSP request များအတွက် JSON-RPC `-32601 Method not found` response ကို ထည့်သွင်းပြီး notification behavior ကို မပြောင်းလဲဘဲ ထိန်းသိမ်းထားပါသည်။
- Collection literal parsing နှင့် `join`/map-key `contains` AST နှင့် legacy behavior parity ကို ပြင်ဆင်ထားပါသည်။
- Annotation၊ CLI help နှင့် framed LSP request များအတွက် cross-process integration/end-to-end tests များ ထည့်သွင်းထားပါသည်။
- Linux x86_64၊ macOS ARM64 နှင့် Windows x86_64 အတွက် archive checksum နှင့် smoke tests ပါသော hardened GitHub Actions release packaging ကို ထည့်သွင်းထားပါသည်။

### Verification

- Native unit/integration suite: **tests 229 ခု pass** ဖြစ်ပါသည်။
- Audit regression နှင့် end-to-end tests: **tests 3 ခု pass** ဖြစ်ပါသည်။
- Formatting၊ whitespace၊ release build၊ CLI smoke၊ example execution နှင့် package checksum checks များ အောင်မြင်ပါသည်။
- GitHub Actions release workflow သည် tag/Cargo version matching ကို စစ်ဆေးပြီး verified platform archives များကို publish လုပ်ပါသည်။

## [2.0.0] — 2026-08-20

Zap P2 သည် native runtime၊ deterministic package registry၊ async foundation နှင့် editor integration များအတွက် Ecosystem milestone ကို ပြီးစီးစေပါသည်။

### ထည့်သွင်းပြီးသော အင်္ဂါရပ်များ

- Registry dependency များအတွက် exact၊ caret၊ tilde နှင့် comparator version-range selection ကို deterministic ပြုလုပ်ခြင်း။
- HTTPS registry transport၊ SHA-256 artifact verification၊ signed-index HMAC verification၊ deterministic cache pruning၊ offline reuse နှင့် authenticated local registry persistence။
- Checksum-verified package publishing၊ atomic artifact storage နှင့် signed index rewriting။
- `async fn`၊ deterministic `Future` values၊ `await`၊ poll-based timers၊ cancellation tokens၊ cancellable tasks၊ task limits၊ poll budgets နှင့် deterministic suspension controls။
- LSP document synchronization၊ diagnostics၊ hover၊ context-aware completion၊ formatting၊ go-to-definition နှင့် workspace symbols။
- P2 foundation အပြည့်အစုံကို ဖော်ပြထားသော English/Burmese documentation updates များ။

### Verification

- Native test suite: **tests 223 ခု pass** ဖြစ်ပါသည်။
- Formatting၊ `cargo check`၊ whitespace နှင့် strict Clippy gates များ အောင်မြင်ပါသည်။
- Linux x86_64၊ Windows x86_64 နှင့် macOS ARM64 CI checks များ အောင်မြင်ပါသည်။
- Release artifacts များကို tag-triggered GitHub Actions workflow မှ ထုတ်ပေးပါသည်။

## [2.0.2] — 2026-08-20

### P3.2 Structured Error Model

- `raise <expression>` နှင့် same-level `try`/`catch <binding>:` syntax များကို ထည့်သွင်းပြီး bare `raise`၊ binding မမှန်ခြင်း၊ catch မရှိခြင်းနှင့် catch body မရှိခြင်းတို့အတွက် deterministic parser diagnostics များကို ထည့်သွင်းထားပါသည်။
- Function၊ loop၊ nested block နှင့် module များအတွင်း structured raise propagation ကို catch binding restoration နှင့် re-raise behavior အပါအဝင် အကောင်အထည်ဖော်ထားပါသည်။
- Catch မလုပ်နိုင်သော raised value များကို process boundary တွင် `raised error: <value>` ဟူသော stable diagnostic အဖြစ် ထုတ်ပေးထားပါသည်။
- Rust 1.75 compatibility နှင့် deterministic AST/legacy behavior ကို ထိန်းသိမ်းကာ native suite **tests 229 ခု pass** ဖြစ်ကြောင်း စစ်ဆေးထားပါသည်။

### P2 Ecosystem တိုးတက်မှု

- HTTPS registry index နှင့် artifact transport၊ content-addressed cache နှင့် SHA-256 integrity enforcement များကို ထည့်သွင်းထားပါသည်။
- Metadata validation ပါသော remote publishing နှင့် deterministic nested dependency traversal/cycle diagnostics များကို ထည့်သွင်းထားပါသည်။
- Stable Rust နှင့် ကိုက်ညီသော single-threaded async runtime foundation၊ `async fn`၊ deterministic `Future` value နှင့် `await` expression များကို ထည့်သွင်းထားပါသည်။
- Stdio JSON-RPC LSP တွင် text synchronization၊ deterministic diagnostics၊ parser-span hover နှင့် context-aware completion များကို တိုးချဲ့ထားပါသည်။
- English/Burmese P2 roadmap၊ async/LSP guide နှင့် syntax reference များကို synchronize ပြုလုပ်ထားပါသည်။

### Verification

- Native test suite: **tests 223 ခု pass** ဖြစ်ပါသည်။
- Formatting၊ `cargo check` နှင့် `git diff --check` အောင်မြင်ပါသည်။
- Strict Clippy နှင့် Linux၊ Windows၊ macOS ARM64 checks များ GitHub Actions တွင် အောင်မြင်ပါသည်။
- P2 track အားလုံး green နှင့် verified မဖြစ်မချင်း release tag မတင်ရသေးပါ။

## [1.0.0] — 2026-08-20

Zap P1 သည် standalone native runtime အတွက် ပထမဆုံး ပြည့်စုံသော Language Core milestone ဖြစ်ပါသည်။ ဤ release တွင် language semantics များကို ကြိုတင်ခန့်မှန်းနိုင်စေရန်၊ direct AST execution၊ လုံခြုံသော diagnostics နှင့် နောင် Ecosystem အလုပ်များအတွက် ခိုင်မာသော foundation ကို အဓိကထားပါသည်။

### ထည့်သွင်းပြီးသော အင်္ဂါရပ်များ

- Functions၊ methods၊ closures၊ indexing၊ built-ins၊ filesystem၊ JSON၊ environment၊ path နှင့် time helpers များအတွက် direct AST evaluation။
- User-defined functions၊ methods နှင့် closures များအတွက် default parameters နှင့် named arguments။
- `option<T>` နှင့် `result<T>` guards၊ complex boolean conditions၊ aliases နှင့် `else` branch restoration အတွက် static type narrowing။
- OOP methods နှင့် fields များအတွက် `public`၊ `private` နှင့် `protected` visibility rules။
- Module-aware private access checks နှင့် protected inheritance behavior။
- Constructor visibility၊ field default initialization၊ explicit `super.init()` delegation နှင့် implicit parent-constructor delegation တစ်ကြိမ်သာ ပြုလုပ်ခြင်း။
- Text၊ math၊ collections၊ filesystem၊ JSON၊ environment၊ path နှင့် time standard-library APIs များကို stabilization ပြုလုပ်ခြင်း။
- Deterministic public standard-library domain catalog နှင့် English/Burmese API indexes။
- Canonical `zap.lock` generation၊ dependency entries များကို sorted ပြုလုပ်ခြင်း၊ missing/stale lockfile rejection နှင့် deterministic local dependency validation။
- Structured diagnostics၊ JSON diagnostics၊ source locations၊ secret redaction နှင့် runtime resource limits။
- Linux၊ macOS နှင့် Windows CLI version၊ help နှင့် example execution အတွက် cross-platform CI smoke checks။

### Documentation

- Main README နှင့် English/Burmese learning guides များကို update ပြုလုပ်ထားပါသည်။
- Type-narrowing၊ package/lockfile နှင့် public standard-library indexes များအတွက် English/Burmese documentation များ ထည့်သွင်းထားပါသည်။
- P1 progress roadmap နှင့် release documentation များကို synchronize ပြုလုပ်ထားပါသည်။

### Verification

- Native tests 109 ခု pass ဖြစ်ပါသည်။ ၎င်းတွင် unit tests 31 ခုနှင့် integration tests 78 ခု ပါဝင်ပါသည်။
- Formatting၊ whitespace၊ release build၊ CLI version/help နှင့် runnable example checks များ local တွင် အောင်မြင်ပါသည်။
- GitHub Actions release workflow သည် stable Rust formatting၊ Clippy၊ check၊ test၊ version/tag matching နှင့် Linux/macOS/Windows artifact verification များကို အလိုအလျောက်လုပ်ဆောင်ပါသည်။

### Scope

P1 တွင် remote package registry၊ package publishing၊ async execution နှင့် LSP/editor integration များ မပါဝင်သေးပါ။ ထိုအလုပ်များကို P2 Ecosystem roadmap အတွက် ချန်ထားပါသည်။

## ယခင် release များ

ယခင် version များ၏ အသေးစိတ်မှတ်တမ်းကို [`CHANGELOG.md`](CHANGELOG.md) တွင် ကြည့်ရှုနိုင်ပါသည်။
