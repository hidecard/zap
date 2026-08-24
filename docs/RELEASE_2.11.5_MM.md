# Zap v2.11.5 Release Notes

**Release line:** v2.11.5
**အကျယ်အဝန်း:** Release-gate၊ provenance နှင့် developer-validation hardening
**အခြေအနေ:** Incremental infrastructure နှင့် documentation release

## အနှစ်ချုပ်

Zap v2.11.5 တွင် cross-platform release contract ကို ပိုမိုခိုင်မာစေရန် Windows CLI workflow အတွင်း version/help output၊ example execution၊ project creation၊ project checking၊ locked build နှင့် project tests များကို မဖြစ်မနေ run စေပါသည်။ Windows smoke operation တစ်ခုခု fail ဖြစ်ပါက platform job သည် fail-closed ဖြစ်ပြီး release publish မလုပ်နိုင်ပါ။

ထို့အပြင် bilingual canonical current-status page များ၊ signed provenance asset သည် machine-readable release identity record ဖြစ်ကြောင်း ရှင်းလင်းချက်နှင့် local prerequisite မရှိခြင်းကို actual test failure နှင့် ခွဲခြားပေးသော `make doctor` / `scripts/doctor.sh` တို့ကို ထည့်ထားပါသည်။

Bootstrap stage သည် **B0** အဖြစ်သာ ဆက်ရှိပါသည်။ Complete compiler/runtime semantics အတွက် Rust သည် reference owner ဖြစ်နေဆဲပါသည်။ ဤပြောင်းလဲမှုများသည် release နှင့် developer evidence ကိုသာ တိုးတက်စေပြီး fully Zap-only compiler၊ self-hosting သို့မဟုတ် B4 ဖြစ်ပြီဟု မဆိုပါ။

## ပြောင်းလဲမှုများ

| နယ်ပယ် | ပြောင်းလဲမှု | Boundary |
|---|---|---|
| Windows release gate | `zap.exe` version/help၊ example၊ `zap new`၊ `zap check`၊ `zap build --locked` နှင့် `zap test` smoke operation များကို မဖြစ်မနေ ထည့်ထားပါသည်။ | Windows job သည် fail-closed ဆက်ဖြစ်သည် |
| Developer diagnostics | `scripts/doctor.sh`၊ `scripts/test_doctor.sh` နှင့် `make doctor` ထည့်ထားပြီး normal/strict mode များက environment မပြည့်စုံမှုနှင့် test failure ကို ခွဲခြားပေးသည်။ | Diagnostic helper သာဖြစ်ပြီး test အစားထိုးမဟုတ်ပါ |
| Current status | Active၊ completed၊ provisional နှင့် deferred အပိုင်းများအတွက် bilingual `docs/CURRENT_STATUS_EN.md` နှင့် `docs/CURRENT_STATUS_MM.md` ထည့်ထားပါသည်။ | Current-status index ဖြစ်ပြီး historical record များ immutable ဆက်ရှိသည် |
| Release provenance | Versioned manifest/provenance asset fields များကို documentation hub များမှ link ချိတ်၍ ရှင်းလင်းဖော်ပြထားပါသည်။ | ရှိပြီးသား signed release schema က authoritative ဖြစ်သည် |
| Validation | Current-status pair check နှင့် doctor regression check များကို documentation consistency နှင့် release preflight ထဲ ထည့်ထားပါသည်။ | Bootstrap stage မတိုးမြှင့်ပါ |

## Open pull request နှင့် branch history

PR #13 နှင့် PR #14 တွင် ပါသော README/bootstrap အလုပ်များသည် လက်ရှိ master documentation နှင့် bootstrap contract များဖြင့် superseded ဖြစ်နေသော stale work ဖြစ်ပါသည်။ PR #1 တွင် ပါသော အဟောင်း security-hardening line ၏ အဓိက safeguards များသည် လက်ရှိ master ၏ runtime၊ registry၊ deployment နှင့် RustSec gate များတွင် ပါဝင်ပြီးဖြစ်ပါသည်။ ရှိပြီးသား tag များကို rewrite မလုပ်ပါ။ PR closure သည် source history နှင့် သီးခြားလုပ်ဆောင်မည်ဖြစ်ပြီး fork branch တစ်ခုမျှကို ဤ release ဖြင့် မဖျက်ပါ။

## တိကျသောကန့်သတ်ချက်များ

B0 bootstrap boundary သည် မပြောင်းလဲပါ။ B1/B2 candidate များသည် provisional နှင့် corpus-limited အဖြစ် ဆက်ရှိပါသည်။ General arbitrary-program parsing၊ complete diagnostic parity၊ broad type inference၊ typed-IR ownership၊ package/build ownership၊ VM ownership၊ platform-seed acceptance နှင့် self-hosting များသည် deferred ဖြစ်နေဆဲပါသည်။

နောက်ဆုံး publish လုပ်ထားသော v2.11.4 release သည် immutable ဖြစ်ပါသည်။ v2.11.5 သည် tag အသစ်ဖြစ်ပြီး v2.11.3 သို့မဟုတ် v2.11.4 ကို rewrite/retag မလုပ်ပါ။

## Verification

Exact committed v2.11.5 candidate သည် version၊ bilingual documentation၊ Markdown link၊ ownership၊ formatting၊ bootstrap၊ native/host tests၊ dependency audit၊ deployment policy နှင့် release preflight checks အားလုံးကို pass ပြီးမှသာ publish လုပ်ရမည်။ Final preflight total ကို ထို exact candidate မှသာ မှတ်တမ်းတင်ရမည်။

## References

[1]: ../docs/CURRENT_STATUS_MM.md
[2]: ../.github/workflows/release.yml
[3]: ../scripts/doctor.sh
[4]: ../scripts/test_doctor.sh
[5]: ../scripts/aggregate_release_manifest.sh
[6]: ../scripts/sign_release_artifacts.sh
