# Zap Documentation Navigation

**Verified baseline:** Zap v2.2.7
**ရည်ရွယ်ချက်:** ဤစာမျက်နှာသည် learner၊ language user၊ package author၊ runtime maintainer နှင့် release operator များအတွက် Burmese entry point ဖြစ်ပါသည်။ Normative behavior သည် canonical specification သို့မဟုတ် explicit linked contract တွင်သာ သတ်မှတ်ပါသည်။ ရှင်းလင်းဖော်ပြသည့် guide များသည် ထို contract များကို တိတ်တဆိတ် override မလုပ်ရပါ။

## လေ့လာမည့်လမ်းကြောင်း ရွေးချယ်ခြင်း

| အသုံးပြုသူ | ဤနေရာမှ စတင်ရန် | ဆက်လက်ဖတ်ရှုရန် |
|---|---|---|
| အသစ်စတင်လေ့လာသူ | [Burmese learning guide](LEARN_ZAP_MM.md) | [Syntax guide](SYNTAX_GUIDE.md)၊ [examples](../examples) |
| Language user | [Syntax guide](SYNTAX_GUIDE.md) | [Language specification](LANGUAGE_SPEC_MM.md)၊ [type-check matrix](TYPECHECK_CONFORMANCE_MATRIX_MM.md) |
| Package author | [Burmese package guide](PACKAGE.md) | [Stdlib index](STDLIB_INDEX_MM.md)၊ [registry/authentication contract](REGISTRY_AUTH_MM.md) |
| Framework contributor | [Framework guide](FRAMEWORK_MM.md) | [Zap-first Web guide](ZAP_WEB_NATIVE_MM.md)၊ [Web Framework guide](WEB_FRAMEWORK_MM.md)၊ [zap-host adapter](ZAP_HOST_MM.md)၊ [zap-host quickstart](ZAP_HOST_QUICKSTART_MM.md)၊ [Framework starters](../frameworks)၊ [ecosystem roadmap](ECOSYSTEM.md)၊ [package guide](PACKAGE.md) |
| Runtime maintainer | [Language specification](LANGUAGE_SPEC_MM.md) | [Memory model](MEMORY_MODEL_MM.md)၊ [diagnostics](DIAGNOSTIC_MODEL_MM.md)၊ [async boundaries](ASYNC_BOUNDARIES_MM.md) |
| Tooling contributor | [Async/LSP guide](ASYNC_LSP_MM.md) | [LSP implementation](../native/src/lsp.rs)၊ [canonical VS Code extension](../vscode-extension)၊ [editor assets](../editors/vscode)၊ [semantic-parity validator](../scripts/test_lsp_semantic_parity.sh)၊ [protocol synchronization contract](../scripts/test_lsp_protocol_sync.sh) |
| Deployment operator | [Deployment guide](DEPLOYMENT_MM.md) | [Registry deployment policy](../deploy/registry-deployment-policy.toml)၊ [security policy](../SECURITY.md) |
| Release operator | [Release version policy](RELEASE_VERSION_POLICY_MM.md) | [Release signing](RELEASE_SIGNING_MM.md)၊ [rollback runbook](RELEASE_ROLLBACK_RUNBOOK_MM.md)၊ [benchmark contract](BENCHMARK_HARNESS_MM.md)၊ [v2.2.0 နောက်ပိုင်း remediation/provenance](POST_V2.2.0_REMEDIATION_MM.md)၊ [release preflight](../scripts/release_preflight.sh) |
| Language designer | [Traits/composition RFC](TRAITS_RFC_MM.md) | [Language specification](LANGUAGE_SPEC_MM.md)၊ [compatibility template](COMPATIBILITY_CHANGE_TEMPLATE_MM.md) |

## M3-DOC-01 စစ်ဆေးပြီးသော documentation surface များ

| Audience section | Verified entry point | Canonical companion |
|---|---|---|
| Learner | [လေ့လာရေး guide](LEARN_ZAP_MM.md) — v2.2.7 | [Syntax guide](SYNTAX_GUIDE.md) |
| Language user | [Syntax guide](SYNTAX_GUIDE.md) — v2.2.7 | [Language specification](LANGUAGE_SPEC_MM.md) |
| Package author | [Package guide](PACKAGE.md) — v2.2.7 | [Stdlib reference](STDLIB_INDEX_MM.md)၊ [registry contract](REGISTRY_AUTH_MM.md) |
| Framework contributor | [Framework guide](FRAMEWORK_MM.md) — v2.2.7 | [Zap-first Web guide](ZAP_WEB_NATIVE_MM.md)၊ [Web Framework guide](WEB_FRAMEWORK_MM.md)၊ [zap-host adapter](ZAP_HOST_MM.md)၊ [zap-host quickstart](ZAP_HOST_QUICKSTART_MM.md)၊ [Framework starters](../frameworks)၊ [ecosystem roadmap](ECOSYSTEM.md) |
| Runtime maintainer | [Memory model](MEMORY_MODEL_MM.md) — v2.2.7 | [Runtime state](RUNTIME_STATE_MM.md)၊ [memory budget](MEMORY_BUDGET_OBJECT_STORE_MM.md) |
| Deployment/security operator | [Deployment boundaries](DEPLOYMENT_MM.md) — v2.2.7 | [Security policy](../SECURITY.md)၊ [release signing](RELEASE_SIGNING_MM.md) |

## Normative contract map

| Domain | Canonical contract | Executable evidence |
|---|---|---|
| Language semantics | [Language specification](LANGUAGE_SPEC_MM.md) | [Specification ownership index](SPEC_OWNERSHIP_INDEX.tsv) |
| Diagnostics | [Diagnostic model](DIAGNOSTIC_MODEL_MM.md) | Native diagnostic tests |
| Memory and borrowing | [Memory model](MEMORY_MODEL_MM.md) | Borrow နှင့် memory-limit regressions |
| Memory budget/object store | [MemoryBudget and ObjectStore contract](MEMORY_BUDGET_OBJECT_STORE_MM.md) | Run-owned budget နှင့် object-store isolation regressions |
| Runtime state | [Runtime state and execution context](RUNTIME_STATE_MM.md) | Runtime-state isolation၊ workspace နှင့် reset regressions |
| AST foundation | [AST foundation status](P0_FOUNDATION_STATUS_MM.md) | Canonical AST၊ export နှင့် compatibility-boundary regressions |
| Async boundaries | [Async boundary contract](ASYNC_BOUNDARIES_MM.md) | Async runtime နှင့် adapter tests |
| Framework adapters | [Framework guide](FRAMEWORK_MM.md) · [Zap-first Web guide](ZAP_WEB_NATIVE_MM.md) · [Web Framework guide](WEB_FRAMEWORK_MM.md) · [zap-host adapter](ZAP_HOST_MM.md) · [zap-host quickstart](ZAP_HOST_QUICKSTART_MM.md) | Zap-first scaffold checks၊ starter smoke tests၊ Web contract tests၊ host-capability contract tests နှင့် Axum/Tower adapter tests |
| Standard library | [Stdlib index](STDLIB_INDEX_MM.md) · [Stability policy](STDLIB_POLICY_MM.md) | Machine-readable builtin catalog၊ stability/deprecation metadata၊ schema-2 determinism class နှင့် security corpus |
| Native/legacy compatibility | [P0-01 parity matrix](P001_PARITY_MATRIX_MM.md) | `scripts/test_p001_parity.sh` |
| Verification/replay | [P1-05 replay နှင့် M2-VERIFY-01 bounded replay contract](P105_REPLAY_MM.md) | `scripts/test_p105_replay.sh` နှင့် `scripts/test_m2_verify_replay.sh` |
| Performance | [Benchmark harness](BENCHMARK_HARNESS_MM.md) | Checked-in `benchmark-results/native-summary.csv` နှင့် CI threshold gate |
| Releases | [Release version policy](RELEASE_VERSION_POLICY_MM.md) · [v2.2.0 နောက်ပိုင်း remediation/provenance](POST_V2.2.0_REMEDIATION_MM.md) | `scripts/validate_release_version.sh` နှင့် release preflight |

## Version နှင့် contribution စည်းမျဉ်းများ

Authoritative package version သည် `native/Cargo.toml` ဖြစ်ပါသည်။ Release-facing surface များအားလုံးသည် ထို version နှင့် ကိုက်ညီရမည်ဖြစ်ပြီး CI က ထို consistency ကို စစ်ဆေးပါသည်။ Normative rule ပြောင်းလဲပါက English/Burmese contract နှစ်ခုလုံးကို တစ်ပြိုင်တည်း update လုပ်ရမည်။ `SPEC_OWNERSHIP_INDEX.tsv` တွင် fixture owner ထည့်/ပြင်ရမည်၊ bilingual compatibility template ဖြင့် compatibility impact မှတ်တမ်းတင်ရမည်၊ merge မလုပ်မီ regression evidence ထည့်ရမည်။ Public standard-library change များတွင် catalog နှင့် stability policy pair ကိုလည်း update လုပ်ရမည်။

Documentation ပြောင်းလဲမှုများသည် English/Burmese pair ကို ထိန်းသိမ်းရမည်၊ repository-relative links သုံးရမည်၊ deferred behavior ကို explicit ဖော်ပြရမည်၊ executable gate မရှိသေးသော production scheduling၊ cancellation၊ sandbox သို့မဟုတ် performance guarantee များကို မဆိုရပါ။ Framework ပြောင်းလဲမှုများတွင် Framework guide pair၊ starter manifest/lockfile နှင့် host-adapter boundary ကို update လုပ်ရမည်ဖြစ်ပြီး မထောက်ပံ့သေးသော core syntax မထည့်ရပါ။ [v2.2.0 နောက်ပိုင်း remediation/provenance record](POST_V2.2.0_REMEDIATION_MM.md) တွင် immutable v2.2.0 asset များနှင့် နောက်ပိုင်း `master` correction များကို ခွဲခြားဖော်ပြထားပြီး ထို correction များကို v2.2.2 တွင် ထုတ်ဝေပြီး post-v2.2.2 hardening ကို v2.2.3 တွင် ထုတ်ဝေထားသည်။ လက်ရှိအလုပ်များအတွက် [v2.2.7 release notes](RELEASE_2.2.7_MM.md)၊ [remaining TODO register](PDF_REMAINING_TODO_MM.md) နှင့် [next-step plan](NEXT_TODO_PLAN_MM.md) ကို ကြည့်ရှုနိုင်ပါသည်။
