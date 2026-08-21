# Zap Memory Model

## အကျယ်အဝန်း

Zap value များသည် ownership ကို ရှင်းလင်းစွာ သတ်မှတ်ထားသော reference-counted primitives များကို အသုံးပြုသည်။ Function closure နှင့် object field များသည် reference-counted ဖြစ်ပြီး single-threaded interpreter လမ်းကြောင်းတွင် mutable state ကို `RefCell` ဖြင့် ထိန်းချုပ်ထားသည်။

## Ownership contract

`Value::Object` သည် reference-counted field map တစ်ခုကို ပိုင်ဆိုင်သည်။ Object value ကို clone လုပ်ပါက object field များကို copy မလုပ်ဘဲ handle ကိုသာ clone လုပ်သည်။ ထို့ကြောင့် object aliasing ကို သိသာစွာ သတ်မှတ်နိုင်ပြီး field mutation ကို shared field map တစ်ခုအတွင်း ထိန်းထားနိုင်သည်။

Cyclic object graph များကို reference counting တစ်ခုတည်းဖြင့် အလိုအလျောက် မရှင်းနိုင်ပါ။ Embedder နှင့် runtime cleanup path များသည် နောက်ဆုံး external object handle ကို မလွှတ်မီ cyclic field များကို ရှင်းလင်းရမည်။ ဤ boundary အတွက် runtime တွင် `clear_object_fields()` နှင့် diagnostic/regression test များအတွက် `object_field_count()` ကို ပေးထားသည်။

လက်ရှိ interpreter သည် mutable object access အတွက် single-threaded ဖြစ်သည်။ ဤ API သည် thread-safe ownership သို့မဟုတ် tracing garbage collector ရှိသည်ဟု မဆိုလိုပါ။ အနာဂတ် multi-threaded runtime သည် ဤ handle များကို thread များကြား တိုက်ရိုက်မျှဝေမည့်အစား synchronization သို့မဟုတ် tracing collector design အသစ်တစ်ခု လိုအပ်မည်။

## Regression guarantee

Native test `cyclic_object_graph_can_be_explicitly_broken` သည် self-referential object တစ်ခု ဖန်တီးပြီး cycle ရှိကြောင်း စစ်ဆေးသည်။ ထို့နောက် field များကို ရှင်းလင်းကာ field allocation ပြန်လွှတ်နိုင်ကြောင်း စစ်ဆေးသည်။ ဤ test သည် memory contract ကို အတည်ပြုခြင်းဖြစ်ပြီး arbitrary cycle များကို အလိုအလျောက် garbage collect လုပ်သည်ဟု ဆိုလိုခြင်းမဟုတ်ပါ။

## နောက်ပိုင်းလုပ်ရန်

Heap statistics၊ allocation counters၊ cycle diagnostics၊ weak references နှင့် tracing collector တို့သည် နောက် milestone များတွင် သီးခြား design ပြုလုပ်ရမည့်အချက်များ ဖြစ်သည်။ ၎င်းတို့ကို လက်ရှိ `Rc<RefCell>` contract နှင့် ရောနှောမထားသင့်ပါ။
