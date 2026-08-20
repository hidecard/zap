# Zap v0.8.0 — Developer Tooling Foundation

Zap v0.8.0 သည် OOP audit patch များအပြီး developer workflow ကို ပိုမိုယုံကြည်စိတ်ချရစေရန် tooling foundation ထည့်သွင်းထားသော release ဖြစ်သည်။ Runtime language core သည် v0.7.x ၏ OOP၊ collections၊ file helpers၊ modules နှင့် native CLI capabilities များကို ဆက်လက်ထိန်းသိမ်းထားသည်။

## အသစ်ပါဝင်သော CLI Commands

```bash
zap lint main.zp
zap check --json .
```

`zap lint` သည် source file အတွင်း tabs၊ trailing whitespace နှင့် အလွန်ရှည်သော lines များကို စစ်ဆေးပေးသည်။ Issue ရှိလျှင် file နှင့် line number ပါသော diagnostic ပြပြီး non-zero exit status ပြန်ပေးသည်။

`zap check --json` သည် `zap.toml`၊ package name၊ version နှင့် entry file ကို machine-readable JSON အဖြစ် ပြန်ပေးသည်။ Editor၊ CI နှင့် automation scripts များတွင် အသုံးပြုနိုင်သည်။

```json
{"ok":true,"project":"my-zap-project 0.8.0 (main: main.zp)"}
```

မမှန်သော project ဖြစ်ပါက—

```json
{"ok":false,"kind":"ProjectError","message":"cannot read zap.toml: No such file or directory","error":"cannot read zap.toml: No such file or directory"}
```

## Standard Library

v0.8.0 တွင် အောက်ပါ helpers များ ဆက်လက်ပါဝင်သည်။

| Group | Functions |
|---|---|
| Collections | `is_empty`၊ `sum`၊ `reverse`၊ `sort`၊ `get` |
| Files | `read_text`၊ `write_text`၊ `read_lines`၊ `write_lines` |
| Paths | `path_join`၊ `basename`၊ `dirname`၊ `exists` |
| Time/environment | `now`၊ `sleep`၊ `env`၊ `has_env` |
| Math/text/JSON | `abs`၊ `min`၊ `max`၊ `pow`၊ `sqrt`၊ `upper`၊ `lower`၊ `json`၊ `from_json` |

## OOP Audit Stability

v0.8.0 သည် v0.7.1 တွင် ပြင်ဆင်ထားသော OOP behavior များကို ဆက်လက်ထိန်းသိမ်းသည်။ Unknown class၊ unknown parent၊ inherited constructor၊ child method override၊ empty class နှင့် property state persistence များကို regression tests ဖြင့် စစ်ဆေးထားသည်။

## Verification

Release မတင်မီ `cargo test`၊ `cargo check`၊ `git diff --check`၊ `zap lint` နှင့် `zap check --json` ကို CI/local workflow တွင် run လုပ်ရမည်။ Native regression tests 25 ခု pass ဖြစ်ရမည်။ Cross-platform release archives များတွင် Linux x86_64၊ macOS ARM64 နှင့် Windows x86_64 binaries၊ README၊ docs၊ examples နှင့် SHA-256 checksum ပါဝင်သည်။

## မပါဝင်သေးသောအရာများ

Structured `Result` propagation၊ line/column diagnostics၊ HTTP client၊ `async/await`၊ task cancellation၊ channels၊ package lockfile၊ package registry၊ language server နှင့် bytecode VM များသည် v0.8.0 တွင် မပါဝင်သေးပါ။ ၎င်းတို့ကို `ROADMAP_0.8.0.md` တွင် implementation order ဖြင့် ဖော်ပြထားသည်။
