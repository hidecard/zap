# Zap B3 Milestone TODO Plan

## ရည်ရွယ်ချက်

B3 milestone ၏ အဓိကရည်ရွယ်ချက်မှာ B2 တွင် တည်ဆောက်ပြီးသော canonical AST၊ type-checker နှင့် typed-IR အခြေခံများကို package/build/test-runner foundation အဖြစ် စနစ်တကျ ချိတ်ဆက်ရန် ဖြစ်သည်။ B3 သည် parser/typechecker feature အသစ်များထည့်ခြင်းထက် **ပြန်လည်တည်ဆောက်နိုင်မှု၊ dependency graph၊ schema stability၊ deterministic verification နှင့် developer tooling** ကို အဓိကထားရမည်။

## လက်ရှိအခြေအနေ

B1 lexer/parser differential နှင့် arbitrary-block coverage gates များ pass ဖြစ်နေပြီး B2 typecheck၊ typed-IR၊ flow-sensitive၊ recursive-CFG နှင့် generic end-to-end gates များလည်း pass ဖြစ်နေသည်။ B2 typed-IR candidate benchmark baseline သည် local Linux x86_64 environment တွင် candidate gate အတွက် iteration တစ်ကြိမ်လျှင် ခန့်မှန်းအားဖြင့် 1.06–1.08 seconds နှင့် peak RSS 53–55 MiB ဖြစ်သည်။ ဤတန်ဖိုးများကို absolute limit မဟုတ်ဘဲ regression baseline အဖြစ်သာ အသုံးပြုရမည်။

## B3 TODO အစီအစဉ်

| Priority | TODO | Acceptance gate | အထောက်အထား |
|---|---|---|---|
| P0 | Canonical AST schema ကို versioned contract အဖြစ် သတ်မှတ်ရန် | `verify_b3_canonical_ast_schema.sh` | schema fields၊ version နှင့် representative fixtures |
| P0 | Package/build/test-runner foundations ကို reproducible လုပ်ရန် | `verify_b3_build_plan_10.sh` | clean checkout မှ build၊ test၊ artifact output |
| P0 | Module dependency graph နှင့် cycle diagnostics တည်ငြိမ်စေရန် | `verify_b3_dependency_graph_11.sh` | graph ordering၊ duplicate/cycle cases |
| P1 | Typed-IR benchmark ကို CI regression contract ထဲ ထည့်ရန် | benchmark schema/provenance/regression gates | elapsed time၊ peak RSS၊ commit/toolchain provenance |
| P1 | CLI/LSP နှင့် canonical AST diagnostics parity ကို ဆက်လက်ခိုင်မာစေရန် | existing B3/B4 diagnostic gates | stable code၊ line၊ column၊ message |
| P1 | Rust-free bootstrap seed pipeline ကို package boundary များနှင့် ချိတ်ဆက်ရန် | `verify_b3_*` bootstrap gates | no-Cargo seed execution evidence |
| P2 | Warning/debt inventory ကို issue-linked register အဖြစ် ထိန်းသိမ်းရန် | CI lint and documentation consistency | owner၊ risk၊ planned milestone၊ evidence |
| P2 | Benchmark output ကို baseline comparison နှင့် ပေါင်းစည်းရန် | `check_benchmark_regression.sh` | 20% regression threshold၊ missing-suite rejection |

## Implementation sequence

ပထမအဆင့်တွင် canonical AST schema နှင့် dependency graph ကို အရင်တည်ငြိမ်စေရမည်။ ဒုတိယအဆင့်တွင် clean build နှင့် test-runner foundation ကို ထည့်သွင်းရမည်။ တတိယအဆင့်တွင် typed-IR benchmark ကို CI artifact/provenance နှင့် ချိတ်ဆက်ပြီး peak RSS နှင့် elapsed-time regression ကို စစ်ဆေးရမည်။ နောက်ဆုံးတွင် CLI/LSP parity၊ Rust-free bootstrap နှင့် documentation consistency ကို release gate အဖြစ် စုစည်းရမည်။

## Definition of done

B3 ကို ပြီးစီးသည်ဟု သတ်မှတ်ရန် clean checkout မှ build/test runner များ deterministic ဖြစ်ရမည်၊ canonical AST schema version contract မပျက်ရမည်၊ dependency graph ၏ cycle/duplicate cases များအတွက် stable diagnostics ရှိရမည်၊ typed-IR benchmark သည် provenance sidecar နှင့်အတူ ထုတ်ပေးရမည်၊ baseline ထက် 20% ကျော် performance regression မရှိရမည်၊ peak RSS တန်ဖိုးများကို platform/toolchain metadata နှင့်အတူ သိမ်းဆည်းရမည်။ ထို့အပြင် `cargo check`၊ `cargo clippy -- -D warnings` နှင့် B1/B2 regression gates အားလုံး pass ဖြစ်ရမည်။

## Known technical debt policy

`while_else_diagnostic` ကဲ့သို့ compatibility diagnostics များကို parser hot path မှ ခွဲထားရမည်။ Token cursor helper များသည် EOF တွင် panic မဖြစ်ဘဲ explicit error node သို့မဟုတ် diagnostic ပြန်ပေးရမည်။ Dead compatibility code ကို ချက်ချင်းဖယ်ရှားမည့်အစား owner၊ removal condition နှင့် regression evidence ကို မှတ်တမ်းတင်ပြီး B3/B4 boundary တွင်သာ ဖယ်ရှားရမည်။ CI action deprecation notices များသည် code warning မဟုတ်သောကြောင့် action version policy နှင့် သီးခြားစီ စောင့်ကြည့်ရမည်။

## လက်ရှိ blockers

လက်ရှိ local Rust warning checks တွင် compiler warning မတွေ့ပါ။ GitHub Actions တွင် Node.js 20 forced-runtime deprecation annotation သည် upstream action runtime notice ဖြစ်ပြီး repository source warning မဟုတ်ပါ။ B3 ၏ နောက်ထပ်အလုပ်သည် canonical schema၊ build plan၊ dependency graph နှင့် benchmark regression integration ကို priority အလိုက် ဆက်လက်လုပ်ဆောင်ရန် ဖြစ်သည်။

## Progress update — canonical AST schema versioning

B3 ၏ canonical AST schema versioning အပိုင်းကို စတင်အကောင်အထည်ဖော်ပြီးဖြစ်သည်။ `bootstrap/contracts/AST_SCHEMA.toml` တွင် schema name `zap.ast`၊ stable version `1`၊ envelope/node/diagnostic required fields နှင့် minor-additive compatibility policy ကို သတ်မှတ်ထားသည်။ `bootstrap/contracts/VERSIONS.toml` သည် ထို contract ကို path ဖြင့် link လုပ်ထားပြီး schema version ကို language/compiler version များနှင့် သီးခြားထိန်းသိမ်းထားသည်။

`verify_b3_canonical_ast_schema.sh` သည် contract metadata နှင့် parser မှ ထုတ်ပေးသော `zap.ast` envelope ၏ `schema_version = 1` ကို တိုက်ရိုက်စစ်ဆေးသည်။ Release flow တွင် Unix/Windows package နှစ်မျိုးလုံး၌ `contracts/AST_SCHEMA.toml` ပါဝင်ရမည်ဟု သတ်မှတ်ထားပြီး archive verification နှင့် release preflight တွင်လည်း required input အဖြစ် ထည့်သွင်းထားသည်။

## Updated TODO status

| အခြေအနေ | အလုပ် |
|---|---|
| ပြီးစီး | Versioned canonical AST schema v1 contract နှင့် B3 validation gate |
| ပြီးစီး | Release preflight နှင့် Unix/Windows archive integration |
| ဆက်လုပ်ရန် P0 | Canonical AST schema ကို typed-IR serialization နှင့် cross-version reader tests များဖြင့် ချိတ်ဆက်ရန် |
| ဆက်လုပ်ရန် P0 | Package/build/test-runner foundation ကို clean checkout နှင့် release artifact အထိ ပြီးစီးအောင်လုပ်ရန် |
| ဆက်လုပ်ရန် P0 | Dependency graph cycle/duplicate diagnostics ကို versioned contract အဖြစ် တည်ငြိမ်စေရန် |
| ဆက်လုပ်ရန် P1 | Schema v1 fixture matrix၊ unknown-field reader compatibility နှင့် breaking-change test cases ထည့်ရန် |
| ဆက်လုပ်ရန် P1 | B2 typed-IR performance/RSS benchmark ကို CI regression artifact နှင့် release preflight ထဲ ချိတ်ဆက်ရန် |
| ဆက်လုပ်ရန် P1 | CLI/LSP diagnostic parity နှင့် Rust-free bootstrap seed pipeline ကို B3 gate အဖြစ် စုစည်းရန် |
| ဆက်လုပ်ရန် P2 | CI action runtime deprecation notice နှင့် remaining compatibility/dead-code debt ကို owner/retirement condition ဖြင့် register လုပ်ရန် |

Schema contract ပြင်ဆင်မှုများပြီးနောက် B1 parser၊ B2 aggregate နှင့် B3 canonical schema gate များကို ပြန်လည်စစ်ဆေးရမည်။ Release tag မတင်မီ `release_preflight.sh` ကို correct package version နှင့် clean tree ဖြင့် run လုပ်ရန်လိုအပ်သည်။
