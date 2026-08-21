# Zap Documentation Navigation

**Verified baseline:** Zap v2.1.11
**ရည်ရွယ်ချက်:** ဤစာမျက်နှာသည် learner၊ language user၊ package author၊ runtime maintainer နှင့် release operator များအတွက် Burmese entry point ဖြစ်ပါသည်။ Normative behavior သည် canonical specification သို့မဟုတ် explicit linked contract တွင်သာ သတ်မှတ်ပါသည်။ ရှင်းလင်းဖော်ပြသည့် guide များသည် ထို contract များကို တိတ်တဆိတ် override မလုပ်ရပါ။

## လေ့လာမည့်လမ်းကြောင်း ရွေးချယ်ခြင်း

| အသုံးပြုသူ | ဤနေရာမှ စတင်ရန် | ဆက်လက်ဖတ်ရှုရန် |
|---|---|---|
| အသစ်စတင်လေ့လာသူ | [Burmese learning guide](LEARN_ZAP_MM.md) | [Syntax guide](SYNTAX_GUIDE.md)၊ [examples](../examples) |
| Language user | [Syntax guide](SYNTAX_GUIDE.md) | [Language specification](LANGUAGE_SPEC_MM.md)၊ [type-check matrix](TYPECHECK_CONFORMANCE_MATRIX_MM.md) |
| Package author | [Burmese package guide](PACKAGE.md) | [Stdlib index](STDLIB_INDEX_MM.md)၊ [registry/authentication contract](REGISTRY_AUTH_MM.md) |
| Runtime maintainer | [Language specification](LANGUAGE_SPEC_MM.md) | [Memory model](MEMORY_MODEL_MM.md)၊ [diagnostics](DIAGNOSTIC_MODEL_MM.md)၊ [async boundaries](ASYNC_BOUNDARIES_MM.md) |
| Tooling contributor | [Async/LSP guide](ASYNC_LSP_MM.md) | [LSP implementation](../native/src/lsp.rs)၊ [VS Code extension](../vscode-extension) |
| Deployment operator | [Deployment guide](DEPLOYMENT_MM.md) | [Registry deployment policy](../deploy/registry-deployment-policy.toml)၊ [security policy](../SECURITY.md) |
| Release operator | [Release version policy](RELEASE_VERSION_POLICY_MM.md) | [Release signing](RELEASE_SIGNING_MM.md)၊ [rollback runbook](RELEASE_ROLLBACK_RUNBOOK_MM.md)၊ [benchmark contract](BENCHMARK_HARNESS_MM.md) |

## Normative contract map

| Domain | Canonical contract | Executable evidence |
|---|---|---|
| Language semantics | [Language specification](LANGUAGE_SPEC_MM.md) | [Specification ownership index](SPEC_OWNERSHIP_INDEX.tsv) |
| Diagnostics | [Diagnostic model](DIAGNOSTIC_MODEL_MM.md) | Native diagnostic tests |
| Memory and borrowing | [Memory model](MEMORY_MODEL_MM.md) | Borrow နှင့် memory-limit regressions |
| Runtime state | [Runtime state and execution context](RUNTIME_STATE_MM.md) | Runtime-state isolation နှင့် reset regressions |
| Async boundaries | [Async boundary contract](ASYNC_BOUNDARIES_MM.md) | Async runtime နှင့် adapter tests |
| Standard library | [Stdlib index](STDLIB_INDEX_MM.md) | Builtin catalog နှင့် security corpus |
| Native/legacy compatibility | [P0-01 parity matrix](P001_PARITY_MATRIX_MM.md) | `scripts/test_p001_parity.sh` |
| Verification/replay | [P1-05 replay contract](P105_REPLAY_MM.md) | `scripts/test_p105_replay.sh` |
| Performance | [Benchmark harness](BENCHMARK_HARNESS_MM.md) | Checked-in `benchmark-results/native-summary.csv` နှင့် CI threshold gate |
| Releases | [Release version policy](RELEASE_VERSION_POLICY_MM.md) | `scripts/validate_release_version.sh` နှင့် release preflight |

## Version နှင့် contribution စည်းမျဉ်းများ

Authoritative package version သည် `native/Cargo.toml` ဖြစ်ပါသည်။ Release-facing surface များအားလုံးသည် ထို version နှင့် ကိုက်ညီရမည်ဖြစ်ပြီး CI က ထို consistency ကို စစ်ဆေးပါသည်။ Normative rule ပြောင်းလဲပါက English/Burmese contract နှစ်ခုလုံးကို တစ်ပြိုင်တည်း update လုပ်ရမည်။ `SPEC_OWNERSHIP_INDEX.tsv` တွင် fixture owner ထည့်/ပြင်ရမည်၊ bilingual compatibility template ဖြင့် compatibility impact မှတ်တမ်းတင်ရမည်၊ merge မလုပ်မီ regression evidence ထည့်ရမည်။

Documentation ပြောင်းလဲမှုများသည် English/Burmese pair ကို ထိန်းသိမ်းရမည်၊ repository-relative links သုံးရမည်၊ deferred behavior ကို explicit ဖော်ပြရမည်၊ executable gate မရှိသေးသော production scheduling၊ cancellation၊ sandbox သို့မဟုတ် performance guarantee များကို မဆိုရပါ။ လက်ရှိအလုပ်များအတွက် [remaining TODO register](PDF_REMAINING_TODO_MM.md) နှင့် [next-step plan](NEXT_TODO_PLAN_MM.md) ကို ကြည့်ရှုနိုင်ပါသည်။
