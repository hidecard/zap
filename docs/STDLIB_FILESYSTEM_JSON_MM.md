# Zap Standard Library — Filesystem နှင့် JSON

ဤစာမျက်နှာတွင် Zap native runtime မှ ပံ့ပိုးထားသော filesystem နှင့် JSON APIs များ၏ တည်ငြိမ်ထားသော public contract ကို ဖော်ပြထားပါသည်။ ဤ function များသည် direct AST call path မှ structured arguments ဖြင့် အလုပ်လုပ်ပါသည်။

## Filesystem APIs

| Function | Signature | Return | လုပ်ဆောင်ချက် |
|---|---|---|---|
| `read_text` | `read_text(path: text)` | `text` | UTF-8 text file ကို ဖတ်သည်။ |
| `write_text` | `write_text(path: text, content: text)` | `none` | UTF-8 text file ကို ရေးသား/အစားထိုးသည်။ |
| `read_lines` | `read_lines(path: text)` | `list<text>` | File ကို line များအဖြစ် ဖတ်ပြီး line separator များကို ဖယ်ရှားသည်။ |
| `write_lines` | `write_lines(path: text, lines: list<text>)` | `none` | Text line များကို platform သင့် newline ဖြင့် ရေးသည်။ |
| `exists` | `exists(path: text)` | `bool` | Path ရှိ/မရှိ စစ်သည်။ |
| `path_join` | `path_join(first: text, second: text, ...)` | `text` | Host platform ၏ path rules ဖြင့် path အပိုင်းများကို ပေါင်းသည်။ |
| `basename` | `basename(path: text)` | `text` | Path ၏ နောက်ဆုံးအပိုင်းကို ပြန်ပေးသည်။ |
| `dirname` | `dirname(path: text)` | `text` | Parent path ကို ပြန်ပေးသည်။ |

Filesystem function များအားလုံးသည် argument count နှင့် type များကို စစ်ဆေးပါသည်။ File ဖတ်/ရေးရာတွင် path မရနိုင်ခြင်း၊ content decode မရခြင်း သို့မဟုတ် write မအောင်မြင်ခြင်း ဖြစ်ပါက runtime error ပြန်ပေးပါသည်။ Source file နှင့် file read များတွင် runtime safety limit ရှိသောကြောင့် data အလွန်ကြီးလျှင် အပိုင်းငယ်များအဖြစ် စီမံသင့်ပါသည်။

```zap
let path: text = path_join("data", "users.txt")
write_lines(path, ["alice", "bob"])
let users: list<text> = read_lines(path)
if exists(path):
    say basename(path)
```

## JSON APIs

| Function | Signature | Return | လုပ်ဆောင်ချက် |
|---|---|---|---|
| `json` | `json(value)` | `text` | Zap value ကို JSON text အဖြစ် encode လုပ်သည်။ |
| `from_json` | `from_json(source: text)` | `any` | JSON text ကို Zap value အဖြစ် parse လုပ်သည်။ |

JSON conversion သည် deterministic ဖြစ်ပါသည်။ `none` သည် JSON `null` ဖြစ်ပြီး boolean၊ number၊ text၊ list နှင့် map များသည် သက်ဆိုင်ရာ JSON value များအဖြစ် ပြောင်းလဲပါသည်။ Zap ၏ `option` နှင့် `result` များသည် round trip ပြုလုပ်သည့်အခါ variant information မပျောက်စေရန် tagged object ပုံစံကို အသုံးပြုပါသည်။

```zap
let source: text = json([1, 2, 3])
let values = from_json(source)
say values[1]

let record = from_json("{\"name\":\"Zap\",\"version\":1}")
say record["name"]
```

`json` သည် argument တစ်ခုတည်းသာ လက်ခံပါသည်။ `from_json` သည် text argument တစ်ခုတည်းသာ လက်ခံပါသည်။ JSON မမှန်ခြင်း၊ မထောက်ပံ့သော numeric value များ၊ မသိသော Zap variant tag များနှင့် Zap integer range ပြင်ပ number များသည် ရှင်းလင်းသော runtime error ဖြစ်စေပါသည်။ JSON input/output ကို **8 MiB** safety limit ဖြင့် ကန့်သတ်ထားပြီး limit ကျော်လွန်သော payload များကို မလုပ်ဆောင်ဘဲ reject လုပ်ပါသည်။

## Error examples

```zap
// Type error: from_json သည် text လိုအပ်သည်။
let value = from_json(42)

// Parse error: source သည် valid JSON မဟုတ်ပါ။
let broken = from_json("{invalid}")
```

ဤ APIs များသည် support လုပ်ထားသော platform များအကြား portable ဖြစ်စေရန် ရည်ရွယ်ထားပါသည်။ Path separator များကို host runtime က သတ်မှတ်သောကြောင့် separator ကို ကိုယ်တိုင်ပေါင်းမည့်အစား `path_join` ကို အသုံးပြုသင့်ပါသည်။
