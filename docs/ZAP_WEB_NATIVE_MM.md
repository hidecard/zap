# Zap-first Web Framework လမ်းညွှန်

**အတည်ပြုထားသော baseline:** Zap v2.11.16၊ merged `master`။ မူလ Framework အလုပ်များကို Web contract foundation အဖြစ် ဆက်လက်ထိန်းသိမ်းထားသည်။

## ရည်ရွယ်ချက်

Zap Web ကို **Zap ကိုယ်တိုင်က အဓိက Web framework ဖြစ်သည့် full-stack platform** အဖြစ် တည်ဆောက်နေပါသည်။ Django မှာကောင်းမွန်သော project structure၊ routing၊ app modules၊ model၊ migration၊ authentication၊ admin၊ testing နှင့် deployment workflow များကို လေ့လာပြီး Zap language အတွက် သီးခြားပုံစံဖြင့် ပြန်တည်ဆောက်မည်ဖြစ်သည်။ Python syntax သို့မဟုတ် Django implementation ကို တိုက်ရိုက်ကူးယူမည် မဟုတ်ပါ။

အခြေခံမူမှာ—

> **Code နည်းနည်းရေး၊ အတွင်း flow ကို နားလည်၊ လုံခြုံစွာ deploy လုပ်။**

Framework သည် safe default များ ပေးရမည်ဖြစ်သော်လည်း hidden magic များလွန်းခြင်း မဖြစ်ရပါ။ Developer သည် project က ဘာလုပ်နေသည်ကို source နှင့် command များမှ တိုက်ရိုက်နားလည်နိုင်ရမည်။

## လက်ရှိ runnable ဖြစ်သောအရာ

လက်ရှိ repository တွင် Zap source file များနှင့် native Zap project checker ကို အသုံးပြုသော Web-first project scaffold ရှိပါသည်။ Python သို့မဟုတ် JavaScript application layer မလိုပါ။

```text
zap new shop
cd shop
zap check
zap web check
zap web routes
zap db check
zap db inspect --json
zap db plan
zap db migrate --dry-run
zap db migrate --check
zap db migrate
zap test tests
zap run main.zp
zap dev
```

`zap new <directory>` သည် project တစ်ခုလုံးကို command တစ်ကြောင်းတည်းဖြင့် ဖန်တီးပေးသော Zap ၏ canonical project generator ဖြစ်ပါသည်။ `zap.toml`၊ `zap.lock`၊ `main.zp`၊ `web.zp`၊ `models/`၊ `functions/`၊ `ui/`၊ `routes/`၊ `middleware/`၊ `migrations/`၊ `admin/`၊ `public/`၊ `server.zp` နှင့် `tests/` ပါသော user-managed project structure ကို ထုတ်ပေးပါသည်။ Django-style `startapp` command မရှိပါ။ Project ဖန်တီးပြီးသည်နှင့် file/module များကို user ကိုယ်တိုင် ထည့်ခြင်း၊ ဖျက်ခြင်း၊ ပြင်ခြင်းနှင့် စီမံခြင်း ပြုလုပ်နိုင်ပါသည်။ Generated `public/` directory ထဲတွင် ရိုးရိုး HTML entrypoint၊ CSS နှင့် `/api/tasks` ကို ခေါ်သည့် browser ES module ပါရှိပြီး run လုပ်ရန် Node.js မလိုပါ။ Generated `ui/ui.zp` သည် browser entrypoint၊ asset root၊ frontend mode နှင့် runtime တွင် Node မလိုကြောင်း သတ်မှတ်ပေးသော သီးခြား UI boundary ဖြစ်ပါသည်။ Generated entrypoint သည် application metadata၊ route table၊ model metadata၊ UI metadata၊ middleware order နှင့် admin registry ကို deterministic အဖြစ် print လုပ်ပါသည်။ Generated `server.zp` သည် bounded native development server entrypoint ဖြစ်ပြီး `zap dev` ဖြင့် စတင်နိုင်ပါသည်။ သို့သော် complete production Web platform မဟုတ်သေးပါ။

Native CLI သည် ယခု constrained `[web]` manifest section နှင့် optional `[database]` section ကို စစ်ဆေးနိုင်ပါသည်။ Routes file၊ model directory၊ middleware file၊ migration directory၊ admin file နှင့် server entrypoint တို့ ရှိ/မရှိ၊ path များသည် safe relative path ဖြစ်/မဖြစ်နှင့် first Web profile သည် JSON-by-default ဖြစ်/မဖြစ် စစ်ဆေးပါသည်။ `[web]` မပါသော generic Zap project များသည် အရင်အတိုင်း valid ဖြစ်နေပါမည်။ `zap web check` သည် project structure ကို စစ်ပြီး `zap web routes` သည် listener မဖွင့်ဘဲ export လုပ်ထားသော `routes()` factory ကို execute လုပ်ကာ route table ကို ပြပါသည်။ `zap db check` သည် structured migration declaration နှင့် deterministic SQL plan ကို စစ်ပါသည်။ `zap dev` သည် manifest ထဲက `server.zp` ကို Web validation ပြီးမှ run ပါသည်။

## Project နှင့် app model

Zap Web project သည် deploy လုပ်မည့် website boundary ဖြစ်ပါသည်။ Generated directory များသည် hidden framework registration မဟုတ်ဘဲ user ကိုယ်တိုင် စီမံနိုင်သော ရိုးရိုး Zap module များ ဖြစ်ပါသည်။ Accounts၊ catalog၊ billing သို့မဟုတ် device ကဲ့သို့ feature များကို ဤ directory များအောက်တွင် စိတ်ကြိုက် စီမံနိုင်ပြီး သီးခြား app-generation command မလိုပါ။

လက်ရှိ scaffold ၏ ဖတ်ရလွယ်သော layout မှာ—

```text
my_app/
├── zap.toml
├── zap.lock
├── main.zp
├── web.zp
├── server.zp
├── models/
│   └── user.zp
├── functions/
│   └── user_functions.zp
├── ui/
│   └── ui.zp
├── routes/
│   └── routes.zp
├── middleware/
│   └── middleware.zp
├── migrations/
│   └── 0001_initial.zp
├── admin/
│   └── admin.zp
├── public/
│   ├── index.html
│   └── assets/
│       ├── app.css
│       └── app.js
└── tests/
    └── web_test.zp
```

`routes/routes.zp` သည် route catalog၊ `models/` သည် data metadata၊ `functions/` သည် business operation နှင့် request handler၊ `ui/ui.zp` သည် browser-facing UI metadata၊ `public/` သည် HTML/CSS/JavaScript asset များ၊ `middleware/middleware.zp` သည် cross-cutting policy အစီအစဉ်၊ `migrations/` သည် schema intent၊ `admin/admin.zp` သည် management registration နှင့် `tests/` သည် project test များကို ပိုင်ဆိုင်ပါသည်။ ဤ file များသည် user က တိုက်ရိုက်ပြင်နိုင်သော explicit source file များဖြစ်ပြီး သီးခြား app registry သို့မဟုတ် generator မလိုပါ။ Generated `zap.toml` တွင် `[database] driver = "sqlite"` နှင့် `url = "data/zap.sqlite3"` ကိုလည်း ကြေညာထားပါသည်။ ဤ structure သည် convention သက်သက်မဟုတ်ဘဲ `[web]` manifest၊ optional `[database]` validator နှင့် project checker က စစ်ဆေးပေးသည့် convention ဖြစ်ပါသည်။

## Runtime independence နှင့် frontend integration

Installed Zap executable တစ်ခုရှိရုံဖြင့် project validation၊ testing နှင့် server execution လုပ်နိုင်ရန် ရည်ရွယ်ထားပါသည်။ Deployment host တွင် Python၊ Node.js၊ Rust၊ Java သို့မဟုတ် အခြား language runtime ထပ်မလိုသင့်ပါ။ Rust သည် native Zap executable ကို implement/distribute လုပ်ရန် အသုံးပြုသော source/build detail ဖြစ်ပြီး Zap project ၏ runtime dependency မဟုတ်ပါ။ Cross-platform release များတွင် support လုပ်သည့် operating system တစ်ခုချင်းစီအတွက် pinned executable သို့မဟုတ် installer ပေးရမည်။

Browser boundary သည် ရိုးရိုး file များကိုသာ အသုံးပြုပါသည်။ `public` directory မှ HTML၊ CSS၊ JavaScript၊ image၊ font နှင့် Wasm ကို `web_static` ဖြင့် serve လုပ်နိုင်ပြီး browser application က Zap JSON route များကို ခေါ်နိုင်ပါသည်။ React၊ Vue၊ Svelte၊ Alpine သို့မဟုတ် အခြား JavaScript framework ကို build-time toolchain အဖြစ် optional သုံးနိုင်ပြီး ထွက်လာသော file များကို `public/assets/` ထဲ ထည့်နိုင်ပါသည်။ ထို့နောက် deployed process အတွက် Zap နှင့် ထို browser output file များသာ လိုအပ်ပြီး Node သည် runtime prerequisite မဟုတ်ပါ။ Zap က npm package install၊ JavaScript framework execution သို့မဟုတ် compiler/bundler အစားထိုးခြင်း မလုပ်ပေးသေးပါ။

`web_static` builtin သည် asset များကို project root အတွင်းတွင်သာ ချုပ်ထားပြီး traversal နှင့် unsupported extension များကို reject လုပ်ကာ binary asset များကို bounded response representation ဖြင့် ပြန်ပေးပါသည်။ `web_static_spa(asset, root, fallback)` သည် requested asset သို့မဟုတ် client-side route အတွက် validated entry document ကို serve လုပ်ပါသည်။ နောက်ဆုံး `*name` wildcard သည် `/assets/chunks/app.js` ကဲ့သို့ nested path များကို support လုပ်ပြီး API နှင့် asset path များကို SPA fallback မတိုင်မီ ထားရမည်။ Cache fingerprint၊ server-side rendering သို့မဟုတ် production static CDN များသည် deployment concern များအဖြစ် ဆက်ရှိပါသည်။

## လက်ရှိ route declaration contract

လက်ရှိ Zap parser တွင် first-class `route GET "/..." fn(req)` statement မရှိသေးပါ။ ထို့ကြောင့် scaffold သည် ရိုးရိုး exported Zap function တစ်ခုက route table ပြန်ပေးသည့်ပုံစံကို အသုံးပြုထားပါသည်။

```zap
export fn routes():
    return [{"method": "GET", "path": "/", "handler": "home", "scope": ""}, {"method": "GET", "path": "/users/:id", "handler": "get_user", "scope": "users:read"}]
```

ဤနည်းသည် လက်ရှိ project ကို runnable နှင့် inspectable ဖြစ်စေပြီး future parser/AST ပြောင်းလဲမှုအတွက် compatibility-reviewed RFC တစ်ခု ထားနိုင်စေပါသည်။ `zap web routes` သည် listener မဖွင့်ဘဲ ဤ factory ကို execute လုပ်ကာ method/path registration များ unique ဖြစ်/မဖြစ် စစ်ပြီး resolved table ကို text သို့မဟုတ် JSON အဖြစ် ပြပါသည်။ Live Web server သည် အတူတူ conflict check လုပ်သည့်အပြင် traffic လက်မခံမီ named handler အားလုံး resolve ဖြစ်/မဖြစ်ကို ထပ်မံစစ်ပါသည်။ အောက်ပါ concise route form သည် **design notation သာ ဖြစ်ပြီး လက်ရှိ parser က မလက်ခံသေးပါ**။ Parser contract မထည့်သွင်းမီ project ထဲတွင် မသုံးရပါ။

```zap
route GET "/users/:id" handler get_user scope "users:read"
```

အနာဂတ် route system တွင် ordered matching၊ typed path parameter၊ route name၊ reverse URL၊ conflict detection၊ method policy နှင့် centralized `400/404/405/500` handling တို့ ပါသင့်ပါသည်။ Framework defaults များကို developer မမြင်ရအောင် မဖုံးကွယ်ဘဲ tooling ဖြင့် route catalog ကို ဖော်ပြနိုင်ရမည်။

## Request နှင့် response model

Map နှင့် list value များအတွက် JSON ကို default API representation အဖြစ် သုံးပါမည်။ HTML rendering ကို ambiguous return value ဖြင့် ခန့်မှန်းမည့်အစား future standard template surface ဖြင့် explicit လုပ်သင့်ပါသည်။ လက်ရှိ `frameworks/web` contract နှင့် `zap-host` adapter သည် path length၊ body length၊ request-ID bound၊ method policy၊ traversal rejection၊ security headers နှင့် stable error shape များကို စစ်ဆေးပါသည်။

Production request pipeline ကို အောက်ပါအစီအစဉ်ဖြင့် သတ်မှတ်သင့်ပါသည်။

| အစီအစဉ် | Boundary | တာဝန် |
|---:|---|---|
| ၁ | Transport | HTTP parse၊ protocol/connection limit |
| ၂ | Request policy | method/path normalize၊ traversal၊ request ID နှင့် body bound စစ်ခြင်း |
| ၃ | Correlation | valid request ID ဖန်တီး/လက်ခံခြင်း၊ invalid ID မ echo ခြင်း |
| ၄ | Middleware | security header၊ proxy policy၊ rate limit၊ session၊ identity context |
| ၅ | Router | ordered route match နှင့် path parameter conversion |
| ၆ | Authorization | repository မခေါ်မီ scope/permission စစ်ခြင်း |
| ၇ | Validation | bounded JSON input ကို DTO ပြောင်းခြင်း၊ field error ပြန်ခြင်း |
| ၈ | Service | business policy နှင့် transaction boundary |
| ၉ | Repository | injected database adapter မှ parameterized operation |
| ၁၀ | Response | explicit DTO serialize လုပ်ခြင်း၊ internal field redact လုပ်ခြင်း |

လက်ရှိ `frameworks/web` contract နှင့် `host/zap-host` adapter သည် ဤ boundary များ၏ safe subset ကို implement လုပ်ထားပါသည်။ Zap-first project scaffold သည် full native server မရှိသေးသော်လည်း တူညီသော boundary များကို ရိုးရိုး Zap data ဖြင့် မှတ်တမ်းတင်ထားပါသည်။

## Typed request validation နှင့် Result-aware responses

Native runtime တွင် `web_validate_request(body, schema)` ကို bounded typed boundary အဖြစ် ပေးထားပါသည်။ `body` သည် parse လုပ်ပြီးသား map ဖြစ်နိုင်သကဲ့သို့ 64 KiB ထက် မကျော်သော raw JSON text လည်း ဖြစ်နိုင်ပါသည်။ Schema တွင် field အများဆုံး 64 ခု ပါနိုင်ပြီး field specification များသည် `text`၊ `number`၊ `bool`၊ `map`၊ `list` နှင့် `none` ကို support လုပ်ပါသည်။ `required` option နှင့် text အတွက်သာ `max_len` option ကို ထည့်နိုင်ပါသည်။ Unknown field၊ ပျောက်နေသော required field၊ invalid JSON၊ type မကိုက်ညီမှုနှင့် length violation များသည် `status`၊ `code`၊ `message` နှင့် ရှိပါက `field` ပါသော `ResultErr` map အဖြစ် ပြန်လာပါသည်။

```zap
export fn create_user(request):
    let schema = {"name": {"type": "text", "max_len": 120}, "email": {"type": "text", "max_len": 254}}
    let checked = web_validate_request(request["body"], schema)
    if is_err(checked):
        return checked
    let payload = unwrap(checked)
    return ok({"status": 201, "body": json({"created": true, "body": payload})})
```

Native server သည် centralized Result response middleware အဖြစ် လုပ်ဆောင်ပါသည်။ `ResultOk(response_map)` သည် ရှိပြီးသား response encoder ကို သုံးပြီး 400–599 status နှင့် safe error code ပါသော error map ကို `error`၊ `message` နှင့် request ID ပါသည့် JSON error response အဖြစ် ပြောင်းပေးပါသည်။ Validator သည် malformed JSON၊ invalid body shape၊ invalid schema နှင့် field-level request violation များအတွက် `400` ကို ရည်ရွယ်ချက်ရှိရှိ အသုံးပြုပါသည်။ Semantic အရ invalid ဖြစ်သော payload အတွက် handler က `422` ကို ရွေးချယ်ပြန်ပေးနိုင်ပါသည်။ ရှိပြီးသား direct response map များကို compatibility အတွက် ဆက်လက်ထောက်ပံ့ပါသည်။ Raise သို့မဟုတ် malformed response value ဖြစ်ပါက `500 handler_error` အဖြစ် fail closed လုပ်ပါသည်။ ဤသည်မှာ bounded response/validation contract ဖြစ်ပြီး full schema compiler သို့မဟုတ် production middleware graph မဟုတ်သေးပါ။

## Middleware design

Middleware သည် decorator စုစည်းမှုမဟုတ်ဘဲ ordered request/response pipeline ဖြစ်ပါသည်။ Middleware entry တစ်ခုစီတွင် name၊ stage၊ order နှင့် short-circuit behavior ကို ဖော်ပြသင့်ပါသည်။ Handler မခေါ်မီ request ကို reject လုပ်နိုင်သည်၊ request context ထဲ data ထည့်နိုင်သည် သို့မဟုတ် response ပြန်ရာတွင် header ထည့်နိုင်သည်။

Scaffold မှာ request ID၊ auth နေရာနှင့် security header ကို ပြထားပါသည်။

```zap
export fn middleware_stack():
    return [{"name": "request_id", "stage": "before", "order": 10}, {"name": "auth", "stage": "before_handler", "order": 40}, {"name": "security_headers", "stage": "after", "order": 90}]
```

Framework checker သည် duplicate name၊ invalid order၊ dependency မကိုက်ညီမှုနှင့် database operation ပြီးမှ authorization စစ်သည့် unsafe placement များကို reject လုပ်သင့်ပါသည်။ Middleware order ကို `zap web check` သို့မဟုတ် `zap web routes` သို့မဟုတ် အနာဂတ် `zap explain` command မှ ပြသနိုင်ရမည်။

## Models၊ DTO နှင့် ORM လမ်းကြောင်း

Model သည် database schema intent ၏ source ဖြစ်ပါသည်။ DTO သည် request/response data boundary ဖြစ်ပါသည်။ နှစ်ခုကို မရောသင့်ပါ။ Untrusted request ကို model သို့ တိုက်ရိုက် map လုပ်ခြင်းသည် internal field leak၊ validation bypass သို့မဟုတ် schema change ကို မရည်ရွယ်ဘဲ API change ဖြစ်စေနိုင်ပါသည်။

လက်ရှိ scaffold သည် model metadata ကို Zap function ဖြင့် မှတ်တမ်းတင်ပါသည်။

```zap
export fn user_model():
    return {"name": "User", "table": "users", "fields": {"id": "number primary_key", "name": "text required", "email": "email unique"}}
```

စီစဉ်ထားသည့် ORM သည် dynamic ORM အလွန်ကြီးတစ်ခုမဟုတ်ဘဲ typed model metadata၊ nullability/uniqueness၊ relationship၊ parameterized query၊ transaction handle၊ pool timeout၊ cancellation/deadline နှင့် stable database error classification ကို ဦးစားပေးသင့်ပါသည်။ Query construction သည် inspectable ဖြစ်ရပြီး untrusted value ကို SQL ထဲ တိုက်ရိုက် string concatenate မလုပ်ရပါ။

Production repository ကို provider-neutral interface နောက်ကွယ်တွင် inject လုပ်ရမည်။ Deterministic `database_contract.zp` နှင့် `WebGateway` သည် contract test အတွက် အသုံးဝင်သော်လည်း real database driver မဟုတ်ပါ။ Native runtime တွင် structured migration workflow အတွက် SQLite-first adapter ကို ထည့်သွင်းထားပါသည်။ PostgreSQL၊ MySQL နှင့် အခြား backend များအတွက် capability၊ query၊ transaction နှင့် migration test သီးခြားပါသော explicit adapter များ လိုအပ်ပါသည်။

## Migrations

Migration သည် code နှင့်အတူ version-control ထဲ commit လုပ်ထားသည့် schema intent ဖြစ်ပါသည်။ Scaffold တွင်—

```zap
export fn migration():
    return {"id": "0001_initial", "depends_on": [], "operations": [{"kind": "create_table", "table": "users", "columns": {"id": "integer primary key", "name": "text not null", "email": "text not null unique"}}]}
```

ပထမဆုံး native adapter သည် **SQLite-first** ဖြစ်ပါသည်။ Migration file တစ်ခုစီတွင် exported၊ parameter မပါသော `migration()` function တစ်ခုသာ ပါရမည်ဖြစ်ပြီး return value သည် literal map/list tree ဖြစ်ရပါမည်။ ပထမအဆင့်တွင် `create_table` နှင့် `add_column` operation များကိုသာ support လုပ်ထားပါသည်။ Identifier များကို allow-list လုပ်ထားပြီး column type/modifier များကို bounded ထားပါသည်။ Arbitrary SQL၊ function call၊ variable name နှင့် interpolation များကို reject လုပ်ပါသည်။

`zap db check` သည် migration declaration များကို validate လုပ်ပြီး database မဖွင့်ဘဲ deterministic SQL plan ကို compile လုပ်ပါသည်။ `zap db plan` သည် SQLite migration ledger ရှိလျှင် ဖတ်ကာ pending SQL ကို ပြပါသည်။ `zap db plan --json` သည် machine-readable output ထုတ်ပေးပါသည်။ `zap db migrate --dry-run` သည် read-only plan ကိုသာ run ပါသည်။ `zap db migrate` သည် SQLite database ကို ဖန်တီးပြီး pending migration များကို transaction တစ်ခုအတွင်း apply လုပ်ကာ foreign key ကို enable လုပ်ပြီး `__zap_migrations` ledger ထဲ migration တစ်ခုစီ၏ checksum ကို မှတ်တမ်းတင်ပါသည်။ Apply ပြီးသား migration ကို မသိမသာ edit လုပ်၍ မရဘဲ command က fail ဖြစ်ပြီး migration အသစ်ရေးရန် လိုအပ်ပါသည်။ Controlled deployment သို့မဟုတ် test environment အတွက် manifest URL ကို `ZAP_DATABASE_URL` ဖြင့် override လုပ်နိုင်ပါသည်။

Production migration workflow သည် migration ရေးခြင်း၊ dependency/SQL plan စစ်ခြင်း၊ isolated environment တွင် apply လုပ်ခြင်း၊ compatibility check လုပ်ခြင်း၊ rolling update အတွင်း schema version နှစ်ခုလုံးကို support လုပ်ခြင်းနှင့် applied migration ကို atomic မှတ်တမ်းတင်ခြင်းတို့ ဖြစ်နေဆဲဖြစ်ပါသည်။ လက်ရှိ native slice သည် additive table/column operation များကိုသာ ရည်ရွယ်ထားပါသည်။ Destructive operation၊ PostgreSQL/MySQL adapter၊ distributed migration lock၊ rollback orchestration၊ connection pool နှင့် production deployment policy များသည် နောက်အဆင့်လုပ်ငန်းများ ဖြစ်ပါသည်။

## Authentication နှင့် authorization

Authentication သည် “ဒီသူဘယ်သူလဲ” ကို ဖြေပြီး authorization သည် “ဒီ identity ဘာလုပ်ခွင့်ရှိလဲ” ကို ဖြေပါသည်။ Zap Web တွင် နှစ်ခုကို ခွဲထားသင့်ပါသည်။ Host သို့မဟုတ် standard identity adapter က credential ကို verify လုပ်ပြီး verified identity object သာ application သို့ ပို့ရမည်။ Raw bearer token၊ cookie၊ password သို့မဟုတ် private key ကို arbitrary handler များထံ မပေးရ၊ log ထဲ မရေးရပါ။

Application contract တွင် user၊ group/role၊ scope/permission၊ session/token identity၊ reviewed password hashing၊ cookie session အတွက် CSRF policy၊ login throttling နှင့် audit event များ ပါသင့်ပါသည်။ Repository မခေါ်မီ authorization စစ်ရမည်ဖြစ်ပြီး admin route များသည် explicit administrative permission နှင့် secure session policy လိုအပ်ပါသည်။

လက်ရှိ scaffold သည် route table ထဲ scope ကို မှတ်တမ်းတင်ထားပြီး `web_serve` သည် အဆိုပါ field ကို metadata အဖြစ်သာ သုံးကာ authorization ကို အလိုအလျောက် မစစ်ပါ။ အရင် Web contract များက deterministic `401` နှင့် `403` decision ကို implement လုပ်ထားသော်လည်း real identity backend နှင့် session store သည် provider-specific အလုပ်ဖြစ်ပါသည်။ Protected operation များမပြုမီ application ကိုယ်တိုင် explicit authorization စစ်ရပါမည်။

## Admin လမ်းကြောင်း

Admin သည် internal model-centric management surface ဖြစ်ပါသည်။ Opt-in နှင့် explicit ဖြစ်ရမည်။ Database column အားလုံးကို အလိုအလျောက် မဖော်ပြရပါ။ Scaffold တွင် public fields နှင့် admin permissions ခွဲထားသော User registration ကို ပြထားပါသည်။

```zap
export fn admin_registry():
    return [{"model": "User", "list": ["id", "name", "email"], "permissions": ["admin:read", "admin:write"]}]
```

အနာဂတ် built-in admin package သည် public API နှင့် တူညီသော model/DTO/authorization boundary ကို သုံးသင့်ပါသည်။ Password hash၊ credential၊ internal flag နှင့် audit internals များကို default အားဖြင့် မထုတ်ရပါ။ Admin သည် product front end အစားထိုးမဟုတ်ပါ။

## Testing model

`zap test tests` သည် nested `*_test.zp` files များကို ရှာဖွေပြီး `zap.toml` ရှိသည့် အနီးဆုံး project root မှ import များကို resolve လုပ်ပါသည်။ ထို့ကြောင့် test directory ထဲ shared module များကို ထပ်ကူးရန် မလိုပါ။

Test layer များကို အောက်ပါအစီအစဉ်ဖြင့် တိုးသင့်ပါသည်။

| Layer | စစ်ဆေးရမည့် evidence |
|---|---|
| Language | parser၊ type၊ memory နှင့် deterministic runtime test |
| Contract | route၊ DTO၊ auth၊ rate-limit နှင့် migration metadata test |
| Handler | fake repository/identity inject လုပ်ထားသော request/response test |
| Database | isolated test database နှင့် rollback fixture ပါသော adapter test |
| HTTP | loopback end-to-end header၊ status၊ limit၊ shutdown test |
| Security | invalid input၊ credential leak၊ CSRF၊ SSRF၊ traversal၊ permission corpus |
| Operations | readiness၊ drain၊ restart၊ migration lock၊ log redaction၊ resource-boundary test |

Database အသုံးပြုသည့် test များသည် disposable isolated database သုံးရမည်။ Production credential သို့မဟုတ် production data ကို test runner ထဲ မသုံးရပါ။

## LSP synchronization

Native LSP သည် incremental document synchronization (`textDocumentSync.change = 2`) ကို ယခု advertise လုပ်ပါသည်။ `didChange` notification တစ်ခုတွင် sequential full-document သို့မဟုတ် range edit အများဆုံး 128 ခု ပါနိုင်သည်။ Range position များကို negotiated UTF-8၊ UTF-16 သို့မဟုတ် UTF-32 encoding အတိုင်း စစ်ဆေးပြီး character boundary ပေါ်တွင်သာ edit လုပ်ခွင့်ရှိသည်။ Document version များသည် အစဉ်တိုးရမည်ဖြစ်ပြီး 32 MiB workspace byte cap ကို edit တစ်ခုချင်းစီပြီးတိုင်း enforce လုပ်ပါသည်။ Malformed၊ out-of-range၊ oversized၊ မဖွင့်ထားသော document အတွက် range edit နှင့် stale update များကို stored document မပြောင်းဘဲ reject လုပ်ပါသည်။

၎င်းသည် bounded synchronization foundation ဖြစ်ပြီး complete IDE refactoring system မဟုတ်သေးပါ။ Cross-file semantic rename၊ project-wide dependency invalidation၊ incremental compilation၊ debugging နှင့် profiling များသည် သီးခြား milestone များအဖြစ် ဆက်ရှိပါသည်။

## CLI workflow

လက်ရှိ support လုပ်ထားသည့် workflow မှာ—

```bash
zap new shop
cd shop
zap check
zap web check
zap web routes
zap db check
zap db inspect --json
zap db plan
zap db migrate --dry-run
zap db migrate --check
zap db migrate
zap test tests
zap run main.zp
zap dev
```

`zap db inspect` သည် read-only adapter/status view ဖြစ်ပြီး SQLite file မရှိသေးလျှင် file အသစ် မဖန်တီးပါ။ `zap db migrate --check` သည် deployment တွင် အသုံးပြုနိုင်သော check ဖြစ်ပြီး migration ledger ကို validate လုပ်ကာ pending migration မရှိမှသာ success exit ပြန်ပေးပါသည်။ `--json` သုံးလျှင် automation အတွက် `ok: true` သို့မဟုတ် `ok: false` ပါဝင်ပါသည်။ `zap dev` သည် manifest ထဲက `server.zp` ကို run ပါသည်။ Scaffold ထဲရှိ server သည် `ZAP_WEB_PORT` ကိုဖတ်ပြီး မသတ်မှတ်လျှင် `3000` ကို အသုံးပြုပါသည်။ လက်ရှိ native server သည် loopback ပေါ်တွင် bounded HTTP/1.0 သို့မဟုတ် HTTP/1.1 request များကို လက်ခံပြီး exact path၊ `:parameter` နှင့် နောက်ဆုံး `*wildcard` segment များကို match လုပ်ကာ request map ကို Zap handler ထံ ပေးပြီး security header ပါသော framed response ပြန်ပေးပါသည်။ Generated Web scaffold သည် `public/index.html`, `public/assets/app.css` နှင့် `public/assets/app.js` ကို `web_static` ဖြင့် serve လုပ်ပြီး browser module က `/api/tasks` ကို ခေါ်ပါသည်။ Port ပြောင်းလိုပါက `ZAP_WEB_PORT=3100 zap dev` ဟု run နိုင်ပါသည်။ ၎င်းသည် single-threaded နှင့် blocking ဖြစ်သော development/reference server ဖြစ်သဖြင့် concurrency၊ cancellation၊ TLS/edge policy၊ readiness integration နှင့် production operation evidence များ မပြည့်မီ production server ဟု မဆိုရပါ။

နောက်ထပ် CLI command များကို semantics အမှန်နှင့် test evidence ရှိမှသာ ထည့်သင့်ပါသည်။ `zap web routes` သည် listener မစဘဲ resolved route table ကို ပြပြီး `zap explain route <path>` သည် handler မ execute ဘဲ concrete path match၊ extracted parameter နှင့် wildcard value များကို ယခုရှင်းပြပေးပါသည်။ ကျန်ရှိသော roadmap တွင် API documentation အတွက် `zap docs` နှင့် production config/security policy အတွက် `zap deploy preflight` တို့ ပါဝင်ပါသည်။ လက်ရှိ `zap db migrate` implementation သည် SQLite-first နှင့် additive operation များအတွက်သာ ဖြစ်ပြီး provider-neutral production migration system ဟု မယူဆရပါ။

`zap run main.zp` သည် contract preview အဖြစ် ဆက်ရှိနေပါမည်။ `zap dev` သည် ပထမဆုံး Zap-native HTTP execution path ဖြစ်ပြီး `host/zap-host` သည် operational Axum/Tower reference adapter ဖြစ်ပါသည်။ Production rule ပြည့်မီသည်အထိ နှစ်ခုလုံးကို complete production Web platform ဟု မဖော်ပြရပါ။

## Django နှင့် မတူသည့် Zap direction

Zap သည် project/app၊ URL declaration၊ model metadata၊ migration၊ auth၊ admin၊ test နှင့် CLI workflow တို့၏ ကောင်းမွန်သော full-stack idea ကို လက်ခံပါသည်။ သို့သော် implementation နှင့် language identity ကို မကူးပါ။

| အကြောင်းအရာ | Django မှ လေ့လာသည့် idea | Zap-native choice |
|---|---|---|
| Syntax | Python module၊ class၊ decorator | Plain Zap module အရင်၊ syntax အသစ်ကို parser/AST RFC ဖြင့်သာ ထည့်ခြင်း |
| Default | Convention over configuration | Inspectable manifest နှင့် route metadata ပါသော safe convention |
| ORM | Dynamic model/query API ကျယ်ပြန့်ခြင်း | Explicit DTO နှင့် adapter capability ပါသော typed boundary သေးသေး |
| Async | Sync/async ကို framework က အလိုအလျောက်ပြောင်းခြင်း | I/O boundary ကို ရှင်းလင်းစွာ ဖော်ပြခြင်း၊ blocking ကို မဖုံးကွယ်ခြင်း |
| Error | Exception ကို framework က HTTP error ပြောင်းခြင်း | Result/Option နှင့် centralized response mapping |
| Admin | Model-centric internal interface | Explicit registration၊ least-privilege field နှင့် permission policy |
| Deployment | External WSGI/ASGI server | Runtime/operation gate ပြည့်မှ native Zap server target |

## Production rule

Zap Web project တစ်ခုကို production-ready ဟု သတ်မှတ်နိုင်ရန် native runtime၊ HTTP server၊ database adapter၊ identity system၊ rate-limit store၊ migration၊ admin၊ observability၊ deployment နှင့် security evidence အားလုံးကို တစ်စုတစ်စည်း version-control နှင့် CI တွင် verify လုပ်ထားရမည်။ လက်ရှိ scaffold သည် ထို platform အတွက် **Zap-native project layer ပထမအဆင့်** ဖြစ်ပြီး complete production platform မဟုတ်သေးပါ။

## ကိုးကားချက်များ

[1]: https://docs.djangoproject.com/en/6.1/intro/tutorial01/ "Django first-app tutorial"
[2]: https://docs.djangoproject.com/en/6.1/topics/http/urls/ "Django URL dispatcher"
[3]: https://docs.djangoproject.com/en/6.1/topics/http/middleware/ "Django middleware"
[4]: https://docs.djangoproject.com/en/6.1/topics/db/models/ "Django models"
[5]: https://docs.djangoproject.com/en/6.1/topics/migrations/ "Django migrations"
[6]: https://docs.djangoproject.com/en/6.1/topics/auth/ "Django authentication"
[7]: https://docs.djangoproject.com/en/6.1/ref/contrib/admin/ "Django admin"
[8]: https://docs.djangoproject.com/en/6.1/topics/testing/overview/ "Django testing"
