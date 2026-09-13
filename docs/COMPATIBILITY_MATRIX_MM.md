# Zap လုံးချုံညီသုံးနိုင်မှု ကွန်ပလ accretion

ဤစာရွက်စာတမ်းသည် Zap အစိတ်အပိုင်းများနှင့် ပလတ်ဖောင်းများအကြား ဗားရှင်း လုံးချုံညီသုံးနိုင်မှုကို မှတ်တမ်းတင်ထားပါသည်။

## လက်ရှိ ပြန်လည်လွှာရှင်မှာ

| အစိတ်အပိုင်း | ဗားရှင် | မှတ်ချက် |
|--------------|---------|----------|
| Compiler | 2.11.18 | Native Rust implementation |
| Language | 2.11.18 | AST schema v1 |
| Standard Library | 2.11.18 | 9 ဒိုမိန်းအတွက် 68 entries |
| Package Format | 2.11.18 | `zap.toml` manifest |
| LSP | 2.11.18 | `zap lsp` |
| Bootstrap Stage | B4 | Rust-free full-language certified |

## ပလတ်ဖောင်း ထောက်ခံမှု

| ပလတ်ဖောင်း | အကွဲအ�ကား | Binary | CI |
|-------------|----------------|--------|-----|
| Linux x86_64 | ✅ Supported | `zap` | ✅ |
| macOS x86_64 | ✅ Supported | `zap` | ✅ |
| macOS ARM64 | ✅ Supported | `zap` | ✅ |
| Windows x86_64 | ✅ Supported | `zap.exe` | ✅ |
| Linux ARM64 | ⏳ လာမည် | - | ❌ |
| Windows ARM64 | ⏳ လာမည် | - | ❌ |

## Bootstrap ကတ်စတုတ်များ

| ကတ်စတု | ဗားရှင် | အခြေအနေ |
|---------|---------|----------|
| AST Schema | 1 | Stable |
| Token Schema | 1 | Stable |
| Diagnostic Schema | 1 | Stable |
| Typed-IR Schema | 1 | Reference-only |
| Artifact Schema | 1 | Stable |

## Standard Library လုံးချုံညီသုံးနိုင်မှု

| ဒိုမိန်း | လုံးချုံညီသုံးနိုင်မှု | ဥပမာ |
|---------|---------------------|-------|
| async | runtime-dependent | `task_spawn`, `task_join` |
| collections | pure | `append`, `count`, `enumerate` |
| filesystem | external-io | `read_text`, `write_text` |
| json | pure | `from_json`, `json` |
| logging | pure | `log_json`, `log_record` |
| math | pure | `abs`, `max`, `min`, `pow`, `sqrt` |
| network | external-io | `http_get`, `url_parse` |
| system | pure/external-io | `dirname`, `env`, `config_dir` |
| text | pure | `contains`, `join`, `len`, `split`, `trim` |
| time | input-deterministic/runtime-dependent | `utc_now`, `duration_between` |

## ဘာသာစကားမျက်နှာ အခြေအနေ

| ရိုက်ခတ်မှု | အခြေအနေ | Bootstrap Stage |
|---------|--------|-----------------|
| Lexer/parser | ✅ ပြီးပြီ | B1 |
| Type checker | ✅ ပြီးပြီ | B2 |
| Generic types | ✅ ပြီးပြီ | B2 |
| Flow analysis | ✅ ပြီးပြီ | B2 |
| Canonical AST bridge | ✅ ပြီးပြီ | B3 |
| Typed-IR producer | ✅ ပြီးပြီ | B3 |
| Bytecode lowering | ✅ ပြီးပြီ | B3 |
| VM execution | ✅ ပြီးပြီ | B3 |
| Package build | ✅ ပြီးပြီ | B3 |
| Self-hosting | 🔄 Not-certified | B4 |

## ချိုးဖောက်မှု ပြောင်းလဲမှု မူဝနဲ့

- ချိုးဖောက်မှု ပြောင်းလဲမှုများသည် major version bump လိုအပါသည်
- Deprecated features သည် အနည်းဆုံး 2 minor versions သတိပေးခြင်းဖြင့်
- Security fixes သည် supported versions သို့ backport လုပ်နိုင်ပါသည်
- AST schema ပြောင်းလဲမှုများသည် migration guide လိုအပါသည်

## ထောက်ခံမှု မူဝနဲ့

| Version | Supported | Security Fixes | Bug Fixes |
|---------|-----------|----------------|-----------|
| 2.x | ✅ Yes | ✅ Yes | ✅ Yes |
| 1.x | ❌ No | ❌ No | ❌ No |
