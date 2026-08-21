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

## Provenance policy

Protected release တစ်ခုသည် signed provenance ကို အသုံးပြုရမည်ဖြစ်ပြီး provenance identity မပြည့်စုံပါက fail-closed ဖြစ်ရမည်။ Signed mode တွင် semantic-version tag ref (`refs/tags/vX.Y.Z`)၊ 40-hex commit SHA အပြည့်၊ numeric CI workflow run ID နှင့် HTTPS source URI တို့ မဖြစ်မနေ လိုအပ်သည်။ Signing key သည် 40-hex OpenPGP fingerprint အပြည့်သို့ resolve ဖြစ်ရမည်။ မရှင်းလင်းသော short key ID အစား ထို fingerprint ကို `signing.key_id` ထဲတွင် မှတ်တမ်းတင်သည်။ Signed mode တွင် `TRUSTED_SIGNING_FINGERPRINTS` allowlist လည်း မဖြစ်မနေ လိုအပ်သည်။ လက်ရှိ fingerprint သည် allowlist ထဲရှိ 40-hex entry အပြည့်တစ်ခုနှင့် ကိုက်ညီရမည်။ Allowlist မရှိခြင်း၊ ပုံစံမမှန်ခြင်း သို့မဟုတ် မကိုက်ညီခြင်းတို့သည် fail-closed ဖြစ်ရမည်။ စီစဉ်ထားသော key rotation ကာလအတွင်း old နှင့် new full fingerprint နှစ်ခုကို အချိန်ကန့်သတ်ထားသော transition window အတွက် allowlist ထဲတွင် တစ်ပြိုင်တည်း ထားနိုင်သည်။ Window မကုန်မီ old fingerprint ကို ဖယ်ရှားရမည်ဖြစ်ပြီး allowlist ပြင်ပ key သည် protected release ကို မည်သည့်အခါမျှ sign မလုပ်ရပါ။

Release signer သည် archive၊ manifest နှင့် aggregate checksum index များကို sign မလုပ်မီ manifest subject တစ်ခုချင်းစီကို ၎င်း၏ per-artifact checksum နှင့် စစ်ဆေးသည်။ ထို့နောက် generated provenance document ကို sign လုပ်ပြီး detached signature အားလုံးကို verify ပြုလုပ်ပြီးမှသာ success ပြန်ပေးသည်။ Development-only unsigned mode ကို provenance ထဲတွင် ရှင်းလင်းစွာ ဖော်ပြထားရမည်ဖြစ်ပြီး protected release workflow တွင် မသုံးရပါ။ Identity field ပျောက်ဆုံးခြင်း၊ tag/ref မမှန်ခြင်း၊ commit ပုံစံမမှန်ခြင်း၊ source URI မလုံခြုံခြင်း၊ key မရရှိခြင်း၊ checksum မကိုက်ညီခြင်း သို့မဟုတ် signature verification မအောင်မြင်ခြင်းတို့ ဖြစ်ပါက artifact မ publish မလုပ်ဘဲ operation ကို ရပ်တန့်ရမည်။ Published-release verifier သည် adversarial fixture တစ်ခုချင်းစီကို ပြန်လည် sign လုပ်ပြီးမှ စစ်ဆေးသည့်အတွက် commit၊ subject၊ source၊ ref၊ workflow နှင့် signing-mode metadata များကို ပြောင်းလဲထားသော်လည်း detached signature မှန်ကန်နေခြင်းတစ်ခုတည်းကို လုံလောက်သည်ဟု မယူဆဘဲ reject လုပ်သည်။

## Yanked-release policy

Registry package record တစ်ခုတွင် version တစ်ခုကို dependency resolution အသစ်များအတွက် မရွေးချယ်စေရန် `yanked: true` သတ်မှတ်နိုင်သည်။ အဟောင်း index record များအတွက် field မပါလျှင် `false` ဟု သတ်မှတ်သော်လည်း yanked value ပုံစံမမှန်ပါက safe ဟု တိတ်တဆိတ် မယူဆဘဲ reject လုပ်ရမည်။ Exact-version နှင့် range resolution နှစ်မျိုးစလုံးသည် yanked candidate များကို ကျော်ရမည်။ Exact yanked request သည် `registry package is yanked: <name> <version>` ကို ပြန်ပေးပြီး ကိုက်ညီသည့် range candidate အားလုံး yanked ဖြစ်ပါက `all matching registry packages are yanked: <name> <requirement>` ကို ပြန်ပေးရမည်။ ထို stable diagnostics များသည် မလုံခြုံသော release ကို မရွေးဘဲ fail-closed ဖြစ်စေသည်။

ရှိပြီးသား lockfile တစ်ခုတွင် yanked version ပါနေပါက explicit locked၊ checksum-verified offline သို့မဟုတ် update operation အတွက်သာ ဆက်အသုံးပြုနိုင်သည်။ Resolver သည် lockfile အသစ်ထဲသို့ yanked version မထည့်ရ၊ update operation သည် healthy locked version ကို yanked version ဖြင့် တိတ်တဆိတ် အစားမထိုးရ။ Cache ထဲတွင် artifact ရှိနေခြင်းသည် yanked flag ကို မကျော်လွှားနိုင်ပါ။ Lockfile က version ကို explicit သတ်မှတ်ပြီး checksum ကိုက်ညီသည့်အခါမှသာ cached artifact ကို ဆက်သုံးနိုင်သည်။ Compatibility test သည် explicit locked yanked artifact အတွက် checksum validation ကို ဆက်သုံးနိုင်ပြီး resolver selection ကို ခွင့်မပြုကြောင်း စစ်ဆေးထားသည်။ Registry metadata ကို publish သို့မဟုတ် ပြောင်းလဲရာတွင် deterministic ordering ကို ထိန်းသိမ်းရမည်ဖြစ်ပြီး authenticated၊ signed metadata update မရှိဘဲ yanked marker ကို မဖယ်ရှားရ။

## End-to-end lockfile/cache compatibility audit

End-to-end compatibility contract သည် သီးခြား helper များကိုသာ မဟုတ်ဘဲ locked-cache လမ်းကြောင်းတစ်ခုလုံးကို စစ်ဆေးသည်။ Clean-machine fixture သည် native runtime ကို `--locked` ဖြင့် build လုပ်ပြီး lockfile နှင့် cache verification tests များကို run သည်။ ထို့နောက် cached artifact ကို explicit lockfile version သည် manifest requirement ကို ဖြည့်မီပြီး artifact bytes များသည် မှတ်တမ်းတင်ထားသော SHA-256 checksum နှင့် ကိုက်ညီသည့်အခါမှသာ ပြန်လည်အသုံးပြုနိုင်ကြောင်း စစ်ဆေးသည်။ Explicit locked yanked artifact ကိုလည်း စမ်းသပ်သည်။ ထို artifact သည် verified cache မှ locked operation အတွက် ဆက်အသုံးပြုနိုင်သော်လည်း exact နှင့် range resolution အသစ်များသည် yanked candidate များကို ဆက်လက် reject လုပ်ရမည်။

Audit သည် cache bytes ပြောင်းလဲထားခြင်း၊ lockfile checksum မကိုက်ညီခြင်း၊ lockfile record ပုံစံမမှန်ခြင်းနှင့် manifest requirement ကို မဖြည့်မီသော cached version များကို reject လုပ်သည်။ Offline boundary ကိုလည်း စစ်ဆေးသည်။ Clean copy သည် registry network access မရှိဘဲ ပြီးစီးရမည်ဖြစ်ပြီး locked path သည် cache ရှိနေခြင်းကို အသုံးပြု၍ dependency၊ checksum သို့မဟုတ် yanked-release policy ကို မလျော့စေရပါ။ ပြန်လည်ထပ်လုပ်နိုင်သော command သည် `scripts/verify_clean_machine_locked.sh` ဖြစ်ပြီး final gate တွင် `lockfile_security_tests` နှင့် ရှိပြီးသား checksum-cache regression test ပါဝင်သည်။

## လက်ရှိ v2.1-B နယ်နိမိတ်

လက်ရှိ slice တွင် canonical origin၊ bounded trust policy၊ persistent policy configuration၊ credential persistence၊ environment-backed token selection၊ credential-aware remote index loading၊ test-only rustls fixture ဖြင့် successful authenticated HTTPS fetch/publish coverage၊ secret redaction၊ trust နှင့် credential CLI commands၊ stable 401/403 diagnostics၊ panic မဖြစ်ဘဲ wrong key သို့မဟုတ် mutated payload များကို reject လုပ်သော fail-closed signed-index verification tests နှင့် protected-release provenance အတွက် tag၊ commit၊ workflow၊ source၊ checksum၊ full signing-fingerprint identity နှင့် explicit full-fingerprint rotation allowlist checks များ ပါဝင်သည်။ Fixture သည် injected test agent တစ်ခုတည်းကသာ ယုံကြည်သော generated localhost certificate ကို အသုံးပြုပြီး production request များတွင် ပုံမှန် certificate verification ကို ဆက်လက်ထိန်းသိမ်းထားသည်။ OS keychain၊ certificate pinning၊ automatic redirect support နှင့် production signed-index key-management policy များသည် နောက်ပိုင်း hardening scope အဖြစ် ကျန်ရှိသည်။
