# Zap v0.7.0 Roadmap နှင့် Feature Boundary

## ရည်ရွယ်ချက်

v0.7.0 သည် OOP foundation ကို ဆက်လက်အသုံးချနိုင်စေရန် standard library၊ collection operations၊ line-based file I/O နှင့် explicit CLI workflow ကို အဓိကထားသော release ဖြစ်သည်။ Language တစ်ခုကို အလျင်အမြန် feature အများကြီးဖြည့်ခြင်းထက် syntax compatibility၊ error message၊ tests နှင့် cross-platform behavior ကို ထိန်းသိမ်းခြင်းကို ဦးစားပေးထားသည်။

## Implemented in v0.7.0

| အပိုင်း | အခြေအနေ |
|---|---|
| OOP | `class`၊ `extends`၊ `new`၊ `init`၊ methods၊ properties၊ `self` နှင့် inherited methods |
| Collection helpers | `is_empty`၊ `sum`၊ `reverse`၊ `sort`၊ `get` |
| File helpers | `read_text`၊ `write_text`၊ `read_lines`၊ `write_lines` |
| Existing standard library | path၊ time၊ environment၊ math၊ text၊ JSON၊ assertion |
| CLI | `init`၊ `check`၊ `build`၊ `test`၊ `fmt`၊ explicit `run`၊ version၊ help |
| Testing | Native integration regression suite၊ OOP နှင့် v0.7 helpers coverage |
| Documentation | README၊ syntax reference၊ Burmese lessons၊ release notes |

## Language features that remain intentionally simple

Zap သည် လက်ရှိတွင် dynamic values နှင့် optional variable annotations ကို အသုံးပြုသည်။ Annotation ရေးထားပါက runtime တွင် basic type mismatch ကို စစ်ဆေးပေးသော်လည်း full compile-time type checker မဟုတ်သေးပါ။ Error များသည် runtime `Result` value model မဟုတ်သေးဘဲ command failure နှင့် readable error message အဖြစ် ထွက်ပေါ်သည်။

ဤ boundary သည် beginner များအတွက် syntax ကို ရိုးရှင်းစေပြီး runtime ကို လွယ်ကူစွာ တိုးချဲ့နိုင်စေရန် ရည်ရွယ်ထားသည်။

## v0.8.0 အတွက် ဦးစားပေးအစီအစဉ်

### 1. Structured error values

`Result` သို့မဟုတ် equivalent error value ကို စတင်သတ်မှတ်မည်။ Error တွင် kind၊ message၊ file၊ line နှင့် column metadata ပါဝင်သင့်သည်။ `?` propagation syntax ကို error semantics တည်ငြိမ်ပြီးမှ ထည့်မည်။

### 2. Module boundary နှင့် package foundation

လက်ရှိ local module search ကို explicit module exports၊ circular import detection၊ package metadata validation နှင့် lockfile-ready manifest structure ဖြင့် တိုးချဲ့မည်။ Remote package registry ကို API နှင့် security policy မသတ်မှတ်မီ မထည့်သွင်းသေးပါ။

### 3. Type diagnostics

Optional annotations ကို function parameters၊ return types၊ object properties နှင့် module boundaries သို့ တဖြည်းဖြည်း တိုးချဲ့မည်။ `zap check --json` သည် editor နှင့် CI integration အတွက် ထည့်သင့်သည်။

### 4. Async I/O foundation

Web၊ AI API နှင့် IoT event loop များအတွက် `async`၊ `await`၊ task cancellation၊ timeout နှင့် channels ကို အရင်သတ်မှတ်မည်။ Shared mutable threads နှင့် locks များကို memory model မတည်ငြိမ်မီ မထည့်သွင်းသေးပါ။

### 5. Tooling quality

`zap lint`၊ `zap doc`၊ test filtering၊ watch mode၊ structured diagnostics နှင့် reproducible build metadata များကို ထည့်မည်။

## မလုပ်သင့်သေးသောအရာများ

Package registry၊ multiple inheritance၊ unrestricted FFI၊ unsafe memory access၊ implicit network access နှင့် production-grade web framework များကို foundation မတည်ငြိမ်မီ မထည့်သွင်းသင့်ပါ။ Feature တစ်ခုစီသည် syntax specification၊ runtime behavior၊ error behavior၊ documentation နှင့် regression tests အားလုံးပါမှ stable feature အဖြစ် သတ်မှတ်မည်။

## Acceptance criteria

နောက် release မတိုင်မီ—

1. Linux၊ Windows နှင့် macOS ARM64 တွင် core tests တစ်ပြေးညီ pass ဖြစ်ရမည်။
2. Built-in တစ်ခုစီအတွက် success နှင့် invalid input test ရှိရမည်။
3. README နှင့် Burmese learning guide တွင် implemented/proposed boundary တူညီရမည်။
4. Release archive တစ်ခုစီတွင် binary၊ installation instructions နှင့် SHA-256 checksum ပါရမည်။
5. Breaking syntax ပြောင်းလဲမှုတိုင်းတွင် migration note ပါရမည်။
