# Zap Bootstrap နှင့် Self-Hosting Contract

**အခြေအနေ:** Zap v2.9.2 အတွက် B0 reference baseline

Zap ၏ self-hosting roadmap သည် အဆင့်လိုက်ဖြစ်သည်။ လက်ရှိ release သည် **Rust reference/native implementation** ဖြစ်ပြီး fully Zap-only compiler မဖြစ်သေးပါ။ Normative stage contract၊ သီးခြား version identity များနှင့် machine-readable ownership record များကို [`bootstrap/contracts`](../bootstrap/contracts/BOOTSTRAP_CONTRACT_MM.md) အောက်တွင် ထိန်းသိမ်းထားသည်။

## လက်ရှိ B0 boundary

Reference pipeline သည် အောက်ပါအတိုင်း ဖြစ်သည်။

```text
Zap source -> Rust lexer -> AST parser -> evaluator/runtime
```

ထို့ကြောင့် လက်ရှိ compiler ကို build လုပ်ရန် Rust/Cargo လိုအပ်နေသေးသည်။ Operating-system loader နှင့် explicit documented platform boundary များကို infrastructure boundary အဖြစ် လက်ခံထားပြီး အခြား language runtime နှင့် framework များကို လက်ရှိ Zap compiler path တွင် မလိုအပ်စေရ။

## Canonical inspection commands

Native CLI တွင် read-only B0 inspection command များ ထည့်ပြီးဖြစ်သည်။

```text
zap bootstrap status
zap bootstrap tokens <file.zp>
zap bootstrap ast <file.zp>
zap bootstrap diagnostics <file.zp>
```

ပထမ batch တွင် token၊ AST၊ diagnostic၊ metadata၊ platform-boundary နှင့် standard-library fixture များကို [`bootstrap/fixtures`](../bootstrap/fixtures) အောက်တွင် freeze လုပ်ထားသည်။ [`scripts/bootstrap/verify_b0_artifacts.sh`](../scripts/bootstrap/verify_b0_artifacts.sh) ကို run လုပ်ပါက artifact များကို ပြန်တည်ဆောက်ပြီး committed corpus နှင့် byte-for-byte နှိုင်းယှဉ်ပေးမည်။

## အဆင့်ဆိုင်ရာ policy

| အဆင့် | အဓိပ္ပါယ် | ခွင့်ပြုသော release claim |
|---|---|---|
| B0 | Rust က reference behavior နှင့် fixture များကို ပိုင်ဆိုင် | Rust reference/native implementation |
| B1 | Zap lexer/parser က B0 artifact များကို ပြန်ထုတ် | Zap bootstrap compiler foundation |
| B2 | Zap diagnostics/type checker က B0 acceptance/rejection ကို ပြန်ထုတ် | Zap bootstrap compiler foundation |
| B3 | Zap stdlib၊ typed IR၊ package resolver နှင့် test runner က offline/deterministic အလုပ်လုပ် | Zap-owned compiler pipeline in transition |
| B4 | Zap compiler က documented platform seed ဖြင့် မိမိကိုယ်ကို ပြန် build | Fully Zap-only self-hosted compiler |

B4 bootstrap check မအောင်မြင်သေးသရွေ့ release တစ်ခုတွင် B4 wording မသုံးရ။ နောင် semantic သို့မဟုတ် artifact change များအတွက် bilingual contract update၊ fixture change၊ ownership record၊ လိုအပ်ပါက compatibility decision နှင့် regression evidence လိုအပ်သည်။

## နောက် gate

နောက် implementation gate သည် B1 ဖြစ်သည်။ B0 fixture corpus တွင် သတ်မှတ်ထားသော token schema ကို ထုတ်ပေးနိုင်သည့် Zap-owned lexer နှင့် candidate output ကို Rust reference နှင့် နှိုင်းယှဉ်သည့် differential runner ကို တည်ဆောက်ရမည်။ Parser migration၊ type checking၊ VM နှင့် native backend အလုပ်များကို ထို gate မကျော်မီ ပြီးစီးသည်ဟု မဆိုရ။
