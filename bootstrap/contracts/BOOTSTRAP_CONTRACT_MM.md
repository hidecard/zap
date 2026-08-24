# Zap Bootstrap Contract

**အခြေအနေ:** Zap v2.10.1 အတွက် B0 normative contract

ဤ contract သည် လက်ရှိ Rust reference implementation မှ Zap-only self-hosted ecosystem သို့ သွားမည့် အဆင့်လိုက်လမ်းကြောင်းကို သတ်မှတ်သည်။ Zap သည် လက်ရှိတွင် self-hosted ဖြစ်ပြီးဟု မဆိုလိုပါ။ Operating-system loader၊ executable format၊ filesystem နှင့် document လုပ်ထားသော platform seed များသည် အဆင့်တိုင်း၏ platform boundary အဖြစ် ဆက်လက်ရှိမည်။

## Bootstrap အဆင့်များ

| အဆင့် | ပြည့်ရမည့်အရာ | Rust/Cargo အခြေအနေ | Release wording |
|---|---|---|---|
| B0 | Rust implementation က reference behavior နှင့် canonical fixtures ကို ပိုင်ဆိုင်ရမည် | လိုအပ် | Rust reference/native implementation |
| B1 | Zap-owned lexer နှင့် parser က B0 token/AST contract များကို ပြန်ထုတ်ရမည် | B0 seed က candidate ကို build ပေးနိုင် | Zap bootstrap compiler foundation |
| B2 | Zap-owned diagnostics နှင့် type checker က B0 accept/reject behavior ကို ပြန်ထုတ်ရမည် | B0/B1 bridge ကျန်နိုင် | Zap bootstrap compiler foundation |
| B3 | Zap-owned stdlib၊ typed IR၊ package resolver နှင့် test runner က deterministic offline artifacts ထုတ်ရမည် | B0 bridge အနည်းငယ် ကျန်နိုင် | Zap-owned compiler pipeline in transition |
| B4 | Zap compiler က documented platform seed ဖြင့် မိမိ source ကို ပြန် build ရမည် | Compiler path တွင် မလို | Fully Zap-only self-hosted compiler |

B4 မအောင်မြင်သေးသရွေ့ documentation နှင့် release notes များတွင် Zap ကို fully self-hosted သို့မဟုတ် fully Zap-only ဟု မဖော်ပြရ။

## Ownership rules

Lexer သည် tokenization နှင့် source span ကို ပိုင်ဆိုင်သည်။ Parser သည် syntax နှင့် AST construction ကို ပိုင်ဆိုင်သည်။ Type checker သည် static acceptance/rejection ကို ပိုင်ဆိုင်သည်။ Diagnostics သည် stable code၊ severity၊ location၊ notes နှင့် help ကို ပိုင်ဆိုင်သည်။ Evaluator သို့မဟုတ် VM သည် execution ကို ပိုင်ဆိုင်သည်။ Package layer သည် manifest၊ lockfile၊ resolution၊ hash၊ signature နှင့် offline policy ကို ပိုင်ဆိုင်သည်။ Platform seed သည် operating-system interaction ကို ပိုင်ဆိုင်သည်။ Layer တစ်ခုသည် အခြား layer ၏ contract ကို မည်သည့်အခါမျှ တိတ်တဆိတ် ပြန်မသတ်မှတ်ရ။

Normative rule တစ်ခုချင်းစီတွင် English specification section၊ Burmese counterpart၊ owner နှင့် deterministic fixture အနည်းဆုံးတစ်ခု ရှိရမည်။ Fixture မရှိသော behavior ကို provisional အဖြစ်သာ သတ်မှတ်ပြီး stable ဟု မကြေညာရ။

## Artifact contract

Canonical artifacts များသည် UTF-8 JSON object ဖြစ်ရမည်။ Object key များကို lexicographic order ဖြင့် စီထားရမည်။ Array order သည် deterministic ဖြစ်ရမည်။ Timestamp၊ pointer address၊ source name အဖြစ် အတိအကျ ပေးထားခြင်းမရှိသော host path များ မပါရ။ Schema version ကို explicit ထည့်ရမည်။ Token stream၊ AST snapshot၊ diagnostic၊ typed IR၊ manifest/lockfile၊ test result နှင့် release manifest များသည် လိုအပ်သော artifact families များဖြစ်သည်။

Producer သည် malformed input နှင့် မသိသော artifact schema version ကို ခန့်မှန်းမလုပ်ဘဲ reject လုပ်ရမည်။ Consumer သည် မသိသော required field များကို reject လုပ်ရမည်။ သက်ဆိုင်ရာ artifact family တွင် source location ပါပါက ထို location ကို ထိန်းသိမ်းရမည်။

## Capability contract

Compiler-core operation များသည် default အနေဖြင့် pure ဖြစ်ရမည်။ Source နှင့် fixture ဖတ်ခြင်းသည် bounded ဖြစ်ပြီး explicit input အဖြစ်သာ ပေးရမည်။ Package resolution သည် default အနေဖြင့် offline ဖြစ်ရမည်။ Network သုံးခြင်းသည် explicit user command နှင့် host capability လိုအပ်သည်။ Process execution၊ environment ဖတ်ခြင်း၊ arbitrary file write၊ clock၊ randomness နှင့် socket များသည် compiler ၏ ambient capability မဖြစ်ရ။

Path operation အားလုံးသည် absolute path၊ traversal component၊ symlink escape၊ oversized input နှင့် platform-specific ambiguity များကို သက်ဆိုင်ရာ host boundary contract အတိုင်း reject လုပ်ရမည်။ Resource limit များသည် observable diagnostic behavior ၏ အစိတ်အပိုင်းဖြစ်သည်။

## Reproducibility နှင့် differential gates

Source bytes၊ contract versions၊ compiler inputs နှင့် platform-seed version တူညီလျှင် repeated run များမှ token၊ AST၊ diagnostic၊ typed-IR နှင့် artifact hash များ တူရမည်။ B1 နှင့် နောက်ပိုင်း implementation များကို owned corpus ပေါ်တွင် B0 နှင့် နှိုင်းယှဉ်ရမည်။ Mismatch ဖြစ်ပါက defect သို့မဟုတ် explicit compatibility decision အဖြစ် မှတ်တမ်းတင်ရမည်။ Fixture ကို contract record မရှိဘဲ ပြောင်း၍ ဖုံးကွယ်ခြင်း မပြုရ။

## Version policy

Language version၊ compiler version၊ standard-library version နှင့် artifact schema version တစ်ခုချင်းစီသည် `VERSIONS.toml` တွင် သီးခြား field ဖြစ်သည်။ Language semantic change တစ်ခုအတွက် specification update၊ English/Burmese documentation parity၊ fixtures၊ changelog entry နှင့် explicit compatibility decision လိုအပ်သည်။ Artifact-schema change တစ်ခုအတွက် migration note နှင့် သီးခြား schema-version decision လိုအပ်သည်။

## လက်ရှိအခြေအနေ

Zap v2.10.1 သည် **B0** ဖြစ်သည်။ လက်ရှိ native Rust lexer၊ parser၊ evaluator၊ standard library၊ registry နှင့် host boundary များသည် reference owner များဖြစ်သည်။ Bootstrap directory များသည် B1 အတွက် လိုအပ်သော contract နှင့် corpus ကို စတင်တည်ဆောက်ပေးခြင်းသာဖြစ်ပြီး native implementation ကို အစားမထိုးသေးပါ။
