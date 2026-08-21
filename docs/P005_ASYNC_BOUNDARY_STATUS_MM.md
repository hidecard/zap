# P0-05-A Async Boundary Status

## အကောင်အထည်ဖော်ပြီးသောအပိုင်း

P0-05 Deterministic နှင့် production async boundary ၏ ပထမ slice ကို အကောင်အထည်ဖော်ပြီးဖြစ်သည်။ `async_capabilities()` သည် argument မလိုသော deterministic runtime builtin ဖြစ်ပြီး async runtime ၏ လက်ရှိ semantic boundary ကို stable map အဖြစ် ပြန်ပေးသည်။

Report ထဲတွင် single-threaded poll-budget executor၊ fixed worker adapter၊ bounded non-blocking TCP adapter၊ bounded deadline/output process adapter၊ terminate-then-drain process cancellation၊ eager language-level future facade၊ deferred language-level scheduling/cancellation/timeout နှင့် arbitrary foreign blocking call ကို interrupt မလုပ်နိုင်ခြင်းတို့ကို ဖော်ပြထားသည်။ Worker count၊ task count၊ read/socket/process-output limit နှင့် timeout defaults များကိုလည်း ဖော်ပြသည်။

`async_capabilities()` သည် worker မစတင်ပါ၊ network/process operation မလုပ်ပါ၊ task scheduling ကို မပြောင်းပါ။ ထို့ကြောင့် capability report သည် descriptive နှင့် deterministic ဖြစ်ပြီး deployment policy အတွက် runtime boundary ကို မရောထွေးစေပါ။ P0-05-B တွင် worker/task/read/socket/process limits များကို admission မတိုင်မီ typed preflight validation ဖြင့် စစ်ဆေးပြီး invalid-limit error များကို deterministic ဖြစ်အောင် ထည့်သွင်းထားသည်။ Configured socket bound ထက်ကြီးသော TCP request ကိုလည်း queue admission မတိုင်မီ reject လုပ်သည်။

## Test evidence

Runtime direct-builtin test သည် stable capability fields၊ default worker limit၊ resource-limit preflight status နှင့် zero-argument contract ကို စစ်ဆေးသည်။ Native AST integration test သည် `.zp` source မှ `async_capabilities()` ကို ခေါ်နိုင်ကြောင်းနှင့် process cancellation boundary ကို report လုပ်ကြောင်း စစ်ဆေးသည်။ Async runtime tests များသည် zero worker/task/read limits၊ zero adapter limits၊ oversized TCP request နှင့် queue admission မတိုင်မီ rejection များကို စစ်ဆေးသည်။ Public builtin catalog တွင် async domain အောက်၌ API entry ထည့်ထားသည်။

## နောက်ထပ် P0-05 အလုပ်များ

P0-05-B ၏ typed resource-limit preflight validation နှင့် TCP request-size admission check များ ပြီးစီးပြီ။ P0-05-C တွင် Linux x86_64၊ Windows x86_64 နှင့် macOS ARM64 target များအတွက် တူညီသော focused async matrix script၊ target-named CI log artifact နှင့် versioned limitation policy ကို ထည့်သွင်းထားသည်။ Matrix သည် worker concurrency၊ invalid-limit preflight၊ TCP round trip/response/request bounds၊ platform-native process output/cancellation နှင့် bounded file read များကို စစ်ဆေးသည်။ Executor-backed language-level scheduling နှင့် language-level cancellation/timeout controls ကိုလည်း နောက်ပိုင်း runtime slice အဖြစ်သာ ဆက်လုပ်ရမည်။

P1-05-A တွင် fixed-seed property/fuzz replay နှင့် parser၊ JSON၊ lockfile၊ registry၊ memory၊ async boundary failure corpus များကို CI artifact အဖြစ် ထိန်းသိမ်းရန် ဆက်လုပ်ရမည်။ ထို့နောက် P0-01 native/legacy parity report နှင့် P0-02 specification ownership index ကို ဆက်လုပ်မည်။
