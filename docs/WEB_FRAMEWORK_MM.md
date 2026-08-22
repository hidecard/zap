# Zap Web Framework Foundation

**အတည်ပြုထားသော baseline:** Zap v2.2.3
**Branch:** `Framework`
**အခြေအနေ:** Web Foundation v0.2 — run လို့ရသော contract package ဖြစ်ပြီး production HTTP adapter ကို သီးခြားထားသည်

## ရည်ရွယ်ချက်

Web package သည် Zap application နှင့် host HTTP implementation ကြားတွင် တည်ငြိမ်သော boundary တစ်ခု သတ်မှတ်ပေးသည်။ Zap ဘက်တွင် bounded request ကို normalize လုပ်ပြီး route/method policy ကို စစ်ဆေးကာ bounded response map ပြန်ပေးသည်။ Listener မဖွင့်ပါ၊ TLS မပိုင်ပါ၊ blocking socket work မလုပ်ပါ။

အနာဂတ် host adapter သည် Zap runtime ကို network reactor အသစ်တစ်ခုအဖြစ် ပြန်ရေးမည့်အစား ရှိပြီးသား HTTP stack ကို reuse လုပ်နိုင်ရန် ဒီ contract ကို သတ်မှတ်ထားခြင်း ဖြစ်သည်။ Rust adapter သည် Axum/Tower ကဲ့သို့ ရှိပြီးသား routing နှင့် middleware ecosystem ကို အသုံးပြုပြီး document လုပ်ထားသော DTO boundary ကိုသာ ဘာသာပြန်နိုင်သည်။[1]

## လက်ရှိ package ဖွဲ့စည်းပုံ

| ဖိုင် | တာဝန် | အခြေအနေ |
|---|---|---|
| `zap.toml` | Dependency-free contract package manifest | ပြီးစီး |
| `zap.lock` | Canonical empty-dependency lockfile | ပြီးစီး |
| `web_contract.zp` | Request, response, security-header နှင့် router functions | ပြီးစီး |
| `main.zp` | Deterministic end-to-end contract demonstration | ပြီးစီး |
| `web_contract_test.zp` | Positive/negative contract regression suite | ပြီးစီး |
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

လက်ရှိ `web_contract_test.zp` သည် positive route များ၊ 400/405 rejection၊ request-ID validation၊ method normalization၊ JSON content type နှင့် no-store policy ကို cover လုပ်ထားသည်။ ဤ test သည် TLS၊ authentication၊ production concurrency သို့မဟုတ် external network behavior ပြီးစီးကြောင်း သက်သေမဟုတ်ပါ။

## Route တိုးချဲ့နည်း

Route တစ်ခုထည့်ရန် `web_contract.zp` ၏ route table/logic ကို update လုပ်ရမည်။ ထို့နောက် positive test တစ်ခု၊ လိုအပ်လျှင် negative သို့မဟုတ် authorization test တစ်ခုနှင့် bilingual documentation update တို့ ထည့်ရမည်။ Starter validator၊ `zap check`၊ `zap build`၊ `zap run` နှင့် `zap test` အားလုံး pass ဖြစ်မှ route change ပြီးစီးသည်ဟု သတ်မှတ်ရမည်။

အနာဂတ် adapter package မထည့်မီ capability list၊ DTO schema၊ error taxonomy၊ timeout policy၊ body limits၊ cancellation behavior၊ log-redaction policy နှင့် shutdown contract ကို အရင် document လုပ်ရမည်။ Adapter သည် Web contract ကို depend လုပ်ရမည်။ Web contract သည် server၊ database၊ cloud သို့မဟုတ် UI stack တစ်ခုခုကို မdepend လုပ်ရ။

## မလုပ်သေးသည့်အရာများ

လက်ရှိ Web Foundation သည် production HTTP server ဖြစ်ကြောင်း မဆိုထားပါ။ Multi-request reactor၊ TLS၊ HTTP/2/HTTP/3 policy၊ WebSocket၊ database access၊ template၊ static-file serving၊ authentication၊ authorization၊ rate limit၊ background jobs၊ cloud deployment နှင့် automatic code generation မပါဝင်သေးပါ။ ထို feature များသည် contract နှင့် evidence သီးခြားလိုအပ်သည်။

## References

[1]: https://docs.rs/axum/latest/axum/ "Axum documentation — routing, extractors, responses, and Tower integration"
