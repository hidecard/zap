# Zap တွင် ကျန်ရှိသော Engineering To-do Register

**အခြေခံအခြေအနေ:** Zap v2.1.6 verified release  
**ရင်းမြစ်:** `Zap_တွင်_ပြင်ဆင်သင့်သောအချက်များ.pdf`  
**ရည်ရွယ်ချက်:** ပြီးစီးပြီးသား release အလုပ်များကို မပြီးသေးဟု မတွက်ဘဲ PDF အကြံပြုချက်များထဲမှ အမှန်တကယ်ကျန်ရှိသောအချက်အားလုံးကို စောင့်ကြည့်ရန်။

## Status အဓိပ္ပါယ်

| Status | အဓိပ္ပါယ် |
|---|---|
| Done | Code/test သို့မဟုတ် release evidence ဖြင့် အကောင်အထည်ဖော်ပြီးဖြစ်သည်။ |
| Partial | အခြေခံ implementation ရှိသော်လည်း PDF recommendation ၏ အရေးကြီး gap များ ကျန်နေသည်။ |
| Todo | မအကောင်အထည်ဖော်ရသေးခြင်း သို့မဟုတ် repository evidence မရှိသေးခြင်း။ |
| Deferred | Core semantics သို့မဟုတ် deployment architecture ကို ပြောင်းလဲရသောကြောင့် နောက် milestone သို့ ရည်ရွယ်ချက်ရှိရှိ ရွှေ့ထားခြင်း။ |

## P0 — ယုံကြည်စိတ်ချရမှု အခြေခံ

| ID | လုပ်ငန်း | Status | ပြီးစီးမှု စံနှုန်း |
|---|---|---|---|
| P0-01 | Native/legacy conformance စာချုပ် | Partial | Native behavior ကို canonical သတ်မှတ်ပြီး legacy fixture parity report၊ migration policy နှင့် CI conformance command ရှိရမည်။ |
| P0-02 | ပေါင်းစည်းထားသော language specification | Partial | Syntax၊ precedence၊ typing၊ runtime behavior၊ compatibility နှင့် version ownership ကို normative specification တစ်ခုတည်းက သတ်မှတ်ရမည်။ |
| P0-03 | Structured diagnostics | Partial | User-facing error တိုင်းတွင် severity၊ stable code၊ message၊ source span၊ notes/help ပါပြီး snapshot test ရှိရမည်။ |
| P0-04 | Memory နှင့် reference-cycle စာချုပ် | Todo | `Rc<RefCell>` policy ကို docs တွင် ရှင်းလင်းပြီး cycle regression tests နှင့် explicit non-thread-safe boundary ထည့်ရမည်။ |
| P0-05 | Deterministic နှင့် production async boundary | Partial | Deterministic executor ကို သီးခြားရှင်းပြပြီး production I/O၊ blocking call၊ cancellation နှင့် scheduling boundary များ သတ်မှတ်ရမည်။ |

## P1 — Production readiness

| ID | လုပ်ငန်း | Status | ပြီးစီးမှု စံနှုန်း |
|---|---|---|---|
| P1-01 | Gradual type checking ပြီးစီးအောင်လုပ်ခြင်း | Partial | Annotation enforcement၊ collection element typing၊ runtime mismatch diagnostics နှင့် generic/inference limits ကို docs/test ဖြင့် သတ်မှတ်ရမည်။ |
| P1-02 | Benchmark နှင့် profiling harness | Todo | Loop၊ call၊ closure၊ allocation၊ dispatch၊ import၊ JSON နှင့် async scheduling အတွက် repeatable benchmark ရှိရမည်။ |
| P1-03 | Registry supply-chain hardening | Partial | Redaction tests၊ traversal/security fuzzing၊ signature/checksum fail-closed tests၊ provenance policy၊ key rotation နှင့် yanked-release rules ရှိရမည်။ |
| P1-04 | Deterministic package resolution | Partial | Clean machine တွင် `zap install --locked` နှင့် `zap build --locked` သည် verified reproducible result ထုတ်ရမည်။ |
| P1-05 | Conformance/property/fuzz test layers | Partial | Parser golden၊ property tests၊ fuzz targets၊ memory regression၊ async determinism၊ security input နှင့် cross-platform case များ CI တွင် မြင်ရမည်။ |

## P2 — ရေရှည် language နှင့် ecosystem

| ID | လုပ်ငန်း | Status | ပြီးစီးမှု စံနှုန်း |
|---|---|---|---|
| P2-01 | Composition နှင့် traits/interfaces | Deferred | Inheritance မှ migration၊ composition၊ trait/protocol rules နှင့် method resolution အတွက် RFC ကို implementation မစမီ ရေးသား/သုံးသပ်ရမည်။ |
| P2-02 | Standard-library API stability policy | Partial | Public module တိုင်းတွင် stability label၊ deprecation period၊ semantic-versioning rule နှင့် platform support matrix ရှိရမည်။ |
| P2-03 | LSP/VS Code semantic parity | Partial | Rename၊ nested/module-aware indexing၊ async-aware completion/hover နှင့် canonical parser/AST coverage ကို test လုပ်ရမည်။ |
| P2-04 | Learning/reference documentation ခွဲခြားခြင်း | Partial | Beginner guide၊ syntax reference၊ specification၊ stdlib reference၊ package author guide၊ runtime internals နှင့် deployment/security docs များ navigation နှင့် verified-version metadata ပါရမည်။ |

## လုပ်ဆောင်မည့်အစီအစဉ်

၁။ **P0-03:** Structured diagnostic schema နှင့် snapshot fixtures ကို ပြီးစီးအောင်လုပ်ရန်။  
၂။ **P0-04:** Memory contract သတ်မှတ်ပြီး object/closure cycle regression tests ထည့်ရန်။  
၃။ **P0-05:** Deterministic async limitations နှင့် production boundaries ကို documentation တွင် ပြတ်သားစွာ သတ်မှတ်ရန်။  
၄။ **P1-02:** Performance claim မပြုမီ benchmark/profiling harness တည်ဆောက်ရန်။  
၅။ **P1-03:** Registry redaction၊ fail-closed၊ traversal နှင့် provenance tests ထည့်ရန်။  
၆။ **P1-05:** Parser golden၊ property၊ fuzz၊ memory နှင့် security test layers တိုးချဲ့ရန်။  
၇။ **P1-01/P1-04:** Gradual typing documentation နှင့် clean-machine locked-install verification ပြီးစီးရန်။  
၈။ **P2-02/P2-03/P2-04:** Stdlib policy၊ tooling parity နှင့် documentation navigation ပြီးစီးရန်။  
၉။ **P2-01:** Parser/runtime မပြောင်းမီ traits/composition RFC ကို ရေးသားပြီး review လုပ်ရန်။

## Release policy

အဆင့်တိုင်းတွင် pinned CI toolchain ဖြင့် strict Clippy၊ formatter၊ full native test suite၊ သက်ဆိုင်ရာ conformance tests၊ English/Burmese documentation parity နှင့် `git diff --check` အားလုံး အောင်မြင်ရမည်။ သက်ဆိုင်ရာ acceptance criteria နှင့် CI gates များ မပြီးမချင်း release tag အသစ် မထုတ်ရ။
