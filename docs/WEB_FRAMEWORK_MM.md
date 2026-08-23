# Zap Web Framework Foundation

**အတည်ပြုထားသော baseline:** Zap v2.2.7
**Branch:** `Framework`

**အခြေအနေ:** Web Foundation v0.2 — run လို့ရသော contract package နှင့် initial `zap-host` adapter prototype ပါဝင်ပြီး production integration များကို သီးခြားထားသည်

## ရည်ရွယ်ချက်

Web package သည် Zap application နှင့် host HTTP implementation ကြားတွင် တည်ငြိမ်သော boundary တစ်ခု သတ်မှတ်ပေးသည်။ Zap ဘက်တွင် bounded request ကို normalize လုပ်ပြီး route/method policy ကို စစ်ဆေးကာ bounded response map ပြန်ပေးသည်။ Listener မဖွင့်ပါ၊ TLS မပိုင်ပါ၊ blocking socket work မလုပ်ပါ။

Host adapter သည် Zap runtime ကို network reactor အသစ်တစ်ခုအဖြစ် ပြန်ရေးမည့်အစား ရှိပြီးသား HTTP stack ကို reuse လုပ်နိုင်ရန် ဒီ contract ကို သတ်မှတ်ထားခြင်း ဖြစ်သည်။ ပထမဆုံး Rust adapter ကို [`host/zap-host`](../host/zap-host) အောက်တွင် ထည့်သွင်းထားပြီး Axum/Tower ကို အသုံးပြုကာ document လုပ်ထားသော DTO boundary ကိုသာ ဘာသာပြန်သည်။[1]

## လက်ရှိ package ဖွဲ့စည်းပုံ

| ဖိုင် | တာဝန် | အခြေအနေ |
|---|---|---|
| `zap.toml` | Dependency-free contract package manifest | ပြီးစီး |
| `zap.lock` | Canonical empty-dependency lockfile | ပြီးစီး |
| `web_contract.zp` | Request, response, security-header နှင့် router functions | ပြီးစီး |
| `main.zp` | Deterministic end-to-end contract demonstration | ပြီးစီး |
| `web_contract_test.zp` | Positive/negative contract regression suite | ပြီးစီး |
| `frontend_contract.zp` | Browser asset manifest၊ HTML/CSS/JS route နှင့် JSON API integration contract | ပြီးစီး |
| `frontend_contract_test.zp` | Asset content-type၊ traversal နှင့် Node-free runtime regression suite | ပြီးစီး |
| `public/index.html` | Browser entrypoint reference | ပြီးစီး |
| `public/assets/app.css` | Reference stylesheet | ပြီးစီး |
| `public/assets/app.js` | JSON API သုံးသော browser ES module | ပြီးစီး |
| `README.md` | Quick-start နှင့် package boundary | ပြီးစီး |

`frameworks/web` directory ထဲမှ run ပါ။

```bash
zap lock
zap check
zap build
zap run main.zp
zap test .
```

Package တွင် external dependency မရှိပါ။ ပထမဆုံး Web milestone သည် registry၊ network နှင့် host-runtime behavior များကို မစစ်ဆေးရသေးဘဲ ဖုံးကွယ်မသွားစေရန် semantics နှင့် safety ကို dependency-free အဖြစ် အရင်သက်သေပြရမည်။

## Request contract

Export လုပ်ထားသော `normalize_request(method, path, body, request_id)` function သည် text value လေးခုကို လက်ခံပြီး map တစ်ခု ပြန်ပေးသည်။ Host adapter သည် ထို map မတည်ဆောက်မီ transport-level limit များကို ကိုယ်တိုင် အရင်စစ်ရမည်။

| Field | စည်းမျဉ်း | Reject behavior |
|---|---|---|
| `method` | Trim နှင့် upper-case ပြုလုပ်ပြီး `GET`, `POST` ကိုသာ လက်ရှိ support လုပ်သည် | မသိသော method ကို contract က HTTP-style `405` ပြန်ပေးသည် |
| `path` | `/` ဖြင့် စရမည်၊ `..` မပါရ၊ လက်ရှိ text-length contract အရ 2,048 bytes ထက် မကျော်ရ | မမှန်သော path ကို `400` ပြန်ပေးသည် |
| `body` | လက်ရှိ text-length contract အရ 65,536 bytes ထက် မကျော်ရ | body ကြီးလွန်းလျှင် `400` ပြန်ပေးသည် |
| `request_id` | ဗလာမဖြစ်ရ၊ 128 bytes ထက် မကျော်ရ | မရှိခြင်း/ကြီးလွန်းခြင်းကို `400` ပြန်ပေးသည် |
| `valid` | Path, body, request ID စစ်ဆေးချက်အားလုံး pass ဖြစ်မှ true | Router သည် route dispatch မလုပ်မီ invalid request ကို ရပ်တန့်သည် |

Query parsing၊ content negotiation၊ cookie parsing၊ multipart upload နှင့် automatic JSON decoding များကို core ထဲတွင် မထည့်သေးပါ။ ထိုအရာများကို limit နှင့် test သီးခြားရှိသော adapter/package အဖြစ်သာ ထည့်ရမည်။

## Response contract

`response(status, body, headers)` သည် stable field လေးခုပါသော map ကို ပြန်ပေးသည်။

```text
{
  "status": number,
  "content_type": "application/json",
  "headers": map<text, text>,
  "body": text
}
```

Starter သည် `x-content-type-options: nosniff` နှင့် `cache-control: no-store` ကို default အဖြစ် ထည့်ပေးသည်။ ဤ defaults များသည် contract demonstration အတွက် conservative policy သာဖြစ်ပြီး production security policy အပြည့်အစုံ မဟုတ်ပါ။ Production adapter တွင် TLS၊ CORS၊ CSP၊ HSTS၊ cookies၊ compression၊ access logging နှင့် cache behavior တို့ကို explicit policy အဖြစ် သတ်မှတ်ရမည်။

## Frontend asset နှင့် JavaScript interoperability

`frontend_contract.zp` သည် Zap ပိုင် browser boundary ဖြစ်ပါသည်။ `frontend_asset_manifest()` က `public` root နှင့် browser runtime အတွက် Node မလိုကြောင်း သတ်မှတ်ပါသည်။ `web_static(asset_path, root_dir)` သည် filesystem capability ဖြင့် ကာကွယ်ထားသော builtin ဖြစ်ပြီး UTF-8 HTML၊ CSS၊ JavaScript/ES module၊ JSON၊ SVG နှင့် text file များကို response map အဖြစ် ပြန်ပေးပါသည်။ Root နှင့် resolved file ကို Zap workspace အတွင်းမှာပဲ ထားပြီး absolute path၊ traversal component၊ encoded traversal၊ backslash၊ မထောက်ပံ့သော extension နှင့် 2 MiB ထက်ကြီးသော file များကို reject လုပ်ပါသည်။ မရှိသော သို့မဟုတ် မထောက်ပံ့သော asset သည် deterministic `404` ပြန်ပြီး unsafe/unreadable path သည် fail closed ဖြစ်ပါသည်။

Route matcher သည် နောက်ဆုံး segment `*name` wildcard ကို support လုပ်သောကြောင့် `/assets/*path` ဖြင့် `/assets/chunks/app.js` ကဲ့သို့ nested bundle path များကို serve လုပ်နိုင်ပါသည်။ Wildcard သည် route parameter သာဖြစ်ပြီး static builtin က canonicalization နှင့် workspace confinement ကို ထပ်စစ်ပါသည်။ Raw file handle သို့မဟုတ် arbitrary SQL/process capability မပေးသည့်အပြင် binary image/font streaming မပါသေးပါ။

HTML၊ CSS နှင့် JavaScript ကို လက်ရေးဖြင့်ရေးနိုင်သကဲ့သို့ optional frontend toolchain ဖြင့်လည်း build လုပ်နိုင်ပါသည်။ React၊ Vue၊ Svelte၊ Alpine သို့မဟုတ် အခြား JavaScript framework များသည် browser output ကို `public/assets/` ထဲသို့ compile လုပ်ပြီး `/api/tasks` ကဲ့သို့ Zap JSON route ကို ခေါ်နိုင်ပါသည်။ Deployed Zap process အတွက် Zap runtime နှင့် browser output files သာ လိုအပ်ပြီး Node သည် **build-time optional tool** ဖြစ်ကာ **run-time prerequisite မဟုတ်ပါ**။ Zap က npm install၊ JavaScript bundle၊ component hydration သို့မဟုတ် framework-specific adapter ကို ယခုမလုပ်ပေးသေးပါ။

## Route table

လက်ရှိ router သည် သေးငယ်ပြီး deterministic ဖြစ်သည်။

| Method | Path | Status | Body အဓိပ္ပာယ် |
|---|---|---:|---|
| `GET` | `/` | 200 | Greeting နှင့် request ID |
| `GET` | `/health` | 200 | Health status |
| `POST` | `/echo` | 200 | Bounded body နှင့် request ID ကို echo ပြန်ပေးသည် |
| Support လုပ်ထားသော method | မသိသော path | 404 | `not_found` error |
| Support မလုပ်သော method | Valid path မည်သည့် path မဆို | 405 | `method_not_allowed` error |
| Invalid path/body/request ID | မည်သည့် path မဆို | 400 | `invalid_request` error |

Invalid request ဖြစ်နေချိန်တွင် route function သည် handler ကို မခေါ်ပါ။ ဤအစီအစဉ်သည် အရေးကြီးသည်။ Validation နှင့် capability policy ကို application dispatch မတိုင်မီ လုပ်ထားသောကြောင့် အနာဂတ် adapter သည် authentication၊ rate limit နှင့် tracing ကို တည်ငြိမ်သော boundary ပတ်လည်တွင် ထည့်နိုင်သည်။

## Host-adapter contract

Production Web adapter သည် အောက်ပါ pipeline ကို အကောင်အထည်ဖော်သင့်သည်။

```text
HTTP bytes
  -> transport parser နှင့် maximum-size checks
  -> method/path/header normalization
  -> identity နှင့် capability checks
  -> bounded Zap request DTO
  -> route(request)
  -> response schema validation
  -> HTTP status/header/body encoding
  -> redaction ပါသော access log
```

Adapter ၏ တာဝန်များမှာ အောက်ပါအတိုင်း ဖြစ်သည်။

| Boundary | လိုအပ်သော behavior |
|---|---|
| Listener | ခွင့်ပြုထားသော address တွင်သာ bind လုပ်ပြီး development listener ကို မတော်တဆ public မဖြစ်စေရ |
| TLS | ရွေးချယ်ထားသော host stack သို့မဟုတ် trusted proxy တွင် terminate လုပ်ပြီး certificate rotation ကို document လုပ်ရမည် |
| Headers | Name များ normalize လုပ်၊ count/size cap ထား၊ authorization/cookie ကို log မှ redact လုပ်ရမည် |
| Body | Zap value မတည်ဆောက်မီ 64 KiB contract limit ကို စစ်ရမည်။ Streaming သည် API သီးခြားရှိမှသာ သုံးရမည် |
| Timeout | Header/body/handler/shutdown deadline များထားပြီး cancellation ကို handler boundary ထိ ပို့ရမည် |
| Identity | Authenticated identity ကို bounded DTO သို့ ပြောင်းပြီး raw socket/credential handle မပေးရ |
| Response | Status range၊ header name/value၊ content type နှင့် body size ကို write မလုပ်မီ validate လုပ်ရမည် |
| Observability | Request ID၊ route၊ status၊ duration နှင့် outcome ကို log လုပ်ပြီး secret/unbounded body ကို မထည့်ရ |
| Shutdown | Request အသစ်လက်မခံတော့ဘဲ bounded in-flight work ကို drain လုပ်ကာ forced termination ကို explicit report လုပ်ရမည် |

Cross-thread execution လိုအပ်လျှင် live Rust `Rc<RefCell>` value၊ socket object၊ OS handle သို့မဟုတ် thread-affine state ကို worker thread သို့ မပို့ရပါ။ Serializable DTO သို့မဟုတ် ownership boundary အသစ်ကိုသာ သုံးရမည်။

## Security model

Web starter သည် သတ်မှတ်ထားသော boundary အတွင်းတွင်သာ safe ဖြစ်သည်။ Traversal ပုံစံ path၊ support မလုပ်သော method၊ empty request ID နှင့် body ကြီးလွန်းခြင်းတို့ကို reject လုပ်သည်။ Authentication၊ authorization၊ TLS၊ CSRF protection၊ rate limit၊ request signing သို့မဟုတ် process sandbox ကို ကိုယ်တိုင် မပေးသေးပါ။

Real listener မဖွင့်မီ adapter သည် အောက်ပါ controls များကို ဖြည့်ရမည်။

| Threat | အနည်းဆုံး control | လိုအပ်သော evidence |
|---|---|---|
| Request-body memory exhaustion | Zap value မတည်ဆောက်မီ reject လုပ်ပြီး body bytes cap ထားရမည် | Oversize-body negative test နှင့် RSS/budget evidence |
| Path traversal/ambiguous routing | တစ်ကြိမ်တည်း canonicalize လုပ်၊ traversal/invalid encoding reject လုပ်၊ normalized path ဖြင့်သာ route လုပ်ရမည် | Encoded traversal corpus နှင့် route differential test |
| Header injection | Control character နှင့် adapter policy အရ duplicate-sensitive header များ reject လုပ်ရမည် | Header fuzz corpus |
| Authentication confusion | Authenticated identity ကို user-supplied field နှင့် ခွဲထားရမည် | Forged-ID နှင့် missing-identity tests |
| Secret leakage | Authorization၊ cookie နှင့် provider credential ကို logs ထဲ redact လုပ်ရမည် | Golden redaction fixtures |
| Slow client/server | Header/body/handler/shutdown deadline နှင့် cancellation ရှိရမည် | Timeout/cancellation test |
| Replay/duplicate command | Mutating route အတွက် idempotency policy ရှိရမည် | Duplicate-request test နှင့် stable outcome |
| Response splitting | Write မလုပ်မီ status/name/value/body encoding validate လုပ်ရမည် | Malformed-response corpus |
| Route explosion denial | Route table နှင့် dispatch cost ကို bounded/deterministic ထားရမည် | Route-count/worst-case dispatch benchmark |

## Testing contract

Web package သည် test layer လေးမျိုးကို ခွဲထားရမည်။

1. **Contract tests** သည် network မပါဘဲ normalization၊ route status၊ response schema၊ headers နှင့် negative case များကို စစ်ရမည်။
2. **Adapter tests** သည် fake host request/response DTO သုံးပြီး transport failure ကို typed Zap-facing failure သို့ မှန်ကန်စွာ ပြောင်းနိုင်ကြောင်း စစ်ရမည်။
3. **Integration tests** သည် host adapter ရှိလာပြီးမှ loopback server သုံးရမည်။ Payload bounded ဖြစ်ရမည်၊ fixed/injected listener သုံးရမည်၊ cleanup explicit လုပ်ရမည်။
4. **Security/reliability tests** သည် malformed path၊ oversized header/body၊ timeout၊ cancellation၊ duplicate request၊ log redaction နှင့် shutdown race များကို inject လုပ်ရမည်။

လက်ရှိ `web_contract_test.zp` သည် positive route များ၊ 400/405 rejection၊ request-ID validation၊ method normalization၊ JSON content type နှင့် no-store policy ကို cover လုပ်ထားသည်။ `api_contract_test.zp` သည် DTO mapping၊ repository success/not-found behavior၊ 401/403 authorization၊ 429 quota exhaustion၊ window reset၊ clock reversal နှင့် invalid policy ကို ထပ်မံ cover လုပ်ထားသည်။ `frontend_contract_test.zp` သည် asset manifest၊ HTML/CSS/JavaScript response type၊ browser API wiring နှင့် no-Node runtime declaration ကို cover လုပ်ပါသည်။ Native evaluator tests များသည် missing/unsupported asset၊ encoded traversal rejection၊ workspace confinement နှင့် final-segment wildcard matching ကို cover လုပ်ပါသည်။ ဤ test များသည် TLS၊ production concurrency သို့မဟုတ် external network behavior ပြီးစီးကြောင်း သက်သေမဟုတ်ပါ။

## API နှင့် DTO contract

Web API layer သည် server router မဟုတ်ဘဲ orchestration contract ဖြစ်သည်။ `api_contract.zp` သည် `get_user_api`, `create_user_api`, `list_users_api` functions များကို export လုပ်ထားသည်။ Host adapter သည် `GET /users/{id}`, `POST /users`, `GET /users` ကဲ့သို့ route များကို ထို functions များနှင့် map လုပ်နိုင်သော်လည်း variable-path matching ကို adapter ကသာ တာဝန်ယူရမည်။

| API function | Input DTO | Success | အရေးကြီးသော failures |
|---|---|---:|---|
| `get_user_api` | `request_id`, numeric `user_id`, auth context, rate state, timestamp | 200 | 401, 403, 404, 429, 500 |
| `create_user_api` | `request_id`, body DTO `{name, email}`, auth context, rate state, timestamp | 201 | 400, 401, 403, 429, 500, 503 |
| `list_users_api` | `request_id`, auth context, rate state, timestamp | 200 | 401, 403, 429, 500, 503 |

API သည် `response` နှင့် updated `rate_state` ပါသော wrapper ကို ပြန်ပေးသည်။ Wrapper မရှိပါက host adapter သည် successful request ပြီးနောက် quota state ကို မတော်တဆ ပျောက်စေနိုင်သည်။

Request DTO validator သည် text ဖြစ်သော `name` နှင့် `email` ကိုသာ လက်ခံသည်။ Name ကို trim လုပ်၊ email ကို lower-case ပြောင်း၊ length limit ထားပြီး `@` marker အနည်းဆုံးလိုအပ်သည်။ ဤသည်မှာ သေးငယ်သော contract ဖြစ်ပြီး complete email verification policy မဟုတ်ပါ။ Real API တွင် schema package ပိုတင်းကျပ်စွာ ထည့်နိုင်သော်လည်း explicit size limit နှင့် deterministic error mapping ကို မဖျက်ရ။

## Database integration boundary

`database_contract.zp` သည် `repository_info`, `find_user`, `insert_user`, `list_users` functions များဖြင့် repository boundary ကို သတ်မှတ်ထားသည်။ လက်ရှိ implementation သည် deterministic fake repository ဖြစ်သောကြောင့် credential၊ network၊ database process သို့မဟုတ် mutable global state မလိုဘဲ API ကို test လုပ်နိုင်သည်။ ထပ်တိုး `database_adapter.zp` သည် user lookup/insert အတွက် provider-neutral parameterized query descriptor နှင့် public field များသာ ထုတ်ပေးသည့် `user_row_dto` mapping ကို သတ်မှတ်ထားသည်။ ၎င်းသည် adapter intent ကိုသာ ဖော်ပြပြီး connection ဖွင့်ခြင်း သို့မဟုတ် request-time query execute လုပ်ခြင်း မပြုပါ။

| Database boundary | Contract requirement |
|---|---|
| Driver selection | PostgreSQL၊ SQLite၊ MySQL သို့မဟုတ် အခြား driver ကို host adapter က ရွေးရမည်။ Zap code သည် driver တစ်ခုကို မယူဆရ |
| Query arguments | Validated DTO fields ကို bound parameters အဖြစ် ပေးရမည်။ User text နှင့် SQL ကို string concatenate မလုပ်ရ။ `database_adapter.zp` query descriptor ကို အသုံးပြုရမည် |
| Transactions | Transaction begin/commit/rollback ကို adapter က ပိုင်ဆိုင်ပြီး typed success/failure DTO သာ ပြန်ပေးရမည် |
| Connection pool | Pool size၊ acquisition timeout၊ idle timeout နှင့် shutdown ကို adapter က ပိုင်ဆိုင်ရမည် |
| Failure mapping | Not-found သည် domain result ဖြစ်ရမည်။ Connection/timeout/pool failure ကို repository-unavailable result အဖြစ် map လုပ်ရမည် |
| Returned row | `user_response` မှတစ်ဆင့် public fields များသာ map လုပ်ရမည်။ Password hash၊ token၊ internal note သို့မဟုတ် driver handle မထုတ်ရ |
| Observability | Operation name၊ duration၊ outcome နှင့် request ID ကို log လုပ်ရမည်။ Query value နှင့် secret ကို redact လုပ်ရမည် |

API သည် repository not-found ကို `404` နှင့် repository availability failure ကို `503` အဖြစ် map လုပ်သည်။ Duplicate execution လုံခြုံသော operation များတွင်သာ retry/idempotency ထည့်ရမည်။ Insert ကို မစဉ်းစားဘဲ blind retry မလုပ်ရ။

## Authentication နှင့် authorization

`auth_contract.zp` သည် raw credential ကို host က validate လုပ်ပြီးသားဟု ယူဆသည်။ Host သည် `authenticated`၊ bounded `subject` နှင့် bounded scope list ကိုသာ `auth_context` သို့ ပေးရမည်။ Zap contract သည် `Authorization` header၊ cookie၊ private key သို့မဟုတ် token secret ကို မဖတ်ရ။

`authorize(context, "users:read")` သို့မဟုတ် `authorize(context, "users:write")` သည် deterministic decision ပြန်ပေးသည်။ Identity မရှိလျှင် `401`၊ authenticated ဖြစ်သော်လည်း required scope မရှိလျှင် `403`၊ internal policy မမှန်လျှင် `500` ဖြစ်သည်။ User-supplied request field သည် authenticated subject ကို အစားမထိုးနိုင်ရန် host adapter က စစ်ရမည်။

Production adapter ၏ အနည်းဆုံး policy သည် issuer၊ audience၊ expiry၊ signature၊ token type နှင့် key rotation ကို host identity layer တွင် စစ်ရန်၊ Zap value မတည်ဆောက်မီ subject/scope count ကို bound လုပ်ရန်၊ log လမ်းကြောင်းအားလုံးတွင် raw credential ကို redact လုပ်ရန်နှင့် revocation/clock-skew behavior ကို သတ်မှတ်ရန် ဖြစ်သည်။ ဤအရာများကို contract starter က မအကောင်အထည်ဖော်သေးပါ။

## Rate-limiting contract

`rate_limit_contract.zp` သည် deterministic fixed-window decision function ကို အကောင်အထည်ဖော်သည်။ State တွင် `key`, `limit`, `window_ms`, `window_start`, `used` ပါဝင်ပြီး `allow_request` သည် `allowed`၊ `remaining` သို့မဟုတ် `retry_after_ms` နှင့် next state ကို ပြန်ပေးသည်။

| Decision | Status | Host က လုပ်ရမည့်အရာ |
|---|---:|---|
| Quota အတွင်း valid request | 200 | ပြန်ရလာသော state ကို request result လက်ခံမီ atomic persist လုပ်ရမည် |
| Quota ပြည့်သွားခြင်း | 429 | `retry_after_ms` ပြန်ပေးပြီး protected repository operation ကို မခေါ်ရ |
| Window သက်တမ်းကုန်ခြင်း | 200 | Usage ကို reset လုပ်၊ timestamp အသစ်နှင့် window start ကို persist လုပ်ရမည် |
| Clock နောက်ပြန်သွားခြင်း | 500 | Decision ကို reject လုပ်ပြီး adapter တွင် monotonic clock သုံးရမည် |
| Key/policy မမှန်ခြင်း | 500 | Fail closed လုပ်ပြီး configuration owner ကို alert လုပ်ရမည် |

Adapter သည် authenticated subject သို့မဟုတ် normalized network identity ကဲ့သို့ keying strategy ကို ရွေးချယ်ရမည်။ Arbitrary client header ကို identity key အဖြစ် မယုံရ။ Zap function သည် shared state ကို mutate မလုပ်ဘဲ new state ပြန်ပေးသောကြောင့် concurrent request များ quota ကျော်မသွားရန် host တွင် atomic store သို့မဟုတ် single-owner event loop လိုအပ်သည်။

Fixed-window algorithm သည် foundation သာဖြစ်ပြီး abuse control အပြည့်အစုံ မဟုတ်ပါ။ Burst၊ endpoint၊ organization နှင့် global limits များ ထည့်မည်ဆိုလျှင် limiter တစ်ခုချင်းစီအတွက် key၊ clock၊ storage၊ failure နှင့် observability contract သီးခြားထားရမည်။

## API security နှင့် reliability test matrix

| Test group | လိုအပ်သော cases | Pass evidence |
|---|---|---|
| DTO | Missing field၊ wrong type၊ empty/oversized name၊ invalid email၊ lower-case normalization | `api_contract_test.zp` နှင့် boundary corpus |
| Repository | Found row၊ not-found row၊ invalid ID၊ insert success၊ unavailable/timeout mapping | Fake repository contract နှင့် adapter failure tests |
| Authorization | Unauthenticated၊ missing scope၊ valid read/write၊ forged subject mismatch | 401/403 matrix နှင့် identity-binding fixture |
| Rate limit | First request၊ last allowed request၊ 429၊ reset၊ duplicate state၊ clock reversal၊ invalid policy | State transition table နှင့် atomic-store test |
| API composition | Auth မတိုင်မီ repository မခေါ်ခြင်း၊ rate limit မတိုင်မီ repository မခေါ်ခြင်း၊ insert မတိုင်မီ DTO စစ်ခြင်း၊ response တိုင်း request ID ပါခြင်း | Call-order သို့မဟုတ် fake-adapter trace |
| Reliability | Repository timeout၊ cancellation၊ pool exhaustion၊ retry/insert idempotency၊ shutdown | Bounded deadline ပါသော fault-injection report |
| Security | Header redaction၊ body cap၊ path normalization၊ response validation၊ log ထဲ raw credential မပါခြင်း | Golden logs နှင့် malformed-input corpus |

ပြန်လည်အသုံးပြုနိုင်သော contract test layer သည် database/network မပါဘဲ run လုပ်သည်။ Zap-native Web slice တွင် bounded၊ single-threaded development server အတွက် loopback integration test လည်း ရှိလာပါပြီ။ Production promotion မလုပ်မီ fake-host adapter suite နှင့် injected listener၊ bounded payload၊ deterministic clock၊ cancellation၊ explicit cleanup ပါသော production-oriented loopback suite တို့ကို ထည့်ရမည်။

## Route တိုးချဲ့နည်း

Route တစ်ခုထည့်ရန် `web_contract.zp` ၏ route table/logic ကို update လုပ်ရမည်။ ထို့နောက် positive test တစ်ခု၊ လိုအပ်လျှင် negative သို့မဟုတ် authorization test တစ်ခုနှင့် bilingual documentation update တို့ ထည့်ရမည်။ Starter validator၊ `zap check`၊ `zap build`၊ `zap run` နှင့် `zap test` အားလုံး pass ဖြစ်မှ route change ပြီးစီးသည်ဟု သတ်မှတ်ရမည်။

အနာဂတ် adapter package မထည့်မီ capability list၊ DTO schema၊ error taxonomy၊ timeout policy၊ body limits၊ cancellation behavior၊ log-redaction policy နှင့် shutdown contract ကို အရင် document လုပ်ရမည်။ Adapter သည် Web contract ကို depend လုပ်ရမည်။ Web contract သည် server၊ database၊ cloud သို့မဟုတ် UI stack တစ်ခုခုကို မdepend လုပ်ရ။

## မလုပ်သေးသည့်အရာများ

လက်ရှိ Web Foundation သည် production HTTP server ဖြစ်ကြောင်း မဆိုထားပါ။ Zap-native slice သည် bounded၊ loopback-only၊ single-threaded development/reference server ကို ပေးထားသော်လည်း concurrent production reactor မဟုတ်ပါ။ API၊ database၊ authentication နှင့် rate-limit files များသည် contract prototype နှင့် deterministic test double များအဖြစ် ဆက်ရှိနေပြီး TLS၊ HTTP/2/HTTP/3 policy၊ WebSocket၊ real database connectivity၊ credential verification၊ distributed quota storage၊ template၊ static-file serving၊ background jobs၊ cloud deployment နှင့် automatic code generation မပေးသေးပါ။ Production feature တစ်ခုချင်းစီအတွက် host adapter contract နှင့် evidence သီးခြားလိုအပ်သည်။

## References

[1]: https://docs.rs/axum/latest/axum/ "Axum documentation — routing, extractors, responses, and Tower integration"
