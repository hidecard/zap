# Zap Section A — လက်ရှိ Status Checklist

**နောက်ဆုံးစစ်ဆေးမှု:** `master` / verified follow-up changes through cursor-driven parser foundation

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

- [x] **AST expression-to-type bridge — bounded foundation** — literal၊ unary၊ binary၊ list နှင့် empty-list AST node များအတွက် node-kind-based inference gate pass; environment-aware arbitrary expression inference မပြီးသေးပါ။

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

- [x] Immutable line-cursor foundation — parser sequence traversal သည် cursor state၊ peek၊ advance နှင့် EOF checks ကို အသုံးပြုသည်။

- [x] Structural indentation foundation — indentation width၊ one-level transition validation နှင့် line-aware unexpected indentation diagnostic ထည့်ထားသည်။

- [x] Unexpected multi-level indentation fixture — one-level jump violation ကို line-aware diagnostic ဖြင့် differential corpus ထဲတွင် စစ်ဆေးထားသည်။

- [x] History-backed indentation stack — stack level history နှင့် depth pointer ဖြင့် valid prior-level dedent၊ inconsistent dedent နှင့် one-level jump ကို စစ်ဆေးနိုင်သည်။

- [x] Recursive block parser foundation — cursor-based `parse_block_from` သည် nested `if`/`for`/`while` body နှင့် sibling statements များကို arbitrary-depth bounded cases အတွက် စုစည်းနိုင်သည်။

- [x] `if/else` body ownership — same-level `else:` နှင့် indented body ကို recursive block result အဖြစ် attach လုပ်နိုင်သည်။

- [x] `elif/else` chain lowering — bounded chain ကို nested `if` representation သို့ ပြောင်းပြီး final `else` body ကို ထိန်းသိမ်းနိုင်သည်။

- [x] Missing-block diagnostics — `if`/`elif`/`else` header များတွင် indented body မရှိပါက stable syntax diagnostic ပြနိုင်သည်။

- [x] Generic loop-control statements — recursive block path သည် `break` နှင့် `continue` ကို statement AST node များအဖြစ် ထုတ်ပေးနိုင်သည်။

- [x] Generic top-level control-flow route — bounded `for`/`while` programs များကို recursive block parser သို့ ချိတ်ဆက်ထားသည်။

- [x] Boolean expression inference slice — unary `not`, comparison matrix, and logical `and/or` expressions infer as `bool` in the candidate.

- [x] Arithmetic expression inference slice — parenthesized expressions, unary negation, subtraction, division, and remainder infer numeric types for compatible operands.

- [x] Compound `is_option_none(...) and ...` guard — bounded true-branch narrowing to `none` with existing else-branch restoration.

- [x] Flat sequence final-line handling — sentinel normalization preserves the last statement when source has no trailing newline.

- [x] B0/B1/B2/VM regression scripts — နောက်ဆုံး consolidated run တွင် pass.

- [x] B1 token cursor foundation verifier — immutable cursor နှင့် indentation relation matrix automated gate pass.

- [x] B2 AST expression bridge verifier — parser-shaped expression nodes ၆ မျိုးအတွက် deterministic type outputs pass.

- [x] B1 unexpected indentation differential — multi-level indentation jump fixture နှင့် reference diagnostic pass.

- [x] B1 recursive block ၁၀-case verifier — nested branch/loop ownership၊ sibling dedent နှင့် generic `if` entrypoint cases pass.

- [x] B1 branch-chain ၁၀-case verifier — `if/elif/else` ownership၊ valid/inconsistent dedent နှင့် missing-body diagnostics pass.

- [x] B1 generic control-flow ၁၀-case verifier — recursive `for`/`while`/`if`၊ `break`/`continue` နှင့် indentation diagnostics pass.

## လက်ရှိနောက်တစ်ဆင့်

1. Token span line/column များမှ parser-owned full indentation stack တည်ဆောက်ရန်။ History/depth foundation ပြီးသော်လည်း token-native ownership နှင့် all grammar interaction မပြီးသေးပါ။

1. Recursive `parse_block(indent)` နှင့် statement-list builder ကို remaining function/class/control-flow grammar အားလုံးအတွက် line-count dispatch မပါဘဲ ပြောင်းရန်။ `if` route သည် bounded generic path အဖြစ် စတင်ပြီးဖြစ်သည်။

1. Token-derived indentation stack နှင့် arbitrary-depth block diagnostics ကို broaden လုပ်ရန်။

1. `while ... else` ကို language reference syntax အဖြစ် မပံ့ပိုးသေးကြောင်း သတ်မှတ်ပြီး valid loop-block variants ကို ဆက်စစ်ရန်။

1. Mixed top-level statements၊ arbitrary-depth nested blocks နှင့် invalid indentation diagnostics fixtures ထည့်ရန်။

1. ထို parser AST ကို general typed-IR နှင့် type-checker pipeline သို့ ဆက်စပ်ရန်။

1. A gates အားလုံး pass ပြီးမှသာ B section ကို စတင်ရန်။

## B အပိုင်း

B section ကို မစတင်သေးပါ။ Section A ၏ full type inference၊ arbitrary parser၊ diagnostic parity၊ general typed-IR၊ package/build ownership၊ VM ownership၊ platform-seed acceptance နှင့် B4 self-rebuild များ မပြီးမချင်း B ကို intentionally hold ထားသည်။
