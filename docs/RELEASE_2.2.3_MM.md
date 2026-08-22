# Zap v2.2.3 Release Notes

Zap v2.2.3 သည် v2.2.2 နောက်ပိုင်း runtime-reliability release ဖြစ်သည်။ Bounded cycle-safe equality၊ checked object နှင့် lexical-frame borrow propagation၊ deterministic task နှင့် frame invariant fallback၊ LSP rename scope-stack hardening နှင့် English/Burmese documentation synchronization များကို ဤ release တွင် ထည့်သွင်းထားသည်။

## Release provenance

Zap v2.2.3 ကို published v2.2.2 release နောက်ပိုင်း `master` history မှ တည်ဆောက်ထားသည်။ v2.2.0 tag နှင့် release သည် [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb) commit တွင် immutable ဖြစ်ပြီး v2.2.1 သည် [`201fad4`](https://github.com/hidecard/zap/commit/201fad4c7fbee38e3fabf63bf17d50eb4d70f784) တွင် immutable ဖြစ်ကာ v2.2.2 သည် [`f932e21`](https://github.com/hidecard/zap/commit/f932e21d190f59e722bf17dfdc214cef75ade698) တွင် immutable ဖြစ်သည်။ အဆိုပါ release များ၏ tag၊ release၊ signed asset၊ checksum၊ provenance နှင့် release note များကို rewrite မလုပ်ထားပါ။

## Runtime equality safety

Canonical AST `==` နှင့် `!=` များသည် checked၊ bounded `try_values_equal` path ကို အသုံးပြုသည်။ List၊ map၊ `Result`၊ `Option` နှင့် `Future` value များကို recursive နှိုင်းယှဉ်ပြီး object pair များကို cycle guard ဖြင့် ကာကွယ်ထားသည်။ Callable value များတွင် handle identity ကို အသုံးပြုသည်။ Traversal ကို `MAX_RUNTIME_VALUE_NODES` ဖြင့် ကန့်သတ်ထားပြီး internal borrow conflict ဖြစ်ပါက panic မဖြစ်ဘဲ typed `BorrowError` result ကို ပြန်ပေးသည်။ Infallible `PartialEq` compatibility view ကို ဆက်လက်ထားရှိပြီး checked comparison မပြီးဆုံးနိုင်ပါက `false` ကို ပြန်ပေးသည်။

## Checked borrow နှင့် invariant hardening

Logical memory sizing၊ validation နှင့် canonical AST member read များတွင် object နှင့် `EnvFrame` borrow များက typed borrow failure ကို ပြန်လည် propagate လုပ်သည်။ Task join နှင့် function/method frame invariant path များတွင် unchecked existence သို့မဟုတ် frame-borrow `expect` path များအစား deterministic fallback များကို အသုံးပြုသည်။ ဤပြင်ဆင်မှုများသည် ရှိပြီးသား single-threaded `Rc`/`RefCell` runtime ကို ခိုင်မာစေပြီး ownership model အသစ် သို့မဟုတ် automatic cycle collection မထည့်သွင်းပါ။

## LSP rename boundary

LSP rename scope-stack path သည် internal scope invariant မရရှိပါက fail closed ပြုလုပ်ပြီး audit တွင် ကျန်ရှိသော `unwrap` path ကို ဖယ်ရှားထားသည်။ လက်ရှိ protocol boundary များ မပြောင်းလဲပါ။ Document synchronization သည် `change: 1` ပါသော full-text ဖြစ်ပြီး unsupported range edit များကို reject လုပ်သည်။ Semantic rename သည် cross-file မဟုတ်ဘဲ file-local အဖြစ်သာ ဆက်ရှိသည်။

## Documentation နှင့် traceability

English နှင့် Burmese README၊ release၊ memory၊ diagnostics၊ runtime-state၊ type-check၊ roadmap၊ policy၊ security၊ learner-guide နှင့် specification-ownership surface များတွင် v2.2.3 ကို လက်ရှိ release အဖြစ် ဖော်ပြထားသည်။ Bilingual release note နှစ်ခုလုံးတွင် runtime behavior နှင့် limitation များကို တူညီစွာ မှတ်တမ်းတင်ထားသည်။ Parser syntax အသစ်၊ runtime syntax အသစ်၊ traits implementation သို့မဟုတ် composition syntax မထည့်သွင်းပါ။

## Compatibility နှင့် known limitations

ဤ release သည် ရှိပြီးသား Zap language နှင့် package contract များကို ထိန်းသိမ်းထားသည်။ Runtime သည် single-threaded `Rc`/`RefCell` infrastructure အဖြစ် ဆက်ရှိသည်။ Strong reference cycle များကို ဆက်လက်ထောက်ပံ့သော်လည်း `clear_object_fields()` ဖြင့် explicit cleanup လုပ်ရမည်။ Public weak reference၊ automatic tracing collection နှင့် tracing collector များသည် unsupported ဖြစ်နေဆဲ ဖြစ်သည်။ Async function များသည် eager scheduled-value behavior အတိုင်း ဆက်ရှိပြီး lazy မဟုတ်ပါ။ LSP synchronization သည် full-text only ဖြစ်ကာ range change များကို reject လုပ်ပြီး rename သည် file-local ဖြစ်သည်။ Traits နှင့် composition သည် design-only RFC နှင့် unsupported syntax အဖြစ် ဆက်ရှိပြီး production reactor သို့မဟုတ် multi-thread runtime မဟုတ်ပါ။

## Verification နှင့် reproducibility

Release candidate သည် strict formatting နှင့် Clippy၊ serial native unit/integration suite၊ bilingual documentation consistency၊ specification ownership validation၊ standard-library policy၊ LSP protocol နှင့် semantic-parity check၊ VS Code package validation၊ parity/replay/async matrix၊ benchmark၊ packaging နှင့် archive manifest၊ signing နှင့် registry policy harness၊ release-version validation၊ clean-tree release preflight နှင့် `git diff --check` အားလုံးကို အောင်မြင်ရန် လိုအပ်သည်။ Cargo.lock ကို regenerate/update မလုပ်ဘဲ `zap-native` package version stanza ကိုသာ 2.2.2 မှ 2.2.3 သို့ manually synchronize လုပ်ထားသည်။

## Upgrade guidance

Target platform နှင့် ကိုက်ညီသော archive ကို [v2.2.3 GitHub release](https://github.com/hidecard/zap/releases/tag/v2.2.3) မှ download လုပ်ပြီး published checksum နှင့် signature/provenance information ကို verify လုပ်ပါ။ ထို့နောက် [English README](../README.md) သို့မဟုတ် [Burmese README](../README_MM.md) ထဲရှိ installation instruction ကို လိုက်နာပါ။ Published [v2.2.2 release](https://github.com/hidecard/zap/releases/tag/v2.2.2)၊ [v2.2.1 release](https://github.com/hidecard/zap/releases/tag/v2.2.1) နှင့် historical [v2.2.0 release](https://github.com/hidecard/zap/releases/tag/v2.2.0) များကို ပြင်ဆင်ခြင်းမရှိဘဲ ဆက်လက်ရရှိနိုင်သည်။

## References

[1]: https://github.com/hidecard/zap/releases/tag/v2.2.0 "Zap v2.2.0 release"
[2]: https://github.com/hidecard/zap/releases/tag/v2.2.1 "Zap v2.2.1 release"
[3]: https://github.com/hidecard/zap/releases/tag/v2.2.2 "Zap v2.2.2 release"
[4]: https://github.com/hidecard/zap/commit/ed1cb46 "Harden runtime borrows and cyclic equality"
[5]: https://github.com/hidecard/zap/commit/3e58e10 "Harden LSP rename scope stack"
[6]: https://github.com/hidecard/zap/commit/349f68a "Synchronize post-v2.2.2 documentation"

