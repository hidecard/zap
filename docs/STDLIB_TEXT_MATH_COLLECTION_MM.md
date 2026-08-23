# Zap Standard Library — Text၊ Math နှင့် Collection

ဤစာမျက်နှာတွင် Zap ၏ တည်ငြိမ်အောင်ပြင်ဆင်ထားသော text၊ math နှင့် collection helper များကို ဖော်ပြထားပါသည်။ API များသည် direct AST evaluation ကို အသုံးပြုပြီး argument အရေအတွက်နှင့် runtime type များကို စစ်ဆေးပြီးမှ လုပ်ဆောင်ပါသည်။

## Text APIs

| Function | အသုံးပြုပုံ | Return | အလုပ်လုပ်ပုံ |
|---|---|---|---|
| `len` | `len(value)` | `number` | Text ၏ Unicode character အရေအတွက်၊ list/map ၏ element အရေအတွက်ကို ပြန်ပေးသည်။ |
| `str` | `str(value)` | `text` | Zap value ကို display text အဖြစ် ပြောင်းသည်။ |
| `type` | `type(value)` | `text` | Runtime category (`none`, `bool`, `number`, `text`, `list`, `map`, `object`, `result`, `option`) ကို ပြန်ပေးသည်။ |
| `contains` | `contains(text, part)` သို့မဟုတ် `contains(list, value)` | `bool` | Text ထဲတွင် ပါဝင်မှု သို့မဟုတ် list membership ကို စစ်သည်။ |
| `is_empty` | `is_empty(value)` | `bool` | Text၊ list သို့မဟုတ် map အတွင်း element မရှိခြင်းကို စစ်သည်။ |
| `split` | `split(value, separator)` | `list<text>` | Text ကို text separator ဖြင့် ခွဲသည်။ |
| `join` | `join(values, separator)` | `text` | Text list ကို separator ဖြင့် ဆက်သည်။ |
| `trim` | `trim(value)` | `text` | အစနှင့်အဆုံး whitespace များကို ဖယ်ရှားသည်။ |
| `lower` | `lower(value)` | `text` | Text ကို lowercase ပြောင်းသည်။ |
| `upper` | `upper(value)` | `text` | Text ကို uppercase ပြောင်းသည်။ |
| `replace` | `replace(value, from, to)` | `text` | Text တစ်ခု၏ occurrence အားလုံးကို အခြား text ဖြင့် အစားထိုးသည်။ |
| `starts_with` | `starts_with(value, prefix)` | `bool` | Text သည် prefix ဖြင့် စ/မစ စစ်သည်။ |
| `ends_with` | `ends_with(value, suffix)` | `bool` | Text သည် suffix ဖြင့် ဆုံး/မဆုံး စစ်သည်။ |
| `char_at` | `char_at(value, index)` | `text` | သတ်မှတ်ထားသော zero-based Unicode character index မှ character တစ်လုံးကို ပြန်ပေးသည်။ |
| `substring` | `substring(value, start, end)` | `text` | UTF-8 byte offset မဟုတ်ဘဲ character range `start <= index < end` ကို ပြန်ပေးသည်။ |
| `codepoints` | `codepoints(value)` | `list<number>` | Unicode character တစ်လုံးချင်းစီ၏ numeric code point များကို ပြန်ပေးသည်။ |

Text operations များသည် character အပေါ် အခြေခံသည့်နေရာတွင် Unicode ကို ထည့်သွင်းစဉ်းစားပါသည်။ `char_at`၊ `substring`၊ `codepoints`၊ `len` နှင့် `reverse` တို့သည် UTF-8 byte offset မသုံးဘဲ Unicode scalar value များအပေါ် အလုပ်လုပ်ပါသည်။ Negative index နှင့် `char_at` range ကျော်ခြင်းကို reject လုပ်ပြီး `substring` ၏ end သည် text length ထက်ကျော်ပါက ရှိသမျှ suffix ကိုသာ ပြန်ပေးပါသည်။ `join` သည် list element အားလုံး `text` ဖြစ်ရန်လိုပြီး mixed value များကို အလိုအလျောက် text မပြောင်းပါ။

```zap
let source: text = "  Zap Language  "
say trim(source)
say upper(trim(source))
say replace("zap language", "zap", "Zap")
say starts_with("Zap", "Z")
say char_at("က🙂ab", 1)
say substring("က🙂ab", 1, 3)
say codepoints("က🙂")
say join(["web", "ai", "iot"], ", ")
```

## Math APIs

| Function | အသုံးပြုပုံ | Return | အလုပ်လုပ်ပုံ |
|---|---|---|---|
| `abs` | `abs(value)` | `number` | Number ၏ absolute value ကို ပြန်ပေးသည်။ အနည်းဆုံး signed integer ကို overflow ဖြစ်သောကြောင့် reject လုပ်သည်။ |
| `min` | `min(left, right)` | `number` | Number နှစ်ခုထဲမှ ငယ်သောတန်ဖိုးကို ပြန်ပေးသည်။ |
| `max` | `max(left, right)` | `number` | Number နှစ်ခုထဲမှ ကြီးသောတန်ဖိုးကို ပြန်ပေးသည်။ |
| `pow` | `pow(base, exponent)` | `number` | Integer power တွက်သည်။ exponent သည် negative မဖြစ်ရဘဲ `1_000_000` ထက် မကျော်ရပါ။ |
| `sum` | `sum(values)` | `number` | List ထဲရှိ number များကို checked integer arithmetic ဖြင့် ပေါင်းသည်။ |
| `range` | `range(end)` သို့မဟုတ် `range(start, end)` | `list<number>` | `start <= value < end` ဖြစ်သော half-open integer range ကို ဖန်တီးသည်။ |

Math helper များသည် integer `number` များကို အသုံးပြုပါသည်။ `pow` သည် negative သို့မဟုတ် limit ကျော်သော exponent ကို execution မတိုင်မီ reject လုပ်ကာ checked exponentiation-by-squaring ကို အသုံးပြုပါသည်။ Overflow နှင့် မမှန်ကန်သော exponent များကို wrap မလုပ်ဘဲ deterministic runtime error ပြန်ပေးပါသည်။

```zap
say abs(-42)
say min(8, 3)
say max(8, 3)
say pow(2, 10)
say sum([2, 4, 6])
say range(3)
say range(2, 5)
```

## Collection APIs

| Function | အသုံးပြုပုံ | Return | အလုပ်လုပ်ပုံ |
|---|---|---|---|
| `keys` | `keys(value)` | `list<text>` | Map ၏ text keys များကို ပြန်ပေးသည်။ |
| `entries` | `entries(value)` | `list<map>` | `{key, value}` record များကို key အက္ခရာစဉ်အလိုက် ပြန်ပေးသည်။ |
| `enumerate` | `enumerate(values)` | `list<map>` | List index ကို သုညမှ စတင်ပြီး `{index, value}` record များ ပြန်ပေးသည်။ |
| `count` | `count(values, item)` | `number` | List ထဲတွင် `item` နှင့် တူသော value အရေအတွက်ကို ရေတွက်သည်။ |
| `reverse` | `reverse(values)` | `list<T>` | မူလ list ကို မပြောင်းဘဲ ပြောင်းပြန်ထားသော copy ကို ပြန်ပေးသည်။ |
| `contains` | `contains(values, item)` | `bool` | Zap value equality ဖြင့် list membership ကို စစ်သည်။ |
| `is_empty` | `is_empty(values)` | `bool` | List သို့မဟုတ် map အတွင်း element မရှိခြင်းကို စစ်သည်။ |

```zap
let values: list<number> = [1, 2, 1, 3]
say count(values, 1)
say contains(values, 3)
say reverse(values)

let record = {"name": "Zap", "version": 1}
say keys(record)
say entries(record)
say enumerate(["first", "second"])
say is_empty({})
```

`entries` နှင့် `enumerate` တို့သည် runtime iteration limit အတွင်း bounded ဖြစ်ပြီး input collection ကို မပြောင်းလဲပါ။ Map entry order သည် deterministic ဖြစ်သဖြင့် support လုပ်ထားသော platform များတွင် input တူပါက output တူညီပါသည်။

## Validation နှင့် Error များ

Stabilized helper အားလုံးသည် argument အရေအတွက် မမှန်ခြင်းနှင့် runtime value type မကိုက်ညီခြင်းကို explicit error ဖြင့် reject လုပ်ပါသည်။ ဥပမာ `join([1, 2], ",")`၊ `sum([1, "two"])`၊ `pow(2, -1)` နှင့် `abs(-9223372036854775808)` တို့သည် value များကို အလိုအလျောက် မပြောင်းဘဲ သို့မဟုတ် overflow မဖြစ်စေဘဲ fail ဖြစ်ပါမည်။

Named arguments များကို user-defined function၊ method နှင့် closure များအတွက် support လုပ်ထားပါသည်။ Built-in helper များသည် လက်ရှိ positional arguments ကို အသုံးပြုပြီး named syntax ဖြင့် ခေါ်လျှင် မထောက်ပံ့သေးကြောင်း ရှင်းလင်းသော diagnostic ပြန်ပေးပါသည်။
