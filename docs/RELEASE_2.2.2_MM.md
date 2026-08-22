# Zap v2.2.2 Release Notes

Zap v2.2.2 သည် published v2.2.1 release နောက်ပိုင်း corrective maintenance release ဖြစ်သည်။ `master` တွင် ပြီးစီးခဲ့သော core-runtime safety အလုပ်၊ canonical AST compatibility ပြင်ဆင်မှု၊ standard-library catalog synchronization၊ editor grammar parity နှင့် bilingual documentation update များကို ဤ release တွင် package လုပ်ထားသည်။

## Release provenance

Zap v2.2.2 ကို post-v2.2.1 `master` history မှ တည်ဆောက်ထားသည်။ Historical v2.2.0 tag နှင့် release သည် [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb) commit တွင် signed asset၊ checksum၊ provenance နှင့် release note များနှင့်အတူ immutable အဖြစ် ဆက်ရှိသည်။ Published v2.2.1 tag နှင့် release ကိုလည်း မပြောင်းလဲထားပါ။ Historical tag သို့မဟုတ် asset တစ်ခုမျှ rewrite မလုပ်ထားပါ။

## Runtime borrow နှင့် cycle safety

Canonical AST execution သည် active lexical-frame boundary များတွင် checked `EnvFrame` operation များကို အသုံးပြုသည်။ Frame တစ်ခု borrow ဖြစ်နေချိန်တွင် unchecked `RefCell` path မှ panic ဖြစ်မည့်အစား stable `BorrowError` result ကို ပြန်ပေးသည်။ Strong `Rc` object နှင့် capture cycle များကို ဆက်လက်ထောက်ပံ့သော်လည်း cleanup တာဝန်သည် `clear_object_fields()` ဖြင့် explicit ဖြစ်သည်။ `memory_stats()` တွင် `cycle_policy=explicit_clear_object_fields` ကို ဖော်ပြထားသည်။ Public weak-reference API သို့မဟုတ် automatic tracing collector မထည့်သွင်းပါ။

## Canonical helper compatibility

Canonical AST dispatcher တွင် ယခင်က documentation ပြုထားသော်လည်း native execution path တွင် ပျောက်နေသော `assert`၊ `sort` နှင့် `sqrt` helper များကို ပြန်လည်ထည့်သွင်းထားသည်။ `assert` သည် expected/observed failure ကို deterministic အတိုင်း ပြသည်။ `sort` သည် number-only သို့မဟုတ် text-only list ကို ascending အတိုင်း clone လုပ်ပြီး ပြန်ပေးသည်။ `sqrt` သည် non-negative integer ကို လက်ခံပြီး rounded integer square root ကို ပြန်ပေးသည်။ Standard-library catalog နှင့် bilingual policy/index metadata များသည် editor surface ၏ builtin ၇၆ ခုလုံးကို ယခု cover လုပ်ထားသည်။

## VS Code grammar နှင့် catalog parity

Canonical `vscode-extension/` package နှင့် `editors/vscode/` mirror နှစ်ခုလုံးသည် `assert`၊ `sort` နှင့် `sqrt` အပါအဝင် public builtin catalog အပြည့်ကို highlight လုပ်သည်။ Package validation သည် metadata၊ grammar parity၊ catalog coverage၊ archive integrity နှင့် generated/VCS entry မပါဝင်မှုတို့ကို အတည်ပြုသည်။ Extension manifest ကို 2.2.2 အဖြစ် version လုပ်ထားသည်။

## Documentation နှင့် traceability

English/Burmese README၊ release၊ standard-library၊ typecheck၊ runtime၊ memory၊ roadmap၊ policy နှင့် security surface များတွင် v2.2.2 ကို လက်ရှိ verified release အဖြစ် ဖော်ပြထားသည်။ Documentation တွင် v2.2.0 နှင့် v2.2.1 provenance ကို explicit ထားရှိပြီး cycle policy သည် automatic collection မဟုတ်ဘဲ explicit cleanup ဖြစ်ကြောင်း ဖော်ပြထားသည်။ Parser၊ runtime သို့မဟုတ် traits syntax အသစ် မထည့်သွင်းပါ။

## Compatibility နှင့် known limitations

ဤ release သည် canonical execution path ကို ပြင်ဆင်ပြီး checked borrow boundary ကို ခိုင်မာစေသော်လည်း ရှိပြီးသား Zap language နှင့် package contract များကို ထိန်းသိမ်းထားသည်။ Async function များသည် document လုပ်ထားသော eager scheduled-value behavior အတိုင်း ဆက်ရှိပြီး lazy မဟုတ်ပါ။ LSP synchronization သည် full-text only `change: 1` ဖြစ်ပြီး rename သည် file-local ဖြစ်သည်။ Runtime သည် single-threaded `Rc`/`RefCell` infrastructure အဖြစ် ဆက်ရှိပြီး traits/composition သည် design-only RFC နှင့် unsupported syntax အဖြစ် ဆက်ရှိသည်။ Public weak reference၊ automatic cycle collection နှင့် tracing collection များသည် unsupported သို့မဟုတ် deferred ဖြစ်နေဆဲ ဖြစ်သည်။

## Verification နှင့် reproducibility

Release candidate သည် complete serial native suite ဖြစ်သော unit test ၂၂၉ ခုနှင့် integration test ၂၅၆ ခု၊ warnings denied strict Clippy၊ formatting၊ LSP protocol synchronization၊ LSP semantic parity၊ canonical VS Code package contract၊ standard-library policy၊ parity/replay/async matrix၊ benchmark၊ packaging၊ documentation consistency နှင့် regression harness၊ specification ownership နှင့် `git diff --check` များကို အောင်မြင်ထားသည်။ Grammar-synchronized state အတွက် GitHub Actions run [`32584437606`](https://github.com/hidecard/zap/actions/runs/32584437606) သည် success ဖြင့် ပြီးစီးထားသည်။ Cargo.lock ကို regenerate/update မလုပ်ဘဲ `zap-native` package version stanza ကိုသာ 2.2.1 မှ 2.2.2 သို့ patch လုပ်ထားသည်။

## Upgrade guidance

Target platform နှင့် ကိုက်ညီသော archive ကို [v2.2.2 GitHub release](https://github.com/hidecard/zap/releases/tag/v2.2.2) မှ download လုပ်ပြီး published checksum နှင့် signature/provenance information ကို verify လုပ်ပါ။ ထို့နောက် [English README](../README.md) သို့မဟုတ် [Burmese README](../README_MM.md) ထဲရှိ installation instruction ကို လိုက်နာပါ။ Published [v2.2.1 release](https://github.com/hidecard/zap/releases/tag/v2.2.1) နှင့် historical [v2.2.0 release](https://github.com/hidecard/zap/releases/tag/v2.2.0) များကို ပြင်ဆင်ခြင်းမရှိဘဲ ဆက်လက်ရရှိနိုင်သည်။

## References

[1]: https://github.com/hidecard/zap/releases/tag/v2.2.0 "Zap v2.2.0 release"
[2]: https://github.com/hidecard/zap/releases/tag/v2.2.1 "Zap v2.2.1 release"
[3]: https://github.com/hidecard/zap/commit/73a1fb5840af4e36789f9572078b0215282291ea "Checked EnvFrame borrows and explicit cycle policy"
[4]: https://github.com/hidecard/zap/commit/4db20741a34100c99cacca1811eea551b2040ce5 "Builtin grammar catalog synchronization"
