# Zap Rust မမှီခိုသော ကိုယ်ပိုင် Language ဖြစ်လာရန် အသေးစိတ် TODO

ဤစာတမ်းသည် Zap ကို လက်ရှိ Rust reference implementation အပေါ် မှီခိုနေသည့် B0 အခြေအနေမှ **Zap source ဖြင့် မိမိ compiler/runtime/build system ကို ပြန်တည်ဆောက်နိုင်သည့် B4 self-hosted language** အဖြစ် ပြောင်းလဲရန် အစမှအဆုံး လုပ်ဆောင်ရမည့်အရာများကို အစဉ်လိုက် သတ်မှတ်ထားသည်။

ဤ roadmap ၏ ရည်ရွယ်ချက်မှာ Rust ကို repository ထဲမှ ချက်ချင်းဖျက်ရန် မဟုတ်ပါ။ ပထမအဆင့်များတွင် Rust implementation ကို reference oracle နှင့် bootstrap seed အဖြစ် အသုံးပြုနိုင်သည်။ သို့သော် B4 ပြည့်မြောက်သောအခါ user-facing compiler/build path တွင် Rust နှင့် Cargo မလိုတော့ရပါ။ လက်ရှိ B0 boundary နှင့် stage definitions ကို [B0 Baseline](../bootstrap/BASELINE_B0.md) နှင့် [Bootstrap Contract](../bootstrap/contracts/BOOTSTRAP_CONTRACT_MM.md) တို့တွင် အခြေခံထားသည်။

## လက်ရှိ master update — TODO status snapshot

ဤ snapshot သည် လက်ရှိ GitHub `master` ၏ update များကို စစ်ဆေးပြီး ပြန်လည်သတ်မှတ်ထားခြင်းဖြစ်သည်။ နောက်ဆုံး update များတွင် B1 lexer/parser verification gate များ၊ B2 type-check/typed-IR verifier များ၊ native-binary portability, clean-repo-root check နှင့် bootstrap smoke checks များကို တိုးချဲ့ထားသည်။ သို့သော် bootstrap contract သည် Zap ကို **B0** ဟု ဆက်လက်သတ်မှတ်ထားပြီး Rust native implementation သည် lexer, parser, evaluator, standard library, registry နှင့် host boundary များ၏ reference owner ဖြစ်နေသေးသည်။ [1] [2]

| Area | Latest evidence | TODO status |
|---|---|---|
| B1 lexer | Lexer contract, CR handling, token/indentation checks နှင့် batch verification scripts ရှိပြီး အချို့ candidate parity gates ပြီးစီးထားသည် | **Partial / provisional** — arbitrary full-language canonical ownership မရသေး |
| B1 parser | Boundary fixtures, general parser gates, control-flow/block checks, parser portability fixes နှင့် aggregate gate ရှိသည် | **Partial / provisional** — Rust reference AST/diagnostic owner မပြောင်းသေး |
| B2 type checker | Generic constraints, compound bounds, aliases, flow/dataflow, recursive alias diagnostics နှင့် verifier များ တိုးချဲ့ထားသည် | **Partial / provisional** — complete language-wide type ownership မရသေး |
| B2 typed IR | Arbitrary typed-IR, expression, generic, trait နှင့် reference-compare verification scripts တိုးလာသည် | **Partial / provisional** — Zap compiler က full source ကို Rust မခေါ်ဘဲ typed IR ထုတ်နိုင်ကြောင်း B4-level proof မရှိသေး |
| B3 build/package/VM | Build plan, dependency graph, package metadata နှင့် VM candidate files/fixtures ရှိသည် | **Not certified** — canonical executable ownership နှင့် full runtime replacement မပြီးသေး |
| B4 self-rebuild | B4 acceptance manifest, byte determinism, second-stage rebuild, clean-environment gate scripts ရှိသည် | **Not certified** — acceptance rows များသည် provisional ဖြစ်ပြီး contract status သည် `not-certified` |
| Rust independence | Non-Rust seed/smoke tooling နှင့် portability checks တိုးတက်လာသည် | **Not achieved** — normal compiler/runtime/build path တွင် Rust/Cargo dependency boundary မဖယ်ရှားသေး |
| CI | Latest master CI run အောင်မြင်ထားသည် | CI green သည် B4 self-hosting အောင်မြင်သည်ဟု မဆိုလို |

### ဤ update ပြီးနောက် အဓိကကျန် TODO

- [ ] B1 lexer/parser candidate gate များကို **canonical Zap-owned front end** အဖြစ် promote လုပ်ပြီး Rust fallback မရှိကြောင်း prove လုပ်ပါ။
- [ ] B2 type checker/typed IR verifier များကို full supported language surface အထိ ချဲ့ပြီး `.zp → typed IR` ကို Zap-only path ဖြင့် run ပါ။
- [ ] B3 lowerer, bytecode, VM, package/build နှင့် runtime ကို sample fixture မဟုတ်ဘဲ compiler source အပြည့်အဝအတွက် run နိုင်စေပါ။
- [ ] B4 acceptance rows အားလုံးကို supported targets သုံးမျိုးတွင် pass လုပ်ပြီး `not-certified` ကို `certified` သို့ evidence ဖြင့်သာ ပြောင်းပါ။
- [ ] Clean environment တွင် Rust/Cargo မရှိဘဲ compiler source ကို self-rebuild လုပ်ပြီး deterministic artifact နှစ်ဆင့် comparison ထုတ်ပါ။
- [ ] Rust implementation ကို reference-only boundary သို့ ရွှေ့ပြီး normal `zap build`, `zap test`, `zap run`, `zap compiler rebuild` path မှ Rust dependency ဖယ်ရှားပါ။

## 0။ အောင်မြင်မှုအဓိပ္ပာယ်နှင့် မပြောင်းလဲရမည့်စည်းမျဉ်း

### 0.1 Final target

B4 ပြည့်မြောက်သည်ဟု သတ်မှတ်ရန် အောက်ပါ command မျိုးသည် Rust/Cargo မသုံးဘဲ အလုပ်လုပ်ရမည်။

```text
zap build
zap test
zap run main.zp
zap package
zap compiler rebuild
```

Compiler source, parser, type checker, IR, lowerer, VM/runtime boundary, package/build logic နှင့် standard-library contract တို့၏ semantic owner သည် Zap ဖြစ်ရမည်။ Rust implementation သည် compatibility oracle, historical reference သို့မဟုတ် optional development tool အဖြစ်သာ ကျန်ရမည်။

### 0.2 မဖြစ်မနေလိုက်နာရမည့် စည်းမျဉ်းများ

| စည်းမျဉ်း | လိုက်နာရန် |
|---|---|
| Semantic single owner | Rule တစ်ခုကို Rust နှင့် Zap နှစ်နေရာက သီးခြားဆုံးဖြတ်ခွင့် မရှိရ။ Canonical language specification က အဆုံးအဖြတ်ပေးရမည်။ |
| Differential verification | B1–B3 အတွင်း Zap output ကို Rust reference နှင့် fixture အလိုက် နှိုင်းယှဉ်ရမည်။ |
| No hidden fallback | Zap compiler မအောင်မြင်သည့်အခါ Rust evaluator/compiler ကို အလိုအလျောက် မခေါ်ရ။ Fallback ရှိပါက mode ကို အတိအလင်းပြရမည်။ |
| Deterministic output | Token, AST, typed IR, bytecode, diagnostic, package lock နှင့် rebuild artifact များကို deterministic ဖြစ်စေရမည်။ |
| Bounded runtime | Memory, recursion, task, I/O, process, network နှင့် output limits များကို specification နှင့် tests ထဲတွင် သတ်မှတ်ရမည်။ |
| Cross-platform contract | Linux, Windows, macOS တို့တွင် semantic behavior တူပြီး OS-specific limitation များကို သီးခြားမှတ်တမ်းတင်ရမည်။ |

## 1။ Phase 0 — Scope, specification နှင့် ownership ကို freeze လုပ်ခြင်း

ဤအဆင့်မပြီးမချင်း compiler implementation ကို အကြီးစားတိုးချဲ့ခြင်း မလုပ်ရ။ Specification မတည်ငြိမ်လျှင် Rust နှင့် Zap parity ကို တိုင်းတာ၍ မရနိုင်ပါ။

### 1.1 Canonical language specification

- [ ] Syntax grammar ကို lexer, parser, statement, expression, function, class, module, generic, async နှင့် error syntax အလိုက် စုစည်းပါ။
- [ ] Operator precedence, associativity, indentation, newline, comment, Unicode identifier နှင့် literal rules များကို တစ်နေရာတည်းတွင် သတ်မှတ်ပါ။
- [ ] Value/type model ကို `none`, boolean, number, text, list, map, function, object, option/result နှင့် module အလိုက် သတ်မှတ်ပါ။
- [ ] Mutation, closure capture, scope, shadowing, return/break/continue, exception propagation နှင့် cleanup semantics များကို သတ်မှတ်ပါ။
- [ ] Module search path, import alias, cycle detection, package manifest, lockfile နှင့် registry behavior များကို normative rule အဖြစ် သတ်မှတ်ပါ။
- [ ] Async scheduling, poll budget, timeout, cancellation, join နှင့် blocking boundary များကို broad `async` syntax မတိုးမီ သတ်မှတ်ပါ။
- [ ] Rule တစ်ခုချင်းစီအတွက် `rule_id`, specification section, fixture path, status နှင့် owner ပါသော machine-readable index ပြုလုပ်ပါ။

**ပြီးမြောက်မှုစံ:** Public rule မည်သည့်တစ်ခုမျှ owner မဲ့မနေဘဲ specification rule တိုင်းသည် pass/reject fixture တစ်ခုနှင့် ချိတ်ထားရမည်။

### 1.2 Compatibility policy

- [ ] Legacy behavior ကို normative, compatible, deprecated, rejected, native-only ဟူ၍ အမျိုးအစားခွဲပါ။
- [ ] Rust reference နှင့် Zap compiler ကွာခြားသည့် behavior တိုင်းအတွက် migration note နှင့် version policy ရေးပါ။
- [ ] Stable diagnostics အတွက် `ZAP-*` code, severity, span, notes, help နှင့် exit status contract သတ်မှတ်ပါ။
- [ ] Specification change တစ်ခုတိုင်းအတွက် RFC, fixture update, changelog, migration guide နှင့် compatibility decision လိုအပ်သည့် workflow တည်ဆောက်ပါ။

## 2။ Phase 1 — B1 Zap lexer ကို canonical လုပ်ခြင်း

လက်ရှိ `bootstrap/b1/lexer.zp` ကို candidate အဖြစ်မထားတော့ဘဲ real compiler pipeline ၏ ပထမဆုံး canonical stage အဖြစ် ပြောင်းရမည်။

### 2.1 Lexer implementation

- [ ] UTF-8 input reader နှင့် byte/character position conversion ကို Zap ဖြင့် အကောင်အထည်ဖော်ပါ။
- [ ] Identifier, keyword, number, string, escape, comment, newline နှင့် indentation token များကို အကောင်အထည်ဖော်ပါ။
- [ ] Operators, delimiters, multi-character operators နှင့် malformed token recovery ကို အကောင်အထည်ဖော်ပါ။
- [ ] Every token တွင် byte offset, line, column, length နှင့် source file ID ပါသော span ထည့်ပါ။
- [ ] Invalid UTF-8, invalid character, unterminated string, overflow, indentation jump နှင့် unexpected EOF များအတွက် stable diagnostics ထုတ်ပါ။
- [ ] Lexer output ကို JSON/text canonical snapshot အဖြစ် serialize/deserialize လုပ်နိုင်စေပါ။

### 2.2 Lexer verification

- [ ] Existing lexer fixtures အားလုံးကို B1 runner ဖြင့် run ပါ။
- [ ] Rust lexer output နှင့် Zap lexer output ကို token kind, lexeme, span, diagnostic အလိုက် compare ပါ။
- [ ] Unicode, deep indentation, long identifier, long string, numeric boundary နှင့် malformed corpus ထည့်ပါ။
- [ ] Fixed-seed fuzz runner ထည့်ပြီး failing seed နှင့် minimized input ကို archive လုပ်ပါ။
- [ ] Lexer သည် arbitrary malformed source တွင် panic မဖြစ်ကြောင်း CI gate ထည့်ပါ။

**Gate B1-Lexer:** Supported lexical grammar အားလုံးအတွက် Rust/Zap token parity ရှိရမည်။ Zap lexer သည် Rust lexer ကို runtime အတွင်း မခေါ်ရ။

## 3။ Phase 2 — B1 Zap parser နှင့် AST ကို canonical လုပ်ခြင်း

### 3.1 Parser implementation

- [ ] Token cursor, lookahead, error recovery နှင့် synchronization points ကို Zap ဖြင့် အပြီးသတ်ပါ။
- [ ] Declaration, assignment, function, class, module, import, conditional, loop, try/catch, raise နှင့် expression grammar များကို ထည့်ပါ။
- [ ] Calls, member access, indexing, list/map literal, generic syntax, grouped expression နှင့် precedence chain များကို အကောင်အထည်ဖော်ပါ။
- [ ] AST node တိုင်းတွင် source span နှင့် stable node kind ပါစေ။
- [ ] Parser error တစ်ကြိမ်တွင် လိုအပ်သလို multiple diagnostics ထုတ်နိုင်ပြီး recovery ပြီးနောက် arbitrary valid suffix ကို ဆက် parse နိုင်စေပါ။
- [ ] AST ကို canonical JSON/typed representation အဖြစ် ထုတ်ပြီး snapshot compare လုပ်နိုင်စေပါ။

### 3.2 Parser verification

- [ ] Existing parser fixtures အားလုံးကို Zap parser ဖြင့် run ပါ။
- [ ] Rust AST နှင့် Zap AST ကို normalized form ဖြင့် compare ပါ။
- [ ] Valid programs, rejected programs, nested blocks, deep nesting, recursive calls, Unicode နှင့် malformed recovery corpus ထည့်ပါ။
- [ ] Parser output ကို source formatting မပြောင်းဘဲ stable ဖြစ်ကြောင်းစစ်ပါ။
- [ ] Parse-only CLI mode (`zap parse file.zp --json`) ထည့်ပါ။

**Gate B1-Parser:** Supported syntax အတွက် Rust/Zap AST parity, stable diagnostics နှင့် no-panic property ပြည့်ရမည်။

## 4။ Phase 3 — Type system နှင့် semantic analyzer ကို Zap-owned လုပ်ခြင်း

### 4.1 Type model

- [ ] Primitive, collection, function, object/class, alias, option/result နှင့် module types ကို canonical type representation အဖြစ် သတ်မှတ်ပါ။
- [ ] Type variables, generic parameter, constraints, substitution, unification နှင့် occurs-check ကို အကောင်အထည်ဖော်ပါ။
- [ ] Literal inference, collection inference, return inference နှင့် function parameter inference ကို သတ်မှတ်ပါ။
- [ ] Branch narrowing, loop fixpoint, mutation invalidation, assignment compatibility နှင့် closure capture checking ကို အကောင်အထည်ဖော်ပါ။
- [ ] Module export/import type checking နှင့် cross-module generic signature checking ထည့်ပါ။
- [ ] Type errors များကို stable code/span/expected/actual/help ပုံစံဖြင့် ထုတ်ပါ။

### 4.2 Typed IR

- [ ] AST မှ typed IR သို့ lowering contract သတ်မှတ်ပါ။
- [ ] Symbol ID, scope ID, type ID, module ID, source span နှင့် inferred signature metadata ထည့်ပါ။
- [ ] Typed IR schema ကို versioned JSON နှင့် binary/internal form နှစ်မျိုးထားပါ။
- [ ] Typed IR validation pass ထည့်ပြီး invalid IR ကို runtime သို့ မပို့ရ။
- [ ] Rust type checker မခေါ်ဘဲ B2 Zap type checker တစ်ခုတည်းဖြင့် typed IR ထုတ်ပါ။

### 4.3 Type checker verification

- [ ] Positive/negative fixture တိုင်းကို expected type result သို့မဟုတ် expected diagnostic ဖြင့် သတ်မှတ်ပါ။
- [ ] Generic, option/result, nested collection, alias, class, imported module, loops နှင့် reassignment corpus ထည့်ပါ။
- [ ] Rust/Zap normalized typed-IR parity report ထုတ်ပါ။
- [ ] Inference မရသော case များကို deterministic rejection ဖြင့် handle လုပ်ပါ။
- [ ] Type checking သည် source order နှင့် platform အလိုက် မပြောင်းကြောင်း စစ်ပါ။

**Gate B2:** `.zp → Zap typed IR` ကို Rust မခေါ်ဘဲ ထုတ်နိုင်ပြီး accepted/rejected corpus အားလုံးတွင် semantic parity ရှိရမည်။

## 5။ Phase 4 — IR, lowering နှင့် executable representation

### 5.1 Intermediate representation

- [ ] Typed IR မှ executable IR သို့ explicit lowering stage ထည့်ပါ။
- [ ] Constants, local slots, global/module slots, functions, closures, calls, branches, loops, exceptions နှင့် cleanup blocks ကို IR instruction များဖြင့် သတ်မှတ်ပါ။
- [ ] Source map နှင့် debug metadata ထည့်ပါ။
- [ ] IR verifier ဖြင့် stack height, register/slot type, control-flow reachability နှင့် exception edge များ စစ်ပါ။
- [ ] Bytecode/instruction format ကို versioned, endian-independent, deterministic ဖြစ်အောင် သတ်မှတ်ပါ။
- [ ] Bytecode disassembler နှင့် inspection command (`zap inspect --bytecode`) ထည့်ပါ။

### 5.2 Lowering verification

- [ ] Every supported AST/typed-IR construct အတွက် lowering fixture ထည့်ပါ။
- [ ] Rust bytecode/reference execution နှင့် Zap bytecode ကို normalized instruction/observable result အလိုက် compare ပါ။
- [ ] Invalid typed IR ကို safe diagnostic ဖြင့် reject လုပ်ပါ။
- [ ] Bytecode output ကို repeated build နှစ်ကြိမ်တွင် byte-for-byte တူစေရမည်။

## 6။ Phase 5 — Zap-owned VM နှင့် runtime core

### 6.1 VM

- [ ] Instruction dispatch loop, call frame, local environment, return value နှင့် stack/slot management ကို Zap ဖြင့် အကောင်အထည်ဖော်ပါ။
- [ ] Function call, closure, recursion, object method, class instance, field read/write နှင့် module initialization ထည့်ပါ။
- [ ] Branch, loop, break/continue, raise/try/catch နှင့် cleanup semantics ထည့်ပါ။
- [ ] Runtime error တွင် source span, stack trace, diagnostic code နှင့် deterministic message ပါစေ။
- [ ] Instruction budget, recursion limit, memory/value size limit နှင့် output limit ထည့်ပါ။
- [ ] VM state snapshot/debug step mode ထည့်ပါ။

### 6.2 Value/memory model

- [ ] Number, text, list, map, object, function, module နှင့် error value representation ကို language-level contract အဖြစ် သတ်မှတ်ပါ။
- [ ] Heap allocation, deallocation, object cycle, closure capture နှင့် field cleanup ကို Rust-specific API မဟုတ်ဘဲ language behavior အဖြစ် ဖော်ပြပါ။
- [ ] Reference counting, arena သို့မဟုတ် tracing GC တစ်ခုကို ရွေးပြီး rationale, limits နှင့် concurrency boundary ရေးပါ။
- [ ] Borrow conflict နှင့် invalid access ကို panic မဖြစ်စေဘဲ stable runtime error အဖြစ် ပြပါ။
- [ ] Heap statistics နှင့် memory-limit test fixture ထည့်ပါ။

### 6.3 VM verification

- [ ] `bootstrap/fixtures/vm` နှင့် core examples အားလုံးကို Zap VM ဖြင့် run ပါ။
- [ ] Rust evaluator နှင့် observable result, stdout, stderr, exit code, raised value နှင့် diagnostic ကို compare ပါ။
- [ ] Repeated run တွင် result နှင့် diagnostics တူကြောင်း စစ်ပါ။
- [ ] Resource exhaustion, malformed bytecode, deep recursion, object cycle နှင့် shutdown tests ထည့်ပါ။

**Gate B3-VM:** Core language programs များကို Zap compiler + Zap VM တစ်ခုတည်းဖြင့် run နိုင်ပြီး Rust evaluator fallback မရှိရ။

## 7။ Phase 6 — Standard library နှင့် OS boundary ကို Rust-independent လုပ်ခြင်း

### 7.1 Standard-library contract

- [ ] Public API တစ်ခုချင်းစီအတွက် status (experimental/provisional/stable/deprecated/platform-specific) သတ်မှတ်ပါ။
- [ ] Input/output limits, timeout, cancellation, determinism, error codes နှင့် platform differences မှတ်တမ်းတင်ပါ။
- [ ] String, collection, math, time, JSON, filesystem, process, URL, HTTP, environment နှင့် config APIs များကို language-level interface အဖြစ် သတ်မှတ်ပါ။
- [ ] Runtime core နှင့် OS adapter ကို ခွဲပါ။ Compiler သည် OS-specific Rust module ကို တိုက်ရိုက်မသိရ။
- [ ] Minimal platform seed ABI သတ်မှတ်ပါ။ ဥပမာ file read/write, stdout/stderr, clock, process exit, memory allocation နှင့် socket boundary များ။

### 7.2 Runtime adapter

- [ ] POSIX adapter နှင့် Windows adapter ကို တစ်ခုချင်းစီ သတ်မှတ်ထားသော ABI ဖြင့် တည်ဆောက်ပါ။
- [ ] Path separator, permissions, newline, process group, signal, symlink နှင့် archive limitation များကို explicit behavior လုပ်ပါ။
- [ ] Network/process calls များအတွက် deadline, output bound, cancellation နှင့် child cleanup ထည့်ပါ။
- [ ] Sandbox/permission policy နှင့် capability list ကို runtime configuration အဖြစ် သတ်မှတ်ပါ။

**ပြီးမြောက်မှုစံ:** Application-level Zap source သည် Rust crate API သို့မဟုတ် Cargo dependency ကို မမြင်ရ။

## 8။ Phase 7 — Module, package, registry နှင့် build system ကို Zap-owned လုပ်ခြင်း

- [ ] `zap.toml` schema, workspace, module root, entry list နှင့် target profile ကို specification အဖြစ် freeze လုပ်ပါ။
- [ ] Deterministic module resolver, duplicate detection, missing import, cycle report နှင့် alias resolution ကို Zap ဖြင့် အကောင်အထည်ဖော်ပါ။
- [ ] Lockfile parser/writer, checksum verification, offline mode, dependency graph နှင့် cache layout ထည့်ပါ။
- [ ] `zap build`, `zap test`, `zap run`, `zap fmt`, `zap check`, `zap package` commands များကို Zap toolchain ဖြင့် အကောင်အထည်ဖော်ပါ။
- [ ] Registry transport, authentication, signature, version selection, download bound နှင့် failure diagnostics ကို contract လုပ်ပါ။
- [ ] Build graph ကို deterministic topological order ဖြင့် run ပါ။
- [ ] Source-to-artifact provenance ထည့်ပြီး compiler version, target, input hashes နှင့် dependency hashes မှတ်တမ်းတင်ပါ။
- [ ] Cargo/Rust မရှိသော clean container တွင် sample project build ပြီး run နိုင်ကြောင်း စစ်ပါ။

## 9။ Phase 8 — Formatter, LSP နှင့် developer tooling

- [ ] Canonical Zap parser ကို formatter, LSP, syntax highlighter နှင့် test runner တို့အားလုံး share လုပ်စေပါ။
- [ ] Formatter သည် parse/format/parse လုပ်ပြီး semantic AST မပြောင်းကြောင်း စစ်ပါ။
- [ ] LSP တွင် diagnostics, completion, hover, definition, references, rename, document symbols, workspace symbols နှင့် semantic tokens ထည့်ပါ။
- [ ] Imported-but-unopened module, generic signature, nested range, async boundary နှင့် source span conversion fixture များထည့်ပါ။
- [ ] VS Code extension သည် Rust parser သို့မဟုတ် duplicated grammar မသုံးရ။
- [ ] CLI/LSP diagnostic code နှင့် range conversion ကို တစ်နေရာတည်းက ပိုင်ဆိုင်စေပါ။

## 10။ Phase 9 — Self-hosting bootstrap chain

### 10.1 Seed compiler

- [ ] Platform seed ၏ အဓိက capability များ၊ executable format နှင့် supported target များကို သတ်မှတ်ပါ။
- [ ] Seed compiler သည် B1 lexer/parser ကို load/run လုပ်နိုင်ရမည်။
- [ ] Seed compiler သည် B2 type checker နှင့် typed IR ကို load/run လုပ်နိုင်ရမည်။
- [ ] Seed compiler သည် B3 lowerer/VM/build/package stages ကို load/run လုပ်နိုင်ရမည်။
- [ ] Seed မှထုတ်သော compiler သည် မိမိ Zap compiler source ကို compile လုပ်နိုင်ရမည်။

### 10.2 Rebuild proof

- [ ] Clean machine/container တစ်ခုတွင် seed artifact နှင့် Zap source သာဖြင့် compiler build လုပ်ပါ။
- [ ] First build output ကို second build ဖြင့် ပြန် compile လုပ်ပါ။
- [ ] Compiler executable hash, bytecode, embedded stdlib, package lock နှင့် semantic test digest များ compare ပါ။
- [ ] Hash မတူပါက reproducibility report ထုတ်ပြီး nondeterministic source ကို ပြင်ပါ။
- [ ] Rust/Cargo မရှိသော environment တွင် B4 rebuild ကို run ပါ။
- [ ] Linux x86_64, Windows x86_64 နှင့် macOS ARM64 အတွက် target verification ပြုလုပ်ပါ။

**Gate B4:** Zap compiler သည် documented platform seed ဖြင့် မိမိ Zap source ကို ပြန် buildနိုင်ပြီး final user build path တွင် Rust မလိုတော့ရ။

## 11။ Phase 10 — Rust dependency ဖယ်ရှားခြင်းနှင့် release

- [ ] End-user installer/image မှ Rust toolchain, Cargo နှင့် native Rust compiler requirement ဖယ်ရှားပါ။
- [ ] Runtime/compiler source tree တွင် Rust implementation ကို `reference/` သို့မဟုတ် `legacy/` အဖြစ် သီးခြားခွဲပါ။
- [ ] CI ကို `zap test` နှင့် `zap compiler rebuild` ကို primary gate လုပ်ပြီး Rust test ကို compatibility-only gate လုပ်ပါ။
- [ ] Dependency scanner ဖြင့် release artifact ထဲတွင် Rust/Cargo မပါကြောင်း စစ်ပါ။
- [ ] Reproducible archive, checksum, signature, SBOM, provenance နှင့် rollback procedure ထည့်ပါ။
- [ ] Release notes တွင် B4/self-hosted claim ကို rebuild evidence နှင့်သာ ပြုလုပ်ပါ။
- [ ] Rust reference ကို မည်သည့် version အထိ ထိန်းသိမ်းမည်၊ မည်သည့် release တွင် archive လုပ်မည်ကို သတ်မှတ်ပါ။

## 12။ Test နှင့် CI matrix

| Test layer | စစ်ရမည့်အရာ | Gate |
|---|---|---|
| Lexer | Token, span, malformed input, Unicode, indentation | B1 |
| Parser | AST, precedence, recovery, diagnostics | B1 |
| Type checker | Inference, generics, modules, flow, mutation | B2 |
| IR | Typed IR validity, lowering, deterministic serialization | B2/B3 |
| VM | Calls, closures, objects, exceptions, limits | B3 |
| Stdlib | I/O bounds, errors, determinism, platform behavior | B3 |
| Package | Manifest, lockfile, checksum, offline build | B3/B4 |
| Tooling | Formatter/LSP/VS Code parity | B3 |
| Bootstrap | Seed build, self-rebuild, artifact comparison | B4 |
| Security | Fuzz, malformed data, path traversal, resource exhaustion | Every release |

CI တွင် Linux, Windows, macOS; debug/release; clean/no-Rust environment; fixed-seed replay; formatting; lint; documentation link check; package reproducibility နှင့် bootstrap rebuild job များ ထည့်ရမည်။

## 13။ လုပ်ဆောင်ရမည့် အစဉ်တို

1. Specification နှင့် ownership index ကို freeze လုပ်ပါ။
2. B1 lexer ကို Zap ဖြင့်ပြီး differential tests ဖြတ်ပါ။
3. B1 parser/AST ကို Zap ဖြင့်ပြီး differential tests ဖြတ်ပါ။
4. B2 type checker/typed IR ကို Rust မခေါ်ဘဲ canonical လုပ်ပါ။
5. B3 lowering နှင့် executable IR ကို versioned လုပ်ပါ။
6. B3 VM/runtime core ကို Zap ဖြင့် run နိုင်အောင်လုပ်ပါ။
7. Standard library နှင့် platform ABI ကို language contract အဖြစ် ခွဲပါ။
8. Module/package/registry/build commands ကို Zap-owned လုပ်ပါ။
9. Formatter/LSP/tooling ကို canonical parser နှင့် ချိတ်ပါ။
10. Platform seed ဖြင့် compiler ကို bootstrap လုပ်ပါ။
11. Compiler source ကို မိမိ compiler ဖြင့် ပြန် buildပြီး deterministic compare လုပ်ပါ။
12. Rust/Cargo မရှိသော clean environment နှင့် target သုံးမျိုးတွင် release verify လုပ်ပါ။
13. Rust implementation ကို reference-only အဖြစ် ခွဲပြီး B4 self-hosted release ထုတ်ပါ။

## 14။ မပြီးသေးကြောင်း သတ်မှတ်ရမည့် အချက်များ

အောက်ပါအရာတစ်ခုခုရှိနေသေးပါက Zap ကို fully Rust-independent ဟု မကြေညာရ။

- Zap parser/type checker/VM သည် Rust implementation ကို hidden fallback အဖြစ် ခေါ်နေခြင်း။
- `zap build` သည် Cargo သို့မဟုတ် Rust compiler ကို လိုနေခြင်း။
- Standard library semantics ကို Rust module ၏ undocumented behavior က ဆုံးဖြတ်နေခြင်း။
- Self-rebuild သည် sample program အနည်းငယ်သာဖြစ်ပြီး compiler source အပြည့်ကို မပြန် buildနိုင်ခြင်း။
- Clean machine တွင် seed, Zap source နှင့် documented platform capability များဖြင့် build မဖြစ်ခြင်း။
- Rebuild output သည် deterministic မဖြစ်ခြင်း။
- Windows/macOS target တွင် semantic parity နှင့် artifact verification မရှိခြင်း။

## References

[1]: ../bootstrap/contracts/BOOTSTRAP_CONTRACT_MM.md "Zap Bootstrap Contract"
[2]: ../bootstrap/contracts/B4_RUST_FREE_FULL_LANGUAGE_CONTRACT.toml "B4 Rust-Free Full-Language Contract"
[3]: ../bootstrap/contracts/B4_ACCEPTANCE.tsv "B4 Acceptance Matrix"

- [Zap repository](https://github.com/hidecard/zap)
- [B0 Baseline Freeze](../bootstrap/BASELINE_B0.md)
- [Bootstrap Contract](../bootstrap/contracts/BOOTSTRAP_CONTRACT_MM.md)
- [Bootstrap source tree](../bootstrap/)
- [Current language status](CURRENT_STATUS_MM.md)
- [Existing Burmese TODO](TODO_ZAP_MM.md)
