# MemoryBudget နှင့် ObjectStore Contract

**Design status:** M2-MEM-02 logical accounting နှင့် rollback slice

**စစ်ဆေးထားသော baseline:** Zap v2.2.7။

## ရည်ရွယ်ချက်

Zap သည် လက်ရှိ `Rc<RefCell>` runtime သည် tracing garbage collector မဟုတ်ကြောင်း မဖုံးကွယ်ဘဲ memory behavior ကို bounded နှင့် observable ဖြစ်စေရန် လိုအပ်ပါသည်။ ဤ contract သည် run-owned accounting boundary ပထမအဆင့်ကို သတ်မှတ်ပါသည်။ Logical budget admission ကို Rust allocator measurement နှင့် ခွဲခြားပြီး object identity၊ raw address နှင့် process-global counter များကို public result ထဲသို့ မထည့်ပါ။

## Ownership

`ExecutionContext` တစ်ခုစီသည် `RuntimeState` မှတစ်ဆင့် `MemoryBudget` တစ်ခုနှင့် `ObjectStore` တစ်ခုကို ပိုင်ဆိုင်ပါသည်။ Context reset ပြုလုပ်သောအခါ store နှစ်ခုလုံးကို ရှင်းလင်းပါသည်။ Independent context များသည် တစ်ခုနှင့်တစ်ခု၏ logical byte၊ task admission၊ output usage သို့မဟုတ် object counter များကို မမြင်နိုင်ပါ။ LSP `LspState` သည် သီးခြား per-session state ဖြစ်ပြီး language execution budget တွင် မပါဝင်ပါ။

| Component | တာဝန် | Public guarantee |
|---|---|---|
| `MemoryBudget` | Logical byte၊ admitted task နှင့် bounded output ကို မှတ်တမ်းတင်သည် | Deterministic admission နှင့် typed limit error၊ allocator-size claim မရှိ |
| `ObjectStore` | Run-owned object allocation နှင့် live/deallocated count ကို မှတ်တမ်းတင်သည် | Counter များသည် per-run ဖြစ်ပြီး သတ်မှတ်ထားသည့်အတိုင်း monotonic ဖြစ်ကာ raw address မပါ |
| `Value` validation | Text၊ collection၊ graph နှင့် node limit များကို စစ်ဆေးသည် | ရှိပြီးသား value-limit error များကို fail-closed ထားသည် |
| Rust allocator | Process ၏ အမှန်တကယ် memory | ဤ contract မှ တိုင်းတာ/ဖော်ပြခြင်း မပြု |

## Logical accounting units

Budget သည် Rust heap-layout measurement မဟုတ်သော deterministic logical byte unit များကို အသုံးပြုပါသည်။ Scalar သို့မဟုတ် wrapper တစ်ခုစီတွင် fixed base charge ရှိပြီး text တွင် UTF-8 payload length၊ list/map တွင် fixed container/slot charge နှင့် map key byte များ၊ object တွင် fixed base၊ class-name byte နှင့် per-field storage၊ callable value တွင် function metadata၊ parameter/default metadata နှင့် reachable live closure-frame binding များကို ထည့်တွက်ပါသည်။ Nested value များကို object၊ frame နှင့် function identity guard များဖြင့် traverse လုပ်သဖြင့် cycle များကို bounded ထားပြီး charge calculation တစ်ကြိမ်အတွင်း shared reference များကို ထပ်မတွက်ပါ။ Counter overflow သို့မဟုတ် configured limit ကျော်မည့် request ကို admission မလုပ်မီ stable error ဖြင့် ငြင်းပယ်ပါသည်။ Accounting သည် available budget ရှိနေသကဲ့သို့ မမှားယွင်းစွာ ပြန်မလည်ပါ။

Runtime တွင် logical byte/object charge reserve၊ logical task admit/complete၊ output reserve နှင့် byte/output checkpoint ကို save/restore ပြုလုပ်ရန် explicit method များကို ပေးပါသည်။ Canonical AST literal၊ container၊ cloned access result၊ builtin result နှင့် registered callable capture များကို materialization boundary တွင် charge လုပ်ပါသည်။ Object construction သည် default၊ explicit field နှင့် initializer များ run ပြီးနောက် finalized field shape အပေါ် charge လုပ်သဖြင့် default/nested value များသည် မိမိ AST charge နှင့် object storage charge အသီးသီး ရရှိပါသည်။ AST expression၊ builtin dispatch သို့မဟုတ် constructor မအောင်မြင်ပါက byte/output checkpoint သို့ rollback ပြုလုပ်ပြီး task admission သည် သီးခြား task lifecycle contract အတိုင်း ဆောင်ရွက်ပါသည်။ ဤ charge များသည် logical accounting unit များသာဖြစ်ပြီး allocator-size measurement မဟုတ်ပါ။

## Default limits

Initial default များသည် ရှိပြီးသား limit များနှင့် ကိုက်ညီသော conservative value များ ဖြစ်ပါသည်။ ဤ slice တွင် explicit runtime-state API မှတစ်ဆင့်သာ configure လုပ်နိုင်ပြီး သီးခြား configuration contract မရှိသေးသရွေ့ environment variable အသစ် သို့မဟုတ် user-facing syntax မထည့်သွင်းပါ။

| Limit | အဓိပ္ပါယ် | Failure boundary |
|---|---|---|
| `max_bytes` | Execution တစ်ခုက reserve လုပ်ထားသော logical byte စုစုပေါင်း | `memory budget exceeded` |
| `max_tasks` | Concurrent/admitted logical task count | `task budget exceeded` |
| `max_output_bytes` | Execution တစ်ခုက admit လုပ်ထားသော output byte | `output budget exceeded` |

## Object lifecycle

Production object construction သည် လက်ရှိ context ပိုင် object store ကို လက်ခံအသုံးပြုပါသည်။ Allocation ပြုလုပ်သောအခါ `object_allocations` နှင့် `live_objects` တိုးပြီး tracked field storage drop ဖြစ်သောအခါ `live_objects` လျော့ကာ `object_deallocations` တိုးပါသည်။ Explicit cleanup သည် attempt၊ success နှင့် borrow failure များကို record လုပ်ပြီး bounded validation သည် validation run ကို record လုပ်သည်။ Reset လုပ်သောအခါ active store အသစ်ကို အသုံးပြုသဖြင့် ယခင် run မှ ထိန်းထားသော object များသည် နောက် run ၏ counter ကို မပြောင်းလဲနိုင်ပါ။ Test-only သို့မဟုတ် compatibility constructor သည် untracked standalone object ဖန်တီးနိုင်သော်လည်း production process-global statistic ကို ပြန်မထည့်ရပါ။ Execution context မှ ခေါ်သော `memory_stats()` သည် လက်ရှိ execution store နှင့် budget ကို ပြသပြီး context-free compatibility call တွင် stable zero counter နှင့် default budget field များကို ပြသပါသည်။

Object counter များသည် diagnostic evidence သာဖြစ်ပြီး reclamation guarantee မဟုတ်ပါ။ လက်ရှိ cycle policy သည် `explicit_clear_object_fields` ဖြစ်ပြီး cycle များကို ရှိပြီးသား checked field API များမှတစ်ဆင့် explicit break လုပ်နိုင်ဆဲ ဖြစ်ပါသည်။ Public weak reference နှင့် automatic tracing collection များသည် deferred ဖြစ်နေဆဲဖြစ်ပြီး unsupported/not implemented အဖြစ် ဆက်လက်ဖော်ပြရမည်။ Lexical-frame snapshot၊ insertion၊ assignment နှင့် import synchronization များကိုလည်း checked operation များဖြင့် ဆောင်ရွက်ပြီး frame borrow ဖြစ်နေချိန်တွင် panic မဖြစ်ဘဲ deterministic `BorrowError` ပြန်ပေးသည်။ Canonical AST equality သည် visited object-pair short-circuit နှင့် တူညီသော `max_value_nodes` bound ပါသည့် checked cycle-safe comparator ကို အသုံးပြုပြီး object-field borrow conflict ကို `==` နှင့် `!=` မှတစ်ဆင့် ပြန်ပေးသည်။

## Errors နှင့် determinism

Budget failure များသည် stable operation-specific text ကို အသုံးပြုပြီး structured diagnostic boundary အတွင်း `ZAP-MEMORY-001` သို့ map လုပ်ရမည်။ Admission sequence တူညီပါက repeated run များတွင် counter နှင့် failure point တူညီရမည်။ Failed reservation သည် fail ဖြစ်သည့် resource ကို မစားသုံးရပါ။ Release သည် underflow မဖြစ်ရပါ။ Reset သည် active counter နှင့် usage အားလုံးကို initial state သို့ ပြန်ထားပြီး old object store ကို detach လုပ်ရမည်။

## Compatibility boundary

ဤ slice တွင် executor-backed language scheduling၊ foreign blocking call ကို forced interrupt ပြုလုပ်ခြင်း၊ weak reference သို့မဟုတ် tracing garbage collection မထည့်သွင်းပါ။ ရှိပြီးသား first-class callable value နှင့် parent-linked `EnvFrame` binding များကို semantics မပြောင်းဘဲ accounting ပြုလုပ်ပါသည်။ ရှိပြီးသား `read_lines`/`write_lines` compatibility behavior နှင့် canonical AST boundary မပြောင်းလဲပါ။

## Acceptance evidence

M2-MEM-02 implementation slice အတွက် independent context များတွင် budget နှင့် object store isolation ရှိရမည်။ Reset သည် old store ကို detach လုပ်ပြီး active counter အားလုံးကို ရှင်းလင်းရမည်။ Nested value၊ callable capture/default metadata၊ finalized object field နှင့် builtin output များတွင် deterministic logical charge ရှိရမည်။ Expression/builtin/constructor reservation မအောင်မြင်ပါက byte/output usage rollback ဖြစ်ရမည်။ Object allocation/deallocation၊ validation နှင့် cleanup diagnostic များသည် deterministic ဖြစ်ရမည်။ Byte/object/task/output over-limit case များသည် fail closed ဖြစ်ရမည်။ Repeated module execution သည် context cache တစ်ခုကို ပြန်သုံးရမည်။ JSON/LSP/CLI error propagation သည် panic-free ဖြစ်ရမည်။ Full native suite နှင့် cross-platform CI သည် အောင်မြင်ရမည်။ Focused evaluator/value regression များသည် accounting path အသစ်များကို cover လုပ်ပြီး repository-wide gate သည် နောက်ဆုံး acceptance check ဖြစ်သည်။

## ကိုးကားရန်

* [Runtime-state contract](RUNTIME_STATE_MM.md)
* [Memory model](MEMORY_MODEL_MM.md)
* [Remaining TODO register](PDF_REMAINING_TODO_MM.md)
