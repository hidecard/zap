# Zap Production Database Operations

Zap တွင် လက်ရှိ deterministic SQLite-first migration engine ပါဝင်ပါသည်။ Production application တစ်ခုတွင် schema migration၊ request-time database access နှင့် connection-pool lifecycle တို့ကို တာဝန်ခွဲထားရမည်။ Migration command သည် release operation ဖြစ်ရမည်။ Repository က pool ကို ပိုင်ရမည်။ Web process က readiness နှင့် graceful shutdown ကို ပိုင်ရမည်။

## Native adapter က ယနေ့ ပိုင်ဆိုင်ထားသောအရာများ

Native adapter သည် `[database]` manifest section ကို ဖတ်ပြီး `driver = "sqlite"` ကို support လုပ်ပါသည်။ Relative project database path သို့မဟုတ် deployment မှ ပေးသော `ZAP_DATABASE_URL` ကို resolve လုပ်သည်။ Declarative `.zp` migrations များကို ရှာဖွေပြီး dependency order၊ checksum နှင့် pending plan ကို validate လုပ်ကာ SQLite transaction တစ်ခုအတွင်း apply လုပ်ပါသည်။ `__zap_migrations` ledger တွင် migration ID၊ applied time နှင့် checksum ကို သိမ်းပါသည်။ Applied migration ပျောက်သွားခြင်း သို့မဟုတ် content ပြောင်းသွားခြင်းရှိပါက fail closed လုပ်ပြီး migration အသစ်ဖန်တီးရန် တောင်းဆိုပါသည်။

| Concern | Native Zap behavior | Production implication |
|---|---|---|
| Migration format | Bounded operation ပါသော declarative `.zp` file | Migration ကို release artifact အဖြစ် review/test လုပ်ရမည် |
| Ordering | Explicit `depends_on` နှင့် deterministic order | Cycle နှင့် missing dependency ကို apply မလုပ်မီ reject လုပ်မည် |
| Drift | SHA-256 checksum ledger | Applied migration ကို မပြင်ဘဲ migration အသစ်ရေးရမည် |
| Apply | Transaction တစ်ခုအတွင်း SQLite apply | Release မတိုင်မီ backup လုပ်ပြီး apply ပြီးနောက် verify လုပ်ရမည် |
| Rollback | Automatic down migration မရှိ | Backup/restore သို့မဟုတ် tested forward migration သုံးရမည် |
| External provider | Native adapter တွင် မပါသေး | Provider-specific host repository နှင့် migration tool ထည့်ရမည် |

## Release migration procedure

Release pipeline သည် immutable application artifact ကို အရင် build လုပ်ရမည်။ ထို့နောက် database ကို မထိမီ project validation နှင့် read-only migration plan ကို ထုတ်ရမည်။

```bash
zap build --locked /srv/zap/app
zap web check /srv/zap/app
zap db check /srv/zap/app
zap db inspect --json /srv/zap/app
zap db plan --json /srv/zap/app
zap db migrate --dry-run /srv/zap/app
```

လက်ရှိ SQLite adapter အတွက် Web process ကို ရပ်ပြီး database copy ကို verify လုပ်ကာ checked-in `zap-web-migrate.service` ကို invoke လုပ်ပါ။ ထို unit သည် `flock` ဖြင့် migration ကို serialize လုပ်ပြီး transaction apply လုပ်ကာ ပြီးဆုံးလျှင် `zap db migrate --check` ကို run လုပ်ပါသည်။ Migration unit ကို worker boot တိုင်းတွင် auto-run မလုပ်ဘဲ manual release operation အဖြစ် ထားပါ။

Migration သည် rollout အတွင်း old schema နှင့် new schema နှစ်မျိုးလုံးကို application အသစ်က ကိုင်တွယ်နိုင်မှ safe ဖြစ်ပါသည်။ Expand-and-contract pattern ကို ဦးစားပေးပါ။ Column/table အသစ်ကို အရင်ထည့်၊ shape နှစ်မျိုးလုံးဖတ်နိုင်သော code deploy လုပ်၊ bounded backfill run လုပ်ပြီး old reader အားလုံး ပျောက်ပြီးမှ old column ဖယ်ပါ။ Native migration format သည် general SQL migration system ထက် ရည်ရွယ်ချက်ရှိရှိ သေးငယ်ပါသည်။ ထို့ကြောင့် destructive သို့မဟုတ် provider-specific assumption မထည့်ပါနှင့်။

## Failure နှင့် recovery

`zap db migrate --check` က pending migration ပြပါက release မready ဖြစ်ပါသည်။ Apply fail ဖြစ်ပါက journal စစ်ပြီး database ကို မဖျက်မီ preserve လုပ်ပါ။ Migration ledger က checksum drift ပြပါက repository history ဖြင့် မူလ file မှန်ကြောင်း သေချာမှသာ ပြန်ထားပါ။ မဟုတ်ပါက migration အသစ်ရေးပါ။ Destructive change apply ပြီးသားဖြစ်ပါက tested backup/restore သို့မဟုတ် forward corrective migration သုံးပါ။ systemd restart အောင်မြင်ခြင်းကို schema recovery evidence ဟု မယူဆရပါ။

SQLite တွင် writer တစ်ချိန်တည်းသာ ရှိစေပြီး transaction များကို တိုတောင်းစွာ ထားပါ။ Native adapter သည် opened connection တိုင်းတွင် bounded busy timeout နှင့် foreign-key enforcement ကို သတ်မှတ်ထားပါသည်။ Connection pool ကြီးခြင်းသည် SQLite write capacity ကို parallel မဖြစ်စေဘဲ lock contention နှင့် file-descriptor pressure တိုးစေနိုင်ပါသည်။ Read/write behavior၊ backup strategy၊ WAL policy နှင့် filesystem durability ကို တကယ့် host အခြေအနေအလိုက် review လုပ်ရမည်။

## Connection-pool ownership

Web framework သည် database credential၊ SQL သို့မဟုတ် provider-specific pool object များကို Zap source ထဲ မထည့်ရပါ။ Production `UserRepository` က provider pool ကို ပိုင်ဆိုင်ပြီး `WebGateway` သို့ typed operation များ ပေးရမည်။ Host adapter တွင် `AppConfig.database_pool` မှတစ်ဆင့် အောက်ပါ policy configuration ကို ထည့်သွင်းထားပါသည်။

| Setting | Environment variable | Default | Bound |
|---|---|---:|---:|
| Maximum connections | `ZAP_DB_MAX_CONNECTIONS` | 16 | 1–256 |
| Acquisition timeout | `ZAP_DB_ACQUIRE_TIMEOUT_MS` | 1000 ms | 1 ms–30 s |
| Query/statement timeout | `ZAP_DB_QUERY_TIMEOUT_MS` | 5000 ms | 1 ms–120 s |

ဤ fields များသည် contract သတ်မှတ်ခြင်းဖြစ်ပြီး `DemoRepository` ကို real pool အဖြစ် မပြောင်းပေးပါ။ Real repository သည် connection acquire နှင့် statement execute အချိန်တွင် ထို values များကို အသုံးပြုရမည်။ Acquire/query limit ကျော်လွန်ပါက typed unavailable/internal error ပြန်ပေးရမည်။

Pool sizing သည် language constant မဟုတ်ဘဲ deployment calculation ဖြစ်ပါသည်။ Application instance အားလုံး၏ pool maximum ပေါင်းလဒ်သည် database server connection budget အောက်တွင် ရှိရမည်။ Administration၊ migration၊ monitoring နှင့် failover အတွက် connection reserve ချန်ထားပါ။ Provider ၏ connection setting နှင့် limit များသည် အဆုံးသတ် authority ဖြစ်ပါသည် [1]။ အစတွင် conservative value သုံးပြီး pool wait time နှင့် database saturation ကိုတိုင်းကာ tune လုပ်ပါ။ Pool သည် limit မရှိသော queue မဟုတ်ပါ။ Acquire timeout၊ query timeout နှင့် shutdown အားလုံးကို bounded လုပ်ရမည်။ Shutdown အချိန်တွင် acquire အသစ်ပိတ်ပြီး in-flight work ပြီးဆုံးရန် စောင့်ရမည်။

## Repository transaction contract

Production repository boundary သည် အောက်ပါ flow ဖြစ်သင့်ပါသည်။

```text
request
  -> authenticate and authorize
  -> deadline ဖြင့် pool connection acquire
  -> statement အများစု atomic ဖြစ်ရန်လိုမှ transaction begin
  -> parameterized query နှင့် subject/tenant predicate သုံး
  -> commit သို့မဟုတ် rollback
  -> connection release
  -> provider error ကို typed DatabaseError ပြောင်း
```

Repository သည် raw driver error၊ SQL text၊ credential၊ password material သို့မဟုတ် internal column များကို JSON DTO layer သို့ မပေးရပါ။ Duplicate-key ကို stable conflict result၊ unavailable pool/database ကို dependency-unavailable result အဖြစ် map လုပ်ရမည်။ Cancellation ဖြစ်လျှင် connection release လုပ်ရမည်။ Readiness သည် bounded database ping သို့မဟုတ် equivalent health check လုပ်သင့်ပြီး liveness သည် database နှင့် သီးခြားရှိရမည်။

External PostgreSQL/MySQL provider များအတွက် reviewed async pool implementation ကို host adapter ထဲတွင် သုံးပါ။ Migration ကို provider migration tool သို့မဟုတ် သီးခြား review ပြီးသော Zap adapter တွင် ထားပါ။ SQLite `.zp` migration engine က အခြား provider SQL dialect ကို မသိမသာ interpret လုပ်သည်ဟု မယူဆရပါ။ Web instance များစွာရှိပါက local `/run/zap` lock မဟုတ်ဘဲ database advisory migration lock သို့မဟုတ် orchestrator lock သုံးရမည်။

## Shutdown နှင့် observability

SIGTERM ရသောအခါ process သည် draining state ဝင်ကာ readiness fail ဖြစ်စေရမည်။ Application work အသစ်ကို ပိတ်၊ outstanding query များကို cancel/timeout လုပ်၊ configured drain budget အတွင်း pool close လုပ်ပြီး deterministic status ဖြင့် exit လုပ်ရမည်။ Pool acquisition latency၊ active/idle count၊ timeout count၊ migration ID/checksum နှင့် database error category များကို မှတ်တမ်းတင်နိုင်သော်လည်း credential ပါသော URL သို့မဟုတ် raw SQL value များကို မမှတ်တမ်းတင်ရပါ။

## ကိုးကားချက်များ

[1]: https://www.postgresql.org/docs/current/runtime-config-connection.html PostgreSQL documentation — connection and authentication configuration.
[2]: https://documentation.suse.com/smart/security/html/systemd-securing/index.html SUSE Linux Enterprise Server — Securing systemd Services.
[3]: https://docs.nginx.com/nginx/admin-guide/web-server/reverse-proxy/ NGINX — Reverse Proxy Administration Guide.
