# Zap Programming Language — Initial Design

**Version:** 0.6.0 design update  
**Author:** Manus AI

## ရည်ရွယ်ချက်

Zap သည် ဖတ်ရလွယ်ကူပြီး Web application များတွင် အသုံးပြုရလွယ်ကူသော general-purpose programming language ဖြစ်မည်။ ပထမဆုံး version တွင် learning curve ကို လျှော့ချရန် indentation-based syntax၊ ရိုးရှင်းသော data types နှင့် တိကျသော error messages များကို ဦးစားပေးမည်။

Web နှင့် AI တို့ကို language core ထဲသို့ အလွန်အမင်းထည့်သွင်းမည့်အစား standard library နှင့် package system အဖြစ် ထားရှိမည်။ ထိုနည်းလမ်းသည် language ကို သေးငယ်ပြီး သင်ယူရလွယ်ကူစေသလို၊ နောက်ပိုင်းတွင် web server၊ HTTP client၊ JSON၊ data science နှင့် model API များကို လွယ်ကူစွာ ထပ်တိုးနိုင်စေသည်။

## အခြေခံမူများ

| မူ | ဆုံးဖြတ်ချက် |
|---|---|
| Syntax | Indentation-based blocks နှင့် ရှင်းလင်းသော English keywords |
| Runtime | ပထမ prototype တွင် tree-walk interpreter |
| Typing | Dynamic typing; v0.6.0 မှစ၍ variable-level optional type annotations ကို စစ်ဆေးနိုင်မည် |
| Strings | Double quote နှင့် single quote နှစ်မျိုးစလုံး |
| Collections | list, map, set ကို built-in အဖြစ် ထည့်မည် |
| Errors | Runtime error message များကို ပြသမည်။ Structured source diagnostics သည် v0.6.x အတွင်း တိုးချဲ့မည် |
| Web | `web` standard module မှ server/request API ပေးမည် |
| AI | `ai` module မှ provider-neutral text generation API ပေးမည် |
| Interoperability | ပထမအဆင့်တွင် native modules နှင့် package APIs ဖြင့် extension ပြုလုပ်နိုင်မည် |

## နမူနာ program

```zap
say "Hello from Zap"

name = "World"
if name != "":
    say "Hello, " + name

for item in ["web", "ai"]:
    say item
```

## Web နမူနာ

```zap
use web

web.route "/hello" with request:
    return web.text "Hello Zap"

web.listen 3000
```

## AI နမူနာ

```zap
use ai

answer = ai.ask "Explain HTTP in one sentence"
say answer.text
```

Native v0.5.0 တွင် `web` နှင့် `ai` သည် module architecture အတွက် placeholder အဆင့်ဖြစ်သည်။ v0.6.0 တွင် `path`၊ `time`၊ `env` နှင့် basic `math` utilities ကို native built-ins အဖြစ် ထည့်သွင်းပြီး production-level networking၊ async runtime နှင့် model provider adapters များကို သီးခြား release အဖြစ် တဖြည်းဖြည်း ထည့်မည်။

## v0.6.0 OOP Foundation

Zap v0.6.0 တွင် beginner-friendly class-based OOP foundation ကို native runtime အတွင်း ထည့်သွင်းထားသည်။ Class declaration၊ object creation၊ `init` constructor၊ `self` method receiver၊ property read/write၊ method arguments၊ single inheritance နှင့် method override တို့ကို support လုပ်သည်။ Object များကို list၊ map၊ JSON နှင့် type inspection တို့နှင့် ပေါင်းစပ်အသုံးပြုနိုင်သည်။

```zap
class Device:
    fn init(self, name):
        self.name = name

    fn label(self):
        return "device: " + self.name

let device = new("Device", "sensor")
say device.label()
```

OOP implementation ၏ stable boundary တွင် interfaces၊ abstract classes၊ access modifiers၊ generics နှင့် multiple inheritance များ မပါဝင်သေးပါ။

## မထည့်သွင်းသေးသောအရာများ

v0.6.0 တွင် macro system၊ package registry၊ full static type checker၊ threads နှင့် shared mutable concurrency များကို မထည့်သွင်းသေးပါ။ Async task၊ cancellation နှင့် channel model ကို design အဆင့်မှ စတင်ပြီး runtime semantics တည်ငြိမ်မှသာ production feature အဖြစ် ထည့်သွင်းမည်။

## အောင်မြင်မှုစံနှုန်း

ပထမဆုံး MVP သည် variable assignment၊ literals၊ arithmetic၊ comparison၊ function call၊ `if`၊ `for`၊ `say` နှင့် module import ကို လုပ်ဆောင်နိုင်ရမည်။ သင်ခန်းစာတစ်ခုတည်းဖြင့် beginner သည် အခြေခံ program တစ်ခုရေးနိုင်ပြီး၊ Web နှင့် AI API ကို နောက်ပိုင်း standard modules မှတစ်ဆင့် သုံးနိုင်ရန် architecture က အဆင်သင့်ဖြစ်ရမည်။

## v0.6.0 implementation direction

v0.6.0 သည် native runtime၊ standard library နှင့် CLI workflow ကို ဦးစားပေးသည်။ `now()`၊ `sleep()`၊ `env()`၊ `has_env()`၊ `exists()`၊ path helpers နှင့် numeric helpers များသည် လက်တွေ့အသုံးပြုနိုင်သော အခြေခံ API များဖြစ်သည်။ Optional type annotation သည် beginner syntax ကို မပြောင်းလဲဘဲ `zap check` နှင့် runtime diagnostics များအတွက် အခြေခံပေးသည်။

Concurrency အတွက် shared memory threads များကို အလျင်စလို မထည့်ဘဲ `async`/`await`၊ tasks၊ cancellation နှင့် channels ကို စနစ်တကျ သတ်မှတ်မည်။ ထို design သည် Web request၊ AI API နှင့် IoT event loop များအတွက် ရိုးရှင်းပြီး ချဲ့ထွင်နိုင်သော model ဖြစ်စေရန် ရည်ရွယ်သည်။ အသေးစိတ် roadmap ကို [`ROADMAP_0.6.0.md`](ROADMAP_0.6.0.md) တွင် ဖတ်ရှုနိုင်သည်။

## နောက်တစ်ဆင့်

နောက်ထပ် implementation သည် tokenizer၊ parser၊ AST နှင့် evaluator အလွှာများကို native runtime အတွင်း ပိုမိုခိုင်မာအောင် တည်ဆောက်မည်။ ထို architecture သည် language semantics ကို တိကျစွာ ထိန်းသိမ်းပေးပြီး၊ နောက်ပိုင်းတွင် Zap source ကို bytecode သို့မဟုတ် optimized execution format သို့ ပြောင်းလဲနိုင်ရန် အခြေခံဖြစ်မည်။

## References

ဤ draft သည် ပြင်ပအချက်အလက်ကို ကိုးကားထားခြင်းမဟုတ်ဘဲ user ၏ ရည်ရွယ်ချက်အပေါ် အခြေခံထားသော မူကြမ်းဒီဇိုင်းဖြစ်သည်။
