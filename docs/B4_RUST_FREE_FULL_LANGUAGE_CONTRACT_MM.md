# B4 Rust မသုံးသော Full-Language Compiler Contract

**Contract ID:** `B4-RUST-FREE-FULL-LANGUAGE`  
**Schema:** 1  
**အခြေအနေ:** လက်ရှိတွင် အသိအမှတ်ပြုမထားသေးပါ

## ရည်ရွယ်ချက်

ဤစာချုပ်သည် B4 အတွက် တရားဝင် acceptance boundary ဖြစ်သည်။ B4 သည် supported subset ကို သက်သေပြခြင်း မဟုတ်ပါ။ Language surface တစ်ခုလုံး၊ compiler pipeline၊ user-facing CLI၊ package/build path နှင့် test path အားလုံးကို Zap source က ပိုင်ဆိုင်ပြီး compiler path အတွင်း Rust၊ Cargo သို့မဟုတ် Rust host compiler မသုံးဘဲ လုပ်ဆောင်နိုင်မှသာ B4 အဖြစ် အသိအမှတ်ပြုနိုင်မည်။

> Rust-free seed pipeline သည် ကန့်သတ်ထားသော slice တစ်ခုအတွက် independence evidence သာဖြစ်သည်။ Full self-hosting evidence မဟုတ်ပါ။ ထို့ကြောင့် B4 contract integrity နှင့် B4 certification ကို သီးခြားထားသည်။

## ပိုင်ဆိုင်မှုဆိုင်ရာ လိုအပ်ချက်များ

| နယ်ပယ် | B4 လိုအပ်ချက် | သက်သေ |
|---|---|---|
| Language surface | Specification ထဲရှိ syntax၊ expression၊ statement၊ type၊ generic၊ module၊ error၊ async၊ package နှင့် runtime feature အားလုံးအတွက် acceptance fixture ရှိရမည်။ | `bootstrap/contracts/B4_ACCEPTANCE.tsv` |
| Front end | Zap-owned lexer/parser က canonical AST ကို language surface အပြည့်အဝအတွက် ထုတ်ပေးရမည်။ | Source-to-AST fixture results |
| Static pipeline | Zap-owned type checker နှင့် typed-IR producer က accept/reject behavior အပါအဝင် တူညီသော surface ကို cover လုပ်ရမည်။ | Typed-IR နှင့် diagnostic fixtures |
| Execution | Zap-owned lowering၊ bytecode/VM နှင့် runtime က accepted fixtures များကို deterministic အဖြစ် run ရမည်။ | Source-to-VM fixture results |
| CLI | `check`၊ `build`၊ `run`၊ `test`၊ package နှင့် diagnostics path များသည် Zap-owned compiler code ကိုသာ dispatch လုပ်ရမည်။ | CLI ownership နှင့် no-fallback checks |
| Package/build | Manifest၊ lockfile၊ dependency၊ artifact နှင့် rebuild operation များကို Zap က ပိုင်ဆိုင်ရမည်။ | Package/build fixture results |
| Test runner | Test discovery၊ execution၊ result encoding နှင့် failure reporting ကို Zap က ပိုင်ဆိုင်ရမည်။ | Test-runner fixture results |
| Host boundary | OS loading နှင့် document ပြုထားသော platform primitives များသာ seed boundary ဖြစ်ရမည်။ | Platform-seed evidence |

## ခွင့်မပြုသော fallback များ

Compiler path သည် `cargo`၊ `rustc`၊ `rustup`၊ Rust native implementation သို့မဟုတ် Rust host wrapper ကို invoke သို့မဟုတ် depend မလုပ်ရ။ B4 migration မပြီးမချင်း reference oracle အဖြစ် သီးခြား development job တွင် ထားနိုင်သော်လည်း certified Zap CLI/build/test invocation က ထို oracle ကို မရောက်ရ။

## Full-language သတ်မှတ်ချက်

Acceptance manifest သည် လက်ရှိ seed slice ထက် ပိုကျယ်သည်။ Lexer/parser၊ expression/control flow၊ function/closure၊ class/method၊ collection/map၊ alias/generic၊ result/option၊ module/import၊ async၊ diagnostics၊ package/build metadata၊ VM execution နှင့် test-runner output များကို ကိုယ်စားပြု fixture များ ပါဝင်သည်။ Fixture ဖိုင်ရှိရုံဖြင့် မပြီးပါ။ Zap-owned pipeline က သတ်မှတ်ထားသော artifact နှင့် deterministic result ကို ထုတ်ပေးရမည်။

Supported platform အားလုံးတွင် row အားလုံး pass ဖြစ်ရမည်။ တူညီသော source/seed input ဖြင့် independent rebuild နှစ်ကြိမ်၏ artifact bytes တူညီရမည်။ `provisional` row တစ်ခုခု သို့မဟုတ် Rust/Cargo fallback တစ်ခုခု ရှိပါက repository သည် **not-certified** အဖြစ်သာ ရှိရမည်။

## လက်ရှိအခြေအနေ

Zap တွင် verified `ZAP_BOOTSTRAP_BIN` seed (current native binary) နှင့် `bootstrap/b4/compiler_driver.zp` (contract status `owned`) ပါသော Zap-owned compiler driver ရှိပြီး ၎င်းသည် lexer/parser/module-resolution/typecheck/lower/VM/package အဆင့်များကို တိုက်ရိုက်ဦးစီပေးပါပြီး typed-IR schema တွင် `candidate_only` မပါဝင်ပါ။ Driver-owned pipeline၊ source-to-VM၊ module resolution၊ typed-IR semantics၊ byte-determinism၊ second-stage rebuild နှင့် clean-environment gates အားလုံး current seed ဖြင့် အောင်မြင်ပါသည်။ Full B4 certification ရရှိရန် Linux x86_64၊ macOS ARM64 နှင့် Windows x86_64 clean environments တို့တွင် platform-native seeds ဖြင့် ထပ်မံ verify လုပ်ရန် လိုအပ်ပါသည်။

## ကိုးကားချက်များ

[1]: ../bootstrap/contracts/BOOTSTRAP_CONTRACT_EN.md
[2]: ../bootstrap/contracts/B4_RUST_FREE_FULL_LANGUAGE_CONTRACT.toml
[3]: ../bootstrap/contracts/B4_ACCEPTANCE.tsv
