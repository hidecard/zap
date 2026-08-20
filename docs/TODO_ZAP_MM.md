# Zap Language ကျန်ရှိသည့်အလုပ်များ — မြန်မာ To-do List

**လက်ရှိအခြေအနေ — v0.9.2 release နှင့် P0 foundation hardening**

Zap v0.9.1 release line တွင် class-based OOP၊ inheritance၊ collection helpers၊ JSON၊ file I/O၊ function annotations၊ static call checking၊ variable/nested-expression inference၊ typed `Result`/`Option` foundation၊ structured `zap check --json` diagnostics နှင့် cross-platform release workflow များ ပါဝင်သည်။ Release နောက်ပိုင်း native runtime ကို modularize လုပ်ပြီး explicit export visibility၊ canonical-path cache၊ circular import detection နှင့် absolute-path rejection တို့ ထပ်မံပြီးစီးထားသည်။ Native unit tests 25 ခုနှင့် integration tests 47 ခု စုစုပေါင်း 72 ခု အောင်မြင်နေသည်။ v0.9.2 CI တွင် Linux x86_64၊ Windows x86_64 နှင့် macOS ARM64 release build များကို quality gate ဖြင့် စစ်ဆေးထားသည်။ အောက်ပါ To-do list သည် audit findings နှင့် v0.9.0 roadmap ကို အခြေခံထားပြီး **အရင်လုပ်ရမည့် foundation အလုပ်များမှ နောက်ပိုင်း ecosystem အလုပ်များသို့** အစဉ်လိုက် စီထားခြင်း ဖြစ်သည်။

## အခြေအနေသင်္ကေတ

| သင်္ကေတ | အဓိပ္ပာယ် |
|---|---|
| `[x]` | ပြီးမြောက်ပြီး |
| `[ ]` | မပြီးသေး၊ ဆက်လုပ်ရန်လို |
| `P0` | Production stability အတွက် အရေးကြီးဆုံး |
| `P1` | v0.9.0 အတွင်း ဦးစားပေးလုပ်ရန် |
| `P2` | v0.10.0 နှင့် နောက်ပိုင်း |
| `P3` | Ecosystem/framework အဆင့် |

---

## 1။ လက်ရှိပြီးမြောက်ထားသောအရာများ

- [x] Class၊ object၊ constructor နှင့် `self` receiver
- [x] Single inheritance၊ inherited constructor နှင့် method override
- [x] Mutable object properties
- [x] List၊ map၊ string နှင့် basic numeric operations
- [x] JSON encode/decode
- [x] Text နှင့် line-based file I/O
- [x] Path၊ time နှင့် environment helpers
- [x] `zap run`၊ `zap init`၊ `zap build`၊ `zap test`၊ `zap fmt` နှင့် `zap lint`
- [x] `zap check --json` machine-readable diagnostics
- [x] Local module resolution (`main directory`၊ `modules/`၊ `lib/`)
- [x] Explicit `import`/`export` visibility semantics နှင့် private symbol isolation
- [x] Canonical-path module cache ဖြင့် module တစ်ခုကို run တစ်ကြိမ်သာ load လုပ်ခြင်း
- [x] Active import stack ဖြင့် circular import detection
- [x] Absolute module path rejection နှင့် module boundary hardening
- [x] Cross-platform release workflow နှင့် SHA-256 checksums
- [x] Integer overflow နှင့် division/modulo-by-zero runtime safety patch
- [x] Burmese beginner guide၊ syntax guide၊ usage guide နှင့် audit document

---

# 2။ P0 — Foundation နှင့် Production Safety

အောက်ပါအဆင့်များ မပြီးမချင်း async၊ HTTP နှင့် package registry ကဲ့သို့သော feature ကြီးများကို မထည့်သင့်သေးပါ။

## 2.1 Runtime architecture ခွဲခြားခြင်း

- [x] `native/src/main.rs` ၏ core runtime logic ကို dependency အလိုက် modules ခွဲရန်။
- [x] `lexer.rs` — token နှင့် tokenizer
- [x] `parser.rs` — expression/signature/static parser helpers
- [x] `ast.rs` — source-span-aware AST data structures နှင့် expression/statement node foundation ထည့်ထားသည်။
- [x] လက်ရှိ line-based interpreter မှ native AST runtime သို့ နောက်ဆုံး migration ပြီးစီးသည်။ `run()` သည် parseable source အားလုံးကို AST boundary မှတစ်ဆင့် ဖြတ်သန်းပြီး statement၊ function၊ class၊ method နှင့် import nodes များကို native AST executor ဖြင့် ဆောင်ရွက်သည်။ Runtime `Function` struct တွင် `ast_body: Option<Program>` ကို တိုက်ရိုက်သိမ်းထားပြီး function/method call များသည် source-line reconstruction မပြုဘဲ AST body ကို execute လုပ်သည်။ Legacy `body: Vec<String>` ကို older/internal declarations အတွက် compatibility fallback အဖြစ်သာ ထားရှိသည်။
- [x] `value.rs` — Zap value နှင့် object model
- [x] `evaluator.rs` — expression/statement execution၊ function/method calls၊ modules နှင့် control flow
- [x] `stdlib.rs` — pure math/text standard-library operations ၏ ပထမဆုံး extraction
- [x] `diagnostics.rs` — structured errors
- [x] `project.rs` — manifest၊ module၊ project validation
- [x] `cli.rs` — command-line argument handling နှင့် command orchestration
- [x] Public/internal API boundary များကို `cli.rs`၊ `project.rs` နှင့် evaluator modules အကြား ပိုမိုရှင်းလင်းစွာ သတ်မှတ်ရန်။
- [x] Architecture ခွဲပြီးနောက် လက်ရှိ native test suite အားလုံး pass ဖြစ်သည်။ လက်ရှိ native unit tests 25 ခုနှင့် integration tests 47 ခု အားလုံးအောင်မြင်သည်။

`stdlib.rs` extraction သည် ပထမ milestone ဖြစ်ပြီး path၊ file I/O၊ JSON၊ collection နှင့် system helpers များကို ထပ်မံခွဲထုတ်ရန် ကျန်ရှိသည်။


## 2.2 Source location နှင့် diagnostics

- [x] Token တိုင်းတွင် `file`၊ `line` နှင့် `column` source span သိမ်းရန်။ Lexer သည် one-based line/column နှင့် token span ကို ထုတ်ပေးသည်။
- [x] Lexer နှင့် parse-related error များတွင် အမှားဖြစ်သည့် source location ကို ပြရန်။
- [x] Runtime error schema ကို `ZapError` kind/message/file/line/column model ဖြင့် သတ်မှတ်ပြီး `run_checked` typed boundary ထည့်ထားသည်။

```json
{
  "kind": "TypeError",
  "message": "expected number, got text",
  "file": "main.zp",
  "line": 4,
  "column": 12
}
```

- [x] `zap check --json` တွင် `kind`၊ `message` နှင့် `error` fields ထည့်ရန်။
- [x] `file`၊ `line` နှင့် `column` ကို JSON fields အဖြစ် သီးခြားထည့်ရန်။
- [x] Human-readable error နှင့် `zap check --json` error နှစ်မျိုးလုံးသည် `ZapError` diagnostic boundary ကို အသုံးပြုရန်။
- [x] CLI exit code သတ်မှတ်ချက် ပြုလုပ်ရန်။ `0` = success၊ `1` = program/check failure၊ `2` = CLI usage error။

## 2.3 Error model

- [x] CLI diagnostic boundary တွင် internal Rust error message များကို `ZapError` enum ဖြင့် ခွဲခြားရန်။ `SyntaxError`၊ `NameError`၊ `TypeError`၊ `ValueError`၊ `IOError`၊ `FileNotFound`၊ `PermissionError`၊ `OverflowError` နှင့် `ProjectError` kinds များကို သတ်မှတ်ထားသည်။ `run_checked` boundary မှ runtime String errors များကို typed `ZapError` သို့ ပြောင်းပေးသည်။ Runtime internals အားလုံးကို enum သို့ တိုက်ရိုက်ပြောင်းလဲခြင်းသည် နောက်ထပ် parser/evaluator migration အဖြစ် ကျန်ရှိသည်။
- [x] `SyntaxError`၊ `NameError`၊ `TypeError`၊ `ValueError`၊ `IOError`၊ `FileNotFound`၊ `PermissionError` နှင့် `OverflowError` များကို `ZapError` variants အဖြစ် သတ်မှတ်ရန်။
- [x] User program ထဲတွင် recoverable value အသုံးပြုရန် `ok(value)` နှင့် `err(value)` Result constructors ထည့်ရန်။
- [x] `some(value)`၊ `option_none()`၊ `is_ok`၊ `is_err`၊ `is_some` နှင့် `is_option_none` helpers ထည့်ရန်။
- [x] `unwrap` နှင့် `unwrap_or` semantics ကို သတ်မှတ်ရန်။
- [x] Result/Option JSON serialization နှင့် `type()` support ထည့်ရန်။
- [x] `result<number>`၊ `result<text>`၊ `option<number>` နှင့် `option<text>` ကဲ့သို့သော Result/Option payload annotation များကို static type checker ဖြင့် စစ်ရန်။ `ok(value)`၊ `err(value)` နှင့် `some(value)` payload type မကိုက်ညီပါက `TypeError` diagnostic ထုတ်သည်။
- [x] Result error အတွက် `?` automatic propagation semantics သတ်မှတ်ပြီး အကောင်အထည်ဖော်ရန်။ `try`/`catch` equivalent နှင့် typed payload propagation သည် နောက်ထပ်အလုပ်ဖြစ်သည်။
- [x] Error message တွင် `password`၊ `secret`၊ `token` နှင့် `api_key` key/value များကို `<redacted>` အဖြစ် ဖုံးကွယ်ရန်။
- [x] Panic၊ unchecked unwrap နှင့် silent fallback များကို user input path များတွင် ဖယ်ရှားရန်။ Production user-input paths များတွင် panic-causing `unwrap`/`expect` မရှိတော့ဘဲ JSON conversion fallback များကို typed errors ဖြင့် ပြောင်းထားသည်။

## 2.4 Parser correctness နှင့် language consistency

- [x] Mixed indentation ကို တိတိကျကျ reject လုပ်ရန်။
- [x] Blank line၊ comment နှင့် nested block handling ကို test ပြုလုပ်ရန်။
- [x] Expression အဆုံးတွင် မသုံးရသေးသော token ကျန်နေပါက error ပြရန်။ AST program parser တွင်လည်း unmatched `else` နှင့် malformed control-flow blocks များကို reject လုပ်သည်။
- [x] Function၊ class၊ `if`၊ `for` နှင့် `while` syntax တူညီသော indentation-aware AST parser ဖြင့် စစ်ရန်။ ထို့အပြင် typed `let`၊ `say`၊ `import/use` နှင့် dotted property assignment များကိုလည်း AST statement nodes အဖြစ် parse လုပ်နိုင်သည်။ Function/class declarations များသည် parameter/return annotations၊ optional single inheritance name နှင့် nested body spans များကို AST ထဲတွင် သိမ်းထားသည်။ Parseable program အားလုံးသည် `run()` မှ AST boundary နှင့် native AST executor ကို အသုံးပြုသည်။ Function/method runtime object များတွင် AST body ကို တိုက်ရိုက်သိမ်းပြီး declaration/module forms များအတွက် source reconstruction မလိုတော့ပါ။
- [x] Division၊ modulo၊ arithmetic overflow နှင့် negative index behavior ကို specification ထဲတွင် တိတိကျကျရေးရန်။ Arithmetic overflow၊ zero division/modulo နှင့် `i64::MIN / -1` ကို runtime error အဖြစ် ပြန်ပေးသည်။ Indexing သည် zero-based ဖြစ်ပြီး negative index သည် reverse index မဟုတ်ဘဲ `index out of range` ဖြစ်သည်။ အသေးစိတ်ကို [`P0_FOUNDATION_STATUS_EN.md`](P0_FOUNDATION_STATUS_EN.md) တွင် ဖတ်ရှုနိုင်သည်။
- [x] Execution depth၊ loop count နှင့် source line count အတွက် runtime resource limits သတ်မှတ်ရန်။ လက်ရှိ limit များမှာ depth `256`၊ loop iteration `100000` နှင့် source lines `100000` ဖြစ်သည်။

## 2.5 Test foundation

- [x] `zap test --filter <name>` ထည့်ရန်။
- [x] `zap test --fail-fast` ထည့်ရန်။
- [x] Test တစ်ခုချင်းစီအတွက် pass/fail summary ထုတ်ရန်။
- [x] Assertion တွင် expected/actual value ပြရန်။
- [x] Exit code နှင့် test result JSON report ထည့်ရန်။
- [x] Error cases၊ malformed source၊ Unicode၊ Windows path နှင့် permission failure tests တိုးရန်။
- [x] CI တွင် Linux၊ Windows နှင့် macOS ARM64 test matrix သတ်မှတ်ရန်။

---

# 3။ P1 — v0.9.0 Language Core

## 3.1 Function type system

- [x] Function parameter annotation syntax သတ်မှတ်ရန်။

  လက်ရှိ syntax သည် `fn add(a: number, b: number) -> number:` ဖြစ်ပြီး parameter နှင့် return annotation များကို runtime တွင် စစ်ဆေးသည်။

```zap
fn add(a: number, b: number) -> number:
    return a + b
```

- [ ] `text`၊ `number`၊ `bool`၊ `list`၊ `map`၊ `object` နှင့် `none` type များ စစ်ရန်။
- [x] Function argument type ကို function call အချိန်တွင် runtime စစ်ရန်။
- [x] Return expression type နှင့် function return annotation ကို runtime စစ်ရန်။
- [x] Function signature ၏ annotation syntax နှင့် allowed types ကို `zap check` တွင် static စစ်ရန်။
- [x] Function call argument count နှင့် literal argument type ကို `zap check` တွင် static စစ်ရန်။
- [x] Literal variable expression၊ arithmetic/text nested expression နှင့် annotated function-return expression များ၏ inferred type ကို function call အတွင်း static စစ်ရန်။
- [ ] Branch/loop control-flow အတွင်း type narrowing နှင့် reassignment inference တိုးရန်။
- [x] Annotated variable literal/nested-expression mismatch ကို `zap check` တွင် စစ်ရန်။
- [x] Unknown function annotation များအတွက် `zap check` နှင့် `zap check --json` diagnostic ထုတ်ရန်။
- [x] Function call ၏ argument count နှင့် literal argument type mismatch ကို `zap check --json` structured diagnostic အဖြစ် ထုတ်ရန်။
- [x] Literal variable နှင့် ရိုးရိုး nested expression များ၏ inferred type mismatch ကို static diagnostic အဖြစ် ထုတ်ရန်။
- [ ] Complex nested call၊ collection element နှင့် control-flow expression များ၏ inferred type mismatch ကို တိုးချဲ့ရန်။

`zap check --json` သည် ယခု `file`၊ `line` နှင့် `column` fields များကို သီးခြားပြန်ပေးနိုင်ပြီး editor/CI tooling များက diagnostic location ကို တိုက်ရိုက်အသုံးပြုနိုင်သည်။
- [x] `any` type ၏ လက်ရှိ permissive runtime semantics ကို documentation တွင် ဖော်ပြရန်။
- [ ] Generic list/map/function design ကို syntax မတည်ငြိမ်မီ အရင်ဆုံးရေးသားသတ်မှတ်ရန်။

## 3.2 OOP ပြည့်စုံမှု

- [ ] `super.init()` ကို explicit အသုံးပြုနိုင်ရန်။
- [ ] `super.method()` ဖြင့် parent method ခေါ်နိုင်ရန်။
- [ ] Constructor မခေါ်သော object creation ကို စစ်ဆေးရန်။
- [ ] Interface/trait စနစ်ကို single inheritance နှင့် မရှုပ်ထွေးအောင် ဒီဇိုင်းချရန်။
- [ ] Abstract class နှင့် abstract method semantics သတ်မှတ်ရန်။
- [ ] Public/private/protected visibility modifiers စဉ်းစားရန်။
- [ ] Object equality၊ string representation နှင့် hash behavior သတ်မှတ်ရန်။
- [ ] Circular inheritance နှင့် duplicate method definitions ကို reject လုပ်ရန်။

## 3.3 Stable module system

- [x] `use` syntax နှင့်အတူ explicit `import`/`export` semantics တည်ငြိမ်စေရန်။
- [x] Export မလုပ်ထားသော symbol ကို module အပြင်မှ မမြင်ရအောင်လုပ်ရန်။
- [x] Module တစ်ခုကို နှစ်ကြိမ် load မဖြစ်စေရန် canonical-path module cache ထည့်ရန်။
- [x] Circular import detection ထည့်ရန်။
- [ ] Relative path၊ package name နှင့် standard module resolution rules သတ်မှတ်ရန်။
- [x] Absolute module path များကို reject လုပ်ပြီး module boundary ကို မတော်တဆ ပြင်ပဖိုင်သို့ မချဲ့စေရန် ကာကွယ်ရန်။
- [x] `../` traversal ကို project root အပြင် မထွက်စေရန် parent-directory traversal rejection policy ထည့်ရန်။ Project-root canonical/symlink policy သည် နောက်ထပ် hardening အဖြစ် ကျန်ရှိသည်။
- [x] API documentation နှင့် module example များ ထည့်ရန်။

## 3.4 Standard library တိုးချဲ့ခြင်း

- [ ] `http` client — method၊ URL၊ headers၊ status၊ body နှင့် timeout
- [ ] URL parsing နှင့် query parameter helpers
- [ ] Regex matching နှင့် replacement
- [ ] Base64၊ URL encoding နှင့် common text encoding
- [ ] Command-line arguments (`args`) နှင့် process exit
- [ ] Directory create/list/remove helpers
- [ ] File metadata၊ file size နှင့် safe temporary files
- [ ] Date/time formatting နှင့် timezone-aware values
- [ ] Randomness နှင့် cryptographic randomness ကို ခွဲခြားရန်
- [ ] Environment variable ကို optional result အဖြစ် ပြန်ရန်
- [ ] Stream/iterator abstraction အတွက် design ရေးရန်
- [ ] Database API ကို standard core မဟုတ်ဘဲ package boundary အဖြစ် စဉ်းစားရန်

---

# 4။ P1 — Package Manager နှင့် Reproducible Build

- [ ] `zap.toml` manifest schema ကို version လုပ်ရန်။
- [ ] Dependency declaration syntax သတ်မှတ်ရန်။

```toml
[dependencies]
text_utils = "1.0.0"
local_lib = { path = "../local_lib" }
```

- [ ] `zap add <package>` command
- [ ] `zap remove <package>` command
- [ ] `zap install` command
- [ ] `zap update` command
- [ ] `zap.lock` lockfile schema
- [ ] Semantic version constraint rules
- [ ] Local path dependency support
- [ ] Git dependency support
- [ ] Registry dependency support
- [ ] SHA-256 checksum verification
- [ ] Dependency graph cycle detection
- [ ] Offline/cache mode
- [ ] Reproducible build verification
- [ ] Malicious package၊ path escape နှင့် dependency confusion ကာကွယ်ရေး
- [ ] `zap publish` ကို registry security နှင့် ownership policy ပြီးမှ ထည့်ရန်

---

# 5။ P2 — Async နှင့် Concurrency

Async syntax မထည့်မီ semantics ကို အရင်ဆုံး သတ်မှတ်ရမည်။

- [ ] Event loop သို့မဟုတ် task scheduler ရွေးချယ်ရန်။
- [ ] `async fn` နှင့် `await` syntax specification ရေးရန်။
- [ ] Future/Task value model သတ်မှတ်ရန်။
- [ ] Task handle၊ join နှင့် result retrieval ထည့်ရန်။
- [ ] Cancellation token နှင့် cancellation propagation ထည့်ရန်။
- [ ] Timeout နှင့် deadline API ထည့်ရန်။
- [ ] Bounded channel နှင့် backpressure ထည့်ရန်။
- [ ] Message passing နှင့် shared mutable state ၏ ကန့်သတ်ချက် သတ်မှတ်ရန်။
- [ ] Async error propagation ကို `Result` နှင့် ချိတ်ရန်။
- [ ] Deterministic async test utilities ထည့်ရန်။
- [ ] Race condition၊ deadlock နှင့် resource leak tests ရေးရန်။
- [ ] File/network I/O ကို blocking နှင့် non-blocking API ခွဲရန်။

---

# 6။ P2 — Developer Tooling

- [ ] Lexer/parser AST dump command (`zap ast`)
- [ ] Static type checking command ကို အပြည့်အစုံလုပ်ရန်။
- [ ] `zap fmt --check` ထည့်ရန်။
- [ ] Formatter output သည် deterministic ဖြစ်ကြောင်း test လုပ်ရန်။
- [ ] Configurable lint rules (`zap.toml`) ထည့်ရန်။
- [ ] Unused variable/function warning ထည့်ရန်။
- [ ] Dead code နှင့် unreachable branch warning ထည့်ရန်။
- [ ] Test coverage report ထည့်ရန်။
- [ ] Property-based testing နှင့် fuzzing ထည့်ရန်။
- [ ] Benchmark command နှင့် profiling output ထည့်ရန်။
- [ ] VS Code syntax highlighting နှင့် snippets
- [ ] Language Server Protocol (LSP)
- [ ] Go-to-definition၊ diagnostics၊ autocomplete နှင့် rename support
- [ ] Debugger protocol သို့မဟုတ် minimal breakpoint debugger
- [ ] API documentation generator
- [ ] REPL သို့မဟုတ် interactive playground

---

# 7။ P2 — Runtime Security နှင့် Performance

- [ ] User program အတွက် maximum source size သတ်မှတ်ရန်။
- [ ] Maximum recursion depth သတ်မှတ်ရန်။
- [ ] Maximum loop iterations ကို configuration ပြုလုပ်ရန်။
- [ ] Maximum string/list/map size သတ်မှတ်ရန်။
- [ ] File read/write size limit ထည့်ရန်။
- [ ] HTTP response body size limit ထည့်ရန်။
- [ ] Path traversal နှင့် symlink escape စစ်ရန်။
- [ ] JSON depth နှင့် nesting limit သတ်မှတ်ရန်။
- [ ] Deterministic map serialization option ထည့်ရန်။
- [ ] Interpreter allocation နှင့် execution benchmark ရေးရန်။
- [ ] AST evaluator နှင့် bytecode VM performance ကို နှိုင်းယှဉ်ရန်။
- [ ] Bytecode VM prototype တည်ဆောက်ရန်။
- [ ] Long-running task memory leak စစ်ရန်။
- [ ] Crash၊ panic နှင့် malformed input fuzz test ပြုလုပ်ရန်။

---

# 8။ P3 — Framework နှင့် Ecosystem

Core language နှင့် package system တည်ငြိမ်ပြီးမှ အောက်ပါ domain packages များကို သီးခြားတည်ဆောက်ရန်။

## Web

- [ ] HTTP server package
- [ ] Routing နှင့် middleware
- [ ] Request/response၊ cookie၊ session API
- [ ] Template သို့မဟုတ် component system
- [ ] Static asset serving
- [ ] WebSocket support
- [ ] Secure defaults နှင့် request limits

## AI

- [ ] Provider-independent AI client interface
- [ ] HTTP streaming response support
- [ ] JSON schema output
- [ ] Embedding/vector API boundary
- [ ] Retry၊ timeout၊ rate limit နှင့် secret management
- [ ] Local model/remote provider ကို core runtime နှင့် မရောရန်

## Mobile

- [ ] Android bridge design
- [ ] iOS bridge design
- [ ] UI/runtime boundary
- [ ] Cross-compilation နှင့် packaging
- [ ] Native permission model

## IoT/Embedded

- [ ] Serial၊ GPIO၊ I2C၊ SPI API boundary
- [ ] Cross-compilation target
- [ ] Resource-constrained runtime profile
- [ ] OTA update နှင့် device security design
- [ ] Hardware API ကို standard core မဟုတ်ဘဲ platform package အဖြစ် ခွဲရန်

---

# 9။ Documentation နှင့် Community

- [ ] Syntax guide ကို feature တစ်ခုချင်းစီအတွက် executable sample နှင့် တိုးချဲ့ရန်။
- [ ] Burmese beginner guide နှင့် English reference တို့၏ feature boundary တူညီစေရန်။
- [ ] Version migration guide (`v0.8.0 → v0.9.0`) ရေးရန်။
- [ ] Language specification ကို implementation-independent ဖြစ်အောင် ရေးရန်။
- [ ] Error catalog နှင့် troubleshooting guide ရေးရန်။
- [ ] Standard library API reference generator ထည့်ရန်။
- [ ] Code of Conduct၊ contribution guide နှင့် security policy ထည့်ရန်။
- [ ] Changelog ကို semantic versioning နှင့် ချိတ်ရန်။
- [ ] Beginner examples၊ web example၊ AI example နှင့် file-processing example ထည့်ရန်။
- [ ] Breaking change များအတွက် deprecation policy သတ်မှတ်ရန်။

---

# 10။ Release အလိုက် လုပ်ဆောင်ရန်အစီအစဉ်

| Release | အဓိကရည်မှန်းချက် | ပြီးမြောက်ရန်လိုအပ်သောအရာ |
|---|---|---|
| v0.8.1 | Stability patch | Diagnostics အခြေခံပြင်ဆင်မှု၊ error regression tests၊ cross-platform bug fixes |
| v0.9.0 | Language foundation | Runtime modularization၊ source spans၊ structured errors၊ function type checking၊ test filter၊ stable modules |
| v0.9.1 | Standard library | HTTP client၊ URL၊ regex၊ encoding၊ process args၊ safe filesystem API |
| v0.10.0 | Package ecosystem | `zap.lock`၊ dependency resolver၊ checksum၊ `zap add/install/update` |
| v0.11.0 | Async foundation | Future/task model၊ async/await၊ cancellation၊ timeout၊ bounded channels |
| v0.12.0 | Tooling | LSP၊ formatter/linter config၊ coverage၊ fuzzing၊ debugger prototype |
| v1.0.0 | Production stability | Stable specification၊ compatibility policy၊ security audit၊ reproducible release၊ complete documentation |
| v1.x | Framework ecosystem | Web၊ AI၊ Mobile နှင့် IoT packages/frameworks |

---

# 11။ Release မလုပ်မီ Acceptance Checklist

- [ ] Linux၊ Windows နှင့် macOS ARM64 builds အားလုံး pass ဖြစ်ရန်။
- [ ] Native unit/integration tests အားလုံး pass ဖြစ်ရန်။
- [ ] Burmese guide၊ syntax guide၊ README နှင့် release notes တို့ feature boundary တူရန်။
- [ ] CLI error များသည် သင့်တော်သော non-zero exit code ပြန်ရန်။
- [ ] JSON diagnostics schema documented ဖြစ်ရန်။
- [ ] No panic/unchecked unwrap on malformed user source test pass ဖြစ်ရန်။
- [ ] Fuzz test နှင့် resource-limit test pass ဖြစ်ရန်။
- [ ] Package checksum နှင့် lockfile verification pass ဖြစ်ရန်။
- [ ] Release archives တွင် SHA-256 checksum ပါရန်။
- [ ] Version၊ changelog၊ tag နှင့် binary filenames တူညီရန်။
- [ ] Fresh machine installation နှင့် `zap init` → `zap run` workflow စမ်းပြီးရန်။
- [ ] Upgrade နှင့် rollback လုပ်ငန်းစဉ် စာတမ်းပြုထားရန်။

---

# 12။ အကြံပြုထားသော လက်တွေ့လုပ်ဆောင်မှုအစဉ်

**ပထမအဆင့်** အဖြစ် runtime source ကို module ခွဲပြီး structured diagnostics ထည့်ရန်။ ထို့နောက် function type checking နှင့် stable module import/export ကို တည်ဆောက်ရမည်။ အဲဒီ foundation များ တည်ငြိမ်ပြီးမှ HTTP standard library နှင့် package lockfile ကို ထည့်သွင်းသင့်သည်။ Async/await ကို syntax အနေဖြင့် အလျင်စလိုမထည့်ဘဲ task lifecycle၊ cancellation နှင့် error propagation ကို အရင်ဆုံး စမ်းသပ်သတ်မှတ်ရမည်။

**နောက်ဆုံးအဆင့်** တွင်သာ LSP၊ debugger၊ package registry နှင့် Web/AI/Mobile/IoT frameworks များကို တည်ဆောက်သင့်သည်။ ဤအစီအစဉ်သည် Zap ကို feature များပြားသော်လည်း မတည်ငြိမ်သော language မဖြစ်စေဘဲ သင်ယူရလွယ်ကူပြီး production အတွက် ယုံကြည်စိတ်ချရသော ecosystem ဖြစ်လာစေရန် ရည်ရွယ်ထားသည်။

ဆက်စပ်စာရွက်စာတမ်းများ — [`ROADMAP_0.8.0.md`](ROADMAP_0.8.0.md)၊ [`AUDIT_LANGUAGE_COMPARISON_2026-08.md`](AUDIT_LANGUAGE_COMPARISON_2026-08.md)၊ [`PACKAGES.md`](PACKAGES.md) နှင့် [`LEARN_ZAP_MM.md`](LEARN_ZAP_MM.md)။
