# Zap Production Operations Runbook

**စစ်ဆေးထားသော baseline:** Zap v2.11.5 development line

**အကျယ်အဝန်း:** ဤ runbook သည် built-in authenticated registry ကို Linux systemd service နှင့် nginx TLS ingress အနောက်တွင် production သုံးရန် reference deployment အဖြစ် သတ်မှတ်ထားသည်။ Cloud provisioning အလိုအလျောက်လုပ်ပေးသည့်စနစ် မဟုတ်သောကြောင့် firewall၊ certificate authority၊ secret manager၊ monitoring၊ backup နှင့် approval အဆင့်များကို မိမိ environment နှင့် ကိုက်ညီအောင် operator က ဖြည့်စွက်ရမည်။

> **မဖြစ်မနေလိုက်နာရန်:** `zap registry serve` ကို Internet သို့ တိုက်ရိုက် မဖွင့်ပါနှင့်။ Backend ကို loopback တွင်သာ bind လုပ်ပြီး TLS ကို ထိန်းသိမ်းထားသော ingress proxy တွင် terminate လုပ်ပါ။ Backend ၏ filesystem နှင့် network permission များကို တင်းကျပ်ထားပါ။

## ၁။ Architecture နှင့် လိုအပ်သော component များ

Reference deployment တွင် release binary၊ `127.0.0.1:8787` တွင် bind လုပ်သော systemd backend၊ public entry point တစ်ခုတည်းဖြစ်သော nginx TLS virtual host နှင့် external secret/monitoring/backup system ပါဝင်သည်။

| Component | Production control |
|---|---|
| Zap binary | ယုံကြည်ရသော release မှ download လုပ်ပြီး SHA-256 စစ်၊ version pin လုပ်၊ rollback အတွက် ယခင် binary ကို ထားရန် |
| Registry data | `/var/lib/zap-registry` အောက်တွင်ထား၊ သီးခြား backup လုပ်ပြီး restore စမ်းရန် |
| Service | `deploy/zap-registry.service` ထဲရှိ DynamicUser၊ StateDirectory၊ quota၊ loopback binding နှင့် process-group cleanup ကို အသုံးပြုရန် |
| Ingress | `deploy/zap-registry.nginx.conf` ကို အခြေခံပြီး hostname နှင့် certificate path အစားထိုးရန် |
| Credential | `ZAP_REGISTRY_TOKEN` နှင့် `ZAP_REGISTRY_SIGNING_SECRET` ကို secret manager သို့မဟုတ် mode-0600 env file မှ inject လုပ်ရန် |
| Monitoring | systemd journal၊ nginx log၊ host resource metrics နှင့် external health check စုဆောင်းရန် |
| Backup/recovery | Registry data backup၊ signing-key material policy နှင့် production မတိုင်မီ restore drill ပြုလုပ်ရန် |

Backend တွင် request worker ၈ ခုနှင့် bounded queue ၃၂ ခု ပါဝင်သည်။ Queue ပြည့်ပါက work ကို အကန့်အသတ်မဲ့ လက်မခံဘဲ `503 Service Unavailable` ပြန်ပေးသည်။ Local probe အတွက် authentication မလိုသော `GET /healthz` နှင့် `GET /readyz` ပါရှိပြီး nginx သည် ထို path များကို loopback မှသာ ခွင့်ပြုသည်။

## ၂။ Host ပြင်ဆင်ခြင်း

Dedicated Linux host သို့မဟုတ် VM ကို အသုံးပြုပါ။ OS security update ပြုလုပ်ပါ၊ administrative access ကန့်သတ်ပါ၊ system time synchronize လုပ်ပါ၊ service မစမီ firewall သတ်မှတ်ပါ။ Ingress လိုအပ်သော TLS port၊ ပုံမှန်အားဖြင့် TCP 443 ကိုသာ expose လုပ်ပြီး TCP 8787 ကို public မဖွင့်ပါနှင့်။

```bash
sudo apt-get update
sudo apt-get install --yes nginx curl ca-certificates
sudo install -d -m 0750 /etc/zap
sudo install -d -m 0755 /var/lib/zap-registry
```

Systemd unit သည် `StateDirectory=zap-registry` ဖြင့် final state directory ကို စီမံသောကြောင့် systemd setup ကို အသုံးပြုပါ။ Secret နှင့် private key များကို repository checkout သို့မဟုတ် `/var/lib/zap-registry` ထဲ မထားပါနှင့်။

## ၃။ Binary install နှင့် verify

Approved channel မှ release archive နှင့် checksum ကို download လုပ်ပါ။ Extract/install မလုပ်မီ checksum စစ်ပါ။

```bash
sha256sum -c zap-<version>-linux-x86_64.tar.gz.sha256
```

Version အသစ်သည် health/smoke check အောင်မြင်သည်အထိ ယခင် binary ကို မဖျက်ပါနှင့်။

```bash
install -d -m 0755 /usr/local/bin
install -m 0755 ./bin/zap /usr/local/bin/zap.new
/usr/local/bin/zap.new --version
mv /usr/local/bin/zap.new /usr/local/bin/zap
/usr/local/bin/zap --version
```

Source build လုပ်ပါက repository ၏ pinned Rust toolchain နှင့် locked dependency graph ကို အသုံးပြုပါ။

```bash
cargo build --release --locked --manifest-path native/Cargo.toml
install -m 0755 native/target/release/zap /usr/local/bin/zap.new
```

Production release သည် reviewed/signed release process မှလာသင့်သည်။ Repository-side RustSec နှင့် provenance checks များသည် downloaded artifact ကို operator ကိုယ်တိုင် verify လုပ်ရန် အစားထိုးမဟုတ်ပါ။

## ၄။ Secret များကို လုံခြုံစွာ configure လုပ်ခြင်း

Deployment secret manager ကို အသုံးပြုပြီး `/etc/zap/registry.env` ဖန်တီးပါ။ Real value ပါသော file ကို repository ထဲသို့ မတင်ပါနှင့်။

```bash
sudo install -m 0600 /dev/null /etc/zap/registry.env
sudoedit /etc/zap/registry.env
```

လိုအပ်သော variables—

```text
ZAP_REGISTRY_TOKEN=generated-service-token
ZAP_REGISTRY_SIGNING_SECRET=generated-signing-secret
```

High-entropy value များကို သီးခြားစီ generate လုပ်ပါ။ Bearer token သည် service request authentication အတွက်ဖြစ်ပြီး signing secret သည် persisted signed index ကို ကာကွယ်သည်။ Signing secret ပြောင်းလဲခြင်းသည် ရှိပြီးသား signed metadata ကို invalidate လုပ်နိုင်သောကြောင့် migration၊ backup၊ overlap နှင့် rollback procedure ပါသော သီးခြား approval လိုအပ်သည်။ Token သို့မဟုတ် signing secret ကို systemd command line၊ nginx config၊ source၊ manifest၊ lockfile၊ log သို့မဟုတ် chat message ထဲ မထည့်ပါနှင့်။

## ၅။ Systemd install နှင့် validate

Reviewed unit ကို install ပြီး exact contents validate လုပ်ပါ။

```bash
sudo install -m 0644 deploy/zap-registry.service /etc/systemd/system/zap-registry.service
scripts/validate_registry_deployment.sh
sudo systemd-analyze verify /etc/systemd/system/zap-registry.service
sudo systemctl daemon-reload
sudo systemctl enable --now zap-registry.service
```

Command argument order ကို အထူးသတိပြုပါ—

```text
/usr/local/bin/zap registry serve /var/lib/zap-registry 127.0.0.1:8787
```

Registry root သည် ပထမ argument ဖြစ်ပြီး optional bind address သည် ဒုတိယ argument ဖြစ်သည်။

```bash
sudo systemctl status --no-pager zap-registry
sudo ss -ltnp | grep ':8787'
```

Unit တွင် DynamicUser၊ StateDirectory၊ protected system/home path၊ NoNewPrivileges၊ empty capability set၊ quota၊ loopback-only address policy နှင့် `KillMode=control-group` ပါဝင်သည်။ Application issue ဖြေရှင်းရန် security control ကို မဖယ်ရှားပါနှင့်။ မဖြစ်မနေပြင်ရပါက သီးခြား security review မှတ်တမ်းတင်ပါ။

## ၆။ nginx TLS ingress configure လုပ်ခြင်း

Reference config ကို copy လုပ်ပြီး example hostname နှင့် certificate path များကို organization ၏ certificate system မှ ထုတ်ထားသော certificate ဖြင့် အစားထိုးပါ။

```bash
sudo install -m 0644 deploy/zap-registry.nginx.conf /etc/nginx/conf.d/zap-registry.conf
sudoedit /etc/nginx/conf.d/zap-registry.conf
sudo nginx -t
sudo systemctl reload nginx
```

Reference proxy သည် HTTP ကို HTTPS သို့ redirect လုပ်သည်၊ TLS 1.2/1.3 သုံးသည်၊ body/request size ကန့်သတ်သည်၊ `GET` နှင့် `POST` ကိုသာ ခွင့်ပြုသည်၊ proxy timeout သတ်မှတ်သည်၊ loopback `127.0.0.1:8787` သို့ forward လုပ်သည်။ High-volume public service အတွက် organization-approved rate limit၊ WAF၊ logging နှင့် upstream access policy များကို ထပ်ထည့်ပါ။

`/healthz` နှင့် `/readyz` သည် `127.0.0.1` နှင့် `::1` ကိုသာ ခွင့်ပြုသည်။ Load balancer က probe လုပ်ရန်လိုပါက fixed source CIDR တစ်ခုတည်းကိုသာ allow ပြုလုပ်ပါ။ TCP 8787 ကို host/network firewall တွင် ပိတ်ထားပါ။

## ၇။ Public traffic မတိုင်မီ smoke test

Liveness၊ readiness၊ log နှင့် public TLS endpoint ကို အစဉ်လိုက် စစ်ပါ။

```bash
curl --fail http://127.0.0.1:8787/healthz
curl --fail http://127.0.0.1:8787/readyz
sudo journalctl -u zap-registry --since '5 minutes ago' --no-pager
sudo nginx -t
curl --fail --silent --show-error https://registry.example/healthz
```

Public health request ကို nginx policy အရ ခွင့်ပြုထားမှသာ အောင်မြင်ရမည်။ Disposable fixture ဖြင့် authentication နှင့် method restriction စစ်ပါ။ Unauthenticated publish သည် `401` ပြန်ရမည်၊ traversal package identity ကို reject လုပ်ရမည်၊ checksum မမှန်လျှင် persistence မလုပ်မီ reject လုပ်ရမည်၊ signed index ကို ပြန်ဖတ်နိုင်ရမည်။

## ၈။ Client configuration နှင့် package workflow

Developer/CI machine တွင် exact HTTPS origin ကို trust လုပ်ပြီး credential ကို environment-variable reference ဖြင့် configure လုပ်ပါ။

```bash
zap registry trust add https://registry.example/team
export ZAP_REGISTRY_TOKEN_CI="$(secret-manager read zap/registry/read-token)"
zap registry credential set https://registry.example/team --token-env ZAP_REGISTRY_TOKEN_CI
zap install --locked .
```

Emergency network freeze သို့မဟုတ် offline deployment အတွက်—

```bash
ZAP_OFFLINE=1 zap install --locked .
```

Offline mode သည် cache ထဲတွင် ရှိပြီး checksum မှန်သော artifact များကိုသာ သုံးရမည်။ Publish မလုပ်မီ package ကို format၊ lint၊ check၊ test နှင့် locked build လုပ်ပါ။

```bash
zap fmt main.zp
zap lint main.zp
zap check .
zap test --fail-fast .
zap build --locked .
```

Publish မလုပ်မီ archive checksum တွက်ပါ။

```bash
checksum="$(sha256sum ./demo.pkg | awk '{print $1}')"
export ZAP_REGISTRY_TOKEN="$(secret-manager read zap/registry/publish-token)"
zap registry publish https://registry.example/team/publish ./demo.pkg demo 1.0.0 "$checksum"
```

Index ထဲရှိ package identity၊ version၊ provenance နှင့် checksum ကို ပြန်စစ်ပါ။ Approved registry policy မရှိဘဲ released version ကို overwrite မလုပ်ပါနှင့်။

## ၉။ Monitoring နှင့် alerting

Zap တွင် built-in Prometheus metrics သို့မဟုတ် durable job queue မရှိပါ။ အနည်းဆုံး systemd service state၊ restart count၊ CPU/memory/task/file descriptor pressure၊ nginx 4xx/5xx rate၊ TLS certificate expiry၊ registry disk usage၊ health/readiness failure နှင့် authentication failure များကို monitor လုပ်ပါ။

Repeated restart၊ readiness failure၊ sustained `503`၊ disk threshold ကျော်ခြင်း၊ certificate expiry နီးကပ်ခြင်း၊ registry data directory မမျှော်လင့်ဘဲ ပြောင်းလဲခြင်းနှင့် unauthorized/forbidden response တိုးလာခြင်းများအတွက် alert ထားပါ။ Log များကို secret redaction နှင့် retention policy ပါသော central system သို့ စုဆောင်းပါ။

## ၁၀။ Backup၊ restore နှင့် rollback

`/var/lib/zap-registry` ကို approved encrypted backup system ဖြင့် backup လုပ်ပါ။ Environment file နှင့် signing-secret backup ကို သီးခြား access control ဖြင့် ထားပါ။ Restore စမ်းသပ်ပြီးမှ backup ပြည့်စုံသည်ဟု သတ်မှတ်ပါ။

```bash
sudo systemctl stop zap-registry
sudo tar --xattrs --acls -czf /secure-backup/zap-registry-$(date -u +%Y%m%dT%H%M%SZ).tar.gz /var/lib/zap-registry
sudo systemctl start zap-registry
sudo curl --fail http://127.0.0.1:8787/readyz
```

Restore လုပ်ရာတွင် service ကို stop လုပ်၊ staging directory သို့ restore လုပ်၊ ownership/permission နှင့် signed-index integrity စစ်၊ restored state ကိုနေရာချ၊ service start လုပ်ပြီး smoke test အပြည့် run ပါ။ Untrusted archive ကို live state ပေါ် တိုက်ရိုက်မဖြန့်ပါနှင့်။

Application rollback လုပ်ရန် ယခင် verify လုပ်ထားသော binary ကို install၊ service restart၊ readiness verify နှင့် registry/index behavior compare လုပ်ပါ။ Binary rollback အတွင်း signing secret ကို မသိမ်းမဆည်း ပြန်မပြောင်းပါနှင့်၊ registry data မဖျက်ပါနှင့်။ Incident၊ version၊ checksum၊ approval နှင့် validation result ကို မှတ်တမ်းတင်ပါ။

## ၁၁။ Security boundary နှင့် deferred controls

`ZAP_UNTRUSTED=1` သည် capability denial နှင့် bounded execution ပေးသော်လည်း universal kernel sandbox မဟုတ်ပါ။ မယုံကြည်ရသော customer code အတွက် သီးခြား VM/container policy၊ read-only source mount၊ dedicated writable directory၊ minimal environment၊ host credential မပါခြင်း၊ CPU/memory/process/time quota၊ syscall/network policy နှင့် explicit egress allowlist တို့ကို အသုံးပြုပါ။

Reference deployment တွင် built-in certificate pinning၊ OS keychain integration၊ universal cross-platform sandbox၊ cloud firewall provisioning နှင့် production signed-index key-management policy မပါဝင်ပါ။ ထို control များကို deployment owner က ထပ်မံ provision/review လုပ်ရမည်။ Filesystem canonicalization သည် OS အားလုံးတွင် check/use race အားလုံးကို မဖယ်ရှားနိုင်သောကြောင့် attacker က host filesystem ကို race လုပ်နိုင်သည့်အခြေအနေတွင် descriptor-relative သို့မဟုတ် handle-based isolation မရှိဘဲ အားမကိုးပါနှင့်။

## ၁၂။ Production release gate

Production traffic လက်ခံမီ အောက်ပါ gate တစ်ခုချင်းစီ၏ evidence ရှိရမည်။

| Gate | Evidence |
|---|---|
| Artifact integrity | Release checksum/signature စစ်ပြီး version မှတ်တမ်းတင်ထားခြင်း |
| Dependency security | Current advisory database ဖြင့် CI RustSec audit အောင်မြင်ခြင်း |
| Runtime quality | Locked format/check/Clippy/tests အောင်မြင်ခြင်း |
| Deployment contract | `scripts/validate_registry_deployment.sh` နှင့် `systemd-analyze verify` အောင်မြင်ခြင်း |
| Network boundary | Backend loopback-only၊ firewall က 8787 ပိတ်၊ TLS ingress active ဖြစ်ခြင်း |
| Secrets | Secret-manager injection အောင်မြင်ပြီး repository/log တွင် secret မပါခြင်း |
| Recovery | Backup နှင့် clean restore drill အောင်မြင်ခြင်း |
| Observability | Log၊ health check၊ certificate/resource/5xx alert active ဖြစ်ခြင်း |
| Rollback | Previous binary နှင့် approved rollback procedure ရှိခြင်း |

Gate အားလုံးအောင်မြင်ပြီးမှ DNS သို့မဟုတ် public load balancer ကို ingress သို့ route လုပ်ပါ။ Repository branch သို့မဟုတ် pull request သည် publish လုပ်ထားသော release artifact မဟုတ်ကြောင်း မှတ်သားပါ။ Checksum၊ signature၊ provenance နှင့် published asset verification ပြီးသော tagged release ကိုသာ install လုပ်ပါ။
