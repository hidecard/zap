# Zap v0.8.0 Roadmap နှင့် v0.9.0 Plan

## v0.8.0 Implemented Scope

Zap v0.8.0 သည် language runtime ကို အလျင်အမြန် feature များဖြင့် မတည်ငြိမ်စေဘဲ developer experience ကို ခိုင်မာစေရန် ရည်ရွယ်သည်။

| Feature | Status |
|---|---|
| OOP class/object/method | Implemented |
| Inheritance နှင့် inherited constructor | Implemented |
| Collection helpers | Implemented |
| Line-based file I/O | Implemented |
| `zap run` | Implemented |
| `zap lint` | Implemented |
| `zap check --json` | Implemented |
| Optional variable annotations | Implemented |
| Function parameter/return annotations နှင့် runtime checks | Implemented in current v0.9 work |
| Static signature validation in `zap check` | Implemented in current v0.9 work |
| Structured JSON diagnostics (`kind`, `message`, `error`, `file`, `line`, `column`) | Implemented in current v0.9 work |
| Checked integer arithmetic နှင့် modulo-by-zero diagnostics | Implemented in audit patch |
| Structured `Result` | Planned |
| `async/await` နှင့် channels | Planned |
| HTTP client | Planned |
| Package registry/lockfile | Planned |

## Audit Findings နှင့် Production Gaps

လက်ရှိ v0.9 development တွင် function parameter နှင့် return annotations များကို `number`၊ `text`၊ `bool`၊ `list`၊ `map`၊ `none` နှင့် `any` အတွက် runtime call checks ဖြင့် အကောင်အထည်ဖော်ထားပြီး typed-function regression test ပါဝင်သည်။ `zap check` တွင် function signature၊ argument count/type၊ literal variable နှင့် ရိုးရိုး nested expression များကို static စစ်ဆေးပြီး `zap check --json` သည် `kind`၊ `message`၊ `error`၊ `file`၊ `line` နှင့် `column` fields ပါသော structured diagnostic ပြန်ပေးသည်။

v0.8.0 comparative audit အရ Zap ၏ ကျန်ရှိနေသေးသော အဓိက production gaps များမှာ control-flow type narrowing၊ generic/nullable/union type system၊ explicit import/export modules၊ lockfile/checksum package workflow၊ HTTP/encoding/regex standard-library coverage၊ test filtering/coverage/fuzzing၊ နှင့် cancellation ပါသော async tasks ဖြစ်သည်။ Source-location ပါသော structured JSON diagnostics နှင့် ရိုးရိုး function-call static inference များကို v0.9 development တွင် ပြီးစီးထားသည်။ Python ၏ typing နှင့် callable/generic annotations၊ JavaScript ၏ import/export/dynamic modules၊ Go ၏ modules/testing/fuzzing/profiling၊ Dart ၏ Futures/Streams/isolates တို့က mature ecosystem baseline အဖြစ် reference လုပ်ထားသည်။

Audit patch တွင် signed integer arithmetic ကို checked operations အဖြစ် ပြောင်းလဲထားပြီး addition၊ subtraction၊ multiplication overflow နှင့် division/modulo by zero များကို process panic မဖြစ်စေဘဲ user-facing runtime error အဖြစ် ပြန်ပေးသည်။ Native regression tests သည် လက်ရှိ 25 unit tests နှင့် 47 integration tests အဖြစ် စုစုပေါင်း 72 tests ရှိပြီး အားလုံး pass ဖြစ်သည်။

## v0.9.0 Priority Order

### 1။ Structured Error Model

Runtime error များကို string တစ်ခုတည်းအဖြစ် မပြန်ဘဲ `kind`၊ `message`၊ `file`၊ `line` နှင့် `column` ပါသော error value အဖြစ် ပြန်ပေးမည်။ နောက်ပိုင်းတွင်—

```zap
let result = read_text("config.json")
if result.ok:
    say result.value
else:
    say result.error.message
```

ပုံစံဖြင့် recoverable errors ကို အသုံးပြုနိုင်ရန် ရည်ရွယ်သည်။

### 2။ Type Checking တိုးချဲ့မှု

လက်ရှိ function parameter/return annotation၊ function-call argument count/type၊ literal variable နှင့် ရိုးရိုး nested expression များကို `zap check` မှ static စစ်ဆေးပြီးဖြစ်သည်။ နောက်ထပ် control-flow narrowing၊ reassignment inference၊ generic collection နှင့် nullable/union type များကို တိုးချဲ့မည်။ Runtime တွင် dynamic behavior ကို မဖျက်ဘဲ static diagnostics အဖြစ် ဆက်လက်တည်ဆောက်မည်။

```zap
fn add(a: number, b: number) -> number:
    return a + b
```

### 3။ Test Tooling

`zap test` တွင် test filter၊ fail-fast option၊ summary output နှင့် watch mode ထည့်မည်။ CI တွင် JSON test report ထုတ်နိုင်ရန်လည်း စီစဉ်မည်။

### 4။ Module/Package Foundation

Local modules အပြီး package metadata၊ dependency name/version နှင့် lockfile schema ကို အရင်သတ်မှတ်မည်။ Remote registry မစတင်မီ reproducible local build နှင့် checksum verification ကို အရင်တည်ဆောက်မည်။

### 5။ Async I/O Boundary

`async/await` ကို language syntax အဖြစ် မထည့်မီ task lifecycle၊ timeout၊ cancellation နှင့် error propagation semantics များကို သတ်မှတ်မည်။ Shared mutable threads များထက် tasks နှင့် message passing ကို ဦးစားပေးမည်။

### 6။ HTTP Client

Timeout၊ status code၊ headers၊ body size limit နှင့် TLS error များပါသော minimal HTTP client ကို standard library module အဖြစ် စတင်မည်။ Web server framework သည် နောက် release အတွက် ဖြစ်သည်။

## မထည့်သင့်သေးသောအရာများ

Generics၊ multiple inheritance၊ shared mutable threads၊ reflection၊ native mobile bindings နှင့် package registry ကို semantics နှင့် security boundary များ မတည်ငြိမ်မီ မထည့်သင့်သေးပါ။

## Detailed Implementation Checklist

အလုပ်တစ်ခုချင်းစီ၏ implementation order၊ release mapping၊ security checklist နှင့် acceptance criteria အပြည့်အစုံကို [`TODO_ZAP_MM.md`](TODO_ZAP_MM.md) တွင် ဖတ်ရှုနိုင်သည်။

## Release Acceptance Criteria

- Native tests များ pass ဖြစ်ရမည်။
- Linux၊ Windows နှင့် macOS ARM64 build workflow အောင်မြင်ရမည်။
- README၊ Syntax Guide၊ Burmese Learning Guide နှင့် Usage Guide တို့တွင် implemented/proposed boundary တူညီရမည်။
- CLI error များသည် non-zero exit code ပြန်ပေးရမည်။
- Release archive တစ်ခုစီတွင် SHA-256 checksum ပါရမည်။
- Arithmetic overflow နှင့် zero-division regression tests pass ဖြစ်ရမည်။
- Audit တွင် ဖော်ပြထားသော implemented/proposed feature boundary ကို Burmese guide တွင်လည်း တူညီစွာ ထိန်းသိမ်းရမည်။
