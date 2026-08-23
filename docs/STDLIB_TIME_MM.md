# Zap Time Standard Library

`time` domain သည် UTC timestamp များနှင့် integer-millisecond duration helper များကို ပေးပါသည်။ လက်ရှိ release line တွင် ဤ API များကို direct builtin များအဖြစ် အသုံးပြုနိုင်ပါသည်။

## API ရည်ညွှန်းချက်

| API | Arguments | Result |
|---|---|---|
| `utc_now()` | မရှိပါ | `unix_seconds` နှင့် `unix_millis` ပါသော map ကို ပြန်ပေးသည်။ |
| `duration_parts(milliseconds)` | integer millisecond duration တစ်ခု | `milliseconds`၊ `days`၊ `hours`၊ `minutes`၊ `seconds` နှင့် `millis` ပါသော map ကို ပြန်ပေးသည်။ |
| `duration_between(end_millis, start_millis)` | integer millisecond timestamp နှစ်ခု | စစ်ဆေးထားသော `end_millis - start_millis` ကွာခြားချက်အတွက် အထက်ပါ duration map ကို ပြန်ပေးသည်။ |
| `sleep(milliseconds)` | non-negative integer duration တစ်ခု၊ `60_000` အထိ | bounded wall-clock delay ပြီးနောက် `none` ပြန်ပေးသည်။ |

`utc_now()` သည် Unix time ကို UTC ဖြင့် အခြေခံပြီး local timezone ပေါ် မူတည်ခြင်းမရှိပါ။ Millisecond တန်ဖိုးသည် seconds တန်ဖိုးနှင့် ကိုက်ညီပြီး `unix_seconds * 1000` ထက် ကြီးသို့မဟုတ် ညီကာ `(unix_seconds + 1) * 1000` ထက် ငယ်ပါသည်။

`sleep` သည် bounded system operation ဖြစ်ပါသည်။ Negative value သို့မဟုတ် `60_000` milliseconds ထက်ကျော်သော value များကို sleep မလုပ်မီ deterministic error ဖြင့် reject လုပ်သည်။ ၎င်းသည် scheduler၊ reactor သို့မဟုတ် lazy async continuation မဟုတ်ပါ။

`duration_parts` သည် input ၏ sign ကို ထိန်းသိမ်းထားပါသည်။ Component field များကို unit boundary တစ်ခုစီတွင် သုညဘက်သို့ ဖြတ်တောက်သောကြောင့် တိကျသော signed value လိုပါက `milliseconds` ကို အသုံးပြုနိုင်ပါသည်။ `duration_between` သည် checked subtraction ကို အသုံးပြုပြီး timestamp နှစ်ခုကြောင့် signed integer range overflow ဖြစ်နိုင်ပါက runtime error ပြန်ပေးပါသည်။ လုံခြုံစွာ ကိုယ်စားမပြုနိုင်သော duration များကိုလည်း decomposition က ငြင်းပယ်ပါသည်။

## ဥပမာ

```zap
let now = utc_now()
say now["unix_seconds"]
say now["unix_millis"]

let started = now["unix_millis"] - 90_061_007
let elapsed = duration_between(now["unix_millis"], started)
say elapsed["days"]
say elapsed["hours"]
say elapsed["minutes"]
say elapsed["seconds"]
say elapsed["millis"]
```

Implementation နှင့် regression coverage အတွက် `native/src/evaluator.rs` ကို ကြည့်ပါ။ API catalog ကို `native/src/stdlib_catalog.rs` တွင် ထိန်းသိမ်းထားပါသည်။
