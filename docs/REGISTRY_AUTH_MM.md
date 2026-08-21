# Zap v2.1-B Registry Authentication နှင့် Trusted Registry

## ရည်ရွယ်ချက်

Zap သည် **transport security**၊ **registry trust**၊ **authentication** နှင့် **artifact integrity** တို့ကို သီးခြားခွဲခြားထားသည်။ HTTPS သည် request transport ကို ကာကွယ်ပေးသည်၊ trusted-registry policy သည် မည်သည့် remote origin ကို အသုံးပြုခွင့်ရှိသည်ကို ဆုံးဖြတ်သည်၊ credential သည် request ပြုလုပ်သူ၏ identity ကို သက်သေပြသည်၊ lockfile checksum သည် download ပြုလုပ်ထားသော artifact ကို စစ်ဆေးသည်။ ထိန်းချုပ်မှုတစ်ခုသည် အခြားထိန်းချုပ်မှုကို အစားထိုးမရပါ။

## Trusted registry policy

Remote registry origin များကို နှိုင်းယှဉ်မည့်အရင် canonicalize လုပ်ရမည်။ Zap သည် host name ကို lowercase ပြောင်းခြင်း၊ default port ဖယ်ရှားခြင်း၊ path prefix normalize လုပ်ခြင်း၊ userinfo၊ query၊ fragment၊ backslash၊ whitespace နှင့် traversal segment များကို reject လုပ်ခြင်းတို့ ပြုလုပ်ပြီး entries များကို deterministic order ဖြင့် သိမ်းဆည်းသည်။ Local path နှင့် `file://` source များသည် explicit local-source behavior အတိုင်း ဆက်လက်အလုပ်လုပ်သည်။

Trusted origin များကို အောက်ပါ command များဖြင့် စီမံနိုင်သည်။

```text
zap registry trust list
zap registry trust add https://registry.example/team
zap registry trust remove https://registry.example/team
```

မယုံကြည်ရသော remote ကို network request မပြုလုပ်မီ reject လုပ်သည်။ `http://` ကို default အားဖြင့် ပိတ်ထားပြီး `ZAP_ALLOW_INSECURE_HTTP=1` ကို ထိန်းချုပ်ထားသော fixture များအတွက် explicit သတ်မှတ်သည့်အခါသာ အသုံးပြုရန် ရည်ရွယ်သည်။

## Credential configuration

Credential များကို canonical registry origin နှင့် path prefix အလိုက် scope ချထားသည်။ ပိုမိုတိကျသော path credential သည် ပိုမိုကျယ်ပြန့်သော origin credential ထက် ဦးစားပေးရရှိသည်။ Token ကို HTTPS သို့မဟုတ် local file origin အတွက်သာ လက်ခံပြီး 4096 bytes အတွင်း ကန့်သတ်ထားသည်။ whitespace သို့မဟုတ် control character ပါသော token များကို reject လုပ်သည်။

Shell history ထဲတွင် secret မပေါ်စေရန် environment-variable reference ဖြင့် token ကို configure လုပ်ပါ။

```text
export ZAP_REGISTRY_TOKEN_CI='replace-with-a-secret'
zap registry credential set https://registry.example/team --token-env ZAP_REGISTRY_TOKEN_CI
zap registry credential list
zap registry credential remove https://registry.example/team
```

List command သည် origin များကိုသာ ပြပြီး token တန်ဖိုးကို မပြပါ။ Persistent configuration path ကို `ZAP_REGISTRY_CONFIG`၊ ထို့နောက် `$HOME/.config/zap/registry.json` နှင့် နောက်ဆုံး `.zap/registry.json` အစီအစဉ်ဖြင့် ရွေးချယ်သည်။ File ကို 64 KiB အတွင်း ကန့်သတ်ထားပြီး update လုပ်ရာတွင် temporary file ရေးပြီး atomic replacement ပြုလုပ်သည်။

Credential resolution သည် explicit API token၊ origin-scoped configured credential နှင့် `ZAP_REGISTRY_TOKEN` အစီအစဉ်ဖြင့် လုပ်ဆောင်သည်။ Credential များကို manifest၊ lockfile၊ log၊ diagnostic သို့မဟုတ် changelog ထဲသို့ မရေးရပါ။

## Stable authentication diagnostics

HTTP authentication response များအတွက် stable code များကို သုံးပြီး လက်ရှိ string-based API ကို ဆက်လက်ထိန်းသိမ်းထားသည်။

| Code | အဓိပ္ပာယ် |
|---|---|
| `ZAP-REG-AUTH-001` | `401` response အတွက် credential မရှိခြင်း။ |
| `ZAP-REG-AUTH-002` | `401` response အတွက် ပေးထားသော credential ကို reject လုပ်ခြင်း။ |
| `ZAP-REG-AUTH-003` | `403` response အတွက် credential permission မလုံလောက်ခြင်း။ |

Diagnostic ထဲတွင် canonical origin ကိုသာ ထည့်ပြီး bearer token ကို မထည့်ရပါ။ Authentication မဟုတ်သော service response များသည် ယခင် HTTP status diagnostic ကို ဆက်လက်ထိန်းသိမ်းသည်။

## Operation order

Install၊ update၊ cache နှင့် publish operations များသည် source normalize လုပ်ခြင်း၊ trusted-origin policy enforce လုပ်ခြင်း၊ origin-scoped credential resolve လုပ်ခြင်း၊ secure transport enforce လုပ်ခြင်း၊ request ပြုလုပ်ခြင်း၊ response validate လုပ်ခြင်းနှင့် checksum သို့မဟုတ် signature verify လုပ်ခြင်း အစီအစဉ်အတိုင်း လုပ်ဆောင်ရမည်။ `ZAP_REGISTRY_INDEX` သည် remote index URL ဖြစ်ပါက dependency resolution သည် index request အတွက်လည်း persistent နှင့် environment-backed credential store တူညီစွာ load လုပ်ရမည်။ Offline operation များတွင် authentication သို့မဟုတ် network access မပြုလုပ်ရပါ။

## လက်ရှိ v2.1-B နယ်နိမိတ်

လက်ရှိ slice တွင် canonical origin၊ bounded trust policy၊ persistent policy configuration၊ credential persistence၊ environment-backed token selection၊ credential-aware remote index loading၊ secret redaction၊ trust နှင့် credential CLI commands နှင့် stable 401/403 diagnostics များ ပါဝင်သည်။ Successful authenticated HTTPS fetch/publish အတွက် local TLS server fixture သည် release-review item အဖြစ် ကျန်ရှိနေသည်၊ အကြောင်းမှာ plaintext HTTP fixture များတွင် credential ထည့်သုံးခြင်းကို လုံခြုံရေးမူဝါဒအရ ခွင့်မပြုသောကြောင့် ဖြစ်သည်။ OS keychain၊ certificate pinning၊ automatic redirect support နှင့် signed-index policy enforcement များသည် နောက်ပိုင်း hardening scope အဖြစ် ကျန်ရှိသည်။
