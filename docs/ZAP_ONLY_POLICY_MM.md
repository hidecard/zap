# Zap-Only Dependency Policy

**အခြေအနေ:** Zap v2.9.0 အတွက် self-hosting policy မူကြမ်း

## ရည်ရွယ်ချက်

ရေရှည်တွင် Zap သည် compiler၊ type checker၊ standard library၊ package manager နှင့် bootstrap test runner များကို Rust၊ Cargo၊ Go၊ Python၊ Node.js၊ JavaScript framework သို့မဟုတ် အခြား third-party language runtime မလိုဘဲ Zap source ကိုယ်တိုင်မှ build/run လုပ်နိုင်ရမည်။

ဤရည်ရွယ်ချက်သည် operating system၊ CPU architecture၊ executable loader၊ linker၊ filesystem သို့မဟုတ် minimal platform seed မလိုဟု ဆိုလိုခြင်းမဟုတ်ပါ။ ထိုအရာများသည် language/framework dependency မဟုတ်ဘဲ platform boundary များဖြစ်သည်။

## Dependency အမျိုးအစားများ

| အမျိုးအစား | ဥပမာ | ရေရှည် policy |
|---|---|---|
| Zap-owned | `.zp` compiler source၊ Zap stdlib၊ typed IR၊ bootstrap fixtures | ဤ repository တွင် versioned အဖြစ် မဖြစ်မနေ ပါရမည် |
| ယာယီ bootstrap | လက်ရှိ Rust reference runtime နှင့် Cargo build | B0/B1 transition အတွင်းသာ ခွင့်ပြုပြီး B4 clean build တွင် မလိုရ |
| Platform seed | OS loader၊ syscall၊ libc/ABI သို့မဟုတ် platform-specific seed binary သေးငယ် | အနည်းဆုံးထား၊ documentation ရေး၊ language semantics အပြင်ဘက်တွင် ထားရမည် |
| တားမြစ်ထားသော build dependency | Zap compiler compile ရန်လိုသော Python/Node/Go/Cargo script | B4 compiler path မှ ဖယ်ရှားရမည် |
| Optional integration | React/Vue/Svelte၊ external database၊ reverse proxy | Application တွင် သုံးနိုင်သော်လည်း Zap core အတွက် မလိုရ |

## လိုအပ်သော architecture

Zap compiler path ကို Zap-owned pure layers များဖြစ်သည့် lexer၊ parser/AST၊ type checker၊ diagnostic model၊ typed IR၊ pure standard library၊ package resolver နှင့် bootstrap test runner အဖြစ် ခွဲရမည်။ Filesystem၊ process၊ network၊ Web နှင့် database operation များသည် implicit compiler dependency မဟုတ်ဘဲ capability-backed host adapter ဖြစ်ရမည်။

ပထမ self-hosted compiler သည် stable typed IR သို့မဟုတ် canonical artifact format ကို ထုတ်ပေးသင့်သည်။ B4 reproducibility မရမချင်း native machine code ကို တိုက်ရိုက်ထုတ်ရန် မစသင့်ပါ။ B4 ပြီးမှ Zap-owned backend သို့မဟုတ် documentation ပါသော minimal platform seed ဖြင့် native code generation ကို ထပ်လုပ်နိုင်သည်။

## B0–B4 acceptance

- **B0:** လက်ရှိ Rust implementation သည် reference implementation ဖြစ်ပြီး အစပိုင်း behavior ကို ပိုင်သည်။
- **B1:** Zap lexer/parser output သည် B0 token၊ AST နှင့် diagnostic fixture များနှင့် တူရမည်။
- **B2:** Zap type checker သည် B0 accept/reject decision နှင့် diagnostic JSON ကို တူညီစွာ ထုတ်ရမည်။
- **B3:** Zap stdlib၊ package resolver နှင့် typed IR bridge သည် network သို့မဟုတ် ambient environment မပါဘဲ run နိုင်ရမည်။
- **B4:** Rust/Cargo၊ Go၊ Python၊ Node.js နှင့် JavaScript runtime မပါသော clean machine တွင် documented platform seed တစ်ခုတည်းဖြင့် Zap source မှ compiler ကို ပြန် build နိုင်ရမည်။

အဆင့်တိုင်းတွင် source hash၊ compiler hash၊ schema version နှင့် artifact hash များကို မှတ်တမ်းတင်ရမည်။ Clean build နှစ်ကြိမ်ဆက်တိုက်တွင် canonical artifact hash တူရမည်။

## မလုပ်ရမည့် shortcut များ

Zap compiler မရှိသေးမီ `Cargo.toml` ဖယ်ရှားခြင်းသည် self-hosting မဟုတ်ဘဲ လက်ရှိ build path ကို ဖျက်ခြင်းသာ ဖြစ်သည်။ Rust နေရာတွင် အခြား host language တစ်ခုကို အစားထိုးခြင်းလည်း Zap-only မဟုတ်ပါ။ Third-party parser၊ VM၊ package manager သို့မဟုတ် Web framework သည် core bootstrap path ထဲ မပါရ။ ထည့်မည်ဆိုပါက Zap-owned source အဖြစ် ပြန်ရေးထားခြင်း သို့မဟုတ် license နှင့် deterministic contract ရှင်းလင်းစွာရှိသော vendored source ဖြစ်ရမည်။

## Release gate

B4 clean machine test အောင်မြင်ပြီး၊ compiler/stdlib source အားလုံး Zap-owned ဖြစ်ပြီး၊ bootstrap test runner သည် Zap-owned ဖြစ်ကာ platform seed ကို document လုပ်ထားပြီး၊ build command သည် တားမြစ်ထားသော runtime မလိုမှသာ “Zap-only self-hosted” ဟု release တွင် ကြေညာနိုင်သည်။ ထိုအချိန်မတိုင်မီ release wording ကို “Rust-bootstrapped” သို့မဟုတ် “self-hosting foundation” ဟုသာ သုံးရမည်။
