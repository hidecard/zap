# Zap v2.11.18 Release Preparation

ဤစာရွက်သည် v2.11.18 release line အတွက် လက်ရှိပြင်ဆင်ရေး အလုပ်များကို မှတ်တမ်းတင်ပါသည်။ Canonical release-version policy ကို [`RELEASE_VERSION_POLICY_MM.md`](RELEASE_VERSION_POLICY_MM.md) တွင် လည်းကြည့်ပါ။

## အခြေအနေ

v2.11.18 သည် နောက်ဆုံး publish လုပ်ထားသော release ဖြစ်ပါသည်။ v2.11.18 preparation သည် ဆက်လက်လုပ်ဆောင်နေဆဲ ဖြစ်ပြီး v2.11.18 tag မရှိသေးပါ၊ public GitHub Release လည်း publish မရှိသေးပါ။

## လိုအပ်သော release surface များ (v2.11.18)

| Surface | လိုအပ်ချက် |
|---|---|
| `native/Cargo.toml` | `version` ကို `2.11.17` မှ `2.11.18` သို့ bump လုပ်ရန် |
| `native/Cargo.lock` | `zap-native` package version ကို `2.11.18` သို့ update လုပ်ရန် |
| `CHANGELOG.md`, `CHANGELOG_EN.md`, `CHANGELOG_MM.md` | `2.11.18` ကို ဖော်ပြရန် |
| `README.md`, `README_MM.md` | လက်ရှိ release line နှင့် archive filename များ update လုပ်ရန် |
| `SECURITY.md` | `Latest v2.11.x` နှင့် integrity URL update လုပ်ရန် |
| `docs/TYPECHECK_CONFORMANCE_MATRIX_EN.md`, `docs/TYPECHECK_CONFORMANCE_MATRIX_MM.md` | `2.11.18` ကို ဖော်ပြရန် |
| `vscode-extension/package.json` | `version` ကို `2.11.18` သို့ bump လုပ်ရန် |
| `docs/RELEASE_2.11.18_EN.md`, `docs/RELEASE_2.11.18_MM.md` | v2.11.18 အတွက် bilingual release notes ရေးသားရန် |
| `docs/CURRENT_STATUS_EN.md`, `docs/CURRENT_STATUS_MM.md` | v2.11.18 prep step ကို မှတ်တမ်းတင်ရန် |

`scripts/validate_release_version.sh` သည် surface အပေါ်ရှိ အချက်အလက်အားလုံးကို TSV report တစ်ခုတည်းဖြင့် enforce လုပ်ပါသည်။ `scripts/release_preflight.sh` တွင် B4 rust-free full-language contract ၏ witness check (status label နှင့် acceptance row count) ကို ထပ်ဆင့်ထည့်သွင်းထားပြီး acceptance row များက ဆက်လက် `provisional` / `not-certified` ဖြစ်နေသော်လည်း contract ကို preflight log ထဲတွင် မှတ်တမ်းတင်ပါသည်။

## Preflight workflow

`.github/workflows/prepare-v2.11.18.yml` သည် preflight scripts သို့မဟုတ် B4 contract ကို ထိမိသော `master` push တိုင်းတွင် CI-side v2.11.18 preflight ကို run ပါသည်။ `workflow_dispatch` ဖြင့်လည်း run နိုင်ပါသည်။ ၎င်းသည် `EXPECTED_VERSION=2.11.18`၊ `RELEASE_TAG=v2.11.18` နှင့် `ZAP_SKIP_RELEASE_NOTES=1` ကို သတ်မှတ်ပြီး bilingual release notes မရေးသားမီ အခြား required surface အားလုံးကို exercise လုပ်ပါသည်။ `release.yml` workflow သည် `v2.11.18` tag ပေါ်တွင် `RUN_CARGO_AUDIT=1`၊ `SKIP_DEPLOYMENT_VALIDATION=0` နှင့် `ZAP_SKIP_RELEASE_NOTES` override မပါဘဲ preflight ကိုယ်တိုင်ကို ပြန် run ပါသည်။

## Cross-platform baseline

`scripts/benchmark_b2_typed_ir.sh` ကို v2.11.18 preflight (P1-09) ထဲသို့ ချိတ်ဆက်ထားပါသည်။ ၎င်းသည် `(target_triple, suite)` baseline row ကို `benchmark-results/b2-typed-ir.baseline.tsv` ထဲသို့ ရေးသားပြီး M2-BENCH-01 compatible provenance sidecar ကိုလည်း ထုတ်ပေးပါသည်။ Release preflight သည် aggregator (`scripts/aggregate_b2_typed_ir.sh`) က deterministic per-suite summary CSV ကို ထုတ်ပေးရန် လိုအပ်ပြီး baseline table တွင် suite တစ်ခုချင်းစီအတွက် row အနည်းဆုံးရှိကြောင်း verify လုပ်ပါသည်။ `ci.yml` ရှိ cross-platform build matrix သည် `zap-b2-typed-ir-baseline-<sha>` ကို upload လုပ်သောကြောင့် release တစ်ခုချင်းအတွက် per-target execution evidence စုဆောင်းပါသည်။

## B4 contract witness

B4 rust-free full-language contract သည် supported target အားလုံးတွင် acceptance row အားလုံး pass မဖြစ်မချင်း `not-certified` အတိုင်း ရှိနေပါသည်။ Release preflight သည် contract status label (`not-certified` / `provisional` / `certified`) နှင့် B4 acceptance TSV ရှိ `provisional` နှင့် `not-certified` row အရေအတွက်ကို မှတ်တမ်းတင်ပါသည်။ Contract က certified မဟုတ်သရွေ့ preflight က `WARN` ထုတ်ပြီး ဆက်လက် run ပါသည်။ Release ကို ထိုအခြေအနေတွင် publish လုပ်ပါသည်။ Contract ကို `certified` သို့ ရွှေ့မည်ဆိုပါက supported target အားလုံးတွင် acceptance row အားလုံး pass ဖြစ်ပြီးနောက် doc/PR ဖြင့် မှတ်တမ်းတင်ရပါမည်။

## v2.11.18 acceptance gates

| Gate | အခြေအနေ |
|---|---|
| Cargo, lockfile, CLI, changelogs, READMEs, security, conformance matrix, vscode-extension နှင့် bilingual release notes တလျှောက် version consistency | `scripts/validate_release_version.sh` (CI) |
| B2 typed-IR cross-platform baseline | `scripts/test_b2_typed_ir_benchmark.sh` + `scripts/test_aggregate_b2_typed_ir.sh` (CI) |
| Documentation consistency, ownership, parity, fixed-seed replay, bounded replay, async boundary, platform archive, registry corpus, benchmark regression/provenance, stdlib policy, B0/B1/B2/B3 contracts, LSP/VS Code parity | `scripts/release_preflight.sh` (CI dry-run + tag run) |
| B4 contract witness | `scripts/release_preflight.sh` `check_b4_contract_witness` (status ကို မှတ်တမ်းတင်ပြီး certification မလိုအပ်) |

`release.yml` သည် tag commit ပေါ်တွင် preflight, cross-platform native build, immutable-tag publish job နှင့် downloaded asset verification အပါအဝင် end-to-end pass ဖြစ်မှသာ v2.11.18 tag ကို push လုပ်ပါမည်။
