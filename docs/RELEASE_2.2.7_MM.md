# Zap v2.2.7 Release Notes

Zap v2.2.7 သည် Framework dependency-security နှင့် runtime-quality maintenance release ဖြစ်သည်။ Framework branch တွင် ကျန်ရှိနေသော RustSec advisory ၆ ခုကို ဖြေရှင်းခြင်း၊ Framework native dependency graph ကို v2.2.6 security-clean baseline နှင့် align လုပ်ခြင်း၊ Rust 1.88.0 toolchain pin လုပ်ခြင်း၊ native/host RustSec CI coverage ထည့်ခြင်းနှင့် rcgen 0.13 TLS test compatibility ပြင်ခြင်းများ ပါဝင်သည်။ Document လုပ်ထားသော Zap-native Framework boundary ကို ထိန်းသိမ်းထားပြီး production integration များ ရှိပြီးသားဟု မဆိုပါ။

## Release provenance

v2.2.7 release candidate ကို published v2.2.6 release နောက်ပိုင်း Framework remediation history မှ ပြင်ဆင်ထားပြီး v2.2.6 သည် immutable ဖြစ်သည်။ v2.2.0 tag နှင့် release သည် [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb) commit တွင် immutable ဖြစ်ပြီး v2.2.1 သည် [`201fad4`](https://github.com/hidecard/zap/commit/201fad4c7fbee38e3fabf63bf17d50eb4d70f784) တွင် immutable ဖြစ်သည်။ v2.2.2 သည် [`f932e21`](https://github.com/hidecard/zap/commit/f932e21d190f59e722bf17dfdc214cef75ade698) တွင် immutable ဖြစ်ပြီး v2.2.3 သည် [`758d9fa`](https://github.com/hidecard/zap/commit/758d9faf04154721788016937b0963bd9d0872a8) တွင် immutable ဖြစ်သည်။ v2.2.4 သည် [`00d2847`](https://github.com/hidecard/zap/commit/00d2847eaf149821c88f1ed060085972eca993b2) တွင် immutable ဖြစ်ပြီး v2.2.5 သည် [`e5f3ea7`](https://github.com/hidecard/zap/commit/e5f3ea7195d4b8bb1e3c38c4618be834bf50c558) commit တွင် immutable ဖြစ်သည်။ အဆိုပါ tag၊ release၊ signed asset၊ checksum၊ provenance နှင့် release note များကို rewrite မလုပ်ထားပါ။

## Active runtime reliability baseline

Canonical AST path နှင့် compatibility-only legacy path နှစ်ခုစလုံးတွင် filesystem line builtin များသည် text I/O နှင့် တူညီသော context-aware workspace confinement ကို အသုံးပြုပါသည်။ Traversal၊ workspace အပြင်ရှိ absolute path နှင့် active workspace အပြင်သို့ ရောက်သွားသော symlink-resolved path များကို reject လုပ်ပြီး ရှိပြီးသား file-size နှင့် capability limit များကို ထိန်းသိမ်းထားပါသည်။ Synchronous နှင့် async process timeout/cancellation များတွင် Unix တွင် သီးခြား process group ဖွဲ့ပြီး negative group identifier အတွက် လိုအပ်သော `kill -KILL -- -PID` form ကို အသုံးပြုပါသည်။ Windows တွင် platform process utility ဖြင့် recursive tree termination ကို တောင်းဆိုကာ direct-child cleanup နှင့် wait ကို ဆက်လက်လုပ်ဆောင်ပါသည်။ Focused cancellation နှင့် direct process-group regression များဖြင့် ဤ boundary ကို စစ်ဆေးထားပါသည်။

## Bounded operations and URL handling

Canonical နှင့် legacy execution နှစ်ခုစလုံးတွင် `sleep` နှင့် `pow` သည် explicit bounded policy တစ်ခုတည်းကို အသုံးပြုပါသည်။ Sleep သည် negative သို့မဟုတ် limit ကျော်သော duration ကို reject လုပ်ပြီး exponentiation သည် checked exponentiation-by-squaring နှင့် stable overflow/limit diagnostic များကို အသုံးပြုကာ unbounded repeated multiplication မလုပ်တော့ပါ။ URL parsing သည် malformed၊ empty နှင့် out-of-range authority port များကို reject လုပ်ပြီး valid host-only နှင့် bracketed IPv6 form များကို ထိန်းသိမ်းထားပါသည်။ ၎င်းတို့သည် defensive limit နှင့် correctness fix များသာဖြစ်ပြီး public syntax အသစ် မထည့်သွင်းပါ။

## CLI, project, and test-harness correctness

`build --locked` သည် strict project-validation path မှတစ်ဆင့် valid existing lockfile တစ်ခုကို မဖြစ်မနေလိုအပ်စေပြီး ordinary non-locked build/check/install behavior များကို မပြောင်းလဲပါ။ Test discovery သည် real directory များကို တစ်ကြိမ်သာ canonicalize လုပ်ကာ symlink directory entry များကို skip လုပ်ပြီး deterministic ordering ကို ထိန်းသိမ်းကာ symlink loop များကို ရှောင်ရှားပါသည်။ Registry security-test fixture များသည် shared environment guard တစ်ခုကို အသုံးပြုကြပြီး Windows legacy path regression သည် Zap string-literal rule နှင့်ကိုက်ညီသော backslash encoding ကို အသုံးပြုပါသည်။ Standard-library catalog နှင့် mirrored VS Code grammar နှစ်ခုသည် `sleep` အပါအဝင် တူညီသော builtin set ကို cover လုပ်ပါသည်။

## README and release-surface synchronization

English နှင့် Burmese README၊ navigation hub၊ syntax နှင့် language reference၊ runtime နှင့် memory record၊ standard-library policy၊ type-check matrix၊ learner guide၊ TODO register၊ security metadata၊ changelog၊ VS Code manifest နှင့် release-note link များကို v2.2.7 active line သို့ synchronize လုပ်ထားပါသည်။ v2.2.0 မှ v2.2.5 အထိ historical reference များသည် current-installation target မဟုတ်ဘဲ historical provenance အဖြစ်သာ ဆက်ရှိပါသည်။ အသစ်ရေးသားသော release document တွင် author attribution အသစ် မထည့်သွင်းပါ။

## Compatibility and framework boundary

ဤ release သည် ရှိပြီးသား Zap language နှင့် package contract များကို ထိန်းသိမ်းထားသည်။ Traits၊ interfaces၊ composition၊ generic declaration syntax၊ broad async syntax၊ public weak reference၊ automatic cycle collection၊ tracing collector၊ production reactor၊ multi-thread language runtime၊ ranged LSP change၊ cross-file rename သို့မဟုတ် Web/App/IoT host/adapter များကို မထည့်သွင်းပါ။ Runtime သည် single-threaded `Rc`/`RefCell` infrastructure အဖြစ် ဆက်ရှိပြီး async သည် eager scheduled-value behavior ကို ထိန်းသိမ်းထားသည်။ LSP သည် rejected range edit နှင့် file-local rename only ပါသော full-text synchronization အဖြစ် ဆက်ရှိသည်။ Framework dependency remediation ကို ဤ release တွင် ထည့်သွင်းထားပြီး production framework integration များကို သီးခြား gate ချထားဆဲဖြစ်သည်။

## Verification and dependency advisory status

v2.2.7 release သည် strict formatting၊ warnings-denied Clippy၊ locked compilation၊ native unit/integration suite အပြည့်အစုံ၊ filesystem/process/network/security corpus၊ project/lockfile test၊ LSP နှင့် VS Code parity၊ documentation consistency၊ standard-library policy၊ ownership/parity/replay/async matrix၊ packaging နှင့် clean tagged-name release preflight များကို အောင်မြင်ခဲ့ပါသည်။ Release preflight နှင့် tag-triggered release workflow နှစ်ခုလုံးသည် `scripts/check_rustsec_audit.sh` နှင့် `RUN_CARGO_AUDIT=1` မှတစ်ဆင့် modern RustSec audit ကို enforce လုပ်ထားပြီး advisory များကို ignore မလုပ်သလို lockfile ကိုလည်း မပြောင်းလဲပါ။ ထုတ်ဝေထားသော locked graph တွင် `ureq 2.12.1`၊ `url 2.5.8`၊ `idna 1.1.0`၊ `rustls 0.23.40`၊ `rustls-webpki 0.103.15`၊ `rcgen 0.13.2` နှင့် development-only `time 0.3.47` တို့ ပါဝင်ပါသည်။ Strict `cargo-audit 0.22.2` scan သည် native locked crate dependency ၉၉ ခုနှင့် host-adapter locked crate dependency ၇၀ ခုကို စစ်ဆေးပြီး graph နှစ်ခုစလုံးတွင် unresolved advisory သုညခုကို report လုပ်ပါသည်။ `time 0.3.47` သည် Rust 1.88.0 လိုအပ်သောကြောင့် candidate source နှင့် CI quality job တွင် Rust 1.88.0 ကို အသုံးပြုထားပါသည်။ ၎င်းသည် build toolchain ပြောင်းလဲမှုသာဖြစ်ပြီး Zap language contract ကို မပြောင်းလဲပါ။ Local release-candidate verification တွင် native tests ၂၄၈ ခုနှင့် corpus case ၂၅၈ ခု၊ host tests ၂ ခုနှင့် case ၈ ခု၊ native/host format နှင့် Clippy၊ native release build၊ Framework starter checks ၁၄၇ ခုနှင့် documentation consistency အားလုံး pass ဖြစ်ပါသည်။ GitHub publication၊ checksum၊ signature နှင့် provenance များသည် PR merge ပြီး v2.2.7 tag workflow ပြီးဆုံးသည်အထိ pending ဖြစ်သည်။

## Historical release preservation

v2.2.7 maintenance အလုပ်များသည် ယခင် release tag နှင့် published asset အားလုံးကို ထိန်းသိမ်းထားပါသည်။ Source change များသည် Framework dependency remediation၊ host-adapter lint correctness၊ release/CI gate synchronization၊ TLS test compatibility နှင့် bilingual documentation အတွင်းသာ ကန့်သတ်ထားပြီး ရှိပြီးသား language contract ကို မပြောင်းလဲပါ။ Future AST/typed-IR checker redesign၊ syntax-aware formatter/linter expansion၊ race-resistant descriptor-relative filesystem API၊ complete DNS-to-connection pinning နှင့် universal descendant cleanup များသည် preserved language contract မပြောင်းလဲဘဲ သီးခြား implement/verify မလုပ်မချင်း explicit follow-up boundary အဖြစ် ဆက်ရှိပါသည်။

## Upgrade guidance

Publication ပြီးနောက် target platform နှင့် ကိုက်ညီသော archive ကို [v2.2.7 GitHub release](https://github.com/hidecard/zap/releases/tag/v2.2.7) မှ download လုပ်ပြီး published checksum နှင့် signature/provenance information ကို verify လုပ်ပါ။ ထို့နောက် [English README](../README.md) သို့မဟုတ် [Burmese README](../README_MM.md) ထဲရှိ installation instruction ကို လိုက်နာပါ။ Published [v2.2.5 release](https://github.com/hidecard/zap/releases/tag/v2.2.5)၊ [v2.2.4 release](https://github.com/hidecard/zap/releases/tag/v2.2.4)၊ [v2.2.3 release](https://github.com/hidecard/zap/releases/tag/v2.2.3)၊ [v2.2.2 release](https://github.com/hidecard/zap/releases/tag/v2.2.2)၊ [v2.2.1 release](https://github.com/hidecard/zap/releases/tag/v2.2.1) နှင့် historical [v2.2.0 release](https://github.com/hidecard/zap/releases/tag/v2.2.0) များကို ပြင်ဆင်ခြင်းမရှိဘဲ ဆက်လက်ရရှိနိုင်သည်။

## References

[1]: https://github.com/hidecard/zap/releases/tag/v2.2.0 "Zap v2.2.0 release"
[2]: https://github.com/hidecard/zap/releases/tag/v2.2.1 "Zap v2.2.1 release"
[3]: https://github.com/hidecard/zap/releases/tag/v2.2.2 "Zap v2.2.2 release"
[4]: https://github.com/hidecard/zap/releases/tag/v2.2.3 "Zap v2.2.3 release"
[5]: https://github.com/hidecard/zap/releases/tag/v2.2.4 "Zap v2.2.4 release"
[6]: https://github.com/hidecard/zap/releases/tag/v2.2.5 "Zap v2.2.5 release"
[7]: https://github.com/hidecard/zap/commit/cf614e2 "Fix Windows legacy path fixture"
[8]: https://github.com/hidecard/zap/commit/0b0e276 "Core hardening maintenance"
[9]: https://github.com/hidecard/zap/commit/d5c2cde "Align grammar with cataloged sleep builtin"
