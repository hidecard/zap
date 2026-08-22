# Zap v2.2.0 Release Notes

**အတည်ပြုထားသော version:** v2.2.0
**Release date:** 2026-08-22
**Release line:** Stable 2.2.x

## အနှစ်ချုပ်

Zap v2.2.0 သည် M3-LSP-01၊ M3-DOC-01 နှင့် M4-RFC-01 အထိ ကျန်ရှိသော audit roadmap milestone များကို ပြီးစီးစေပါသည်။ Editor semantic behavior၊ documentation navigation၊ standard-library stability policy၊ reproducible verification၊ registry safety၊ runtime state isolation များကို ခိုင်မာစေပြီး canonical AST execution boundary နှင့် deterministic async runtime ၏ explicit production-I/O limitation များကို ဆက်လက်ထိန်းသိမ်းထားပါသည်။

ဤ release ကို release candidate သည် repository quality gate နှင့် version-consistency check များကို အောင်မြင်ပြီးနောက် verified `master` commit မှ ထုတ်ဝေထားပါသည်။ Supported release target များမှာ Linux x86_64၊ Windows x86_64 နှင့် macOS ARM64 ဖြစ်နေဆဲ ဖြစ်ပါသည်။

## Runtime နှင့် language foundation များ

Native runtime သည် canonical source → lexer → AST parser → evaluator pipeline ကို ဆက်လက်အသုံးပြုသည်။ Per-run `ExecutionContext` နှင့် `RuntimeState` များက workspace root၊ module cache၊ import-cycle tracking၊ execution depth၊ logical memory/task/output budget၊ object-store counter နှင့် parent-linked closure frame များကို ခွဲခြားထားသည်။ First-class callable value နှင့် executor-backed `ScheduledFuture` language scheduling ကို ဆက်လက်အသုံးပြုနိုင်သည်။

Async language boundary တွင် cooperative `task_cancel`၊ poll-budget `task_join_timeout`၊ deterministic `Cancelled` နှင့် `TimedOut` diagnostic၊ task admission၊ readiness observation နှင့် reset isolation များ ပါဝင်သည်။ Runtime သည် production I/O reactor မဟုတ်ဘဲ deterministic language scheduler ဖြစ်နေဆဲဖြစ်သည်။ Blocking work၊ external process interruption၊ socket readiness နှင့် supervision များကို သီးခြား async boundary နှင့် deployment contract များက သတ်မှတ်သည်။

## Registry၊ verification နှင့် benchmark hardening

Registry transport သည် client read နှင့် response size ကို bound လုပ်ပြီး partial/chunked body များကို support လုပ်သည်။ Invalid သို့မဟုတ် truncated `Content-Length` declaration များကို reject လုပ်ပြီး slow-peer failure များကို normalize လုပ်သည်။ Bounded replay job၊ native platform matrix၊ deterministic archive check၊ target-named log၊ benchmark provenance sidecar၊ variance field နှင့် registry TCP fixture များကို CI နှင့် release preflight ထဲ ချိတ်ဆက်ထားသည်။

Cargo package version သည် single source of truth ဖြစ်နေသည်။ Release validator သည် `native/Cargo.toml`၊ `native/Cargo.lock` ထဲရှိ `zap-native` entry၊ CLI output၊ tag၊ changelog များ၊ README နှစ်ခု၊ security metadata၊ type-check matrix၊ release note နှင့် installer/version-agnostic policy surface များကို စစ်ဆေးသည်။

## Standard-library stability policy

M3-STDLIB-01 သည် public domain ၁၂ ခုနှင့် catalog ထဲရှိ builtin တစ်ခုချင်းစီအတွက် machine-readable catalog ကို ပေးသည်။ Record တစ်ခုချင်းစီတွင် stability၊ introduction release၊ deprecation window၊ semantic-versioning policy၊ supported target၊ input/output limit၊ timeout/error behavior နှင့် determinism များကို ဖော်ပြထားသည်။ English/Burmese policy pair နှင့် catalog regression contract များကို CI နှင့် release preflight တွင် ထည့်သွင်းထားသည်။

## LSP နှင့် VS Code semantic parity

M3-LSP-01 သည် `zap lsp` ကို parser/lexer-backed rename edit၊ didClose document cleanup၊ nested/module-aware workspace symbol၊ catalog-driven completion နှင့် asynchronous builtin များအတွက် hover/signature metadata များဖြင့် တိုးချဲ့ထားသည်။ Repository ထဲတွင် catalog နှင့် ကိုက်ညီသော TextMate grammar၊ language configuration နှင့် extension manifest ပါဝင်သည်။ `scripts/validate_vscode_assets.py` နှင့် `scripts/test_lsp_semantic_parity.sh` များသည် CI နှင့် release preflight တွင် editor/catalog parity ကို စစ်ဆေးသည်။

Rename edit များသည် lexer span ကို ထိန်းသိမ်းပြီး string literal များကို ပြန်မရေးပါ။ Workspace symbol state သည် session ပိုင်ဖြစ်ပြီး ပိတ်သည့် document များကို index မှ ဖယ်ရှားသည်။ ထို behavior များကို focused native LSP regression များဖြင့် စစ်ဆေးထားသည်။

## Bilingual documentation navigation

M3-DOC-01 သည် English/Burmese audience split ကို ပြီးစီးစေပါသည်။ Documentation hub များတွင် learner၊ language user၊ package author၊ runtime maintainer၊ tooling contributor၊ deployment/security operator၊ release operator နှင့် language designer များအတွက် explicit လမ်းကြောင်းများ ပါဝင်သည်။ Learner၊ syntax၊ specification၊ standard-library၊ package-author၊ runtime၊ memory၊ deployment၊ security၊ tooling နှင့် release document များတွင် verified-version metadata နှင့် canonical companion link များ ပါဝင်သည်။

Documentation consistency validator သည် ရှိပြီးသား contract pair များအပြင် bilingual traits RFC pair ကိုပါ စစ်ဆေးသည်။ Repository-relative navigation၊ section parity၊ code-fence parity၊ stale-version detection၊ required file နှင့် README navigation link များကို regression test လုပ်ထားသည်။

## Traits နှင့် composition RFC

M4-RFC-01 သည် design-only milestone ဖြစ်သည်။ Bilingual RFC တွင် composition နှင့် single inheritance နှိုင်းယှဉ်ချက်၊ required/provided method၊ method lookup/visibility၊ missing/conflicting implementation diagnostic၊ inheritance migration၊ hybrid static/dynamic dispatch၊ rejected alternative၊ package compatibility impact နှင့် implementation gate များကို သတ်မှတ်ထားသည်။

> **Compatibility ဆုံးဖြတ်ချက်:** v2.2.0 တွင် `trait`၊ `interface`၊ `with` သို့မဟုတ် conflict-resolution syntax အသစ်များကို implementation မလုပ်သေးသကဲ့သို့ supported ဟုလည်း မကြေညာပါ။ ရှိပြီးသား `class` နှင့် single `extends` behavior များ မပြောင်းပါ။ အနာဂတ် implementation အတွက် reviewed RFC၊ specification ownership၊ bilingual contract၊ conformance fixture နှင့် explicit version decision များ လိုအပ်သည်။

## Compatibility boundary

ရှိပြီးသား `.zp` program၊ single-inheritance class၊ canonical AST execution၊ legacy compatibility-only line-bodied function record၊ deterministic async scheduling၊ registry contract နှင့် standard-library behavior များကို ၎င်းတို့၏ specification နှင့် stability record များအတိုင်း ဆက်လက်ထိန်းသိမ်းထားသည်။ RFC example များကို supported syntax အဖြစ် တိတ်တဆိတ်မပြောင်းပါ။ Allocator-level memory measurement၊ tracing collection၊ multi-thread-safe task state သို့မဟုတ် external production deployment ကိုလည်း မကတိပြုပါ။

## Verification

Release candidate သည် pinned toolchain ဖြင့် Rust formatting၊ `-D warnings` ပါ strict Clippy၊ full native all-target/all-feature test suite၊ M3-LSP-01 semantic-parity harness၊ documentation consistency နှင့် positive/negative regression harness၊ specification ownership validation၊ benchmark နှင့် registry contract gate များ၊ `git diff --check` တို့ကို အောင်မြင်ခဲ့သည်။ Expected version `2.2.0` ဖြင့် release-version validator အောင်မြင်ပြီး required-file၊ contract၊ deployment-policy နှင့် version check များပါသော release preflight ကို publication မတိုင်မီ အောင်မြင်ခဲ့သည်။

## ကိုးကားရန်

[1]: ../README_MM.md — Burmese project status၊ installation၊ architecture နှင့် release asset များ။
[2]: ../README.md — English project status နှင့် release guidance။
[3]: DOCUMENTATION_NAVIGATION_MM.md — Burmese audience နှင့် contract navigation။
[4]: LANGUAGE_SPEC_MM.md — Canonical language semantics နှင့် compatibility ownership။
[5]: STDLIB_POLICY_MM.md — Public standard-library stability policy။
[6]: ASYNC_LSP_MM.md — Async နှင့် LSP boundary contract။
[7]: TRAITS_RFC_MM.md — M4-RFC-01 traits နှင့် composition design record။
[8]: ../CHANGELOG_MM.md — Burmese release history အပြည့်အစုံ။
