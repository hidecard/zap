# P0-05-B Async Resource-Limit Preflight Plan

## ရည်ရွယ်ချက်

Production-oriented async adapter တိုင်းသည် invalid သို့မဟုတ် အရွယ်အစားကျော်လွန်သော resource request များကို admission မပြုမီ ငြင်းပယ်နိုင်ရမည်။ Error များ deterministic ဖြစ်ရမည်၊ output/deadline/cancellation cleanup များ bounded ဖြစ်ရမည်၊ Rust 1.75 compatibility boundary ကို မပြောင်းရပါ။

## ဤအဆင့်၏ scope

ဤအဆင့်တွင် fixed-worker scheduler၊ regular-file read adapter၊ bounded TCP adapter နှင့် bounded process adapter များကို လွှမ်းခြုံမည်။ Language-level `async/await` scheduling၊ arbitrary foreign-blocking interruption နှင့် platform-specific CI provisioning များကို မထည့်သေးဘဲ နောက် P0-05/P1-05 အလုပ်အဖြစ် ဆက်ထားမည်။

| ID | လိုအပ်ချက် | Acceptance evidence |
|---|---|---|
| P005B-01 | Worker creation မတိုင်မီ `ThreadRuntimeLimits` ကို validate လုပ်ရန် | Zero workers၊ zero tasks နှင့် zero read bytes များအတွက် stable typed errors ပြန်ပေးပြီး invalid limit တွင် worker thread မစတင်ခြင်း |
| P005B-02 | Adapter task admission မတိုင်မီ adapter limits များကို validate လုပ်ရန် | Zero socket bytes၊ zero socket timeout၊ zero process-output bytes နှင့် zero process timeout များကို deterministic အတိုင်း ငြင်းပယ်ခြင်း |
| P005B-03 | Configured socket/read bound ထက်ကြီးသော TCP request ကို queue admission မတိုင်မီ ငြင်းပယ်ရန် | Typed input-limit error ပြန်ပေးပြီး task slot မစားသုံးခြင်း |
| P005B-04 | ရှိပြီးသား output/deadline/cancellation behavior ကို ထိန်းသိမ်းရန် | Oversized response၊ oversized process output၊ deadline နှင့် terminate-then-drain tests များ ဆက်လက် pass ဖြစ်ခြင်း |
| P005B-05 | Caller အားလုံးအတွက် error များ stable ဖြစ်ရန် | Error variants/messages များ deterministic ဖြစ်ပြီး address/secret မထုတ်ဖော်ခြင်း၊ English/Burmese docs တွင် မှတ်တမ်းတင်ခြင်း |
| P005B-06 | Capability reporting ကို ရိုးသားစွာ ထိန်းရန် | `async_capabilities()` တွင် resource-limit preflight enforce လုပ်ထားကြောင်း ပြပြီး language-level scheduling support ရှိသည်ဟု မဆိုခြင်း |

## Implementation အစီအစဉ်

ပထမဦးစွာ limit struct များတွင် typed validation methods ထည့်ပြီး thread creation သို့မဟုတ် adapter admission မတိုင်မီ ခေါ်ရမည်။ ထို့နောက် TCP request-size preflight check ထည့်ရမည်။ နောက်တစ်ဆင့်တွင် zero/oversized boundary တစ်ခုချင်းစီအတွက် focused unit tests ထည့်ပြီး successful I/O၊ response limit၊ process limit၊ cancellation နှင့် worker admission integration tests များကို မပျက်စေရပါ။ နောက်ဆုံးတွင် async runtime contract၊ standard-library index၊ changelog နှင့် P0/P1 register များကို update လုပ်ရမည်။

## Release gates

Rustfmt၊ strict Clippy၊ full native suite၊ focused P0-05-B tests၊ `git diff --check` နှင့် English/Burmese documentation parity အားလုံး pass ဖြစ်မှသာ ဤအဆင့် ပြီးစီးသည်ဟု သတ်မှတ်ရမည်။ နောက်ပိုင်း cross-platform matrix provisioning နှင့် သီးခြား review/revert လုပ်နိုင်ရန် ဤ commit ကို နောက် P0/P1 အလုပ်များနှင့် မရောသင့်ပါ။
