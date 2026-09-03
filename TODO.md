# Zap Remaining TODO

**စစ်ဆေးသည့်နေ့:** 2026-09-03
**Repository:** [hidecard/zap](https://github.com/hidecard/zap)
**Latest published release:** [v2.11.18](https://github.com/hidecard/zap/releases/tag/v2.11.18)
**Current branch:** `master`
**Bootstrap stage:** B0

> ဤ TODO သည် repository အတွင်းရှိ current-status၊ next-plan နှင့် milestone စာတမ်းများကို အခြေခံထားသော လုပ်ငန်းစာရင်းဖြစ်သည်။ B1/B2/B3/B4 bootstrap evidence များသည် လက်ရှိတွင် corpus-limited/provisional ဖြစ်ပြီး Rust native implementation ကို အစားထိုးပြီးသားဟု မယူဆရပါ။

## Release အခြေအနေ

| အချက် | အခြေအနေ | မှတ်ချက် |
|---|---|---|
| Published releases | ပြီးစီး | လက်ရှိ public latest release သည် `v2.11.18` ဖြစ်သည်။ |
| Tags | ပြီးစီး | `v2.11.18` အထိ version tags ရှိသည်။ ယခင် tags များကို rewrite မလုပ်ရ။ |
| Release provenance | ပြီးစီး/ထိန်းသိမ်းရန် | Versioned manifest၊ checksum၊ detached signature နှင့် signed provenance ကို release တစ်ခုစီတွင် ဆက်လက်ထိန်းသိမ်းရန်။ |
| Next release | မစတင်ရသေး | Tag အသစ်၊ clean commit နှင့် release preflight အောင်မြင်ပြီးမှသာ ဆက်လုပ်ရန်။ |

## လက်ကျန်အဓိကလုပ်ငန်းများ

### P0 — Release နှင့် quality gates

- [ ] နောက် release အတွက် exact version consistency ကို `Cargo.toml`၊ lockfile၊ CLI output၊ README၊ changelog နှင့် release metadata အားလုံးတွင် စစ်ဆေးရန်။
- [ ] Clean checkout မှ release preflight၊ source validation၊ Linux/macOS/Windows native jobs နှင့် artifact verification ကို ပြန်လည်အောင်မြင်စွာ run ရန်။
- [ ] Published artifact များ၏ checksum၊ manifest၊ detached signature နှင့် provenance ကို isolated verification ဖြင့် ထပ်မံစစ်ဆေးရန်။
- [ ] Filesystem race boundary၊ DNS-to-connection pinning နှင့် host-specific process-cleanup guarantee များအတွက် specification နှင့် regression tests တိုးချဲ့ရန်။

### B1 — Parser/lexer ownership

- [ ] Arbitrary-program parser ownership ကို တိုးချဲ့ရန်။ All valid/invalid grammar၊ function/class/module nesting၊ try/catch နှင့် token-native arbitrary parse coverage များ လိုအပ်သည်။
- [ ] Rust reference diagnostics နှင့် candidate parser diagnostics ကြား full parity matrix တည်ဆောက်ရန်။
- [ ] Generic indentation၊ nested control-flow နှင့် parser-produced AST metadata များအတွက် bounded fixture မဟုတ်သော broader coverage တိုးချဲ့ရန်။

### B2 — Type-checker နှင့် typed IR

- [ ] Full flow-sensitive ownership၊ arbitrary CFG၊ nested condition/loop convergence နှင့် short-circuit path sensitivity ကို ပြီးစီးအောင်လုပ်ရန်။
- [ ] All parser AST expression/statement kinds အတွက် general typed-IR production emitter တည်ဆောက်ရန်။
- [ ] Generic call return instantiation၊ aliasing/mutation၊ loop fixpoint transfer နှင့် complete reference diagnostic parity ကို တိုးချဲ့ရန်။
- [ ] Candidate typed-IR ကို corpus-limited evidence မှ production Zap-owned pipeline အဖြစ် မပြောင်းမီ acceptance matrix ထည့်ရန်။

### B3 — Package/build/test-runner foundation

- [ ] Zap-owned package resolver၊ dependency graph၊ semver range နှင့် deterministic build/test-runner pipeline ကို clean environment မှ reproducible ဖြစ်အောင် ချိတ်ဆက်ရန်။
- [ ] Registry/package metadata နှင့် artifact provenance ကို end-to-end test ဖြင့် စစ်ဆေးရန်။
- [ ] Benchmark၊ peak RSS နှင့် elapsed-time regression evidence များကို CI artifact အဖြစ် ထိန်းသိမ်းရန်။

### B4 — Self-hosting

- [ ] Platform seed ဖြင့် compiler source ကို ပြန် build/run လုပ်နိုင်ကြောင်း သက်သေပြရန်။
- [ ] Self-rebuild output ကို byte-for-byte deterministic ဖြစ်ကြောင်း စစ်ဆေးရန်။
- [ ] Native-independent compiler/VM execution နှင့် full self-hosting acceptance matrix ကို ဖြည့်ရန်။
- [ ] အထက်ပါ acceptance များ မအောင်မြင်သေးသရွေ့ full B4/self-hosting claim မပြုရန်။

## Documentation နှင့် maintenance

- [ ] `docs/CURRENT_STATUS_EN.md` နှင့် `docs/CURRENT_STATUS_MM.md` ကို release status ပြောင်းတိုင်း synchronized update လုပ်ရန်။
- [ ] `CHANGELOG.md`၊ `CHANGELOG_EN.md` နှင့် `CHANGELOG_MM.md` တွင် release scope နှင့် deferred scope ကို တစ်ပြေးညီ မှတ်တမ်းတင်ရန်။
- [ ] Root directory တွင် generated test runners၊ temporary fixtures နှင့် local toolchain artifacts များ မတက်လာစေရန် `.gitignore` pattern များကို ထိန်းသိမ်းရန်။
- [ ] Test scripts များသည် temporary files အားလုံးကို `trap` ဖြင့် cleanup လုပ်ပြီး repository root ကို မညစ်ပတ်စေရန် ပြန်လည်စစ်ဆေးရန်။

## ရှင်းလင်းပြီးသော ဖိုင်များ

ဤ audit တွင် repository root ထဲရှိ script-generated `.zp` runners/fixtures များ၊ local toolchain snapshot/assert artifacts များနှင့် audit အတွက်သာ ဖန်တီးထားသော temporary notes များကို source/documentation မဟုတ်သော clutter အဖြစ် သတ်မှတ်ထားသည်။ ၎င်းတို့ကို ဖယ်ရှားပြီး canonical fixtures များကို `bootstrap/fixtures/` အောက်တွင်သာ ထိန်းသိမ်းမည်။

## အညွှန်းစာတမ်းများ

- [Current status — English](docs/CURRENT_STATUS_EN.md)
- [Current status — Myanmar](docs/CURRENT_STATUS_MM.md)
- [Next TODO plan — English](docs/NEXT_TODO_PLAN_EN.md)
- [Next TODO plan — Myanmar](docs/NEXT_TODO_PLAN_MM.md)
- [Section A checklist — Myanmar](SECTION_A_STATUS_CHECKLIST_MM.md)
- [B3 milestone plan — Myanmar](docs/B3_MILESTONE_PLAN_MM.md)
- [B4 contract — English](docs/B4_RUST_FREE_FULL_LANGUAGE_CONTRACT_EN.md)
