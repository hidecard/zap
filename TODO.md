# Zap Remaining TODO

**စစ်ဆေး/Update သည့်နေ့:** 2026-09-03
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

- [ ] `zap --version` သည် source version၊ `native/Cargo.toml`၊ lockfile၊ README၊ changelog နှင့် release metadata အားလုံးနှင့် တူညီကြောင်း clean checkout မှ စစ်ဆေးရန်။
- [ ] Native CLI binary မရှိလျှင် release validator သည် `<missing>` ကိုသာ ပြရုံမဟုတ်ဘဲ install/build prerequisite နှင့် ပြင်ဆင်ရမည့် command ကို ရှင်းလင်းစွာ ပြသရန်။
- [ ] `scripts/test_validate_release_version.sh` သည် native binary ပါသော CI job တွင် အောင်မြင်ရမည်။ Local environment တွင် binary မရှိခြင်းကို release success ဟု မယူဆရ။

**Acceptance:** `zap --version`၊ Cargo version၊ lockfile version နှင့် release tag တို့ တစ်ခုတည်းဖြစ်ရမည်။ Version mismatch၊ missing binary နှင့် wrong artifact တို့ကို deterministic nonzero exit ဖြင့် ပြရမည်။

### P0.2 Cross-platform release verification

- [ ] Linux x86_64၊ macOS ARM64 နှင့် Windows x86_64 အတွက် source validation၊ native build၊ smoke test နှင့် package install test ကို CI တွင် မဖြစ်မနေ run ရန်။
- [ ] Release archive၊ aggregate checksum၊ detached signature၊ signed provenance နှင့် versioned manifest ကို clean verifier ဖြင့် စစ်ဆေးရန်။
- [ ] Published release တစ်ခု၏ tag ကို rewrite မလုပ်ဘဲ failed workflow ဖြစ်ပါက immutable tag မရွှေ့ဘဲ safe rerun လုပ်ရန်။

**Acceptance:** Platform သုံးခုလုံးတွင် install → `zap --version` → hello-world → test command အစဉ်အတိုင်း အောင်မြင်ရမည်။ Artifact တစ်ခုခု မရှိလျှင် verification သည် pass မဖြစ်ရ။

### P0.3 Runtime/security regression gates

- [ ] Filesystem race boundary၊ DNS-to-connection pinning နှင့် host-specific process cleanup အတွက် focused regression tests ထည့်ရန်။
- [ ] Dependency audit၊ license check၊ secret scan နှင့် malformed-source no-panic gate များကို release preflight ထဲတွင် ဆက်လက်ထိန်းသိမ်းရန်။
- [ ] Release notes တွင် implemented scope နှင့် deferred scope ကို သီးခြားဖော်ပြရန်။

## P1 — အသုံးပြုနိုင်သော language platform

### P1.1 Install နှင့် onboarding

- [ ] Linux/macOS/Windows installation guide သုံးခုကို clean machine သို့မဟုတ် clean container မှ စမ်းသပ်ရန်။
- [ ] `hello.zp`၊ variables၊ functions၊ collections၊ errors၊ modules နှင့် testing ပါသော end-to-end tutorial တစ်ခုရေးရန်။
- [ ] Language reference တွင် grammar၊ type rules၊ runtime errors၊ exit codes နှင့် supported-platform policy ကို တစ်နေရာတည်းတွင် စုစည်းရန်။
- [ ] Example projects အနည်းဆုံး ၅ ခု ထည့်ရန်: CLI၊ file processing၊ HTTP client/server၊ JSON data၊ package သုံးသည့် project။

**Acceptance:** Zap မသိသော developer တစ်ဦးသည် README မှစ၍ ၁၅ မိနစ်အတွင်း install လုပ်ပြီး example project တစ်ခု run/test လုပ်နိုင်ရမည်။

### P1.2 Standard library baseline

- [ ] `std` namespace ၏ public API policy နှင့် compatibility policy သတ်မှတ်ရန်။
- [ ] အရင်ဆုံး filesystem/path၊ text/encoding၊ JSON၊ HTTP၊ time၊ process၊ logging နှင့် testing modules ကို stable API အဖြစ် သတ်မှတ်ရန်။
- [ ] Module တစ်ခုစီအတွက် API docs၊ examples၊ error behavior နှင့် cross-platform tests ထည့်ရန်။

**Acceptance:** Example projects များသည် private/internal Rust implementation ကို တိုက်ရိုက်မသုံးဘဲ documented standard library API ဖြင့်သာ build/test လုပ်နိုင်ရမည်။

### P1.3 Tooling

- [ ] `zap fmt`၊ `zap test`၊ `zap check` နှင့် `zap doc` command များ၏ stable contract သတ်မှတ်ရန်။
- [ ] LSP hover၊ diagnostics၊ document symbols နှင့် signature help ကို supported language subset အတွက် end-to-end စစ်ဆေးရန်။
- [ ] Formatter output ကို deterministic ဖြစ်စေပြီး CI တွင် format check ထည့်ရန်။
- [ ] Error messages များတွင် source span၊ error code၊ explanation နှင့် fix suggestion ပါဝင်စေရန်။

**Acceptance:** CI၊ README examples နှင့် VS Code extension တို့သည် command/API တစ်ခုတည်းကို အသုံးပြုရမည်။ Tool output သည် clean checkout နှစ်ကြိမ်တွင် တူညီရမည်။

## P1 — Package ecosystem နှင့် reproducible builds

- [ ] Package manifest schema သတ်မှတ်ရန်: name၊ version၊ Zap compatibility၊ dependencies၊ license၊ source integrity နှင့် build metadata။
- [ ] Semantic version range၊ dependency graph၊ transitive resolution နှင့် conflict diagnostic ကို implement လုပ်ရန်။
- [ ] Lockfile format သတ်မှတ်ပြီး resolved versions၊ checksums၊ source URLs နှင့် toolchain identity ကို သိမ်းရန်။
- [ ] `zap add`၊ `zap remove`၊ `zap update`၊ `zap build`၊ `zap test` နှင့် `zap publish` workflow တည်ဆောက်ရန်။
- [ ] Registry API၊ package search၊ immutable version publishing၊ checksum/signature verification နှင့် yanked package policy သတ်မှတ်ရန်။
- [ ] Registry မရနိုင်သည့်အခါ cache/offline build နှင့် clear failure behavior ထည့်ရန်။

**Acceptance:** Package A သည် Package B ကို dependency အဖြစ် သုံးနိုင်ရမည်။ Clean machine တွင် lockfile တစ်ခုတည်းဖြင့် byte-identical dependency resolution ရရမည်။ Tampered package သို့မဟုတ် checksum မကိုက်သော artifact ကို install မလုပ်ရ။

## P2 — Language ownership: B1/B2

### P2.1 Parser/lexer ownership

- [ ] All supported valid/invalid grammar အတွက် canonical fixture matrix တည်ဆောက်ရန်။
- [ ] Function/class/module nesting၊ try/catch၊ generic syntax၊ indentation နှင့် arbitrary expression/block coverage တိုးချဲ့ရန်။
- [ ] Rust reference diagnostics နှင့် Zap-owned parser diagnostics ကို error code၊ span၊ message class အလိုက် differential test လုပ်ရန်။
- [ ] Parser-produced AST တွင် source span၊ node kind၊ metadata နှင့် error recovery contract တစ်ပြေးညီဖြစ်စေရန်။

**Acceptance:** Supported grammar matrix ၏ အားလုံးသည် Rust reference နှင့် Zap-owned path တွင် တူညီသော accept/reject result နှင့် equivalent diagnostic class ရရမည်။

### P2.2 Type-checker နှင့် flow analysis

- [ ] All expression/statement kinds အတွက် general typed-IR production emitter တည်ဆောက်ရန်။
- [ ] Generic call return instantiation၊ aliasing/mutation၊ collection inference၊ condition-derived narrowing၊ short-circuit path sensitivity နှင့် loop fixpoint convergence ကို implement လုပ်ရန်။
- [ ] Arbitrary CFG၊ nested branch/loop join၊ break/continue edges နှင့် complete flow-sensitive diagnostic parity ကို စစ်ဆေးရန်။
- [ ] Candidate bounded slices များကို “implemented” ဟု သတ်မှတ်မီ arbitrary-program acceptance matrix ထည့်ရန်။

**Acceptance:** Fixture အသစ်ထည့်တိုင်း bounded special case မဟုတ်ဘဲ general AST path မှ ဖြေရှင်းနိုင်ရမည်။ Same program ကို clean runs များတွင် typed-IR နှင့် diagnostics တူညီရမည်။

## P3 — B4 self-hosting

- [ ] Platform seed ဖြင့် Zap compiler source ကို clean environment တွင် compile/run လုပ်ရန်။
- [ ] Seed output နှင့် native/reference output ကို artifact manifest၊ checksum နှင့် behavior tests ဖြင့် နှိုင်းယှဉ်ရန်။
- [ ] Self-rebuild ကို အနည်းဆုံး နှစ်ကြိမ် run ပြီး byte-for-byte deterministic output ရရှိကြောင်း စစ်ဆေးရန်။
- [ ] Rust မပါဘဲ compiler → bytecode/IR → VM execution လမ်းကြောင်းကို full acceptance matrix ဖြင့် စစ်ဆေးရန်။
- [ ] Independent verifier script ဖြင့် B4 evidence ကို clean checkout မှ ပြန်လည်စစ်ဆေးနိုင်အောင် ပြုလုပ်ရန်။

**Acceptance:** Clean seed တစ်ခုက Zap compiler ကို build လုပ်နိုင်ရမည်။ ထပ်မံ rebuild လုပ်သော artifact သည် byte-for-byte တူရမည်။ Native/reference implementation မပါဘဲ supported language subset ၏ compile/run tests များ အောင်မြင်ရမည်။

## P4 — Adoption နှင့် project sustainability

- [ ] Contributor guide၊ RFC template၊ issue templates၊ security reporting နှင့် release calendar ကို တစ်နေရာတည်းတွင် ချိတ်ဆက်ရန်။
- [ ] Public compatibility matrix တွင် compiler၊ standard library၊ package format၊ LSP နှင့် platform support version များကို မှတ်တမ်းတင်ရန်။
- [ ] First-party packages/examples အနည်းဆုံး ၁၀ ခုကို CI တွင် build/test လုပ်ရန်။
- [ ] User feedback ရယူရန် issue/discussion workflow နှင့် monthly progress report ထည့်ရန်။
- [ ] Documentation ကို English/Myanmar နှစ်ဘာသာ synchronized update လုပ်ရန်။

**Acceptance:** External contributor တစ်ဦးသည် repository clone → toolchain setup → test run → small change → CI result အထိ အထူးအကူအညီမလိုဘဲ လုပ်ဆောင်နိုင်ရမည်။

## မလုပ်သင့်သေးသော scope

အောက်ပါအရာများကို P0–P2 acceptance မပြီးမချင်း အဓိက roadmap မဖြစ်စေရ: web/mobile framework ecosystem၊ language syntax အကြီးစားပြောင်းလဲမှုများ၊ broad async feature expansion၊ package registry မရှိဘဲ third-party framework များ၊ self-hosting evidence မပြည့်မီသော “fully self-hosted” claim နှင့် benchmark တစ်ခုတည်းကို အခြေခံထားသော performance claim များ။

## Documentation synchronization

- [ ] `docs/CURRENT_STATUS_EN.md` နှင့် `docs/CURRENT_STATUS_MM.md` ကို ဤ TODO ၏ P0–P4 status ပြောင်းတိုင်း update လုပ်ရန်။
- [ ] `CHANGELOG.md`၊ `CHANGELOG_EN.md` နှင့် `CHANGELOG_MM.md` တွင် implemented၊ provisional နှင့် deferred scope ကို ခွဲခြားရေးရန်။
- [x] Root-level generated `.zp` runners နှင့် local toolchain artifacts များကို ဖယ်ရှားပြီး `/*.zp` နှင့် `rustup_*.snap/assert` ignore rules ထည့်ထားသည်။
- [ ] Test scripts များ၏ temporary files အားလုံးကို `trap` ဖြင့် cleanup လုပ်ပြီး repository root မညစ်ပတ်ကြောင်း CI assertion ထည့်ရန်။

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
