# Zap တွင် Production OAuth2/OIDC နှင့် JWT Authentication

Zap ၏ production host adapter ကို **OAuth2 resource server** အဖြစ် တည်ဆောက်ရမည်။ Browser login နှင့် OAuth2 authorization-code flow ကို Identity Provider က ဆောင်ရွက်ပြီး Zap သို့ access token ရောက်လာသောအခါ request သည် gateway/repository ထံ မရောက်မီ validate လုပ်ရမည်။ ID token ကို API access token အဖြစ် မသုံးရပါ။ Browser client များအတွက် Authorization Code + PKCE နှင့် identity provider တွင် exact redirect URI registration ကို အသုံးပြုပါ [1]။

## Framework branch တွင် ထည့်သွင်းထားသောအရာ

`host/zap-host/src/auth.rs` တွင် `JwtAuthenticator` နှင့် bounded OIDC/JWT configuration contract ပါရှိပါသည်။ `ZAP_AUTH_MODE=jwt` ဖြစ်လျှင် `AppState::from_env` က ၎င်းကို ရွေးချယ်ပါသည်။ Local starter သည် identity provider မရှိဘဲ run နိုင်ရန် default demo mode ကို ဆက်လက်ထားရှိပြီး production deployment template/policy တွင် JWT mode သတ်မှတ်ထားပါသည်။

Request extension ထဲ `Identity` မထည့်မီ အောက်ပါ checks များကို လုပ်ဆောင်ပါသည်။

| Check | Production behavior |
|---|---|
| Authorization header | `Bearer <token>` value တစ်ခုလိုအပ်ပြီး token size ကို bounded လုပ်ထားသည် |
| Algorithm | Explicit allowlist လိုအပ်ပြီး deployment template တွင် `RS256` သာ ခွင့်ပြုသည် |
| Key ID | `kid` လိုအပ်ပြီး configured JWKS document ထဲတွင် lookup လုပ်သည် |
| Signature | ရွေးချယ်ထားသော RSA/EC JWK ဖြင့် JWS signature verify လုပ်သည် |
| Claims | `iss`၊ `aud`၊ `exp` နှင့် `nbf` စစ်ပြီး bounded non-empty `sub` လိုအပ်သည် |
| Scopes | OAuth `scope` နှင့် provider-style `scp` ကို လက်ခံ၊ normalize နှင့် deduplicate လုပ်သည် |
| Provider failure | JWKS network failure ကို `503`၊ malformed/expired/incorrect token ကို `401` ပြန်သည် |
| Key rotation | JWKS ကို bounded TTL ဖြင့် cache လုပ်သည်။ Unknown `kid` တွင် serialized refresh တစ်ကြိမ်လုပ်ပြီး refresh cooldown ထားသည်။ Stale key fail-open မလုပ်ပါ |

Algorithm၊ issuer၊ audience၊ JWKS URL၊ clock skew၊ cache TTL နှင့် token-size limit များကို startup တွင် validate လုပ်ပါသည်။ JWKS URL သည် HTTPS ဖြစ်ရမည်။ JWKS HTTP client တွင် redirect ပိတ်ထားပြီး connection/request timeout ကို bounded လုပ်ထားပါသည်။

## Production environment

`deploy/zap-web.env.example` ကို secret/configuration management ဖြင့် ကူးယူပြီး placeholder များကို ပြင်ပါ။ Placeholder ပါသော file ကို commit မလုပ်ပါနှင့်။

```dotenv
ZAP_AUTH_MODE=jwt
ZAP_AUTH_ISSUER=https://login.example.com/
ZAP_AUTH_AUDIENCE=https://api.example.com
ZAP_AUTH_JWKS_URL=https://login.example.com/.well-known/jwks.json
ZAP_AUTH_ALLOWED_ALGORITHMS=RS256
ZAP_AUTH_CLOCK_SKEW_SECONDS=30
ZAP_AUTH_JWKS_CACHE_SECONDS=300
ZAP_AUTH_MAX_TOKEN_BYTES=16384
```

Issuer နှင့် audience သည် provider ထုတ်ပေးသော access-token contract နှင့် အတိအကျ ကိုက်ညီရမည်။ API သည် frontend client identifier မဟုတ်ဘဲ API အတွက် ထုတ်ပေးသော `aud` ပါသည့် access token ကို လက်ခံသင့်ပါသည်။ JWKS endpoint ကို stable ထားပြီး key အသစ်၏ `kid` ပါသော token မထုတ်မီ provider ၏ public key အသစ်ကို publish လုပ်ပါ။

## Authorization flow boundary

Browser/mobile client သည် provider ၏ Authorization Code + PKCE flow ကို သုံးသင့်ပါသည်။ Client က provider ထံ code exchange လုပ်ပြီး Zap API အတွက် access token ရယူကာ HTTPS `Authorization: Bearer` header ဖြင့် ပို့ရမည်။ Zap သည် access token ကို validate လုပ်မည်ဖြစ်သော်လည်း login page၊ password grant၊ token endpoint၊ refresh-token store သို့မဟုတ် browser session cookie ကို မတည်ဆောက်ပါ။

Authentication ပြီးနောက် authorization သည် သီးခြားဆုံးဖြတ်ချက် ဖြစ်ပါသည်။ Handler များတွင် `users:read`၊ `users:write` ကဲ့သို့ scope လိုအပ်ချက် ထည့်ပြီး repository query များတွင် subject/tenant ownership ကို enforce လုပ်ရမည်။ Signature မှန်ခြင်းတစ်ခုတည်းဖြင့် resource အားလုံးကို ခွင့်ပြုသည်ဟု မယူဆရပါ။

## HTTP error contract

| Situation | Response |
|---|---|
| Missing/malformed/expired၊ issuer/audience မကိုက်၊ signature မှား၊ algorithm မခွင့်ပြု၊ `kid` မတွေ့ | `401 unauthenticated` |
| Identity မှန်သော်လည်း route scope မရှိ | `403 forbidden` |
| JWKS provider timeout သို့မဟုတ် temporary fetch failure | `503 authentication_unavailable` |
| Deployment configuration မှား သို့မဟုတ် JWKS document အသုံးမပြုနိုင် | `500 authentication_unavailable` နှင့် deployment alert |

Provider-specific parsing detail ကို client သို့ မပြန်ရပါ။ Raw `Authorization` header၊ access token၊ personal-data ပါသော claims သို့မဟုတ် credential ပါသော JWKS URL ကို log မလုပ်ရပါ။ ရှိပြီးသား sensitive-header middleware သည် trace output ထဲ authorization header ကို ဆက်လက် redact လုပ်ပါသည်။

## Key rotation runbook

Key အသစ်၏ `kid` ဖြင့် public JWK အသစ်ကို publish လုပ်ပြီး ယခင် public JWK ကို ဆက်ထားပါ။ JWKS endpoint သည် key နှစ်ခုလုံး serve လုပ်ပြီးမှ key အသစ်ဖြင့် token ထုတ်ပါ။ Old token ၏ maximum lifetime အပြင် configured cache TTL နှင့် clock skew အထိ key နှစ်ခုလုံးထားပါ။ Old token များ valid မဖြစ်နိုင်တော့မှ old key ကိုဖယ်ပါ။ Unknown `kid` ရောက်လျှင် Zap သည် bounded refresh တစ်ကြိမ်လုပ်မည်။ Refresh ထပ်ခါထပ်ခါမဖြစ်အောင် throttle လုပ်ပြီး cache expire ပြီးနောက် stale key ကို လက်မခံပါ။

Staging တွင် rotation စမ်းသပ်ရာ၌ old-key token ထုတ်၊ new JWK ထည့်၊ new-key token ထုတ်၊ overlap အတွင်း token နှစ်မျိုးလုံး verify လုပ်ပြီး expiry ပြီးမှ old key ဖယ်ပါ။ Algorithm-confusion token၊ issuer မှား token နှင့် API သို့ ID token ပို့ခြင်းတို့ကို reject လုပ်ကြောင်းလည်း စမ်းသပ်ရမည်။

## Deployment checklist

`scripts/validate_zap_host_deployment.sh` နှင့် `scripts/validate_zap_web_deployment.sh` ကို run ပါ။ Managed environment တွင် `ZAP_AUTH_MODE=jwt` ပါကြောင်း၊ JWKS URL သည် HTTPS ဖြစ်ကြောင်း၊ deployment policy တွင် reviewed algorithm list သာ ပါကြောင်းနှင့် demo authentication ပိတ်ထားကြောင်း စစ်ပါ။ `/health` နှင့် `/ready` ကို သီးခြား verify လုပ်ပါ။ Readiness သည် real repository နှင့် identity-provider dependency policy ကို ထည့်သွင်းစဉ်းစားနိုင်သော်လည်း transient JWKS request ကြောင့် liveness ကို မပျက်စေရပါ။

## ကိုးကားချက်များ

[1]: https://www.rfc-editor.org/rfc/rfc9700 RFC 9700 — Best Current Practice for OAuth 2.0 Security.
[2]: https://www.rfc-editor.org/rfc/rfc8725 RFC 8725 — JSON Web Token Best Current Practices.
[3]: https://openid.net/specs/openid-connect-core-1_0.html OpenID Connect Core 1.0.
