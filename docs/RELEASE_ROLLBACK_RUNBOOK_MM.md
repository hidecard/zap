# Zap Release Rollback နှင့် Quarantine လုပ်ငန်းစဉ်လမ်းညွှန်

## ရည်ရွယ်ချက်

ဤလုပ်ငန်းစဉ်လမ်းညွှန်သည် မပြည့်စုံသော၊ verify မလုပ်နိုင်သော၊ tamper ဖြစ်နိုင်သော သို့မဟုတ် လုပ်ငန်းလည်ပတ်မှုအတွက် မလုံခြုံသော Zap release တစ်ခုကို ထိန်းချုပ်တုံ့ပြန်ရန် သတ်မှတ်ထားပါသည်။ ၎င်းသည် v2.1-E release pipeline အတွက် သက်ဆိုင်ပြီး GitHub Release assets၊ platform installers၊ checksums၊ signatures၊ provenance metadata နှင့် registry deployment reference များကို အကျုံးဝင်ပါသည်။

Rollback သည် ထိန်းချုပ်ထားသော ပြောင်းလဲမှုတစ်ခုဖြစ်ပါသည်။ အထောက်အထားများကို မဖျက်ရ၊ ရှိပြီးသား tag ကို ပြန်ရေးရ၊ publish လုပ်ထားသော asset ကို မည်သူမျှမသိဘဲ အစားထိုးရပါ။ ထိခိုက်နေသော release ကို quarantine လုပ်ပြီး logs နှင့် hashes များကို ထိန်းသိမ်းကာ incident ပြန်လည်သုံးသပ်ချိန်အထိ last known-good release သို့ အသုံးပြုသူများကို ညွှန်ပြရပါမည်။

## ကြိုတင်လိုအပ်ချက်များ

Operator သည် repository၊ protected release environment၊ release verification script၊ public signing key၊ registry backup/index storage နှင့် incident communication channel များကို အသုံးပြုခွင့်ရှိရပါမည်။ Production secrets များကို အတည်ပြုထားသော secret manager သို့မဟုတ် CI environment မှသာ ပေးသွင်းရမည်ဖြစ်ပြီး ဤ repository ထဲသို့ မကူးယူရပါ။

လိုအပ်သော command နှင့် file များမှာ အောက်ပါအတိုင်း ဖြစ်ပါသည်။

```text
scripts/verify_published_release.sh
scripts/validate_registry_deployment.sh
scripts/aggregate_release_manifest.sh
docs/RELEASE_ROLLBACK_RUNBOOK_EN.md
docs/RELEASE_ROLLBACK_RUNBOOK_MM.md
```

## Severity နှင့် စတင်အသုံးပြုရမည့် အခြေအနေများ

| စတင်စေသောအခြေအနေ | ကနဦး severity | လိုအပ်သောလုပ်ဆောင်ချက် |
|---|---:|---|
| Release asset ပျောက်ဆုံးခြင်း သို့မဟုတ် upload မပြည့်စုံခြင်း | High | Promotion ရပ်ပြီး release ကို quarantine လုပ်ပါ |
| Checksum မကိုက်ညီခြင်း | Critical | ထိခိုက်သော asset ကို install/distribute မလုပ်ပါနှင့် |
| Signature သို့မဟုတ် provenance verification မအောင်မြင်ခြင်း | Critical | Key နှင့် artifact state ပြန်လည်သုံးသပ်သည်အထိ assets အားလုံး quarantine လုပ်ပါ |
| Installer ပျက်ခြင်း သို့မဟုတ် upgrade မလုံခြုံခြင်း | High | နောက်ထပ် promotion ရပ်ပြီး previous stable version သို့ အသုံးပြုသူများကို ညွှန်ပါ |
| Registry index သို့မဟုတ် cache ပျက်စီးခြင်း | Critical | Write များ freeze လုပ်ပြီး last known-good signed state ကို restore လုပ်ပါ |
| Credential သို့မဟုတ် signing-key ပေါက်ကြားခြင်း | Critical | Credential revoke/rotate လုပ်ပြီး ဆက်စပ် release များ quarantine လုပ်ပါ |
| Release ပြီးနောက် severe regression ဖြစ်ခြင်း | High | Promotion ရပ်ပြီး rollback assessment စတင်ပါ |

## ချက်ချင်း containment လုပ်ဆောင်ချက်များ

1. Release tag၊ commit SHA၊ workflow run ID၊ publish timestamp၊ သတင်းပို့သူနှင့် ပထမဆုံးတွေ့ရှိသည့် လက္ခဏာကို မှတ်တမ်းတင်ပါ။
2. GitHub Release သို့မဟုတ် tag ကို မဖျက်ပါနှင့်၊ မရွှေ့ပါနှင့်။ မူလအခြေအနေကို အထောက်အထားအဖြစ် ထိန်းသိမ်းပါ။
3. Incident record ထဲတွင် release ကို quarantined ဟု မှတ်ပြီး pending promotion၊ marketplace publication၊ registry synchronization သို့မဟုတ် installer distribution အားလုံးကို ရပ်ပါ။
4. Release documentation နှင့် operator communication များကို last known-good version သို့ ညွှန်ပါ။ Verification မအောင်မြင်မချင်း replacement asset ကို safe ဟု မကြေညာပါနှင့်။
5. Credential သို့မဟုတ် signing material ပေါက်ကြားနိုင်သည်ဟု ယူဆရပါက artifact အသစ်မတည်ဆောက်မီ revoke သို့မဟုတ် rotate လုပ်ပါ။

## Verification နှင့် အထောက်အထားစုဆောင်းခြင်း

Published assets များကို သီးခြား isolated directory ထဲသို့ download လုပ်ပြီး public verification key ကို `GNUPGHOME` မှတစ်ဆင့် ရရှိစေကာ verification script ကို run ပါ။

```bash
mkdir -m 700 /tmp/zap-release-incident
# အတည်ပြုထားသော GitHub/registry လုပ်ထုံးလုပ်နည်းဖြင့် assets များကို download လုပ်ပါ။
GNUPGHOME=/secure/verification/gnupg \
  bash scripts/verify_published_release.sh 2.1.0 /tmp/zap-release-incident
```

Verifier output၊ manifest၊ provenance file၊ aggregate checksum file၊ signature files၊ artifact listing နှင့် သက်ဆိုင်သော CI logs များကို incident evidence အဖြစ် သိမ်းဆည်းပါ။ Manifest ထဲရှိ commit နှင့် release ref ကို ရည်ရွယ်ထားသော source commit နှင့် တိုက်စစ်ပါ။ မကိုက်ညီပါက release integrity failure ဖြစ်ပါသည်။

## Rollback ဆုံးဖြတ်ချက်

Checksum သို့မဟုတ် signature ကို verify မလုပ်နိုင်ခြင်း၊ လိုအပ်သော artifact ပျောက်ဆုံးခြင်း၊ installer က ရှိပြီးသား installation ကို ထိခိုက်နိုင်ခြင်း၊ registry index ကို ယုံကြည်၍ မရခြင်း သို့မဟုတ် safe mitigation မရှိသော ပြင်းထန်သည့် regression ဖြစ်ခြင်းတို့တွင် rollback လုပ်ရပါမည်။ Release owner နှင့် security သို့မဟုတ် operations reviewer တစ်ဦးက rollback ဆုံးဖြတ်ချက်ကို အတည်ပြုရပါမည်။ အကြောင်းရင်း၊ evidence၊ ရွေးချယ်ထားသော last known-good version နှင့် အသုံးပြုသူအပေါ် သက်ရောက်မှုကို မှတ်တမ်းတင်ရပါမည်။

## GitHub Release Quarantine

Repository ၏ release policy အတိုင်း GitHub controls ကို အသုံးပြုပြီး ထိခိုက်သော release ကို draft သို့မဟုတ် ထပ်မံဖြန့်ဝေမရနိုင်သော အခြေအနေသို့ ပြောင်းပါ။ ရှိပြီးသား asset များကို နေရာတစ်ခုတည်းတွင် overwrite မလုပ်ပါနှင့်။ မူလ hash များကို ထိန်းသိမ်းထားပြီး မူလ assets များကို authorized reviewer များသာ ရရှိနိုင်သော incident storage သို့ ရွှေ့နိုင်ပါသည်။

ပြင်ဆင်ထားသော release လိုအပ်ပါက version အသစ် သို့မဟုတ် အတည်ပြုထားသော corrective tag အသစ် ဖန်တီးပါ။ မတူညီသော bytes များအတွက် version tag တစ်ခုတည်းကို ပြန်အသုံးမပြုပါနှင့်။ Corrected release သည် preflight၊ artifact aggregation၊ signing၊ provenance နှင့် post-publish verification gates အားလုံး အောင်မြင်ပြီးမှသာ ကြေညာရပါမည်။

## Registry Rollback

1. Index state ကို စစ်ဆေးနေစဉ် registry writes နှင့် package publication များကို freeze လုပ်ပါ။
2. Protected backup storage ထဲရှိ last known-good signed index နှင့် checksum ကို သတ်မှတ်ပါ။
3. Restore မလုပ်မီ backup signature နှင့် checksum ကို verify လုပ်ပါ။
4. Index ကို atomically restore လုပ်ပြီး စစ်ဆေးရန် previous state ကို ထိန်းသိမ်းပါ။ ထို့နောက် operations မှ အတည်ပြုထားသော managed service path ကိုသာ restart လုပ်ပါ။
5. Trusted-registry၊ cache-integrity နှင့် package-resolution checks များကို ပြန် run လုပ်ပါ။
6. Restore လုပ်ထားသော index နှင့် service health မှန်ကန်ကြောင်း operations reviewer အတည်ပြုပြီးမှ writes ပြန်ဖွင့်ပါ။

အသစ်ဖြစ်သည် သို့မဟုတ် လုပ်ငန်းလည်ပတ်ရန် လွယ်ကူသည်ဆိုသော အကြောင်းတစ်ခုတည်းဖြင့် unsigned သို့မဟုတ် unverified index ကို မ restore လုပ်ပါနှင့်။

## အသုံးပြုသူနှင့် Stakeholder ဆက်သွယ်မှု

ပထမဆုံးအသိပေးချက်တွင် ထိခိုက်သော version၊ အသုံးပြုသူများ လုပ်ဆောင်ရမည့်အရာ၊ last known-good version နှင့် downloads သို့မဟုတ် registry installs ရပ်ထားခြင်း ရှိ/မရှိကို ဖော်ပြပါ။ Private incident details၊ credentials၊ signing keys သို့မဟုတ် အတည်မပြုရသေးသော root-cause ခန့်မှန်းချက်များကို မထုတ်ပြန်ပါနှင့်။ နောက်ဆုံးအသိပေးချက်တွင် corrected version၊ verification instructions၊ migration/rollback guidance နှင့် incident closure time ကို ထည့်ပါ။

English နှင့် Burmese release communications များတွင် ထိခိုက်သော version များ၊ အသုံးပြုသူလုပ်ဆောင်ရမည့်အချက်များနှင့် limitations များ တူညီရပါမည်။

## ဝန်ဆောင်မှုပြန်ဖွင့်ရန် Checklist

| စစ်ဆေးမှု | လိုအပ်သောအထောက်အထား |
|---|---|
| Corrected source tag သည် immutable ဖြစ်ခြင်း | Commit နှင့် tag references |
| Preflight အောင်မြင်ခြင်း | CI run link နှင့် preflight summary |
| Platform artifacts အားလုံးရှိခြင်း | Artifact manifest |
| Checksums ကိုက်ညီခြင်း | Aggregate နှင့် per-artifact checksum output |
| Signatures verify အောင်မြင်ခြင်း | Public-key verification output |
| Provenance သည် ရည်ရွယ်ထားသော source နှင့်ကိုက်ညီခြင်း | Provenance JSON နှင့် commit comparison |
| Install/upgrade checks အောင်မြင်ခြင်း | Platform installer test results |
| Registry state ယုံကြည်စိတ်ချရခြင်း | Restored signed index နှင့် service health evidence |
| Documentation update ပြီးခြင်း | English/Burmese release notes နှင့် rollback notice |
| Reviewer approval မှတ်တမ်းရှိခြင်း | Incident နှင့် release approval record |

စစ်ဆေးမှုအားလုံး အောင်မြင်ပြီးမှသာ release ကို quarantine မှ ဖယ်ရှားခြင်း၊ corrected release ကို ကြေညာခြင်းနှင့် registry သို့မဟုတ် marketplace promotion ကို ပြန်ဖွင့်ခြင်းတို့ကို လုပ်ဆောင်ရပါမည်။

## Incident ပြီးနောက် လုပ်ဆောင်ချက်များ

Incident timeline၊ ထိခိုက်သော hashes၊ CI logs၊ operator commands၊ အသုံးပြုသူအပေါ်သက်ရောက်မှု၊ root cause နှင့် corrective actions များကို ထိန်းသိမ်းပါ။ ပေါက်ကြားနိုင်သော credentials များကို rotate လုပ်ပါ။ Failure mode အတွက် regression test ထည့်ပြီး release preflight သို့မဟုတ် verification gate ကို update လုပ်ပါ။ Signing၊ provenance၊ backup သို့မဟုတ် access-control policy များ ပိုမိုခိုင်မာရန် လို/မလို ပြန်လည်သုံးသပ်ပါ။

## လုံခြုံရေးနယ်နိမိတ်များ

ဤ runbook သည် production access၊ credential rotation၊ payment သို့မဟုတ် public communication ကို တစ်ဦးတည်း ခွင့်ပြုခြင်းမဟုတ်ပါ။ ထိုလုပ်ဆောင်ချက်များသည် repository ၏ အတည်ပြုထားသော operator နှင့် reviewer permissions များ လိုအပ်ပါသည်။ ဤ runbook သည် ပြန်လည်အသုံးပြုနိုင်သော reference procedure ဖြစ်ပြီး environment-specific hostnames၊ secrets၊ certificate paths နှင့် private infrastructure details များကို repository အပြင်တွင်သာ ထားရပါမည်။
