# Zap v0.7.1 — OOP Audit Patch

Zap v0.7.1 သည် v0.7.0 OOP foundation ကို audit ပြုလုပ်ပြီး class validation၊ inheritance constructor behavior နှင့် object semantics များကို ပိုမိုတည်ငြိမ်စေသော patch release ဖြစ်သည်။

## ပြင်ဆင်ချက်များ

### Class Validation

`new("Missing")` ကဲ့သို့ မရှိသော class ကို ဖန်တီးရန် ကြိုးစားပါက runtime သည် `unknown class: Missing` error ပြန်ပေးသည်။ `class Child extends Missing:` ကဲ့သို့ မရှိသော parent class ကို သုံးပါက class declaration အဆင့်တွင် `unknown parent class: Missing` error ပြန်ပေးသည်။

### Constructor Behavior

Child class တွင် ကိုယ်ပိုင် `init` ရှိပါက parent class ၏ `init` ကို အရင် run ပြီး child constructor ကို ဆက် run သည်။ ထို့ကြောင့် inherited fields နှင့် child fields နှစ်မျိုးစလုံးကို object state ထဲတွင် မှန်ကန်စွာ ရရှိနိုင်သည်။

```zap
class Base:
    fn init(self):
        self.ready = true

class Child extends Base:
    fn init(self):
        self.child = true

let item = new("Child")
say item.ready
say item.child
```

Output သည်—

```text
true
true
```

### Method Override နှင့် Empty Class

Child class တွင် parent method အမည်တူ method ရှိပါက child implementation ကို ခေါ်သည်။ Method မရှိသော empty class ကိုလည်း object အဖြစ် ဖန်တီးနိုင်သည်။

## Verification

v0.7.1 patch တွင် OOP regression test များအပါအဝင် native integration tests **24 ခု** run ပြီး pass ဖြစ်ထားသည်။ စစ်ဆေးထားသောအချက်များမှာ class၊ object၊ method၊ constructor၊ mutable property၊ inheritance၊ override၊ empty class၊ unknown class နှင့် unknown parent error များ ဖြစ်သည်။

## မပါဝင်သေးသောအရာများ

Explicit `super.method()` calls၊ interfaces၊ abstract classes၊ private/protected modifiers၊ generics နှင့် multiple inheritance များသည် v0.7.1 တွင် မပါဝင်သေးပါ။ ၎င်းတို့ကို language semantics နှင့် diagnostics သေချာသတ်မှတ်ပြီးမှ နောက် release များတွင် ထည့်သွင်းမည်။

## Installation

GitHub Releases မှ platform နှင့်ကိုက်ညီသော archive ကို download ပြီး SHA-256 checksum ဖြင့် စစ်ဆေးပါ။ Archive အတွင်းပါ binary ကို system PATH ထဲသို့ ထည့်ပြီး—

```bash
zap --version
zap run main.zp
```

ဖြင့် အသုံးပြုနိုင်သည်။

အသေးစိတ် syntax အတွက် [`SYNTAX_GUIDE.md`](SYNTAX_GUIDE.md) နှင့် Burmese သင်ခန်းစာအတွက် [`LEARN_ZAP_MM.md`](LEARN_ZAP_MM.md) ကို ဖတ်ရှုပါ။
