# Zap v2.11.0 Release Notes

**Release line:** v2.11.0
**စစ်ဆေးထားသော baseline:** Published Zap v2.10.1
**အခြေအနေ:** Deterministic Web route-explanation increment ကို publish လုပ်ပြီး

## အနှစ်ချုပ်

Zap v2.11.0 တွင် `zap explain route <path>` ကို ထည့်သွင်းထားပါသည်။ ဤ command သည် inspect-only native Web command ဖြစ်ပြီး user-managed Zap Web project ကို load/validate လုပ်ကာ native route matcher ကို ပြန်သုံးပြီး concrete request path နှင့် ကိုက်ညီသော declared route များကို ပြပေးပါသည်။ Listener မဖွင့်သကဲ့သို့ handler ကိုလည်း execute မလုပ်ပါ။

ဤ command သည် bounded ဖြစ်ပြီး automation အတွက် သုံးနိုင်ပါသည်။ 2,048 bytes အထိ safe absolute path ကိုသာ လက်ခံပြီး matching မလုပ်မီ query string ကို ဖယ်ရှားပါသည်။ `:parameter` နှင့် နောက်ဆုံး `*wildcard` extraction များကို ပြပြီး declaration order ကို ထိန်းထားကာ JSON output ကို support လုပ်ပါသည်။ Middleware၊ authorization သို့မဟုတ် business execution ကို trace လုပ်သည်ဟု မဟန်ဆောင်ဘဲ path candidate များကိုသာ ရှင်းပြပါသည်။

## အကောင်အထည်ဖော်ပြီးသော ပြောင်းလဲမှုများ

| နယ်ပယ် | ပြောင်းလဲမှု | အထောက်အထား |
|---|---|---|
| CLI surface | `zap explain route <path> [directory] [--json]` နှင့် help coverage ကို ထည့်သွင်းထားသည်။ | Native CLI unit test များ |
| Matching semantics | Native Web server သုံးသော exact၊ `:parameter` နှင့် နောက်ဆုံး `*wildcard` matcher ကိုပဲ ပြန်သုံးထားသည်။ | Native route matcher test နှင့် generated-project smoke test |
| Safety bounds | Project loading မလုပ်မီ relative path၊ traversal-shaped path၊ empty interior segment နှင့် 2,048 bytes ကျော်သော path များကို reject လုပ်သည်။ | Native CLI unit test များ |
| Human output | Project validity၊ path၊ candidate count၊ declaration index၊ method၊ pattern၊ handler နှင့် extracted parameter များကို ပြသည်။ | Generated-project smoke test |
| JSON output | Tooling အတွက် `project`၊ normalized `path` နှင့် order ထိန်းထားသော `matches` record များကို ထုတ်ပေးသည်။ | Generated-project smoke test |
| Documentation | Bilingual framework၊ native Web၊ learner၊ P2 status နှင့် command-validation documentation များကို update လုပ်ထားသည်။ | Documentation၊ link၊ policy နှင့် framework gate များ |

## အသုံးပြုပုံ

```bash
zap explain route /users/42
zap explain route /assets/chunks/app.js ./shop --json
```

Specific route နှင့် catch-all SPA fallback နှစ်ခုလုံး apply ဖြစ်သောအခါ concrete path တစ်ခုသည် declaration တစ်ခုထက်ပို၍ match ဖြစ်နိုင်ပါသည်။ Command သည် path candidate အားလုံးကို declaration order အတိုင်း ပြပါသည်။ Request အချိန်တွင် native server သည် တောင်းထားသော method အတွက် ပထမဆုံး matching declaration ကိုသာ ရွေးချယ်ပါသည်။ Path match ရှိသော်လည်း method မကိုက်ပါက `405`၊ path match မရှိပါက `404` ဖြစ်ပါသည်။

Example JSON shape:

```json
{"project":"valid Zap Web project: shop 0.1.0 (main: main.zp)","path":"/users/42","matches":[{"index":5,"method":"GET","path":"/users/:id","handler":"get_user","params":{"id":"42"}}]}
```

## Compatibility နှင့် boundaries

ဤသည်မှာ additive CLI capability ဖြစ်ပါသည်။ ရှိပြီးသား `zap web routes`၊ `zap web check`၊ `zap dev`၊ direct response map၊ Result-aware handler နှင့် user-managed project directory များသည် မပြောင်းလဲပါ။ Command သည် project file များကို မပြင်ပါ၊ network listener မစပါ၊ handler မခေါ်ပါ၊ middleware execution မစစ်ပါ၊ authorization မလုပ်ပါ၊ business behavior ကိုလည်း ခန့်မှန်းမပေးပါ။

Route explanation သည် production request tracer သို့မဟုတ် complete route compiler မဟုတ်ပါ။ Native Web server သည် bounded single-threaded development/reference server အဖြစ် ဆက်ရှိပါသည်။ Graceful shutdown၊ concurrent production serving၊ cancellation/backpressure၊ readiness၊ TLS/edge integration၊ observability၊ provider-neutral database support နှင့် အခြား documentation ထဲရှိ production milestone များသည် သီးခြားအလုပ်များအဖြစ် ဆက်ရှိပါသည်။

## Verification

ဤ increment သည် native formatting နှင့် test suite၊ release build၊ 201 checks ပါသော framework starter validation၊ standard-library policy validation၊ 174 checks ပါသော bilingual documentation consistency၊ link 763 ခုပါသော Markdown link validation၊ VS Code asset validation၊ whitespace check နှင့် clean-tree release preflight တွင် 199 checks passed၊ warning 1 ခု၊ failure 0 ခု ရရှိခဲ့ပါသည်။

## References

[1]: ../docs/WEB_FRAMEWORK_MM.md
[2]: ../docs/ZAP_WEB_NATIVE_MM.md
[3]: ../docs/LEARN_ZAP_MM.md
