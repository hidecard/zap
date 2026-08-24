# Zap လက်ရှိအခြေအနေ

**အခြေအနေ label:** active
**နောက်ဆုံး publish လုပ်ထားသော release:** [v2.11.9](https://github.com/hidecard/zap/releases/tag/v2.11.9)
**နောက် release line:** v2.11.10 preparation
**Bootstrap stage:** B0

> Zap သည် Rust reference/native implementation ဖြစ်သည်။ `bootstrap/` အောက်ရှိ Zap lexer၊ parser၊ type-checker နှင့် typed-IR အလုပ်များသည် provisional၊ corpus-limited evidence သာဖြစ်ပြီး fully Zap-only သို့မဟုတ် self-hosted compiler ဖြစ်ကြောင်း မသက်သေပြပါ။

## Release နှင့် provenance

နောက်ဆုံး publish လုပ်ထားသော release သည် v2.11.9 ဖြစ်သည်။ ၎င်း၏ tag နှင့် workflow record များသည် immutable release evidence ဖြစ်ပြီး နောက် release သည် tag အသစ်ကိုသာ အသုံးပြုရမည်၊ ယခင် tag များကို rewrite မလုပ်ရ။ Publish လုပ်ထားသော release တစ်ခုစီတွင် versioned manifest၊ aggregate checksum၊ detached signature နှင့် signed provenance asset ပါဝင်သည်။

Versioned provenance asset သည် release identity အတွက် canonical machine-readable record ဖြစ်သည်။ ၎င်းတွင် source URI၊ tag/ref၊ source commit၊ workflow run ID၊ reproducible manifest နှင့် checksum၊ signing metadata နှင့် SHA-256 digest/size ပါသော artifact subjects များကို မှတ်တမ်းတင်သည်။ Release verifier သည် downloaded archive နှင့် signature များနှင့်အတူ ဤ record ကို စစ်ဆေးသည်။

## လက်ရှိ implementation အခြေအနေ

| နယ်ပယ် | အခြေအနေ label | လက်ရှိ boundary |
|---|---|---|
| Native compiler/runtime | active | Complete semantics နှင့် supported release target များအတွက် Rust သည် reference owner ဖြစ်နေဆဲ။ |
| B0 artifacts | completed | Canonical token၊ AST၊ diagnostic၊ metadata၊ VM နှင့် platform-seed fixture များ reproducible ဖြစ်သည်။ |
| B1 lexer/parser candidates | provisional | Candidate output သည် owned corpus အပေါ်သာ စစ်ဆေးထားပြီး Rust pipeline ကို မအစားထိုးပါ။ |
| B2 type-checker candidate | provisional | Selected declaration၊ conditional၊ function၊ call၊ bounded list-element diagnostic၊ paired nested-list index slice၊ bounded text-key map-element slice နှင့် bounded direct-`is_some` branch-local option-narrowing slice များ ပါဝင်သည်။ |
| Typed-IR candidate | provisional | ရှိပြီးသား annotated declaration slice တစ်ခုတည်းကိုသာ cover လုပ်သည်။ |
| Malformed-source safety | regression-gated | Invalid-source corpus အသေးတစ်ခုသည် panic သို့မဟုတ် unchecked-unwrap signature မပါဘဲ nonzero ဖြင့် fail ရမည်။ ဤသည်မှာ safety regression gate ဖြစ်ပြီး compiler ownership evidence မဟုတ်ပါ။ |
| B3 package/build foundations | reference-only | Offline/deterministic foundation check များသည် compiler ownership ကို Zap သို့ မလွှဲပြောင်းပါ။ |
| B4 self-hosting | deferred | Self-rebuild acceptance မအောင်မြင်သေးသရွေ့ B4 claim မပြုရ။ |

## နောက် bounded work

v2.11.8 release တွင် text literal ဖြင့် index လုပ်သော bounded `map<text,number>` element နှင့် paired incompatible assignment ကို B2 evidence corpus ထဲသို့ ထည့်ထားသည်။ v2.11.9 release တွင် indented `if` body တစ်ခုအတွင်း tracked `option<number>` ကို direct `is_some` guard ဖြင့် narrowing လုပ်သည့် bounded case နှင့် paired incompatible payload assignment ကို ထည့်ထားသည်။ နောက် roadmap အလုပ်သည် ဤ bounded branch slice ၏ အပြင်ဘက်ကို သီးခြား fixture evidence များဖြင့် B2 inference နှင့် diagnostic coverage ဆက်လက်ချဲ့ထွင်ရန် ဖြစ်သည်။ Malformed-source no-panic behavior ကို regression-gated အဖြစ် ဆက်လက်ထိန်းသိမ်းမည်ဖြစ်ပြီး candidate typed-IR ထုတ်လုပ်မှုကိုလည်း ထို owned analysis တစ်ခုတည်းမှသာ တိုးချဲ့မည်။ Generic declaration၊ nested map၊ deeper nested inference၊ compound guard၊ loop narrowing၊ reassignment invalidation၊ arbitrary program parsing၊ package/build ownership၊ VM ownership နှင့် platform-seed self-hosting များသည် သက်ဆိုင်ရာ acceptance criteria များ မပြည့်မချင်း deferred အဖြစ် ဆက်ရှိသည်။

## Developer environment

Local validation မစတင်မီ `make doctor` ကို run ပါ။ ၎င်းသည် Cargo၊ Rust၊ rustup၊ Python၊ cargo-audit၊ pinned toolchain၊ host target နှင့် သတ်မှတ်ထားသော `ZAP_BIN` သို့မဟုတ် built runtime ကို သီးခြားစီ report လုပ်ပေးသည်။ Normal mode တွင် environment မပြည့်စုံပါက report သာလုပ်ပြီး tests fail ဖြစ်သည်ဟု မဆိုပါ။ Prerequisite အားလုံးမရှိလျှင် `bash scripts/doctor.sh --strict` ကို အသုံးပြုနိုင်သည်။

## Status policy

ဤစာမျက်နှာသည် current-status index ဖြစ်သည်။ Historical release notes နှင့် changelog များသည် immutable record များဖြစ်ပြီး လက်ရှိ implementation claim အဖြစ် မဖတ်ရ။ Behavior change တစ်ခုစီတွင် သက်ဆိုင်ရာ English/Burmese contract၊ fixture၊ ownership record၊ validation gate နှင့် release documentation များကို တစ်ပြိုင်တည်း update လုပ်ရမည်။
