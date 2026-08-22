# v2.2.0 နောက်ပိုင်း ပြင်ဆင်မှုနှင့် Provenance မှတ်တမ်း

**မှတ်တမ်းအခြေအနေ:** v2.2.1 corrective release ထုတ်ဝေပြီး၊ နောက်ဆက်တွဲ engineering queue ကို သီးခြား track လုပ်ထားသည်

## အကျုံးဝင်မှုနှင့် provenance

ဤမှတ်တမ်းသည် Zap v2.2.0 ၏ deep technical review အပြီး စတင်ခဲ့သော corrective-release cycle ကို မှတ်တမ်းတင်သည်။ ထုတ်ဝေပြီးသား v2.2.0 release notes နှင့် သီးခြားထားရခြင်းမှာ သမိုင်းမှတ်တမ်းကို မပြန်ရေးရန် ဖြစ်သည်။ Published **v2.2.0 tag သည် immutable** ဖြစ်ပြီး [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb) commit ကို ညွှန်ပြထားသည်။ အောင်မြင်ခဲ့သော release workflow ကို [GitHub Actions run 32546657968](https://github.com/hidecard/zap/actions/runs/32546657968) တွင် မှတ်တမ်းတင်ထားသည်။ နောက်ပိုင်း correction များကို ထို historical tag သို့ backport၊ force-push သို့မဟုတ် v2.2.0 ၏ အစိတ်အပိုင်းဟု မဖော်ပြပါ။

လက်ရှိ `master` branch တွင် နောက်ပိုင်း corrective commit များ ပါရှိသည်။ ထိုပြောင်းလဲမှုများကို **v2.2.1** အဖြစ် package ပြုလုပ်ပြီး ထုတ်ဝေထားသည်။ v2.2.0 archive ကို install လုပ်သော user များသည် v2.2.0-tagged behavior ကို ရရှိမည်ဖြစ်ပြီး v2.2.1 release တွင် အောက်တွင် ဖော်ပြထားသော corrective behavior များ ပါဝင်သည်။

| Provenance boundary | Commit သို့မဟုတ် record | အဓိပ္ပာယ် |
|---|---|---|
| Published v2.2.0 | [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb) နှင့် [release v2.2.0](https://github.com/hidecard/zap/releases/tag/v2.2.0) | Historical release asset၊ checksum၊ provenance နှင့် release note များ ဖြစ်ပြီး မပြောင်းလဲပါ။ |
| LSP synchronization correction | [`c2a662f`](https://github.com/hidecard/zap/commit/c2a662f) | Standard full-document `didChange`၊ version tracking၊ accepted-buffer diagnostics နှင့် unsupported range change များကို safe rejection ပြုလုပ်ခြင်း။ |
| Semantic rename correction | [`eed2dc4`](https://github.com/hidecard/zap/commit/eed2dc4) | Shadowing၊ closure၊ parameter နှင့် import alias ပါဝင်သော file-local lexical binding resolution။ |
| LSP interoperability correction | [`cdf2aa1`](https://github.com/hidecard/zap/commit/cdf2aa1) | Negotiated position encoding၊ strict file URI နှင့် workspace bounds၊ encoding-aware range များ။ |
| VS Code delivery correction | [`f77f265`](https://github.com/hidecard/zap/commit/f77f265) | Canonical extension source၊ package validation၊ catalog-aligned asset နှင့် native-LSP rename provider။ |
| Standard-library determinism correction | [`2c4c928`](https://github.com/hidecard/zap/commit/2c4c928) | Schema-2 `determinism_class` taxonomy၊ legacy-boolean compatibility၊ explicit builtin exception နှင့် bilingual policy update။ |

## Corrective milestones

| Milestone | အခြေအနေ | ပြင်ဆင်ထားသော contract နှင့် evidence |
|---|---|---|
| LSP-SYNC-01 | အကောင်အထည်ဖော်ပြီး | Server သည် `openClose: true` နှင့် `change: 1` ပါသော `textDocumentSync` ကို ကြေညာသည်။ Versioned document text ကို သိမ်းဆည်းပြီး standard full-text `params.contentChanges` ကို အသုံးပြုကာ stale update ကို မူလ accepted state မပျက်စီးစေဘဲ diagnostics ကို ထို state မှ ထုတ်ပေးသည်။ Incremental range change ကို အနီးစပ်ဆုံး apply မလုပ်ဘဲ reject လုပ်သည်။ Unit test နှင့် `scripts/test_lsp_protocol_sync.sh` သည် wire-level behavior ကို စစ်ဆေးသည်။ |
| LSP-REN-01 | အကောင်အထည်ဖော်ပြီး | Active file အတွင်း function၊ class၊ module၊ `let`၊ `for`၊ `catch`၊ parameter၊ nested closure နှင့် import alias များအတွက် အနီးဆုံး lexical declaration ကို resolve လုပ်သည်။ Comment၊ string၊ `.` နောက်က member name၊ keyword နှင့် catalog builtin များကို မထိပါ။ Cross-file rename ကို ရည်ရွယ်ချက်ရှိရှိ support မလုပ်သေးပါ။ |
| LSP-INTEROP-01 | အကောင်အထည်ဖော်ပြီး | Position encoding ကို UTF-8၊ default UTF-16 သို့မဟုတ် UTF-32 အဖြစ် negotiate လုပ်သည်။ Malformed escape၊ URI host၊ NUL byte နှင့် decoded traversal ပါသော file URI များကို reject လုပ်သည်။ Workspace index ကို document ၂၅၆ ခု၊ import level ၃၂ နှင့် source text ၃၂ MiB အထိ ကန့်သတ်ထားသည်။ |
| EXT-201 | အကောင်အထည်ဖော်ပြီး | `vscode-extension/` သည် canonical distributable source ဖြစ်သည်။ Manifest၊ grammar၊ configuration၊ catalog coverage၊ LSP content-change behavior၊ rename provider နှင့် `.vsix` archive layout ကို package contract ဖြင့် စစ်ဆေးသည်။ `editors/vscode/` သည် catalog-aligned static asset mirror အဖြစ် ဆက်ရှိသည်။ |
| API-301 | အကောင်အထည်ဖော်ပြီး | Catalog schema သည် version 2 ဖြစ်သည်။ `DeterminismClass` သည် `pure`၊ `input-deterministic`၊ `runtime-dependent` နှင့် `external-io` ကို ခွဲခြားသည်။ Schema-version-1 `deterministic` boolean ကို compatibility view အဖြစ် ဆက်ထိန်းထားပြီး ပထမနှစ် class များအတွက်သာ true ဖြစ်သည်။ Domain ၁၂ ခုနှင့် catalog builtin အားလုံးတွင် explicit, regression-tested classification ရှိပြီး path/log builder၊ URL transform၊ duration transform၊ clock access နှင့် environment/configuration access များအတွက် builtin-level exception များ ပါဝင်သည်။ |
| DOC-401 | ဤမှတ်တမ်း | ဤ bilingual record၊ roadmap/progress wording၊ navigation link နှင့် README status text များသည် immutable v2.2.0 release နှင့် post-release master correction များကို ခွဲခြားဖော်ပြသည်။ |

## Public contract နှင့် limitation များ

ပြင်ဆင်ပြီးသော LSP contract သည် ရည်ရွယ်ချက်ရှိရှိ ကန့်သတ်ထားသည်။ Standard full-text synchronization နှင့် version-aware state ကို support လုပ်သော်လည်း incremental range application ကို support မလုပ်သေးပါ။ File-local semantic rename ကို ပေးသော်လည်း cross-file rename ကို မပေးပါ။ UTF-8၊ UTF-16 သို့မဟုတ် UTF-32 position column ကို negotiate လုပ်နိုင်ပြီး strict file-URI နှင့် workspace-size boundary များကို enforce လုပ်သည်။ ဤ limitation များသည် normative ဖြစ်သဖြင့် README၊ English/Burmese LSP guide နှင့် v2.2.1 release note များတွင် ဆက်လက်မြင်သာရမည်။

Canonical VS Code package သည် rename ကို native LSP သို့ delegate လုပ်ပြီး checked-in grammar နှင့် configuration ကို package လုပ်သည်။ Repository package smoke test သည် metadata၊ source coverage၊ archive integrity နှင့် provider wiring ကို သက်သေပြသော်လည်း external VS Code host သို့မဟုတ် Marketplace installation တစ်ခုချင်းစီကို စမ်းသပ်ပြီးပြီဟု မဆိုလိုပါ။ Extension version သည် Cargo-authoritative release version နှင့် ချိတ်ဆက်ထားသည်။

Standard-library catalog သည် coarse claim တစ်ခုတည်း မပြုဘဲ determinism class ကို report လုပ်သည်။ Pure နှင့် input-deterministic transformation များကို runtime state သို့မဟုတ် external I/O ပေါ် မူတည်သော operation များမှ ခွဲခြားထားသည်။ Traits/composition proposal သည် design-only အဖြစ် ဆက်ရှိပြီး ဤ corrective cycle တွင် trait၊ interface၊ composition သို့မဟုတ် method-resolution parser/runtime implementation မထည့်သွင်းပါ။

## Verification နှင့် release policy

Corrective milestone တစ်ခုစီကို focused commit အဖြစ် push မလုပ်မီ pinned Rust formatting check၊ strict Clippy၊ complete native test suite၊ focused LSP protocol နှင့် semantic-parity test၊ canonical VS Code package contract၊ standard-library policy contract၊ release-version validation၊ bilingual documentation consistency၊ specification ownership နှင့် `git diff --check` အားလုံး အောင်မြင်ရမည်။ API-301 သည် authoritative version 2.2.0 ရှိနေစဉ် ထို gates အားလုံးကို pass လုပ်ခဲ့သောကြောင့် taxonomy commit သည် v2.2.0 artifact ကို ပြင်ဆင်ခြင်းမဟုတ်ဘဲ post-release `master` correction ဖြစ်သည်။

v2.2.1 release ကို clean commit မှ ပြင်ဆင်ပြီး Cargo၊ manually patched package lock entry၊ CLI output၊ VS Code manifest နှစ်ခု၊ changelog၊ bilingual README/archive link၊ security metadata၊ documentation နှင့် release note များသည် တစ်ပြေးညီဖြစ်ခဲ့သည်။ Release preflight ကို `EXPECTED_VERSION=2.2.1` ဖြင့် run လုပ်ပြီး GitHub Actions workflow က platform၊ signing၊ checksum၊ provenance နှင့် publication condition များအားလုံးကို စစ်ဆေးအတည်ပြုခဲ့သည်။ ထိုလုပ်ငန်းစဉ်အတွင်း v2.2.0 tag နှင့် release asset များကို မထိခဲ့ပါ။

## Next release boundary

ထုတ်ဝေပြီးသော v2.2.1 note များတွင် ပြင်ဆင်ပြီးသော LSP synchronization၊ file-local rename၊ position/URI/workspace bounds၊ canonical VS Code package နှင့် determinism taxonomy တို့ကို အကျဉ်းချုပ်ဖော်ပြထားသည်။ Full-sync-only နှင့် file-local-only limitation များကိုလည်း ရှင်းလင်းဖော်ပြပြီး ဤမှတ်တမ်းသို့ link ချိတ်ထားသည်။ v2.2.1 သည် patch release အသစ်ဖြစ်ပြီး v2.2.0 ကို retag သို့မဟုတ် rewrite မလုပ်ထားပါ။

## References

1. [Published v2.2.0 tag](https://github.com/hidecard/zap/releases/tag/v2.2.0) နှင့် [immutable tag commit](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb)။
2. [Successful v2.2.0 release workflow](https://github.com/hidecard/zap/actions/runs/32546657968)။
3. [LSP synchronization contract](ASYNC_LSP_MM.md) နှင့် [protocol regression harness](../scripts/test_lsp_protocol_sync.sh)။
4. [Standard-library stability and determinism policy](STDLIB_POLICY_MM.md)။
5. [v2.2.0 historical release notes](RELEASE_2.2.0_MM.md) — ဤ record သည် ၎င်းကို ပြန်မရေးပါ။
6. [Published v2.2.1 release](https://github.com/hidecard/zap/releases/tag/v2.2.1)၊ [v2.2.1 release workflow](https://github.com/hidecard/zap/actions/runs/32575824809) နှင့် [v2.2.1 release notes](RELEASE_2.2.1_MM.md)။
