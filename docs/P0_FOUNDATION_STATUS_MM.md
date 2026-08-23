# P0 AST Foundation အခြေအနေ

**Zap v2.2.7 နှင့် စစ်ဆေးအတည်ပြုထားပါသည်။**

## AST migration အခြေအနေ

Zap တွင် source span ပါဝင်သော AST foundation ကို `native/src/ast.rs` တွင် ထည့်သွင်းထားပါသည်။ Parser သည် precedence၊ call၊ indexing၊ conditional expression နှင့် `await` ပါသော expression များအပြင် assignment၊ typed `let` declaration၊ `say`၊ import၊ return၊ `break`၊ `continue`၊ `if/else`၊ `while`၊ `for`၊ `try/catch`၊ `raise`၊ module၊ function၊ class နှင့် exported binding/function များကို လက်ခံပါသည်။

| Declaration | Support ပြုထားသော AST shape | မှတ်ချက် |
|---|---|---|
| `fn add(a: number) -> number:` | `Stmt::Function` | Name၊ parameters၊ optional annotation များ၊ return annotation၊ body၊ visibility၊ async flag နှင့် export flag များကို သိမ်းထားပါသည်။ |
| `class Child(Parent):` | `Stmt::Class` | Class name၊ optional single parent name နှင့် indented body ကို သိမ်းထားပါသည်။ |
| `let total: number = 1` | `Stmt::Declaration` | Variable name၊ optional annotation၊ value၊ export flag နှင့် source span ကို သိမ်းထားပါသည်။ |
| `say value` | `Stmt::Say` | Native AST execution အတွက် output expression ကို သိမ်းထားပါသည်။ |
| `import` / `use` | `Stmt::Import` | Module path၊ alias နှင့် explicit-import mode ကို သိမ်းထားပါသည်။ |
| `if` / `while` / `for` / `try` | Control-flow statement nodes | Indentation-aware block parser နှင့် native flow propagation တစ်ခုတည်းကို အသုံးပြုပါသည်။ |

AST parser သည် four-space indentation ကို enforce လုပ်ပြီး tab/mixed indentation များကို reject လုပ်ပါသည်။ Declaration သို့မဟုတ် control-flow header တိုင်း၏နောက်တွင် indented body လိုအပ်ပြီး one-based source location များကို ထိန်းသိမ်းထားပါသည်။ `run()` သည် normal source program တိုင်းကို AST boundary မှတစ်ဆင့် parse လုပ်ပြီး parse failure ဖြစ်လျှင် syntax diagnostic ပြန်ပေးပါသည်။ Line interpreter သို့ fallback မလုပ်တော့ပါ။ Local module file များကိုလည်း export marker များအပါအဝင် AST boundary မှတစ်ဆင့် parse နှင့် execute လုပ်ပါသည်။

## Compatibility-only legacy boundary

Line interpreter သည် `ast_body: Program` မပါဘဲ `body: Vec<String>` သာ ပါသော ယခင် သို့မဟုတ် test-only path မှ ဖန်တီးထားသော `Function` record များအတွက် internal compatibility boundary အဖြစ် ဆက်ရှိနေပါသည်။ Source အသစ်များနှင့် parser မှ အသစ်ဖန်တီးသော function များသည် ထို representation ကို မမှီခိုရပါ။ Line interpreter ထဲသို့ syntax အသစ် မထည့်ပါ။ Compatibility behavior ကို လက်ရှိ release line အတွက် ထိန်းသိမ်းထားပြီး legacy fixture များနှင့် migration guidance ကို ပြန်လည်သုံးသပ်ပြီး သီးခြား documented breaking release ဆုံးဖြတ်ချက်ချပြီးမှသာ ဖယ်ရှားနိုင်ပါသည်။

> **မူဝါဒ:** Parser ပိုင် source များအတွက် Native AST execution သည် normative ဖြစ်သည်။ Line-based execution သည် compatibility-only ဖြစ်ပြီး normal-program fallback မဟုတ်ပါ။

## Runtime safety semantics

Zap integer arithmetic သည် checked operation များကို အသုံးပြုပါသည်။ Addition၊ subtraction နှင့် multiplication တွင် signed integer range ကျော်လွန်ပါက `OverflowError` ပြန်ပေးပါသည်။ Division နှင့် modulo ကို zero ဖြင့်လုပ်ပါက panic မဖြစ်စေဘဲ runtime error ပြန်ပေးပါသည်။ `i64::MIN / -1` နှင့် `i64::MIN % -1` ကိုလည်း integer overflow အဖြစ် reject လုပ်ပါသည်။

Sequence indexing သည် zero-based ဖြစ်ပါသည်။ Negative numeric index သည် invalid ဖြစ်ပြီး `index out of range` ပြန်ပေးပါသည်။ Sequence length ထက်ကြီး/တူသော index တွင်လည်း error တူညီပါသည်။ Map indexing သည် text key ကို အသုံးပြုပြီး key မရှိလျှင် `key not found` ပြန်ပေးပါသည်။

## လက်ခံအတည်ပြုမှု အခြေအနေ

Native suite သည် လက်ရှိ **232 unit tests** နှင့် **256 core integration tests** ကို pass ဖြစ်ပါသည်။ Function၊ method၊ export နှင့် local-module body များအပါအဝင် parser ပိုင် program အားလုံးအတွက် AST boundary နှင့် native AST execution ကို အသုံးပြုနေပါသည်။ Resource limit၊ typed diagnostic နှင့် legacy source-line compatibility များကို legacy-created runtime function များနှင့် explicit compatibility test များအတွက်သာ ဆက်လက်ထားရှိပါသည်။

## P0 completion boundary

Normal program နှင့် local module များအတွက် canonical AST execution slice ပြီးစီးပါပြီ။ AST parser မှ ဖန်တီးသော runtime function နှင့် class method များသည် ၎င်းတို့၏ `Program` body ကို တိုက်ရိုက်သိမ်းထားပြီး source reconstruction မလုပ်ဘဲ execute လုပ်ပါသည်။ ၎င်းတို့၏ lexical closure များတွင် parent-linked live binding cell များကို ထိန်းသိမ်းထားသောကြောင့် outer reassignment ကို returned closure က မြင်နိုင်ခြင်း၊ sibling closure များက captured binding တစ်ခုတည်းကို share လုပ်ခြင်း၊ အတွင်း `let` declaration က outer binding ကို မပြောင်းဘဲ shadow လုပ်ခြင်းနှင့် recursive call များတွင် ကိုယ်ပိုင် call frame ထိန်းသိမ်းခြင်းတို့ကို support လုပ်ပါသည်။ ကျန်ရှိသော line-based representation သည် older/internal declaration များအတွက် compatibility format အဖြစ် ရည်ရွယ်ချက်ရှိရှိ ထိန်းသိမ်းထားပါသည်။ ၎င်းကို ဖယ်ရှားခြင်းသည် implicit behavior change မဟုတ်ဘဲ future breaking compatibility decision တစ်ခု ဖြစ်ပါသည်။
