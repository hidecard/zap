# P0-04 Memory Contract Status

## လက်ရှိပြီးစီးမှု

P0-04 Memory နှင့် reference-cycle contract ၏ ပထမ implementation slice ကို အကောင်အထည်ဖော်ပြီးဖြစ်သည်။ `Value::Object` field storage ကို tracked ownership wrapper ဖြင့် ပြောင်းလဲထားပြီး live object count၊ object allocation count နှင့် object deallocation count များကို thread-local runtime boundary အတွင်း တိုင်းတာနိုင်သည်။

`memory_stats()` builtin သည် `live_objects`၊ `object_allocations`၊ `object_deallocations`၊ `max_text_bytes`၊ `max_collection_items` နှင့် `max_value_nodes` fields များကို ပြန်ပေးသည်။ Public weak references ကို `unsupported_public_api`၊ tracing collection ကို `not_implemented` ဟု explicit capability status အဖြစ် ပြထားသည်။

Public builtin boundary များတွင် text တစ်ခုလျှင် 8 MiB၊ list/map တစ်ခုလျှင် entry 100,000 နှင့် traversed value graph တစ်ခုလျှင် node 100,000 limit များကို cycle-safe validation ဖြင့် စစ်ဆေးသည်။ Cycle များအတွက် visited object identity set ကို အသုံးပြုသဖြင့် self-referential graph များတွင် recursion မဆုံးနိုင်ခြင်း မဖြစ်စေရပါ။ Object field read/write များကို checked `try_borrow`/`try_borrow_mut` ဖြင့် ပြုလုပ်ပြီး conflict ဖြစ်ပါက panic မဖြစ်ဘဲ stable `ZAP-BORROW-001` ပါသော `BorrowError` ပြန်ပေးသည်။

## Regression evidence

`cyclic_object_graph_can_be_explicitly_broken` သည် self-reference ကို ဖန်တီး၊ field များကို clear လုပ်၊ tracked object live count/deallocation count ကို စစ်ဆေးပြီး field storage ပြန်လွှတ်နိုင်ကြောင်း အတည်ပြုသည်။ `conflicting_object_borrows_return_typed_failures` နှင့် `json_conversion_propagates_borrow_error_without_panic` တို့သည် conflict borrow များတွင် panic မဖြစ်ဘဲ typed failure ပြန်ပေးကြောင်း စစ်ဆေးသည်။ `memory_stats` unit၊ AST integration၊ oversized text/list rejection နှင့် cycle-safe validation tests များလည်း ပါဝင်သည်။

## ရည်ရွယ်ချက်ရှိရှိ deferred ထားသည့်အရာများ

Public weak-reference API၊ closure-level allocation accounting၊ process-wide heap telemetry၊ arbitrary cycle များကို အလိုအလျောက် reclaim လုပ်ခြင်းနှင့် tracing garbage collector တို့ကို လက်ရှိ single-threaded `Rc<RefCell>` contract အတွင်း မထည့်သေးပါ။ ဤအရာများသည် နောက်ထပ် design milestone များ ဖြစ်သည်။

## Validation

Pinned Rust 1.75 toolchain ဖြင့် rustfmt check၊ strict Clippy `-D warnings`၊ full native test suite နှင့် `git diff --check` အားလုံး pass ဖြစ်သည်။
