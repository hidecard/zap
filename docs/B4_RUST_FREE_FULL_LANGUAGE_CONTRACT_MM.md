# B4 Rust မသုံးသော Full-Language Compiler Contract

**Contract ID:** `B4-RUST-FREE-FULL-LANGUAGE`  
**Schema:** 2  
**အခြေအနေ:** လက်ရှိတွင် အသိအမှတ်ပြုမထားသေးပါ

## ရည်ရွယ်ချက်

ဤစာချုပ်သည် B4 အတွက် တရားဝင် acceptance boundary ဖြစ်သည်။ Language surface တစ်ခုလုံး၊ compiler pipeline၊ user-facing CLI၊ package/build path နှင့် test path အားလုံးကို Zap source က ပိုင်ဆိုင်ပြီး compiler path အတွင်း Rust၊ Cargo သို့မဟုတ် Rust host compiler မသုံးဘဲ လုပ်ဆောင်နိုင်မှသာ B4 အဖြစ် အသိအမှတ်ပြုနိုင်မည်။

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

## Schema v2 seed provenance

Acceptable path များမှာ:

1. **Python seed compiler + C backend**
   - `host/zap-bootstrap/compile.py` က reference seed compiler အဖြစ် လုပ်ဆောင်သည်
   - `host/zap-bootstrap/c_backend.py` က Zap bytecode မှ C ထုတ်ပြီး system C compiler ဖြင့် native binary ထုတ်သည်
   - System C compiler (`gcc`/`clang`/MSVC `cl.exe`) သည် platform primitive ဖြစ်ပြီး Rust fallback မဟုတ်ပါ

2. **Zap-written compiler + C backend**
   - Full compiler pipeline က `bootstrap/b1/b2/b3/b4/` တွင် Zap source ဖြင့် ရေးထားရမည်
   - C backend က native code generation အတွက် အသုံးပြုမည်

## Full-language သတ်မှတ်ချက်

Acceptance manifest သည် လက်ရှိ seed slice ထက် ပိုကျယ်သည်။ Lexer/parser၊ expression/control flow၊ function/closure၊ class/method၊ collection/map၊ alias/generic၊ result/option၊ module/import၊ async၊ diagnostics၊ package/build metadata၊ VM execution နှင့် test-runner output များကို ကိုယ်စားပြု fixture များ ပါဝင်သည်။ Fixture ဖိုင်ရှိရုံဖြင့် မပြီးပါ။ Zap-owned pipeline က သတ်မှတ်ထားသော artifact နှင့် deterministic result ကို ထုတ်ပေးရမည်။

Supported platform အားလုံးတွင် row အားလုံး pass ဖြစ်ရမည်။ တူညီသော source/seed input ဖြင့် independent rebuild နှစ်ကြိမ်၏ emitted C နှင့် stdout bytes တူညီရမည်။ Native executable hash များသည် platform အလိုက် ကွဲနိုင်သည်။ `provisional` row တစ်ခုခု သို့မဟုတ် Rust/Cargo fallback တစ်ခုခု ရှိပါက repository သည် **not-certified** အဖြစ်သာ ရှိရမည်။

## Acceptance commands

```text
scripts/bootstrap/verify_b4_c_backend_acceptance.sh
scripts/bootstrap/verify_b4_rust_free_contract.sh
```

ပထမ gate သည် B4-FULL-013..018 ကို compile/build/run/hash လုပ်ပြီး `target/b4-c-backend-acceptance.tsv` ထုတ်ပေးသည်။ Cross-platform CI က Linux/Windows/macOS မှ emitted C နှင့် stdout hash များကို နှိုင်းယှဉ်သည်။

## လက်ရှိအခြေအနေ

Zap တွင် Rust-free C backend path၊ 19-row Schema v2 acceptance manifest နှင့် B4-FULL-013..018 အတွက် local executable evidence ရှိသော်လည်း B4 သည် **လက်ရှိတွင် အသိအမှတ်ပြုမထားသေးပါ** (not-certified)။

- `bootstrap/contracts/B4_ACCEPTANCE.tsv` တွင် rows 19 ခု ပါဝင်ပြီး current local evidence အရ အားလုံး `pass` ဖြစ်သည်
- `scripts/bootstrap/verify_b4_c_backend_acceptance.sh` က CLI၊ self-rebuild، cross-platform replay၊ byte determinism၊ second-stage rebuild နှင့် clean-environment execution ကို စစ်သည်
- `host/zap-bootstrap/verify_c_backend.py` တွင် representative programs 10 ခု pass ဖြစ်သည်
- Cross-platform CI matrix နှင့် hash aggregator ကို wiring လုပ်ထားသည်
- Python seed compiler သည် လက်ရှိအချိန်အထိ reference implementation ဖြစ်ပြီး Zap-owned production compiler မဟုတ်သေးပါ

ကျန်သော certification blockers များမှာ Linux/Windows/macOS matrix က ဤ revision အတွက် အောင်မြင်ရန်၊ Python lowering path ကို Zap-owned B1..B4 production pipeline သို့ ပြောင်းရန် နှင့် cross-platform evidence ရပြီးနောက် contract review လုပ်ရန် ဖြစ်သည်။

## ကိုးကားချက်များ

[1]: ../bootstrap/contracts/BOOTSTRAP_CONTRACT_EN.md
[2]: ../bootstrap/contracts/B4_RUST_FREE_FULL_LANGUAGE_CONTRACT.toml
[3]: ../bootstrap/contracts/B4_ACCEPTANCE.tsv
