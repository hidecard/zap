# Zap Type Narrowing

Zap တွင် `option<T>` နှင့် `result<T>` value များအတွက် branch အတွင်း အသုံးပြုနိုင်သော type narrowing ရှိပါသည်။ Guard အောင်မြင်သော block အတွင်းတွင်သာ payload type သို့ ပြောင်းလဲပြီး block ပြီးလျှင် မူလ wrapper type ကို ပြန်လည်ရရှိပါသည်။

## Guard ဖြင့် payload အသုံးပြုခြင်း

Function တစ်ခုက payload type ကို လိုအပ်သောအခါ value ၏ အမျိုးအစားကို သက်သေပြရန် `is_some`၊ `is_ok` သို့မဟုတ် `is_err` ကို အသုံးပြုနိုင်ပါသည်။

```zap
fn use_number(value: number):
    say value

let maybe: option<number> = some(7)
let result: result<number> = ok(9)

if is_some(maybe):
    use_number(maybe)

if is_ok(result):
    use_number(result)
```

`is_err(result)` သည် result ၏ error payload type ကို သတ်မှတ်ချက်အရ narrow လုပ်ပေးပါသည်။

## Boolean conjunction (`and`)

`and` ကို အသုံးပြုသောအခါ လုံခြုံသော guard တစ်ခုချင်းစီ၏ အချက်အလက်ကို တူညီသော branch အတွင်း ပေါင်းစပ်အသုံးပြုနိုင်ပါသည်။

```zap
let maybe: option<number> = some(7)
let result: result<number> = ok(9)

if is_some(maybe) and is_ok(result):
    let first: number = maybe
    let second: number = result
```

ဤ narrowing သည် branch အတွင်းတွင်သာ သက်ရောက်ပြီး မသေချာသော guard ကို အလိုအလျောက် narrow မလုပ်ပါ။

## Safe disjunction (`or`)

`or` expression တွင် alternative အားလုံးက variable တစ်ခုတည်းနှင့် payload type တစ်ခုတည်းကို သက်သေပြသောအခါမှသာ safe narrowing ပြုလုပ်ပါသည်။ ထိုနည်းဖြင့် `or` ၏ တစ်ဖက်က မတူညီသောအချက်ပေးနေသော်လည်း မှားယွင်းစွာ narrow ဖြစ်ခြင်းကို ကာကွယ်ပါသည်။

```zap
let maybe: option<number> = some(7)

if is_some(maybe) or is_some(maybe):
    let value: number = maybe
```

Alternative များက တူညီသောအချက်ကို မပေးနိုင်ပါက wrapper type ကို ဆက်လက်အသုံးပြုပါ သို့မဟုတ် branch ခွဲရေးပါ။

## Alias variables

Alias variable များသည် မူလ `option<T>` သို့မဟုတ် `result<T>` inferred type ကို ထိန်းသိမ်းထားပြီး သီးခြား narrow လုပ်နိုင်ပါသည်။

```zap
let original: option<number> = some(7)
let alias = original

if is_some(alias):
    let value: number = alias
```

Alias ကို narrow လုပ်ခြင်းသည် မူလ variable ၏ static type ကို မပြောင်းလဲပါ။

## `else` branch နှင့် type restoration

အောင်မြင်သော branch တွင် payload type ရရှိပါသည်။ `else` branch တွင် success condition မအောင်မြင်ကြောင်းကို ထည့်သွင်းစဉ်းစားသော်လည်း value ကို payload အဖြစ် အလိုအလျောက် မပြောင်းဘဲ option/result wrapper အဖြစ် ဆက်လက်ထားရှိပါသည်။

```zap
let maybe: option<number> = some(7)

if is_some(maybe):
    let value: number = maybe
else:
    let still_wrapped: option<number> = maybe
```

Conditional ပြီးလျှင် `maybe` သည် branch နှစ်ခုလုံးတွင် `option<number>` အဖြစ် ပြန်လည်ရရှိပါသည်။ Guard သို့မဟုတ် explicit unwrap မရှိဘဲ `number` လိုအပ်သော function ထဲသို့ တိုက်ရိုက်ပေးပါက `zap check` က reject လုပ်ပါသည်။

## Scope နှင့် diagnostics

Narrowing သည် indentation အရ guarded block အတွင်းရှိ nested statements များတွင်သာ သက်ရောက်ပါသည်။ Sibling statement များ သို့မဟုတ် conditional ပြီးနောက် code ထဲသို့ မပျံ့နှံ့ပါ။ Narrowed value ကို မှားယွင်းသော scope တွင် အသုံးပြုပါက `option<number>` နှင့် `result<number>` ကဲ့သို့ wrapper type များအပါအဝင် expected နှင့် actual type များကို diagnostic ဖြင့် ပြပေးပါသည်။

## လက်ရှိကန့်သတ်ချက်

လက်ရှိ implementation သည် direct predicate guards၊ safe common facts ပါသော boolean `and`/`or` combinations၊ inferred aliases နှင့် branch restoration များကို support လုပ်ပါသည်။ User-defined predicates၊ mutation-sensitive alias analysis နှင့် complex loop invariants များသည် နောက်ပိုင်း static-checker အလုပ်များ ဖြစ်ပါသည်။
