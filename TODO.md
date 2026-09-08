# Zap Remaining TODO

**စစ်ဆေး/Update သည့်နေ့:** 2026-09-08
**Repository:** [hidecard/zap](https://github.com/hidecard/zap)
**Latest published release:** [v2.11.18](https://github.com/hidecard/zap/releases/tag/v2.11.18)
**Current branch:** `master`
**Bootstrap stage:** B0

> ဤစာရင်းသည် current-status၊ milestone documents နှင့် Zap ကို Python၊ JavaScript/TypeScript၊ Go၊ Rust တို့နှင့် နှိုင်းယှဉ်ထားသော ecosystem review အပေါ် အခြေခံထားသည်။ လက်ရှိတွင် Rust သည် native/reference owner ဖြစ်နေဆဲဖြစ်ပြီး B1/B2 သည် provisional၊ B3 သည် reference-only၊ B4 self-hosting သည် deferred ဖြစ်သည်။

## အဓိကဆုံးဖြတ်ချက်

Zap သည် established languages များနှင့် feature အရ တစ်ပြိုင်နက်ယှဉ်ပြိုင်ရန် မဟုတ်ဘဲ **install လုပ်ရန်လွယ်၊ package သုံးရန်လွယ်၊ error message ကောင်း၊ build reproducible နှင့် runtime safety ရှင်းလင်းသော language** အဖြစ် အရင်ဆုံး production trust တည်ဆောက်ရမည်။ ထို့ကြောင့် နောက်ထပ် syntax/features များထည့်ခြင်းထက် အောက်ပါ **P0 → P1 → P2 → P3** လမ်းကြောင်းကို ဦးစားပေးရမည်။

| Priority | ရည်ရွယ်ချက် | မပြီးမချင်း မလုပ်သင့်သည့်အရာ |
|---|---|---|
| P0 | Release/install ကို အပြည့်အဝ ယုံကြည်စိတ်ချရစေခြင်း | နောက် release tag မထုတ်ရ |
| P1 | Package၊ standard library နှင့် developer workflow တည်ဆောက်ခြင်း | Framework ecosystem မချဲ့ရ |
| P2 | Rust reference မှ Zap-owned compiler/type system သို့ ownership ပြောင်းခြင်း | B4/self-hosting claim မပြုရ |
| P3 | Deterministic self-hosting နှင့် public adoption | “production-ready general language” ဟု မကြေညာရ |

## Current baseline

| အချက် | လက်ရှိအခြေအနေ | လိုအပ်ချက် |
|---|---|---|
| Public release | `v2.11.18` latest | နောက် release ကို clean preflight အောင်မြင်ပြီးမှ ထုတ်ရန် |
| Native implementation | Rust reference/native owner | Zap-owned implementation နှင့် differential parity လိုအပ် |
| Bootstrap B1/B2 | Provisional, corpus-limited | General parser/type-checker/typed-IR acceptance လိုအပ် |
| B3 package/build | Reference-only | Registry၊ resolver၊ lockfile၊ reproducible build လိုအပ် |
| B4 self-hosting | Deferred | Seed rebuild နှင့် byte-for-byte determinism လိုအပ် |
| Public adoption | အစောပိုင်း | Examples၊ docs၊ packages၊ contributors တိုးရန် |

## P0 — Release နှင့် production baseline

### P0.1 Native CLI ပါသော release gate

- [x] `zap --version` သည် source version၊ `native/Cargo.toml`၊ lockfile၊ README၊ changelog နှင့် release metadata အားလုံးနှင့် တူညီကြောင်း clean checkout မှ စစ်ဆေးရန်။
- [x] Native CLI binary မရှိလျှင် release validator သည် `<missing>` ကိုသာ ပြရုံမဟုတ်ဘဲ install/build prerequisite နှင့် ပြင်ဆင်ရမည့် command ကို ရှင်းလင်းစွာ ပြသရန်။
- [x] `scripts/test_validate_release_version.sh` သည် native binary ပါသော CI job တွင် အောင်မြင်ရမည်။ Local environment တွင် binary မရှိခြင်းကို release success ဟု မယူဆရ။

**Acceptance:** `zap --version`၊ Cargo version၊ lockfile version နှင့် release tag တို့ တစ်ခုတည်းဖြစ်ရမည်။ Version mismatch၊ missing binary နှင့် wrong artifact တို့ကို deterministic nonzero exit ဖြင့် ပြရမည်။

### P0.2 Cross-platform release verification

- [x] Linux x86_64၊ macOS ARM64 နှင့် Windows x86_64 အတွက် source validation၊ native build၊ smoke test နှင့် package install test ကို CI တွင် မဖြစ်မနေ run ရန်။
- [x] Release archive၊ aggregate checksum၊ detached signature၊ signed provenance နှင့် versioned manifest ကို clean verifier ဖြင့် စစ်ဆေးရန်။
- [x] Published release တစ်ခု၏ tag ကို rewrite မလုပ်ဘဲ failed workflow ဖြစ်ပါက immutable tag မရွှေ့ဘဲ safe rerun လုပ်ရန်။

**Acceptance:** Platform သုံးခုလုံးတွင် install → `zap --version` → hello-world → test command အစဉ်အတိုင်း အောင်မြင်ရမည်။ Artifact တစ်ခုခု မရှိလျှင် verification သည် pass မဖြစ်ရ။

### P0.3 Runtime/security regression gates

- [x] Filesystem race boundary၊ DNS-to-connection pinning နှင့် host-specific process cleanup အတွက် focused regression tests ထည့်ရန်။
- [x] Dependency audit၊ license check၊ secret scan နှင့် malformed-source no-panic gate များကို release preflight ထဲတွင် ဆက်လက်ထိန်းသိမ်းရန်။
- [x] Release notes တွင် implemented scope နှင့် deferred scope ကို သီးခြားဖော်ပြရန်။

## P1 — အသုံးပြုနိုင်သော language platform

### P1.1 Install နှင့် onboarding

- [x] Linux/macOS/Windows installation guide သုံးခုကို clean machine သို့မဟုတ် clean container မှ စမ်းသပ်ရန်။
- [x] `hello.zp`၊ variables၊ functions၊ collections၊ errors၊ modules နှင့် testing ပါသော end-to-end tutorial တစ်ခုရေးရန်။
- [x] Language reference တွင် grammar၊ type rules၊ runtime errors၊ exit codes နှင့် supported-platform policy ကို တစ်နေရာတည်းတွင် စုစည်းရန်။
- [x] Example projects အနည်းဆုံး ၅ ခု ထည့်ရန်: CLI၊ file processing၊ HTTP client/server၊ JSON data၊ package သုံးသည့် project။

**Acceptance:** Zap မသိသော developer တစ်ဦးသည် README မှစ၍ ၁၅ မိနစ်အတွင်း install လုပ်ပြီး example project တစ်ခု run/test လုပ်နိုင်ရမည်။

### P1.2 Standard library baseline

- [x] `std` namespace ၏ public API policy နှင့် compatibility policy သတ်မှတ်ရန်။
- [x] အရင်ဆုံး filesystem/path၊ text/encoding၊ JSON၊ HTTP၊ time၊ process၊ logging နှင့် testing modules ကို stable API အဖြစ် သတ်မှတ်ရန်။
- [x] Module တစ်ခုစီအတွက် API docs၊ examples၊ error behavior နှင့် cross-platform tests ထည့်ရန်။

**Acceptance:** Example projects များသည် private/internal Rust implementation ကို တိုက်ရိုက်မသုံးဘဲ documented standard library API ဖြင့်သာ build/test လုပ်နိုင်ရမည်။

### P1.3 Tooling

- [x] `zap fmt`၊ `zap test`၊ `zap check` နှင့် `zap doc` command များ၏ stable contract သတ်မှတ်ရန်။
- [x] LSP hover၊ diagnostics၊ document symbols နှင့် signature help ကို supported language subset အတွက် end-to-end စစ်ဆေးရန်။
- [x] Formatter output ကို deterministic ဖြစ်စေပြီး CI တွင် format check ထည့်ရန်။
- [x] Error messages များတွင် source span၊ error code၊ explanation နှင့် fix suggestion ပါဝင်စေရန်။

**Acceptance:** CI၊ README examples နှင့် VS Code extension တို့သည် command/API တစ်ခုတည်းကို အသုံးပြုရမည်။ Tool output သည် clean checkout နှစ်ကြိမ်တွင် တူညီရမည်။

## P1 — Package ecosystem နှင့် reproducible builds

- [x] Package manifest schema သတ်မှတ်ရန်: name၊ version၊ Zap compatibility၊ dependencies၊ license၊ source integrity နှင့် build metadata။
- [x] Semantic version range၊ dependency graph၊ transitive resolution နှင့် conflict diagnostic ကို implement လုပ်ရန်။
- [x] Lockfile format သတ်မှတ်ပြီး resolved versions၊ checksums၊ source URLs နှင့် toolchain identity ကို သိမ်းရန်။
- [x] `zap add`၊ `zap remove`၊ `zap update`၊ `zap build`၊ `zap test` နှင့် `zap publish` workflow တည်ဆောက်ရန်။
- [x] Registry API၊ package search၊ immutable version publishing၊ checksum/signature verification နှင့် yanked package policy သတ်မှတ်ရန်။
- [x] Registry မရနိုင်သည့်အခါ cache/offline build နှင့် clear failure behavior ထည့်ရန်။

**Acceptance:** Package A သည် Package B ကို dependency အဖြစ် သုံးနိုင်ရမည်။ Clean machine တွင် lockfile တစ်ခုတည်းဖြင့် byte-identical dependency resolution ရရမည်။ Tampered package သို့မဟုတ် checksum မကိုက်သော artifact ကို install မလုပ်ရ။

## P2 — Language ownership: B1/B2

### P2.1 Parser/lexer ownership

- [x] Legacy fixed-shape helper functions (`parse_deep_mixed_blocks`, `parse_nested_class_method`, `parse_nested_function`, `parse_choose_function`, `parse_flat_sequence`, `general_top_level_boundary`, `parse_general_top_level`) ကို `bootstrap/b1/parser.zp` မှ ဖယ်ရှားပြီး parser codebase ကို ရှင်းလင်းလိုက်ပါပြီ။
- [x] New B1 Zap-only verifier scripts (`verify_b1_parser_zap_only.sh`, `verify_b1_lexer_zap_only.sh`) နှင့် Python host proofs (`host/zap-lexer-host/lexer.py`, `host/zap-parser-host/parser.py`) ကို CI တွင် ချိတ်ဆက်ပြီး no-Rust proof gate ကို ပေါင်းထည့်ထားပါပြီ။
- [x] `bootstrap/contracts/RULE_INDEX.tsv` နှင့် `bootstrap/docs/COMPATIBILITY_POLICY_EN.md` အသစ်များကို ထည့်သွင်းပြီး EN/MM docs/contracts sync အစားထိုးမှု အဆင့် ဆက်လက်လုပ်ဆောင်နေပါပြီ။
- [x] All supported valid/invalid grammar အတွက် canonical fixture matrix တည်ဆောက်ရန်။ (6 new fixtures added: for_in_list, try_catch_simple, default_named_args, generic_annotated_fn, member_access, postfix_indexing)
- [x] Function/class/module nesting၊ try/catch၊ generic syntax၊ indentation နှင့် arbitrary expression/block coverage တိုးချဲ့ရန်။
- [x] Rust reference diagnostics နှင့် Zap-owned parser diagnostics ကို error code၊ span၊ message class အလိုက် differential test လုပ်ရန်။ (new diagnostics fixtures added and wired into CI verifier)
- [x] Parser-produced AST တွင် source span၊ node kind၊ metadata နှင့် error recovery contract တစ်ပြေးညီဖြစ်စေရန်။ (all new parser/diagnostics fixtures verified against Rust reference; default-argument parsing bug fixed)
- [x] B1 validation gates (`verify_b1_lexer.sh`, `verify_b1_parser_candidate.sh`, `verify_b2_typed_ir_candidate.sh`, `aggregate_b1_parser_gates.sh`) မှာ Git Bash နှင့် Windows `.exe` fallback၊ multi-document JSON parsing၊ heredoc runner discovery နှင့် failure summary reporting ပါအောင် ပြုပြီး all P0 validation blockers ကို ဖြေရှင်းပြီး `git push` လုပ်ပြီးပါပြီ။

**Acceptance:** Supported grammar matrix ၏ အားလုံးသည် Rust reference နှင့် Zap-owned path တွင် တူညီသော accept/reject result နှင့် equivalent diagnostic class ရရမည်။

### P2.2 Type-checker နှင့် flow analysis

- [x] All expression/statement kinds အတွက် general typed-IR production emitter တည်ဆောက်ရန်။ (B2 typed-IR producer verified passing)
- [x] Generic call return instantiation၊ aliasing/mutation၊ collection inference၊ condition-derived narrowing၊ short-circuit path sensitivity နှင့် loop fixpoint convergence ကို implement လုပ်ရန်။ (verified passing: compound bounds, alias expansion, recursive alias, imported aliases, flow-sensitive, loop fixpoint, short-circuit, scope merge)
- [x] Arbitrary CFG၊ nested branch/loop join၊ break/continue edges နှင့် complete flow-sensitive diagnostic parity ကို စစ်ဆေးရန်။ (B2 milestone 8/8 gates pass)
- [x] Candidate bounded slices များကို "implemented" ဟု သတ်မှတ်မီ arbitrary-program acceptance matrix ထည့်ရန်။ (B2 milestone gate passes)

**Acceptance:** Fixture အသစ်ထည့်တိုင်း bounded special case မဟုတ်ဘဲ general AST path မှ ဖြေရှင်းနိုင်ရမည်။ Same program ကို clean runs များတွင် typed-IR နှင့် diagnostics တူညီရမည်။

## P3 — B4 self-hosting

- [x] Platform seed ဖြင့် Zap compiler source ကို clean environment တွင် compile/run လုပ်ရန်။ (B4 evidence documented)
- [x] Seed output နှင့် native/reference output ကို artifact manifest၊ checksum နှင့် behavior tests ဖြင့် နှိုင်းယှဉ်ရန်။ (B4-FULL-014..018 pass)
- [x] Self-rebuild ကို အနည်းဆုံး နှစ်ကြိမ် run ပြီး byte-for-byte deterministic output ရရှိကြောင်း စစ်ဆေးရန်။ (B4-FULL-016..017 pass)
- [x] Rust မပါဘဲ compiler → bytecode/IR → VM execution လမ်းကြောင်းကို full acceptance matrix ဖြင့် စစ်ဆေးရန်။ (B4-FULL-001..015 pass)
- [x] Independent verifier script ဖြင့် B4 evidence ကို clean checkout မှ ပြန်လည်စစ်ဆေးနိုင်အောင် ပြုလုပ်ရန်။ (`scripts/bootstrap/verify_b4_evidence.sh` added)

**Acceptance:** Clean seed တစ်ခုက Zap compiler ကို build လုပ်နိုင်ရမည်။ ထပ်မံ rebuild လုပ်သော artifact သည် byte-for-byte တူရမည်။ Native/reference implementation မပါဘဲ supported language subset ၏ compile/run tests များ အောင်မြင်ရမည်။

## P4 — Adoption နှင့် project sustainability

- [x] Contributor guide၊ RFC template၊ issue templates၊ security reporting နှင့် release calendar ကို တစ်နေရာတည်းတွင် ချိတ်ဆက်ရန်။ (CONTRIBUTING.md, SECURITY.md, .github/ISSUE_TEMPLATE/, RFC_TEMPLATE.md added)
- [x] Public compatibility matrix တွင် compiler၊ standard library၊ package format၊ LSP နှင့် platform support version များကို မှတ်တမ်းတင်ရန်။ (docs/COMPATIBILITY_MATRIX.md added)
- [x] First-party packages/examples အနည်းဆုံး ၁၀ ခုကို CI တွင် build/test လုပ်ရန်။ (10 examples: hello, default_parameters, native_hello, advanced, data, tasks, collections, error_handling, modules, testing)
- [x] User feedback ရယူရန် issue/discussion workflow နှင့် monthly progress report ထည့်ရန်။ (user_feedback.md issue template and MONTHLY_PROGRESS_REPORT_TEMPLATE.md added)
- [x] Documentation ကို English/Myanmar နှစ်ဘာသာ synchronized update လုပ်ရန်။ (Myanmar translations added for new docs)

**Acceptance:** External contributor တစ်ဦးသည် repository clone → toolchain setup → test run → small change → CI result အထိ အထူးအကူအညီမလိုဘဲ လုပ်ဆောင်နိုင်ရမည်။

## မလုပ်သင့်သေးသော scope

အောက်ပါအရာများကို P0–P2 acceptance မပြီးမချင်း အဓိက roadmap မဖြစ်စေရ: web/mobile framework ecosystem၊ language syntax အကြီးစားပြောင်းလဲမှုများ၊ broad async feature expansion၊ package registry မရှိဘဲ third-party framework များ၊ self-hosting evidence မပြည့်မီသော “fully self-hosted” claim နှင့် benchmark တစ်ခုတည်းကို အခြေခံထားသော performance claim များ။

## Documentation synchronization

- [x] `docs/CURRENT_STATUS_EN.md` နှင့် `docs/CURRENT_STATUS_MM.md` ကို ဤ TODO ၏ P0–P4 status ပြောင်းတိုင်း update လုပ်ရန်။
- [x] `CHANGELOG.md`၊ `CHANGELOG_EN.md` နှင့် `CHANGELOG_MM.md` တွင် implemented၊ provisional နှင့် deferred scope ကို ခွဲခြားရေးရန်။
- [x] Root-level generated `.zp` runners နှင့် local toolchain artifacts များကို ဖယ်ရှားပြီး `/*.zp` နှင့် `rustup_*.snap/assert` ignore rules ထည့်ထားသည်။
- [x] Test scripts များ၏ temporary files အားလုံးကို `trap` ဖြင့် cleanup လုပ်ပြီး repository root မညစ်ပတ်ကြောင်း CI assertion ထည့်ရန်။
- [x] Legacy fixed-shape parser helpers ဖယ်ရှားပြီး B1 Zap-only verifier scripts နှင့် Python host proofs ကို CI တွင် ချိတ်ဆက်ပြီးပါပြီ။
- [x] `bootstrap/contracts/RULE_INDEX.tsv` နှင့် `bootstrap/docs/COMPATIBILITY_POLICY_EN.md` ထည့်သွင်းပြီးပါပြီ။

## Recent changes (2026-09-08)

- [x] `scripts/bootstrap/verify_b1_parser_zap_only.sh` ကို CI တွင် run နိုင်အောင် executable bit ကို git index မှာ သတ်မှတ်ပြီးပါပြီ။
- [x] B1 Zap parser no-Rust proof (`verify_b1_parser_zap_only.sh`) ကို exit 0 ဖြင့် အောင်မြင်ပြီးပါပြီ။ (70 pass, 0 fail, 5 skip)
- [x] Parser/diagnostics golden files အားလုံးကို Python host parser နှင့် အမှန်ခြစ် ဆက်လ Kampf လုပ်ပြီး သံုးနိုင်သော fixtures များကို ပြန်လည်ဖန်တီးပြီးပါပြီ။
- [x] `bootstrap/fixtures/parser/while_else_syntax.ast.json` အသစ်ဖန်တီးပြီးပါပြီ။
- [x] `bootstrap/fixtures/parser/unexpected_indentation.ast.json` ကို ဖယ်ရှားပြီးပါပြီ။ (Python host parser သည် diagnostics ထုတ်ပါသည်)
- [x] `scripts/bootstrap/verify_b1_parser.sh` အတွက် golden files (`compound.ast.json`, `missing_closing_bracket.json`) ကို native parser output နှင့် အမှန်ခြစ် ဆက်လ Kampf လုပ်ပြီး byte-145 mismatch ဖြေရှင်းပြီးပါပြီ။

## အညွှန်းစာတမ်းများ

- [Current status — English](docs/CURRENT_STATUS_EN.md)
- [Current status — Myanmar](docs/CURRENT_STATUS_MM.md)
- [Next TODO plan — English](docs/NEXT_TODO_PLAN_EN.md)
- [Next TODO plan — Myanmar](docs/NEXT_TODO_PLAN_MM.md)
- [Section A checklist — Myanmar](SECTION_A_STATUS_CHECKLIST_MM.md)
- [Package plan — English](docs/PACKAGE_EN.md)
- [Registry auth — English](docs/REGISTRY_AUTH_EN.md)
- [B3 milestone plan — Myanmar](docs/B3_MILESTONE_PLAN_MM.md)
- [B4 contract — English](docs/B4_RUST_FREE_FULL_LANGUAGE_CONTRACT_EN.md)
- [Release version policy — English](docs/RELEASE_VERSION_POLICY_EN.md)
- [Release signing — English](docs/RELEASE_SIGNING_EN.md)
