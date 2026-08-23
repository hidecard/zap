# Zap v2.6.0 Release Notes

**Release line:** v2.6.0
**စစ်ဆေးထားသော baseline:** v2.5.0 နောက်ပိုင်း merged `master`
**အခြေအနေ:** Web observability နှင့် integrity အတွက် incremental release

## အနှစ်ချုပ်

Zap v2.6.0 တွင် `zap-host` adapter အတွက် bounded public `/metrics` endpoint အသစ် ထည့်သွင်းထားပြီး observability contract အတွက် executable evidence ရှိလာပါသည်။ Endpoint သည် path၊ identity၊ request ID သို့မဟုတ် user-controlled label များမပါဘဲ total request၊ 5xx response နှင့် in-flight request process counter များကို Prometheus-style text အဖြစ် ထုတ်ပေးပါသည်။

One-command user-managed `zap new <directory>` workflow သည် မပြောင်းလဲပါ။ Web adapter တွင် bounded request policy၊ request ID၊ timeout၊ rate-limit ordering၊ readiness၊ graceful drain၊ database-pool admission guard နှင့် authentication/authorization seam များ ဆက်လက်ပါဝင်ပါသည်။ ဤ release တွင် ထို boundary များအတွက် documentation နှင့် quickstart example များကို ပိုမိုရှင်းလင်းစွာ ပြင်ထားပါသည်။

## အကောင်အထည်ဖော်ပြီးသော ပြောင်းလဲမှုများ

| နယ်ပယ် | ပြောင်းလဲမှု | အထောက်အထား |
|---|---|---|
| Observability | Prometheus-style text output နှင့် bounded metric name များပါသော public `GET /metrics` ကို ထည့်သွင်းထားသည်။ | Host unit နှင့် HTTP contract tests |
| Security boundary | Metrics output ထဲတွင် user-controlled label နှင့် request-identifying data မပါဝင်ပါ။ | Low-cardinality renderer test |
| Web documentation | English/Burmese host guide နှင့် quickstart တွင် endpoint၊ route table နှင့် curl example ထည့်ထားသည်။ | Bilingual documentation review |
| Release integrity | v2.6.0 metadata၊ manifest၊ specification၊ policy နှင့် current baseline များကို ညှိထားသည်။ | Release/version/documentation gates |

## Compatibility နှင့် boundaries

Endpoint အသစ်သည် additive ဖြစ်သည်။ ရှိပြီးသား health၊ readiness၊ API၊ authentication၊ rate-limit၊ request-ID နှင့် graceful-shutdown contract များ မပြောင်းလဲပါ။ `/metrics` ကို local နှင့် controlled host-adapter monitoring အတွက် ရည်ရွယ်ထားပြီး deployment တစ်ခုချင်းစီသည် management endpoint များကို မိမိ network policy အတိုင်း ကာကွယ်ရမည်။

ဤ release သည် complete ORM၊ provider-neutral production database platform၊ production async I/O reactor၊ user-defined trait/generic declaration၊ cross-file semantic rename၊ SSR/template compiler၊ WebSocket/streaming/upload stack၊ built-in admin UI သို့မဟုတ် real mobile/AI/IoT provider adapter များ ပြီးစီးပြီဟု မဆိုထားပါ။ ၎င်းတို့သည် implementation နှင့် platform evidence လိုအပ်သော သီးခြား milestone များအဖြစ် ဆက်ရှိပါသည်။

## Verification

Focused host adapter format၊ strict Clippy နှင့် test suite များသည် metrics regression အပါအဝင် pass ဖြစ်ပါသည်။ Publication မတိုင်မီ full native/host release gate၊ framework starter check၊ documentation consistency၊ Markdown link validation၊ VS Code parity၊ LSP parity နှင့် clean-tree release preflight အားလုံး pass ဖြစ်ရမည်။

## Upgrade

မိမိ platform နှင့်ကိုက်ညီသော archive ကို v2.6.0 GitHub Release မှ download လုပ်ပြီး checksum နှင့် detached signature ကို verify လုပ်ပါ။ ထို့နောက် `zap --version` ဖြင့် binary version ကို စစ်ဆေးပါ။ ရှိပြီးသား Zap project များသည် manifest နှင့် lockfile workflow ကို ဆက်လက်အသုံးပြုနိုင်ပါသည်။

## References

[1]: ../docs/ZAP_HOST_MM.md
[2]: ../docs/ZAP_HOST_QUICKSTART_MM.md
