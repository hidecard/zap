# Zap ပြောင်းလဲမှုမှတ်တမ်း

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

## [Unreleased]

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
