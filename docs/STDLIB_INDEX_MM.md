# Zap Standard Library Public Modules

Zap ၏ standard library ကို အများပြည်သူအသုံးပြုနိုင်သော domain များအဖြစ် စနစ်တကျ ခွဲခြားထားပါသည်။ Runtime dispatch ကို compatibility အတွက် ဗဟိုမှ ဆက်လက်လုပ်ဆောင်သော်လည်း ဤ index သည် documentation၊ tooling နှင့် နောင်တွင် ထည့်သွင်းမည့် package modules များအတွက် တည်ငြိမ်သော public organization ကို သတ်မှတ်ပေးပါသည်။

| Public module | အသုံးပြုနိုင်သောအပိုင်း | အဓိက API များ |
|---|---|---|
| `text` | စာသားပြောင်းလဲခြင်းနှင့် စီမံခြင်း | `len`, `str`, `type`, `upper`, `lower`, `trim`, `split`, `join`, `contains`, `replace`, `char_at`, `substring`, `codepoints` |
| `math` | ကိန်းဂဏန်းလုပ်ဆောင်ချက်များ | `abs`, `min`, `max`, `pow`, `sqrt` |
| `collections` | List နှင့် map များ | `sum`, `range`, `keys`, `entries`, `enumerate`, `count`, `reverse`, `get` |
| `filesystem` | ကန့်သတ်ထားသော text နှင့် line I/O | `read_text`, `write_text`, `read_lines`, `write_lines`, `exists`, `file_metadata`, `atomic_write` |
| `json` | JSON ပြောင်းလဲခြင်းနှင့် runtime category စစ်ဆေးခြင်း | `json`, `from_json`, `from_json_typed` |
| `system` | Environment၊ configuration နှင့် path များ | `env`, `has_env`, `env_get`, `config_dir`, `config_path`, `path_join`, `basename`, `dirname`, `now`, `sleep` |
| `time` | UTC timestamp နှင့် sign ပါသော duration ခွဲခြမ်းခြင်း | `utc_now`, `duration_parts`, `duration_between` |
| `logging` | Deterministic structured log record နှင့် JSON line များ | `log_record`, `log_json` |
| `runtime` | Bounded memory diagnostics နှင့် capability reporting | `memory_stats` |
| `async` | Deterministic task၊ cancellation၊ timeout နှင့် capability reporting | `spawn`, `task_join`, `task_is_ready`, `async_capabilities` |
| `network` | URL စီမံခြင်း၊ ကန့်သတ်ထားသော HTTP request နှင့် local server | `url_parse`, `url_encode`, `url_decode`, `http_get`, `http_request`, `http_serve_once` |
| `process` | Shell မသုံးသော process execution | `process_run` |

Public builtin အားလုံးသည် argument များကို တိကျစွာ စစ်ဆေးပြီး မမှန်ကန်သော input များကို မျက်ကွယ်မပြုဘဲ structured runtime error ပြန်ပေးပါသည်။ `runtime` domain တွင် live object၊ object allocation/deallocation၊ value-size limit နှင့် deferred capability fields များပါသော `memory_stats()` ကို ပေးထားပါသည်။ Public weak reference များကို unsupported နှင့် tracing collection ကို not implemented ဟု ဖော်ပြထားပါသည်။ Public builtin boundary များသည် oversized သို့မဟုတ် အလွန်နက်ရှိုင်း/စက်ဝိုင်းပါသော value graph များကို deterministic အတိုင်း ငြင်းပယ်ပါသည်။ Filesystem၊ JSON နှင့် HTTP response operation များတွင် 8 MiB safety limit ကို အသုံးပြုပါသည်။ URL input များကို 8 KiB အထိသာ ခွင့်ပြုပါသည်။ `process_run` သည် shell interpretation မလုပ်ဘဲ program ကို တိုက်ရိုက်ခေါ်ယူပြီး text command နှင့် text argument list ကိုသာ လက်ခံကာ UTF-8 stdout/stderr ကို ဖမ်းယူပါသည်။ Output 1 MiB ကျော်လျှင် ငြင်းပယ်ပါသည်။ HTTP request များသည် `http` နှင့် `https` URL များကိုသာ လက်ခံပြီး connect၊ read နှင့် write timeout များကို ကန့်သတ်ထားပါသည်။ `http_serve_once` သည် `127.0.0.1` loopback တွင် bind လုပ်ကာ request တစ်ခုတည်းကိုသာ serve ပြီး request 64 KiB၊ response 8 MiB နှင့် wait time 10 စက္ကန့် ကန့်သတ်ချက်များကို အသုံးပြုပါသည်။ `env_get` သည် process environment ကို မပြောင်းလဲဘဲ default text တန်ဖိုး ပြန်ပေးပါသည်။ `config_dir` သည် Unix-like system များတွင် XDG configuration စည်းမျဉ်း၊ macOS တွင် `Application Support` နှင့် Windows တွင် `APPDATA`/`LOCALAPPDATA` ကို အသုံးပြုပါသည်။ `config_path` သည် relative file name တစ်ခုတည်းကိုသာ လက်ခံပြီး path separator နှင့် traversal component များကို ငြင်းပယ်ပါသည်။ `time` API များသည် UTC နှင့် integer millisecond precision ကို အသုံးပြုပါသည်။ `utc_now()` သည် `unix_seconds` နှင့် `unix_millis` ကို ပြန်ပေးသည်။ `duration_parts(milliseconds)` သည် sign ကို ထိန်းသိမ်းထားသော `days`၊ `hours`၊ `minutes`၊ `seconds`၊ `millis` နှင့် `milliseconds` များကို ပြန်ပေးသည်။ `duration_between(end_millis, start_millis)` သည် စစ်ဆေးထားသော `end_millis - start_millis` ကွာခြားချက်ကို ခွဲခြမ်းပေးသည်။ Overflow ဖြစ်ပါက wrap မလုပ်ဘဲ runtime error ပြန်ပေးပါသည်။ `logging` API များသည် pure record builder များဖြစ်ပါသည်။ `log_record(level, message, fields)` သည် `level`၊ `message` နှင့် `fields` ပါသော map ကို ပြန်ပေးပြီး `log_json(level, message, fields)` သည် field name များကို အက္ခရာစဉ်အလိုက် စီထားသော canonical JSON line တစ်ကြောင်းကို ပြန်ပေးပါသည်။ Level များကို `trace`၊ `debug`၊ `info`၊ `warn` နှင့် `error` များသာ ခွင့်ပြုထားပြီး message ကို 8 KiB၊ fields ကို 64 ခု၊ field name ကို 256 bytes နှင့် encoded output ကို 64 KiB အထိ ကန့်သတ်ထားပါသည်။ ဤ API များသည် process stream များသို့ တိုက်ရိုက်မရေးသဖြင့် output သည် deterministic ဖြစ်ပြီး application က မိမိလိုအပ်သော sink ကို ရွေးချယ်နိုင်ပါသည်။

Async domain သည် eager future များကို အသုံးပြုသော ပထမဆုံး language-level task facade ကို ပေးထားပါသည်။ `async_capabilities()` သည် ဘယ်အလုပ်က deterministic၊ worker-backed၊ bounded၊ cancellable၊ deferred သို့မဟုတ် unsupported ဖြစ်သည်ကို report လုပ်ပေးပြီး typed resource-limit preflight ကို enforce လုပ်ထားကြောင်းလည်း ဖော်ပြသည်။ ၎င်းသည် descriptive သာဖြစ်ကာ worker၊ network သို့မဟုတ် process operation တစ်ခုမျှ မစတင်ပါ။ Public catalog သည် deterministic ဖြစ်ပါသည်။ Builtin တစ်ခုစီသည် တစ်ကြိမ်သာ ပါဝင်ပြီး domain တစ်ခုတည်းနှင့်သာ သက်ဆိုင်ပါသည်။ Native runtime ထဲတွင် tooling နှင့် tests များအသုံးပြုသော catalog ပါရှိသဖြင့် documentation နှင့် implementation တို့ကို တစ်ပြေးညီ ထိန်းသိမ်းနိုင်ပါသည်။

```zap
let endpoint = url_parse("https://example.com:8443/api?q=zap")
say endpoint["host"]
say url_encode("a b/c")

let result = process_run("printf", ["zap"])
say result["success"]
say result["stdout"]

let fallback = env_get("ZAP_OPTIONAL_SETTING", "default")
let settings = config_path("settings.json")

let current = utc_now()
let elapsed = duration_between(current["unix_millis"], current["unix_millis"] - 1500)
say elapsed["milliseconds"]

let event = log_record("info", "server started", {"port": 8080, "mode": "dev"})
say log_json(event["level"], event["message"], event["fields"])

# Loopback တွင် request တစ်ခု serve ပြီး metadata ပြန်ပေးသည်။
let served = http_serve_once(8080, "Hello from Zap")
say served["path"]
```

လက်ရှိ release line တွင် API များကို direct builtin များအဖြစ် အသုံးပြုနိုင်ပါသည်။ Namespace import syntax နှင့် remote standard-library package များကို P1 verification ပြီးနောက် Ecosystem milestone အတွက် ဆက်လက်လုပ်ဆောင်ပါမည်။
