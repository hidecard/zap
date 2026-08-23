# zap-host စတင်အသုံးပြုခြင်းနှင့် Integration လမ်းညွှန်

**အတည်ပြုထားသော baseline:** Zap v2.2.7
**Branch:** `Framework`
**Adapter:** `host/zap-host`

ဤလမ်းညွှန်သည် ပထမဆုံး Axum/Tower host adapter ကို လက်တွေ့စတင်အသုံးပြုရန် entry point ဖြစ်သည်။ Dependency-free Web contract demo ကို run ရန်၊ HTTP boundary ကို test ရန်၊ demo repository နှင့် authenticator ကို အစားထိုးရန်၊ production deployment မတိုင်မီ ပြီးစီးရမည့်အချက်များကို ရှင်းပြထားသည်။ English companion guide ကို [`ZAP_HOST_QUICKSTART_EN.md`](ZAP_HOST_QUICKSTART_EN.md) တွင် ဖတ်နိုင်သည်။

> **အရေးကြီးသည်:** Default executable သည် contract demonstration ဖြစ်သည်။ In-memory repository နှင့် fixed demo authenticator ပါဝင်သည်။ Local development နှင့် adapter tests အတွက် သင့်တော်သော်လည်း production database၊ credential verifier၊ distributed rate limiter သို့မဟုတ် production deployment platform မဟုတ်ပါ။

## ၁။ လိုအပ်ချက်များ

Crate ၏ pinned compatibility line နှင့် ကိုက်ညီသော Rust toolchain လိုအပ်သည်။ လက်ရှိ crate ကို Rust 1.75 ဖြင့် စစ်ဆေးထားပြီး Axum 0.7/Tower-HTTP 0.5 ကို အသုံးပြုထားသည်။ နောက်ပိုင်း compatible stable toolchain သည် အလုပ်လုပ်နိုင်သော်လည်း CI သည် support ပြုထားသော version အတွက် အဓိကအတည်ပြုချက် ဖြစ်သည်။

| လိုအပ်ချက် | လိုအပ်ရသည့်အကြောင်း | စစ်ဆေးရန် |
|---|---|---|
| Rust/Cargo | Host adapter build/run လုပ်ရန် | `rustc --version`, `cargo --version` |
| Local TCP port လွတ်နေခြင်း | Demo listener run ရန် | Default `127.0.0.1:3000` |
| Zap repository | `frameworks/web` contract နှင့် host code ရယူရန် | Repository root တွင် `git status` |
| Database/identity provider မလို | Demo သည် local deterministic doubles သုံးသည် | Production မတိုင်မီ အစားထိုးရမည် |

Repository မရှိသေးလျှင် clone လုပ်ပြီး Framework branch သို့ ပြောင်းပါ။

```bash
git clone https://github.com/hidecard/zap.git
cd zap
git switch Framework
```

Repository ရှိပြီးသားဖြစ်ပါက `git pull --ff-only origin Framework` လုပ်ပြီး `git branch --show-current` ဖြင့် branch ကို စစ်ပါ။

## ၂။ ငါးမိနစ်အတွင်း Demo run လုပ်ခြင်း

Repository root မှ local environment file ပြင်ဆင်ပါ။ Example ထဲတွင် non-secret demo values များသာ ပါသည်။ Password၊ signing key၊ database URL သို့မဟုတ် bearer token များကို example သို့မဟုတ် commit လုပ်ထားသော file ထဲ မထည့်ရ။

```bash
cd host/zap-host
cp .env.example .env.local
set -a
. ./.env.local
set +a
cargo run
```

Process သည် `127.0.0.1:3000` တွင် bind လုပ်ပြီး listening log ပြမည်။ ထို terminal ကို ဖွင့်ထားကာ အခြား terminal မှ request များ run ပါ။

```bash
curl -i http://127.0.0.1:3000/
curl -i http://127.0.0.1:3000/health
curl -i http://127.0.0.1:3000/ready
curl -i -H 'x-request-id: quickstart-get' \
  http://127.0.0.1:3000/api/users/1
curl -i -H 'x-request-id: quickstart-list' \
  http://127.0.0.1:3000/api/users
curl -i -H 'content-type: application/json' \
  -H 'x-request-id: quickstart-create' \
  -d '{"name":"Bob","email":"bob@example.com"}' \
  http://127.0.0.1:3000/api/users
```

Root၊ health နှင့် readiness routes များသည် public ဖြစ်သည်။ Health သည် lightweight liveness response ဖြစ်ပြီး readiness သည် injected dependency probe ကို ခေါ်သည်။ Demo authenticator သည် fixed identity ကို ပေးသောကြောင့် local တွင် user routes များကို စမ်းနိုင်သည်။ Create request အောင်မြင်လျှင် `201 Created`၊ read/list အောင်မြင်လျှင် `200 OK` ပြန်မည်။ Response တွင် public user DTO နှင့် request ID ပါမည်။ Process ကို `Ctrl-C` ဖြင့် ရပ်နိုင်ပြီး executable သည် Ctrl-C နှင့် SIGTERM ကို လက်ခံကာ Axum graceful shutdown ပြုလုပ်သည်။

Port သို့မဟုတ် local limit ပြောင်းရန် `cargo run` မတိုင်မီ environment variables export လုပ်ပါ။

```bash
ZAP_HOST_ADDR=127.0.0.1:3100 \
ZAP_HOST_MAX_BODY_BYTES=32768 \
ZAP_HOST_REQUEST_TIMEOUT_MS=5000 \
ZAP_HOST_SHUTDOWN_TIMEOUT_MS=30000 \
RUST_LOG=zap_host=debug,tower_http=debug \
cargo run
```

Port အသုံးပြုပြီးသားဖြစ်လျှင် loopback port အသစ်ရွေးပါ။ Public interface သို့ demo ကို expose လုပ်ခြင်းဖြင့် local port ပြဿနာကို မဖြေရှင်းရ။

## ၃။ Request flow ကို နားလည်ခြင်း

Request တိုင်းသည် gateway သို့ မရောက်မီ bounded host policy များကို ဖြတ်ရသည်။ ဤအစီအစဉ်သည် invalid သို့မဟုတ် reject ဖြစ်သော request များကြောင့် repository side effect မဖြစ်စေရန် ဖြစ်သည်။

| အစီအစဉ် | Boundary | တွေ့မြင်ရမည့်အရာ |
|---:|---|---|
| 1 | Request ID နှင့် path policy | ID ပေးထားလျှင် ထိန်းသိမ်း၊ မပေးလျှင် generate; traversal/oversized ID reject |
| 2 | Method policy | မထောက်ပံ့သော method သည် `405` ပြန် |
| 3 | Body နှင့် timeout layers | Oversized body သည် bounded error; hanging request သည် အဆုံးမဲ့မစောင့် |
| 4 | Fixed-window rate gate | Quota ပြည့်လျှင် gateway မခေါ်ဘဲ `Retry-After` နှင့် `429` |
| 5 | Authentication | Demo identity ကို local ပေး; production credential ကို Zap အပြင်တွင် verify |
| 6 | Scope authorization | `users:read`/`users:write` မရှိလျှင် `403` |
| 7 | DTO validation | JSON media type၊ name/email type၊ length၊ trim နှင့် normalization |
| 8 | Repository/gateway | Typed operation မှ public DTO သို့မဟုတ် stable error ပြန် |
| 9 | Response boundary | JSON၊ security headers နှင့် `x-request-id`; internal field မပါ |

## ၄။ Failure နှင့် security path များ စမ်းခြင်း

အောက်ပါ request များဖြင့် unsafe input ကို adapter က မျက်စိမှိတ်လက်မခံကြောင်း စစ်ပါ။

```bash
# Traversal marker: 400 invalid_request မျှော်လင့်ပါ။
curl -i -H 'x-request-id: traversal-check' \
  http://127.0.0.1:3000/api/../users

# မထောက်ပံ့သော method: 405 method_not_allowed မျှော်လင့်ပါ။
curl -i -X DELETE -H 'x-request-id: method-check' \
  http://127.0.0.1:3000/health

# မထောက်ပံ့သော media type: 415 unsupported_media_type မျှော်လင့်ပါ။
curl -i -H 'content-type: text/plain' \
  -H 'x-request-id: media-check' \
  --data '{"name":"A","email":"a@b"}' \
  http://127.0.0.1:3000/api/users

# မမှန်သော DTO: stable validation code ပါသော 400 မျှော်လင့်ပါ။
curl -i -H 'content-type: application/json' \
  -H 'x-request-id: dto-check' \
  --data '{"name":"","email":"not-an-email"}' \
  http://127.0.0.1:3000/api/users
```

Client များသည် provider-specific text မဟုတ်ဘဲ stable status နှင့် error code ပေါ်တွင်သာ branch လုပ်သင့်သည်။ Adapter သည် authorization နှင့် cookie headers များကို diagnostics အတွက် sensitive အဖြစ် mark လုပ်ပြီး raw credential ကို error body ထဲ မပြန်ပါ။

## ၅။ Quality gates run လုပ်ခြင်း

`host/zap-host` directory မှ host checks များ run ပါ။

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check --all-targets --all-features
cargo test --all-targets
```

လက်ရှိ test suite တွင် configuration နှင့် fixed-window behavior unit tests၊ health၊ request ID၊ DTO mapping၊ authentication၊ scope failure၊ invalid route၊ body/media limit၊ rate-limit short-circuit နှင့် database error mapping အတွက် in-process Axum integration tests များ ပါရှိသည်။ Live database သို့မဟုတ် external identity provider မလိုပါ။

Zap-side contract ကို repository root မှ သီးခြားစစ်ပါ။

```bash
cd frameworks/web
zap lock
zap check
zap build
zap run main.zp
zap test .
```

Zap-side package သည် dependency-free အတိုင်း ရှိရမည်။ Host crate သည် standalone Rust package ဖြစ်သောကြောင့် `frameworks/web/zap.toml` ထဲ Axum/Tower dependency မထည့်ရ။

## ၆။ Demo repository ကို အစားထိုးခြင်း

Application-facing database seam သည် `UserRepository` ဖြစ်သည်။ Production implementation သည် driver၊ pool၊ parameterized statements၊ transaction boundary၊ deadline၊ cancellation၊ duplicate-key classification၊ unavailable-service classification နှင့် graceful shutdown ကို ပိုင်ဆိုင်ရမည်။

Integration shape သည် အောက်ပါအတိုင်း ဖြစ်နိုင်သည်။

```rust
let repository = Arc::new(MySqlUserRepository::connect(pool));
let gateway: Arc<dyn WebGateway> = Arc::new(ContractGateway::new(repository));
let authenticator: Arc<dyn Authenticator> = Arc::new(VerifiedIdentityProvider::new(config));
let state = AppState::new(app_config, gateway, authenticator)?;
let router = build_router(state);
```

Production type များသည် application အလိုက် ကွာနိုင်သည်။ အဓိကစည်းမျဉ်းမှာ repository သည် typed row ပြန်ပြီး `ContractGateway` သို့မဟုတ် equivalent mapper သည် `PublicUser { id, name, email }` သာ expose လုပ်ရမည် ဖြစ်သည်။ Password material၊ token၊ secret column၊ internal status နှင့် database diagnostics များကို DTO boundary ဖြတ်မသွားစေရ။

Database outage ဖြစ်လျှင် `503` သို့ map လုပ်ပြီး request ID ပါသော redacted structured event ကို log လုပ်ပါ။ SQL၊ connection string၊ driver message သို့မဟုတ် query parameter များကို client ထံ မပြန်ပါနှင့်။ Duplicate create အတွက် stable `409` policy သုံးပါ။ Subject သို့မဟုတ် tenant ownership ကို request body တစ်ခုတည်းအပေါ် မယုံဘဲ database query နှင့် authorization policy ထဲ enforce လုပ်ပါ။

## ၇။ Demo authenticator ကို အစားထိုးခြင်း

`Authenticator` trait သည် identity verification အတွက် host boundary ဖြစ်သည်။ Production implementation သည် documented issuer၊ audience၊ algorithm၊ expiry၊ key rotation နှင့် revocation policy ဖြင့် credential ကို validate လုပ်ရမည်။ Verified identity နှင့် scopes ကို request extensions မှတစ်ဆင့် ပေးရမည်။

Raw `Authorization` header၊ cookie၊ API key၊ password သို့မဟုတ် bearer token ကို Zap contract ထဲ မပေးရ၊ DTO ထဲ မထည့်ရ၊ logs ထဲ မရေးရ။ Proxy trust boundary နှင့် header-stripping policy explicit မရှိဘဲ proxy forwarded identity ကို မယုံရ။ Authentication နှင့် authorization ကို ခွဲထားပါ။

| ရလဒ် | အဓိပ္ပာယ် | HTTP status |
|---|---|---:|
| Valid identity မရှိ | Authentication က principal မတည်ဆောက်နိုင် | `401` |
| Identity ရှိသော်လည်း scope/ownership မရှိ | Authorization deny | `403` |
| Provider မရနိုင် သို့မဟုတ် policy မလုပ်နိုင် | Dependency/policy failure | `500` သို့မဟုတ် documented fail-closed status |

Demo authenticator သည် local request များ run လို့ရရန် permissive ဖြစ်သည်။ Public bind address မသုံးမီ အစားထိုးရမည်။

## ၈။ Local rate limiter ကို အစားထိုးခြင်း

လက်ရှိ fixed-window store သည် state ကို synchronize လုပ်၍ single process အတွင်း counter oversubscription မဖြစ်စေပါ။ Process/instance များသော deployment တွင် key တစ်ခုတည်းကို check-and-increment atomic လုပ်သော shared store လိုအပ်သည်။

Production မတိုင်မီ အောက်ပါ policy များကို ဆုံးဖြတ်၍ document လုပ်ပါ။

| ဆုံးဖြတ်ချက် | သတ်မှတ်ရမည့်အရာ |
|---|---|
| Key | Verified subject၊ tenant၊ client class၊ route class; arbitrary untrusted header မသုံးရ |
| Store failure | Fail-open/fail-closed နှင့် alerting |
| Window | Fixed/sliding/token-bucket semantics နှင့် reset behavior |
| Retry | `Retry-After` တွက်ချက်မှုနှင့် clock source |
| Scope | Anonymous၊ authenticated၊ admin နှင့် expensive routes အလိုက် quota |
| Rollout | Rolling deployment နှင့် failover အတွင်း state share လုပ်ပုံ |

Rate gate ကို repository access မတိုင်မီ ထားရမည်။ Local mutex သည် local oversubscription ကိုသာ ကာကွယ်ပြီး cross-process atomicity မပေးနိုင်ပါ။

## ၉။ Deployment ပြင်ဆင်ခြင်း

Local development တွင် `ZAP_HOST_ADDR=127.0.0.1:3000` ကို ဆက်ထားပါ။ Public deployment သည် address ပြောင်းရုံဖြင့် ပြီးစီးသည်ဟု မယူဆရ။ Repository တွင် host operational boundary အတွက် reference artifacts များ ပါဝင်သည်: [`deploy/zap-host.service`](../deploy/zap-host.service)၊ [`deploy/zap-host.nginx.conf`](../deploy/zap-host.nginx.conf)၊ [`deploy/zap-host-deployment-policy.toml`](../deploy/zap-host-deployment-policy.toml)၊ [`deploy/zap-host.env.example`](../deploy/zap-host.env.example) နှင့် [`scripts/validate_zap_host_deployment.sh`](../scripts/validate_zap_host_deployment.sh)။ ၎င်းတို့သည် template နှင့် validation evidence များသာဖြစ်ပြီး deployment-specific review အစား မသုံးရ။ `0.0.0.0` သို့ bind မလုပ်မီ အောက်ပါ boundary work များကို ပြီးစီးရမည်။

| နယ်ပယ် | အနည်းဆုံး production work |
|---|---|
| TLS | Controlled edge တွင် TLS terminate သို့မဟုတ် reviewed host TLS config |
| Proxy | Trusted proxy headers၊ forwarded identity နှင့် header stripping policy |
| Identity | `DemoAuthenticator` အစား verified credential နှင့် key rotation |
| Database | `MemoryRepository` အစား real driver/pool/transaction adapter |
| Rate limit | Local state အစား shared atomic store |
| Readiness | Liveness နှင့် သီးခြား dependency-aware readiness |
| Shutdown | Termination မတိုင်မီ readiness remove၊ connection drain၊ downstream cancel၊ pool close နှင့် maximum drain time |
| Observability | Credential/sensitive field redaction၊ request ID နှင့် stable error category |
| Resource policy | Route-specific body၊ timeout၊ concurrency၊ connection နှင့် decompression limits |
| Evidence | Integration၊ load၊ failure-injection နှင့် deployment smoke tests |

HTTP/2၊ HTTP/3၊ WebSocket၊ compression၊ CORS၊ multipart upload၊ background job နှင့် static file များကို foundation တွင် မဖွင့်ထားပါ။ Feature တစ်ခုချင်းစီအတွက် explicit host policy နှင့် tests လိုအပ်သည်။

## ၁၀။ Troubleshooting

| ပြဿနာ | ဖြစ်နိုင်သည့်အကြောင်း | လုပ်ဆောင်ရန် |
|---|---|---|
| `Address already in use` | Configured port ကို အခြား process သုံးနေသည် | `ZAP_HOST_ADDR=127.0.0.1:3100` သတ်မှတ်ပြီး retry |
| Startup တွင် variable reject | Number မမှန်၊ timeout zero သို့မဟုတ် unsafe bound | `.env.example` နှင့် configuration table ကို နှိုင်းယှဉ် |
| `401 unauthenticated` | Custom authenticator က verified identity မပြန် | Raw credential မlog ဘဲ host provider boundary စစ် |
| `403 forbidden` | Identity တွင် route scope မရှိ | Host-issued scope နှင့် tenant/ownership policy စစ် |
| `415 unsupported_media_type` | POST body သည် `application/json` မဟုတ် | Content type မှန်ပေး; check မပိတ်ရ |
| `429 rate_limited` | Fixed-window quota ပြည့် | `Retry-After` လိုက်နာ; scale out မလုပ်မီ shared state သုံး |
| `503 database_unavailable` | Repository dependency မရနိုင် | Pool health/redacted telemetry စစ်; driver text မပြန် |
| Health အောင်သော်လည်း user route fail | `/health` သည် liveness-only ဖြစ် | Dependency-aware readiness သီးခြားထည့် |
| Test အောင်သော်လည်း production မလုံခြုံ | Demo doubles များ အစားမထိုးရသေး | Repository၊ authenticator၊ shared limiter၊ TLS၊ readiness စစ် |

## ၁၁။ နောက်ထပ် production milestone ပြီးစီးမှု သတ်မှတ်ချက်

Controlled integration environment သို့ တင်နိုင်ရန် real gateway seam ရှိရမည်၊ repository နှင့် authenticator ကို injection ဖြင့်ပေးရမည်၊ rate policy သည် shared/atomic ဖြစ်ရမည်၊ edge policy document ရှိရမည်၊ deployment tests များက error ordering နှင့် shutdown behavior ကို သက်သေပြရမည်။ `cargo test` အောင်ခြင်းတစ်ခုတည်းသည် database durability၊ credential correctness၊ distributed quota safety သို့မဟုတ် production capacity ကို မသက်သေပြနိုင်ပါ။

## ဆက်စပ် Documentation

- [`ZAP_HOST_MM.md`](ZAP_HOST_MM.md): Architecture၊ middleware၊ lifecycle၊ security နှင့် production boundary အသေးစိတ်။
- [`ZAP_HOST_EN.md`](ZAP_HOST_EN.md): English architecture နှင့် production checklist။
- [`WEB_FRAMEWORK_MM.md`](WEB_FRAMEWORK_MM.md): Dependency-free Web contract နှင့် DTO/database/auth/rate-limit boundary။
- [`../host/zap-host/README.md`](../host/zap-host/README.md): Crate-level command အကျဉ်းချုပ်။

## ကိုးကားချက်များ

[1]: https://docs.rs/axum/latest/axum/ "Axum documentation"
[2]: https://docs.rs/tower-http/latest/tower_http/ "Tower-HTTP documentation"
[3]: https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs "Axum graceful-shutdown example"
