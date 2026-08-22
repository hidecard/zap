# Zap v2.2.1 Release Notes

Zap v2.2.1 သည် published v2.2.0 release နောက်ပိုင်း corrective patch release ဖြစ်သည်။ `master` တွင် နောက်ပိုင်းမှ ပြီးစီးခဲ့သော reliability၊ LSP interoperability၊ editor delivery၊ standard-library metadata နှင့် documentation correction များကို ဤ release တွင် package လုပ်ထားသည်။

## Release provenance

Zap v2.2.1 ကို corrected post-v2.2.0 `master` history မှ တည်ဆောက်ထားသည်။ Published v2.2.0 tag သည် [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb) commit တွင် signed asset၊ checksum၊ provenance နှင့် release note များနှင့်အတူ immutable အဖြစ် ဆက်ရှိသည်။ နောက်ပိုင်း fix များပါဝင်လာစေရန် v2.2.0 tag သို့မဟုတ် asset တစ်ခုမျှ rewrite မလုပ်ထားပါ။

## LSP document synchronization

Native LSP server သည် standard full-document synchronization ကို advertise နှင့် implement လုပ်သည်။ `initialize` တွင် `textDocumentSync` ကို `openClose: true` နှင့် `change: 1` ဖြင့် ပြသည်။ `didOpen` နှင့် `didChange` သည် လက်ခံထားသော document text နှင့် version ကို သိမ်းဆည်းပြီး diagnostics များကို accepted buffer မှ ထုတ်ပေးသည်။ Version အသစ်ထက် နောက်ကျသော stale update များကို newer state မပျက်စီးစေဘဲ ignore လုပ်သည်။ v2.2.1 သည် full-sync only ကို ရည်ရွယ်ထားသောကြောင့် incremental range change များကို လုံခြုံစွာ reject လုပ်သည်။

## File-local semantic rename

Rename သည် same-spelled lexer token အားလုံးကို ပြောင်းခြင်းအစား active document အတွင်းရှိ lexical binding များကို resolve လုပ်သည်။ Function၊ class၊ module၊ `let`၊ `for`၊ `catch`၊ parameter၊ nested closure နှင့် imported alias များ၏ declaration/reference များကို nearest-scope shadowing ဖြင့် resolve လုပ်သည်။ String၊ comment၊ keyword၊ catalog builtin နှင့် `.` နောက်မှ member name များကို မပြောင်းပါ။ Cross-file rename ကို support မလုပ်သေးဘဲ ပြန်ပေးသော edit များကို active URI အတွင်းတွင်သာ ကန့်သတ်ထားသည်။

## LSP interoperability နှင့် bounds

Server သည် client capability list မှ UTF-8၊ UTF-16 သို့မဟုတ် UTF-32 position encoding ကို negotiate လုပ်ပြီး preference မပေးပါက UTF-16 ကို default သုံးသည်။ File URI များတွင် strict percent-decoding ကို သုံးပြီး malformed escape၊ URI host၊ NUL byte နှင့် traversal segment များကို reject လုပ်သည်။ Workspace indexing တွင် document count၊ import depth နှင့် workspace byte စုစုပေါင်းကို bound လုပ်ထားသဖြင့် oversized သို့မဟုတ် unsafe input များကို skip/reject လုပ်နိုင်ပြီး လက်ခံထားပြီးသား document state ကို မဖယ်ရှားပါ။

## Canonical VS Code extension delivery

`vscode-extension/` သည် Zap extension ၏ canonical distributable source ဖြစ်သည်။ Manifest ကို 2.2.1 အဖြစ် version လုပ်ထားပြီး publisher `ArkarYan` ကို ထိန်းသိမ်းကာ rename ကို native LSP သို့ delegate လုပ်ပြီး package smoke contract ဖြင့် စစ်ဆေးသည်။ `editors/vscode/` သည် catalog-aligned static grammar နှင့် configuration mirror အဖြစ် ဆက်ရှိသည်။ Package contract သည် metadata၊ grammar/configuration parity၊ catalog builtin coverage၊ archive integrity နှင့် generated/VCS entry မပါဝင်မှုကို စစ်ဆေးသည်။

## Standard-library determinism taxonomy

Standard-library catalog သည် schema version 2 ကို အသုံးပြုပြီး public domain နှင့် builtin တစ်ခုချင်းစီအတွက် `determinism_class` ကို ဖော်ပြသည်။ Public label များမှာ `pure`၊ `input-deterministic`၊ `runtime-dependent` နှင့် `external-io` ဖြစ်သည်။ Domain default နှင့် reviewed builtin exception များကို explicit သတ်မှတ်ထားပြီး pure builder၊ input-dependent transform၊ clock/environment access နှင့် network/process operation များကို ခွဲခြားထားသည်။ Legacy `deterministic` boolean ကို compatibility view အဖြစ် ဆက်လက်ရရှိနိုင်ပြီး `pure` နှင့် `input-deterministic` entry များအတွက်သာ true ဖြစ်သည်။

## Documentation နှင့် traceability

English နှင့် Burmese policy၊ index၊ roadmap၊ TODO၊ progress၊ navigation၊ README နှင့် changelog surface များသည် correction boundary တစ်ခုတည်းကို ဖော်ပြသည်။ [v2.2.0 နောက်ပိုင်း remediation record](POST_V2.2.0_REMEDIATION_MM.md) တွင် immutable v2.2.0 tag နောက်ပိုင်း ရောက်ရှိလာသော commit များနှင့် ကျန်ရှိသော limitation များကို ခွဲခြားဖော်ပြထားသည်။ Release note နှင့် documentation consistency harness များသည် bilingual section နှင့် code-fence parity ကို ထိန်းသိမ်းသည်။

## Compatibility နှင့် known limitations

ဤ release သည် ရှိပြီးသား Zap language နှင့် package contract များကို ထိန်းသိမ်းကာ corrective metadata နှင့် editor behavior များကို explicit ပြုလုပ်သည်။ LSP synchronization သည် full-text only ဖြစ်ပြီး incremental range edit ကို apply မလုပ်ပါ။ Rename သည် file-local ဖြစ်ပြီး cross-file edit များကို ပြန်မပေးပါ။ Async runtime သည် full production reactor မဟုတ်ဘဲ deterministic bounded foundation အဖြစ် ဆက်ရှိပြီး multi-thread-safe task state နှင့် external production deployment control များသည် ဤ patch scope ပြင်ပတွင် ရှိသည်။ Traits နှင့် composition သည် design-only ဖြစ်ပြီး syntax ကို support မလုပ်သေးပါ။

## Verification နှင့် reproducibility

Release candidate သည် pinned Rust formatting check၊ warnings denied strict Clippy၊ complete native unit/integration suite၊ wire-level LSP synchronization probe၊ LSP semantic parity၊ canonical VS Code package contract၊ standard-library policy harness၊ Cargo-authoritative release-version validation၊ bilingual documentation consistency နှင့် regression tests၊ specification ownership validation နှင့် `git diff --check` များကို အောင်မြင်ထားသည်။ Cargo.lock ကို regenerate/update မလုပ်ဘဲ `zap-native` package version stanza ကိုသာ 2.2.0 မှ 2.2.1 သို့ patch လုပ်ထားသည်။

## Upgrade guidance

Target platform နှင့် ကိုက်ညီသော archive ကို [v2.2.1 GitHub release](https://github.com/hidecard/zap/releases/tag/v2.2.1) မှ download လုပ်ပြီး published checksum နှင့် signature/provenance information ကို verify လုပ်ပါ။ ထို့နောက် [English README](../README.md) သို့မဟုတ် [Burmese README](../README_MM.md) ထဲရှိ installation instruction ကို လိုက်နာပါ။ Historical v2.2.0 asset များကို လိုအပ်သူများသည် immutable [v2.2.0 release](https://github.com/hidecard/zap/releases/tag/v2.2.0) ကို ဆက်လက်အသုံးပြုနိုင်သည်။

## References

[1]: https://github.com/hidecard/zap/releases/tag/v2.2.0 "Zap v2.2.0 release"
[2]: https://github.com/hidecard/zap/releases/tag/v2.2.1 "Zap v2.2.1 release"
[3]: https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb "Zap v2.2.0 tag commit"
