# Zap Web Production Deployment Runbook

ဤ runbook သည် Ubuntu host ပေါ်တွင် Zap-native Web application တစ်ခုကို production deploy လုပ်သည့်အခါ အသုံးပြုရမည့် boundary ကို ဖော်ပြပါသည်။ Checked-in service configuration သည် Zap ကို loopback တွင်သာ bind လုပ်စေပြီး public TLS boundary ကို Nginx ထံပေးထားသည်။ Database migration ကို Web process နှင့် ခွဲထားသော manual systemd unit ဖြင့် run လုပ်ပါသည်။ systemd restriction များသည် process limit တစ်ခုတည်းကို complete sandbox ဟု မယူဆဘဲ service ၏ filesystem နှင့် kernel authority ကို လျှော့ချသည့် principle ကို လိုက်နာပါသည် [1]။ Domain၊ certificate path၊ filesystem path၊ identity provider၊ database provider နှင့် resource limit များကို deployment review ပြီးမှ ပြောင်းပါ။

## Deployment topology

| Layer | တာဝန် | Public access |
|---|---|---:|
| Nginx | TLS termination၊ HTTP-to-HTTPS redirect၊ request size/timeout၊ forwarded header နှင့် method allowlist | ရှိသည်၊ port 80/443 |
| `zap-web.service` | Zap project validation နှင့် Web server process | မရှိပါ၊ loopback `127.0.0.1:3000` |
| `zap-web-migrate.service` | `flock` exclusive lock အောက်တွင် တစ်ကြိမ် migration apply လုပ်ခြင်း | မရှိပါ |
| Database | လက်ရှိ native adapter အတွက် SQLite file သို့မဟုတ် နောင် provider adapter | Direct public access မရှိပါ |

Browser နှင့် frontend build toolchain များသည် runtime service ၏ အစိတ်အပိုင်းမဟုတ်ပါ။ Deploy artifact တွင် Zap executable၊ Zap project နှင့် build ပြီးသော `public/` tree သာ လိုအပ်ပါသည်။ React၊ Vue၊ Svelte သို့မဟုတ် အခြား JavaScript compiler သည် build-time dependency သာ ဖြစ်ပါသည်။

## Framework branch မှ ပေးထားသော files

```text
deploy/zap-web.service
deploy/zap-web-migrate.service
deploy/zap-web.nginx.conf
deploy/zap-web.env.example
deploy/zap-web-deployment-policy.toml
scripts/validate_zap_web_deployment.sh
```

Repository gate ကို artifact မကူးမီ run ပါ။

```bash
./scripts/validate_zap_web_deployment.sh
```

## Host ပြင်ဆင်ခြင်း

Dedicated unprivileged account နှင့် directory များကို ဖန်တီးပါ။ Application ကို root အဖြစ် မ run ပါနှင့်။ Populated secret environment file ကို repository ထဲ မထားပါနှင့်။

```bash
sudo useradd --system --user-group --home /srv/zap --shell /usr/sbin/nologin zap
sudo install -d -o zap -g zap -m 0750 /srv/zap/app
sudo install -d -o zap -g zap -m 0700 /srv/zap/app/data
sudo install -d -o root -g root -m 0755 /etc/zap/tls
sudo install -d -o root -g root -m 0755 /etc/zap
sudo install -m 0755 bin/zap /usr/local/bin/zap
sudo install -m 0644 deploy/zap-web.service /etc/systemd/system/zap-web.service
sudo install -m 0644 deploy/zap-web-migrate.service /etc/systemd/system/zap-web-migrate.service
sudo install -m 0644 deploy/zap-web.nginx.conf /etc/nginx/sites-available/zap-web.conf
sudo ln -sfn /etc/nginx/sites-available/zap-web.conf /etc/nginx/sites-enabled/zap-web.conf
```

Application artifact ကို `/srv/zap/app` သို့ ownership နှင့် mode ထိန်းသိမ်းကာ ကူးပါ။ Service တွင် `ProtectSystem=strict` နှင့် `/srv/zap/app/data` အောက်တွင်သာ write ခွင့် ရှိပါသည်။ External provider ကြောင့် အခြား writable directory လိုပါက service policy နှင့် deployment review နှစ်ခုလုံးကို ပြင်ရမည်ဖြစ်ပြီး filesystem access ကို မသိမသာ မချဲ့ရပါ။

Secret/configuration management မှတစ်ဆင့် environment file ဖန်တီးပါ။

```bash
sudo install -o root -g zap -m 0640 deploy/zap-web.env.example /etc/zap/zap-web.env
sudoedit /etc/zap/zap-web.env
```

Template ထဲတွင် `ZAP_DB_MAX_CONNECTIONS`၊ `ZAP_DB_ACQUIRE_TIMEOUT_MS` နှင့် `ZAP_DB_QUERY_TIMEOUT_MS` ပါရှိပါသည်။ ၎င်းတို့သည် production host adapter အတွက် policy input များဖြစ်ပြီး demo `zap-host` executable ထဲတွင် database pool ကို အလိုအလျောက် မဖန်တီးပေးပါ။ Real repository က ထို settings များကို အသုံးပြုရမည်။

## Nginx နှင့် TLS

`/etc/nginx/sites-enabled/zap-web.conf` ထဲတွင် `app.example.com`၊ certificate path နှင့် private-key path ကို ပြင်ပါ။ Nginx reverse-proxy forwarding နှင့် HTTPS listener behavior သည် Nginx ၏ proxy/TLS configuration model ကို လိုက်နာပါသည် [2] [3]။ Nginx template သည် port 80 မှ HTTPS သို့ redirect လုပ်ပြီး GET/POST သာ ခွင့်ပြုသည်။ Request body ကို 64 KiB၊ client timeout နှင့် upstream timeout များကို ကန့်သတ်ထားပြီး original host၊ protocol နှင့် client chain ကို loopback upstream သို့ forward လုပ်ပါသည်။ Upstream ကို public address တွင် မဖွင့်ရပါ။

Reload မလုပ်မီ validate ပါ။

```bash
sudo nginx -t
sudo systemctl reload nginx
curl -fsS https://app.example.com/health
curl -i https://app.example.com/ready
```

`/health` သည် liveness signal ဖြစ်ပါသည်။ `/ready` သည် dependency/readiness signal ဖြစ်ပြီး process အသက်ရှင်နေသော်လည်း traffic လက်ခံရန် မသင့်သေးလျှင် `503` ပြန်နိုင်ပါသည်။ Load balancer သည် `/ready` fail ဖြစ်သော instance ကို rotation မှ ဖယ်ရမည်။ `/health` အောင်မြင်ခြင်းကို database အသုံးပြုနိုင်သည်ဟု မယူဆရပါ။

TLS certificate နှင့် private key များကို Git အပြင်ဘက်တွင် provision လုပ်ပါ။ Private-key file ကို root-only mode ထားပြီး certificate management process ဖြင့် rotate လုပ်ပါ။ `nginx -t` အောင်မြင်မှသာ reload လုပ်ပါ။ Template ထဲက HSTS header သည် domain ကို အမြဲ HTTPS သုံးနိုင်သောအခြေအနေတွင်သာ သင့်တော်ပါသည်။

## Migration-first rollout

Deployment သည် database ကို မထိမီ artifact ကို validate လုပ်ရမည်။ အကြံပြုထားသော order မှာ အောက်ပါအတိုင်း ဖြစ်ပါသည်။

```bash
sudo systemctl daemon-reload
sudo systemctl stop zap-web.service
sudo -u zap /usr/local/bin/zap build --locked /srv/zap/app
sudo -u zap /usr/local/bin/zap web check /srv/zap/app
sudo -u zap /usr/local/bin/zap db check /srv/zap/app
sudo -u zap /usr/local/bin/zap db plan /srv/zap/app
sudo -u zap /usr/local/bin/zap db migrate --dry-run /srv/zap/app
```

SQLite adapter အတွက် production migration မလုပ်မီ database backup ကို verify လုပ်ပြီး သိမ်းပါ။

```bash
sudo install -o zap -g zap -m 0600 /srv/zap/app/data/zap.sqlite3 \
  "/srv/zap/app/data/zap.sqlite3.$(date -u +%Y%m%dT%H%M%SZ).bak"
sudo systemctl start zap-web-migrate.service
sudo systemctl status zap-web-migrate.service --no-pager
sudo -u zap /usr/local/bin/zap db migrate --check /srv/zap/app
sudo systemctl start zap-web.service
sudo systemctl is-active --quiet zap-web.service
```

Migration unit သည် `/usr/bin/flock -n /run/zap/zap-web-migrate.lock` အောက်တွင် apply လုပ်ပါသည်။ Host တစ်ခုတည်းပေါ်တွင် deploy operator နှစ်ဦးက migration တစ်ချိန်တည်း apply လုပ်ခြင်းကို ကာကွယ်ပေးပါသည်။ Multi-host deployment တွင် provider advisory lock သို့မဟုတ် deployment orchestrator lock သုံးရမည်။ Local filesystem lock သည် distributed lock မဟုတ်ပါ။

Migration unit ကို Web process start တိုင်းတွင် အလိုအလျောက် run ရန် unconditional dependency အဖြစ် မ enable ပါနှင့်။ Migration သည် release operation ဖြစ်ပြီး worker တစ်ခုချင်း boot hook မဟုတ်ပါ။ Migration fail ဖြစ်ပါက Web service ကို stopped ထားပြီး journal စစ်ပါ။ လိုအပ်လျှင် verified backup ကို restore လုပ်ကာ forward corrective migration အသစ်ဖြင့် ပြန် deploy ပါ။ Applied migration file များသည် checksum ဖြင့် ကာကွယ်ထားသဖြင့် အဟောင်းကို မပြင်ဘဲ migration အသစ်ဖန်တီးပါ။

## Service စတင်ခြင်းနှင့် စောင့်ကြည့်ခြင်း

```bash
sudo systemctl enable --now zap-web.service
sudo systemctl status zap-web.service --no-pager
sudo journalctl -u zap-web.service -n 100 --no-pager
sudo ss -ltnp | grep ':3000'
curl -fsS http://127.0.0.1:3000/health
```

မျှော်မှန်းထားသော socket သည် loopback-only ဖြစ်ရမည်။ Process က public address တွင် listen လုပ်နေပါက deployment ကို ရပ်ပြီး service/environment configuration ကို ပြင်ပြီးမှ Nginx ဖြင့် expose လုပ်ပါ။ Rollout အတွင်း readiness ဖြင့် traffic အသစ်ပိတ်၊ drain period စောင့်ပြီးမှ process အဟောင်းကို terminate လုပ်ပါ။ ပုံမှန် deployment တွင် `kill -9` မသုံးရပါ။

## Rollback boundary

Framework migration format သည် transactional၊ checksum-protected SQLite apply ကို ပေးထားသော်လည်း automatic down migration မပေးသေးပါ။ Application rollback နှင့် schema rollback သည် သီးခြားဆုံးဖြတ်ရမည့်အရာများ ဖြစ်ပါသည်။ Additive schema ပြောင်းလဲမှုဖြင့် backward-compatible application release ကို rollback လုပ်နိုင်တတ်သော်လည်း destructive schema change အတွက် tested backup/restore သို့မဟုတ် forward compatibility migration လိုပါသည်။ systemd restart အောင်မြင်ရုံဖြင့် rollback safe ဖြစ်သည်ဟု မဆိုရပါ။

## Production limitations

Checked-in `zap-web` units များသည် demo `zap-host` authenticator သို့မဟုတ် memory repository ကို production identity/persistence အဖြစ် မပြောင်းလဲပေးပါ။ Production deployment အတွက် real authenticator၊ real repository/pool implementation၊ multiple instance အတွက် shared rate-limit state၊ secret-redacted observability၊ provider-specific egress controls၊ certificate automation၊ backup verification နှင့် load/chaos evidence များ လိုအပ်နေသေးပါသည်။

## ကိုးကားချက်များ

[1]: https://documentation.suse.com/smart/security/html/systemd-securing/index.html SUSE Linux Enterprise Server — Securing systemd Services.
[2]: https://docs.nginx.com/nginx/admin-guide/web-server/reverse-proxy/ NGINX — Reverse Proxy Administration Guide.
[3]: https://nginx.org/en/docs/http/configuring_https_servers.html NGINX — Configuring HTTPS Servers.
