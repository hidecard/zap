# Zap Security Audit နှင့် ပြုပြင်ပြီးအခြေအနေ

**Audit source:** `zap-security-audit-mm.pdf`  
**စစ်ဆေးထားသောအခြေအနေ:** v2.1 development line၊ lockfile/checksum implementation ပြီးနောက်  
**စစ်ဆေးမှုအမျိုးအစား:** Defensive source review၊ local build/test validation နှင့် regression verification

## အနှစ်ချုပ်

ပူးတွဲထားသော security audit တွင် runtime public surface နှင့်သက်ဆိုင်သော finding ၅ ခုကို ဖော်ထုတ်ထားပါသည်။ ပထမ finding သုံးခုမှာ untrusted execution အတွက် အရေးကြီးသော process execution၊ filesystem/environment access နှင့် SSRF/local-network access ဖြစ်ပြီး နောက်ထပ် finding နှစ်ခုမှာ registry local-source trust နှင့် process hard timeout မရှိခြင်းတို့ ဖြစ်ပါသည်။

ယခု runtime တွင် `ZAP_UNTRUSTED=1` ဖြင့် explicit restricted mode ထည့်သွင်းထားပါသည်။ ထို mode တွင် filesystem access၊ environment access၊ process execution၊ outbound network access နှင့် local registry source များကို default အားဖြင့် ပိတ်ထားပါသည်။ HTTP request များတွင် loopback/private/link-local destination များကို reject လုပ်ခြင်း၊ automatic redirect ပိတ်ခြင်း၊ request body limit၊ response-size limit နှင့် timeout များ ထည့်သွင်းထားပါသည်။ Process execution တွင် ပြီးဆုံးပြီးမှ elapsed time စစ်ခြင်းမဟုတ်ဘဲ deadline ရောက်လျှင် child process ကို kill လုပ်နိုင်သော hard timeout ရှိပါသည်။

> Restricted mode သည် runtime capability boundary ဖြစ်ပြီး operating-system sandbox အပြည့်အစုံ မဟုတ်ပါ။ Multi-tenant production deployment များတွင် OS-level sandbox၊ container၊ VM သို့မဟုတ် isolated worker၊ least-privilege credentials နှင့် network egress control များကို ထပ်မံအသုံးပြုရပါမည်။

## Finding Status

| ID | Finding | မူလ Risk | လက်ရှိအခြေအနေ | လက်ရှိ Control |
|---|---|---:|---|---|
| F-01 | Arbitrary process execution | High | Restricted mode တွင် mitigated | `process_run` သည် process capability လိုအပ်ပြီး hard deadline နှင့် stdout/stderr bounds ရှိသည် |
| F-02 | Unrestricted filesystem နှင့် environment access | High | Restricted mode တွင် mitigated၊ trusted mode သည် ရည်ရွယ်ချက်အတိုင်း အားကောင်းနေသည် | File နှင့် environment builtins များသည် capability လိုအပ်ပြီး restricted mode တွင် default deny ဖြစ်သည် |
| F-03 | SSRF နှင့် local-network probing | High | Restricted mode တွင် mitigated၊ OS egress control ဆက်လက်လိုအပ်သည် | Loopback၊ private၊ link-local၊ unspecified၊ broadcast၊ ULA နှင့် IPv6 link-local destination များ reject လုပ်သည်၊ redirect ပိတ်ထားသည် |
| F-04 | Registry local-source trust boundary | Medium | Restricted mode တွင် mitigated | `ZAP_UNTRUSTED=1` ဖြစ်လျှင် `file://` နှင့် bare local registry source များကို reject လုပ်သည် |
| F-05 | Post-hoc process timeout | Medium | Fixed | Child process ကို spawn/poll လုပ်ပြီး deadline ကျော်လျှင် kill လုပ်ကာ deterministic timeout error ပြန်ပေးသည် |

## အကောင်အထည်ဖော်ပြီးသော Controls

### Restricted capability mode

မယုံကြည်ရသော Zap source ကို run မလုပ်မီ host environment တွင် `ZAP_UNTRUSTED=1` သတ်မှတ်ပါ။ အောက်ပါ capability များကို default deny လုပ်ထားပါသည်။

- filesystem read/write;
- environment နှင့် configuration read;
- external process execution;
- outbound HTTP/HTTPS request နှင့် local HTTP serving; နှင့်
- local registry index/package source များ။

Variable မသတ်မှတ်ထားသော trusted local developer mode သည် backward-compatible ဖြစ်နေပါသည်။ Downloaded code ကို run မည့် host များတွင် trusted mode ကို မဖွင့်ဘဲ explicit allowlist သို့မဟုတ် isolated worker အသုံးပြုသင့်ပါသည်။

### Network policy

HTTP request များသည် `http` နှင့် `https` scheme များကိုသာ လက်ခံပါသည်။ Restricted mode တွင် request မပို့မီ DNS resolution လုပ်ပြီး resolved address တစ်ခုချင်းစီကို စစ်ဆေးပါသည်။ Loopback၊ RFC1918/private IPv4၊ link-local၊ unspecified၊ broadcast၊ IPv6 unique-local နှင့် IPv6 link-local destination များကို reject လုပ်ပါသည်။ Public URL တစ်ခုမှ blocked destination သို့ redirect မဖြစ်စေရန် automatic redirect ကို ပိတ်ထားပါသည်။ Request body ကို 64 KiB၊ response ကို 8 MiB အထိသာ ခွင့်ပြုပြီး connect/read/write timeout များကိုလည်း ကန့်သတ်ထားပါသည်။

### Process policy

`process_run` သည် shell string concatenation မသုံးသော non-shell API ဖြစ်သဖြင့် command နှင့် arguments များကို shell command တစ်ခုအဖြစ် မပေါင်းစပ်ပါ။ ယခုတွင် process capability လိုအပ်ခြင်း၊ output limit၊ ten-second deadline၊ overdue child kill နှင့် stable timeout error များ ရှိပါသည်။ CPU၊ memory၊ process-count နှင့် process-group isolation များသည် deployment ဘက်မှ ဆက်လက်ထည့်သွင်းရမည့် controls ဖြစ်ပါသည်။

### Registry source policy

Registry package identity နှင့် SHA-256 artifact validation များကို ဆက်လက် enforce လုပ်ထားပါသည်။ Restricted mode တွင် `file://` နှင့် bare local path registry source များကို reject လုပ်ပါသည်။ Trusted local development fixture များကို restricted mode မဟုတ်သောအခြေအနေတွင် ဆက်လက်အသုံးပြုနိုင်ပါသည်။

## Verification

Capability denial၊ private-network rejection နှင့် oversized HTTP request body အတွက် regression tests များ ထည့်သွင်းထားပါသည်။ Native suite၊ formatting၊ compilation၊ P3 smoke fixture နှင့် whitespace validation အားလုံး အောင်မြင်ပါသည်။

| စစ်ဆေးမှု | ရလဒ် |
|---|---|
| Rust formatting check | Passed |
| Native compilation | Passed |
| Native tests | 236 passed, 0 failed |
| P3.3 smoke fixture | `P3.3 smoke OK` |
| Diff whitespace check | Passed |
| Cross-platform release baseline | v2.0.3 Linux၊ Windows နှင့် macOS matrix passed |

## ကျန်ရှိနေသော Deployment Requirements

Audit ၏ P0 recommendation ဖြစ်သော OS-level sandbox ကို language runtime တစ်ခုတည်းဖြင့် အပြည့်အဝ အာမခံနိုင်မည်မဟုတ်ပါ။ Multi-tenant service၊ downloaded plugin သို့မဟုတ် untrusted CI submission များအတွက် isolated worker boundary၊ least-privilege filesystem permission၊ restricted environment injection၊ network egress filtering၊ resource quota၊ process-group cleanup နှင့် audit logging များ ထည့်သွင်းရပါမည်။

v2.1 တွင် ဆက်လက်လုပ်ဆောင်ရန်မှာ workspace-confined filesystem mode နှင့် symlink-safe canonicalization၊ explicit trusted-registry policy၊ target OS တစ်ခုချင်းစီအတွက် process-group/resource controls၊ CI dependency audit နှင့် SSRF၊ path escape၊ environment filtering၊ timeout၊ body-size နှင့် registry-source integration fixtures များ ဖြစ်ပါသည်။

## လုံခြုံစွာအသုံးပြုရန်

`ZAP_UNTRUSTED=1` ကို capability denial နှင့် request policy အတွက် defensive runtime mode အဖြစ် သတ်မှတ်ရပါမည်။ Kernel-enforced sandbox အစားထိုးအဖြစ် မဖော်ပြသင့်ပါ။ Security-sensitive host များတွင် operating-system isolation နှင့် ပေါင်းစပ်အသုံးပြုပြီး downloaded Zap source၊ package index နှင့် package archive များကို verify မလုပ်မချင်း untrusted input အဖြစ် ကိုင်တွယ်ရပါမည်။
