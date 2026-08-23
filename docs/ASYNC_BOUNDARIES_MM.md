# Zap Async Boundary များ

**အခြေအနေ:** Zap v2.2.7 အတွက် normative runtime-boundary လမ်းညွှန်

## ရည်ရွယ်ချက်

Zap တွင် language-runtime စမ်းသပ်မှုများနှင့် ကန့်သတ်ထားသော integration test များအတွက် deterministic၊ single-threaded task executor ရှိပြီး explicitly submitted bounded production I/O adapter များလည်း ရှိပါသည်။ ဤစာတမ်းသည် ထို contract များကို နောက်ပိုင်းတွင် ထည့်သွင်းနိုင်မည့် full production asynchronous reactor နှင့် ခွဲခြားဖော်ပြပါသည်။ Deterministic task runner ကို ပြည့်စုံသော network reactor၊ thread scheduler သို့မဟုတ် interruption mechanism အဖြစ် မဖော်ပြရပါ။

## လက်ရှိ deterministic executor

လက်ရှိ executor သည် task များကို ထည့်သွင်းသည့်အစီအစဉ်အတိုင်း သိမ်းဆည်းပြီး no-op waker ဖြင့် poll လုပ်ပါသည်။ `run_until_idle()` သည် သတ်မှတ်ထားသော အများဆုံး poll budget ကို အသုံးပြုပြီး `run_with_budget()` သည် poll အရေအတွက်၊ pending task အရေအတွက်နှင့် budget ကုန်ဆုံးခြင်း ရှိ/မရှိ ပါဝင်သော `RunReport` ကို ပြန်ပေးပါသည်။ Executor သည် task အများဆုံးအရေအတွက်နှင့် run တစ်ကြိမ်လျှင် poll အများဆုံးအရေအတွက်ကို ကန့်သတ်နိုင်ပါသည်။ Language `async fn` call များတွင် လက်ရှိသတ်မှတ်ထားသော eager scheduled-value contract ကို အသုံးပြုပါသည်။ Invocation အချိန်တွင် argument validation ပြုလုပ်ပြီး function body ကို observable effect များအပါအဝင် ချက်ချင်း execute လုပ်ကာ ပြီးစီးသော result ကို caller ၏ `RuntimeState` ထဲတွင် schedule လုပ်ပြီး `ScheduledFuture` task handle ပြန်ပေးသည်။ `await` နှင့် `task_join` သည် result ကို consume မလုပ်မီ context ပိုင် executor ကို drive လုပ်ပြီး `task_is_ready` သည် poll မလုပ်ဘဲ readiness ကို စောင့်ကြည့်သည်။

ဤ eager contract သည် လက်ရှိ language surface အတွက် ရည်ရွယ်ချက်ရှိရှိ သတ်မှတ်ထားခြင်းဖြစ်သည်။ Function execution ကို `await` သို့မဟုတ် `join` အချိန်အထိ ရွှေ့ဆိုင်းသည်ဟု မဆိုလိုပါ။ အနာဂတ် lazy-continuation design အတွက် capture၊ cancellation နှင့် context-lifetime semantics သီးခြားလိုအပ်ပြီး ဤ patch တွင် မပါဝင်ပါ။

| Contract | လက်ရှိအပြုအမူ |
|---|---|
| Scheduling | Cooperative၊ single-threaded polling ဖြစ်ပြီး task အစီအစဉ်မှာ deterministic ဖြစ်သည်။ Language task handle များကို လက်ရှိ `ExecutionContext` က ပိုင်ဆိုင်သည်။ |
| Wake-up | Operating-system reactor မရှိပါ။ Executor သည် no-op waker ကို အသုံးပြုသည်။ |
| Fairness | Poll budget နှင့် task အစီအစဉ်ပေါ်တွင် ကန့်သတ်ထားပြီး latency အာမခံချက် မရှိပါ။ |
| Shared state | Runtime task handle များသည် `Rc<RefCell<...>>` ကို အသုံးပြု၍ `Send`/`Sync` မဖြစ်ပါ။ |
| Failure | Context ပိုင် task တစ်ခုစီတွင် `Pending`၊ `Ready`၊ `Cancelled`၊ `TimedOut` သို့မဟုတ် `Joined` state ရှိသည်။ ပထမ join သည် known task result သို့မဟုတ် terminal error ကို consume လုပ်ပြီး id မတွေ့ပါက `UnknownTask`၊ join ထပ်လုပ်ပါက `AlreadyJoined` ကို ပြန်ပေးသည်။ |
| Cancellation | Cancellation token ကို wrapped future မ poll မီ စစ်ဆေးပြီး cancellation သည် cooperative ဖြစ်သည်။ |
| Limits | `max_tasks` နှင့် `max_polls_per_run` တို့က executor work အကန့်အသတ်မဲ့ ဖြစ်ခြင်းကို တားဆီးသည်။ |

Executor သည် deterministic language semantics၊ context-owned `ScheduledFuture` handle များ၊ unit test၊ conformance fixture နှင့် blocking မလုပ်သော သေးငယ်သည့် in-process task များအတွက် သင့်တော်ပါသည်။ Production-grade socket readiness၊ parallel CPU execution၊ preemptive fairness သို့မဟုတ် arbitrary code ကို အတင်းအကျပ် ရပ်တန့်နိုင်ခြင်းကို အာမခံရန် မသင့်တော်ပါ။

## Production boundary

လက်ရှိ production boundary တွင် explicitly submitted worker operations မှတစ်ဆင့် bounded file၊ TCP နှင့် process adapter များ ပါဝင်သော်လည်း general operating-system reactor မပါဝင်သေးပါ။ Full production asynchronous I/O layer သည် readiness event များကို စောင့်ဆိုင်းခြင်း၊ file descriptor များ register/remove လုပ်ခြင်း၊ timer များကို ကိုင်တွယ်ခြင်းနှင့် busy polling မလုပ်ဘဲ task များကို wake လုပ်ခြင်းတို့ ပါဝင်ရမည်။ ထို reactor သည် လက်ရှိ stable contract ၏ အပြင်ဘက်တွင် ရှိနေပြီး stable Zap API အဖြစ် မဖော်ပြမီ support ပြုမည့် platform များ၊ readiness semantics၊ timer precision၊ shutdown အပြုအမူနှင့် resource limits များကို သတ်မှတ်ရမည်။

Blocking system call များအတွက် explicit adapter boundary လိုအပ်ပါသည်။ Blocking filesystem operation၊ process wait၊ DNS lookup သို့မဟုတ် foreign-function call တို့ကို reactor thread ပေါ်တွင် မလုပ်ရပါ။ Production design သည် bounded blocking pool သို့မဟုတ် OS-specific cancellable operation တစ်ခုခုကို အသုံးပြုရမည်။ Cancellation request သည် result ကို စောင့်ဆိုင်းခြင်းကို ရပ်နိုင်သော်လည်း adapter က လုံခြုံသော interruption guarantee ကို documentation ဖြင့် မပေးထားလျှင် arbitrary blocking syscall ကို kill လုပ်နိုင်သည်ဟု မဖော်ပြရပါ။

Multi-thread scheduling သည် သီးခြား boundary တစ်ခုဖြစ်ပါသည်။ လက်ရှိ `Rc<RefCell>` task state ကို worker thread များအကြား ရွှေ့၍ မရပါ။ Production scheduler အတွက် `Send`/`Sync` လုံခြုံသော task state၊ ownership transfer rules၊ memory-ordering model၊ deterministic shutdown နှင့် worker count/queue depth limits များ လိုအပ်ပါသည်။ ထိုပြောင်းလဲမှုများသည် semantic နှင့် architectural change များဖြစ်သောကြောင့် လက်ရှိ executor မှ အလိုအလျောက် အဓိပ္ပါယ်ကောက်ယူ၍ မရပါ။

## Cancellation နှင့် timeout semantics

Cancellation သည် cooperative ဖြစ်ပြီး သတ်မှတ်ထားသော precedence ရှိပါသည်။ Cancellation-aware wrapper သည် inner future ကို poll မလုပ်မီ token ကို စစ်ဆေးပါသည်။ Language `task_cancel(future)` API သည် context ပိုင် pending task အတွက် cancellation request ပြုလုပ်ပြီး request လက်ခံ/မလက်ခံကို ပြန်ပေးသည်။ ထို့နောက် `task_join` က deterministic `Cancelled` failure ကို report လုပ်ကာ task ကို `Joined` သို့ ပြောင်းသည်။ `task_join_timeout(future, poll_budget)` သည် ပေးထားသော poll budget အများဆုံးအထိ drive လုပ်ပြီး task မ ready ဖြစ်သေးပါက `TimedOut` state သို့ ပြောင်းသည်။ ထို task ကို နောက်တစ်ကြိမ် join လုပ်ပါက result ကို ဒုတိယအကြိမ် မစားသုံးဘဲ `AlreadyJoined` ကို ပြန်ပေးသည်။ Known task တစ်ခု၏ ပထမ join သည် cancellation/timeout အပါအဝင် admitted-task budget slot တစ်ခုတည်းကိုသာ release လုပ်ပြီး unknown သို့မဟုတ် repeated join များသည် slot ကို မလျှော့ပါ။ Cancel လုပ်ထားသော task သည် တိတ်တဆိတ် ပျောက်ကွယ်သွားခြင်းမဟုတ်ဘဲ cancellation result ဖြင့် ပြီးဆုံးပါသည်။ Timeout ကို operation နှင့် timer future တို့၏ race အဖြစ် အကောင်အထည်ဖော်သင့်ပြီး shutdown တွင် timer နှင့် operation နှစ်ခုလုံး ပါဝင်ရမည်။ Timeout သည် underlying blocking operation ကို အတင်းအကျပ် terminate လုပ်ပြီးပြီဟု မဆိုလိုပါ။

Task error များသည် join handle များမှ typed result အဖြစ် ပြန့်ပွားပါသည်။ လက်ရှိ context တွင် id မရှိပါက `UnknownTask`၊ ပထမ result consumption ပြီးနောက် `AlreadyJoined` ကို ပြန်ပေးသည်။ `reset_for_run()` သည် scheduler ကို ဖယ်ရှားပြီး task budget ကိုလည်း တစ်ပြိုင်တည်း reset လုပ်သဖြင့် run ဟောင်းမှ handle များကို run အသစ်တွင် observe မလုပ်နိုင်ပါ။ Caller က join handle ကို drop လုပ်ပါက result ကို ဆက်လက် observe မလုပ်နိုင်တော့သော်လည်း API က ထိုအပြုအမူကို တိတိကျကျ သတ်မှတ်မထားလျှင် handle drop သည် အထွေထွေ cancellation guarantee မဟုတ်ပါ။ Production API များသည် cancellation သည် best effort ဟုတ်/မဟုတ်၊ completion မတိုင်မီ resource များ ပိတ်သိမ်းမည်/မည်မဟုတ်နှင့် reactor shutdown error များကို မည်သို့ report လုပ်မည်ကို သတ်မှတ်ရမည်။

## M2-VERIFY-02 platform-native matrix

GitHub Actions သည် single Linux run တစ်ခုကို cross-platform evidence ဟု မယူဆဘဲ native Linux x86_64၊ Windows x86_64 နှင့် macOS ARM64 target များတွင် focused matrix ကို run လုပ်ပါသည်။ Target တစ်ခုချင်းစီ၏ exact unit regression များတွင် worker admission၊ bounded TCP exchange၊ oversized request/response rejection၊ process output/status/cancellation၊ bounded file read၊ newline byte preservation နှင့် directory rejection တို့ ပါဝင်ပါသည်။ Linux နှင့် macOS တွင် shared Unix filesystem boundary ကို စစ်ဆေးသော Unix symlink rejection case ကိုလည်း ထပ်မံ run လုပ်ပါသည်။ Windows တွင် symlink policy အတွက် တူညီသော evidence ရှိသည်ဟု မဆိုဘဲ platform limitation ကို မှတ်တမ်းတင်ထားပါသည်။

Matrix သည် runner တိုင်းတွင် `scripts/test_platform_archive.sh` ကိုလည်း run လုပ်ပါသည်။ ထို regression သည် CRLF ပါသော tree အသေးတစ်ခုကို ဖန်တီး၍ deterministic tar.gz archive ကို နှစ်ကြိမ် build လုပ်ပြီး bytes တူညီမှု၊ sorted member name များနှင့် extract လုပ်ထားသော payload bytes တူညီမှုကို စစ်ဆေးပါသည်။ Runner တစ်ခုချင်းစီသည် target triple၊ runner operating system၊ Rust/Cargo version၊ exact test name များ၊ archive result နှင့် final status ပါသော target-named log ကို ရေးပါသည်။ Matrix သည် support ပြုထားသော CI target များတွင် repository ၏ document လုပ်ထားသော behavior ကိုသာ သက်သေပြပြီး arbitrary third-party binary သို့မဟုတ် foreign operating-system call များ portable ဖြစ်သည်ဟု မဆိုပါ။

## Stability rules

Deterministic executor၊ eager language async scheduled-value contract နှင့် context-owned language scheduling boundary သည် v2.2.7 အတွက် stable baseline ဖြစ်ပါသည်။ API အသစ်တိုင်းသည် deterministic-only၊ reactor-backed သို့မဟုတ် blocking-adapted ဟုတ်/မဟုတ် ဖော်ပြရမည်။ Documentation နှင့် diagnostics များတွင်လည်း ထိုတူညီသော စကားလုံးများကို အသုံးပြုရမည်။ သက်ဆိုင်ရာ semantic၊ reactor နှင့် platform gates မရှိမချင်း release note သို့မဟုတ် benchmark တစ်ခုခုတွင် lazy continuation၊ parallel scheduling သို့မဟုတ် production non-blocking I/O ရှိသည်ဟု မဆိုရပါ။

အနာဂတ် production implementation တွင် အနည်းဆုံး အောက်ပါအချက်များ ပါဝင်ရမည်။

၁။ Platform-specific readiness backend များနှင့် deterministic test backend ပါဝင်သော reactor abstraction တစ်ခု။
၂။ Timer registration၊ cancellation နှင့် monotonic-clock rules များ။
၃။ Operation တစ်မျိုးချင်းစီအလိုက် shutdown နှင့် cancellation behavior ကို သတ်မှတ်ထားသော bounded blocking adapter တစ်ခု။
၄။ `Send`/`Sync` လုံခြုံသော scheduler state သို့မဟုတ် single-thread-only API boundary ကို explicit ပြုလုပ်ထားခြင်း။
၅။ Supported platform တိုင်းတွင် socket readiness၊ process/filesystem adapter၊ timeout propagation၊ cancellation race၊ shutdown နှင့် resource limits အတွက် integration tests များ။

## Verification

လက်ရှိ contract ကို bounded polling၊ task limits၊ eager async invocation output ordering၊ explicit terminal state၊ one-time admitted-task release၊ unknown/repeated join၊ context-owned language-task readiness/completion၊ scheduler reset isolation၊ language `task_cancel` နှင့် `task_join_timeout`၊ cancellation precedence၊ timeout behavior၊ child-process cancellation နှင့် M2-VERIFY-02 target-native filesystem/process/socket/archive case များအတွက် native tests များဖြင့် စစ်ဆေးထားပါသည်။ ထို tests များသည် deterministic semantics နှင့် document လုပ်ထားသော adapter boundary များကိုသာ စစ်ဆေးပြီး lazy continuation၊ production reactor သို့မဟုတ် arbitrary blocking work အတွက် forced cancellation ကို အတည်ပြုခြင်း မဟုတ်ပါ။
