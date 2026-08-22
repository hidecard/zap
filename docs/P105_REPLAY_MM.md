# P1-05-A Replayable Verification Layers

## ရည်ရွယ်ချက်

P1-05-A သည် လက်ရှိ panic-free corpus စစ်ဆေးမှုများကို inline test data ပေါ်တွင်သာ မမှီခိုဘဲ ပြန်လည် replay လုပ်နိုင်အောင် ပြင်ဆင်ပါသည်။ Replay တစ်ကြိမ်စီတွင် positive decimal `ZAP_CORPUS_SEED`၊ တည်ငြိမ်သော category အစဉ်၊ lexical အစဉ်ဖြင့်ဖတ်ထားသော durable fixture များနှင့် seed အပေါ်မူတည်သော deterministic permutation ကို အသုံးပြုပါသည်။ Default seed သည် `20260821` ဖြစ်ပါသည်။

> Failure report တစ်ခုသည် seed၊ fixture path၊ SHA-256 digest နှင့် တူညီသော case ကို ပြန်လည်ဖန်တီးနိုင်မည့် input bytes များကို မှတ်တမ်းတင်ထားမှသာ လက်တွေ့အသုံးဝင်သော replay evidence ဖြစ်ပါသည်။

## Corpus အမျိုးအစားများ

| Category | Boundary | Durable fixture ဥပမာများ | စစ်ဆေးသည့် boundary |
|---|---|---|---|
| `parser` | Lexer နှင့် AST rejection | မပြီးဆုံးသော string၊ မမှန်သော operator၊ မပိတ်ရသေးသော group၊ malformed annotation | `tokenize_with_spans` နှင့် `ast::parse_program` ကို panic-free replay နှစ်ကြိမ်စီ run လုပ်ခြင်း |
| `json` | Tagged JSON conversion | မသိသော variant၊ payload မပါခြင်း၊ မမှန်သော variant type၊ nested extra fields | `serde_json` decode ပြီး `json_to_value` သို့ ပို့ခြင်း |
| `lockfile` | Resolved lockfile parsing | မထောက်ပံ့သော version၊ field မပြည့်စုံခြင်း၊ invalid escape၊ traversal ဆန်သော name | `parse_resolved_lockfile` |
| `registry` | Registry index parsing | JSON ပျက်ခြင်း၊ packages မပါခြင်း၊ duplicate package၊ traversal ဆန်သော package name | `parse_index_bytes` |
| `memory` | Bounded value graph validation | node budget ကျော်လွန်သော value graph | `Value::validate_memory_limits` |
| `async` | Deterministic scheduler budget | poll budget သုည၊ တစ်၊ နှစ် | `AsyncRuntime::run_with_budget` |

Fixture များကို [`corpus/p1-05`](../corpus/p1-05) အောက်တွင် ထားရှိပြီး review လုပ်ရလွယ်ကူသော text file အဖြစ် ရေးထားပါသည်။ Fixture တစ်ခုစီတွင် owner category တစ်ခုသာ ရှိပြီး generated ephemeral input ဖြင့် မတိတ်တဆိတ် အစားထိုးခြင်း မပြုရပါ။

## Fixed-seed replay

Shared native replay helper သည် `ZAP_CORPUS_SEED` ကို ဖတ်ပြီး မသတ်မှတ်ထားပါက `20260821` ကို အသုံးပြုပါသည်။ Category တစ်ခုစီတွင် deterministic Fisher–Yates-style permutation ကို သုံးပါသည်။ Valid seed သည် positive decimal integer ဖြစ်ရပါမည်။ Seed တူပြီး checkout တူပါက fixture အစဉ်နှင့် outcome တူညီရမည်။ Seed ပြောင်းပါက fixture content နှင့် assertion မပြောင်းဘဲ replay အစဉ်သာ deterministic အဖြစ် ပြောင်းလဲရမည်။

Local entrypoint သည် အောက်ပါအတိုင်း ဖြစ်ပါသည်။

```text
ZAP_CORPUS_SEED=20260821 scripts/test_p105_replay.sh
```

Script သည် `target/p105-replay.log` ကိုရေးပြီး seed၊ category၊ relative fixture path၊ SHA-256 digest နှင့် base64-encoded input bytes များကို record တစ်ကြောင်းစီတွင် ထည့်သွင်းပါသည်။ Mutable temporary file များကို မမှီခိုဘဲ CI failure ကို ပြန်လည်စမ်းသပ်ရန် ဤ evidence များသည် အနည်းဆုံးလိုအပ်ချက် ဖြစ်ပါသည်။

## CI နှင့် failure-corpus policy

`scripts/test_p105_layers.sh` သည် လက်ရှိ CLI mutation corpus မတိုင်မီ replay gate ကို run လုပ်ပါသည်။ GitHub Actions သည် documentation တွင်သတ်မှတ်ထားသော seed ကိုပေးပြီး `target/p105-replay.log` ကို `zap-p105-replay-<commit>` artifact အဖြစ် upload လုပ်ပါသည်။ Failure report တွင် commit၊ seed၊ category၊ fixture path၊ digest နှင့် failure ကို ပြန်ဖန်တီးနိုင်သော အငယ်ဆုံး durable fixture ပါဝင်ရပါမည်။

Security၊ parser၊ memory သို့မဟုတ် async regression အသစ်တစ်ခု ဖြစ်ပါက owner category အောက်တွင် fixture အသစ်ထည့်ရမည်၊ replay test သို့မဟုတ် သက်ဆိုင်ရာ domain test တွင် focused assertion ထည့်ရမည်၊ public contract ပြောင်းလဲပါက bilingual changelog entry ထည့်ရမည်။ Fixture များတွင် secret၊ host-specific absolute path၊ timestamp၊ memory address သို့မဟုတ် nondeterministic network data မပါရပါ။

## M2-VERIFY-01 bounded replay job

Bounded verification job သည် single replay pass ကို အကြိမ်ရေကန့်သတ်ထားသော repeatable CI workload အဖြစ် တိုးချဲ့ထားပါသည်။ Entry point သည် အောက်ပါအတိုင်း ဖြစ်ပါသည်။

```text
ZAP_CORPUS_SEED=20260821 ZAP_CORPUS_ROUNDS=12 scripts/test_m2_verify_replay.sh
```

`ZAP_CORPUS_ROUNDS` ၏ default သည် 12 ဖြစ်ပြီး 1–64 အတွင်းသာ ခွင့်ပြုကာ အပြင်ဘက်တန်ဖိုးများကို fail-closed ပြုလုပ်ပါသည်။ Round တစ်ခုစီတွင် category ခြောက်မျိုးလုံးပါသော corpus ကို native replay test မှတစ်ဆင့် run လုပ်ပြီး SHA-256 outcome digest တစ်ခု ထုတ်ပေးပါသည်။ Fixture တစ်ခုချင်းစီသည် default အားဖြင့် 64 KiB ထက် မကျော်ရ၊ corpus အားလုံး၏ စုစုပေါင်းသည် 8 MiB ထက် မကျော်ရပါ။ ထို limit များကို `ZAP_CORPUS_MAX_FIXTURE_BYTES` နှင့် `ZAP_CORPUS_MAX_TOTAL_BYTES` မှတစ်ဆင့် explicit ပြောင်းလဲမှသာ ခွင့်ပြုပါသည်။ Missing/empty corpus directory၊ မမှန်ကန်သော numeric setting၊ မပြည့်စုံသော round marker၊ fixture count ပြောင်းလဲမှု၊ မမှန်သော digest နှင့် ထပ်ခါတလဲလဲ outcome digest ကွာခြားမှုများကို job က reject လုပ်ပါသည်။

Job သည် seed၊ round count၊ fixture count/bytes၊ configured bounds၊ fixture-manifest digest၊ round တစ်ခုချင်းစီ၏ outcome digest နှင့် final status ပါသော `target/m2-verify-replay.tsv` ကို ရေးပါသည်။ Raw native test output ကို `target/m2-verify-replay.log` တွင် ထိန်းသိမ်းပါသည်။ CI သည် file နှစ်ခုလုံးကို `zap-m2-verify-replay-<commit>` artifact အဖြစ် upload လုပ်ပြီး release preflight သည်လည်း contract-report directory တစ်ခုတည်းအောက်တွင် bounded gate ကို run လုပ်ပါသည်။ ထိုနည်းဖြင့် unbounded fuzzing service အသစ် မထည့်ဘဲ long-running verification ကို ပြန်လည်ဖန်တီးနိုင်ပါသည်။

## Boundary နှင့် နောက်မှလုပ်မည့် scope

ဤ verification slice သည် deterministic bounded replay၊ ထပ်ခါတလဲလဲ semantic outcome comparison၊ durable regression input နှင့် fail-closed corpus-size control များကို ပေးသော်လည်း unbounded fuzzing service၊ allocator-level telemetry၊ arbitrary-cycle reclamation သို့မဟုတ် သီးခြား document လုပ်ထားသော cooperative task control များအပြင် အခြား language-level async feature များ ရှိသည်ဟု မဆိုလိုပါ။ ထိုအရာများသည် roadmap တွင် သီးခြားပိုင်ဆိုင်သည့် item များအဖြစ် ဆက်လက်ရှိပါမည်။ Replay layer သည် လက်ရှိ Rust 1.75 toolchain နှင့် ကိုက်ညီပြီး third-party fuzzing runtime အသစ် မထည့်သွင်းပါ။

## Acceptance evidence

Milestone ကို လက်ခံရန် category ခြောက်မျိုးလုံး repository မှ load လုပ်နိုင်ရမည်၊ seed တူလျှင် repeated result တူညီရမည်၊ seed အခြားတစ်ခုသုံးလျှင် deterministic အစဉ်ပြောင်းရမည်၊ malformed input များသည် panic မဖြစ်ဘဲ fail-closed ဖြစ်ရမည်၊ replay log တွင် input evidence ပါရမည်၊ bounded job သည် သတ်မှတ်ထားသော round အားလုံးကို run ပြီး outcome digest တူညီရမည်၊ မမှန်သော round/corpus-size setting များ fail-closed ဖြစ်ရမည်၊ CI နှင့် release preflight က TSV/log evidence ကို archive လုပ်ရမည်၊ full repository quality gates အားလုံး အောင်မြင်ရမည်။
