# Zap v2.1.12 Release Notes

**ထုတ်ဝေသည့်ရက်:** 2026-08-22

## Release summary

Zap v2.1.12 တွင် normal source program နှင့် local module များအတွက် canonical AST execution ကို normative အဖြစ် သတ်မှတ်ထားပါသည်။ လက်ရှိ language behavior ကို ထိန်းသိမ်းထားသော်လည်း normal-program fallback အဖြစ် legacy line interpreter သို့ ကျသွားခြင်းကို ဖယ်ရှားပြီး line execution ကို explicit compatibility boundary အတွက်သာ ထားရှိပါသည်။

## Highlights

AST parser သည် exported binding နှင့် function များကို `Stmt::Declaration` နှင့် `Stmt::Function` node များအတွင်း တိုက်ရိုက်ကိုယ်စားပြုပါသည်။ Local module file များကို main program နှင့်တူညီသော AST executor ဖြင့် parse/execute လုပ်ပြီး explicit import အတွက် export marker များကို ထိန်းသိမ်းထားပါသည်။ ရှိပြီးသား fixture များအသုံးပြုသော `?` Result/Option propagation syntax၊ prefix `not` expression နှင့် empty-class declaration များကိုလည်း parser က ကိုင်တွယ်ပါသည်။

Normal execution path ကို ရွေးချယ်ရာတွင် parsed AST မှ source line များကို ပြန်လည်တည်ဆောက်ခြင်း မလုပ်တော့ပါ။ `ast_program_compatible` သည် `parse_program` မှ လက်ရှိထုတ်ပေးသော syntax အားလုံးကို လွှမ်းခြုံသော explicit compatibility predicate ဖြစ်ပါသည်။ Legacy line interpreter ကို `ast_body: Program` မပါဘဲ `body: Vec<String>` ပါသော older/internal `Function` record များအတွက်သာ ဆက်လက်ထားရှိပါသည်။ ထို path ထဲသို့ syntax အသစ် မထည့်သင့်ပါ။

ဤ milestone တွင် AST parser၊ export၊ canonical module-import၊ syntax-failure၊ `?` propagation၊ boolean-prefix၊ empty-class နှင့် inherited-field-visibility regressions များ ထည့်သွင်းထားပါသည်။ English/Burmese AST foundation status၊ roadmap၊ README နှင့် documentation-consistency coverage များကိုလည်း တစ်ပြိုင်နက် update လုပ်ထားပါသည်။

## Compatibility and deferred scope

Normal source program တစ်ခု AST parse မအောင်မြင်ပါက legacy line path ဖြင့် interpret မလုပ်တော့ဘဲ syntax diagnostic ပြန်ပေးပါသည်။ ထို့ကြောင့် parser ပိုင် source များအတွက် AST parser/evaluator boundary သည် normative ဖြစ်လာပါသည်။ Parser က လက်ခံသော ရှိပြီးသား source behavior များဖြစ်သည့် Result/Option propagation၊ OOP visibility နှင့် local module export များကို ဆက်လက်ထိန်းသိမ်းထားပါသည်။

Legacy line executor ကို pre-AST သို့မဟုတ် test-created line-bodied function များအတွက် compatibility-only အဖြစ် ထားရှိပါသည်။ ထို representation ကို ဖယ်ရှားရန် legacy fixture နှင့် migration guidance များကို ပြန်လည်သုံးသပ်ပြီး သီးခြား documented breaking compatibility decision လိုအပ်ပါသည်။ ဤ release တွင် first-class callable value၊ parent-linked environment frame၊ cumulative memory budget၊ broad async syntax သို့မဟုတ် traits/interfaces semantics အသစ်များ မပါဝင်ပါ။

## Verification

ဤ milestone သည် Rust 1.75.0 formatting၊ `-D warnings` ပါသော strict Clippy၊ full native all-target/all-feature suite၊ 82 checks ပါသော documentation consistency validation၊ ၎င်း၏ regression harness နှင့် `git diff --check` များကို pass ဖြစ်ပါသည်။ Native integration suite တွင် test 254 ခု pass ဖြစ်ပြီး focused AST/export/module regressions များလည်း pass ဖြစ်ပါသည်။

## Upgrade guidance

မိမိ operating system နှင့် architecture ကိုက်ညီသော archive ကို [v2.1.12 GitHub Release](https://github.com/hidecard/zap/releases/tag/v2.1.12) မှ download လုပ်၍ upgrade လုပ်နိုင်ပါသည်။ Install မလုပ်မီ published checksum နှင့် signature ကို verify လုပ်ပါ။ Parser ပိုင် program များအတွက် source-language migration မလိုအပ်ပါ။ သို့သော် undocumented legacy line-interpreter fallback ကို မှီခိုထားသော code များကို compatibility-sensitive အဖြစ် ပြန်လည်သုံးသပ်သင့်ပါသည်။

## Documentation

[English AST foundation status](P0_FOUNDATION_STATUS_EN.md)၊ [Burmese AST foundation status](P0_FOUNDATION_STATUS_MM.md)၊ [English runtime-state contract](RUNTIME_STATE_EN.md)၊ [Burmese runtime-state contract](RUNTIME_STATE_MM.md)၊ [English documentation navigation](DOCUMENTATION_NAVIGATION_EN.md) နှင့် [Burmese documentation navigation](DOCUMENTATION_NAVIGATION_MM.md) ကို ဖတ်ရှုနိုင်ပါသည်။ ကျန်ရှိသော memory၊ async၊ conformance၊ specification၊ tooling၊ benchmark၊ registry-edge-case နှင့် traits work များကို bilingual TODO register နှင့် next-step plan များတွင် မှတ်တမ်းတင်ထားပါသည်။
