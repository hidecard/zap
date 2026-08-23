# Zap Language Comparison Audit — Current Capability Record

## 1. Scope and baseline

This document records the current Zap capability baseline for the **v2.2.6** release candidate and the maintenance review. Publication remains subject to the complete local and GitHub release gates. It replaces the obsolete v0.8.0/v0.9.0 comparison and must not be read as a claim that every future item listed here has been implemented. The working-tree maintenance fixes described in Section 4 remain subject to the complete local and GitHub release gates before publication of the next release.

ဤစာတမ်းသည် **v2.2.6** release candidate နှင့် maintenance review အတွက် Zap ၏ လက်ရှိစွမ်းရည်အခြေအနေကို မှတ်တမ်းတင်ထားခြင်း ဖြစ်ပါသည်။ Publication မပြုမီ local နှင့် GitHub release gate အားလုံးကို ဖြတ်သန်းရမည် ဖြစ်ပါသည်။ ယခင် v0.8.0/v0.9.0 အခြေခံ audit အချက်အလက်များကို အစားထိုးထားပြီး အောက်တွင်ဖော်ပြထားသော future work အားလုံး ပြီးစီးပြီးဖြစ်သည်ဟု မဆိုလိုပါ။ Working tree ထဲရှိ maintenance ပြင်ဆင်ချက်များသည် နောက်ထပ် release မထုတ်မီ local နှင့် GitHub release gates အားလုံးကို ဖြတ်သန်းရမည် ဖြစ်ပါသည်။

| Item | Current record |
|---|---|
| Repository | [hidecard/zap](https://github.com/hidecard/zap), default branch `master` |
| Prepared baseline | [v2.2.6 release candidate](https://github.com/hidecard/zap/releases/tag/v2.2.6) |
| Runtime | Native Rust executable with canonical AST execution and compatibility-only legacy line execution for retained legacy function records |
| Language surface | Variables, functions, closures, classes, single inheritance, local modules, Result/Option, `?`, JSON, bounded filesystem/network/process helpers, and deterministic language tasks |
| Memory/concurrency boundary | Single-threaded `Rc`/`RefCell` object model, run-owned logical budgets, explicit `clear_object_fields()` cycle policy, and eager scheduled-value async semantics |
| Tooling | `zap check`, `check --json`, `test`, `fmt`, `lint`, lock/install/update commands, registry commands, LSP, and VS Code package |
| LSP boundary | Full synchronization using `change: 1`; range edits are rejected; rename is file-local and cross-file rename is not supported |
| Security posture | `ZAP_UNTRUSTED=1` denies sensitive capabilities but is not an OS sandbox; deployment still needs least privilege, egress controls, quotas, and process isolation |

## 2. Current strengths

Zap’s strongest characteristics are its intentionally small indentation-based syntax, standalone native distribution, explicit runtime errors, bounded public I/O, deterministic replay and benchmark evidence, bilingual documentation, and disciplined release validation. The runtime now uses per-run `ExecutionContext` and `RuntimeState` boundaries for module-cache isolation, import-cycle tracking, execution depth, logical memory/task/output budgets, object lifecycle accounting, and task state.

Zap ၏ အဓိကအားသာချက်များမှာ indentation-based syntax ရိုးရှင်းခြင်း၊ standalone native executable အဖြစ် ဖြန့်ချိနိုင်ခြင်း၊ explicit runtime errors၊ public I/O များကို ကန့်သတ်ထားခြင်း၊ deterministic replay နှင့် benchmark evidence၊ bilingual documentation နှင့် စနစ်တကျ release validation ရှိခြင်းတို့ ဖြစ်ပါသည်။ Runtime တွင် per-run `ExecutionContext` နှင့် `RuntimeState` များကို အသုံးပြု၍ module-cache isolation၊ import-cycle tracking၊ execution depth၊ logical memory/task/output budget၊ object lifecycle accounting နှင့် task state များကို run တစ်ခုချင်းစီအလိုက် ခွဲခြားထားပါသည်။

| Dimension | Zap today | Practical implication |
|---|---|---|
| Syntax | Beginner-oriented indentation and compact expressions | Easy to prototype and teach, but not a replacement for a large ecosystem language |
| Type checking | Optional annotations, runtime validation, known call-site checks, literal/nested inference, and bounded control-flow narrowing | Useful diagnostics exist; a complete typed compiler and cross-module type system remain future work |
| Memory | Host Rust memory with `Rc<RefCell>` values, logical budgets, and explicit cycle cleanup | Single-threaded ownership is explicit; automatic weak references or tracing collection are not provided |
| Async | Eager scheduled values, deterministic polling, task join/cancel/timeout, and bounded adapters | The model is controlled and testable, not a general production reactor or multi-thread guarantee |
| Modules/packages | Local module graph, registry foundation, checksums, lockfile, and dependency commands | Package integrity is stronger than the old baseline, but the ecosystem remains small |
| Tooling | CLI, LSP foundation, VS Code package, replay, benchmark, parity, and policy gates | Release discipline is strong; formatter/linter/test-runner depth is still limited |

## 3. Comparison with established languages

The comparison below is deliberately capability-oriented. Python offers a large dynamic ecosystem and external typing tools; JavaScript/TypeScript offers a mature module and web ecosystem; Rust offers ownership, traits, and compile-time safety; Go offers integrated modules, testing, fuzzing, coverage, and profiling; Dart offers Futures, Streams, isolates, and client/UI integration. Zap’s differentiator is a smaller, bilingual, native, safety-oriented runtime boundary rather than breadth or ecosystem size.[1] [2] [3] [4] [5]

အောက်ပါနှိုင်းယှဉ်ချက်သည် capability အပေါ် အခြေခံထားပါသည်။ Python တွင် dynamic ecosystem နှင့် external typing tools ကြီးမားပြီး JavaScript/TypeScript တွင် mature module/web ecosystem ရှိပါသည်။ Rust တွင် ownership၊ traits နှင့် compile-time safety၊ Go တွင် modules/testing/fuzzing/coverage/profiling workflow၊ Dart တွင် Futures/Streams/isolates နှင့် client/UI integration များ ရှိပါသည်။ Zap ၏ ထူးခြားချက်မှာ feature breadth သို့မဟုတ် ecosystem အရွယ်အစားမဟုတ်ဘဲ သေးငယ်သော bilingual native runtime နှင့် safety-oriented boundary ဖြစ်ပါသည်။

| Dimension | Zap | Python | JavaScript/TypeScript | Rust | Go | Dart |
|---|---|---|---|---|---|---|
| Learning | Small indentation-based syntax and standalone executable | Easy entry and very large ecosystem | Familiar web model with broad semantics | Steeper ownership model | Compact compiled language | Strong tooling and client ecosystem |
| Types | Optional annotations and partial static/runtime checking | Dynamic runtime with external checkers | Dynamic JS or TypeScript static layer | Strong static types, generics, traits, ownership | Strong static types, interfaces, generics | Sound null safety and generics |
| Concurrency | Deterministic single-thread task boundary and bounded adapters | `asyncio`, threads, processes | Promise/event-loop ecosystem | Async, threads, channels, and futures | Goroutines, channels, and mature tooling | Event loop, Futures, Streams, isolates |
| Packages | Local modules, registry, checksums, and lockfile foundation | PyPI and virtual-environment ecosystem | npm, ESM/CommonJS, bundlers | Cargo and crates.io | Go modules and publishing | pub.dev and library ecosystem |
| Tooling | CLI/LSP/VS Code with reproducibility gates | Test, type, lint, profile, and IDE ecosystem | Test runners, bundlers, and browser tools | Cargo test/fmt/clippy and compiler diagnostics | Integrated test/fuzz/coverage/profile tools | Analyzer, formatter, DevTools, Flutter tools |
| Production fit | Controlled scripts and language/runtime foundation | Broad automation, data, and web use | Web-native breadth | Systems and backend strength | Backend, cloud, and CLI strength | Client/mobile/UI strength |

## 4. Confirmed maintenance findings and current status

The v2.2.6 maintenance review confirmed four correctness/security-boundary defects and one bounded-operation hardening opportunity, plus a cross-platform process-cleanup hardening change and a registry-test race fix. The maintenance working tree fixes `read_lines` and `write_lines` so both canonical AST and retained legacy execution route through workspace confinement; makes `build --locked` require an existing `zap.lock` even for dependency-free projects; rejects malformed and out-of-range URL ports; makes test discovery ignore symlink entries and visit canonical directories only once; and bounds `sleep` and `pow` with explicit stable limits while using checked exponentiation-by-squaring.

v2.2.6 maintenance review တွင် correctness/security boundary ဆိုင်ရာ defect လေးခုနှင့် bounded-operation hardening တစ်ခုကို အတည်ပြုခဲ့ပြီး cross-platform process cleanup hardening နှင့် registry-test race fix ကိုလည်း ထည့်သွင်းထားပါသည်။ Maintenance working tree တွင် canonical AST နှင့် retained legacy execution နှစ်မျိုးလုံး၏ `read_lines`/`write_lines` ကို workspace confinement ဖြင့် ကာကွယ်ထားပါသည်။ Dependency မရှိသော project များအတွက်ပင် `build --locked` သည် ရှိပြီးသား `zap.lock` ကို မဖြစ်မနေလိုအပ်စေပါသည်။ Malformed နှင့် range ကျော် URL port များကို reject လုပ်ပြီး test discovery သည် symlink entry များကို skip လုပ်ကာ canonical directory တစ်ခုကို တစ်ကြိမ်သာ visit လုပ်ပါသည်။ `sleep` နှင့် `pow` တွင် explicit stable limit များ ထည့်ပြီး checked exponentiation-by-squaring ကို အသုံးပြုထားပါသည်။

| Finding | Status in the maintenance working tree | Contract |
|---|---|---|
| Filesystem line-I/O confinement | Fixed with focused traversal regression coverage | Relative paths remain inside the active workspace; existing explicit limits remain unchanged |
| Strict `build --locked` | Fixed with project-level regression coverage | Locked mode requires `zap.lock`; ordinary `build`, `check`, and dependency-free non-locked install behavior remain separate |
| Malformed URL ports | Fixed with parser regressions | Empty, non-numeric, and out-of-range ports return deterministic parser errors |
| Symlink-loop test discovery | Fixed with canonical visited-set and symlink skipping | Test collection is bounded against directory-link cycles and remains deterministic |
| Unbounded `sleep`/`pow` | Fixed with shared limits and checked exponentiation | `sleep` is limited to 60,000 ms and `pow` exponent is limited to 1,000,000; overflow remains an explicit error |
| Registry-test environment race | Fixed as maintenance reliability | Secure/insecure HTTP fixture tests share one environment mutex |

## 5. Explicit limitations and deferred work

The following items remain outside the complete guarantee of this maintenance release. Universal descendant cleanup across every host configuration, race-resistant descriptor-relative filesystem APIs, DNS-to-connection pinning, AST/typed-IR replacement of the heuristic checker, syntax-aware formatting, per-test timeout/coverage/profiling, and dependency-version upgrades require separate design or lockfile work. v2.2.6 does add isolated process groups on Unix and recursive tree-termination requests on Windows for the supported process timeout/cancellation paths, but this remains best-effort host integration rather than a universal OS sandbox guarantee. The current RustSec scan found six advisories in the locked graph involving `idna 0.4.0`, `rustls-webpki 0.102.8`, and dev-only `time 0.3.30`; those upgrades are recorded for a separate dependency-maintenance pass because the repository release policy explicitly forbids `cargo update` and lockfile regeneration in the focused release work.

အောက်ပါအရာများသည် ဤ maintenance release တွင် universal guarantee အဖြစ် မပါဝင်သေးပါ။ Host configuration အားလုံးအတွက် universal descendant cleanup၊ race-resistant descriptor-relative filesystem API၊ DNS-to-connection pinning၊ heuristic checker ကို AST/typed-IR checker အဖြစ် ပြောင်းလဲခြင်း၊ syntax-aware formatting၊ per-test timeout/coverage/profiling နှင့် dependency-version upgrade များသည် သီးခြား design သို့မဟုတ် lockfile work လိုအပ်ပါသည်။ v2.2.6 တွင် Unix အတွက် isolated process group နှင့် Windows အတွက် recursive tree-termination request များကို supported process timeout/cancellation path တွင် ထည့်သွင်းထားသော်လည်း ၎င်းသည် best-effort host integration သာဖြစ်ပြီး universal OS sandbox guarantee မဟုတ်ပါ။ လက်ရှိ RustSec scan တွင် locked graph အတွင်း `idna 0.4.0`၊ `rustls-webpki 0.102.8` နှင့် dev-only `time 0.3.30` ဆိုင်ရာ advisory ခြောက်ခု တွေ့ရှိခဲ့ပါသည်။ Focused release policy က `cargo update` နှင့် lockfile regeneration ကို တားမြစ်ထားသဖြင့် ထို dependency upgrades များကို သီးခြား dependency-maintenance pass အဖြစ် မှတ်တမ်းတင်ထားပါသည်။

Framework/Web/App/IoT ecosystem work is also explicitly deferred. No `zap-host`, `zap-web`, `zap-app`, `zap-edge`, `zap-io`, Axum, Tauri, Wasm, MQTT, production reactor, multi-thread runtime, framework syntax, UI, adapter, host, or ABI implementation belongs in the current maintenance branch. That work may be planned later on a separate branch only after the user explicitly starts the framework phase.

Framework/Web/App/IoT ecosystem အလုပ်များကိုလည်း အတိအလင်း defer လုပ်ထားပါသည်။ လက်ရှိ maintenance branch တွင် `zap-host`၊ `zap-web`၊ `zap-app`၊ `zap-edge`၊ `zap-io`၊ Axum၊ Tauri၊ Wasm၊ MQTT၊ production reactor၊ multi-thread runtime၊ framework syntax၊ UI၊ adapter၊ host သို့မဟုတ် ABI implementation မရှိပါ။ User က framework phase ကို သီးခြားစတင်ရန် တိကျစွာ တောင်းဆိုပြီးနောက်မှသာ future separate branch တွင် planning ပြုလုပ်နိုင်ပါသည်။

## 6. Recommended priority after this maintenance release

The next core-maintenance pass should first plan the locked dependency upgrades and a modern advisory gate, then address the residual filesystem race boundary, DNS-to-connection pinning, and stronger host-specific process-cleanup guarantees. Only after those contracts are specified should the project consider deeper test-runner, formatter/linter, typed-IR, or incremental LSP work. Traits/interfaces/composition, web/mobile/IoT hosts, and other framework ecosystem proposals remain future branch work rather than current release tasks.

ဤ maintenance release နောက်ပိုင်းတွင် ပထမဆုံး locked dependency upgrade နှင့် modern advisory gate ကို စီစဉ်သင့်ပြီး၊ ထို့နောက် ကျန်ရှိသော filesystem race boundary၊ DNS-to-connection pinning နှင့် host-specific process-cleanup guarantee များကို ဖြေရှင်းသင့်ပါသည်။ ထို contracts များ သတ်မှတ်ပြီးမှသာ test-runner၊ formatter/linter၊ typed-IR သို့မဟုတ် incremental LSP အလုပ်များကို ဆက်လက်စဉ်းစားသင့်ပါသည်။ Traits/interfaces/composition၊ web/mobile/IoT host များနှင့် အခြား framework ecosystem proposal များသည် current release task မဟုတ်ဘဲ future separate branch work ဖြစ်ပါသည်။

## 7. Assessment

Zap has moved beyond a prototype: its native runtime, canonical AST path, per-run state isolation, explicit memory policy, bounded I/O, registry integrity, deterministic task boundary, LSP foundation, bilingual documentation, and release evidence form a credible early-stage language/runtime foundation. It should nevertheless be described as a controlled foundation rather than a production-ready general-purpose ecosystem. The repository’s own security posture is accurate: capability denial is not an OS sandbox, and hostile multi-tenant or downloaded-code execution requires external isolation.

Zap သည် prototype အဆင့်ကို ကျော်လွန်လာပြီး native runtime၊ canonical AST path၊ per-run state isolation၊ explicit memory policy၊ bounded I/O၊ registry integrity၊ deterministic task boundary၊ LSP foundation၊ bilingual documentation နှင့် release evidence များဖြင့် early-stage language/runtime foundation ကောင်းတစ်ခု ဖြစ်လာပါသည်။ သို့သော် production-ready general-purpose ecosystem ဟု မဖော်ပြသင့်သေးဘဲ controlled foundation ဟုသာ သတ်မှတ်သင့်ပါသည်။ Capability denial သည် OS sandbox မဟုတ်ကြောင်းနှင့် hostile multi-tenant သို့မဟုတ် downloaded-code execution အတွက် external isolation လိုအပ်ကြောင်း repository ၏ security posture က မှန်ကန်စွာ သတ်မှတ်ထားပါသည်။

## References

[1]: https://docs.python.org/3/library/typing.html "Python typing documentation"

[2]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Modules "MDN JavaScript modules"

[3]: https://doc.rust-lang.org/book/ "The Rust Programming Language"

[4]: https://go.dev/doc/ "Go documentation"

[5]: https://dart.dev/language/concurrency "Dart concurrency documentation"

[6]: https://github.com/hidecard/zap "Zap repository"

[7]: https://github.com/hidecard/zap/releases/tag/v2.2.6 "Zap v2.2.6 release"
