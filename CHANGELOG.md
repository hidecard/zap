# Zap Change Log

Zap ၏ version အလိုက် ပြောင်းလဲမှုများနှင့် verified development changes များကို ဤဖိုင်တွင် မှတ်တမ်းတင်ထားသည်။ Stable release မဟုတ်သေးသော အလုပ်များကို သီးခြားဖော်ပြထားသည်။

## [Unreleased]

## [2.11.5] - 2026-08-24

### Release summary
- Windows CLI smoke tests၊ bilingual current-status/provenance documentation နှင့် missing-prerequisite များကို test failure နှင့် ခွဲခြားပေးသော `make doctor` ကို ထည့်ထားသည်။ Bootstrap stage သည် B0 ဖြစ်ပြီး Rust သည် reference owner အဖြစ် ဆက်ရှိသည်။

## [2.11.4] - 2026-08-24

### Release summary
- Provisional list-element inference path နှင့် deterministic native/Zap-candidate negative collection-element diagnostic fixture ကို ထည့်သွင်းထားသည်။ Rust သည် reference owner အဖြစ် ဆက်ရှိပြီး bootstrap stage သည် B0 ဖြစ်သည်။

## [2.11.3] - 2026-08-24

### Release summary
- Annotated function၊ return propagation၊ incompatible function-call diagnostic ပါသော provisional Zap-owned B2 corpus ကို ချဲ့ထွင်ပြီး transient target-native test အတွက် fail-closed one-retry release hardening ထည့်ထားသည်။ B0/non-self-hosted boundary ကို ထိန်းသိမ်းထားသည်။

## [2.11.2] - 2026-08-24

### Release summary
- Annotated function၊ return propagation နှင့် incompatible function-call diagnostic ပါသော provisional Zap-owned B2 type-checker corpus ကို ချဲ့ထွင်ထားပြီး B0/non-self-hosted boundary ကို ထိန်းသိမ်းထားသည်။

## [2.11.1] - 2026-08-24

### Release summary
- Provisional Zap-owned type-checker candidate၊ candidate-only typed-IR producer နှင့် deterministic B2 differential gates ကို ထည့်သွင်းထားပြီး B0/non-self-hosted boundary ကို ထိန်းသိမ်းထားသည်။

## [2.11.0] - 2026-08-24

### Release summary
- Deterministic Web route explanation

## [2.10.1] - 2026-08-24

### Release summary
- Zap-only bootstrap parser/diagnostics foundation၊ token-driven delimiter diagnostics နှင့် B2 typed-IR/type-check conformance evidence ကို ထည့်သွင်းထားသည်။ B0/non-self-hosted boundary ကို ထိန်းသိမ်းထားသည်။

## [2.10.0] - 2026-08-24

### Release summary
- Native Web validation and centralized Result error middleware

## [2.9.2] - 2026-08-24

### Release summary
- macOS တွင် အလုပ်မလုပ်သော GNU-only `chmod --reference` ကို portable `stat`/`chmod` implementation ဖြင့် ပြင်ဆင်ပြီး installer/uninstaller release verification ကို ခိုင်မာစေသည်။

## [2.9.0] - 2026-08-24

### Release summary
- Installer၊ Makefile၊ release archive၊ production host configuration နှင့် documentation validation များကို harden လုပ်ထားသည်။ Host adapter တွင် persistent SQLite repository၊ bounded cursor pagination နှင့် explicit production fail-closed behavior ပါဝင်သည်။

## [2.9.0] - 2026-08-24

### Release summary
- Reject duplicate Web route registrations before serving

## [2.8.0] - 2026-08-24

### Release summary
- Add validated Web route inspection and improve zap new scaffold clarity

## [2.7.0] - 2026-08-23

### Release summary
- Add bounded incremental LSP synchronization with UTF-aware range safety and deterministic rejection of malformed edits.

## [2.6.0] - 2026-08-23

### Release summary
- Add bounded host metrics and strengthen Web observability evidence without exposing user-controlled labels.

## [2.5.0] - 2026-08-23

### Release summary
- Harden documentation integrity and strengthen the native project foundation.

## [2.4.0] - 2026-08-23

### Release summary
- Rebuilt the bilingual Zap Language Guide, simplified the documentation surface, and clarified the one-command user-managed project workflow.

## [2.3.0] - 2026-08-23

### Framework and modular Web scaffold
- Integrated the Zap-native Framework runtime, frontend boundary, authentication/deployment contracts, registry hardening, and bilingual operational documentation.
- Added an explicit generated `ui/ui.zp` module so Web projects separate models, services/functions, browser UI metadata, routes, middleware, migrations, admin registration, and tests.
- Preserved the standalone native runtime boundary: deployed Zap projects do not require Python, Node.js, Rust, Java, or another language runtime.

## [2.2.6] - 2026-08-23

### Release summary
- Core reliability hardening: confined line I/O, strict locked builds, bounded operations, malformed-port rejection, cycle-safe test discovery, registry-test isolation, and cross-platform compatibility.

## [2.2.5] - 2026-08-23

### Release summary
- Hardened HTTP request URL invariant handling without changing parser/runtime syntax or framework scope.

## [2.2.4] - 2026-08-23

### Release summary
- Post-v2.2.3 active-baseline documentation synchronization for the language specification and generic type-check decision records.

## [2.2.3] - 2026-08-22

### Release summary
- Runtime reliability hardening with bounded cycle-safe equality, checked borrow propagation, panic-free invariants, and synchronized bilingual documentation.

## [2.2.2] - 2026-08-22

### Release summary
- Completed the post-v2.2.1 runtime safety, canonical builtin, documentation, and editor-parity maintenance cycle.

### Post-v2.2.0 corrective cycle
- LSP-SYNC-01၊ LSP-REN-01၊ LSP-INTEROP-01 နှင့် EXT-201 ကို post-release `master` တွင် ပြီးစီးထားသည်။ Standard full-document synchronization၊ versioned state၊ file-local scope-aware rename၊ negotiated positions၊ strict file URI၊ bounded workspace index နှင့် canonical VS Code package များ ပါဝင်ပြီး incremental range change နှင့် cross-file rename ကို support မလုပ်သေးပါ။
- API-301 တွင် schema-2 `determinism_class` (`pure`၊ `input-deterministic`၊ `runtime-dependent` နှင့် `external-io`) နှင့် compatibility-preserving legacy `deterministic` boolean ကို ထည့်သွင်းထားသည်။
- DOC-401 သည် v2.2.0 tag commit [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb) နှင့် ၎င်း၏ asset များ immutable ဖြစ်ကြောင်း၊ ပြင်ဆင်ထားသော behavior ကို v2.2.1 patch အသစ်တွင်သာ ထုတ်ဝေမည်ဖြစ်ကြောင်း မှတ်တမ်းတင်သည်။ အသေးစိတ်ကို [`POST_V2.2.0_REMEDIATION_MM.md`](docs/POST_V2.2.0_REMEDIATION_MM.md) တွင် ကြည့်ပါ။

## [2.2.1] - 2026-08-22

### Corrective release
- v2.2.0 နောက်ပိုင်း LSP synchronization၊ scope-aware file-local rename၊ URI/position/workspace-boundary hardening၊ canonical VS Code package နှင့် schema-2 standard-library determinism taxonomy များကို ထုတ်ဝေထားသည်။
- Explicit limitation များကို ဆက်လက်ထိန်းသိမ်းထားသည်။ LSP synchronization သည် full-text only ဖြစ်ပြီး cross-file rename မရှိပါ။ Async သည် full production reactor မဟုတ်ဘဲ bounded foundation ဖြစ်ပြီး traits/composition သည် design-only ဖြစ်သည်။
- Release scope နှင့် provenance အတွက် [`docs/RELEASE_2.2.1_MM.md`](docs/RELEASE_2.2.1_MM.md) နှင့် [`docs/POST_V2.2.0_REMEDIATION_MM.md`](docs/POST_V2.2.0_REMEDIATION_MM.md) ကို ကြည့်ပါ။

## [2.2.0] - 2026-08-22

### Release summary
- Completed the audited runtime, verification, registry, standard-library, LSP/editor, bilingual documentation, and traits/composition RFC milestones; the traits RFC remains design-only and does not enable new syntax.

## [2.1.14] - 2026-08-21

### Release summary
- Hardened explicit workspace and LSP state migration with Windows-compatible line-helper behavior.

## [2.1.13] - 2026-08-21

### Release summary
- Migrated workspace confinement and LSP document ownership into explicit state objects.

## [2.1.12] - 2026-08-22

### Release summary
- Canonical AST execution now owns normal programs and local modules; legacy line execution is compatibility-only.

## [2.1.11] - 2026-08-21

### Release summary
- Added the first explicit per-run RuntimeState and ExecutionContext boundary with bilingual documentation and regression coverage.

## [2.1.10] - 2026-08-21

### Release summary
- Added bilingual documentation consistency validation, navigation landing pages, and p95 benchmark regression protection.

## [2.1.9] - 2026-08-21

### Release summary
- Added panic-free object borrow diagnostics and checked field access.

## [2.1.8] - 2026-08-21

### Release summary
- Hardened release version consistency validation across all user-facing release surfaces.

## [2.1.7] - 2026-08-21

### Release summary
- Expanded specification ownership and added error-free P0/P1 release preflight gates.

## [2.1.6] - 2026-08-21

### Release summary
- Hardened type-check conformance, pinned CI toolchains, and verified cross-platform release flow.

## [2.1.5] - 2026-08-21

### Release summary
- Signed release publication and cross-platform reproducible packaging hardening.

## [2.1.4] - 2026-08-21

### Release summary
- Fix Windows clean-profile installer verification on PowerShell runners.

## [2.1.3] - 2026-08-21

### Release summary
- Corrected deterministic Windows archive-root packaging and release verification.

## [2.1.2] - 2026-08-21

### Release summary
- Cross-platform deterministic archive packaging and release-workflow reproducibility hardening.

## [2.1.1] - 2026-08-21

### Release summary
- Type-checking conformance and cross-tool diagnostic hardening.

### Package reliability
- Legacy lockfile များကို conservative အတိုင်း migrate လုပ်ရန် `zap lock-migrate [dir]` command အသစ် ထည့်သွင်းထားပါသည်။
- v1 lockfile compatibility ကို ဆက်လက်ထိန်းသိမ်းထားပြီး verified registry metadata မရှိပါက registry version သို့မဟုတ် checksum များကို မခန့်မှန်းဘဲ migration ကို ရပ်တန့်ပါသည်။
- `zap install` သည် transitive package များအပါအဝင် resolved registry graph တစ်ခုလုံးကို deterministic `name@version` order ဖြင့် ပြသပြီး ရှိပြီးသား dependency-count prefix ကို ဆက်လက်ထိန်းသိမ်းထားပါသည်။
- Transitive resolution၊ cache verification၊ stable install output၊ transitive artifact ပျောက်ဆုံးမှု၊ cached checksum မကိုက်ညီမှုနှင့် မပြည့်စုံသော v2 lockfile များအတွက် deterministic diagnostics ပါသော offline nested-registry integration fixtures များ ထည့်သွင်းထားပါသည်။
- Canonical project lockfile မှ keep entries များကို ရယူသော `zap registry gc [--dry-run] [dir]` ကို ထည့်သွင်းထားပါသည်။ Dry-run တွင် cache မပြောင်းဘဲ stale နှင့် temporary candidate များကို ပြသပြီး ပုံမှန် run တွင် candidate များကို deterministic lexical order ဖြင့် ဖယ်ရှားပါသည်။
- Transport နှင့် registry-service failure coverage များ ထည့်သွင်းပြီး insecure HTTP rejection၊ malformed remote-index diagnostic နှင့် non-2xx fetch/publish response များအတွက် deterministic HTTP-status error များကို စစ်ဆေးပြီးဖြစ်သည်။
### Async နှင့် tooling
- Deterministic `AsyncRuntime::spawn_joinable(future)`၊ `JoinHandle<T>::is_ready()` နှင့် future အဖြစ် output join လုပ်နိုင်မှုတို့ကို ထည့်သွင်းထားပါသည်။
- `SpawnError::TaskLimitReached` ကို propagate လုပ်ပြီး runtime task order၊ poll budget၊ Rust 1.75 compatibility နှင့် worker thread မဖန်တီးသော execution model ကို ထိန်းသိမ်းထားပါသည်။ Regression coverage နှင့် bilingual guide များကို `docs/ASYNC_RUNTIME_EN.md` နှင့် `docs/ASYNC_RUNTIME_MM.md` တွင် ထည့်သွင်းထားပါသည်။
- `AsyncRuntime::spawn_joinable_cancellable(future)` နှင့် `CancellationToken` ကို ထည့်သွင်းပြီး cancellation ဖြစ်သော join များကို inner future ကို ဆက်မ poll လုပ်ဘဲ `JoinError::Cancelled` ဖြင့် resolve လုပ်ပါသည်။
- `timeout_ticks(future, ticks)` သည် wall-clock time မဟုတ်ဘဲ executor poll အရေအတွက်အပေါ် အခြေခံ၍ `TimeoutError` ကို deterministic အတိုင်း propagate လုပ်ပြီး cancellation၊ timeout failure နှင့် completion လမ်းကြောင်းများအတွက် regression tests ထည့်သွင်းထားပါသည်။
- `spawn_joinable_result(future)` နှင့် `spawn_joinable_result_cancellable(future)` တို့ကို ထည့်သွင်းပြီး `TaskJoinError::Failed(E)` ဖြင့် typed task failure များကို propagate လုပ်ပါသည်။ Cancellation ကို inner future မ poll မီ စစ်ဆေးပြီး repeated join များကို `AlreadyJoined` ဖြင့် ပြတ်သားစွာ ပြန်ပေးပါသည်။ Typed failure၊ cancellation precedence နှင့် repeated join များအတွက် regression tests နှင့် bilingual async guide update များ ထည့်သွင်းထားပါသည်။
- Evaluator နှင့် legacy expression path နှစ်ခုလုံးတွင် language-level task facade builtins `spawn`၊ `task_join` နှင့် `task_is_ready` များကို ထည့်သွင်းထားပါသည်။ Eager Future semantics၊ strict arity/type diagnostics၊ async spawn/readiness/join behavior နှင့် invalid-input regression coverage များကို async နှင့် async/LSP guides များတွင် မှတ်တမ်းတင်ထားပါသည်။

### Standard library
- Symlink-safe portable metadata အတွက် `file_metadata(path)` နှင့် bounded temporary-file/sync/rename workflow ပါသော `atomic_write(path, content)` standard-library APIs များကို ထည့်သွင်းထားပါသည်။
- လက်ရှိ JSON safety limit အောက်တွင် runtime category ကို စစ်ဆေးပေးသော `from_json_typed(source, expected)` နှင့် UTF-8 byte များအစား Unicode scalar value များဖြင့် အလုပ်လုပ်သော `char_at`၊ `substring`၊ `codepoints` APIs များကို ထည့်သွင်းထားပါသည်။
- Bounded output နှင့် stable runtime errors ပါသော deterministic collection helpers `entries(map)` နှင့် `enumerate(list)` များကို ထည့်သွင်းထားပါသည်။
- UTC time API များကို ထည့်သွင်းထားပြီး `utc_now()` သည် seconds နှင့် millisecond timestamp များကို ပြန်ပေးကာ `duration_parts(milliseconds)` နှင့် `duration_between(end_millis, start_millis)` သည် sign ထိန်းသိမ်းထားသော checked decomposition နှင့် overflow diagnostic များကို ပေးပါသည်။
- UTC timestamp consistency၊ positive/negative duration နှင့် invalid input များအတွက် regression tests များ၊ English/Burmese time guides များကို ထည့်သွင်းထားပါသည်။
- Deterministic structured logging builtins `log_record(level, message, fields)` နှင့် `log_json(level, message, fields)` များကို ထည့်သွင်းထားပါသည်။ Canonical field ordering၊ accepted levels နှင့် validation rules များကို regression tests ဖြင့် စစ်ဆေးထားပါသည်။
- Structured logging တွင် message 8 KiB၊ field 64 ခု၊ field-name 256 bytes နှင့် encoded JSON output 64 KiB safety limits များ သတ်မှတ်ထားပြီး English/Burmese guides များကို `docs/STDLIB_LOGGING_EN.md` နှင့် `docs/STDLIB_LOGGING_MM.md` တွင် ထည့်သွင်းထားပါသည်။


## [2.0.4] - 2026-08-20

### Package reliability
- Registry-backed lockfile များကို version 2 သို့ တိုးချဲ့ပြီး resolved package version၊ source နှင့် SHA-256 checksum များကို deterministic `[resolved]` section ဖြင့် pin လုပ်ထားပါသည်။
- `zap lock` နှင့် `zap update` သည် resolved transitive packages များကို မှတ်တမ်းတင်ပြီး `zap install` သည် pinned graph တစ်ခုလုံးကို ပြန်လည် resolve/verify လုပ်ပါသည်။
- ရှိပြီးသား v1 lockfile များနှင့် compatibility ထိန်းသိမ်းထားပြီး offline cache reuse နှင့် checksum-pinned install integration coverage ထည့်သွင်းထားပါသည်။

### Security audit remediation
- `ZAP_UNTRUSTED=1` restricted mode ဖြင့် filesystem၊ environment၊ process၊ network၊ local HTTP serving နှင့် local registry-source capability များကို default deny လုပ်ထားပါသည်။
- Loopback၊ private၊ link-local၊ unspecified၊ broadcast နှင့် IPv6 local destination များကို SSRF ကာကွယ်မှုဖြင့် block လုပ်ပြီး automatic HTTP redirects များကို restricted mode တွင် ပိတ်ထားပါသည်။
- HTTP request body limit၊ hard process deadline/kill behavior နှင့် capability/network regression tests များ ထည့်သွင်းထားပါသည်။
- OS-level sandbox၊ least-privilege deployment၊ resource quotas နှင့် network egress controls များသည် production deployment အတွက် ဆက်လက်လိုအပ်ပါသည်။

### Documentation နှင့် editor integration
- Main README တွင် v2.0.4 installation links၊ current status၊ security-mode note နှင့် official VS Code Marketplace v0.5.0 installation လမ်းညွှန်များကို update လုပ်ထားပါသည်။
- Extension manifest နှင့် documentation များကို Marketplace publisher `ArkarYan` နှင့် synchronize လုပ်ပြီး `code --install-extension ArkarYan.zap-language-support` command ကို ထည့်သွင်းထားပါသည်။

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
