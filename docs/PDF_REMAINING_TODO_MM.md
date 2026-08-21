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
| P0-02 | ပေါင်းစည်းထားသော language specification | Partial | `LANGUAGE_SPEC_MM.md` သည် syntax၊ precedence၊ typing၊ runtime behavior၊ compatibility နှင့် version ownership အတွက် canonical semantic index ဖြစ်လာပြီ။ Fragmented rule များကို အပြည့်အဝ ရွှေ့ပြောင်းခြင်းနှင့် complete conformance fixture များသာ ကျန်ရှိသည်။ |
| P0-03 | Structured diagnostics | Partial | User-facing error တိုင်းတွင် severity၊ stable code၊ message၊ source span၊ notes/help ပါပြီး snapshot test ရှိရမည်။ |
| P0-04 | Memory နှင့် reference-cycle စာချုပ် | Partial | `Rc<RefCell>` ownership policy၊ explicit non-thread-safe boundary၊ `Value::object`၊ `clear_object_fields`၊ `object_field_count` နှင့် cycle-breaking regression test ကို docs/code တွင် ထည့်ပြီးဖြစ်သည်။ Heap statistics၊ allocation counters၊ weak references နှင့် tracing collection တို့သာ ကျန်ရှိသည်။ |
| P0-05 | Deterministic နှင့် production async boundary | Partial | Deterministic executor ကို သီးခြားရှင်းပြပြီး production I/O၊ blocking call၊ cancellation နှင့် scheduling boundary များ သတ်မှတ်ရမည်။ |

## P1 — Production readiness

| ID | လုပ်ငန်း | Status | ပြီးစီးမှု စံနှုန်း |
|---|---|---|---|
| P1-01 | Gradual type checking ပြီးစီးအောင်လုပ်ခြင်း | ပြီးစီး | Annotation enforcement၊ collection element typing၊ runtime mismatch diagnostics၊ control-flow narrowing၊ structured diagnostics၊ TC-001–TC-012 conformance evidence နှင့် generic/inference limits ကို ဘာသာနှစ်မျိုး type-system contract များတွင် မှတ်တမ်းတင်ပြီး စမ်းသပ်ထားသည်။ |
| P1-02 | Benchmark နှင့် profiling harness | ပြီးစီး | Dependency-free repeatable harness သည် loop၊ user-defined call၊ captured-state closure၊ collection allocation၊ JSON conversion၊ deterministic async scheduling နှင့် explicit module/import dispatch များကို CSV output ဖြင့် လွှမ်းခြုံထားသည်။ `scripts/aggregate_benchmark.sh` သည် deterministic min/mean/max summary ထုတ်ပေးပြီး CI သည် timing threshold မသတ်မှတ်ဘဲ seven-suite smoke နှင့် artifact upload ကို run လုပ်သည်။ |
| P1-03 | Registry supply-chain hardening | Partial | Redaction၊ traversal၊ wrong-key/mutated-payload fail-closed tests၊ protected-release provenance identity checks၊ adversarial signed-provenance mutation coverage၊ full-fingerprint signing-key rotation allowlist၊ yanked metadata parsing/resolution enforcement၊ unauthorized publish rejection၊ invalid package identity rejection နှင့် publish checksum mismatch rejection ကို အကောင်အထည်ဖော်ပြီးဖြစ်သည်။ Exact နှင့် range resolution နှစ်မျိုးစလုံးသည် yanked candidate များကို ကျော်ပြီး malformed yanked metadata ကို fail-closed reject လုပ်သည်။ Stable exact/range all-yanked diagnostics များကိုလည်း test လုပ်ပြီးဖြစ်သည်။ Explicit locked yanked-artifact checksum path ကို စစ်ဆေးပြီးဖြစ်သော်လည်း end-to-end lockfile/cache compatibility final audit သာ ကျန်ရှိသည်။ |
| P1-04 | Deterministic package resolution | ပြီးစီး | `scripts/verify_clean_machine_locked.sh` သည် registry access မလိုဘဲ `zap install --locked` နှင့် `zap build --locked` ၏ clean-copy repeatability ကို သက်သေပြပြီး tampered `zap.lock` ကို reject လုပ်သည်။ |
| P1-05 | Conformance/property/fuzz test layers | Partial | Parser golden-style unit test များ၊ deterministic parser/lexer/JSON/lockfile/registry-security corpus များ၊ collection/filesystem regression များ၊ async cancellation/scheduler cases များနှင့် malformed source mutation ခုနစ်မျိုးပါဝင်သော deterministic fuzz-style CLI corpus ကို `scripts/test_p105_layers.sh` မှတစ်ဆင့် CI တွင် မြင်နိုင်ပြီ။ Malformed mutation များသည် panic မဖြစ်ဘဲ safe rejection ပြုလုပ်သည်။ Linux၊ Windows နှင့် macOS build/test matrix coverage ကို ဆက်လက် run လုပ်သည်။ Long-running fuzz target များ၊ allocator/heap-level counter များနှင့် ထပ်မံ platform-specific input case များကို ဆက်လက်ထည့်ရန် ကျန်ရှိသည်။ |

## P2 — ရေရှည် language နှင့် ecosystem

| ID | လုပ်ငန်း | Status | ပြီးစီးမှု စံနှုန်း |
|---|---|---|---|
| P2-01 | Composition နှင့် traits/interfaces | Deferred | Inheritance မှ migration၊ composition၊ trait/protocol rules နှင့် method resolution အတွက် RFC ကို implementation မစမီ ရေးသား/သုံးသပ်ရမည်။ |
| P2-02 | Standard-library API stability policy | Partial | Public module တိုင်းတွင် stability label၊ deprecation period၊ semantic-versioning rule နှင့် platform support matrix ရှိရမည်။ |
| P2-03 | LSP/VS Code semantic parity | Partial | Rename၊ nested/module-aware indexing၊ async-aware completion/hover နှင့် canonical parser/AST coverage ကို test လုပ်ရမည်။ |
| P2-04 | Learning/reference documentation ခွဲခြားခြင်း | Partial | Beginner guide၊ syntax reference၊ specification၊ stdlib reference၊ package author guide၊ runtime internals နှင့် deployment/security docs များ navigation နှင့် verified-version metadata ပါရမည်။ |

## လုပ်ဆောင်မည့်အစီအစဉ်

၁။ **P0-03:** Structured diagnostic schema နှင့် snapshot fixtures ကို ပြီးစီးအောင်လုပ်ရန်။
၂။ **P0-04:** အကောင်အထည်ဖော်ပြီးသော memory contract ကို heap statistics၊ allocation counters၊ weak-reference diagnostics နှင့် closure-cycle coverage ဖြင့် ဆက်လက်တိုးချဲ့ရန်။
၃။ **P0-05:** Deterministic async limitations နှင့် production boundaries ကို documentation တွင် ပြတ်သားစွာ သတ်မှတ်ရန်။  
၄။ **P1-02:** Performance claim မပြုမီ benchmark/profiling harness တည်ဆောက်ရန်။  
၅။ **P1-03:** Registry redaction၊ fail-closed၊ traversal၊ provenance၊ key-rotation နှင့် yanked-release tests ထည့်ရန်။ လက်ရှိ slice တွင် signed tag၊ commit၊ workflow၊ HTTPS source၊ checksum၊ signing fingerprint အပြည့်အစုံ၊ trusted-fingerprint allowlist၊ adversarial signed-provenance mutation rejection၊ yanked candidate skip၊ malformed-yanked rejection၊ stable exact/range all-yanked diagnostics နှင့် explicit locked-cache checksum compatibility လိုအပ်ချက်များကို enforce လုပ်ထားသည်။
၆။ **P1-05:** Parser golden၊ property၊ fuzz၊ memory နှင့် security test layers တိုးချဲ့ရန်။  
၇။ **P1-01/P1-04:** ပြီးစီးပြီး။ ဘာသာနှစ်မျိုး gradual-typing baseline ကို မှတ်တမ်းတင်ပြီး clean-machine locked install/build verifier ကို executable နှင့် deterministic အဖြစ် ပြီးစီးထားသည်။
၈။ **P2-02/P2-03/P2-04:** Stdlib policy၊ tooling parity နှင့် documentation navigation ပြီးစီးရန်။  
၉။ **P2-01:** Parser/runtime မပြောင်းမီ traits/composition RFC ကို ရေးသားပြီး review လုပ်ရန်။

## Release policy

အဆင့်တိုင်းတွင် pinned CI toolchain ဖြင့် strict Clippy၊ formatter၊ full native test suite၊ သက်ဆိုင်ရာ conformance tests၊ English/Burmese documentation parity နှင့် `git diff --check` အားလုံး အောင်မြင်ရမည်။ သက်ဆိုင်ရာ acceptance criteria နှင့် CI gates များ မပြီးမချင်း release tag အသစ် မထုတ်ရ။
