# Zap v2.1.7 Release Notes

**Release date:** 2026-08-21  
**Release tag:** `v2.1.7`

## အနှစ်ချုပ်

Zap v2.1.7 သည် reliability နှင့် release-engineering patch release ဖြစ်ပါသည်။ Bilingual specification ownership index ကို stable rule ID ၂၇ ခုအထိ ချဲ့ထွင်ပြီး P0/P1 contract validation ကို ခိုင်မာစေပါသည်။ ပြင်ဆင်ပြီးသား Windows နှင့် macOS cross-platform behavior များကို ဆက်လက်ထိန်းသိမ်းထားပြီး release preflight သည် deployment validation မတိုင်မီ ownership၊ native/legacy parity၊ fixed-seed replay နှင့် focused async gate များကို run လုပ်ပါသည်။

## Specification ownership နှင့် compatibility

Canonical English နှင့် Burmese language specification များသည် machine-readable rule-to-section-to-fixture index သို့ link ချိတ်ထားပါသည်။ Index သည် source execution၊ precedence၊ typing၊ functions၊ modules၊ memory၊ deterministic/production async boundary၊ diagnostics၊ registry၊ lockfile၊ JSON/filesystem limits၊ standard-library catalog၊ CLI JSON၊ compatibility policy နှင့် CI enforcement များကို လွှမ်းခြုံထားပါသည်။ Validator သည် bilingual section မရှိခြင်း၊ fixture owner မရှိခြင်း၊ duplicate ID၊ invalid policy value နှင့် required domain မရှိခြင်းများကို reject လုပ်ပါသည်။

အနာဂတ် normative၊ compatibility၊ deprecated သို့မဟုတ် rejected behavior များအတွက် bilingual compatibility/deprecation change template များကို ထည့်သွင်းထားပါသည်။ Behavior အသစ်တစ်ခုသည် legacy acceptance တစ်ခုတည်းကို အားမကိုးဘဲ canonical section၊ fixture owner၊ migration path၊ version impact နှင့် verification evidence များကို explicit သတ်မှတ်ရမည်။

## Verification နှင့် CI hardening

P1-05 fixed-seed replay layer သည် parser၊ JSON၊ lockfile၊ registry၊ memory နှင့် async failure fixture များကို deterministic evidence ဖြင့် ဆက်လက်ထိန်းသိမ်းထားပါသည်။ P0-01 native/legacy matrix သည် versioned common၊ native-only နှင့် rejected fixture ခြောက်ခု၏ normalized output digest များကို နှိုင်းယှဉ်ပါသည်။ P0-05 focused async matrix သည် Linux x86_64၊ Windows x86_64 နှင့် macOS ARM64 တွင် process၊ file၊ socket၊ deadline၊ cancellation နှင့် output-limit case များကို ဆက်လက် run လုပ်ပါသည်။

Repository သည် Rust `1.75.0` ကို `rustfmt` နှင့် `clippy` component များဖြင့် pin လုပ်ထားပါသည်။ Strict Clippy ကို `-D warnings` ဖြင့် run လုပ်ပြီး CI သည် release build မတိုင်မီ formatting၊ Cargo check၊ native tests၊ conformance gate၊ ownership validation၊ parity၊ replay၊ async matrix နှင့် deployment-policy validation များကို run လုပ်ပါသည်။

## Release engineering

Release process သည် dry-run-first `scripts/bump_release.sh` နှင့် tag-gated `scripts/release_preflight.sh` ကို အသုံးပြုပါသည်။ Preflight သည် deployment validation မတိုင်မီ P0/P1 contract gate လေးမျိုးကို run လုပ်ပြီး formatting၊ strict Clippy၊ Cargo check/test၊ bilingual documentation check၊ target validation၊ source safety နှင့် repository cleanliness များကို ဆက်လက်စစ်ဆေးပါသည်။ Release artifact များတွင် deterministic archive၊ artifact တစ်ခုချင်းစီ၏ SHA-256 sidecar၊ aggregate manifest၊ provenance၊ detached signature နှင့် post-publication verification များ ဆက်လက်ပါဝင်ပါသည်။

## Supported targets

| Platform | Target | Artifact |
|---|---|---|
| Linux | x86_64 GNU | `zap-2.1.7-linux-x86_64.tar.gz` |
| macOS | ARM64 | `zap-2.1.7-macos-arm64.tar.gz` |
| Windows | x86_64 MSVC | `zap-2.1.7-windows-x86_64.zip` |

## Release verification

Release asset အားလုံးနှင့် public verification key ကို download လုပ်ပြီး isolated GPG keyring ထဲသို့ key import လုပ်ကာ အောက်ပါ command ကို run ရမည်။

```bash
GNUPGHOME=/secure/verification/gnupg \
  bash scripts/verify_published_release.sh 2.1.7 ./published-release
```

Command သည် `published release verification: PASSED` ဟု report ပြရမည်။ Checksum၊ signature၊ provenance၊ archive-content သို့မဟုတ် installer check တစ်ခုခု fail ဖြစ်ပါက asset ကို install သို့မဟုတ် redistribute မလုပ်ရပါ။

## Upgrade နှင့် rollback

Zap v2.1.7 install နှင့် CLI smoke test များ အောင်မြင်ကြောင်း မပြီးမချင်း v2.1.6 ကို ဆက်လက်ရရှိနိုင်အောင် ထားရမည်။ Release check တစ်ခုခု fail ဖြစ်ပါက release ကို quarantine လုပ်ပြီး [`RELEASE_ROLLBACK_RUNBOOK_EN.md`](RELEASE_ROLLBACK_RUNBOOK_EN.md) သို့မဟုတ် [`RELEASE_ROLLBACK_RUNBOOK_MM.md`](RELEASE_ROLLBACK_RUNBOOK_MM.md) ကို လိုက်နာရမည်။ Tag တစ်ခုတည်းကို bytes မတူသော release များအတွက် ပြန်အသုံးမပြုရပါ။

## Deferred boundaries

Executor-backed language scheduling၊ language-level async cancellation/timeout syntax၊ public weak references၊ tracing collection၊ long-running fuzz targets၊ allocator-level telemetry နှင့် ကျန် fragmented specification ownership အလုပ်များကို အတိအလင်း deferred ထားပါသည်။ ဤ release တွင် traits/composition implementation သို့မဟုတ် async syntax အကျယ်ပြန့်ကို မစတင်ပါ။

## Operational boundaries

Repository ထဲတွင် production private key၊ passphrase၊ registry secret၊ certificate သို့မဟုတ် infrastructure credential များ မပါဝင်ပါ။ Protected release environment တွင် `ZAP_RELEASE_GPG_PRIVATE_KEY` နှင့် လိုအပ်ပါက `ZAP_RELEASE_GPG_PASSPHRASE` ကို ပေးထားရမည်။ Key distribution၊ rotation၊ release approval နှင့် incident communication များသည် authorized operator တာဝန်များ ဖြစ်ပါသည်။

## Documentation

English နှင့် Burmese release note များကို pair အဖြစ် ထိန်းသိမ်းထားပါသည်။ ဆက်စပ် reference များမှာ [`SPEC_OWNERSHIP_INDEX.tsv`](SPEC_OWNERSHIP_INDEX.tsv)၊ [`SPEC_OWNERSHIP_EN.md`](SPEC_OWNERSHIP_EN.md)၊ [`SPEC_OWNERSHIP_MM.md`](SPEC_OWNERSHIP_MM.md)၊ type-checking conformance matrix များ၊ release-signing documentation၊ deployment documentation နှင့် v2.1 roadmap documents များ ဖြစ်ပါသည်။
