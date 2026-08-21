# Release Version Single-Source-of-Truth Policy

## Authority

`native/Cargo.toml` ထဲရှိ native package version သည် Zap release version ၏ authoritative source ဖြစ်ပါသည်။ `native/Cargo.lock` ထဲရှိ `zap-native` package version သည် ထို version နှင့် တူညီရမည်ဖြစ်ပြီး compiled CLI သည် `zap --version` မှတစ်ဆင့် တူညီသော version ကို report လုပ်ရမည်။

Package version၊ tag၊ CLI output၊ changelog၊ bilingual README onboarding၊ security policy သို့မဟုတ် release note များ မကိုက်ညီပါက release workflow သည် artifact များကို publish မလုပ်ရပါ။ Version ကို အဟောင်း document မှ ခန့်မှန်းခြင်း သို့မဟုတ် release surface တစ်နေရာမှ တစ်နေရာသို့ manual copy လုပ်ခြင်းမဟုတ်ဘဲ validator ဖြင့် တိတိကျကျ စစ်ဆေးထားပါသည်။

## လိုအပ်သော release surface များ

| Surface | လိုအပ်သော contract |
|---|---|
| `native/Cargo.toml` | Authoritative semantic version |
| `native/Cargo.lock` | ကိုက်ညီသော `zap-native` package version |
| `zap --version` | ကိုက်ညီသော CLI output |
| `CHANGELOG.md`၊ `CHANGELOG_EN.md`၊ `CHANGELOG_MM.md` | လက်ရှိ release version ကို ဖော်ပြရမည် |
| `README.md`၊ `README_MM.md` | လက်ရှိ release line၊ release URL နှင့် platform archive သုံးမျိုးကို current အဖြစ် ဖော်ပြရမည် |
| `SECURITY.md` | Supported release line နှင့် official release-integrity URL ကို current အဖြစ် ဖော်ပြရမည် |
| `docs/RELEASE_<VERSION>_EN.md`၊ `docs/RELEASE_<VERSION>_MM.md` | Version အတွက် bilingual release notes ရှိရမည် |
| Git tag `v<VERSION>` | Tag ပေးထားပါက authoritative package version နှင့် ကိုက်ညီရမည် |

## Validation နှင့် evidence

Local တွင် အောက်ပါ gate ကို run နိုင်ပါသည်။

```bash
EXPECTED_VERSION=2.1.9 \
RELEASE_TAG=v2.1.9 \
ZAP_VERSION_REPORT=target/version-consistency.tsv \
scripts/validate_release_version.sh 2.1.9
scripts/test_validate_release_version.sh
```

Validator သည် deterministic TSV evidence ထုတ်ပေးပြီး package/lockfile drift၊ CLI drift၊ stale onboarding link သို့မဟုတ် archive name၊ stale security link၊ bilingual release note မရှိခြင်း၊ hard-coded release template version သို့မဟုတ် tag mismatch များကို fail-closed ပြုလုပ်ပါသည်။ `master` ကဲ့သို့ ရိုးရိုး branch ref များကို release tag ဟု မယူဆပါ။ Implicit tag validation သည် semver ပုံစံရှိသော `v<VERSION>` ref များတွင်သာ အလုပ်လုပ်ပြီး explicit `RELEASE_TAG` ပေးထားပါက အမြဲ enforce လုပ်ပါသည်။ GitHub Actions quality job သည် report ကို upload လုပ်ပြီး positive/negative regression harness ကို run လုပ်ပါသည်။ `scripts/release_preflight.sh` သည် အခြား P0/P1 contract နှင့် deployment gate များမတိုင်မီ ထို validator ကို run လုပ်ပါသည်။

## Release workflow

`scripts/bump_release.sh` ကို dry-run mode ဖြင့် အရင် run လုပ်ရမည်။ ထုတ်ပေးသော Cargo နှင့် changelog diff ကို review လုပ်ပြီး versioned bilingual release notes များ update လုပ်ရမည်။ ထို့နောက် version gate နှင့် full release preflight ကို run လုပ်၍ commit ပြီးမှသာ ကိုက်ညီသော annotated tag ကို ဖန်တီးပြီး push လုပ်ရမည်။ Tag-triggered workflow သည် artifact build/publish မလုပ်မီ tagged source တွင် gate ကို ထပ်မံ run လုပ်ပါသည်။

Documentation-only release mismatch သည် user အသစ်ကို release ဟောင်းသို့ ညွှန်ပြနိုင်သောကြောင့် release-blocking defect ဖြစ်ပါသည်။ `ALLOW_DIRTY`၊ မကိုက်ညီသော tag သို့မဟုတ် manually edited artifact name များဖြင့် version gate ကို bypass မလုပ်ရပါ။
