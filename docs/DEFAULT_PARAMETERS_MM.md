# Zap Default Function Parameters လမ်းညွှန်

**Default function parameter** ဆိုသည်မှာ function ကိုခေါ်ရာတွင် argument တစ်ခုကို မပေးထားပါက function က အလိုအလျောက်အသုံးပြုမည့် တန်ဖိုးကို ကြိုတင်သတ်မှတ်ထားခြင်း ဖြစ်သည်။ အများဆုံးအသုံးပြုမည့် အခြေအနေများအတွက် code ကိုတိုတောင်းစေပြီး optional configuration များရေးသားရာတွင် အထောက်အကူပြုသည်။

## အခြေခံ syntax

Parameter name နောက်တွင် `=` နှင့် default value ကိုရေးသည်။

```zap
fn greet(name = "World"):
    return "Hello, " + name

say greet()
say greet("Zap")
```

ရလဒ်မှာ—

```text
Hello, World
Hello, Zap
```

`greet()` ကို argument မပါဘဲခေါ်သောအခါ `name` သည် `"World"` ဖြစ်သည်။ `greet("Zap")` ကိုခေါ်သောအခါ caller က ပေးသော `"Zap"` ကို default value အစား အသုံးပြုသည်။

## Type annotation နှင့် default value

Parameter တစ်ခုတွင် type annotation နှင့် default value နှစ်ခုလုံး ထည့်နိုင်သည်။ Type ကို အရင်ရေးပြီး `=` နောက်တွင် default value ကိုရေးရသည်။

```zap
fn repeat_message(message: text = "Zap", times: number = 1):
    let index = 0
    while index < times:
        say message
        index = index + 1

repeat_message()
repeat_message("Learning", 2)
```

Default expression ၏ type သည် parameter annotation နှင့် ကိုက်ညီရမည်။ အောက်ပါ code တွင် `number` လိုအပ်သော်လည်း text value ပေးထားသောကြောင့် မမှန်ကန်ပါ။

```zap
fn square(value: number = "one") -> number:
    return value * value
```

မ run မီ type error များကိုရှာဖွေရန် `zap check` ကိုအသုံးပြုပါ။

## Positional နှင့် named binding

Zap သည် positional argument နှင့် named argument နှစ်မျိုးလုံးကို support လုပ်သည်။ Positional argument များကို ဘယ်မှညာသို့ အစဉ်လိုက်ချိတ်ဆက်သည်။ Named argument ကို call အတွင်း `parameter = expression` ပုံစံဖြင့်ရေးပြီး ထိုနာမည်ရှိ parameter သို့ တိုက်ရိုက်ချိတ်ဆက်သည်။ မပေးထားသော parameter များအတွက် သတ်မှတ်ထားသော default value ကို အသုံးပြုသည်။

```zap
fn connect(host: text = "localhost", port: number = 8080, secure: bool = false):
    return host + ":" + str(port) + ":" + str(secure)

say connect()
say connect("api.example.com")
say connect(host = "api.example.com", secure = true)
say connect(port = 443, host = "api.example.com")
```

Named argument သည် အစောပိုင်း default များအားလုံးကို မရေးဘဲ နောက်ပိုင်း default တစ်ခုကို ပြောင်းလိုသောအခါ အသုံးဝင်သည်။ Named argument မတိုင်မီ positional argument ရေးနိုင်သော်လည်း named argument နောက်တွင် positional argument မရေးရပါ။ ထို့ကြောင့် `f(10, c = 30)` သည် မှန်ကန်ပြီး `f(a = 10, 20)` ကို reject လုပ်သည်။

## Required နှင့် defaulted parameter များကို ပေါင်းစပ်ခြင်း

Function တစ်ခုတွင် required parameter နှင့် default parameter နှစ်မျိုးလုံး ထည့်နိုင်သည်။ Required parameter ကို caller က မဖြစ်မနေ ပေးရမည်။ Default parameter ကိုမူ ချန်ထားနိုင်သည်။

```zap
fn create_user(username: text, role: text = "member", active: bool = true):
    return {
        "username": username,
        "role": role,
        "active": active
    }

say create_user("may")
say create_user("may", "admin", false)
```

ပထမ call တွင် required ဖြစ်သော `username` ကိုပေးပြီး `role` နှင့် `active` အတွက် default များကို အသုံးပြုသည်။ ဒုတိယ call တွင် default နှစ်ခုလုံးကို caller က ပြန်လည်သတ်မှတ်သည်။

## Default expression များ၏ အလုပ်လုပ်ပုံ

Default value ကို function declaration အချိန်တွင် တန်ဖိုးတစ်ခုအဖြစ်သာ ကူးယူထားခြင်းမဟုတ်ဘဲ argument မပါသော call အချိန်တွင် expression အဖြစ် evaluate လုပ်သည်။

```zap
fn welcome(prefix: text = "Hello", name: text = "World"):
    return prefix + ", " + name

say welcome()
say welcome("Mingalaba", "Zap")
```

Default expression သည် function call ၏ local environment အတွင်း evaluate လုပ်သည်။ ထို့ကြောင့် default များကို ရိုးရှင်းပြီး ခန့်မှန်းရလွယ်သော expression များအဖြစ်ထားသင့်သည်။ နောက်ပိုင်းတွင် bind လုပ်မည့် parameter ကို အစောပိုင်း default expression ထဲတွင် မမှီခိုသင့်ပါ။ မပေးထားသော default များကို canonical AST expression အဖြစ် parse/evaluate လုပ်ပြီး nested builtin call များပါ legacy line-expression parser သို့ ပြန်မဝင်တော့ပါ။

## Method နှင့် constructor များ

Class method နှင့် constructor များတွင်လည်း default parameter behavior တူညီသည်။ `self` သည် runtime က အလိုအလျောက်ပေးသော parameter ဖြစ်သောကြောင့် caller က ordinary argument အဖြစ် ထပ်မပေးရပါ။

```zap
class User:
    fn init(self, name: text = "Guest"):
        self.name = name

    fn label(self, prefix: text = "User"):
        return prefix + ": " + self.name

let guest = new("User")
let developer = new("User", "Developer")
say guest.label()
say developer.label("Account")
```

Method များတွင် runtime သည် `self` နောက်မှ ကျန်ရှိသော argument များကို စစ်ဆေးသည်။ Constructor နှင့် method များ၏ default value များသည် ordinary function များကဲ့သို့ ချန်ထားနိုင်ပြီး caller value ဖြင့် override လုပ်နိုင်သည်။ Built-in `new(...)` call သည် သီးခြား constructor boundary ဖြစ်ပြီး text class name၊ positional constructor argument များနှင့် positional explicit-field map တစ်ခုကို လက်ခံသည်။ Named argument များကို ရည်ရွယ်ချက်ရှိရှိ reject လုပ်ပြီး deterministic diagnostic ထုတ်ပေးသည်။ Named binding ကို user-defined function နှင့် method များအတွက် ဆက်လက်အသုံးပြုနိုင်သည်။

## Return type နှင့် default parameter

Default parameter သည် return type annotation နှင့် တွဲဖက်အသုံးပြုနိုင်သည်။ Caller က ပေးသော value ဖြစ်စေ default value ဖြစ်စေ parameter type ကို စစ်ဆေးပြီး function ပြန်ပေးသော value ကိုလည်း return annotation အတိုင်း စစ်ဆေးသည်။

```zap
fn port_or_default(port: number = 8080) -> number:
    return port

say port_or_default()
say port_or_default(3000)
```

## Validation rules

| စည်းမျဉ်း | ဥပမာ | ရလဒ် |
|---|---|---|
| Default expression မပါဘဲ `=` မရေးရ | `fn f(value =):` | Parse error |
| Parameter name ထပ်မရေးရ | `fn f(value, value):` | Parse error |
| ပေးထားသော argument သည် annotation နှင့်ကိုက်ညီရမည် | `f(n = "x")` where `n: number` | Type error |
| Default value သည် annotation နှင့်ကိုက်ညီရမည် | `fn f(n: number = "x"):` | Type-checking error |
| Required argument အားလုံးပေးရမည် | `fn f(a, b = 2)` then `f()` | Missing-argument error |
| Parameter အရေအတွက်ထက် မပိုရ | `fn f(a = 1)` then `f(1, 2)` | Argument-count error |
| မသိသော named parameter ကို reject လုပ်ရမည် | `fn f(a)` then `f(b = 1)` | Unknown named-argument error |
| Named name ထပ်မရေးရ | `f(a = 1, a = 2)` | Duplicate named-argument error |
| Named နောက်တွင် positional မရေးရ | `f(a = 1, 2)` | Binding-order error |
| Named binding သည် parameter name အတိုင်းချိတ်သည် | `f(second = 20, first = 10)` | Value များကို name အတိုင်း ချိတ်သည် |
| Built-in constructor name များကို reject လုပ်သည် | `new("User", name = "Guest")` | Deterministic unsupported-named-argument error |

Required parameter နှစ်ခုရှိသော function ကို argument တစ်ခုတည်းဖြင့် ခေါ်ပါက diagnostic သည် အောက်ပါပုံစံနှင့် ဆင်တူမည်—

```text
function expects 2 to 2 arguments, got 1
```

Required parameter တစ်ခုနှင့် default parameter နှစ်ခုရှိပါက အနည်းဆုံး argument တစ်ခု ပေးရမည်။ ထို့ကြောင့် argument သုညခုဖြင့်ခေါ်ပါက `function expects 1 to 3 arguments, got 0` ကဲ့သို့သော error ရနိုင်သည်။

## ပြည့်စုံသော example

Repository ထဲတွင် [`examples/default_parameters.zp`](../examples/default_parameters.zp) အဖြစ် run နိုင်သော example ပါရှိသည်။

```zap
fn greet(name: text = "World", punctuation: text = "!"):
    return "Hello, " + name + punctuation

fn rectangle_area(width: number, height: number = 1) -> number:
    return width * height

fn describe_user(username: text, role: text = "member", active: bool = true):
    say "username=" + username
    say "role=" + role
    say "active=" + str(active)

say greet()
say greet("Zap", ".")
say rectangle_area(8)
say rectangle_area(8, 3)
describe_user("developer")
describe_user("admin", "administrator", false)
```

Run လုပ်ရန်—

```bash
zap examples/default_parameters.zp
```

လက်ရှိ implementation သည် **positional နှင့် named arguments** နှစ်မျိုးလုံးကို default parameters နှင့်အတူ support လုပ်ထားသည်။ `greet(name = "Zap")` ကဲ့သို့သော named call များကို user-defined function နှင့် method များအတွက် structured AST call path မှတစ်ဆင့် အသုံးပြုနိုင်သည်။ `new(...)` အပါအဝင် built-in call များသည် built-in contract က သီးခြားခွင့်ပြုထားခြင်း မရှိလျှင် named argument များကို reject လုပ်သည်။ Native `new(...)` construction၊ default expression နှင့် unsupported-call diagnostic များသည် hidden legacy reparse မရှိဘဲ canonical AST execution path ပေါ်တွင်သာ ဆက်လက်လုပ်ဆောင်သည်။

## ဆက်လက်ဖတ်ရှုရန်

အထွေထွေ syntax reference ကို [`SYNTAX_GUIDE_EN.md`](SYNTAX_GUIDE_EN.md) တွင်ဖတ်နိုင်သည်။ Beginner course ကို [`LEARN_ZAP_MM.md`](LEARN_ZAP_MM.md) တွင်ဖတ်နိုင်သည်။ Implementation regression test ကို [`native/tests/core.rs`](../native/tests/core.rs) ထဲရှိ `applies_default_function_parameters` တွင်ကြည့်နိုင်သည်။
