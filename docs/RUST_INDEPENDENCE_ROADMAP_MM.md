# Rust မှ လွတ်လပ်ရေး roadmap

## လက်ရှိအခြေအနေ

Zap သည် bootstrap stage **B0** တွင်ရှိနေဆဲ ဖြစ်သည်။ Language semantics,
diagnostic, package/build behavior နှင့် supported release artifact အပြည့်အစုံ
အတွက် Rust native compiler/runtime သည် authoritative implementation
ဖြစ်နေဆဲဖြစ်သည်။
[self-hosting acceptance contract](COMPILER_SELF_HOSTING_A_ACCEPTANCE_MM.md)
ရှိ A1 မှ A13 gate အားလုံး မpass မချင်း Zap ကို fully self-hosted ဟု release
သို့မဟုတ် documentation တွင် မဆိုရပါ။

Rust မလိုသော seed path တစ်ခုကို သီးခြား verify လုပ်ထားသည်။

```text
Zap source (supported seed subset)
  -> host/zap-bootstrap/compile.py
  -> bytecode
  -> host/zap-vm-host/run.py
  -> output
```

`scripts/bootstrap/verify_non_rust_seed_pipeline.sh` သည် Rust toolchain
environment variable များကို ဖယ်ရှားပြီး ဤ path ကို run ပါသည်။ Arithmetic,
branch, loop, call, recursion, closure, class/method နှင့် caught raise တို့ကို
fixture အသေးများဖြင့် cover လုပ်ထားသည်။ ဤသည်မှာ Rust မလိုသော execution-path
evidence သာဖြစ်သည်။ Seed host များသည် Python implementation ဖြစ်သေးသဖြင့်
final Zap-owned compiler/runtime မဟုတ်သေးပါ။

## Ownership plan

| Phase | Deliverable | လက်ခံမှု boundary |
|---|---|---|
| 1. Independent seed | Source-to-bytecode-to-VM seed gate ကို CI တွင် green ထားရန် | Supported seed fixture များအတွက် Cargo/rustc/rustup process မလိုခြင်း |
| 2. Canonical front end | B1 lexer/parser ကို fixture shape မဟုတ်ဘဲ arbitrary source consume လုပ်စေရန် | A7/A8 candidate/reference parity နှင့် deterministic evidence ရှိခြင်း |
| 3. Type ownership | Parser AST ပေါ်မှ B2 inference, alias, generic, flow/mutation နှင့် diagnostic ပြီးစီးရန် | Negative case အပါအဝင် A1--A6 matrix pass |
| 4. Compiler ownership | Source-string routing မရှိသော canonical AST -> typed-IR producer နှင့် lowering ပြီးစီးရန် | A9 artifact deterministic နှင့် reference semantic match |
| 5. Runtime ownership | Zap-owned VM တွင် bytecode, call, closure, class, collection, error, limit, package/build ကို execute လုပ်ရန် | A10/A11 differential, limit, security gate pass |
| 6. Self rebuild | Zap ဖြင့် documented seed ကို build လုပ်ပြီး compiler ကို self-rebuild လုပ်ရန် | Linux x86_64, macOS ARM64, Windows x86_64 တွင် A12/A13 pass |

## လုပ်ဆောင်မှု စည်းမျဉ်း

- Increment တိုင်းတွင် positive/negative fixture pair, deterministic replay နှင့်
  candidate/reference comparison လိုအပ်သည်။
- Seed-pipeline test green ဖြစ်ရုံဖြင့် Rust မှ ownership မလွှဲပါ။
- Replacement gate pass ပြီးမှသာ Rust stage တစ်ခုကို ဖယ်ရှားရမည်။
- နောက်ဆုံး bootstrap toolchain သည် Zap-owned ဖြစ်ရမည်။ Python ကို
  corresponding Zap component independent မဖြစ်မချင်း temporary seed host/test
  harness အဖြစ်သာ သုံးနိုင်သည်။

## Run command

```bash
make bootstrap-non-rust-test
# သို့မဟုတ်
bash scripts/bootstrap/verify_non_rust_seed_pipeline.sh
```

[English roadmap](RUST_INDEPENDENCE_ROADMAP_EN.md) နှင့်
[self-hosting acceptance contract](COMPILER_SELF_HOSTING_A_ACCEPTANCE_MM.md) ကိုလည်း
ကြည့်ပါ။
