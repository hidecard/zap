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
| Structured `Result` | Planned |
| `async/await` နှင့် channels | Planned |
| HTTP client | Planned |
| Package registry/lockfile | Planned |

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

လက်ရှိ variable annotation အပြင် function parameter နှင့် return type များကို `zap check` မှ စစ်ဆေးမည်။ Runtime တွင် dynamic behavior ကို မဖျက်ဘဲ static diagnostics အဖြစ် စတင်မည်။

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

## Release Acceptance Criteria

- Native tests များ pass ဖြစ်ရမည်။
- Linux၊ Windows နှင့် macOS ARM64 build workflow အောင်မြင်ရမည်။
- README၊ Syntax Guide၊ Burmese Learning Guide နှင့် Usage Guide တို့တွင် implemented/proposed boundary တူညီရမည်။
- CLI error များသည် non-zero exit code ပြန်ပေးရမည်။
- Release archive တစ်ခုစီတွင် SHA-256 checksum ပါရမည်။
