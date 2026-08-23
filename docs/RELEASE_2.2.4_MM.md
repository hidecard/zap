# Zap v2.2.4 Release Notes

Zap v2.2.4 သည် v2.2.3 နောက်ပိုင်း documentation-baseline maintenance release ဖြစ်သည်။ Fresh audit တွင် တွေ့ရှိသော active language-specification metadata နှင့် generic type-check release-gate reference များကို ပြင်ဆင်ပြီး current README နှင့် release surface များကို synchronize လုပ်ထားပါသည်။ Parser၊ runtime သို့မဟုတ် generic-syntax behavior ပြောင်းလဲမှု မရှိပါ။

## Release provenance

Zap v2.2.4 ကို published v2.2.3 release နောက်ပိုင်း `master` history မှ တည်ဆောက်ထားသည်။ v2.2.0 tag နှင့် release သည် [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb) commit တွင် immutable ဖြစ်ပြီး v2.2.1 သည် [`201fad4`](https://github.com/hidecard/zap/commit/201fad4c7fbee38e3fabf63bf17d50eb4d70f784) တွင် immutable ဖြစ်သည်။ v2.2.2 သည် [`f932e21`](https://github.com/hidecard/zap/commit/f932e21d190f59e722bf17dfdc214cef75ade698) တွင် immutable ဖြစ်ပြီး v2.2.3 သည် [`758d9fa`](https://github.com/hidecard/zap/commit/758d9faf04154721788016937b0963bd9d0872a8) တွင် immutable ဖြစ်သည်။ အဆိုပါ release များ၏ tag၊ release၊ signed asset၊ checksum၊ provenance နှင့် release note များကို rewrite မလုပ်ထားပါ။

## Active specification baseline

English နှင့် Burmese language specification များတွင် v2.2.4 ကို လက်ရှိ normative documentation baseline အဖြစ် သတ်မှတ်ထားပါသည်။ အောက်ခံ syntax၊ typing၊ runtime၊ diagnostics၊ compatibility နှင့် version contract များ မပြောင်းလဲပါ။ Post-v2.2.3 audit တွင် တွေ့ရှိသော stale active metadata ကိုသာ ပြင်ဆင်ထားပါသည်။

## Generic type-check gate

Bilingual generic type-check decision record များတွင် အကောင်အထည်ဖော်ပြီးသား TC-012 baseline အတွက် v2.2.4 release gate ကို သတ်မှတ်ထားပါသည်။ Supported collection နှင့် variant annotation များ မပြောင်းလဲပါ။ User-defined generic declaration၊ advanced inference နှင့် ပိုမိုကျယ်ပြန့်သော generic parser syntax များကို explicit deferred အဖြစ် ဆက်လက်ထားရှိပြီး ဤ release တွင် မထည့်သွင်းပါ။

## README and release-surface synchronization

English နှင့် Burmese README များတွင် v2.2.4 ကို လက်ရှိ release အဖြစ် ဖော်ပြထားပြီး installation link များကို v2.2.4 release သို့ ညွှန်ပြထားပါသည်။ v2.2.4 Linux၊ macOS ARM64 နှင့် Windows archive များကိုလည်း စာရင်းပြုထားပါသည်။ `SECURITY.md`၊ type-check conformance matrix နှစ်ခုနှင့် checked-in VS Code package manifest နှစ်ခုကို တူညီသော active release line သို့ synchronize လုပ်ထားပါသည်။ v2.2.0 မှ v2.2.3 အထိ historical reference များသည် current-installation target မဟုတ်ဘဲ သမိုင်းဆိုင်ရာ provenance အဖြစ်သာ ဆက်ရှိပါသည်။

## Compatibility and language boundary

ဤ release သည် ရှိပြီးသား Zap language နှင့် package contract များကို ထိန်းသိမ်းထားသည်။ Traits၊ interfaces၊ composition၊ parser syntax၊ runtime syntax၊ public weak reference၊ automatic cycle collection သို့မဟုတ် tracing collector များကို မထည့်သွင်းပါ။ Runtime သည် single-threaded `Rc`/`RefCell` infrastructure အဖြစ် ဆက်ရှိပြီး async သည် lazy continuation မဟုတ်ဘဲ eager scheduled-value behavior ကို ထိန်းသိမ်းထားသည်။ LSP သည် rejected range edit နှင့် file-local rename only ပါသော full-text synchronization အဖြစ် ဆက်ရှိသည်။ Production reactor သို့မဟုတ် multi-thread runtime မဟုတ်ပါ။

## Verification and reproducibility

Documentation correction များပြီးနောက် audited v2.2.3 baseline သည် complete release preflight ကို အောင်မြင်ခဲ့ပါသည်။ Strict formatting နှင့် Clippy၊ serial native unit/integration suite၊ bilingual documentation consistency၊ specification ownership validation၊ standard-library policy၊ LSP protocol နှင့် semantic-parity check၊ VS Code package validation၊ parity/replay/async matrix၊ benchmark၊ packaging နှင့် archive manifest၊ signing နှင့် registry policy harness၊ release-version validation နှင့် `git diff --check` အားလုံး ပါဝင်ပါသည်။ v2.2.4 candidate သည်လည်း အဆိုပါ gate များကို အောင်မြင်ရမည်။ `Cargo.lock` ကို regenerate/update မလုပ်ဘဲ `zap-native` package version stanza ကိုသာ 2.2.3 မှ 2.2.4 သို့ manually synchronize လုပ်ထားပါသည်။

## Historical release preservation

Post-v2.2.3 audit တွင် Rust test module များမတိုင်မီ production code အတွင်း panic ဖြစ်စေနိုင်သော call များ မတွေ့ရှိပါ။ Stale active documentation reference များမှအပ actionable runtime သို့မဟုတ် tooling defect မတွေ့ရှိရဘဲ ၎င်းတို့ကို maintenance commit [`5cf2682`](https://github.com/hidecard/zap/commit/5cf2682dd14e62f13a0edba6df9718d76e83459e) တွင် ပြင်ဆင်ထားပါသည်။ Roadmap ထဲရှိ deferred item များသည် architecture သို့မဟုတ် governance scope အဖြစ် ဆက်လက် deferred ဖြစ်ပြီး implementation ပြီးစီးသည်ဟု မဖော်ပြပါ။

## Upgrade guidance

Target platform နှင့် ကိုက်ညီသော archive ကို [v2.2.4 GitHub release](https://github.com/hidecard/zap/releases/tag/v2.2.4) မှ download လုပ်ပြီး published checksum နှင့် signature/provenance information ကို verify လုပ်ပါ။ ထို့နောက် [English README](../README.md) သို့မဟုတ် [Burmese README](../README_MM.md) ထဲရှိ installation instruction ကို လိုက်နာပါ။ Published [v2.2.3 release](https://github.com/hidecard/zap/releases/tag/v2.2.3)၊ [v2.2.2 release](https://github.com/hidecard/zap/releases/tag/v2.2.2)၊ [v2.2.1 release](https://github.com/hidecard/zap/releases/tag/v2.2.1) နှင့် historical [v2.2.0 release](https://github.com/hidecard/zap/releases/tag/v2.2.0) များကို ပြင်ဆင်ခြင်းမရှိဘဲ ဆက်လက်ရရှိနိုင်သည်။

## References

[1]: https://github.com/hidecard/zap/releases/tag/v2.2.0 "Zap v2.2.0 release"
[2]: https://github.com/hidecard/zap/releases/tag/v2.2.1 "Zap v2.2.1 release"
[3]: https://github.com/hidecard/zap/releases/tag/v2.2.2 "Zap v2.2.2 release"
[4]: https://github.com/hidecard/zap/releases/tag/v2.2.3 "Zap v2.2.3 release"
[5]: https://github.com/hidecard/zap/commit/5cf2682 "Fix stale active documentation baselines"
