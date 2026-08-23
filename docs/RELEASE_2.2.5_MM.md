# Zap v2.2.5 Release Notes

Zap v2.2.5 သည် v2.2.4 နောက်ပိုင်း non-framework reliability maintenance release ဖြစ်သည်။ Production HTTP request path အတွင်းရှိ internal URL-parser invariant `unreachable!` branch များကို deterministic error များဖြင့် အစားထိုးပြီး လက်ရှိ English/Burmese release surface များကို update လုပ်ထားပါသည်။ Parser၊ runtime၊ language syntax သို့မဟုတ် framework behavior အသစ် မထည့်သွင်းပါ။

## Release provenance

Zap v2.2.5 ကို published v2.2.4 release နောက်ပိုင်း `master` history မှ တည်ဆောက်ထားသည်။ v2.2.0 tag နှင့် release သည် [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb) commit တွင် immutable ဖြစ်ပြီး v2.2.1 သည် [`201fad4`](https://github.com/hidecard/zap/commit/201fad4c7fbee38e3fabf63bf17d50eb4d70f784) တွင် immutable ဖြစ်သည်။ v2.2.2 သည် [`f932e21`](https://github.com/hidecard/zap/commit/f932e21d190f59e722bf17dfdc214cef75ade698) တွင် immutable ဖြစ်ပြီး v2.2.3 သည် [`758d9fa`](https://github.com/hidecard/zap/commit/758d9faf04154721788016937b0963bd9d0872a8) တွင် immutable ဖြစ်သည်။ v2.2.4 သည် [`00d2847`](https://github.com/hidecard/zap/commit/00d2847eaf149821c88f1ed060085972eca993b2) တွင် immutable ဖြစ်သည်။ အဆိုပါ release များ၏ tag၊ release၊ signed asset၊ checksum၊ provenance နှင့် release note များကို rewrite မလုပ်ထားပါ။

## Active runtime reliability baseline

HTTP request implementation သည် ရှိပြီးသား bounded `parse_url` helper မှ absolute URL များကို parse လုပ်ပြီး ရှိပြီးသား capability နှင့် network-destination check များအောက်တွင် `http` နှင့် `https` request များကိုသာ support လုပ်ဆောင်သည်။ Internal parser-result shape မကိုက်ညီမှု ဖြစ်ပေါ်ပါက `http_request` သည် `unreachable!` ဖြင့် process ရပ်တန့်ခြင်းမပြုတော့ဘဲ invalid result သို့မဟုတ် scheme/host မရှိမှုအတွက် deterministic error ပြန်ပေးသည်။ Valid request behavior၊ URL limit၊ response limit၊ timeout behavior နှင့် capability enforcement များ မပြောင်းလဲပါ။

## HTTP URL invariant handling

Production `http_request` path တွင် internal shape assumption သုံးမျိုးကို explicit စစ်ဆေးထားပါသည်။ Parser result သည် map ဖြစ်ရမည်၊ map ထဲတွင် text scheme ပါရမည်၊ map ထဲတွင် text host ပါရမည်။ ၎င်းတို့သည် public syntax အသစ် သို့မဟုတ် URL contract အသစ် မဟုတ်ဘဲ defensive invariant boundary များသာ ဖြစ်သည်။ User ထည့်သွင်းသော malformed URL များသည် ရှိပြီးသား typed parse error ကို ဆက်လက်ရရှိမည်ဖြစ်ပြီး မမျှော်လင့်ထားသော internal shape များသည် ordinary runtime error အဖြစ် fail closed ဖြစ်မည်။

## README and release-surface synchronization

English နှင့် Burmese README များတွင် v2.2.5 ကို လက်ရှိ release အဖြစ် ဖော်ပြထားပြီး installation link များကို v2.2.5 release သို့ ညွှန်ပြထားပါသည်။ v2.2.5 Linux၊ macOS ARM64 နှင့် Windows archive များကိုလည်း စာရင်းပြုထားပါသည်။ `SECURITY.md`၊ type-check conformance matrix နှစ်ခုနှင့် checked-in VS Code package manifest နှစ်ခုကို တူညီသော active release line သို့ synchronize လုပ်ထားပါသည်။ v2.2.0 မှ v2.2.4 အထိ historical reference များသည် current-installation target မဟုတ်ဘဲ သမိုင်းဆိုင်ရာ provenance အဖြစ်သာ ဆက်ရှိပါသည်။

## Compatibility and framework boundary

ဤ release သည် ရှိပြီးသား Zap language နှင့် package contract များကို ထိန်းသိမ်းထားသည်။ Traits၊ interfaces၊ composition၊ parser syntax၊ runtime syntax၊ public weak reference၊ automatic cycle collection၊ tracing collector သို့မဟုတ် Web/App/IoT framework များကို မထည့်သွင်းပါ။ Framework planning သည် လက်ရှိ core maintenance အလုပ်များပြီးဆုံးပြီးနောက် သီးခြား branch တွင်သာ ဆက်လက်စဉ်းစားမည်ဖြစ်ပြီး v2.2.5 တွင် framework branch သို့မဟုတ် framework implementation မပါဝင်ပါ။ Runtime သည် single-threaded `Rc`/`RefCell` infrastructure အဖြစ် ဆက်ရှိပြီး async သည် lazy continuation မဟုတ်ဘဲ eager scheduled-value behavior ကို ထိန်းသိမ်းထားသည်။ LSP သည် rejected range edit နှင့် file-local rename only ပါသော full-text synchronization အဖြစ် ဆက်ရှိသည်။ Production reactor သို့မဟုတ် multi-thread runtime မဟုတ်ပါ။

## Verification and reproducibility

Focused HTTP-hardening validation တွင် strict formatting၊ warnings denied ဖြင့် Clippy နှင့် serial native suite ကို အောင်မြင်ခဲ့ပါသည်။ Native suite တွင် unit tests ၂၃၂ ခုနှင့် core integration tests ၂၅၆ ခု ပါဝင်သည်။ v2.2.5 release candidate သည် bilingual documentation consistency၊ specification ownership၊ standard-library policy၊ LSP protocol နှင့် semantic parity၊ VS Code package validation၊ parity/replay/async matrix၊ benchmark၊ packaging၊ signing-policy၊ registry-policy၊ deployment-policy၊ release-version validation နှင့် `git diff --check` အပါအဝင် complete release preflight ကို အောင်မြင်ရမည်။ `Cargo.lock` ကို regenerate/update မလုပ်ဘဲ `zap-native` package version stanza ကိုသာ 2.2.4 မှ 2.2.5 သို့ manually synchronize လုပ်ထားပါသည်။

## Historical release preservation

Fresh no-framework audit တွင် actionable TODO/FIXME marker မကျန်ရှိတော့ကြောင်း၊ framework implementation အသစ်မရှိကြောင်းနှင့် HTTP invariant hardening ပြီးနောက် production `unreachable!` သို့မဟုတ် `todo!` path မကျန်ရှိတော့ကြောင်း စစ်ဆေးတွေ့ရှိရသည်။ Roadmap ထဲရှိ ကျန် item များသည် architecture သို့မဟုတ် governance scope အဖြစ် ရည်ရွယ်ချက်ရှိရှိ deferred ထားခြင်းဖြစ်ပြီး implementation ပြီးစီးသည်ဟု မဖော်ပြပါ။ Focused reliability correction ကို [`f4470ab`](https://github.com/hidecard/zap/commit/f4470abdcc314311cf759fa023bf497b1bdd2a94) commit တွင် မှတ်တမ်းတင်ထားပါသည်။

## Upgrade guidance

Target platform နှင့် ကိုက်ညီသော archive ကို [v2.2.5 GitHub release](https://github.com/hidecard/zap/releases/tag/v2.2.5) မှ download လုပ်ပြီး published checksum နှင့် signature/provenance information ကို verify လုပ်ပါ။ ထို့နောက် [English README](../README.md) သို့မဟုတ် [Burmese README](../README_MM.md) ထဲရှိ installation instruction ကို လိုက်နာပါ။ Published [v2.2.4 release](https://github.com/hidecard/zap/releases/tag/v2.2.4)၊ [v2.2.3 release](https://github.com/hidecard/zap/releases/tag/v2.2.3)၊ [v2.2.2 release](https://github.com/hidecard/zap/releases/tag/v2.2.2)၊ [v2.2.1 release](https://github.com/hidecard/zap/releases/tag/v2.2.1) နှင့် historical [v2.2.0 release](https://github.com/hidecard/zap/releases/tag/v2.2.0) များကို ပြင်ဆင်ခြင်းမရှိဘဲ ဆက်လက်ရရှိနိုင်သည်။

## References

[1]: https://github.com/hidecard/zap/releases/tag/v2.2.0 "Zap v2.2.0 release"
[2]: https://github.com/hidecard/zap/releases/tag/v2.2.1 "Zap v2.2.1 release"
[3]: https://github.com/hidecard/zap/releases/tag/v2.2.2 "Zap v2.2.2 release"
[4]: https://github.com/hidecard/zap/releases/tag/v2.2.3 "Zap v2.2.3 release"
[5]: https://github.com/hidecard/zap/releases/tag/v2.2.4 "Zap v2.2.4 release"
[6]: https://github.com/hidecard/zap/commit/f4470ab "Harden HTTP URL invariants"
