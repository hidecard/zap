# Runtime State နှင့် Execution Context

**အခြေအနေ:** Zap v2.1.12 အတွက် first slice ကို အကောင်အထည်ဖော်ပြီး

ဤစာတမ်းသည် explicit runtime-state boundary ၏ ပထမဆုံးအပိုင်းကို သတ်မှတ်ပါသည်။ Evaluator ဆိုင်ရာ state အားလုံးကို object တစ်ခုတည်းသို့ ရွှေ့ပြီးပြီဟု မဆိုလိုဘဲ လက်ရှိရွှေ့ပြီးသော state နှင့် နောက်ပိုင်းလုပ်ရမည့် boundary များကို မှတ်တမ်းတင်ထားပါသည်။

## ရည်ရွယ်ချက်

Source execution တစ်ကြိမ်စီတွင် ကိုယ်ပိုင် `ExecutionContext` တစ်ခု ရရှိပါသည်။ ဤ context သည် independent run၊ test သို့မဟုတ် အနာဂတ် runtime instance များအကြား မရောနှောသင့်သော mutable state ကို ပိုင်ဆိုင်ပါသည်။ ထိုသို့ module cache၊ import-cycle tracking နှင့် execution-depth accounting တို့၏ process-global thread-local ownership ကို ဖယ်ရှားထားပါသည်။

## State ပိုင်ဆိုင်မှု

| State | ပိုင်ရှင် | Contract |
|---|---|---|
| Module cache | `RuntimeState` | Cached module value နှင့် function များသည် လက်ရှိ execution context အတွင်းတွင်သာ ရှိရမည်။ |
| Import-cycle stack | `RuntimeState` | Active module chain ကို explicit စောင့်ကြည့်ပြီး run reset ပြုလုပ်သောအခါ ရှင်းလင်းရမည်။ |
| Execution depth | `RuntimeState` | Nested AST နှင့် legacy execution များသည် context တစ်ခုအတွက် bounded counter တစ်ခုကို မျှဝေသုံးရမည်။ |
| Source workspace confinement | `RuntimeState` | Canonical workspace root ကို context အတွက် တစ်ကြိမ်သတ်မှတ်ပြီး nested module/function call များက အတူတူအသုံးပြုရမည်။ Run reset တွင် ရှင်းလင်းရမည်။ |
| LSP open documents | `LspState` | LSP server session တစ်ခုချင်းစီတွင် ကိုယ်ပိုင် document map ရှိပြီး independent server state များအကြား open-document content မရောနှောရပါ။ |
| Heap statistics နှင့် object ownership | ရှိပြီးသား value boundary | Memory accounting သည် ရှိပြီးသား bounded memory contract အတိုင်း ဆက်လက်ထိန်းချုပ်သည်။ |

## ExecutionContext လည်ပတ်ပုံ

Native entrypoint သည် run စတင်ချိန်တွင် `ExecutionContext` ဖန်တီးပြီး source မ evaluate မီ reset ပြုလုပ်ပါသည်။ Context ကို expression parser၊ AST evaluator၊ legacy evaluator၊ function နှင့် method call၊ object-field initialization နှင့် module loading များမှတစ်ဆင့် ဖြန့်ဝေထားပါသည်။ ထို့ကြောင့် imported module များသည် process-global cache အစား caller ၏ context ကို အသုံးပြုပါသည်။ Workspace သတ်မှတ်သည့် ပထမ AST execution က canonical root ကို `RuntimeState` တွင် သိမ်းထားပြီး nested execution များသည် process working directory ဖြင့် အစားထိုးခြင်းမပြုဘဲ ထို root ကို ဆက်အသုံးပြုပါသည်။ Filesystem builtin များသည် context-aware boundary တစ်ခုတည်းကို အသုံးပြုပါသည်။

Context တစ်ခုကို အခြား context တစ်ခုနှင့် သီးခြားဖန်တီးနိုင်ပါသည်။ Context တစ်ခု၏ module stack သို့မဟုတ် execution-depth counter ကို ပြောင်းလဲခြင်းသည် အခြား context ကို မပြောင်းလဲစေပါ။ Context ကို ပြန်အသုံးပြုမည်ဆိုပါက reset ပြုလုပ်ခြင်းဖြင့် module cache၊ import stack နှင့် depth counter များကို ရှင်းလင်းနိုင်ပါသည်။

## လုံခြုံရေး boundary များ

ရွှေ့ပြောင်းထားသော state သည် execution instance သို့မဟုတ် LSP server session တစ်ခုက ပိုင်ဆိုင်သော single-threaded state ဖြစ်ပါသည်။ `Send`/`Sync` claim၊ worker sharing၊ tracing garbage collection၊ weak reference၊ cumulative byte accounting၊ သို့မဟုတ် language-level task scheduler အသစ်များကို ဤ implementation တွင် မထည့်သွင်းပါ။ လက်ရှိ execution-depth limit သည် bounded ဖြစ်ပါသည်။ Parser ပိုင် source များသည် canonical AST execution ကို အသုံးပြုပြီး line interpreter သည် legacy line-bodied function record များအတွက် explicit compatibility-only အဖြစ်သာ ရှိပါသည်။

## Regression evidence

Runtime-state module တွင် workspace၊ isolation နှင့် reset regression များ ပါဝင်ပါသည်။ LSP module တွင် independent-server document isolation coverage ပါဝင်ပါသည်။ Native suite သည် context-aware call graph မှတစ်ဆင့် AST execution၊ legacy compatibility၊ module import၊ circular-import diagnostic၊ function call၊ method call၊ filesystem confinement နှင့် bounded execution depth များကိုလည်း စစ်ဆေးပါသည်။

ဤ migration slice ၏ acceptance criterion မှာ module၊ depth နှင့် workspace state များသည် execution context ပိုင်ဆိုင်မှုဖြစ်ပြီး context များအကြား မပေါက်ကြားစေရန် ဖြစ်ပါသည်။ LSP document map များသည် server session ပိုင်ဆိုင်မှုဖြစ်ပြီး server state များအကြား မပေါက်ကြားရပါ။ ရှိပြီးသား language နှင့် editor behavior မပြောင်းလဲရပါ။ နောက်ပိုင်း slice များတွင် capability၊ diagnostics၊ memory နှင့် cancellation state များကို explicit boundary များအဖြစ် ဆက်ရွှေ့နိုင်ပါသည်။

## Deferred roadmap

အောက်ပါအလုပ်များသည် သီးခြားကျန်ရှိနေပါသည် — ကျန်ရှိသော hidden state များကို `RuntimeState`/`ExecutionContext` သို့ ပြောင်းခြင်း၊ first-class function value နှင့် `EnvFrame`၊ object-store နှင့် weak-reference policy၊ per-run memory budget၊ typed source-span propagation နှင့် full language-level async task semantics တို့ ဖြစ်ပါသည်။

ထိန်းသိမ်းထားသော contract များအတွက် [Burmese documentation navigation hub](DOCUMENTATION_NAVIGATION_MM.md)၊ [next-step plan](NEXT_TODO_PLAN_MM.md) နှင့် [language specification](LANGUAGE_SPEC_MM.md) ကို ကြည့်ရှုပါ။

## References

[1]: DOCUMENTATION_NAVIGATION_MM.md "Zap Burmese documentation navigation"
[2]: NEXT_TODO_PLAN_MM.md "Zap Burmese next-step plan"
[3]: LANGUAGE_SPEC_MM.md "Zap Burmese language specification"
