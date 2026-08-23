# zap-host Axum/Tower Adapter

**အတည်ပြုထားသော baseline:** Zap v2.2.7
**Branch:** `Framework`
**အခြေအနေ:** `Framework` branch ၏ `host/zap-host` အောက်တွင် အကောင်အထည်ဖော်ထားသော adapter foundation v0.1 ဖြစ်သည်။

ဤလမ်းညွှန်သည် [`frameworks/web`](../frameworks/web) အောက်ရှိ dependency-free Web contracts များအတွက် ပထမဆုံး host-side HTTP adapter ကို သတ်မှတ်ထားပါသည်။ HTTP routing အတွက် Axum နှင့် bounded request handling၊ timeout၊ sensitive-header marking နှင့် request tracing အတွက် Tower/Tower-HTTP ကို အသုံးပြုထားပါသည်။ Adapter ကို Zap language core နှင့် သီးခြားထားပြီး native runtime သည် embeddable Rust library ဖြစ်နေပြီဟု မဆိုပါ။

> **Boundary:** Zap Web modules များသည် deterministic request၊ DTO၊ authorization၊ rate-limit၊ repository နှင့် response contracts များကို သတ်မှတ်သည်။ `zap-host` သည် socket၊ HTTP extraction၊ middleware အစီအစဉ်၊ process lifecycle နှင့် HTTP တန်ဖိုးများကို contract များအဖြစ် ပြောင်းလဲခြင်းကို ပိုင်ဆိုင်သည်။

## ဤ foundation တွင် ပါဝင်သည့်အရာများ

ပထမဆုံး crate သည် production deployment အပြည့်အစုံမဟုတ်ဘဲ runnable adapter skeleton ဖြစ်သည်။ Real Axum router နှင့် Tokio TCP lifecycle ပါဝင်သော်လည်း default gateway သည် in-memory demonstration repository ဖြစ်ပြီး default authenticator သည် သတ်မှတ်ထားသော demo identity ကို လက်ခံသည်။ ဤ default များသည် contract ကို executable/testable ဖြစ်စေရန်သာ ဖြစ်ပြီး deployment မလုပ်မီ အစားထိုးရမည်။

| နယ်ပယ် | `zap-host` တွင် ပါဝင်ပြီးသည် | Production တွင် အစားထိုး/ထပ်ဖြည့်ရမည့်အရာ |
|---|---|---|
| HTTP listener | Tokio `TcpListener` နှင့် `axum::serve` | TLS termination၊ proxy policy၊ deployment health နှင့် socket hardening |
| Routing | `/`၊ `/health`၊ `/api/users`၊ `/api/users/:id` | Versioning၊ API compatibility policy နှင့် application routes အပြည့်အစုံ |
| Request bounds | Path 2,048 bytes၊ body 65,536 bytes၊ request ID 128 bytes | Edge/proxy limits၊ multipart policy၊ decompression limits နှင့် per-route budgets |
| Timeout | Tower `TimeoutLayer`၊ default 10 seconds | Operation-specific deadlines၊ cancellation propagation နှင့် downstream budgets |
| Authentication | `Authenticator` trait နှင့် identity extension | JWT/OIDC/API-key verification၊ key rotation၊ issuer/audience စစ်ဆေးမှုနှင့် revocation policy |
| Authorization | `users:read` နှင့် `users:write` scope စစ်ဆေးမှု | Resource ownership၊ tenant isolation၊ policy evaluation နှင့် audit events |
| Database | `UserRepository` trait နှင့် memory demo | Parameterized SQL/driver adapter၊ pool/transaction policy၊ migrations နှင့် retry taxonomy |
| Rate limiting | In-process fixed-window state | Shared atomic store၊ trusted key policy၊ proxy-aware identity နှင့် failure policy |
| Shutdown | Ctrl-C/SIGTERM graceful shutdown | Readiness transition၊ deployment drain နှင့် bounded shutdown timeout |

## Request pipeline

Adapter သည် domain gateway ကို မခေါ်မီ boundary များကို အရင်အသုံးပြုသည်။ ထိုအကြောင်းကြောင့် oversized၊ unsupported၊ unauthenticated သို့မဟုတ် rate-exhausted request များသည် repository code သို့ မရောက်နိုင်ပါ။

| အဆင့် | တာဝန် | Failure ဥပမာ |
|---|---|---|
| 1. Request policy | Path shape၊ traversal marker၊ method နှင့် request ID ကို စစ်ဆေးပြီး correlation ID attach/generate လုပ်ခြင်း | `400 invalid_request`၊ `405 method_not_allowed` |
| 2. Tower bounds | Body size နှင့် request တစ်ခုလုံး timeout ကို enforce လုပ်ခြင်း | `400 invalid_request`၊ `408 request_timeout` |
| 3. Rate limiter | Gateway မခေါ်မီ configured key/window ကို atomic အဖြစ် consume လုပ်ခြင်း | `429 rate_limited`၊ `500 rate_limit_unavailable` |
| 4. Authenticator | External credential ကို Zap အပြင်တွင် verify လုပ်ပြီး typed identity ပြန်ပေးခြင်း | `401 unauthenticated`၊ `500 authentication_unavailable` |
| 5. Route handler | Required scope၊ path/body parse နှင့် gateway call ကို ကိုင်တွယ်ခြင်း | `400`၊ `403`၊ `404`၊ `409`၊ `503` |
| 6. DTO mapper | Input normalize လုပ်ပြီး public output fields သာ ပြန်ပေးခြင်း | `400 invalid_name`၊ `400 invalid_email` |
| 7. Response boundary | JSON serialize၊ security headers နှင့် `x-request-id` ပြန်ပေးခြင်း | Stable/redacted response |

နောက်ဆုံးထည့်ထားသော middleware သည် Tower တွင် outermost layer ဖြစ်သည်။ Code သည် rate limit ကို authentication မတိုင်မီနှင့် နှစ်ခုစလုံးကို gateway မတိုင်မီ ထားရှိသည်။ ၎င်းသည် မတော်တဆဖြစ်လာသော order မဟုတ်ဘဲ ရည်ရွယ်ထားသော policy ဖြစ်သည်။ Deployment တစ်ခုက order ပြောင်းလိုပါက side-effect နှင့် failure ordering ကို သက်သေပြသော test ထည့်ရမည်။

## Crate ဖွဲ့စည်းပုံ

| Path | တာဝန် |
|---|---|
| [`host/zap-host/Cargo.toml`](../host/zap-host/Cargo.toml) | Rust package နှင့် Axum/Tower feature selection |
| [`host/zap-host/src/lib.rs`](../host/zap-host/src/lib.rs) | Configuration၊ state၊ middleware၊ routes၊ DTOs၊ gateway traits နှင့် demo repository |
| [`host/zap-host/src/main.rs`](../host/zap-host/src/main.rs) | Environment loading၊ logging၊ listener binding နှင့် graceful shutdown |
| [`host/zap-host/tests/http_contract.rs`](../host/zap-host/tests/http_contract.rs) | Axum service tests နှင့် security/reliability behavior |
| [`host/zap-host/Cargo.lock`](../host/zap-host/Cargo.lock) | Reproducible dependency resolution |

## Integration seams

Adapter သည် အစားထိုးနိုင်သော `WebGateway`၊ `UserRepository`၊ `Authenticator` နှင့် `ReadinessProbe` seams များအပေါ် တည်ဆောက်ထားသည်။ `WebGateway` သည် Axum handler အသုံးပြုသည့် application-facing seam ဖြစ်သည်။ `ContractGateway<R>` သည် `UserRepository` row ကို public DTO အဖြစ် map လုပ်ပြီး database failure များကို stable gateway errors အဖြစ် ပြောင်းသည်။ `Authenticator` သည် HTTP request ကို လက်ခံသော်လည်း verify လုပ်ပြီးသား `Identity` သာ ပြန်ပေးရမည်။ `ReadinessProbe` သည် database သို့မဟုတ် provider တစ်ခုကို host crate ထဲ မချည်ဘဲ dependency-aware readiness ကို ပေးသည်။ Raw credential ကို Zap contract သို့မဟုတ် application logs ထဲ မထည့်ရ။

Real application တစ်ခုသည် အောက်ပါပုံစံရှိ `AppState` ကို ပေးနိုင်သည်။

```rust
let repository = Arc::new(MySqlUserRepository::connect(pool));
let gateway: Arc<dyn WebGateway> = Arc::new(ContractGateway::new(repository));
let authenticator: Arc<dyn Authenticator> = Arc::new(OidcAuthenticator::new(issuer_config));
let state = AppState::new(config, gateway, authenticator)?;
let app = build_router(state);
```

`MemoryRepository` သည် adapter ကို run လို့ရစေရန် ထည့်ထားသော demo သာ ဖြစ်သည်။ ၎င်းကို database integration၊ durability guarantee၊ concurrency design သို့မဟုတ် migration strategy ဟု မယူဆရ။ လက်ရှိ native runtime သည် binary crate ဖြစ်သောကြောင့် reviewed library/embedding seam ရှိပြီးမှ `ZapGateway` ကို ထည့်သွင်းသင့်သည်။ Request တိုင်းအတွက် unbounded CLI subprocess ခေါ်ခြင်းသည် production bridge မဟုတ်ပါ။

## Configuration

Executable သည် အောက်ပါ environment variables များကို ဖတ်သည်။ Numeric value မမှန်ခြင်း သို့မဟုတ် unsafe bound ဖြစ်ခြင်းများကို startup အချိန်တွင် fail လုပ်ပြီး မသိမသာ လက်ခံမထားပါ။

| Variable | Default | စည်းမျဉ်း |
|---|---:|---|
| `ZAP_HOST_ADDR` | `127.0.0.1:3000` | Socket address အဖြစ် parse ရမည် |
| `ZAP_HOST_MAX_BODY_BYTES` | `65536` | 1 နှင့် 65,536 ကြား ဖြစ်ရမည် |
| `ZAP_HOST_REQUEST_TIMEOUT_MS` | `10000` | Zero ထက် ကြီးရမည် |
| `ZAP_HOST_SHUTDOWN_TIMEOUT_MS` | `30000` | Signal နောက်ပိုင်း drain အများဆုံးကြာချိန်; zero မဖြစ်ရ |
| `ZAP_HOST_RATE_LIMIT` | `60` | Fixed window အတွင်း requests; zero မဖြစ်ရ |
| `ZAP_HOST_RATE_WINDOW_MS` | `60000` | Fixed-window duration; zero မဖြစ်ရ |
| `ZAP_HOST_RATE_KEY` | `demo-host` | 1–256 bytes; trusted user/tenant policy ဖြင့် အစားထိုးရမည် |

Default bind address သည် demo adapter ကို မတော်တဆ public expose မဖြစ်စေရန် loopback ဖြစ်သည်။ Public bind လုပ်မည့် deployment သည် network policy၊ TLS termination၊ proxy trust၊ access logging နှင့် readiness behavior ကို explicit configure လုပ်ရမည်။

## HTTP contract

| Method | Path | Required scope | Success | မှတ်ချက် |
|---|---|---|---:|---|
| `GET` | `/` | None | `200` | Root response နှင့် correlation ID |
| `GET` | `/health` | None | `200` | Liveness-style response သာ; database readiness မသက်သေပြ |
| `GET` | `/ready` | None | `200`/`503` | Readiness probe result; public နှင့် dependency-aware |
| `GET` | `/api/users` | `users:read` | `200` | Public DTO list |
| `GET` | `/api/users/:id` | `users:read` | `200` | User မရှိလျှင် `404` |
| `POST` | `/api/users` | `users:write` | `201` | String `name` နှင့် `email` ပါသော JSON body |

JSON response အားလုံးတွင် `x-content-type-options: nosniff`၊ `cache-control: no-store` နှင့် validate/generate လုပ်ထားသော `x-request-id` ပါသည်။ Authorization နှင့် cookie headers များကို Tower diagnostics အတွက် sensitive အဖြစ် mark လုပ်ထားသည်။ Error response များတွင် stable error code သာ ပါပြီး driver message၊ SQL၊ credential၊ token သို့မဟုတ် internal row field များ မပါပါ။

`GET /health` သည် public ဖြစ်ပြီး lightweight response သာ ပြန်သည်။ `GET /ready` သည် public ဖြစ်ပြီး injected readiness probe ကို ခေါ်ကာ dependency မ ready ဖြစ်လျှင် `503` ပြန်သည်။ Readiness နှင့် liveness ကို သီးခြားထားရမည်ဖြစ်ပြီး deployment မတိုင်မီ real dependency checks ဖြင့် ချိတ်ဆက်ရမည်။

## Database adapter checklist

Real `UserRepository` implementation သည် parameterized statements နှင့် typed input binding ကို သုံးရမည်။ Connection-pool size၊ acquisition timeout၊ query timeout၊ transaction boundary၊ cancellation behavior၊ duplicate-key classification၊ unavailable-service classification နှင့် graceful pool shutdown ကို adapter က ပိုင်ဆိုင်ရမည်။ Unavailable dependency ကို `503` နှင့် duplicate create ကို `409` အဖြစ် map လုပ်ရမည်ဖြစ်ပြီး provider-specific text ကို client ထံ မပြန်ရ။

Repository သည် `PublicUser` အတွက် လိုအပ်သည့် fields များသာ ပြန်ပေးရမည်။ Secret columns၊ password material၊ access tokens၊ internal status fields နှင့် diagnostic metadata များကို DTO mapper ဖြင့် serialize မလုပ်ရ။ Subject/tenant binding ကို request body တစ်ခုတည်းကို ယုံကြည်မထားဘဲ repository query ထဲတွင် enforce လုပ်ရမည်။

## Authentication နှင့် authorization checklist

Real authenticator သည် approved issuer၊ audience၊ algorithm၊ key rotation၊ expiry နှင့် revocation policy များဖြင့် host boundary တွင် credential ကို validate လုပ်ရမည်။ Handler သည် request extension မှ verified identity နှင့် scopes ကိုသာ ရယူရမည်။ Zap module ထဲ bearer token parse မလုပ်ရ၊ raw `Authorization` header log မလုပ်ရ၊ explicit proxy-trust configuration မရှိဘဲ forwarded identity ကို မယုံရ။

Authentication နှင့် authorization သည် သီးခြားဆုံးဖြတ်ချက်များ ဖြစ်သည်။ ဥပမာသည် scope စစ်ဆေးမှုသာ လုပ်သော်လည်း production code တွင် resource ownership၊ tenant boundary၊ administrative exception၊ audit event နှင့် default-deny behavior ကို သတ်မှတ်ရမည်။ `401` သည် valid identity မတည်ဆောက်နိုင်ခြင်း ဖြစ်ပြီး `403` သည် identity ရှိသော်လည်း ခွင့်မပြုခြင်း ဖြစ်သည်။

## Rate-limit checklist

Sample fixed-window store သည် state update ကို lock လုပ်ထားသောကြောင့် single process အတွင်း counter oversubscription မဖြစ်စေပါ။ Process တစ်ခုထက်ပိုသော production deployment တွင် key တစ်ခုတည်းကို check-and-increment atomic ပြုလုပ်ပေးနိုင်သော shared store operation သုံးရမည်။ Key ကို verified subject + tenant + route class ကဲ့သို့ trusted policy ဖြင့် ဖန်တီးရမည်။ Arbitrary client header ကို မျက်စိမှိတ်ယုံကြည်မထားရ။

Production policy တွင် store failure တွင် fail-open/fail-closed ဘယ်လိုလုပ်မည်၊ `Retry-After` ဘယ်လိုတွက်မည်၊ monotonicity ကို ဘယ်လိုအာမခံမည်၊ route/identity အလိုက် limit မည်သို့ကွာမည်နှင့် rolling deploy တွင် state ကို ဘယ်လို share မည်တို့ကို မှတ်တမ်းတင်ရမည်။ Local mutex သည် distributed quota solution မဟုတ်ပါ။

## Lifecycle နှင့် shutdown

`main.rs` သည် Tokio TCP listener bind လုပ်ပြီး Ctrl-C/SIGTERM အတွက် Axum graceful-shutdown future ကို အသုံးပြုသည်။ Lifecycle state သည် host ကို draining အဖြစ် mark လုပ်ပြီး `/ready` ကို deployment readiness နှင့် ချိတ်နိုင်သည်။ Configured shutdown timeout သည် signal နောက်ပိုင်း drain ကို bounded လုပ်ပေးပြီး Tower request timeout သည် request တစ်ခုချင်းစီ အဆုံးမဲ့မစောင့်စေရန် ကူညီသည်။ Production supervisor သည် termination မတိုင်မီ readiness remove၊ connection drain limit၊ downstream cancellation၊ bounded database-pool close နှင့် exit-status policy ကို ထပ်သတ်မှတ်ရမည်။

ပထမ crate တွင် TLS မပါသေးပါ။ ပုံမှန်အားဖြင့် controlled edge တွင် TLS terminate လုပ်ခြင်း သို့မဟုတ် reviewed host deployment configuration ဖြင့် ထည့်သွင်းခြင်းကို ရွေးချယ်ရမည်။ HTTP/2၊ HTTP/3၊ WebSocket၊ compression၊ CORS၊ proxy headers နှင့် tracing exporters တစ်ခုချင်းစီတွင် explicit policy နှင့် regression tests လိုအပ်သည်။

အဆင့်လိုက် local workflow ကို [`ZAP_HOST_QUICKSTART_MM.md`](ZAP_HOST_QUICKSTART_MM.md) သို့မဟုတ် [`ZAP_HOST_QUICKSTART_EN.md`](ZAP_HOST_QUICKSTART_EN.md) တွင် ဖတ်နိုင်သည်။

## Development နှင့် validation

Repository root မှ run လုပ်ရန်:

```bash
cd host/zap-host
cargo check --all-targets
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo run
```

Integration suite သည် public health၊ request-ID propagation၊ DTO mapping၊ authentication/scope failures၊ path/method/body rejection၊ rate-limit short-circuiting နှင့် database error mapping များကို စစ်ဆေးသည်။ In-process Axum service interface ကို သုံးသောကြောင့် live socket၊ database၊ credential provider သို့မဟုတ် external service မလိုပါ။

## Production မတိုင်မီ ကျန်ရှိသည့် milestone

ဤ crate သည် ပထမဆုံး adapter prototype ဖြစ်သည်။ Production မတိုင်မီ reviewed runtime bridge၊ real authentication provider၊ real database adapter၊ shared rate-limit store၊ TLS/proxy policy၊ observability/redaction review၊ readiness checks၊ injected dependencies ပါသော integration tests နှင့် deployment-specific load/chaos evidence များကို ထပ်ထည့်ရမည်။ Demo memory repository သို့မဟုတ် fixed authenticator ကို production default အဖြစ် မတင်ရ။

## ကိုးကားချက်များ

[1]: https://docs.rs/axum/latest/axum/ "Axum documentation"
[2]: https://docs.rs/tower-http/latest/tower_http/ "Tower-HTTP middleware documentation"
[3]: https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs "Axum graceful-shutdown example"
