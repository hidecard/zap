# Zap Registry Production Deployment Boundaries

**Verified baseline:** Zap v2.2.0
**ရည်ရွယ်ချက်:** Local နှင့် public registry deployment boundary၊ validation၊ TLS၊ supervision၊ credential၊ quota နှင့် egress control များအတွက် operator reference ဖြစ်သည်။
**လမ်းညွှန်:** [Documentation hub](DOCUMENTATION_NAVIGATION_MM.md) · [Package author guide](PACKAGE.md) · [Stdlib reference](STDLIB_INDEX_MM.md) · [Language specification](LANGUAGE_SPEC_MM.md) · [Security policy](../SECURITY.md) · [Release policy](RELEASE_VERSION_POLICY_MM.md)

## အကျယ်အဝန်း

Zap တွင် controlled local registry service ပါဝင်သော်လည်း public production deployment ပြုလုပ်မည်ဆိုပါက service ပတ်ဝန်းကျင်တွင် သတ်မှတ်ထားသော operating boundary များ ထပ်မံလိုအပ်ပါသည်။ ဤလမ်းညွှန်သည် repository အတွင်းရှိ reproducible reference policy ကို သတ်မှတ်ထားခြင်းဖြစ်သည်။ Operator contract နှင့် validation target အဖြစ် အသုံးပြုရန်ဖြစ်ပြီး certificate provision၊ cloud resource ဖန်တီးခြင်း သို့မဟုတ် registry ကို အလိုအလျောက် public publish လုပ်ခြင်း မပြုလုပ်ပါ။

## Reference artifacts များ

| Artifact | ရည်ရွယ်ချက် |
|---|---|
| `deploy/zap-registry.service` | Linux service supervision၊ least privilege၊ filesystem protection၊ quota နှင့် loopback-only network access |
| `deploy/zap-registry.nginx.conf` | TLS termination၊ HTTP-to-HTTPS redirect၊ request limit၊ allowed methods နှင့် loopback upstream policy |
| `deploy/registry.env.example` | deployment-secret-manager ချိတ်ဆက်ရန် redacted environment template |
| `deploy/registry-deployment-policy.toml` | bind address၊ limit၊ sandbox၊ credential နှင့် egress များအတွက် machine-readable deployment contract |
| `scripts/validate_registry_deployment.sh` | reference control များနှင့် secret-file hygiene ကို dependency မလိုဘဲ CI/operator စစ်ဆေးရန် |

## TLS နှင့် ingress

Registry process သည် `127.0.0.1:8787` တွင်သာ bind လုပ်ပြီး public traffic ကို တိုက်ရိုက်လက်ခံရန် မရည်ရွယ်ပါ။ Reference nginx configuration သည် TLS 1.2 သို့မဟုတ် TLS 1.3 ဖြင့် TLS termination ပြုလုပ်သည်၊ cleartext HTTP ကို HTTPS သို့ redirect လုပ်သည်၊ request body ကို 1 MiB အထိ ကန့်သတ်သည်၊ `GET` နှင့် `POST` ကိုသာ ခွင့်ပြုသည်၊ ထို့နောက် bounded proxy timeout များဖြင့် loopback service သို့ forward လုပ်သည်။ Operator သည် example hostname နှင့် certificate path များကို မိမိ platform မှ စီမံထားသော certificate များဖြင့် အစားထိုးရမည်။ Private key များကို repository အပြင်တွင်သာ ထားရမည်။

Backend သည် loopback traffic ကိုသာ လက်ခံသည်။ Public DNS၊ certificate renewal၊ external rate limiting နှင့် organization-specific WAF policy များကို ingress layer က စီမံရမည်။ ဤ control များသည် runtime assumption မဟုတ်ဘဲ deployment provider ၏ တာဝန်အဖြစ် သီးခြားထားရှိထားသည်။

## Supervision နှင့် sandbox

Systemd unit သည် service ကို dynamic `zap-registry` user ဖြင့် run လုပ်ပြီး `NoNewPrivileges`၊ private temporary/device၊ protected system/home path၊ restrictive umask နှင့် write လုပ်ခွင့်ရှိသော directory တစ်ခုဖြစ်သည့် `/var/lib/zap-registry` ကိုသာ အသုံးပြုသည်။ Failed service ကို restart လုပ်ပေးပြီး shutdown အချိန်တွင် process group တစ်ခုလုံးကို ရပ်တန့်ကာ bounded stop timeout အသုံးပြုသည်။ `IPAddressDeny=any` နှင့် loopback allow rule များကြောင့် backend သည် external network connection မပြုလုပ်နိုင်ပါ။ TLS proxy သည် public-facing component တစ်ခုတည်း ဖြစ်သည်။

ဤ unit သည် Linux reference ဖြစ်သည်။ Windows နှင့် macOS deployment များတွင် မိမိတို့၏ native service manager၊ sandbox၊ firewall နှင့် secret-management facility များဖြင့် အလားတူ control များ ပေးရမည်။ CI သည် portable policy contract နှင့် Linux artifact text ကို စစ်ဆေးသော်လည်း operating system အားလုံးတွင် kernel isolation တူညီသည်ဟု မဆိုလိုပါ။

## Resource quota များ

Reference policy သည် memory ကို 256 MiB၊ CPU ကို 50 ရာခိုင်နှုန်း၊ task ကို 64 ခုနှင့် open file ကို 1,024 ခုအထိ ကန့်သတ်ထားသည်။ Registry protocol တွင်လည်း bounded request၊ body၊ response နှင့် timeout limit များကို ဆက်လက်ထိန်းသိမ်းထားသည်။ Workload လိုအပ်ချက်ကို တိုင်းတာပြီးနောက် Operator သည် ဤတန်ဖိုးများကို လျှော့ချနိုင်သော်လည်း explicit risk decision မမှတ်တမ်းတင်ဘဲ limit များကို ဖယ်ရှားခြင်း မပြုရ။

## Credential များ

Authenticated service အတွက် `ZAP_REGISTRY_TOKEN` နှင့် `ZAP_REGISTRY_SIGNING_SECRET` လိုအပ်သည်။ ၎င်းတို့ကို deployment secret manager သို့မဟုတ် အလားတူ protected facility မှ inject လုပ်ရမည်။ File-backed ဖြစ်ပါက mode `0600` ဖြင့် သိမ်းရမည်။ Log၊ archive၊ process argument နှင့် source control များတွင် မပါဝင်ရ။ `deploy/registry.env.example` တွင် placeholder များသာ ပါဝင်သည်။ Validator သည် `deploy/` အောက်ရှိ populated `registry.env`၊ private-key နှင့် certificate file များကို reject လုပ်သည်။

## Egress control

Registry backend သည် loopback-only ဖြစ်သည်။ Reference policy တွင် external egress ကို ပိတ်ထားပြီး service သည် registry data directory သို့သာ ရေးသားနိုင်သည်။ Package retrieval အတွက် outbound connection လိုအပ်ပါက registry service ၏ network permission ကို မသိမသာ ချဲ့ထွင်မည့်အစား သီးခြား allowlisted component တစ်ခုမှသာ ဆောင်ရွက်ရမည်။

## Validation

Service ကို install သို့မဟုတ် publish မလုပ်မီ repository root မှ အောက်ပါ command ကို run လုပ်ရမည်။

```bash
scripts/validate_registry_deployment.sh
```

Validator သည် reference artifact အားလုံးရှိ/မရှိ၊ TLS နှင့် loopback ingress rule များ၊ sandbox နှင့် quota control များ၊ credential ကို secret manager မှ ရယူရမည့် policy၊ external egress ပိတ်ထားမှုနှင့် deployment tree အောက်တွင် populated secret/private-key file မရှိမှုတို့ကို စစ်ဆေးသည်။ GitHub Actions quality workflow တွင်လည်း ဤ gate ကို run လုပ်သည်။

## Boundary နှင့် မပါဝင်သည့် အရာများ

ဤ reference layer ဖြင့် repository-side production-boundary contract ပြီးစီးပါသည်။ သို့သော် public deployment ပြုလုပ်ခြင်း၊ certificate ထုတ်ပေးခြင်း၊ DNS configure လုပ်ခြင်း၊ system package install လုပ်ခြင်း၊ cloud firewall ဖန်တီးခြင်း သို့မဟုတ် OS အားလုံးအတွက် universal sandbox abstraction ပေးခြင်း မပါဝင်ပါ။ ဤအဆင့်များသည် platform-specific operational work ဖြစ်ပြီး registry ကို Internet သို့ ဖွင့်မီ သီးခြား review ပြုလုပ်ရမည်။
