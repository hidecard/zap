# Zap Core Language Specification

## လက်ရှိ native implementation

Zap native runtime ကို Rust ဖြင့်ရေးသားထားပြီး source files များကို standalone execution pipeline ဖြင့် run ပေးသည်။ လက်ရှိ native core တွင် lexer၊ expression parser နှင့် runtime evaluator ပါဝင်သည်။

| Feature | Status |
|---|---|
| Comments | Implemented with `#` |
| Strings | Implemented with double quotes |
| Integers | Implemented |
| Booleans/none | Implemented |
| Lists | Implemented with `[]` |
| Variables | `let name = expression` နှင့် assignment |
| Arithmetic | `+`, `-`, `*`, `/` |
| Comparison | `==`, `!=`, `<`, `>`, `<=`, `>=` |
| Output | `say expression` |
| Conditional | `if expression:` နှင့် `else:` |
| Module declaration | `use module` foundation |

| Loops | `while condition:` နှင့် `for item in list:` |
| Maps | `{key: value}` နှင့် `map["key"]` |
| Builtins | `len`၊ `range`၊ `str`၊ `type`၊ `keys`၊ `contains`၊ `join`၊ `abs`၊ `min`၊ `max`၊ `upper`၊ `lower`၊ `trim`၊ `split`၊ `assert`၊ JSON နှင့် file I/O |

## ဥပမာ

```zap
let numbers = [1, 2, 3]
let total = 2 + 3 * 4
say total

if total > 10:
    say "large"
else:
    say "small"
```

## Native design direction

Zap core ၏ နောက်ထပ် language-level features များကို parser နှင့် AST layer အပေါ်တွင် တည်ဆောက်မည်။ Functions အတွက် lexical scope၊ parameter list၊ return value နှင့် closure capture ကို ထည့်မည်။ Control flow အတွက် block AST ကို အသုံးပြုပြီး လက်ရှိ line-by-line execution ထက် source structure ကို ပိုမိုတိကျစွာ ကိုင်တွယ်မည်။

## ဦးစားပေးအဆင့်

ပထမအဆင့်တွင် functions၊ block scope၊ `while`၊ `for item in list`၊ list indexing နှင့် map/object ကို ထည့်မည်။ ဒုတိယအဆင့်တွင် module import၊ file/JSON/time standard library နှင့် structured error reporting ကို ထည့်မည်။ တတိယအဆင့်တွင် async task၊ native FFI နှင့် compiler bytecode/optimisation ကို ထည့်မည်။

> Framework များကို မစတင်မီ Zap core ၏ syntax၊ scope၊ type/value semantics နှင့် module contract များ တည်ငြိမ်အောင် ပြီးစီးရမည်။
