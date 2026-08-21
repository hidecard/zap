# Zap Memory Model

## အကျယ်အဝန်း

Zap value များသည် ownership ကို ရှင်းလင်းစွာ သတ်မှတ်ထားသော reference-counted primitives များကို အသုံးပြုသည်။ Function closure နှင့် object field များသည် reference-counted ဖြစ်ပြီး single-threaded interpreter လမ်းကြောင်းတွင် mutable state ကို `RefCell` ဖြင့် ထိန်းချုပ်ထားသည်။

## Ownership contract

`Value::Object` သည် reference-counted field map တစ်ခုကို ပိုင်ဆိုင်သည်။ Object value ကို clone လုပ်ပါက object field များကို copy မလုပ်ဘဲ handle ကိုသာ clone လုပ်သည်။ ထို့ကြောင့် object aliasing ကို သိသာစွာ သတ်မှတ်နိုင်ပြီး field mutation ကို shared field map တစ်ခုအတွင်း ထိန်းထားနိုင်သည်။

Cyclic object graph များကို reference counting တစ်ခုတည်းဖြင့် အလိုအလျောက် မရှင်းနိုင်ပါ။ Embedder နှင့် runtime cleanup path များသည် နောက်ဆုံး external object handle ကို မလွှတ်မီ cyclic field များကို ရှင်းလင်းရမည်။ ဤ boundary အတွက် runtime တွင် `clear_object_fields()` နှင့် diagnostic/regression test များအတွက် `object_field_count()` ကို ပေးထားသည်။

လက်ရှိ interpreter သည် mutable object access အတွက် single-threaded ဖြစ်သည်။ Object field read/write များကို checked `try_borrow`/`try_borrow_mut` accessor များဖြင့် ပြုလုပ်သည်။ Read/write conflict ဖြစ်ပါက runtime သည် panic မဖြစ်စေဘဲ stable code `ZAP-BORROW-001` ပါသော typed `BorrowError` ကို ပြန်ပေးသည်။ ထို့ကြောင့် ဤ boundary တွင် `clear_object_fields()` နှင့် `object_field_count()` တို့သည် fallible result ပြန်ပေးသည်။ ဤ API သည် thread-safe ownership သို့မဟုတ် tracing garbage collector ရှိသည်ဟု မဆိုလိုပါ။ အနာဂတ် multi-threaded runtime သည် ဤ handle များကို thread များကြား တိုက်ရိုက်မျှဝေမည့်အစား synchronization သို့မဟုတ် tracing collector design အသစ်တစ်ခု လိုအပ်မည်။

Runtime တွင် argument မလိုသော `memory_stats()` builtin ကို bounded diagnostic record အဖြစ် ထည့်သွင်းထားသည်။ Stable map fields များမှာ `live_objects`၊ `object_allocations`၊ `object_deallocations`၊ `max_text_bytes`၊ `max_collection_items` နှင့် `max_value_nodes` တို့ ဖြစ်သည်။ Public weak reference များကို `unsupported_public_api`၊ tracing collection ကို `not_implemented` ဟု explicit ပြထားသည်။ ဤတန်ဖိုးများသည် capability information ဖြစ်ပြီး cycle များကို အလိုအလျောက် collect လုပ်မည်ဟု ကတိပြုခြင်း မဟုတ်ပါ။

Public builtin boundary များတွင် runtime value များကို cyclic object graph ထဲတွင် recursion မဆုံးနိုင်အောင် bounded validation ဖြင့် စစ်ဆေးသည်။ Text value တစ်ခုလျှင် 8 MiB၊ list သို့မဟုတ် map တစ်ခုလျှင် entry 100,000 နှင့် traverse လုပ်သော value graph တစ်ခုလျှင် node 100,000 အထိ ကန့်သတ်ထားသည်။ Limit ကျော်ပါက deterministic memory-limit error ပြန်ပေးသည်။ ဤစစ်ဆေးမှုသည် bounded validation ဖြစ်ပြီး internal allocation အားလုံးကို global accounting လုပ်ထားသည်ဟု မဆိုလိုပါ။

## Regression guarantee

Native test `cyclic_object_graph_can_be_explicitly_broken` သည် self-referential object တစ်ခု ဖန်တီးပြီး cycle ရှိကြောင်း စစ်ဆေးသည်။ ထို့နောက် field များကို ရှင်းလင်းကာ field allocation ပြန်လွှတ်နိုင်ကြောင်း စစ်ဆေးသည်။ `conflicting_object_borrows_return_typed_failures` regression သည် mutable field borrow တစ်ခုကို active ထားပြီး ပြိုင်တူ read/write များအတွက် panic မဖြစ်ဘဲ deterministic `BorrowError` ပြန်လာကြောင်း စစ်ဆေးသည်။ ဤ test များသည် memory contract ကို အတည်ပြုခြင်းဖြစ်ပြီး arbitrary cycle များကို အလိုအလျောက် garbage collect လုပ်သည်ဟု ဆိုလိုခြင်းမဟုတ်ပါ။

## နောက်ပိုင်းလုပ်ရန်

Weak-reference support နှင့် tracing collector သည် နောက်ပိုင်း milestone များ ဖြစ်နေသေးသည်။ Closure-level allocation accounting၊ process-wide heap telemetry နှင့် arbitrary cycle များကို အလိုအလျောက် reclaim လုပ်ခြင်းတို့ကို လက်ရှိ contract တွင် ရည်ရွယ်ချက်ရှိရှိ မထည့်သေးပါ။ ထိုအရာများကို လက်ရှိ single-threaded `Rc<RefCell>` boundary နှင့် သီးခြား design လုပ်ရမည်။
