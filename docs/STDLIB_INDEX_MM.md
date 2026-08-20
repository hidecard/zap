# Zap Standard Library Public Modules

Zap ၏ standard library ကို အများပြည်သူအသုံးပြုနိုင်သော domain များအဖြစ် စနစ်တကျ ခွဲခြားထားပါသည်။ Runtime dispatch ကို compatibility အတွက် ဗဟိုမှ ဆက်လက်လုပ်ဆောင်သော်လည်း ဤ index သည် documentation၊ tooling နှင့် နောင်တွင် ထည့်သွင်းမည့် package modules များအတွက် တည်ငြိမ်သော public organization ကို သတ်မှတ်ပေးပါသည်။

| Public module | အသုံးပြုနိုင်သောအပိုင်း | အဓိက API များ |
|---|---|---|
| `text` | စာသားပြောင်းလဲခြင်းနှင့် စီမံခြင်း | `len`, `str`, `type`, `upper`, `lower`, `trim`, `split`, `join`, `contains`, `replace` |
| `math` | ကိန်းဂဏန်းလုပ်ဆောင်ချက်များ | `abs`, `min`, `max`, `pow`, `sqrt` |
| `collections` | List နှင့် map များ | `sum`, `range`, `keys`, `count`, `reverse`, `get` |
| `filesystem` | ကန့်သတ်ထားသော text နှင့် line I/O | `read_text`, `write_text`, `read_lines`, `write_lines`, `exists` |
| `json` | JSON ပြောင်းလဲခြင်း | `json`, `from_json` |
| `system` | Environment၊ path နှင့် အချိန် | `env`, `has_env`, `path_join`, `basename`, `dirname`, `now`, `sleep` |
| `network` | URL စီမံခြင်းနှင့် ကန့်သတ်ထားသော HTTP request များ | `url_parse`, `url_encode`, `url_decode`, `http_get`, `http_request` |
| `process` | Shell မသုံးသော process execution | `process_run` |

Public builtin အားလုံးသည် argument များကို တိကျစွာ စစ်ဆေးပြီး မမှန်ကန်သော input များကို မျက်ကွယ်မပြုဘဲ structured runtime error ပြန်ပေးပါသည်။ Filesystem၊ JSON နှင့် HTTP response operation များတွင် 8 MiB safety limit ကို အသုံးပြုပါသည်။ URL input များကို 8 KiB အထိသာ ခွင့်ပြုပါသည်။ `process_run` သည် shell interpretation မလုပ်ဘဲ program ကို တိုက်ရိုက်ခေါ်ယူပြီး text command နှင့် text argument list ကိုသာ လက်ခံကာ UTF-8 stdout/stderr ကို ဖမ်းယူပါသည်။ Output 1 MiB ကျော်လျှင် ငြင်းပယ်ပါသည်။ HTTP request များသည် `http` နှင့် `https` URL များကိုသာ လက်ခံပြီး connect၊ read နှင့် write timeout များကို ကန့်သတ်ထားပါသည်။

Public catalog သည် deterministic ဖြစ်ပါသည်။ Builtin တစ်ခုစီသည် တစ်ကြိမ်သာ ပါဝင်ပြီး domain တစ်ခုတည်းနှင့်သာ သက်ဆိုင်ပါသည်။ Native runtime ထဲတွင် tooling နှင့် tests များအသုံးပြုသော catalog ပါရှိသဖြင့် documentation နှင့် implementation တို့ကို တစ်ပြေးညီ ထိန်းသိမ်းနိုင်ပါသည်။

```zap
let endpoint = url_parse("https://example.com:8443/api?q=zap")
say endpoint["host"]
say url_encode("a b/c")

let result = process_run("printf", ["zap"])
say result["success"]
say result["stdout"]
```

လက်ရှိ release line တွင် API များကို direct builtin များအဖြစ် အသုံးပြုနိုင်ပါသည်။ Namespace import syntax နှင့် remote standard-library package များကို P1 verification ပြီးနောက် Ecosystem milestone အတွက် ဆက်လက်လုပ်ဆောင်ပါမည်။
