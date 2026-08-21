# Zap နောက်ဆက်တွဲ Engineering TODO Plan

**အခြေခံအခြေအနေ:** Zap v2.1.6 verified release ဖြစ်ပြီး P0-03 Structured Diagnostics ကို `master` branch တွင် ပြီးစီးထားသည်။

**ရည်ရွယ်ချက်:** Stable diagnostic contract ပြီးစီးပြီးနောက် ဆက်လက်အကောင်အထည်ဖော်ရမည့် အစီအစဉ်ကို သတ်မှတ်ရန် ဖြစ်သည်။ ပြီးစီးပြီးသား release အလုပ်များကို ထပ်မတွက်ဘဲ ကျန်ရှိနေသော TODO register ကို executable milestone များအဖြစ် ခွဲထားသည်။

**လက်ရှိတိုးတက်မှု:** P0-04 ၏ ပထမ implementation slice ပြီးစီးထားသည်။ Tracked object allocation/deallocation statistics၊ bounded `memory_stats()` diagnostics၊ cycle-safe value validation၊ deterministic value limits၊ regression tests နှင့် bilingual memory-contract documentation များ ထည့်သွင်းပြီးဖြစ်သည်။ P0-05-A နှင့် P0-05-B လည်း ပြီးစီးထားသည်။ Descriptive deterministic `async_capabilities()` builtin၊ catalog entry၊ runtime/AST tests၊ typed resource-limit preflight validation နှင့် TCP request-size admission checks များ ထည့်သွင်းပြီးဖြစ်သည်။ P0-05-C cross-platform matrix coverage၊ executor-backed language scheduling နှင့် language-level cancellation/timeout controls တို့သာ ကျန်ရှိသည်။

## နောက်ထပ် P0/P1 execution queue

P0-04 ၏ ပထမ slice ပြီးစီးပြီးနောက် နောက်အလုပ်များကို feature အရွယ်အစားအစား runtime risk အလိုက် အောက်ပါအတိုင်း စီစဉ်ထားသည်။

| အစီအစဉ် | Work package | Implementation slice | Acceptance evidence |
|---|---|---|---|
| P0-05-A | Async boundary capability contract | Deterministic `async_capabilities()` report ထည့်ရန်၊ executor/worker/network/process/cancellation boundary များကို document လုပ်ရန်၊ language-level cancellation/timeout မထောက်ပံ့သေးကြောင်း explicit ပြရန် | Stable capability map၊ bilingual contract၊ AST/runtime tests နှင့် eager future များကို executor-backed ဟု မဆိုခြင်း |
| P0-05-B | Async resource-limit validation | Worker/task/read/socket/process limits များကို admission မတိုင်မီ စစ်ရန်၊ deadline/output/cancellation behavior ကို fail-closed ထိန်းရန် | Invalid-limit tests၊ deterministic errors နှင့် bounded process cleanup |
| P0-05-C | Cross-platform async matrix | Linux၊ Windows နှင့် macOS target များတွင် path၊ process၊ socket၊ deadline၊ cancellation နှင့် output-limit behavior များ စစ်ရန် | Target-native CI evidence သို့မဟုတ် versioned documented limitation |
| P1-05-A | Replayable verification layers | Parser၊ JSON၊ lockfile၊ registry၊ memory နှင့် async boundary များအတွက် fixed-seed property/fuzz replay နှင့် durable failure fixtures ထည့်ရန် | CI artifact တွင် seed/input ပါပြီး repeated run များ deterministic ဖြစ်ခြင်း |
| P0-01-A | Native/legacy parity report | Legacy fixture များကို ခွဲခြားပြီး ခွင့်မပြုထားသော semantic drift ကို gate လုပ်ရန် | Versioned parity matrix နှင့် executable CI report |
| P0-02-A | Specification ownership index | Public syntax၊ typing၊ runtime၊ limit နှင့် error rule များကို canonical bilingual sections/fixtures များနှင့် map လုပ်ရန် | Normative rule တစ်ခုမျှ owner မရှိဘဲ မကျန်ခြင်းနှင့် rule တိုင်း fixture owner ရှိခြင်း |

P0/P1 safety gates များပြီးနောက် P2-02 standard-library stability၊ P2-03 LSP/VS Code parity နှင့် P2-04 documentation navigation ကို ဆက်လုပ်မည်။ P2-01 traits/composition သည် conformance/specification contract များ မပြီးမချင်း RFC-only milestone အဖြစ် ဆက်ရှိမည်။

## အစီအစဉ်ရေးဆွဲမှု မူဝါဒ

Zap သည် reliability foundation မှ ecosystem feature များသို့ အဆင့်လိုက် ဆက်သွားရမည်။ Memory ownership၊ deterministic execution၊ conformance နှင့် documentation contract များ မခိုင်မာမချင်း asynchronous syntax အသစ်၊ HTTP server framework၊ package publishing သို့မဟုတ် traits implementation များကို မစသင့်သေးပါ။ ပြီးစီးသည့်အလုပ်တိုင်းတွင် code၊ focused regression tests၊ public contract ပြောင်းလဲပါက ဘာသာနှစ်မျိုး documentation နှင့် repository release gates များ ပါဝင်ရမည်။

> **စည်းမျဉ်း:** Linux တွင်သာ အလုပ်လုပ်ခြင်းကို feature complete ဟု မသတ်မှတ်ရပါ။ Semantics၊ failure behavior၊ limits၊ cross-platform expectations နှင့် compatibility status များကို document/test လုပ်ပြီးမှသာ feature complete ဟု သတ်မှတ်ရမည်။

## ဦးစားပေးအဆင့်များ

| Priority | လုပ်ငန်း | လက်ရှိအခြေအနေ | အဓိကရလဒ် | သတ်မှတ် milestone |
|---|---|---|---|---|
| 1 | **P0-04 Memory နှင့် reference-cycle စာချုပ်** | Partial | Ownership policy၊ bounded memory behavior နှင့် deterministic cycle-breaking diagnostics | M1 — Runtime safety |
| 2 | **P0-05 Deterministic နှင့် production async boundary** | Partial | Deterministic executor နှင့် production I/O/blocking work ကို တိတိကျကျ ခွဲခြားသတ်မှတ်ခြင်း | M1 — Runtime safety |
| 3 | **P1-05 Conformance၊ property နှင့် fuzz layers** | Partial | ပိုမိုကျယ်ပြန့်သော panic-free၊ deterministic နှင့် platform-aware verification | M2 — Verification |
| 4 | **P0-01 Native/legacy conformance စာချုပ်** | Partial | Native behavior ကို canonical သတ်မှတ်ပြီး executable legacy-parity report နှင့် migration policy ရှိခြင်း | M2 — Verification |
| 5 | **P0-02 ပေါင်းစည်းထားသော language specification** | Partial | Syntax၊ typing၊ runtime behavior၊ compatibility နှင့် version decision များအတွက် semantic index တစ်ခုတည်းရှိခြင်း | M2 — Verification |
| 6 | **P2-02 Standard-library API stability policy** | Partial | Public API တိုင်းတွင် stability label၊ deprecation rule၊ versioning rule နှင့် platform record ပါခြင်း | M3 — Tooling နှင့် documentation |
| 7 | **P2-03 LSP နှင့် VS Code semantic parity** | Partial | Rename၊ module-aware indexing၊ async-aware editor behavior နှင့် parser/AST parity ကို release-test လုပ်နိုင်ခြင်း | M3 — Tooling နှင့် documentation |
| 8 | **P2-04 Learning/reference documentation ခွဲခြားခြင်း** | Partial | Learner၊ reference၊ specification၊ package-author၊ runtime နှင့် deployment content များကို versioned navigation ဖြင့် ခွဲခြားခြင်း | M3 — Tooling နှင့် documentation |
| 9 | **P2-01 Composition နှင့် traits/interfaces RFC** | Deferred | Parser/runtime ပြောင်းလဲမှု မစမီ reviewed design ရှိခြင်း | M4 — Language design |

## M1 — Runtime safety

### M1.1 — P0-04 Memory contract ပြီးစီးအောင်လုပ်ခြင်း

ပထမအလုပ်မှာ tracing garbage collector ကို အလျင်စလို မထည့်ဘဲ လက်ရှိ `Rc<RefCell>` ownership policy ကို တိုင်းတာနိုင်အောင် ပြုလုပ်ခြင်း ဖြစ်သည်။ ဘယ် value များသည် reference-counted ဖြစ်သလဲ၊ ဘယ် boundary သည် single-threaded ဖြစ်သလဲ၊ object fields များကို ဘယ်လို clear လုပ်သလဲ၊ shutdown သို့မဟုတ် error recovery အတွင်း ဘယ် operation များ safe ဖြစ်သလဲဆိုသည်ကို document လုပ်ရမည်။

| ID | လုပ်ငန်း | ပြီးစီးမှုအထောက်အထား |
|---|---|---|
| M1-04-01 | Live object count၊ tracked allocation count နှင့် မရရှိနိုင်သော metric များကို ရှင်းလင်းဖော်ပြသော stable heap-statistics shape သတ်မှတ်ရန် | ဘာသာနှစ်မျိုး contract document နှင့် deterministic API/test fixture |
| M1-04-02 | ရွေးချယ်ထားသော runtime ownership boundary တွင် allocation/deallocation counters ထည့်ရန် | Repeated execution တွင် counter output တည်ငြိမ်ပြီး raw address သို့မဟုတ် secret မထွက်ခြင်း |
| M1-04-03 | Weak reference ကို support လုပ်မည်၊ မလုပ်နိုင်ကြောင်း explicit သတ်မှတ်မည်၊ သို့မဟုတ် internal diagnostic boundary သို့သာ ကန့်သတ်မည်ဆိုသည်ကို ဆုံးဖြတ်ရန် | ရှင်းလင်းသော design decision၊ unsupported use အတွက် error behavior နှင့် မတော်တဆ thread-safety claim မရှိခြင်း |
| M1-04-04 | `clear_object_fields` နှင့် release behavior ကို စမ်းသပ်သော closure-capture နှင့် object-cycle fixtures ထည့်ရန် | Cycle breaking နှင့် cleanup ပြီးနောက် stable statistics ကို regression tests ဖြင့် သက်သေပြခြင်း |
| M1-04-05 | String၊ list၊ map၊ object နှင့် total execution state အတွက် memory-limit behavior သတ်မှတ်ရန် | Limit table၊ stable error codes နှင့် malformed/oversized-input coverage |

Milestone ပြီးစီးရန် runtime သည် ownership model ကို ရှင်းပြနိုင်ရမည်၊ ကတိပြုထားသော bounded conditions များကို detect/report လုပ်နိုင်ရမည်၊ tracing collection မပေးထားဘဲ ပေးထားသကဲ့သို့ မဖော်ပြရမည်၊ full native suite ကို pass ဖြစ်ရမည်။

### M1.2 — P0-05 Async boundary ပြီးစီးအောင်လုပ်ခြင်း

Async documentation နှင့် implementation တွင် deterministic single-threaded scheduling နှင့် controlled worker resources သို့မဟုတ် operating-system I/O သုံးသော production adapter များကို တိတိကျကျ ခွဲခြားရမည်။ Task lifecycle နှင့် cancellation semantics မတည်ငြိမ်မချင်း `async fn`/`await` syntax အကျယ်ပြန့် မထည့်သင့်သေးပါ။

| ID | လုပ်ငန်း | ပြီးစီးမှုအထောက်အထား |
|---|---|---|
| M1-05-01 | Deterministic executor၊ blocking adapter၊ network adapter၊ process adapter နှင့် cancellation behavior များအတွက် async boundary table တစ်ခုထုတ်ရန် | English/Burmese documentation parity နှင့် linked examples |
| M1-05-02 | Task admission၊ poll budget၊ join၊ timeout၊ cancellation precedence၊ repeated join နှင့် panic-to-error behavior သတ်မှတ်ရန် | Normative contract နှင့် focused unit tests |
| M1-05-03 | ဘယ် operation များ cancellable ဖြစ်သလဲ၊ foreign blocking call များထဲမှ ဘယ်အရာကို interrupt မလုပ်နိုင်သလဲ မှတ်တမ်းတင်ရန် | Explicit limitation tests နှင့် deterministic diagnostics |
| M1-05-04 | Worker count၊ task count၊ output bytes၊ deadline နှင့် child-process cleanup အတွက် resource-limit tests ထည့်ရန် | Cross-platform regression evidence နှင့် supported boundary အတွင်း orphan process မကျန်ကြောင်း သက်သေပြခြင်း |
| M1-05-05 | Local registry-service deployment နှင့် public production deployment အတွက် release checklist ထည့်ရန် | TLS၊ supervision၊ sandbox၊ quota၊ credential နှင့် egress တာဝန်များကို ခွဲခြားဖော်ပြသော deployment documentation |

## M2 — Verification နှင့် language contracts

### M2.1 — P1-05 Verification layers တိုးချဲ့ခြင်း

လက်ရှိ deterministic corpus သည် အခြေခံကောင်းတစ်ခုဖြစ်သော်လည်း ကျန် scope ကို မစီမံနိုင်သော fuzz target တစ်ခုတည်းအဖြစ် မထားသင့်ပါ။ Panic freedom၊ deterministic rejection၊ input-size limits နှင့် platform-specific behavior များကို bounded job များအဖြစ် ခွဲလုပ်ရမည်။

| ID | လုပ်ငန်း | ပြီးစီးမှုအထောက်အထား |
|---|---|---|
| M2-05-01 | Parser၊ JSON၊ lockfile၊ registry နှင့် standard-library အတွက် fixed seed/replay support ပါသော long-running fuzz targets ထည့်ရန် | CI တွင် failing seed ကို ပြန် run နိုင်ပြီး minimized input ကို artifact အဖြစ် သိမ်းနိုင်ခြင်း |
| M2-05-02 | Object cycle၊ oversized values နှင့် repeated module execution အတွက် allocator/heap-level tests ထည့်ရန် | Memory regression artifacts နှင့် bounded runtime behavior |
| M2-05-03 | Windows နှင့် macOS အတွက် path၊ process၊ newline၊ permission နှင့် archive cases ထည့်ရန် | Target-native CI evidence သို့မဟုတ် documented reproducible limitation |
| M2-05-04 | Deterministic ordering၊ diagnostic normalization၊ checksum verification နှင့် lockfile round trip အတွက် property tests ထည့်ရန် | Named၊ replayable ဖြစ်ပြီး CI တွင် run နိုင်ခြင်း |
| M2-05-05 | Security/parser regression တစ်ခုချင်းစီတွင် durable fixture နှင့် rationale ရှိစေရန် failure-corpus ownership policy ထည့်ရန် | Corpus index၊ test naming convention နှင့် changelog procedure |

### M2.2 — P0-01 Native/legacy conformance ပြီးစီးအောင်လုပ်ခြင်း

Legacy behavior နှင့် ကွာခြားမှုကို repository က ပြနိုင်ပြီးမှသာ Native execution ကို canonical ဟု သတ်မှတ်သင့်သည်။ Conformance layer သည် difference များကို broad smoke tests အတွင်း ဖုံးကွယ်မထားဘဲ accepted၊ rejected၊ compatible၊ deprecated နှင့် intentionally divergent အဖြစ် report ပြုလုပ်ရမည်။

| ID | လုပ်ငန်း | ပြီးစီးမှုအထောက်အထား |
|---|---|---|
| M2-01-01 | Legacy fixtures များကို normative၊ compatibility၊ deprecated သို့မဟုတ် rejected အဖြစ် ခွဲခြားရန် | Versioned parity matrix |
| M2-01-02 | Native နှင့် legacy fixtures များကို normalized output ဖြင့် run သော executable conformance command ထည့်ရန် | CI artifact အဖြစ် အသုံးပြုနိုင်သော deterministic report |
| M2-01-03 | Native-only ဖြစ်ရန် ရည်ရွယ်ထားသော behavior များအတွက် migration guidance ရေးရန် | Bilingual migration notes နှင့် examples |
| M2-01-04 | ခွင့်ပြုချက်မရှိသော parity drift အသစ်များအတွက် release gate ထည့်ရန် | CI သည် drift တွင် fail ဖြစ်ပြီး သက်ဆိုင်ရာ fixture ကို ပြပေးခြင်း |

### M2.3 — P0-02 Consolidated specification ပြီးစီးအောင်လုပ်ခြင်း

Language specification သည် semantic truth ၏ owner ဖြစ်လာရမည်။ Syntax guide၊ usage guide၊ runtime notes၊ type-checking matrix နှင့် release notes များသည် learner-friendly ဖြစ်နိုင်သော်လည်း language rule များ တိတ်တဆိတ် မကွဲပြားရပါ။

| ID | လုပ်ငန်း | ပြီးစီးမှုအထောက်အထား |
|---|---|---|
| M2-02-01 | Syntax/runtime/type rule တစ်ခုချင်းစီကို canonical specification section နှင့် mapping လုပ်သော rule index တည်ဆောက်ရန် | Public rule တစ်ခုမျှ owner မရှိဘဲ မကျန်ခြင်း |
| M2-02-02 | Precedence၊ error၊ path၊ module၊ resource-limit နှင့် async rule များကို fragmented files မှ ရွှေ့ သို့မဟုတ် cross-link လုပ်ရန် | Bilingual links နှင့် version ownership |
| M2-02-03 | Normative rule များဘေးတွင် conformance fixture ID ထည့်ရန် | Specification rule တစ်ခုချင်းစီတွင် passing သို့မဟုတ် intentionally failing fixture ကို ရည်ညွှန်းနိုင်ခြင်း |
| M2-02-04 | Future semantics change အတွက် compatibility/deprecation template ထည့်ရန် | Template ကို changelog နှင့် migration document တွင် အသုံးပြုထားခြင်း |

## M3 — Tooling နှင့် documentation

### M3.1 — Standard-library stability policy

Public standard-library API တစ်ခုချင်းစီတွင် experimental၊ provisional၊ stable၊ deprecated သို့မဟုတ် platform-specific ဖြစ်ကြောင်း label ထည့်ရမည်။ API record တစ်ခုချင်းစီတွင် input limits၊ output limits၊ timeout behavior၊ error codes၊ determinism နှင့် platform differences ပါရမည်။

ပထမ deliverable သည် machine-readable သို့မဟုတ် တစ်ပြေးညီဖွဲ့စည်းထားသော API inventory ဖြစ်ရမည်။ ဒုတိယ deliverable သည် public helper တိုင်းတွင် လိုအပ်သော fields များ ပါ/မပါ စစ်ဆေးသော documentation နှင့် CI check ဖြစ်ရမည်။ တတိယ deliverable သည် old behavior ကို သတ်မှတ်ထားသောကာလအတွင်း ထိန်းသိမ်းပေးမည့် သို့မဟုတ် breaking release decision ကို explicit မှတ်တမ်းတင်မည့် deprecation workflow ဖြစ်ရမည်။

### M3.2 — LSP နှင့် VS Code parity

Editor tooling သည် language ကို ဒုတိယအဓိပ္ပာယ်ဖွင့်ဆိုခြင်း မပြုဘဲ canonical parser နှင့် shared diagnostic contract ကို အသုံးပြုရမည်။ နောက်အလုပ်များအဖြစ် rename၊ module-aware indexing၊ nested symbol range၊ async-aware completion/hover နှင့် stable diagnostic snapshots ကို ဦးစားပေးရမည်။

Acceptance အတွက် LSP response တည်ငြိမ်ရမည်၊ CLI ၏ one-based span မှ LSP zero-based range သို့ မှန်ကန်စွာ ပြောင်းရမည်၊ stable `ZAP-*` diagnostic code များ ထိန်းသိမ်းရမည်၊ imported file indexing သည် traversal-safe ဖြစ်ရမည်၊ ဖွင့်ထားသော local module နှင့် မဖွင့်ထားသော local module နှစ်မျိုးလုံးအတွက် fixtures ရှိရမည်။

### M3.3 — Learning/reference documentation ခွဲခြားခြင်း

Documentation ကို learner material၊ syntax reference၊ language specification၊ standard-library reference၊ package-author guidance၊ runtime internals နှင့် deployment/security operations ဟူ၍ ခွဲခြားရမည်။ Section တစ်ခုချင်းစီတွင် verified Zap version ကို ပြပြီး canonical semantic rules သို့ ပြန်ချိတ်ရမည်။

Beginner သည် internals မဖတ်ဘဲ လေ့လာနိုင်ရမည်၊ experienced user သည် normative syntax နှင့် error behavior ကို လွယ်ကူစွာ ရှာနိုင်ရမည်၊ operator သည် local fixture နှင့် public production service ကို မရောဘဲ deployment တာဝန်များကို သိနိုင်ရမည်။ ထိုအခြေအနေသို့ရောက်မှသာ documentation milestone ပြီးစီးသည်ဟု သတ်မှတ်ရမည်။

## M4 — Language-design RFC

### M4.1 — Composition နှင့် traits/interfaces

Traits၊ interfaces၊ composition နှင့် method resolution များကို reviewed RFC မပြီးမချင်း deferred ထားရမည်။ RFC သည် လက်ရှိ single-inheritance model နှင့် composition ကို နှိုင်းယှဉ်ရမည်၊ method lookup နှင့် visibility သတ်မှတ်ရမည်၊ missing/conflicting implementation diagnostics ကို ရှင်းပြရမည်၊ inheritance မှ migration ကို ဖော်ပြရမည်၊ dynamic dispatch သို့မဟုတ် static conformance ကို ရွေးချယ်ရမည်။

RFC တွင် ဘာသာနှစ်မျိုး terminology၊ examples၊ rejected alternatives၊ compatibility impact နှင့် explicit version decision မပါမချင်း parser သို့မဟုတ် runtime implementation မစရပါ။

## Milestone တစ်ခုချင်းစီအတွက် Release-gate checklist

| Gate | လိုအပ်သောရလဒ် |
|---|---|
| Formatting | Pinned toolchain ဖြင့် Rust formatting check pass ဖြစ်ရမည် |
| Static quality | `-D warnings` ပါသော strict Clippy pass ဖြစ်ရမည် |
| Tests | Full native unit/integration suite နှင့် သက်ဆိုင်ရာ focused fixtures များ pass ဖြစ်ရမည် |
| Determinism | Repeated run များတွင် output၊ ordering၊ diagnostics နှင့် archive များ တည်ငြိမ်ရမည် |
| Security | Malformed၊ oversized၊ traversal၊ secret-redaction နှင့် checksum cases များ fail-closed ဖြစ်ရမည် |
| Documentation | English/Burmese public contract များ တစ်ပြေးညီဖြစ်ရမည် |
| Compatibility | Version impact၊ migration path၊ deprecation status နှင့် changelog entry များ explicit ဖြစ်ရမည် |
| Repository hygiene | `git diff --check` pass ဖြစ်ပြီး generated/secret file များ commit မလုပ်ရပါ |

## ချက်ချင်းဆက်လုပ်ရမည့်အလုပ်

နောက်ထပ် implementation slice သည် **P0-05-C Cross-platform async matrix** ဖြစ်ရမည်။ Path၊ process၊ socket၊ deadline၊ cancellation နှင့် output-limit behavior များအတွက် target-native သို့မဟုတ် reproducible limitation evidence ကို ထည့်ရမည်။ P0/P1 safety gates များ မပြီးမချင်း traits implementation သို့မဟုတ် async syntax အကျယ်ပြန့်ကို မစသင့်သေးပါ။

## ဆက်စပ်မှတ်တမ်းများ

ဤ plan သည် [`PDF_REMAINING_TODO_MM.md`](PDF_REMAINING_TODO_MM.md)၊ v2.1 roadmap [`V2.1_ROADMAP_MM.md`](V2.1_ROADMAP_MM.md) နှင့် consolidated language specification [`LANGUAGE_SPEC_MM.md`](LANGUAGE_SPEC_MM.md) များကို ဆက်လက်တိုးချဲ့ထားခြင်း ဖြစ်သည်။
