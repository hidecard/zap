# Zap — ကျန်ရှိနေသေးသော အလုပ်များနှင့် Git update status

## အကျဉ်းချုပ်

လက်ရှိ working tree တွင် B1 parser၊ B2 type inference/generic checking နှင့် B3 typed-IR/lowering အပိုင်းများအတွက် verified progress အများအပြား ရှိနေပါသည်။ သို့သော် ယခင်သတ်မှတ်ထားသော requirement အတိုင်း **full language ownership၊ full Rust-reference parity နှင့် complete runtime coverage မပြီးသေးပါ**။ ထို့ကြောင့် အောက်ပါ remaining work များ မပြီးမချင်း final completion commit/push မပြုလုပ်သင့်ပါ။

## ကျန်ရှိနေသေးသော အလုပ်များ

| ဦးစားပေး | အပိုင်း | လက်ရှိအခြေအနေ | ကျန်ရှိသည့်အလုပ် |
|---|---|---|---|
| P0 | B2 generic constraints | Basic parameter/bound membership၊ arity နှင့် concrete argument checking pass | Multiple type parameters အားလုံးအတွက် inference၊ nested/compound bounds၊ explicit generic call syntax နှင့် reference diagnostic parity ပြီးစီးရန် |
| P0 | B2 alias checking | Nested list/option/result/map alias body validation နှင့် undeclared parameter rejection ထည့်ပြီး၊ existing gates pass | Alias environment၊ alias-of-alias၊ imported aliases၊ generic class/alias၊ recursive aliases နှင့် alias expansion diagnostics ပြီးစီးရန် |
| P0 | B2 dataflow | Branch merge၊ try/catch merge နှင့် limited loop transfer ရှိ | Short-circuit path sensitivity၊ mutation/alias invalidation၊ loop fixpoint၊ break/continue၊ return/break live-path merge နှင့် full reference parity ပြီးစီးရန် |
| P1 | B3 canonical AST bridge | Canonical `target/member` နှင့် structured map fields align; `lower_ast_program` bridge စတင်ထား | All canonical AST expression/statement kinds၊ name/storage semantics၊ function/class/module၊ for/try-catch၊ map/index runtime နှင့် complete opcode coverage align လုပ်ရန် |
| P1 | B3 typed-IR producer | Member/map expression fields တချို့ canonicalized; existing typed-IR gates pass | Source-string shape routing အားလုံးဖယ်ရှားပြီး parser AST ကို တိုက်ရိုက် consume သည့် production emitter ပြီးစီးရန် |
| P1 | B3 VM/runtime | Existing arithmetic/control/call subset pass | Variable load/store၊ member/index mutation၊ calls of arbitrary arity၊ closures/functions/classes နှင့် error semantics ပြီးစီးရန် |
| P1 | B1 broader differential corpus | Existing valid corpus 29/29 နှင့် known diagnostics 10/10 exact pass | Rust reference နှင့် arbitrary valid/invalid corpus တိုးချဲ့ခြင်း၊ all AST node spans၊ malformed recovery၊ multi-diagnostic ordering နှင့် source-name parity ပြီးစီးရန် |
| P2 | B1 parser cleanup | Production `parse()` တွင် fixture routing မကျန်; legacy helper implementations အချို့ file ထဲရှိ | Unused fixed-shape helper paths ဖယ်ရှား/သီးခြား transition module သို့ရွှေ့၊ parser comments/contracts နှင့် full API documentation ပြီးစီးရန် |
| P2 | Regression/CI | Local B1/B2/B3/native tests pass | New differential scripts များကို canonical CI gate အဖြစ် ချိတ်ဆက်ပြီး clean-clone/reproducible execution စစ်ဆေးရန် |

## လက်ရှိ verified evidence

- B1 valid AST/span differential: **29/29 exact pass**။
- B1 known invalid diagnostics differential: **10/10 exact pass**။
- B2 verifier scripts: **33 scripts pass**။
- B3 canonical schema gate နှင့် typed-IR/bytecode gate: pass။
- Native tests: **272 unit/all-target tests pass** နှင့် **259 integration tests pass**။
- `git diff --check`: pass။

## Git update status

လက်ရှိ branch သည် `master` ဖြစ်ပြီး origin ကို fetch/rebase ပြုလုပ်ပြီးနောက် local `master` သည် `origin/master` commit `2de01a5` နှင့် up-to-date ဖြစ်နေပါသည်။ Remote တွင် ဝင်လာသော mutable-loop နှင့် short-circuit loop-control changes များကို local working tree နှင့် ပေါင်းစပ်ထားပါသည်။ Working tree တွင် parser/typecheck/typed-IR/B3 source changes နှင့် new verifier/status files များ uncommitted အဖြစ် ရှိနေပါသည်။

ယခင်သတ်မှတ်ထားသော “အလုပ်အားလုံးပြီးမှ commit/push” requirement နှင့် လက်ရှိ remaining work များကြောင့် **ယခုအချိန်တွင် final commit/push မပြုလုပ်ရသေးပါ**။ Git update အနေဖြင့် origin ကို fetch/rebase လုပ်ထားပြီး status report နှင့် remaining-work report ကို local working tree ထဲသို့ update လုပ်ထားပါသည်။ Full ownership ပြီးဆုံးသည့်အခါမှ scoped commit တစ်ခုဖန်တီးကာ `master` သို့ push လုပ်သင့်ပါသည်။

## အကြံပြုလုပ်ဆောင်ရမည့် အစီအစဉ်

ပထမဦးစွာ B2 alias environment နှင့် generic constraint inference ကို parser AST အပေါ်တွင် တိုက်ရိုက်ချိတ်ဆက်ရမည်။ ထို့နောက် B3 typed-IR producer နှင့် VM တွင် canonical AST schema အားလုံးကို support လုပ်ရမည်။ ထိုအပြီး Rust-reference differential corpus ကို valid/invalid နှစ်မျိုးစလုံး တိုးချဲ့ပြီး full regression နှင့် clean-clone verification ပြုလုပ်ရမည်။ အားလုံး pass ဖြစ်မှသာ commit နှင့် GitHub push ပြုလုပ်ရမည်။
