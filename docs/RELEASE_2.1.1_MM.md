# Zap v2.1.1 Release Notes

**Release date:** 2026-08-21
**Release tag:** `v2.1.1`

## အနှစ်ချုပ်

Zap v2.1.1 သည် ပြီးစီးထားသော v2.1-D runtime/tooling အလုပ်များနှင့် v2.1-E release-engineering pipeline ကို အခြေခံထားသည့် ပထမဆုံး protected v2.1 release ဖြစ်ပါသည်။ ဤ release တွင် production-oriented asynchronous execution၊ bounded I/O adapters၊ registry service controls၊ synchronized developer tooling၊ reproducible artifacts၊ signed metadata၊ provenance နှင့် post-publish verification တို့ ပါဝင်ပါသည်။

## Runtime နှင့် Language

ဤ release တွင် joinable tasks၊ deterministic tick-based timeout behavior၊ cancellation-aware propagation၊ task readiness checks၊ typed task failure handling နှင့် language-level `spawn`၊ `task_join`၊ `task_is_ready` builtins များ ပါဝင်ပါသည်။ Repeated join နှင့် cancellation precedence များကို regression tests ဖြင့် စစ်ဆေးထားပါသည်။

Threaded runtime သည် bounded asynchronous file reads၊ non-blocking TCP request/response exchange နှင့် hard deadline/output limit ပါသော asynchronous process execution ကို ပေးပါသည်။ Forced child-process cancellation ဖြစ်ပါက child ကို terminate လုပ်ပြီး output ကို drain လုပ်သဖြင့် control မရှိသော process ကျန်မနေစေရန် ထိန်းချုပ်ထားပါသည်။

## Type-checking နှင့် Diagnostics

ဤ release တွင် TC-006 မှ TC-012 အထိ conformance baseline ကို ပြီးစီးထားပါသည်။ Loop-boundary restoration၊ option/result alias narrowing၊ conditional-expression typing၊ generic collection နှင့် variant annotation validation၊ stable L3 JSON diagnostics နှင့် L4 CLI/LSP diagnostic agreement တို့ ပါဝင်ပါသည်။ LSP သည် shared source-diagnostic bridge ကို ပြန်လည်အသုံးပြုပြီး CLI checking နှင့် တူညီသော `TypeError` code၊ normalized message နှင့် source-location semantics များကို ထုတ်ပေးပါသည်။ Legacy lint line behavior ကိုလည်း မပျက်စေရန် ထိန်းသိမ်းထားပါသည်။

## Tooling

Formatter၊ LSP server နှင့် VS Code extension တို့သည် finalized async vocabulary တစ်ခုတည်းကို အသုံးပြုပါသည်။ LSP completion၊ diagnostics၊ formatting၊ signature help၊ hover၊ go-to-definition၊ recursive document symbols နှင့် module-aware package indexing များကို local နှင့် unopened files များအတွက် synchronize လုပ်ထားပါသည်။

## Registry နှင့် Deployment

Zap တွင် authenticated loopback registry service၊ signed-index persistence၊ safe path handling၊ trusted-registry controls၊ bounded transport behavior၊ cache verification နှင့် deterministic failure paths များ ပါဝင်ပါသည်။ Reference deployment artifacts များတွင် systemd၊ Nginx TLS termination၊ environment boundaries နှင့် machine-readable deployment policy တို့ ပါဝင်ပါသည်။ Production host provisioning၊ certificates၊ DNS၊ WAF/rate limiting၊ monitoring နှင့် secret-manager setup များသည် operator တာဝန်များအဖြစ် ကျန်ရှိပါသည်။

## Release Engineering

Release pipeline တွင် dry-run-first version/changelog bump helper၊ tag-gated release preflight၊ deterministic three-target artifact aggregation၊ per-artifact နှင့် aggregate SHA-256 verification၊ `zap.release-manifest.v1`၊ `zap.provenance.v1`၊ detached GPG signatures၊ `zap-2.1.1-release-signing-key.asc` public signing-key asset နှင့် post-publish release verification တို့ ပါဝင်ပါသည်။ Unix archive များအတွက် repository-owned platform-neutral deterministic tar.gz helper ကို အသုံးပြုပြီး Windows ZIP entry များအတွက် ZIP က ထောက်ပံ့သော အနိမ့်ဆုံး timestamp ကို အသုံးပြုသဖြင့် reproducible packaging ကို ထိန်းသိမ်းထားပါသည်။ Release incident များအတွက် bilingual rollback/quarantine runbook ကိုလည်း ထည့်သွင်းထားပါသည်။

## Supported Targets

| Platform | Target | Artifact |
|---|---|---|
| Linux | x86_64 GNU | `zap-2.1.1-linux-x86_64.tar.gz` |
| macOS | ARM64 | `zap-2.1.1-macos-arm64.tar.gz` |
| Windows | x86_64 MSVC | `zap-2.1.1-windows-x86_64.zip` |

## Release Verification

Release assets အားလုံးနှင့် public verification key ကို download လုပ်ပြီး trusted public key ကို isolated GPG keyring ထဲသို့ import လုပ်ပြီးနောက် အောက်ပါ command ကို run ပါ။

```bash
GNUPGHOME=/secure/verification/gnupg \
  bash scripts/verify_published_release.sh 2.1.1 ./published-release
```

Command သည် `published release verification: PASSED` ဟု ပြရပါမည်။ Verification သည် archive set အပြည့်အစုံ၊ per-artifact sidecars၊ aggregate checksums၊ manifest/provenance consistency၊ သတ်မှတ်ထားသော archive entries နှင့် detached signatures တစ်ခုချင်းစီကို စစ်ဆေးပါသည်။ Check တစ်ခုခု မအောင်မြင်ပါက asset ကို install သို့မဟုတ် ပြန်လည်ဖြန့်ဝေခြင်း မပြုရပါ။

## Upgrade နှင့် Rollback

အသုံးပြုသူများသည် new installation နှင့် CLI smoke test အောင်မြင်သည်အထိ ယခင် stable release ကို ဆက်လက်သိမ်းထားသင့်ပါသည်။ Checksum၊ signature၊ provenance၊ installer၊ registry index သို့မဟုတ် runtime check တစ်ခုခု မအောင်မြင်ပါက release ကို quarantine လုပ်ပြီး `docs/RELEASE_ROLLBACK_RUNBOOK_EN.md` သို့မဟုတ် `docs/RELEASE_ROLLBACK_RUNBOOK_MM.md` ကို လိုက်နာပါ။ မတူညီသော bytes များအတွက် tag တစ်ခုတည်းကို ပြန်အသုံးမပြုရပါ။

## သိရှိထားသော Operational Boundaries

Repository ထဲတွင် production private keys၊ passphrases၊ registry secrets၊ certificates သို့မဟုတ် infrastructure credentials များ မပါဝင်ပါ။ Protected release environment သည် `ZAP_RELEASE_GPG_PRIVATE_KEY` နှင့် လိုအပ်ပါက `ZAP_RELEASE_GPG_PASSPHRASE` ကို ပေးရပါမည်။ Public-key trust distribution၊ key rotation၊ release approval နှင့် incident communication များသည် ခွင့်ပြုချက်ရှိသော operator တာဝန်များ ဖြစ်ပါသည်။

## Documentation

English နှင့် Burmese release notes များကို အတွဲလိုက် ထိန်းသိမ်းထားပါသည်။ ထပ်မံဖတ်ရှုရန် `docs/RELEASE_SIGNING_EN.md`၊ `docs/RELEASE_SIGNING_MM.md`၊ `docs/DEPLOYMENT_EN.md`၊ `docs/DEPLOYMENT_MM.md` နှင့် v2.1 roadmap documents များကို ကြည့်ရှုနိုင်ပါသည်။
