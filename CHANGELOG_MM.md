# Zap ပြောင်းလဲမှုမှတ်တမ်း

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
