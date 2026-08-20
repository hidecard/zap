# Zap v0.7.0

Zap v0.7.0 သည် v0.6.0 OOP foundation ပေါ်တွင် standard library နှင့် developer workflow ကို တိုးချဲ့ထားသော release ဖြစ်သည်။ ဤ release ၏ ရည်ရွယ်ချက်မှာ everyday scripting၊ data processing နှင့် file-based project များကို ပိုမိုလွယ်ကူစေရန် ဖြစ်သည်။

## အဓိကအပြောင်းအလဲများ

### Collection helpers

အောက်ပါ built-ins များကို ထည့်သွင်းထားသည်။

| Built-in | အလုပ်လုပ်ပုံ |
|---|---|
| `is_empty(value)` | text၊ list၊ map နှင့် `none` တို့ empty ဖြစ်/မဖြစ် စစ်သည် |
| `sum(list)` | number list ၏ စုစုပေါင်းကို ပြန်ပေးသည် |
| `reverse(value)` | text သို့မဟုတ် list ကို ပြောင်းပြန်ထားသော value အသစ် ပြန်ပေးသည် |
| `sort(list)` | number list သို့မဟုတ် text list ကို စီသည် |
| `get(map, key, default)` | key မရှိပါက default value ပြန်ပေးသည် |

```zap
let scores = [8, 3, 10, 5]
say sum(scores)
say join(sort(scores), ",")

let settings = {"mode": "dev"}
say get(settings, "timeout", 30)
```

### Line-based file I/O

`read_lines(path)` နှင့် `write_lines(path, list)` ဖြင့် text file များကို line list အဖြစ် ကိုင်တွယ်နိုင်သည်။

```zap
write_lines("notes.txt", ["one", "two", "three"])
let notes = read_lines("notes.txt")
say join(notes, "|")
```

### CLI workflow

`zap run <file.zp>` သည် source file ကို explicit command ဖြင့် run နိုင်စေသည်။ မူလ `zap <file.zp>` command ကိုလည်း backward-compatible အဖြစ် ဆက်လက်ထောက်ပံ့ထားသည်။

```bash
zap run main.zp
zap check .
zap build .
zap test .
zap fmt main.zp
```

## Verification

v0.7.0 native integration test suite တွင် **21 tests** ပါဝင်ပြီး OOP၊ v0.6.0 standard library၊ v0.7.0 collection helpers၊ line-based file I/O၊ map defaults၊ modules၊ project validation နှင့် CLI behavior များကို စစ်ဆေးထားသည်။

```bash
cd native
cargo test
```

## Installation

GitHub Releases မှ သင့် platform နှင့်ကိုက်ညီသော archive ကို download လုပ်ပြီး SHA-256 checksum ဖြင့် စစ်ဆေးပါ။ ထို့နောက် archive ကို extract လုပ်၍ `zap` binary ကို PATH ထဲသို့ ထည့်ပါ။ Windows တွင် `install_windows.bat` ကို အသုံးပြုနိုင်သည်။

## မပါဝင်သေးသောအရာများ

`async/await`၊ channels၊ HTTP client/server၊ package registry၊ lockfile၊ generics၊ interfaces၊ full static type checker၊ bytecode VM နှင့် FFI များသည် v0.7.0 တွင် မပါဝင်သေးပါ။ ၎င်းတို့ကို runtime semantics၊ security နှင့် cross-platform behavior များ သေချာသတ်မှတ်ပြီး နောက် release များတွင် ဆက်လက်အကောင်အထည်ဖော်မည်။

## Documentation

- [`README.md`](../README.md) — Main complete guide
- [`SYNTAX_GUIDE.md`](SYNTAX_GUIDE.md) — Syntax reference
- [`LEARN_ZAP_MM.md`](LEARN_ZAP_MM.md) — Burmese lesson-based course
- [`USAGE.md`](USAGE.md) — Installation and workflow
- [`DESIGN.md`](DESIGN.md) — Runtime design
- [`ROADMAP_0.7.0.md`](ROADMAP_0.7.0.md) — Next feature roadmap
