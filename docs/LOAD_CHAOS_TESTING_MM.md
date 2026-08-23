# Zap Production Load နှင့် Chaos Testing

Framework branch တွင် bounded standard-library test tool နှစ်ခု ထည့်ထားပါသည်။ `scripts/load_zap_host.py` သည် သတ်မှတ်ပေးထားသော endpoint အတွက် redacted latency/status report ထုတ်ပါသည်။ `scripts/chaos_zap_host.py` သည် authentication နှင့် service recovery experiment များကို opt-in ဖြင့် run လုပ်ပါသည်။ Tool နှစ်ခုလုံးသည် default အနေဖြင့် loopback target သာ သုံးပြီး remote target အတွက် explicit flag လိုအပ်ပါသည်။ မသိသော public service သို့ မတော်တဆ traffic မပို့ပါ။

## Load test အဆင့်များ

Load test ကို local/staging smoke test၊ production canary တွင် short ramp၊ ထို့နောက် bounded soak window အစီအစဉ်ဖြင့် run ပါ။ Deployment version၊ instance count၊ database pool settings၊ request budget၊ success ratio၊ p95/p99 latency၊ CPU/memory၊ database saturation၊ pool wait time နှင့် error category များကို မှတ်တမ်းတင်ပါ။ HTTP status ကောင်းရုံဖြင့် database၊ identity provider သို့မဟုတ် downstream dependency များ ကောင်းသည်ဟု မဆိုရပါ။

| Stage | ရည်ရွယ်ချက် | ဥပမာ budget |
|---|---|---:|
| Smoke | Route၊ proxy၊ auth နှင့် report generation စစ်ခြင်း | 10 seconds၊ workers 2 |
| Ramp | Canary တစ်ခု၏ saturation စတင်သည့်နေရာရှာခြင်း | 60 seconds၊ workers 8–32 |
| Soak | Leak၊ pool starvation နှင့် cache churn ရှာခြင်း | 10–30 minutes၊ approved steady rate |

Script သည် duration နှင့် concurrency နှစ်ခုလုံး bounded ဖြစ်ပါသည်။ Bearer token ကို command line တွင် မပေးဘဲ `ZAP_LOAD_BEARER_TOKEN` environment မှ ဖတ်ပြီး token ကို မပြန်ထုတ်ပါ။ Remote test အတွက် `--allow-remote` လိုအပ်သဖြင့် approved host/canary window တွင်သာ သုံးပါ။

### Health endpoint smoke test

```bash
python3 scripts/load_zap_host.py \
  --url http://127.0.0.1:3000/health \
  --duration-seconds 10 \
  --concurrency 2 \
  --max-p95-ms 200 \
  --min-success-ratio 1.0 \
  --output target/load-health.json
```

### Authenticated API canary

```bash
export ZAP_LOAD_BEARER_TOKEN="$(security-tool read zap/staging/load-token)"
python3 scripts/load_zap_host.py \
  --url https://api.example.com/api/users \
  --allow-remote \
  --duration-seconds 60 \
  --concurrency 16 \
  --max-p95-ms 500 \
  --min-success-ratio 0.995 \
  --output target/load-api-canary.json
unset ZAP_LOAD_BEARER_TOKEN
```

Short-lived၊ least-privilege test token သုံးပါ။ Production administrator token၊ refresh token သို့မဟုတ် user session မှ ကူးထားသော token ကို မသုံးရပါ။ Load test အတွက် read-only path သုံးပြီး synthetic data create လုပ်မည့်အခါ test plan ထဲ cleanup ထည့်ထားရမည်။

Report ထဲတွင် normalized target URL၊ status count၊ request count၊ success ratio နှင့် latency percentile သာ ပါပါသည်။ Authorization header၊ response body၊ query string သို့မဟုတ် token value မပါပါ။

## Chaos experiments

Chaos experiment များကို approved maintenance/canary window အတွင်း operator က `/ready`၊ Nginx၊ systemd၊ database နှင့် identity-provider telemetry ကြည့်နေစဉ် run ပါ။ Service-control experiment အတွက် `--allow-service-control` နှင့် exact confirmation string `I_UNDERSTAND_DOWNTIME` နှစ်ခုလုံး လိုအပ်ပါသည်။

### Invalid JWT rejection

Protected route သည် malformed bearer token ကို reject လုပ်ပြီး application gateway ထံ မရောက်ကြောင်း စစ်ရန်—

```bash
python3 scripts/chaos_zap_host.py \
  --fault invalid-jwt \
  --url http://127.0.0.1:3000/api/users
```

မျှော်မှန်းရလဒ်သည် `401` ဖြစ်ပါသည်။ `200` ပြန်ပါက service သည် demo-authenticator mode တွင် ရှိနေခြင်း သို့မဟုတ် route မကာကွယ်ထားခြင်း ဖြစ်နိုင်ပြီး production gate fail အဖြစ် သတ်မှတ်ရမည်။

### Process restart နှင့် recovery

```bash
sudo python3 scripts/chaos_zap_host.py \
  --fault restart-service \
  --url http://127.0.0.1:3000/health \
  --service zap-web.service \
  --allow-service-control \
  --confirm I_UNDERSTAND_DOWNTIME \
  --recovery-timeout-seconds 60
```

Restart မတိုင်မီ service healthy ဖြစ်ကြောင်းနှင့် recovery budget အတွင်း `/health` က `200` ပြန်ကြောင်း စစ်ပါသည်။ Restart အောင်မြင်ခြင်းတစ်ခုတည်းဖြင့် `/ready`၊ database migration၊ authentication သို့မဟုတ် pool recovery မှန်သည်ဟု မယူဆရပါ။ Readiness နှင့် authenticated smoke test ဆက်လုပ်ရမည်။

### Stop/start recovery

```bash
sudo python3 scripts/chaos_zap_host.py \
  --fault stop-start-service \
  --url http://127.0.0.1:3000/health \
  --service zap-web.service \
  --allow-service-control \
  --confirm I_UNDERSTAND_DOWNTIME \
  --recovery-timeout-seconds 60
```

Stop ပြီးနောက် service unavailable ဖြစ်ပြီး start ပြီးနောက် ပြန်ကောင်းရမည်။ Load balancer နောက်တွင် run မည်ဆိုလျှင် canary ကို rotation မှ အရင်ဖယ်ပါ။ Script သည် local process lifecycle ကိုသာ စစ်ပြီး zero-downtime deployment ဖြစ်သည်ဟု မဆိုပါ။

## Dependency failure experiments

Checked-in script သည် remote database၊ identity provider သို့မဟုတ် network interface ကို kill မလုပ်ပါ။ ထို failure များအတွက် approved staging fault-injection layer သို့မဟုတ် provider-specific test switch လိုအပ်ပါသည်။ Safe plan တစ်ခုမှာ staging JWKS URL ကို `503` ပြန်သော controlled endpoint သို့ ပြောင်း၊ `503 authentication_unavailable` mapping စစ်၊ endpoint ပြန်ကောင်းပြီး recovery စစ်ခြင်း ဖြစ်ပါသည်။ Database အတွက် staging repository/provider kill switch သုံးပြီး `/ready` fail ဖြစ်ကာ `/health` liveness signal ဆက်ရှိကြောင်း စစ်ပါ။

Public DNS poisoning၊ broad firewall flush၊ credential deletion သို့မဟုတ် production database termination ကို ပထမဆုံး chaos experiment အဖြစ် မသုံးရပါ။ Experiment တိုင်းတွင် hypothesis၊ blast-radius limit၊ automatic/manual rollback၊ abort signal နှင့် post-test evidence bundle ပါရမည်။

## CI နှင့် release gate

Shell script များအတွက် `bash -n`၊ Python tool များအတွက် `python3 -m py_compile`၊ checked-in deployment validator များ၊ Rust test/Clippy နှင့် local smoke load test ကို CI တွင် run ပါ။ Remote load နှင့် service-control chaos ကို pull request တိုင်းတွင် မ run ဘဲ approval ရှိသော environment job အဖြစ်သာ run ပါ။
