# Zap Async Boundary များ

**အခြေအနေ:** Zap v2.1.11 အတွက် normative runtime-boundary လမ်းညွှန်

## ရည်ရွယ်ချက်

Zap တွင် language-runtime စမ်းသပ်မှုများနှင့် ကန့်သတ်ထားသော integration test များအတွက် deterministic၊ single-threaded task executor ရှိပြီး explicitly submitted bounded production I/O adapter များလည်း ရှိပါသည်။ ဤစာတမ်းသည် ထို contract များကို နောက်ပိုင်းတွင် ထည့်သွင်းနိုင်မည့် full production asynchronous reactor နှင့် ခွဲခြားဖော်ပြပါသည်။ Deterministic task runner ကို ပြည့်စုံသော network reactor၊ thread scheduler သို့မဟုတ် interruption mechanism အဖြစ် မဖော်ပြရပါ။

## လက်ရှိ deterministic executor

လက်ရှိ executor သည် task များကို ထည့်သွင်းသည့်အစီအစဉ်အတိုင်း သိမ်းဆည်းပြီး no-op waker ဖြင့် poll လုပ်ပါသည်။ `run_until_idle()` သည် သတ်မှတ်ထားသော အများဆုံး poll budget ကို အသုံးပြုပြီး `run_with_budget()` သည် poll အရေအတွက်၊ pending task အရေအတွက်နှင့် budget ကုန်ဆုံးခြင်း ရှိ/မရှိ ပါဝင်သော `RunReport` ကို ပြန်ပေးပါသည်။ Executor သည် task အများဆုံးအရေအတွက်နှင့် run တစ်ကြိမ်လျှင် poll အများဆုံးအရေအတွက်ကို ကန့်သတ်နိုင်ပါသည်။

| Contract | လက်ရှိအပြုအမူ |
|---|---|
| Scheduling | Cooperative၊ single-threaded polling ဖြစ်ပြီး task အစီအစဉ်မှာ deterministic ဖြစ်သည်။ |
| Wake-up | Operating-system reactor မရှိပါ။ Executor သည် no-op waker ကို အသုံးပြုသည်။ |
| Fairness | Poll budget နှင့် task အစီအစဉ်ပေါ်တွင် ကန့်သတ်ထားပြီး latency အာမခံချက် မရှိပါ။ |
| Shared state | Runtime task handle များသည် `Rc<RefCell<...>>` ကို အသုံးပြု၍ `Send`/`Sync` မဖြစ်ပါ။ |
| Failure | Join handle များသည် task failure သို့မဟုတ် cancellation ကို explicit result အဖြစ် ထိန်းသိမ်းသည်။ |
| Cancellation | Cancellation token ကို wrapped future မ poll မီ စစ်ဆေးပြီး cancellation သည် cooperative ဖြစ်သည်။ |
| Limits | `max_tasks` နှင့် `max_polls_per_run` တို့က executor work အကန့်အသတ်မဲ့ ဖြစ်ခြင်းကို တားဆီးသည်။ |

Executor သည် deterministic language semantics၊ unit test၊ conformance fixture နှင့် blocking မလုပ်သော သေးငယ်သည့် in-process task များအတွက် သင့်တော်ပါသည်။ Production-grade socket readiness၊ parallel CPU execution၊ preemptive fairness သို့မဟုတ် arbitrary code ကို အတင်းအကျပ် ရပ်တန့်နိုင်ခြင်းကို အာမခံရန် မသင့်တော်ပါ။

## Production boundary

လက်ရှိ production boundary တွင် explicitly submitted worker operations မှတစ်ဆင့် bounded file၊ TCP နှင့် process adapter များ ပါဝင်သော်လည်း general operating-system reactor မပါဝင်သေးပါ။ Full production asynchronous I/O layer သည် readiness event များကို စောင့်ဆိုင်းခြင်း၊ file descriptor များ register/remove လုပ်ခြင်း၊ timer များကို ကိုင်တွယ်ခြင်းနှင့် busy polling မလုပ်ဘဲ task များကို wake လုပ်ခြင်းတို့ ပါဝင်ရမည်။ ထို reactor သည် လက်ရှိ stable contract ၏ အပြင်ဘက်တွင် ရှိနေပြီး stable Zap API အဖြစ် မဖော်ပြမီ support ပြုမည့် platform များ၊ readiness semantics၊ timer precision၊ shutdown အပြုအမူနှင့် resource limits များကို သတ်မှတ်ရမည်။

Blocking system call များအတွက် explicit adapter boundary လိုအပ်ပါသည်။ Blocking filesystem operation၊ process wait၊ DNS lookup သို့မဟုတ် foreign-function call တို့ကို reactor thread ပေါ်တွင် မလုပ်ရပါ။ Production design သည် bounded blocking pool သို့မဟုတ် OS-specific cancellable operation တစ်ခုခုကို အသုံးပြုရမည်။ Cancellation request သည် result ကို စောင့်ဆိုင်းခြင်းကို ရပ်နိုင်သော်လည်း adapter က လုံခြုံသော interruption guarantee ကို documentation ဖြင့် မပေးထားလျှင် arbitrary blocking syscall ကို kill လုပ်နိုင်သည်ဟု မဖော်ပြရပါ။

Multi-thread scheduling သည် သီးခြား boundary တစ်ခုဖြစ်ပါသည်။ လက်ရှိ `Rc<RefCell>` task state ကို worker thread များအကြား ရွှေ့၍ မရပါ။ Production scheduler အတွက် `Send`/`Sync` လုံခြုံသော task state၊ ownership transfer rules၊ memory-ordering model၊ deterministic shutdown နှင့် worker count/queue depth limits များ လိုအပ်ပါသည်။ ထိုပြောင်းလဲမှုများသည် semantic နှင့် architectural change များဖြစ်သောကြောင့် လက်ရှိ executor မှ အလိုအလျောက် အဓိပ္ပါယ်ကောက်ယူ၍ မရပါ။

## Cancellation နှင့် timeout semantics

Cancellation သည် cooperative ဖြစ်ပြီး သတ်မှတ်ထားသော precedence ရှိပါသည်။ Cancellation-aware wrapper သည် inner future ကို poll မလုပ်မီ token ကို စစ်ဆေးပါသည်။ Cancel လုပ်ထားသော task သည် တိတ်တဆိတ် ပျောက်ကွယ်သွားခြင်းမဟုတ်ဘဲ cancellation result ဖြင့် ပြီးဆုံးပါသည်။ Timeout ကို operation နှင့် timer future တို့၏ race အဖြစ် အကောင်အထည်ဖော်သင့်ပြီး shutdown တွင် timer နှင့် operation နှစ်ခုလုံး ပါဝင်ရမည်။ Timeout သည် underlying blocking operation ကို အတင်းအကျပ် terminate လုပ်ပြီးပြီဟု မဆိုလိုပါ။

Task error များသည် join handle များမှ typed result အဖြစ် ပြန့်ပွားပါသည်။ Caller က join handle ကို drop လုပ်ပါက result ကို ဆက်လက် observe မလုပ်နိုင်တော့သော်လည်း API က ထိုအပြုအမူကို တိတိကျကျ သတ်မှတ်မထားလျှင် handle drop သည် အထွေထွေ cancellation guarantee မဟုတ်ပါ။ Production API များသည် cancellation သည် best effort ဟုတ်/မဟုတ်၊ completion မတိုင်မီ resource များ ပိတ်သိမ်းမည်/မည်မဟုတ်နှင့် reactor shutdown error များကို မည်သို့ report လုပ်မည်ကို သတ်မှတ်ရမည်။

## Stability rules

Deterministic executor သည် v2.1.x အတွက် stable baseline ဖြစ်ပါသည်။ API အသစ်တိုင်းသည် deterministic-only၊ reactor-backed သို့မဟုတ် blocking-adapted ဟုတ်/မဟုတ် ဖော်ပြရမည်။ Documentation နှင့် diagnostics များတွင်လည်း ထိုတူညီသော စကားလုံးများကို အသုံးပြုရမည်။ သက်ဆိုင်ရာ reactor နှင့် platform gates မရှိမချင်း release note သို့မဟုတ် benchmark တစ်ခုခုတွင် parallel scheduling သို့မဟုတ် production non-blocking I/O ရှိသည်ဟု မဆိုရပါ။

အနာဂတ် production implementation တွင် အနည်းဆုံး အောက်ပါအချက်များ ပါဝင်ရမည်။

၁။ Platform-specific readiness backend များနှင့် deterministic test backend ပါဝင်သော reactor abstraction တစ်ခု။
၂။ Timer registration၊ cancellation နှင့် monotonic-clock rules များ။
၃။ Operation တစ်မျိုးချင်းစီအလိုက် shutdown နှင့် cancellation behavior ကို သတ်မှတ်ထားသော bounded blocking adapter တစ်ခု။
၄။ `Send`/`Sync` လုံခြုံသော scheduler state သို့မဟုတ် single-thread-only API boundary ကို explicit ပြုလုပ်ထားခြင်း။
၅။ Supported platform တိုင်းတွင် socket readiness၊ process/filesystem adapter၊ timeout propagation၊ cancellation race၊ shutdown နှင့် resource limits အတွက် integration tests များ။

## Verification

လက်ရှိ contract ကို bounded polling၊ task limits၊ join result၊ cancellation precedence၊ timeout behavior နှင့် child-process cancellation အတွက် native tests များဖြင့် စစ်ဆေးထားပါသည်။ ထို tests များသည် deterministic semantics ကိုသာ စစ်ဆေးပြီး production reactor သို့မဟုတ် arbitrary blocking work အတွက် forced cancellation ကို အတည်ပြုခြင်း မဟုတ်ပါ။
