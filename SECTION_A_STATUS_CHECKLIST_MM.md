# Zap Section A — လက်ရှိ Status Checklist

**နောက်ဆုံးစစ်ဆေးမှု:** `master` / `d52e214`  
**Contract status:** B0 — native Rust reference owner; `self_hosted = false`  
**အဓိပ္ပါယ်:** `[x]` သည် bounded/provisional gate သို့မဟုတ် foundation evidence ပြီးစီးခြင်းကို ဆိုလိုသည်။ `[ ]` သည် checklist တွင် သတ်မှတ်ထားသည့် full/general/ownership acceptance မပြီးသေးခြင်းကို ဆိုလိုသည်။

## Compiler နှင့် Self-hosting

- [ ] **Complete type inference** — လက်ရှိသည် selected declarations၊ functions၊ conditions၊ calls၊ collections နှင့် literals အတွက် bounded candidate slice ဖြစ်သည်။
- [ ] **Broader basic-type inference** — number/text/bool/list/map/none တို့အတွက် အချို့ direct fixtures ရှိသော်လည်း real-world arbitrary inference မပြီးသေးပါ။
- [ ] **Generic declarations** — `identity<T>` နှင့် wrapper substitution bounded evidence ရှိသော်လည်း generic function/type/container အပြည့်အဝ မပြီးသေးပါ။
- [ ] **Complete collection inference** — direct list/map နှင့် selected nested indexing ရှိသော်လည်း arbitrary nested collection inference မပြီးသေးပါ။
- [ ] **Nested map/deeper nested inference** — reference-only/bounded evidence ရှိသော်လည်း candidate general support မပြီးသေးပါ။
- [ ] **Compound type guards** — direct `is_some`/`is_option_none` bounded narrowing ရှိသော်လည်း arbitrary compound/nested guards မပြီးသေးပါ။
- [ ] **Loop mutation analysis** — bounded loop narrowing/restoration ရှိသော်လည်း general mutation analysis မပြီးသေးပါ။
- [x] **Reassignment invalidation — bounded candidate slice** — stale narrowing invalidation ကို candidate တွင် ထည့်ထားသည်။
- [ ] **Arbitrary-program parser coverage** — functions၊ loops၊ classes၊ nested calls၊ parenthesized expressions နှင့် selected nested blocks fixtures ရှိသော်လည်း parser သည် line-count/fixture dispatch ကို ဆက်သုံးနေသေးသည်။
- [ ] **Full diagnostic parity** — delimiter/function edge diagnostics အချို့ရှိသော်လည်း error kind/message/position/failure behavior အားလုံး မညီသေးပါ။
- [ ] **General typed-IR production** — annotated declarations၊ generic identity နှင့် primitive literals အတွက် bounded candidate ရှိသော်လည်း arbitrary program typed-IR မထုတ်နိုင်သေးပါ။
- [ ] **Zap-owned package/build pipeline** — B3 manifest/lock/offline build foundation သည် native Rust-owned ဖြစ်နေသေးသည်။
- [ ] **Zap-owned VM execution** — native VM smoke foundation ရှိသော်လည်း bootstrap-side VM ownership မပြီးသေးပါ။
- [ ] **Platform-seed acceptance** — deny-by-default platform boundary verification ရှိသော်လည်း bootstrap output ကို platform seed မှ self-build/run လုပ်နိုင်ကြောင်း မပြနိုင်သေးပါ။
- [ ] **Full B4 self-hosting** — မရသေးပါ။

## ပြီးစီး/အထောက်အထားရှိသော parser နှင့် foundation gates

- [x] B1 lexer candidate differential corpus — provisional.
- [x] Function/loop/class AST fixtures — provisional reference parity.
- [x] Nested/multi-argument call fixtures — three positional argumentsအထိ bounded support.
- [x] Parenthesized/nested expression fixtures — bounded precedence/span support.
- [x] Nested `for/if` နှင့် `while/if/else/break/continue` fixtures — bounded reference parity.
- [x] Invalid indentation diagnostic — two-space malformed indentation is rejected with reference-matching diagnostic.
- [x] Mixed top-level simple statement sequence — declarations and `say` statements use the append-backed flat sequence path.
- [x] Nested class method body — bounded class member function and return body reference parity.
- [x] Simple while block — `while ready:` with `break` body reference parity.
- [x] Deep mixed nested block — `for/if/while/continue` recursive AST reference parity.
- [x] Four-argument call — nested/top-level comma splitting supports four positional arguments.
- [x] Parenthesized unary `not` — precedence and grouped operand AST parity.
- [x] Nested assignment block — `while` body declaration plus reassignment AST parity.
- [x] Mixed recursive top-level sequence — nested call, `say`, and parenthesized declaration in one flat sequence.
- [x] Append-backed flat declaration sequence — native `append(list, value)` builtin နှင့် bounded parser path ထည့်ထားသည်။
- [x] Boolean expression inference slice — unary `not`, comparison matrix, and logical `and/or` expressions infer as `bool` in the candidate.
- [x] B0/B1/B2/B3/VM regression scripts — နောက်ဆုံး consolidated run တွင် pass.

## လက်ရှိနောက်တစ်ဆင့်

1. Token span line/column များမှ indentation stack တည်ဆောက်ရန်။
2. Recursive `parse_block(indent)` နှင့် statement-list builder ကို line-count dispatch မပါဘဲ ပြောင်းရန်။
3. Token-derived indentation stack နှင့် arbitrary-depth block diagnostics ကို broaden လုပ်ရန်။
4. `while ... else` ကို language reference syntax အဖြစ် မပံ့ပိုးသေးကြောင်း သတ်မှတ်ပြီး valid loop-block variants ကို ဆက်စစ်ရန်။
5. Mixed top-level statements၊ arbitrary-depth nested blocks နှင့် invalid indentation diagnostics fixtures ထည့်ရန်။
6. ထို parser AST ကို general typed-IR နှင့် type-checker pipeline သို့ ဆက်စပ်ရန်။
7. A gates အားလုံး pass ပြီးမှသာ B section ကို စတင်ရန်။

## B အပိုင်း

B section ကို မစတင်သေးပါ။ Section A ၏ full type inference၊ arbitrary parser၊ diagnostic parity၊ general typed-IR၊ package/build ownership၊ VM ownership၊ platform-seed acceptance နှင့် B4 self-rebuild များ မပြီးမချင်း B ကို intentionally hold ထားသည်။
