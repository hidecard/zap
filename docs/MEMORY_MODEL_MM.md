# Zap Memory Model

**Verified baseline:** Zap v2.1.14
**ရည်ရွယ်ချက်:** Ownership၊ borrowing၊ closure capture၊ object field၊ cleanup boundary နှင့် non-goal များအတွက် runtime-maintainer reference ဖြစ်သည်။
**လမ်းညွှန်:** [Documentation hub](DOCUMENTATION_NAVIGATION_MM.md) · [လေ့လာရေး guide](LEARN_ZAP_MM.md) · [Language specification](LANGUAGE_SPEC_MM.md) · [Runtime state](RUNTIME_STATE_MM.md) · [Memory budget/ObjectStore](MEMORY_BUDGET_OBJECT_STORE_MM.md) · [Async/LSP guide](ASYNC_LSP_MM.md)

## အကျယ်အဝန်း

Zap value များသည် ownership ကို ရှင်းလင်းစွာ သတ်မှတ်ထားသော reference-counted primitives များကို အသုံးပြုသည်။ Function closure နှင့် object field များသည် reference-counted ဖြစ်ပြီး single-threaded interpreter လမ်းကြောင်းတွင် mutable state ကို `RefCell` ဖြင့် ထိန်းချုပ်ထားသည်။

## Ownership contract

`Value::Object` သည် reference-counted field map တစ်ခုကို ပိုင်ဆိုင်သည်။ Object value ကို clone လုပ်ပါက object field များကို copy မလုပ်ဘဲ handle ကိုသာ clone လုပ်သည်။ ထို့ကြောင့် object aliasing ကို သိသာစွာ သတ်မှတ်နိုင်ပြီး field mutation ကို shared field map တစ်ခုအတွင်း ထိန်းထားနိုင်သည်။

Cyclic object graph များကို reference counting တစ်ခုတည်းဖြင့် အလိုအလျောက် မရှင်းနိုင်ပါ။ Embedder နှင့် runtime cleanup path များသည် နောက်ဆုံး external object handle ကို မလွှတ်မီ cyclic field များကို ရှင်းလင်းရမည်။ ဤ boundary အတွက် runtime တွင် `clear_object_fields()` နှင့် diagnostic/regression test များအတွက် `object_field_count()` ကို ပေးထားသည်။

Function closure များသည် parent-linked `EnvFrame` node များကို အသုံးပြုသည်။ Frame တစ်ခုသည် မိမိ၏ local binding များကို ပိုင်ဆိုင်ပြီး lexical parent သို့ ချိတ်ထားသည်။ Lookup သည် လက်ရှိ frame မှ parent ဘက်သို့ ရှာဖွေပြီး assignment သည် ရှိပြီးသား binding ၏ အနီးဆုံး frame ကို ပြင်ကာ parent binding မရှိပါက local binding အသစ် ဖန်တီးသည်။ Callable value များသည် frame chain ကို reference-counted handle ဖြင့် ထိန်းထားသောကြောင့် outer function ပြန်ထွက်ပြီးနောက် returned closure က captured state ကို ဆက်လက်အသက်ရှင်စေသည်။ Recursive call များသည် function table မှတစ်ဆင့် ဆက်လက် resolve လုပ်ပြီး closure mutation ကို raw address မဖော်ပြဘဲ captured frame သို့ synchronize လုပ်သည်။

လက်ရှိ interpreter သည် mutable object access အတွက် single-threaded ဖြစ်သည်။ Object field read/write များကို checked `try_borrow`/`try_borrow_mut` accessor များဖြင့် ပြုလုပ်သည်။ Read/write conflict ဖြစ်ပါက runtime သည် panic မဖြစ်စေဘဲ stable code `ZAP-BORROW-001` ပါသော typed `BorrowError` ကို ပြန်ပေးသည်။ ထို့ကြောင့် ဤ boundary တွင် `clear_object_fields()` နှင့် `object_field_count()` တို့သည် fallible result ပြန်ပေးသည်။ ဤ API သည် thread-safe ownership သို့မဟုတ် tracing garbage collector ရှိသည်ဟု မဆိုလိုပါ။ အနာဂတ် multi-threaded runtime သည် ဤ handle များကို thread များကြား တိုက်ရိုက်မျှဝေမည့်အစား synchronization သို့မဟုတ် tracing collector design အသစ်တစ်ခု လိုအပ်မည်။

Runtime တွင် argument မလိုသော `memory_stats()` builtin ကို bounded diagnostic record အဖြစ် ထည့်သွင်းထားသည်။ Stable map fields များမှာ `live_objects`၊ `object_allocations`၊ `object_deallocations`၊ `max_text_bytes`၊ `max_collection_items` နှင့် `max_value_nodes` တို့ ဖြစ်သည်။ Context-backed call များတွင် `cleanup_attempts`၊ `cleanup_successes`၊ `cleanup_failures`၊ `validation_runs`၊ `max_bytes`၊ `used_bytes`၊ `max_tasks`၊ `admitted_tasks`၊ `max_output_bytes` နှင့် `used_output_bytes` တို့ကိုလည်း ထပ်မံဖော်ပြသည်။ Public weak reference များကို `unsupported_public_api`၊ tracing collection ကို `not_implemented` ဟု explicit ပြထားသည်။ ဤတန်ဖိုးများသည် capability information ဖြစ်ပြီး cycle များကို အလိုအလျောက် collect လုပ်မည်ဟု ကတိပြုခြင်း မဟုတ်ပါ။ Logical object admission များသည် fixed base charge နှင့် fixed per-field charge ကို အသုံးပြုပြီး Rust heap-layout measurement မဟုတ်ပါ။

Public builtin boundary များတွင် runtime value များကို cyclic object graph ထဲတွင် recursion မဆုံးနိုင်အောင် bounded validation ဖြင့် စစ်ဆေးသည်။ Text value တစ်ခုလျှင် 8 MiB၊ list သို့မဟုတ် map တစ်ခုလျှင် entry 100,000 နှင့် traverse လုပ်သော value graph တစ်ခုလျှင် node 100,000 အထိ ကန့်သတ်ထားသည်။ Limit ကျော်ပါက deterministic memory-limit error ပြန်ပေးသည်။ ဤစစ်ဆေးမှုသည် bounded validation ဖြစ်ပြီး internal allocation အားလုံးကို global accounting လုပ်ထားသည်ဟု မဆိုလိုပါ။

## Regression guarantee

Native test `parent_linked_closures_preserve_mutation_after_outer_return` သည် returned callable က captured binding ကို call များအကြား ဆက်လက်ထိန်းသိမ်းပြီး ပြောင်းလဲနိုင်ကြောင်း စစ်ဆေးသည်။ Native test `cyclic_object_graph_can_be_explicitly_broken` သည် self-referential object တစ်ခု ဖန်တီးပြီး cycle ရှိကြောင်း စစ်ဆေးသည်။ ထို့နောက် field များကို ရှင်းလင်းကာ field allocation ပြန်လွှတ်နိုင်ကြောင်း စစ်ဆေးသည်။ `conflicting_object_borrows_return_typed_failures` regression သည် mutable field borrow တစ်ခုကို active ထားပြီး ပြိုင်တူ read/write များအတွက် panic မဖြစ်ဘဲ deterministic `BorrowError` ပြန်လာကြောင်း စစ်ဆေးသည်။ M2-MEM-02 regression များသည် context တစ်ခုချင်းစီ၏ validation/cleanup counter၊ cleanup failure accounting၊ output/task admission နှင့် ယခင် run က ထိန်းထားသည့် object များမှ reset ပြီးနောက် state မရောနှောမှုကို စစ်ဆေးသည်။ ဤ test များသည် memory contract ကို အတည်ပြုခြင်းဖြစ်ပြီး arbitrary cycle များကို အလိုအလျောက် garbage collect လုပ်သည်ဟု ဆိုလိုခြင်းမဟုတ်ပါ။

## နောက်ပိုင်းလုပ်ရန်

Weak-reference support နှင့် tracing collector သည် နောက်ပိုင်း milestone များ ဖြစ်နေသေးသည်။ Process-wide heap telemetry နှင့် arbitrary cycle များကို အလိုအလျောက် reclaim လုပ်ခြင်းတို့ကို လက်ရှိ contract တွင် ရည်ရွယ်ချက်ရှိရှိ မထည့်သေးပါ။ EnvFrame lifetime သည် reference-counted ဖြစ်ပြီး captured value က callable ကို ထိန်းထားသောအခါ cycle ဖြစ်နိုင်သည်။ Cycle breaking သည် explicit cleanup တာဝန်အဖြစ် ဆက်ရှိနေပြီး လက်ရှိ single-threaded `Rc<RefCell>` boundary နှင့် သီးခြား design လုပ်ရမည်။
