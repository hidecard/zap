# Zap v2.1.8 Release Notes

**Release date:** 2026-08-21

## အနှစ်ချုပ်

Zap v2.1.8 သည် release-integrity နှင့် documentation consistency ကို အဓိကထားသော patch release ဖြစ်ပါသည်။ Release-facing surface အားလုံးအတွက် Cargo package version ကို explicit source of truth အဖြစ် သတ်မှတ်ပြီး ပုံမှန် CI branch ref များကို release tag ဟု မှားယွင်းမယူဆစေရန် ပြင်ဆင်ထားပါသည်။

## ပါဝင်သောပြောင်းလဲမှုများ

- `native/Cargo.toml`၊ `native/Cargo.lock`၊ `zap --version`၊ optional release tag၊ changelog များ၊ bilingual README release link/archive name များ၊ `SECURITY.md`၊ conformance metadata၊ bilingual release note များ၊ release template နှင့် installer metadata များကို စစ်ဆေးသော P0 single-source-of-truth validator ကို ထည့်သွင်းထားပါသည်။
- Package-version drift၊ tag drift နှင့် `master` ကဲ့သို့ branch ref များအတွက် deterministic TSV evidence နှင့် positive/negative regression coverage ကို ထည့်သွင်းထားပါသည်။
- GitHub Actions နှင့် `scripts/release_preflight.sh` တွင် version gate ကို enforce လုပ်ပြီး release publication မတိုင်မီ review လုပ်နိုင်ရန် evidence upload ပြုလုပ်ထားပါသည်။
- Bilingual release-version policy documentation ကို ထည့်သွင်းပြီး onboarding၊ security၊ conformance၊ roadmap နှင့် changelog metadata များကို v2.1.8 baseline သို့ update လုပ်ထားပါသည်။

## Compatibility

ဤ patch release တွင် language syntax အသစ် သို့မဟုတ် async/traits semantics တိုးချဲ့မှု မပါဝင်ပါ။ ရှိပြီးသား Zap program များသည် v2.1 language contract အတွင်း ဆက်လက်အလုပ်လုပ်ပါသည်။ Release-facing ပြောင်းလဲမှုမှာ fail-closed validation ဖြစ်ပြီး package version၊ CLI version၊ tag၊ archive name သို့မဟုတ် documentation surface တစ်ခုခု မကိုက်ညီပါက publication ကို ရပ်တန့်ပါသည်။

## Verification

Master-branch validation run သည် version gate၊ regression harness၊ formatting၊ strict Clippy၊ Cargo check၊ native test suite ၂၅၄ ခုလုံး၊ focused conformance/security/async/parity/ownership gate များနှင့် Linux၊ Windows၊ macOS ARM64 build job များကို အောင်မြင်စွာ ပြီးစီးထားပါသည်။ အပြည့်အစုံ evidence ကို [GitHub Actions run](https://github.com/hidecard/zap/actions/runs/32505190955) တွင် ကြည့်ရှုနိုင်ပါသည်။
