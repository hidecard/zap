## v2.2.7

- Framework branch တွင် ကျန်ရှိနေသော RustSec advisory ၆ ခုကို v2.2.6 security-clean dependency baseline သို့ update လုပ်၍ ဖြေရှင်းထားသည်။
- Rust 1.88.0 သို့ align လုပ်ခြင်း၊ native/host RustSec CI gate ထည့်ခြင်းနှင့် rcgen 0.13 TLS test compatibility ပြင်ခြင်းများ ပါဝင်သည်။
- Zap-native Framework starter validation နှင့် release-facing documentation များကို synchronize လုပ်ထားသည်။

# Zap ပြောင်းလဲမှုမှတ်တမ်း

## [Unreleased]

## [2.11.12] — 2026-08-24

### Release engineering
- Tracked option<number> တစ်ခုအတွက် provisional၊ corpus-limited direct is_option_none else-body narrowing evidence နှင့် native/candidate paired fixture များ ထည့်ထားသည်။

## [2.11.11] — 2026-08-24

### Release engineering
- Corpus-limited while-loop option narrowing နှင့် loop ပြီးနောက် wrapper restoration evidence ကို ထည့်ပြီး Rust ကို reference owner အဖြစ် ဆက်ထားသည်။

## [2.11.10] — 2026-08-24

### Release engineering
- Branch hygiene နှင့် safe merge/cleanup policy ကို မှတ်တမ်းတင်ပြီး ရည်ရွယ်ချက်ရှိရှိ ထိန်းသိမ်းထားသော superseded branch history ကို မဖျက်ပါ။

## [2.11.9] — 2026-08-24

### Release engineering
- Paired native နှင့် candidate fixture များပါသော bounded direct-is_some branch-local option narrowing evidence ထည့်ထားသည်။

## [2.11.8] — 2026-08-24

### Release engineering
- Tracked map<text,number> variable ကို text literal key ဖြင့် index လုပ်သော provisional၊ corpus-limited B2 map-element inference slice ကို paired native/candidate fixture နှင့် deterministic mismatch diagnostic များဖြင့် ထည့်သွင်းထားပါသည်။ Rust သည် complete reference compiler နှင့် runtime owner အဖြစ် ဆက်ရှိပြီး bootstrap status သည် B0 အဖြစ်သာ ရှိပါသည်။

## [2.11.7] — 2026-08-24

### Malformed-source safety
- Malformed generic၊ unknown-annotation နှင့် incompatible-annotation source များအတွက် fail-closed native CLI regression harness ထည့်ထားပါသည်။ Case တစ်ခုစီသည် panic သို့မဟုတ် unchecked-unwrap signature မပါဘဲ nonzero ဖြင့် fail ရမည်ဖြစ်ပြီး CI နှင့် release preflight နှစ်ခုစလုံးတွင် required ဖြစ်ပါသည်။ Bootstrap သည် B0 ဖြစ်ပြီး Rust သည် reference owner အဖြစ် ဆက်ရှိပါသည်။

## [2.11.6] — 2026-08-24

### B2 nested inference conformance
- `list<list<number>>` nested-index inference အတွက် bounded provisional slice တစ်ခုကို paired positive/negative fixture၊ deterministic native/candidate diagnostic နှင့် `BOOT-023` ownership evidence များနှင့်အတူ ထည့်ထားပါသည်။ Rust သည် reference owner ဖြစ်ပြီး bootstrap သည် B0 အဖြစ် ဆက်ရှိပါသည်။

## [2.11.5] — 2026-08-24

### Release-gate နှင့် developer validation
- Windows release smoke test များတွင် `zap.exe --version`၊ help၊ example၊ `zap new`၊ `zap check`၊ `zap build --locked` နှင့် `zap test` တို့ကို required fail-closed operation များအဖြစ် ထည့်ထားပါသည်။
- Bilingual canonical current-status page၊ signed-provenance documentation နှင့် regression coverage ပါသော `make doctor` ကို ထည့်ထားပါသည်။ Bootstrap သည် B0 ဖြစ်ပြီး Rust သည် reference owner အဖြစ် ဆက်ရှိပါသည်။

## [2.11.4] — 2026-08-24

### B2 bootstrap conformance
- Provisional list-element inference path တစ်ခုနှင့် deterministic native/Zap-candidate diagnostic ပါသော negative collection-element fixture တစ်ခု ထည့်ထားပါသည်။ Rust သည် reference owner အဖြစ် ဆက်ရှိပြီး bootstrap stage သည် B0 ဖြစ်ပါသည်။

## [2.11.3] — 2026-08-24

### Zap-only bootstrap နှင့် release engineering
- Provisional Zap-owned type-checker candidate ကို annotated function တစ်ခု၊ return propagation နှင့် deterministic incompatible function-call diagnostic အထိ ချဲ့ထားပါသည်။ Type checking နှင့် typed IR အပြည့်အစုံသည် native-owned အဖြစ် ဆက်ရှိပြီး stage သည် B0 ဖြစ်ပါသည်။
- သီးခြား macOS ARM64 first-attempt failure နောက်တွင် target-native tests အတွက် fail-closed retry တစ်ကြိမ် ထည့်ထားပါသည်။ ရှိပြီးသား v2.11.2 tag ကို rewrite မလုပ်ထားပါ။

## [2.11.2] — 2026-08-24

### Zap-only bootstrap foundations
- Provisional Zap-owned type-checker candidate ကို annotated function တစ်ခု၊ return propagation နှင့် deterministic incompatible function-call diagnostic အထိ ချဲ့ထားပါသည်။ Type checking နှင့် typed IR အပြည့်အစုံသည် native-owned အဖြစ် ဆက်ရှိပြီး stage သည် B0 ဖြစ်ပါသည်။

## [2.11.1] — 2026-08-24

### Zap-only bootstrap foundations
- Annotated declaration၊ compatible conditional expression နှင့် incompatible number annotation များအတွက် ပထမဆုံး provisional Zap-owned type-checker candidate ကို deterministic CI/release-preflight coverage နှင့်အတူ ထည့်သွင်းထားပါသည်။
- Annotated declaration fixture အတွက် candidate-only Zap typed-IR producer ကို ထည့်သွင်းပြီး ၎င်း၏ owned node field များကို native reference artifact နှင့် နှိုင်းယှဉ်ထားပါသည်။ Type checking နှင့် typed IR အပြည့်အစုံသည် native-owned အဖြစ် ဆက်ရှိပြီး stage သည် B0 အဖြစ်သာ ဆက်ရှိပါသည်။

## [2.11.0] — 2026-08-24

### Release engineering
- Handler မ execute ဘဲ route match ကို ရှင်းပြသော bounded `zap explain route` command၊ parameter/wildcard extraction၊ JSON automation output နှင့် bilingual framework smoke coverage ကို ထည့်သွင်းထားပါသည်။

## [2.10.1] — 2026-08-24

### Zap-only bootstrap foundations
- Canonical reference parser AST နှင့် syntax-diagnostic differential fixture များ၊ arithmetic နှင့် compound corpus case များကို cover လုပ်ပြီး byte-for-byte CI/release-preflight check ပါဝင်သော provisional Zap-written parser candidate ကို ထည့်သွင်းထားပါသည်။
- Parser candidate ၏ substring-based bracket check အစား missing နှင့် unexpected closing bracket များကို cover လုပ်သော token-driven delimiter diagnostics ကို ထည့်သွင်းထားပါသည်။
- Typed-IR reproducibility နှင့် type-check acceptance/rejection fixture များကို ထည့်သွင်းထားသော်လည်း typed IR နှင့် type checking ကို native-owned အဖြစ် တိကျစွာ ထိန်းသိမ်းထားပါသည်။
- Bootstrap identity များကို upstream v2.10.0 နှင့် reconcile လုပ်ပြီး B0/non-self-hosted wording ကို ထိန်းသိမ်းကာ historical v2.9.2 freeze ကို immutable evidence အဖြစ် ဆက်ထားပါသည်။

## [2.10.0] — 2026-08-24

### Release engineering
- Bounded native Web request validation၊ typed ResultOk/ResultErr value များနှင့် centralized Result-aware HTTP error mapping ကို scaffold၊ catalog၊ policy နှင့် ဘာသာစကားနှစ်မျိုး documentation coverage နှင့်အတူ ထည့်သွင်းထားပါသည်။

## [2.9.2] — 2026-08-24

### Release engineering
- macOS တွင် အလုပ်မလုပ်သော GNU-only `chmod --reference` cleanup ကို portable `stat`/`chmod` handling ဖြင့် ပြင်ဆင်ပြီး Linux နှင့် macOS နှစ်မျိုးလုံးတွင် Unix uninstaller verification အောင်မြင်စေပါသည်။

## [2.9.0] — 2026-08-24

### Release engineering
- Unix installer၊ Makefile contributor entrypoint၊ standalone release archive၊ production host configuration နှင့် documentation validation များကို harden လုပ်ထားပါသည်။
- Persistent SQLite host storage၊ bounded cursor pagination၊ explicit production fail-closed behavior နှင့် release-archive link verification ကို ထည့်သွင်းထားပါသည်။

## [2.9.0] — 2026-08-24

### Release engineering
- zap web check နှင့် zap web routes အတွင်း duplicate method/path Web route registration များကို reject လုပ်ပြီး shared validation နှင့် နှစ်ဘာသာ documentation ထည့်သွင်းထားပါသည်။

## [2.8.0] — 2026-08-24

### Release engineering
- Validated Web route table စစ်ဆေးမှု၊ project name အလိုက် scaffold နှင့် နှစ်ဘာသာ Web workflow documentation ကို ထည့်သွင်းထားပါသည်။

## [2.7.0] — 2026-08-23

### Release engineering
- Bounded incremental LSP synchronization၊ UTF-aware range validation၊ sequential edit limit နှင့် regression evidence များကို ထည့်သွင်းပြီး cross-file refactoring boundary များကို ရှင်းလင်းစွာ ထိန်းသိမ်းထားသည်။

## [2.6.0] — 2026-08-23

### Release engineering
- Bounded host metrics endpoint၊ integration evidence နှင့် bilingual Web observability documentation ကို ထည့်သွင်းပြီး production boundary များကို ရှင်းလင်းစွာ ဆက်လက်ထိန်းသိမ်းထားသည်။

## [2.5.0] — 2026-08-23

### Release engineering
- Repository-wide Markdown link validation၊ bilingual operations guide synchronization၊ user-managed Web boundary ရှင်းလင်းချက်နှင့် release provenance record များကို ခိုင်မာစေပါသည်။

## [2.4.0] — 2026-08-23

### Release engineering
- Installation မှ Advanced အထိ bilingual Zap Language Guide အသစ်၊ README ရှင်းလင်းမှုနှင့် stale package note ဖယ်ရှားမှုကို ထည့်သွင်းထားပါသည်။

## [2.3.0] — 2026-08-23

### LSP diagnostics နှင့် code actions
- Server-side `textDocument/codeAction` support ထည့်ပြီး `quickfix`၊ `source` နှင့် `source.organizeImports` capabilities ကို advertise လုပ်ထားပါသည်။
- Tab၊ trailing whitespace နှင့် character 120 ကျော်သော line များအတွက် stable style diagnostic code နှင့် Warning severity ထည့်ထားပါသည်။
- Diagnostic range များကို line တစ်ကြောင်းလုံးမဟုတ်ဘဲ tab၊ trailing whitespace၊ long-line overflow နှင့် quoted identifier ကို ပိုတိကျစွာညွှန်ပြစေပါသည်။
- Tab ကို spaces ပြောင်းခြင်း၊ trailing whitespace ဖျက်ခြင်း၊ function parentheses ထည့်ခြင်းနှင့် တစ်ခုတည်းသာကျန်သော closing delimiter ထည့်ခြင်းတို့အတွက် safe server quick fixes ထည့်ထားပါသည်။
- Diagnostic `fixIds` metadata နှင့် line-aware malformed function-signature error များ ထည့်ထားသော်လည်း CLI diagnostics compatibility ကို ထိန်းသိမ်းထားပါသည်။

## [2.2.6] — 2026-08-23

### Release engineering
- Filesystem confinement၊ locked-build validation၊ bounded builtin၊ URL parsing၊ test discovery၊ registry-test isolation နှင့် cross-platform compatibility များကို harden လုပ်ပြီး framework အလုပ်မထည့်သွင်းပါ။
- ခွင့်ပြုထားသော remediation branch တွင် `ureq 2.12.1`၊ `url 2.5.8`၊ `idna 1.1.0`၊ `rustls 0.23.40`၊ `rustls-webpki 0.103.15`၊ `rcgen 0.13.2` နှင့် development-only `time 0.3.47` သို့ update လုပ်ထားပြီး strict `cargo-audit 0.22.2` သည် locked crate dependency ၈၇ ခုအပေါ် unresolved advisory သုညခုကို report လုပ်ပါသည်။
- `time 0.3.47` လိုအပ်ချက်ကြောင့် ထုတ်ဝေထားသော source နှင့် CI quality job တွင် Rust 1.88.0 ကို pin လုပ်ထားပါသည်။ Clean commit၊ GitHub CI၊ final preflight နှင့် signed-artifact verification gate များ အောင်မြင်ပြီးမှသာ v2.2.6 ကို ထုတ်ဝေခဲ့ပါသည်။ [Published release](https://github.com/hidecard/zap/releases/tag/v2.2.6) နှင့် [release workflow run](https://github.com/hidecard/zap/actions/runs/32638479414) ကို ကြည့်ရှုနိုင်ပါသည်။

## [2.2.5] — 2026-08-23

### Release engineering
- HTTP request URL invariant များကို deterministic error ဖြင့် ပိုမိုခိုင်မာစေပြီး parser/runtime syntax၊ eager async semantics နှင့် framework မပါဝင်သည့် scope ကို မပြောင်းလဲထားပါ။

## [2.2.4] — 2026-08-23

### Release engineering
- Post-v2.2.3 audit အပြီး language specification နှင့် generic type-check release-gate ရှိ active v2.2.3 references များကို synchronize လုပ်ထားပြီး parser၊ runtime သို့မဟုတ် generic syntax behavior ပြောင်းလဲမှု မရှိပါ။

## [2.2.3] — 2026-08-22

### Release engineering
- Bounded cycle-safe equality၊ object နှင့် EnvFrame borrow propagation၊ panic-free task/frame invariant၊ checked LSP rename scope boundary နှင့် synchronized bilingual documentation များကို ထည့်သွင်းထားပါသည်။

## [2.2.2] — 2026-08-22

### Release engineering
- Checked EnvFrame borrow နှင့် explicit cycle policy reporting ကို ပြီးစီးစေပြီး canonical AST assert/sort/sqrt dispatch ကို ပြန်လည်ထည့်သွင်းကာ standard-library metadata နှင့် VS Code grammar များကို synchronize လုပ်ပြီး full serial native suite နှင့် CI gate များကို အောင်မြင်စေပါသည်။

### v2.2.0 နောက်ပိုင်း corrective cycle
- LSP-SYNC-01၊ LSP-REN-01၊ LSP-INTEROP-01 နှင့် EXT-201 ကို `master` တွင် ပြီးစီးထားသည်။ Standard full-document synchronization နှင့် versioned state၊ file-local scope-aware rename၊ negotiated UTF-8/UTF-16/UTF-32 position၊ strict file URI၊ bounded workspace indexing နှင့် canonical VS Code packaging များ ပါဝင်သည်။ Incremental range change နှင့် cross-file rename ကို support မလုပ်သေးပါ။
- API-301 ကို post-release `master` တွင် ပြီးစီးထားသည်။ Schema-2 `determinism_class` (`pure`၊ `input-deterministic`၊ `runtime-dependent` နှင့် `external-io`)၊ explicit domain/builtin coverage နှင့် compatibility-preserving legacy `deterministic` boolean ကို ထည့်သွင်းထားသည်။
- DOC-401 သည် provenance boundary ကို မှတ်တမ်းတင်သည်။ v2.2.0 သည် [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb) tag commit တွင် immutable ဖြစ်ပြီး ပြင်ဆင်ထားသော behavior ကို v2.2.1 patch အသစ်တွင် ထုတ်ဝေရန် စီစဉ်ထားသည်။ [`POST_V2.2.0_REMEDIATION_MM.md`](docs/POST_V2.2.0_REMEDIATION_MM.md) ကို ကြည့်ပါ။

## [2.2.1] — 2026-08-22

### Corrective release
- v2.2.0 နောက်ပိုင်း LSP synchronization၊ scope-aware file-local rename၊ URI/position/workspace-boundary hardening၊ canonical VS Code package နှင့် schema-2 standard-library determinism taxonomy များကို ထုတ်ဝေထားသည်။
- Explicit limitation များကို ဆက်လက်ထိန်းသိမ်းထားသည်။ LSP synchronization သည် full-text only ဖြစ်ပြီး cross-file rename မရှိပါ။ Async သည် full production reactor မဟုတ်ဘဲ bounded foundation ဖြစ်ပြီး traits/composition သည် design-only ဖြစ်သည်။
- Release scope နှင့် provenance အတွက် [`RELEASE_2.2.1_MM.md`](docs/RELEASE_2.2.1_MM.md) နှင့် [`POST_V2.2.0_REMEDIATION_MM.md`](docs/POST_V2.2.0_REMEDIATION_MM.md) ကို ကြည့်ပါ။

## [2.2.0] — 2026-08-22

### Release engineering
- Audited runtime၊ verification၊ registry၊ standard-library၊ LSP/editor၊ bilingual documentation နှင့် traits/composition RFC milestone များကို ပြီးစီးထားပါသည်။

### Compatibility နှင့် language design
- Canonical AST execution၊ single-inheritance `extends`၊ deterministic async scheduling၊ bounded registry behavior နှင့် ရှိပြီးသား standard-library contract များကို ထိန်းသိမ်းထားပါသည်။
- Reviewed bilingual traits/composition RFC ကို design-only record အဖြစ် ထည့်သွင်းထားသော်လည်း `trait`၊ `interface`၊ `with` နှင့် conflict-resolution syntax အသစ်များသည် v2.2.0 တွင် deferred/unsupported ဖြစ်နေဆဲ ဖြစ်ပါသည်။

### Tooling နှင့် documentation
- Parser/lexer-backed LSP rename၊ didClose cleanup၊ nested/module-aware workspace symbol၊ catalog-driven completion၊ async builtin hover/signature metadata နှင့် validated VS Code asset များကို ထည့်သွင်းထားပါသည်။
- Verified-version metadata နှင့် canonical companion link များပါသော bilingual learner/reference navigation ကို ပြီးစီးထားပါသည်။
## [2.1.14] — 2026-08-21

### Release engineering
- Explicit workspace နှင့် LSP state migration ကို ခိုင်မာစေပြီး cross-platform CI regression အပြီး Windows-compatible line-helper behavior ကို ထိန်းသိမ်းထားပါသည်။

## [2.1.13] — 2026-08-21

### Release engineering
- Workspace confinement နှင့် LSP document ownership ကို explicit state object များသို့ ရွှေ့ပြီး isolation regression များ ထည့်သွင်းထားပါသည်။

## [2.1.12] — 2026-08-22

### Release engineering
- Normal program နှင့် local module များကို canonical AST execution ဖြင့် run လုပ်ပြီး legacy line execution ကို compatibility-only boundary အဖြစ် သတ်မှတ်ထားပါသည်။

## [2.1.11] — 2026-08-21

### Release engineering
- Per-run module-cache၊ import-cycle နှင့် execution-depth isolation အတွက် ပထမဆုံး explicit RuntimeState/ExecutionContext boundary၊ regression tests နှင့် bilingual documentation ကို ထည့်သွင်းထားပါသည်။

### Documentation maintenance
- Burmese learner guide ထဲရှိ v0.x historical reference များနှင့် outdated feature-status claim များကို ပြင်ဆင်ပြီး v2.1.10 contract နှင့် deferred scope များနှင့် ကိုက်ညီအောင် ညှိထားပါသည်။
- English Result propagation example များကို supported `result<any>` annotation အသုံးပြုအောင် ပြင်ဆင်ပြီး v2.1.10 English/Burmese release-note pair ကို documentation consistency gate ထဲ ထည့်သွင်းထားပါသည်။
- Release-note verification reference များကို final CI နှင့် release workflow run များသို့ update လုပ်ထားပါသည်။

## [2.1.10] — 2026-08-21

### Release engineering
- Bilingual documentation consistency validation၊ navigation landing page များနှင့် configurable warm-up၊ threshold gate ပါသော p95 benchmark regression protection ကို ထည့်သွင်းထားပါသည်။

### Documentation consistency နှင့် navigation
- Syntax၊ language specification၊ async boundary၊ generic type-checking design၊ P2 progress နှင့် benchmark policy metadata များအတွက် v2.1.9 bilingual documentation baseline ကို တည်ဆောက်ထားပါသည်။
- English/Burmese documentation navigation landing page များကို ထည့်သွင်းပြီး README နှစ်ခုလုံးမှ link ချိတ်ထားသဖြင့် normative contract၊ verification evidence နှင့် contribution path များကို လွယ်ကူစွာ ရှာဖွေနိုင်ပါသည်။
- Section parity၊ code-fence parity၊ stale-version၊ required-file နှင့် navigation-link check များပါသော `scripts/validate_documentation_consistency.sh` နှင့် positive/negative regression harness ကို ထည့်သွင်းထားပါသည်။

### Benchmark regression ကာကွယ်မှု
- Benchmark aggregation တွင် deterministic p95 column ကို တိုးချဲ့ပြီး `ZAP_BENCH_WARMUPS` မှတစ်ဆင့် configurable warm-up iteration များကို ထည့်သွင်းထားပါသည်။
- `scripts/check_benchmark_regression.sh` မှတစ်ဆင့် mean/p95 threshold comparison ကို ထည့်သွင်းပြီး CI နှင့် release-preflight တွင် enforce လုပ်ထားပါသည်။ Checked-in evidence အဖြစ် `benchmark-results/native-summary.csv` ကို အသုံးပြုထားပါသည်။

## [2.1.9] — 2026-08-21

### Release engineering
- Panic မဖြစ်သော object borrow diagnostics၊ checked field access နှင့် stable ZAP-BORROW-001 error များကို ထည့်သွင်းထားပါသည်။

### Memory borrow safety
- Object field များအတွက် checked `try_borrow`/`try_borrow_mut` accessor များနှင့် `RefCell` panic မဖြစ်စေဘဲ stable `ZAP-BORROW-001` diagnostic ပါသော fail-closed `BorrowError` handling ကို ထည့်သွင်းထားပါသည်။
- Recursive JSON error propagation နှင့် conflicting object borrow၊ structured diagnostic၊ safe object-field access regression များကို ထည့်သွင်းထားပါသည်။

## [2.1.8] — 2026-08-21

### Release engineering
- Cargo၊ CLI output၊ tag၊ bilingual onboarding၊ security metadata၊ release note၊ template နှင့် installer များအကြား release version consistency validation ကို ခိုင်မာစေပြီး CI နှင့် release preflight တွင် drift ဖြစ်ပါက fail-closed ပြုလုပ်ထားပါသည်။

### Release version consistency
- Cargo ကို authoritative source အဖြစ် အသုံးပြုသော version validator၊ dynamic CLI/lockfile/tag check၊ bilingual README archive check၊ security-link check နှင့် hard-coded release-template detection များကို ထည့်သွင်းထားပါသည်။
- Deterministic TSV evidence၊ positive/negative version-drift regression test နှင့် CI/release-preflight enforcement များကို ထည့်သွင်းထားပါသည်။
- `master` ကဲ့သို့ CI branch ref များကို release tag ဟု မှားယွင်းယူဆခြင်းကို ပြင်ဆင်ထားပါသည်။ Implicit tag validation သည် semver ပုံစံရှိသော `v<version>` ref များတွင်သာ အလုပ်လုပ်ပြီး explicit `RELEASE_TAG` တန်ဖိုးများကို ဆက်လက် enforce လုပ်ပါသည်။

## [2.1.7] — 2026-08-21

### Release engineering
- Bilingual specification ownership ကို stable rule ID ၂၇ ခုအထိ ချဲ့ထွင်ပြီး ownership၊ parity၊ replay နှင့် async contract များအတွက် release preflight gate များ ထည့်သွင်းထားပါသည်။

### Cross-platform CI hardening
- Temporary directory name များတွင် test-thread label ထည့်ရာ၌ Windows အတွက် မမှန်ကန်သော `::` path separator မဖြစ်စေရန် sanitize လုပ်ထားပြီး Windows native test matrix regression ကို ပြင်ဆင်ထားပါသည်။
- Registry service ၏ non-blocking listener မှ accept လုပ်ပြီးသော socket များကို request read မတိုင်မီ blocking mode သို့ ပြန်လည်သတ်မှတ်ထားပါသည်။ ထို့ကြောင့် macOS target တွင် တစ်ခါတစ်ရံ empty response မြင်ရသော ပြဿနာကို ကာကွယ်ထားပါသည်။

### Specification ownership hardening
- `SPEC_OWNERSHIP_INDEX.tsv` ကို stable rule ID ၂၇ ခုအထိ ချဲ့ထွင်ပြီး source execution၊ precedence၊ typing၊ functions၊ modules၊ memory၊ deterministic/production async boundary၊ diagnostics၊ registry၊ lockfile၊ JSON/filesystem limits၊ standard-library catalog၊ CLI JSON၊ compatibility policy နှင့် CI enforcement များကို လွှမ်းခြုံထားပါသည်။
- `scripts/validate_spec_ownership.sh` ကို ခိုင်မာစေပြီး missing section၊ missing fixture owner၊ duplicate ID၊ invalid policy value နှင့် လိုအပ်သော semantic domain များ မရှိခြင်းကို reject လုပ်ထားပါသည်။
- အနာဂတ် normative၊ compatibility၊ deprecation နှင့် rejection decision များအတွက် bilingual `COMPATIBILITY_CHANGE_TEMPLATE_EN.md` နှင့် `COMPATIBILITY_CHANGE_TEMPLATE_MM.md` records များကို ထည့်သွင်းထားပါသည်။
- `scripts/release_preflight.sh` ကို deployment validation မတိုင်မီ ownership၊ native/legacy parity၊ fixed-seed replay နှင့် focused async contract gate များ run လုပ်အောင် ချဲ့ထွင်ထားပါသည်။

### Native/legacy parity hardening
- `common`၊ `native-only` နှင့် `rejected` policy class များပါသော versioned six-case native/legacy matrix ကို ထည့်သွင်းထားပါသည်။
- Normalized stdout digest comparison၊ deterministic TSV report၊ migration guidance နှင့် `scripts/test_p001_parity.sh` မှတစ်ဆင့် CI parity artifact gate ကို ထည့်သွင်းထားပါသည်။
- Bilingual `docs/P001_PARITY_MATRIX_EN.md` နှင့် `docs/P001_PARITY_MATRIX_MM.md` documentation များကို ထည့်သွင်းထားပါသည်။

### Replayable verification hardening
- Parser၊ JSON၊ lockfile၊ registry၊ memory နှင့် async boundary များအတွက် fixed-seed `ZAP_CORPUS_SEED` replay ကို ထည့်သွင်းထားပါသည်။
- Durable failure fixture ၂၁ ခု၊ deterministic replay ordering၊ `target/p105-replay.log` ထဲတွင် SHA-256/base64 input evidence နှင့် `scripts/test_p105_layers.sh` မှတစ်ဆင့် CI artifact gate ကို ထည့်သွင်းထားပါသည်။
- Seed၊ fixture ownership၊ replay evidence နှင့် deferred fuzz scope များကို သတ်မှတ်သော bilingual `docs/P105_REPLAY_EN.md` နှင့် `docs/P105_REPLAY_MM.md` documentation များကို ထည့်သွင်းထားပါသည်။

### Async boundary hardening
- Single-threaded executor၊ fixed-worker adapter၊ bounded network/process adapter များ၊ cancellation behavior၊ default limits၊ deferred language-level scheduling/cancellation/timeout နှင့် arbitrary foreign blocking call interrupt မထောက်ပံ့ခြင်းတို့ကို ဖော်ပြသော deterministic `async_capabilities()` builtin နှင့် catalog entry ကို ထည့်သွင်းထားပါသည်။
- Zero/oversized worker၊ task၊ read၊ socket နှင့် process limit များအတွက် typed preflight validation နှင့် queue admission မတိုင်မီ TCP request-size rejection ကို ထည့်သွင်းထားပါသည်။
- Linux x86_64၊ Windows x86_64 နှင့် macOS ARM64 အတွက် reproducible focused async matrix နှင့် target-named CI log artifact များကို ထည့်သွင်းပြီး process၊ file၊ socket၊ deadline၊ cancellation နှင့် output-limit behavior များကို လွှမ်းခြုံထားပါသည်။
- Runtime နှင့် AST regression coverage များကို ထည့်သွင်းပြီး English/Burmese async runtime နှင့် standard-library documentation များကို synchronize လုပ်ထားပါသည်။

### Memory contract hardening
- Live object၊ allocation၊ deallocation နှင့် runtime-limit fields များပါသော bounded `memory_stats()` diagnostic ကို ထည့်သွင်းထားပါသည်။
- Public builtin boundary များတွင် text၊ list၊ map၊ object၊ Result/Option နှင့် Future value များအတွက် cycle-safe validation နှင့် deterministic memory-limit error များကို ထည့်သွင်းထားပါသည်။
- Public weak reference နှင့် tracing collection များကို လက်ရှိတွင် unsupported/deferred အဖြစ် ထိန်းသိမ်းပြီး single-threaded ownership boundary ကို documentation တွင် ရှင်းလင်းထားပါသည်။

### Structured diagnostics hardening
- CLI JSON နှင့် LSP output နှစ်ခုလုံးတွင် stable `ZAP-*` codes၊ severity၊ deterministic notes၊ ရွေးချယ်နိုင်သော help၊ source locations နှင့် TypeError parity regression assertions ပါဝင်သည့် တည်ငြိမ်သော structured diagnostic contract ကို ထည့်သွင်းထားပါသည်။
- Diagnostic fields နှင့် compatibility rules များအတွက် bilingual `docs/DIAGNOSTIC_MODEL_EN.md` နှင့် `docs/DIAGNOSTIC_MODEL_MM.md` documentation များကို ထည့်သွင်းထားပါသည်။

## [2.1.6] — 2026-08-21

### Release engineering
- TC-001–TC-012 conformance coverage၊ pinned Rust quality gates၊ Clippy compatibility နှင့် platform အစုံ release packaging verification များကို ပိုမိုတင်းကျပ်ထားပါသည်။

### Type checking နှင့် CI hardening
- English type-checking conformance matrix ကို အတည်ပြုထားသော v2.1.5 TC-001 မှ TC-012 baseline နှင့် ကိုက်ညီအောင် ညှိပြီး advanced generic inference ကို deferred scope အဖြစ် မှတ်တမ်းတင်ထားပါသည်။
- TC-001 မှ TC-010 conformance fixtures များနှင့် CLI/LSP `TypeError` diagnostic-parity regression အတွက် CI gate အမည်သီးခြားများ ထည့်သွင်းထားပါသည်။
- Local နှင့် CI validation များ reproducible ဖြစ်စေရန် repository Rust toolchain ကို `rustfmt` နှင့် `clippy` components ပါသော 1.75.0 သို့ pin လုပ်ထားပါသည်။
- Type-check၊ LSP parity၊ formatting နှင့် CI-contract preflight စစ်ဆေးမှုများကို ထပ်ခါတလဲလဲ လုပ်နိုင်ရန် `scripts/validate_v216_preflight.sh` ကို ထည့်သွင်းထားပါသည်။
- `pipefail` အောက်တွင် အောင်မြင်သော tar-entry check ကို false failure အဖြစ် မမှတ်တမ်းတင်စေရန် published-release archive verification pipe handling ကို ပြင်ဆင်ထားပါသည်။

## [2.1.5] — 2026-08-21

### Release engineering
- Signed release publication၊ provenance verification နှင့် platform အစုံ reproducible packaging hardening များကို ခိုင်မာစေပါသည်။

## [2.1.4] — 2026-08-21

### Release engineering
- Windows clean-profile installer verification တွင် PowerShell read-only HOME variable collision ဖြစ်မှုကို ပြင်ဆင်ထားပါသည်။

## [2.1.3] — 2026-08-21

### Release engineering
- Windows deterministic archive ၏ zap/ root layout နှင့် release verification မကိုက်ညီမှုကို ပြင်ဆင်ထားပါသည်။

## [2.1.2] — 2026-08-21

### Release engineering
- Cross-platform deterministic archive packaging နှင့် release-workflow reproducibility hardening ကို ပြင်ဆင်ထားပါသည်။

## [2.1.1] — 2026-08-21

### Release engineering
- TC-006 မှ TC-012 အထိ conformance coverage၊ structured diagnostic stability နှင့် CLI/LSP TypeError location parity ကို ခိုင်မာအောင် ပြင်ဆင်ထားပါသည်။

### Package reliability

- Legacy lockfile များကို conservative အတိုင်း migrate လုပ်ရန် `zap lock-migrate [dir]` command အသစ် ထည့်သွင်းထားပါသည်။
- v1 lockfile compatibility ကို ဆက်လက်ထိန်းသိမ်းထားပြီး registry version သို့မဟုတ် checksum များကို မခန့်မှန်းပါ။ Registry-backed project များအတွက် verified registry metadata ရှိမှသာ migration လုပ်ပါသည်။
- `zap install` သည် transitive package များအပါအဝင် resolved registry graph တစ်ခုလုံးကို deterministic `name@version` order ဖြင့် ပြသပြီး ရှိပြီးသား dependency-count prefix ကို ဆက်လက်ထိန်းသိမ်းထားပါသည်။
- Transitive resolution၊ cache verification၊ stable install output၊ transitive artifact ပျောက်ဆုံးမှု၊ cached checksum မကိုက်ညီမှုနှင့် မပြည့်စုံသော v2 lockfile များအတွက် deterministic diagnostics ပါသော offline nested-registry integration fixtures များ ထည့်သွင်းထားပါသည်။
- Canonical project lockfile မှ keep entries များကို ရယူသော `zap registry gc [--dry-run] [dir]` ကို ထည့်သွင်းထားပါသည်။ Dry-run တွင် cache မပြောင်းဘဲ stale နှင့် temporary candidate များကို ပြသပြီး ပုံမှန် run တွင် candidate များကို deterministic lexical order ဖြင့် ဖယ်ရှားပါသည်။
- Transport နှင့် registry-service failure coverage များ ထည့်သွင်းပြီး insecure HTTP rejection၊ malformed remote-index diagnostic နှင့် non-2xx fetch/publish response များအတွက် deterministic HTTP-status error များကို စစ်ဆေးပြီးဖြစ်သည်။
- v2.1-B trusted-registry enforcement အပိုင်းကို ပြီးစီးပြီး canonical origin normalization၊ bounded deterministic allowlist၊ persistent `zap registry trust list|add|remove` commands၊ origin-scoped bearer credential၊ bounded `zap registry credential list|set|remove` management၊ token validation/redaction၊ stable `ZAP-REG-AUTH-001`/`002`/`003` diagnostic၊ credential-aware remote index loading နှင့် dependency resolution၊ registry fetch/cache/publish လမ်းကြောင်းများတွင် effective-policy check များ၊ successful authenticated HTTPS fetch/publish အတွက် Rust 1.75-compatible local TLS fixture များကို ထည့်သွင်းထားပါသည်။ Final v2.1.0 release integration လည်း ပြီးစီးပါပြီ။

### Type checking နှင့် conformance

- TC-012 generic syntax ကို v2.1 implemented baseline အဖြစ် လက်ခံထားပါသည်။ `list<T>`၊ `map<K, V>`၊ `option<T>` နှင့် `result<T>` ကို ထောက်ပံ့ပြီး malformed form များကို ဆက်လက် reject လုပ်ပါသည်။ User-defined generic declaration နှင့် advanced inference များကို design record တွင် explicit deferred scope အဖြစ် သတ်မှတ်ထားပါသည်။

- Explicit `is_option_none(value)` else-branch narrowing ကို ထည့်သွင်းထားပါသည်။ True branch တွင် `option<T>` ကို ထိန်းသိမ်းပြီး sound ဖြစ်သောအခါ else branch တွင် payload type ကို အသုံးပြုနိုင်ပါသည်။
- Guard ပါသော `while` body များအတွက် loop-boundary narrowing ကို ထည့်သွင်းထားပါသည်။ Narrowed payload ကို loop အတွင်း အသုံးပြုနိုင်ပြီး loop ပြီးနောက် မူလ wrapper type ကို ပြန်လည်ထားရှိသဖြင့် reassignment နှင့် post-loop use များသည် type-safe ဖြစ်ပါသည်။
- Loop အတွင်း option payload အသုံးပြုမှုနှင့် loop ပြီးနောက် wrapper ပြန်လည်ရရှိမှုအတွက် permanent TC-006 conformance coverage ကို ထည့်သွင်းထားပါသည်။
- `if ... then ... else ...` control-flow expression များအတွက် type checking ကို ထည့်သွင်းထားပါသည်။ Condition သည် `bool` ဖြစ်ရမည်၊ branch result type နှစ်ခု ကိုက်ညီရမည်၊ မကိုက်ညီပါက structured `TypeError` ဖြင့် reject လုပ်ပါသည်။
- `zap check --json` ဖြင့် compatible branch၊ မကိုက်ညီသော branch result နှင့် bool မဟုတ်သော condition များကို စစ်ဆေးသည့် permanent TC-009 conformance fixtures များ ထည့်သွင်းထားပါသည်။ ထို့အပြင် conditional type error များအတွက် `ok`၊ `kind`၊ `file`၊ `line`၊ `column`၊ `message` နှင့် `error` fields များ တည်ငြိမ်စွာ ထွက်ရှိကြောင်း L3 regression ဖြင့် စစ်ဆေးထားပါသည်။
- `option<T>` နှင့် `result<T>` အတွက် alias assignment ဖြတ်သန်းသည့် wrapper preservation နှင့် reassignment ပြီးနောက် narrowing fact invalidation ကို စစ်ဆေးသည့် permanent TC-010 alias-narrowing fixtures များ ထည့်သွင်းထားပါသည်။
- Bilingual type-checking conformance matrix များတွင် TC-006 loop-boundary coverage နှင့် TC-012 generic syntax ကို implemented baseline evidence အဖြစ် မှတ်တမ်းတင်ထားပါသည်။ Future generic declaration နှင့် advanced inference များကိုသာ deferred ထားရှိပါသည်။
- L4 LSP diagnostic regression နှင့် shared source-diagnostic bridge ကို ထည့်သွင်းထားပါသည်။ CLI နှင့် LSP type error များသည် တူညီသော `TypeError` code၊ source-location semantics နှင့် normalized message ကို အသုံးပြုပါသည်။

### Security နှင့် release hardening

- Canonical registry URL normalization၊ adversarial URL rejection၊ trusted-registry နှင့် credential scope boundary၊ bounded allowlist behavior၊ longest-prefix token selection၊ token validation နှင့် secret redaction များအတွက် deterministic security-property corpus tests များ ထည့်သွင်းထားပါသည်။
- Signed registry-index mutation coverage ကို ထည့်သွင်းပြီး malformed နှင့် byte-mutated input များကို `catch_unwind` ဖြင့် run ကာ parser panic မဖြစ်ရ၊ ပြုပြင်ထားသော index ကို လက်မခံရ ဟူသော အခြေအနေကို စစ်ဆေးထားပါသည်။
- Native test suite နှင့်အတူ သီးခြား `security_property` CI step ကို ထည့်သွင်းထားပါသည်။ Formatting၊ Cargo check၊ native test 248 ခု၊ CI strict Clippy၊ cross-platform build နှင့် v2.1.0 release checksum gate များကို ဆက်လက် enforce လုပ်ထားပါသည်။
- Filesystem builtin များအတွက် runtime workspace confinement ကို ထည့်သွင်းထားပါသည်။ Relative နှင့် absolute path များကို active project workspace အပေါ် resolve လုပ်ပြီး parent traversal များကို reject လုပ်ကာ ရှိပြီးသား symlink များကို containment check မပြုမီ canonicalize လုပ်သဖြင့် workspace အပြင်သို့ ဖတ်ခြင်းနှင့် ရေးခြင်း မပြုနိုင်ပါ။
- Parent traversal နှင့် အပြင်ဖိုင်သို့ ညွှန်သော symlink များအတွက် adversarial filesystem regression coverage ထည့်သွင်းပြီး သီးခြား `filesystem_builtins` CI corpus step ကိုလည်း ထည့်သွင်းထားပါသည်။
- Huge numeric literal၊ unterminated string၊ unknown punctuation၊ malformed indentation နှင့် delimiter၊ broken nested syntax၊ panic-free repeated parsing နှင့် monotonic token span များအတွက် deterministic lexer နှင့် parser corpus coverage ထည့်သွင်းထားပါသည်။
- Malformed tagged variant၊ oversized integer၊ recursive malformed input၊ deterministic conversion နှင့် panic-free rejection များအတွက် JSON conversion security coverage ထည့်သွင်းထားပါသည်။
- Unsupported version၊ incomplete သို့မဟုတ် duplicate field၊ invalid escape၊ traversal-like package name၊ strict quoted value၊ deterministic rejection နှင့် panic-free parsing များအတွက် lockfile security coverage ထည့်သွင်းထားပါသည်။
- Complete native suite နှင့် သီးခြားအနေဖြင့် `adversarial_corpus`၊ `malformed_program_corpus`၊ `json_security_corpus`၊ `malformed_lockfile_corpus` နှင့် `lockfile_quoted_values` များကို run သည့် `parser JSON lockfile corpus` CI gate ကို ထည့်သွင်းထားပါသည်။
- Unix release packaging ကို reproducible ဖြစ်စေရန် archive order၊ timestamp၊ ownership၊ numeric ownership နှင့် gzip metadata များကို normalize လုပ်ထားပါသည်။ Upload မပြုမီ CI တွင် Unix archive တစ်ခုချင်းစီကို ပြန်လည် build လုပ်ကာ byte-for-byte တူညီမှုကို လိုအပ်စေပါသည်။
- Windows `Compress-Archive` packaging အစား sorted slash-separated file entry များ၊ Unix epoch entry timestamp၊ stable compression setting များနှင့် deterministic .NET ZIP writer ကို အသုံးပြုထားပါသည်။ Upload မပြုမီ archive ကို ဒုတိယအကြိမ် ပြန်လည် build လုပ်ကာ byte-for-byte တူညီမှု၊ archive contents နှင့် SHA-256 checksum များကို ဆက်လက်စစ်ဆေးပါသည်။
- Clean Unix home နှင့် Windows user profile များအတွက် cross-platform installer verification ကို ထည့်သွင်းထားပါသည်။ Release archive များတွင် uninstall script များပါဝင်ပြီး သက်ဆိုင်ရာ platform တွင် installation၊ version reporting၊ executable launch၊ reinstall/upgrade၊ uninstall cleanup၊ archive contents နှင့် SHA-256 metadata များကို CI ဖြင့် စစ်ဆေးပါသည်။
- Oversized typed-JSON input၊ runtime category မကိုက်ညီမှု၊ Unicode index boundary၊ duration overflow၊ structured-log limit နှင့် oversized atomic-write content များအတွက် `stdlib_security_corpus` adversarial test gate ကို ထည့်သွင်းထားပါသည်။ Case တစ်ခုချင်းစီကို `catch_unwind` အောက်တွင် ထပ်မံ run ပြီး panic မဖြစ်ဘဲ rejection result တည်ငြိမ်ရမည်ဟု စစ်ဆေးပါသည်။

### Async နှင့် tooling

- LSP တွင် `textDocument/documentSymbol` support ထည့်သွင်းပြီး function နှင့် class body များအတွင်း nested-symbol indexing ကို recursive အနေဖြင့် ထည့်သွင်းထားသည်။ Symbol များတွင် deterministic range၊ selection range၊ detail နှင့် child declaration များ ပါဝင်ပြီး class scope နှင့် function scope နှစ်ခုလုံးအတွက် regression coverage ရှိသည်။
- Explicit local import များအတွက် module-aware workspace-symbol indexing ကို ထည့်သွင်းထားသည်။ Indexer သည် imported file များကို လုံခြုံစွာ canonicalize နှင့် bounded-read ပြုလုပ်ကာ traversal ဆန်သော path များကို reject လုပ်ပြီး nested module များကို duplicate မဖြစ်အောင် ထိန်းသိမ်းပါသည်။ Editor တွင် မဖွင့်ထားသော local package file များမှ symbol များကို deterministic အတိုင်း ပြန်ပေးပြီး safe discovery နှင့် traversal exclusion အတွက် regression coverage ရှိပါသည်။
- Production-oriented blocking work အတွက် bounded `ThreadedRuntime` standard-library adapter ကို ထည့်သွင်းထားပါသည်။ Fixed worker scheduling၊ task admission limit၊ cross-thread join wake-up၊ panic-to-error conversion နှင့် capped asynchronous regular-file read များကို ပံ့ပိုးပါသည်။ Parallel execution၊ admission bound၊ wake-up behavior၊ panic propagation နှင့် file-size limit များအတွက် regression coverage ရှိပြီး security contract ကို bilingual async/LSP guide တွင် မှတ်တမ်းတင်ထားပါသည်။
- `ThreadedRuntime` ပေါ်တွင် bounded non-blocking TCP request/response exchange နှင့် asynchronous process execution ကို ထည့်သွင်းထားပါသည်။ Socket operation များတွင် deadline၊ non-blocking polling နှင့် response cap များ အသုံးပြုထားပြီး process operation များတွင် null stdin၊ သီးခြား stdout/stderr drain၊ hard deadline၊ output cap နှင့် structured status report များ ပါဝင်ပါသည်။ Socket round trip၊ oversized response၊ cross-platform process output၊ capped output နှင့် deadline failure များအတွက် regression coverage ရှိပါသည်။ Arbitrary blocking system call များကို forced cancellation ပြုလုပ်ခြင်းသည် ဤ adapter contract ၏ အပြင်ဘက်တွင် ရှိနေဆဲဖြစ်ပါသည်။
- Cancellation-aware child process execution ကို ထည့်သွင်းထားပြီး cancellation သို့မဟုတ် deadline ရောက်ပါက child ကို terminate လုပ်ကာ bounded output ကို drain ပြုလုပ်ပြီး deterministic status/error result ဖြင့် resolve လုပ်ပါသည်။ Arbitrary foreign blocking call များကို interrupt လုပ်နိုင်သည်ဟု မဆိုလိုပါ။ `zap registry serve` ဖြင့် controlled authenticated loopback registry service ကို ထည့်သွင်းထားပြီး bounded HTTP parsing၊ safe in-root path၊ atomic signed-index persistence၊ managed shutdown နှင့် unauthorized၊ traversal၊ malformed၊ oversized request များအတွက် deterministic rejection များ ပါဝင်ပါသည်။ Public deployment အတွက် TLS termination၊ ingress policy၊ external supervision၊ sandbox၊ quota နှင့် egress control များသည် သီးခြားလိုအပ်ချက်များအဖြစ် ကျန်ရှိပါသည်။
- Deterministic `AsyncRuntime::spawn_joinable(future)` task submission၊ `JoinHandle<T>::is_ready()` နှင့် future အဖြစ် output join လုပ်နိုင်မှုတို့ကို ထည့်သွင်းထားပါသည်။
- Joinable task admission အချိန် `SpawnError::TaskLimitReached` ကို propagate လုပ်ပြီး runtime task order၊ poll budget၊ Rust 1.75 compatibility နှင့် worker thread မဖန်တီးသော execution model ကို ထိန်းသိမ်းထားပါသည်။
- Joined output အောင်မြင်မှုနှင့် task-limit error များအတွက် regression coverage ထည့်သွင်းပြီး ပထမ async slice ကို `docs/ASYNC_RUNTIME_EN.md` နှင့် `docs/ASYNC_RUNTIME_MM.md` တွင် မှတ်တမ်းတင်ထားပါသည်။
- `AsyncRuntime::spawn_joinable_cancellable(future)` ကို ထည့်သွင်းထားပြီး `CancellationToken` ပြန်ပေးကာ cancellation ဖြစ်သော join များကို inner future ကို ဆက်မ poll လုပ်ဘဲ `JoinError::Cancelled` ဖြင့် resolve လုပ်ပါသည်။
- `timeout_ticks(future, ticks)` ကို ထည့်သွင်းထားပြီး wall-clock time မဟုတ်ဘဲ executor poll အရေအတွက်အပေါ် အခြေခံ၍ `TimeoutError` ကို propagate လုပ်ပါသည်။ Cancellation၊ timeout failure နှင့် အချိန်မကုန်မီ completion လမ်းကြောင်းများအတွက် regression tests ထည့်သွင်းထားပါသည်။
- `spawn_joinable_result(future)` နှင့် `spawn_joinable_result_cancellable(future)` တို့ကို ထည့်သွင်းပြီး `TaskJoinError::Failed(E)` ဖြင့် typed task failure များကို ထိန်းသိမ်း propagate လုပ်ပါသည်။ Inner future ကို မ poll မီ cancellation ကို စစ်ဆေးပြီး repeated join များကို `AlreadyJoined` ဖြင့် ပြတ်သားစွာ ပြန်ပေးပါသည်။ Typed failure၊ cancellation precedence နှင့် repeated join များအတွက် regression coverage နှင့် bilingual async runtime guide update များ ထည့်သွင်းထားပါသည်။
- Evaluator နှင့် legacy expression path နှစ်ခုလုံးတွင် language-level task facade builtins `spawn`၊ `task_join` နှင့် `task_is_ready` များကို ထည့်သွင်းထားပါသည်။ Eager Future boundary၊ strict arity/type diagnostic၊ async spawn/readiness/join behavior နှင့် invalid-input regression များကို bilingual async runtime နှင့် async/LSP guides များတွင် မှတ်တမ်းတင်ထားပါသည်။
- Formatter၊ LSP နှင့် VS Code tooling များကို finalized async task vocabulary နှင့် synchronize လုပ်ထားပါသည်။ LSP completion တွင် `spawn`၊ `task_join` နှင့် `task_is_ready` descriptions များ ပါဝင်ပြီး TextMate grammar တွင် builtins အဖြစ် highlight လုပ်ကာ extension smoke validation သည် grammar drift ဖြစ်ပါက reject လုပ်ပါသည်။

### Standard library

- Symlink-safe metadata မှ portable `kind`၊ byte `size` နှင့် `readonly` fields များကို ပြန်ပေးသော `file_metadata(path)` ကို ထည့်သွင်းထားပါသည်။
- Bounded `atomic_write(path, content)` ကို ထည့်သွင်းထားပြီး destination နှင့် directory တူ temporary file ကို ရေးသား၊ sync ပြုလုပ်ပြီး rename ဖြင့် commit လုပ်ကာ failure ဖြစ်ပါက temporary file ကို cleanup လုပ်ပါသည်။
- လက်ရှိ 8 MiB JSON limit အောက်တွင် decoded runtime category ကို deterministic စစ်ဆေးပေးသော `from_json_typed(source, expected)` ကို ထည့်သွင်းထားပါသည်။
- UTF-8 byte များအစား Unicode scalar value များဖြင့် index လုပ်သော Unicode-safe `char_at`၊ `substring` နှင့် `codepoints` APIs များကို stable bounds diagnostics နှင့်အတူ ထည့်သွင်းထားပါသည်။
- Bounded output နှင့် stable runtime errors ပါသော deterministic collection helpers `entries(map)` နှင့် `enumerate(list)` များကို ထည့်သွင်းထားပါသည်။
- UTC time API များကို ထည့်သွင်းထားပြီး `utc_now()` သည် seconds နှင့် millisecond timestamp များကို ပြန်ပေးကာ `duration_parts(milliseconds)` နှင့် `duration_between(end_millis, start_millis)` သည် sign ထိန်းသိမ်းထားသော checked decomposition နှင့် overflow diagnostic များကို ပေးပါသည်။
- UTC timestamp consistency၊ positive/negative duration နှင့် invalid input များအတွက် focused time regression tests များ ထည့်သွင်းပြီး API documentation ကို `docs/STDLIB_TIME_EN.md` နှင့် `docs/STDLIB_TIME_MM.md` တွင် ရေးသားထားပါသည်။
- Deterministic structured logging builtins များကို ထည့်သွင်းထားပါသည်။ `log_record(level, message, fields)` သည် validation ပြီးသော map ကို ပြန်ပေးပြီး `log_json(level, message, fields)` သည် field name များကို စီထားသော canonical JSON ကို ပြန်ပေးပါသည်။
- Structured logging အတွက် message 8 KiB၊ field 64 ခု၊ field-name 256 bytes နှင့် encoded output 64 KiB limits များ သတ်မှတ်ထားပြီး ordering၊ လက်ခံသော level၊ validation error နှင့် safety limit များကို regression tests ဖြင့် စစ်ဆေးထားပါသည်။ API documentation ကို `docs/STDLIB_LOGGING_EN.md` နှင့် `docs/STDLIB_LOGGING_MM.md` တွင် ထည့်သွင်းထားပါသည်။


## [2.0.4] — 2026-08-20

### Package reliability

- Registry-backed lockfile များကို version 2 သို့ တိုးချဲ့ပြီး ရွေးချယ်ထားသော package version၊ source နှင့် SHA-256 checksum များအတွက် deterministic `[resolved]` entries များ ထည့်သွင်းထားပါသည်။
- `zap lock` နှင့် `zap update` သည် resolved transitive packages များကို lockfile တွင် မှတ်တမ်းတင်ပြီး `zap install` သည် pinned graph တစ်ခုလုံးကို ပြန်လည် resolve/verify လုပ်ပါသည်။
- ရှိပြီးသား v1 lockfile များနှင့် compatibility ထိန်းသိမ်းထားပြီး offline cache reuse နှင့် checksum-pinned install integration test များ ထည့်သွင်းထားပါသည်။

### Security audit remediation

- `ZAP_UNTRUSTED=1` restricted mode ထည့်သွင်းပြီး filesystem၊ environment၊ process၊ network၊ local HTTP serving နှင့် local registry-source capability များကို default deny လုပ်ထားပါသည်။
- Loopback၊ private၊ link-local၊ unspecified၊ broadcast၊ IPv6 unique-local နှင့် IPv6 link-local destination များအတွက် SSRF ကာကွယ်မှု ထည့်သွင်းပြီး automatic HTTP redirect များကို restricted mode တွင် ပိတ်ထားပါသည်။
- HTTP request body limit၊ hard child-process deadline နှင့် timeout kill behavior များ ထည့်သွင်းပြီး capability denial၊ private destination နှင့် oversized request body regression tests များ ရေးသားထားပါသည်။
- OS-level sandbox၊ least-privilege deployment၊ resource quota နှင့် network egress control များ လိုအပ်နေသေးကြောင်း documentation တွင် ရှင်းလင်းဖော်ပြထားပါသည်။

### Documentation နှင့် editor integration

- Main README တွင် v2.0.4 installation links၊ current project status၊ security-mode note နှင့် official VS Code Marketplace v0.5.0 installation လမ်းညွှန်များကို update လုပ်ထားပါသည်။
- Extension manifest နှင့် documentation များကို Marketplace publisher `ArkarYan` နှင့် synchronize လုပ်ပြီး `code --install-extension ArkarYan.zap-language-support` command ကို ထည့်သွင်းထားပါသည်။

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
