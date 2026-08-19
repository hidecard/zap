# Zap v0.9.0 Development Release Notes

> ဤစာတမ်းသည် stable release announcement မဟုတ်ဘဲ v0.9.0 အတွက် လက်ရှိ development snapshot ၏ verified scope နှင့် မပြီးသေးသောအရာများကို ဖော်ပြထားသည်။ Stable version သည် လက်ရှိတွင် v0.8.0 ဖြစ်သည်။

## အကျဉ်းချုပ်

v0.9.0 development line ၏ ပထမအဆင့်တွင် Zap ၏ function type foundation၊ static signature validation နှင့် machine-readable diagnostics ကို တိုးချဲ့ထားသည်။ ရည်ရွယ်ချက်မှာ Python၊ JavaScript၊ Rust၊ Go နှင့် Dart ကဲ့သို့ mature languages များ၏ type safety နှင့် tooling baseline များကို Zap ၏ ရိုးရှင်းသော syntax နှင့် ပေါင်းစပ်ရန် ဖြစ်သည်။

## အသစ်ထည့်သွင်းထားသောအရာများ

| အပိုင်း | လက်ရှိအခြေအနေ |
|---|---|
| Function parameter annotations | `name: type` syntax ဖြင့် runtime စစ်ဆေးနိုင်သည် |
| Return annotations | `) -> type` syntax ဖြင့် return value စစ်ဆေးနိုင်သည် |
| လက်ခံသော type names | `text`၊ `number`၊ `bool`၊ `list`၊ `map`၊ `object`၊ `none`၊ `any` |
| Static signature check | `zap check` တွင် main source အတွက် စစ်ဆေးနိုင်သည် |
| JSON diagnostics | `kind`၊ `message` နှင့် `error` fields ပါဝင်သည် |
| Arithmetic safety | Overflow နှင့် zero division/modulo များကို runtime error အဖြစ် ပြန်ပေးသည် |
| Regression coverage | Native integration tests 27 ခု pass ဖြစ်သည် |

## အသုံးပြုပုံ

```zap
fn add(a: number, b: number) -> number:
    return a + b

say add(4, 6)
```

Type မကိုက်ညီသော argument သို့မဟုတ် return value ဖြစ်ပါက runtime မှ `type mismatch` error ပြန်ပေးမည်။ Project signature ကို မသိသော annotation များပါ/မပါ စစ်ရန်—

```bash
zap check .
zap check --json .
```

ဥပမာ JSON diagnostic သည်—

```json
{
  "ok": false,
  "kind": "TypeError",
  "message": "TypeError at main.zp:1: unknown type annotation 'unknown_type'",
  "error": "TypeError at main.zp:1: unknown type annotation 'unknown_type'"
}
```

## မှန်ကန်မှုနှင့် လုံခြုံမှု ပြင်ဆင်ချက်များ

Signed integer addition၊ subtraction နှင့် multiplication များကို checked operation အဖြစ် အသုံးပြုထားသည်။ Division by zero၊ modulo by zero နှင့် integer overflow များသည် process panic မဖြစ်စေဘဲ user-facing runtime error အဖြစ် ပြန်ပေးသည်။

## Verification

Development snapshot ကို `cargo test --quiet` ဖြင့် စစ်ဆေးရာ native integration tests 27 ခုလုံး အောင်မြင်ခဲ့သည်။ `cargo check --quiet` နှင့် `git diff --check` လည်း အောင်မြင်ခဲ့သည်။ Cross-platform release workflow သည် Linux x86_64၊ macOS ARM64 နှင့် Windows x86_64 target များအတွက် ဆက်လက်အသုံးပြုနိုင်သည်။

## မပြီးသေးသောအရာများ

Function call တစ်ခုချင်းစီ၏ argument count နှင့် type ကို `zap check` မှ static inference ဖြင့် စစ်ဆေးခြင်း၊ JSON diagnostic တွင် `file`၊ `line` နှင့် `column` fields သီးခြားထည့်ခြင်း၊ structured `Result`၊ explicit import/export၊ package lockfile/registry၊ HTTP client၊ async/await၊ channels၊ task cancellation၊ coverage/fuzzing နှင့် LSP တို့သည် နောက်ဆင့်များ ဖြစ်သည်။

## Release Boundary

ဤစာတမ်းကြောင့် version/tag ကို v0.9.0 သို့ မပြောင်းပါ။ v0.9.0 သည် feature-complete နှင့် cross-platform release artifact verification ပြီးမှသာ stable tag တင်မည်။ လက်ရှိ stable release notes ကို [`RELEASE_0.8.0.md`](RELEASE_0.8.0.md) တွင် ဖတ်ရှုနိုင်ပြီး လုပ်ဆောင်ရန်စာရင်းကို [`TODO_ZAP_MM.md`](TODO_ZAP_MM.md) တွင် ကြည့်နိုင်သည်။
