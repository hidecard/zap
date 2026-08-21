# Zap v2.1.6 Release Notes

**Release date:** 2026-08-21  
**Release tag:** `v2.1.6`

## အနှစ်ချုပ်

Zap v2.1.6 သည် type-checking conformance၊ diagnostic consistency၊ reproducible release tooling နှင့် platform အစုံ CI reliability တို့ကို အဓိကထားသော hardening release ဖြစ်ပါသည်။ v2.1.5 preparation အတွင်း ပြင်ဆင်ထားသော source၊ packaging၊ signing နှင့် publication checks များ အောင်မြင်ပြီးနောက် ဤအလုပ်များကို versioned release အဖြစ် မြှင့်တင်ထားပါသည်။

## Type-checking နှင့် Diagnostics

ဤ release တွင် အတည်ပြုထားသော TC-001 မှ TC-012 အထိ conformance baseline ကို documentation နှင့် CI gate များဖြင့် မှတ်တမ်းတင်ထားပါသည်။ Loop-boundary narrowing၊ conditional-expression typing၊ alias နှင့် wrapper narrowing၊ generic collection နှင့် variant annotation validation၊ stable JSON diagnostics နှင့် CLI/LSP diagnostic agreement တို့ကို named test နှင့် CI gate များဖြင့် စစ်ဆေးထားပါသည်။

CLI နှင့် LSP တို့သည် `TypeError` code၊ normalized messages နှင့် source locations များအတွက် shared source-diagnostic bridge တစ်ခုတည်းကို အသုံးပြုပါသည်။ Legacy lint-line behavior များကို မပျက်စေရန် ထိန်းသိမ်းထားပြီး diagnostic parity regression coverage ကိုလည်း သီးခြားစစ်ဆေးထားပါသည်။

## Toolchain နှင့် CI Hardening

Repository တွင် `rustfmt` နှင့် `clippy` components ပါဝင်သော Rust `1.75.0` ကို pin လုပ်ထားပါသည်။ Strict Clippy ကို `-D warnings` ဖြင့် run ပြီး release build မစမီ formatting၊ Clippy၊ Cargo check၊ native tests၊ conformance gates နှင့် diagnostic-parity tests များကို CI quality job က စစ်ဆေးပါသည်။

Release workflow သည် Linux x86_64၊ macOS ARM64 နှင့် Windows x86_64 builds များကို သီးခြား validate လုပ်ပါသည်။ Target တစ်ခုချင်းစီအတွက် native tests၊ CLI smoke tests၊ installer checks၊ archive-content checks နှင့် reproducibility checks များ run ပါသည်။

## Release Engineering

Release process တွင် dry-run-first `scripts/bump_release.sh` helper၊ tag-gated `scripts/release_preflight.sh`၊ deterministic archive packaging၊ per-artifact SHA-256 sidecars၊ aggregate checksum file၊ `zap.release-manifest.v1`၊ detached GPG signatures၊ `zap.provenance.v1` နှင့် post-publication verification တို့ ပါဝင်ပါသည်။

Unix archives များကို repository-owned deterministic tar.gz helper ဖြင့် ဖန်တီးပါသည်။ Windows ZIP entries များတွင် deterministic ordering နှင့် ZIP ကထောက်ပံ့သော အနိမ့်ဆုံး timestamp ကို အသုံးပြုပါသည်။ Published-release verifier သည် `pipefail` အခြေအနေတွင် archive listing အပြည့်အစုံကို မှန်ကန်စွာ consume လုပ်ပြီး archive contents၊ checksums၊ manifest/provenance consistency၊ signatures နှင့် published signing key တို့ကို စစ်ဆေးပါသည်။

## Supported Targets

| Platform | Target | Artifact |
|---|---|---|
| Linux | x86_64 GNU | `zap-2.1.6-linux-x86_64.tar.gz` |
| macOS | ARM64 | `zap-2.1.6-macos-arm64.tar.gz` |
| Windows | x86_64 MSVC | `zap-2.1.6-windows-x86_64.zip` |

## Release Verification

Release assets အားလုံးနှင့် public verification key ကို download လုပ်ပြီး isolated GPG keyring ထဲသို့ import လုပ်ပြီးနောက် အောက်ပါ command ကို run ပါ။

```bash
GNUPGHOME=/secure/verification/gnupg \
  bash scripts/verify_published_release.sh 2.1.6 ./published-release
```

Command သည် `published release verification: PASSED` ဟု ပြရပါမည်။ Checksum၊ signature၊ provenance၊ archive-content သို့မဟုတ် installer check တစ်ခုခု မအောင်မြင်ပါက asset ကို install သို့မဟုတ် ပြန်လည်ဖြန့်ဝေခြင်း မပြုရပါ။

## Upgrade နှင့် Rollback

Installation နှင့် CLI smoke test အောင်မြင်သည်အထိ ယခင် stable release ကို ဆက်လက်သိမ်းထားသင့်ပါသည်။ Release check တစ်ခုခု မအောင်မြင်ပါက release ကို quarantine လုပ်ပြီး `docs/RELEASE_ROLLBACK_RUNBOOK_EN.md` သို့မဟုတ် `docs/RELEASE_ROLLBACK_RUNBOOK_MM.md` ကို လိုက်နာပါ။ မတူညီသော bytes များအတွက် tag တစ်ခုတည်းကို ပြန်အသုံးမပြုရပါ။

## Operational Boundaries

Repository ထဲတွင် production private keys၊ passphrases၊ registry secrets၊ certificates သို့မဟုတ် infrastructure credentials များ မပါဝင်ပါ။ Protected release environment သည် `ZAP_RELEASE_GPG_PRIVATE_KEY` နှင့် လိုအပ်ပါက `ZAP_RELEASE_GPG_PASSPHRASE` ကို ပေးရပါမည်။ Key distribution၊ rotation၊ release approval နှင့် incident communication များသည် ခွင့်ပြုချက်ရှိသော operator တာဝန်များ ဖြစ်ပါသည်။

## Documentation

English နှင့် Burmese release notes များကို အတွဲလိုက် ထိန်းသိမ်းထားပါသည်။ ထပ်မံဖတ်ရှုရန် type-check conformance matrices၊ `docs/RELEASE_SIGNING_EN.md`၊ `docs/RELEASE_SIGNING_MM.md`၊ deployment documentation နှင့် v2.1 roadmap documents များကို ကြည့်ရှုနိုင်ပါသည်။
