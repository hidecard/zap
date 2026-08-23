# Zap Generic Type Syntax ဆုံးဖြတ်ချက်မှတ်တမ်း

**ဆုံးဖြတ်ချက်:** TC-012 generic type syntax သည် v2.2.7 type-checking contract အတွက် implemented baseline အဖြစ် သတ်မှတ်နိုင်ပါသည်။ ဤ design gate အတွက် parser သို့မဟုတ် runtime code အသစ် ထပ်မံပြင်ဆင်ရန် မလိုအပ်ပါ။

## အကျုံးဝင်သည့်အပိုင်း

Zap သည် angle brackets ဖြင့် nested generic annotation များကို လက်ခံပါသည်။ လက်ရှိထောက်ပံ့ထားသော form များမှာ `list<T>`၊ `map<K, V>`၊ `option<T>` နှင့် `result<T>` ဖြစ်ပြီး type argument တစ်ခုချင်းစီသည် ခွင့်ပြုထားသော primitive၊ wrapper၊ collection သို့မဟုတ် `any` annotation ဖြစ်ရမည်။

လက်ရှိ parser သည် nested type argument များကို deterministic အတိုင်း ခွဲခြမ်းပြီး empty argument၊ မညီသော delimiter၊ မထောက်ပံ့သော generic base နှင့် argument အရေအတွက်မမှန်ခြင်းတို့ကို reject လုပ်ပါသည်။ `map<K, V>` သည် argument နှစ်ခုတိတိသာ ခွင့်ပြုပြီး annotation matching contract အရ key type ကို `text` သို့မဟုတ် `any` သို့ ကန့်သတ်ထားပါသည်။

| Form | အခြေအနေ | လက်ရှိ contract |
|---|---|---|
| `list<T>` | Implemented baseline | Recursively valid type argument တစ်ခုတိတိ ရှိရမည်။ |
| `map<K, V>` | Implemented baseline | Recursively valid argument နှစ်ခုတိတိ ရှိရမည်။ Concrete value နှင့် match လုပ်ရာတွင် key type သည် `text` သို့မဟုတ် `any` ဖြစ်ရမည်။ |
| `option<T>` | Implemented baseline | Recursively valid payload type တစ်ခုတိတိ ရှိရမည်။ `option<any>` သည် concrete option payload နှင့် compatible ဖြစ်နိုင်သည်။ |
| `result<T>` | Implemented baseline | Recursively valid payload type တစ်ခုတိတိ ရှိရမည်။ |
| User-defined generic declarations | Deferred | v2.2.7 တွင် generic class၊ function သို့မဟုတ် type-parameter declaration syntax မထည့်သွင်းပါ။ |
| Unannotated expression မှ generic inference | Deferred | လုံလောက်သော evidence မရှိပါက checker သည် generic type ကို မဖန်တီးဘဲ conservative ဖြစ်ရမည်။ |

## Syntax နှင့် validation စည်းမျဉ်းများ

v2.2.7 အတွက် grammar ဆုံးဖြတ်ချက်သည် ရိုးရှင်းစွာ အောက်ပါအတိုင်း ဖြစ်ပါသည်။

```text
Type        := Primitive | "list<" Type ">"
             | "map<" Type "," Type ">"
             | "option<" Type ">"
             | "result<" Type ">"
```

Nested argument များအနီးရှိ whitespace ကို trim ပြုလုပ်ပြီး လက်ခံပါသည်။ Generic form တစ်ခုချင်းစီတွင် ကိုက်ညီသော `>` ဖြင့် ပိတ်ရမည်။ မမှန်ကန်သော သို့မဟုတ် မထောက်ပံ့သော form များကို `any` အဖြစ် တိတ်တဆိတ် မချဲ့ထွင်ဘဲ unknown type annotation အဖြစ် checker က report လုပ်ရမည်။

> Generic annotation သည် type contract ဖြစ်ပြီး မပြည့်စုံသောအချက်အလက်ကို ခန့်မှန်းရန် တောင်းဆိုခြင်းမဟုတ်ပါ။ Checker က compatibility ကို မသေချာစွာ မသတ်မှတ်နိုင်ပါက program ကို reject လုပ်ရမည် သို့မဟုတ် အနီးကပ် expression rules တွင် သတ်မှတ်ထားပြီးသား conservative `any` boundary ကိုသာ အသုံးပြုရမည်။

## Compatibility နှင့် rollout

ဤဆုံးဖြတ်ချက်သည် branch narrowing နှင့် alias invalidation တွင် အသုံးပြုနေသော `option<T>` နှင့် `result<T>` semantics များကို မပြောင်းလဲစေပါ။ Native test suite တွင် စစ်ဆေးပြီးသား collection form များကိုလည်း formalize လုပ်ပေးပါသည်။ ထို့ကြောင့် v2.2.7 release gate တွင် duplicate experimental parser path အသစ် ထပ်ထည့်မည့်အစား TC-012 ကို implemented baseline အဖြစ် မှတ်တမ်းတင်ထားပါသည်။

နောင်တွင် generic function parameter၊ user-defined generic declaration၊ variance rule နှင့် ပိုမိုအားကောင်းသော collection-element inference များကို ထည့်သွင်းနိုင်ပါသည်။ ထိုအင်္ဂါရပ်များသည် declaration parsing၊ symbol binding၊ call-site inference၊ diagnostic နှင့် LSP synchronization များကို သက်ရောက်စေသောကြောင့် သီးခြား design record လိုအပ်ပါသည်။

## Conformance အထောက်အထား

Native suite သည် valid nested collection နှင့် variant annotation များ၊ incompatible generic assignment များ၊ `list<>` ကဲ့သို့ malformed form များနှင့် nested generic matching ကို စစ်ဆေးထားပါသည်။ ထို test များသည် v2.2.7 အတွက် TC-012 non-regression boundary ဖြစ်ပါသည်။ Generic declaration syntax နှင့် advanced inference များကို explicit deferred scope အဖြစ် ထားရှိရမည်ဖြစ်ပြီး ဤ baseline မှ အလိုအလျောက် မခန့်မှန်းရပါ။

## Acceptance ဆုံးဖြတ်ချက်

TC-012 ကို v2.2.7 အတွက် **implemented baseline** အဖြစ် သတ်မှတ်ပါသည်။ နောက်လာမည့် generic milestone သည် generic declaration နှင့် inference အတွက် သီးခြား design နှင့် implementation phase ဖြစ်ပြီး လက်ရှိ release gate ၏ အစိတ်အပိုင်း မဟုတ်ပါ။

**Author:** Manus AI
**Version:** v2.2.7 design gate
**Status:** Accepted
