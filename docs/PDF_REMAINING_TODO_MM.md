# Zap တွင် ကျန်ရှိသော Engineering To-do Register

**အခြေခံအခြေအနေ:** Zap v2.1.8 verified release
**ရင်းမြစ်:** `Zap_တွင်_ပြင်ဆင်သင့်သောအချက်များ.pdf`  
**ရည်ရွယ်ချက်:** ပြီးစီးပြီးသား release အလုပ်များကို မပြီးသေးဟု မတွက်ဘဲ PDF အကြံပြုချက်များထဲမှ အမှန်တကယ်ကျန်ရှိသောအချက်အားလုံးကို စောင့်ကြည့်ရန်။

**အသေးစိတ်လုပ်ဆောင်မှုအစီအစဉ်:** [`NEXT_TODO_PLAN_MM.md`](NEXT_TODO_PLAN_MM.md) တွင် ကျန်ရှိသောအလုပ်များ၏ milestone အစီအစဉ်၊ implementation tasks၊ acceptance evidence နှင့် release gates များကို အသေးစိတ်ဖော်ပြထားသည်။

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
| P0-01 | Native/legacy conformance စာချုပ် | Partial — P0-01-A implemented | Native behavior ကို canonical သတ်မှတ်ထားသည်။ P0-01-A တွင် `common`၊ `native-only` နှင့် `rejected` policy များပါသော versioned six-case matrix၊ normalized stdout digest၊ deterministic tab-separated report၊ migration guidance နှင့် CI parity artifact gate များကို ထည့်သွင်းပြီးဖြစ်သည်။ Broader legacy inventory နှင့် future drift approval များသာ ကျန်ရှိသည်။ |
| P0-02 | ပေါင်းစည်းထားသော language specification | Partial — ownership expansion slice implemented | `LANGUAGE_SPEC_EN.md` နှင့် `LANGUAGE_SPEC_MM.md` သည် bilingual canonical semantic index အဖြစ် ဆက်လက်ရှိသည်။ Ownership index တွင် required semantic domain များအတွက် stable rule ID ၂၇ ခု ပါဝင်ပြီး unique ID နှင့် domain coverage ကို စစ်ဆေးကာ fixture/test owner များနှင့် bilingual compatibility/deprecation template များကို mapping လုပ်ထားသည်။ ကျန် fragmented rule အားလုံးနှင့် complete conformance fixture ownership များကို ဆက်လက်ချဲ့ထွင်ရန် ကျန်ရှိသည်။ Release preflight တွင် ownership gate နှင့် parity၊ replay၊ focused async contract gate များကိုပါ run လုပ်သည်။ |
| P0-03 | Structured diagnostics | Done | CLI JSON နှင့် LSP diagnostic များတွင် stable `ZAP-*` code၊ kind၊ severity၊ normalized message၊ source span၊ deterministic notes/help နှင့် deterministic snapshot/regression coverage ပါဝင်သည်။ |
| P0-04 | Memory နှင့် reference-cycle စာချုပ် | Partial — checked borrow slice implemented | `Rc<RefCell>` ownership policy၊ explicit non-thread-safe boundary၊ tracked `Value::object`၊ checked object-field `try_borrow`/`try_borrow_mut` accessor များ၊ stable `ZAP-BORROW-001` diagnostic၊ fallible `clear_object_fields`/`object_field_count`၊ bounded `memory_stats()`၊ object allocation/deallocation counters၊ cycle-safe value validation နှင့် deterministic memory-limit tests များကို ထည့်သွင်းပြီးဖြစ်သည်။ Public weak references၊ closure-level/process-wide telemetry၊ per-run byte accounting၊ arbitrary cycle အလိုအလျောက် reclaim လုပ်ခြင်းနှင့် tracing collection တို့သာ ကျန်ရှိသည်။ |
| P0-05 | Deterministic နှင့် production async boundary | Partial | Deterministic executor၊ fixed-worker၊ bounded network/process adapter များ၊ cancellation behavior၊ descriptive `async_capabilities()` report၊ typed resource-limit preflight validation၊ TCP request-size admission check နှင့် target-named CI artifact များပါသော reproducible Linux/Windows/macOS focused matrix ကို document/expose လုပ်ပြီးဖြစ်သည်။ Executor-backed language-level scheduling၊ language-level cancellation/timeout controls နှင့် tooling synchronization တို့သာ ကျန်ရှိသည်။ |
| P0-06 | Release version single-source-of-truth gate | Completed — P0 release slice | `native/Cargo.toml` သည် authoritative version source ဖြစ်သည်။ Validator သည် Cargo၊ Cargo.lock၊ CLI output၊ optional release tag၊ changelog များ၊ bilingual README release link/archive name များ၊ `SECURITY.md`၊ conformance metadata၊ bilingual release note များ၊ release template နှင့် installer metadata များကို စစ်ဆေးသည်။ Deterministic TSV evidence၊ positive/negative regression harness၊ CI artifact upload၊ release-preflight enforcement နှင့် bilingual policy documentation များကို အကောင်အထည်ဖော်ပြီးဖြစ်သည်။ |

## P1 — Production readiness

| ID | လုပ်ငန်း | Status | ပြီးစီးမှု စံနှုန်း |
|---|---|---|---|
| P1-01 | Gradual type checking ပြီးစီးအောင်လုပ်ခြင်း | ပြီးစီး | Annotation enforcement၊ collection element typing၊ runtime mismatch diagnostics၊ control-flow narrowing၊ structured diagnostics၊ TC-001–TC-012 conformance evidence နှင့် generic/inference limits ကို ဘာသာနှစ်မျိုး type-system contract များတွင် မှတ်တမ်းတင်ပြီး စမ်းသပ်ထားသည်။ |
| P1-02 | Benchmark နှင့် profiling harness | ပြီးစီး | Dependency-free repeatable harness သည် loop၊ user-defined call၊ captured-state closure၊ collection allocation၊ JSON conversion၊ deterministic async scheduling နှင့် explicit module/import dispatch များကို CSV output ဖြင့် လွှမ်းခြုံထားသည်။ `scripts/aggregate_benchmark.sh` သည် deterministic min/mean/max summary ထုတ်ပေးပြီး CI သည် timing threshold မသတ်မှတ်ဘဲ seven-suite smoke နှင့် artifact upload ကို run လုပ်သည်။ |
| P1-03 | Registry supply-chain hardening | ပြီးစီး | Redaction၊ traversal၊ wrong-key/mutated-payload fail-closed tests၊ protected-release provenance identity checks၊ adversarial signed-provenance mutation coverage၊ full-fingerprint signing-key rotation allowlist၊ yanked metadata parsing/resolution enforcement၊ unauthorized publish rejection၊ invalid package identity rejection နှင့် publish checksum mismatch rejection ကို အကောင်အထည်ဖော်ပြီးဖြစ်သည်။ Exact နှင့် range resolution နှစ်မျိုးစလုံးသည် yanked candidate များကို ကျော်ပြီး malformed yanked metadata ကို fail-closed reject လုပ်သည်။ Stable exact/range all-yanked diagnostics များလည်း ရှိသည်။ End-to-end locked-cache audit သည် manifest requirement ကိုက်ညီမှု၊ checksum integrity၊ explicit locked yanked-cache reuse၊ offline operation နှင့် tampered lock/cache rejection တို့ကို အတည်ပြုပြီးဖြစ်သည်။ |
| P1-04 | Deterministic package resolution | ပြီးစီး | `scripts/verify_clean_machine_locked.sh` သည် registry access မလိုဘဲ `zap install --locked` နှင့် `zap build --locked` ၏ clean-copy repeatability ကို သက်သေပြပြီး tampered `zap.lock` ကို reject လုပ်သည်။ |
| P1-05 | Conformance/property/fuzz test layers | Partial — P1-05-A replay slice ပြီးစီး | Parser golden-style unit test များ၊ deterministic parser/lexer/JSON/lockfile/registry-security corpus များ၊ collection/filesystem regression များ၊ async cancellation/scheduler cases များနှင့် malformed source mutation ခုနစ်မျိုးပါဝင်သော deterministic fuzz-style CLI corpus ကို `scripts/test_p105_layers.sh` မှတစ်ဆင့် CI တွင် မြင်နိုင်ပြီ။ P1-05-A တွင် fixed-seed `ZAP_CORPUS_SEED` replay၊ parser/JSON/lockfile/registry/memory/async durable fixture category ခြောက်မျိုး၊ SHA-256/base64 replay evidence နှင့် CI artifact upload တို့ကို ထည့်သွင်းပြီးဖြစ်သည်။ Malformed mutation များသည် panic မဖြစ်ဘဲ safe rejection ပြုလုပ်သည်။ Long-running fuzz target များ၊ allocator/heap-level counter များနှင့် ထပ်မံ platform-specific input case များကို ဆက်လက်ထည့်ရန် ကျန်ရှိသည်။ |

## P2 — ရေရှည် language နှင့် ecosystem

| ID | လုပ်ငန်း | Status | ပြီးစီးမှု စံနှုန်း |
|---|---|---|---|
| P2-01 | Composition နှင့် traits/interfaces | Deferred | Inheritance မှ migration၊ composition၊ trait/protocol rules နှင့် method resolution အတွက် RFC ကို implementation မစမီ ရေးသား/သုံးသပ်ရမည်။ |
| P2-02 | Standard-library API stability policy | Partial | Public module တိုင်းတွင် stability label၊ deprecation period၊ semantic-versioning rule နှင့် platform support matrix ရှိရမည်။ |
| P2-03 | LSP/VS Code semantic parity | Partial | Rename၊ nested/module-aware indexing၊ async-aware completion/hover နှင့် canonical parser/AST coverage ကို test လုပ်ရမည်။ |
| P2-04 | Learning/reference documentation ခွဲခြားခြင်း | Partial | Beginner guide၊ syntax reference၊ specification၊ stdlib reference၊ package author guide၊ runtime internals နှင့် deployment/security docs များ navigation နှင့် verified-version metadata ပါရမည်။ |

## လုပ်ဆောင်မည့်အစီအစဉ်

၁။ **P0-06:** ပြီးစီးပြီး။ Cargo-authoritative version validator၊ deterministic evidence၊ negative drift regression harness၊ bilingual policy documentation၊ CI gate နှင့် release-preflight enforcement များကို အကောင်အထည်ဖော်ပြီးဖြစ်သည်။
၂။ **P1-05-A:** ပြီးစီးပြီး။ Fixed-seed property/fuzz replay၊ parser/JSON/lockfile/registry/memory/async durable failure corpus၊ replay evidence နှင့် CI artifact gate များကို အကောင်အထည်ဖော်ပြီးဖြစ်သည်။ P1-05 ၏ broader fuzz နှင့် platform extension များကို သီးခြား track လုပ်မည်။
၃။ **P0-01-A:** ပထမ executable parity slice အဖြစ် ပြီးစီးပြီး။ Six-case native/legacy policy matrix၊ normalized report၊ migration guidance နှင့် CI artifact gate များကို အကောင်အထည်ဖော်ပြီးဖြစ်သည်။ Broader legacy inventory ကို သီးခြား track လုပ်မည်။
၄။ **P0-02-A:** Ownership expansion slice အဖြစ် အကောင်အထည်ဖော်ပြီး။ Bilingual machine-readable index တွင် stable rule ID ၂၇ ခု၊ unique ID/domain validation၊ fixture/test ownership၊ compatibility/deprecation template နှင့် release-preflight contract gate များ ပါဝင်သည်။ ကျန် fragmented rule အားလုံးအတွက် index ကို ဆက်လက်ချဲ့ထွင်ရန် ကျန်ရှိသည်။
၅။ **P1-03:** ပြီးစီးပြီး။ Registry redaction၊ fail-closed၊ traversal၊ provenance၊ key-rotation၊ yanked-release နှင့် end-to-end locked-cache tests များသည် signed tag၊ commit၊ workflow၊ HTTPS source၊ checksum၊ signing fingerprint အပြည့်အစုံ၊ trusted-fingerprint allowlist၊ adversarial signed-provenance mutation rejection၊ yanked candidate skip၊ malformed-yanked rejection၊ stable exact/range all-yanked diagnostics၊ manifest requirement ကိုက်ညီမှု၊ offline cache reuse နှင့် tampered lock/cache rejection တို့ကို enforce လုပ်ထားသည်။
၆။ **P0-04:** Weak-reference၊ closure/process-wide telemetry၊ arbitrary-cycle reclamation နှင့် tracing-collector design ကျန်ရှိမှုများကိုသာ ဆက်လုပ်ရန်။
၇။ **P1-01/P1-04:** ပြီးစီးပြီး။ ဘာသာနှစ်မျိုး gradual-typing baseline ကို မှတ်တမ်းတင်ပြီး clean-machine locked install/build verifier ကို executable နှင့် deterministic အဖြစ် ပြီးစီးထားသည်။
၈။ **P2-02/P2-03/P2-04:** Stdlib policy၊ tooling parity နှင့် documentation navigation ပြီးစီးရန်။
၉။ **P2-01:** Parser/runtime မပြောင်းမီ traits/composition RFC ကို ရေးသားပြီး review လုပ်ရန်။

## Release policy

အဆင့်တိုင်းတွင် pinned CI toolchain ဖြင့် strict Clippy၊ formatter၊ full native test suite၊ သက်ဆိုင်ရာ conformance tests၊ English/Burmese documentation parity နှင့် `git diff --check` အားလုံး အောင်မြင်ရမည်။ သက်ဆိုင်ရာ acceptance criteria နှင့် CI gates များ မပြီးမချင်း release tag အသစ် မထုတ်ရ။
