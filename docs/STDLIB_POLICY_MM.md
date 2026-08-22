# Standard-Library Stability Policy

## အခြေအနေနှင့် အကျုံးဝင်မှု

ဤ policy သည် public standard-library domain တစ်ခုချင်းစီနှင့် တိုက်ရိုက်ဖော်ပြထားသော builtin များအတွက် compatibility contract ကို သတ်မှတ်သည်။ Machine-readable source သည် native [`stdlib_catalog.rs`](../native/src/stdlib_catalog.rs) catalog ဖြစ်ပြီး ဤစာတမ်းက metadata ကို user နှင့် maintainer များ မည်သို့ဖတ်ရမည်ကို ရှင်းပြသည်။ Runtime dispatch ကို evaluator ထဲတွင် ဗဟိုပြုထားဆဲဖြစ်ပြီး catalog သည် ဒုတိယ implementation path တစ်ခု မဖန်တီးပါ။

ဤ policy သည် လက်ရှိ release line ဖြစ်သော **v2.2.0** အတွက် သက်ရောက်ပြီး public builtin တစ်ခုကို ထည့်ခြင်း၊ ပြင်ခြင်း၊ deprecated ပြုလုပ်ခြင်း သို့မဟုတ် ဖယ်ရှားခြင်းတိုင်းတွင် ပြန်လည်သုံးသပ်ရမည်။

## Stability model

Public domain နှင့် builtin တစ်ခုချင်းစီတွင် stability label တစ်ခု၊ စတင်ထည့်သွင်းသည့် release တစ်ခု၊ deprecation-window value တစ်ခု၊ semantic-versioning rule တစ်ခု၊ platform-support declaration တစ်ခု၊ input/output limit များ၊ timeout policy တစ်ခု၊ error contract တစ်ခုနှင့် determinism flag တစ်ခု ရှိရမည်။ Public API အသစ်ကို catalog ထဲသို့ field တစ်ခုခု မပြည့်စုံဘဲ ထည့်သွင်းခြင်း မပြုရ။

| Label | အဓိပ္ပာယ် | Compatibility အကျိုးဆက် |
|---|---|---|
| `stable` | Release line အတွက် ထောက်ပံ့ထားသော public behavior | Bug fix နှင့် compatible addition ကို minor release တွင် ထည့်နိုင်သည်။ Breaking semantic change အတွက် major release သို့မဟုတ် အတည်ပြုထားသော migration plan လိုအပ်သည် |
| `experimental` | Design ပြောင်းလဲနိုင်သေးသော opt-in behavior | Opt-in boundary နှင့် migration risk ကို documentation တွင် ဖော်ပြရမည်။ `stable` သို့ မြှင့်ရန် catalog နှင့် regression review လိုအပ်သည် |
| `platform-specific` | သတ်မှတ်ထားသော target family များတွင်သာ ထောက်ပံ့သော behavior | Platform matrix သည် normative ဖြစ်ပြီး unsupported target များတွင် silent emulation မပြုဘဲ stable diagnostic ဖြင့် fail ရမည် |
| `deprecated` | Migration ကာလအတွင်း ထိန်းသိမ်းထားသော ရှိပြီးသား behavior | Catalog တွင် deprecation window နှင့် replacement ကို ဖော်ပြရမည်။ Window မကုန်မီ ဖယ်ရှားခြင်း မပြုရ |

လက်ရှိ public catalog တွင် release လုပ်ပြီးသား domain နှင့် builtin အားလုံးကို `stable`၊ `2.1.14` တွင် စတင်အသုံးပြုနိုင်ပြီး active deprecation window မရှိဟု သတ်မှတ်ထားသည်။ အနာဂတ် entry များတွင် အခြား label သုံးမည်ဆိုပါက သက်ဆိုင်ရာ documentation နှင့် test လိုအပ်ချက်များကို ဖြည့်ဆည်းရမည်။

## Public domain policy

အောက်ပါ table သည် domain-level normative summary ဖြစ်သည်။ Catalog တွင် ပိုမိုကျဉ်းမြောင်းသောတန်ဖိုး မပေးထားပါက builtin တစ်ခုချင်းစီသည် ၎င်း၏ domain ၏ limit နှင့် error contract ကို အမွေဆက်ခံသည်။

| Public domain | Stability | Since | Deprecation window | Semver rule | ထောက်ပံ့သော target | Input limit | Output limit | Timeout policy | Deterministic |
|---|---|---|---|---|---|---|---|---|---|
| `text` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 KiB text argument | 8 KiB text result | not applicable | yes |
| `math` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | bounded integer arguments | bounded integer result | not applicable | yes |
| `collections` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 MiB logical collection graph | 8 MiB logical collection graph | not applicable | yes |
| `filesystem` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 MiB path/content input | 8 MiB text/line output | not applicable | yes |
| `json` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 MiB JSON input | 8 MiB JSON output | not applicable | yes |
| `system` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 KiB environment/path input | 8 KiB text or structured result | not applicable | yes |
| `time` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | checked integer milliseconds | checked duration map | not applicable | yes |
| `logging` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 KiB message and 64 fields | 64 KiB encoded record | not applicable | yes |
| `runtime` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | bounded diagnostic request | bounded statistics map | not applicable | yes |
| `async` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | run-owned task and poll budgets | bounded task result | cooperative cancellation or poll-budget timeout | yes |
| `network` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 KiB URL and 8 MiB request body | 8 MiB response body | bounded connect/read/write; server wait 10 seconds | yes |
| `process` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | text command, text arguments, 1 MiB output | 1 MiB captured stdout/stderr | bounded child wait and cleanup | yes |

Public domain အားလုံးသည် stable runtime diagnostic contract ကို အသုံးပြုသည်။ Invalid type၊ malformed value၊ path escape၊ oversized value၊ မရရှိနိုင်သော platform operation နှင့် logical budget ကျော်လွန်မှုများသည် fail-closed ဖြစ်ရမည်။ `deterministic` field သည် repeatable Zap-level behavior ကိုသာ ဆိုလိုပြီး external clock၊ network peer၊ process scheduling သို့မဟုတ် filesystem latency တို့ deterministic ဖြစ်သည်ဟု မဆိုလိုပါ။

## API evolution နှင့် semver rules

**Minor-compatible** change တွင် builtin အသစ်ထည့်ခြင်း၊ returned record ထဲသို့ optional field အသစ်ထည့်ခြင်း၊ stable code ကို မပြောင်းဘဲ diagnostic ကို ရှင်းလင်းခြင်း သို့မဟုတ် valid program များ၏ အဓိပ္ပာယ်မပြောင်းဘဲ bug fix ပြုလုပ်ခြင်းတို့ ပါဝင်သည်။ ထို change များအတွက် catalog metadata၊ English/Burmese documentation၊ regression test နှင့် compatibility record တို့ လိုအပ်သည်။

ရှိပြီးသား valid program ၏ အဓိပ္ပာယ်ပြောင်းခြင်း၊ လက်ခံထားသော input ကို invalid ပြောင်းခြင်း၊ stable result field ကို ဖယ်ရှားခြင်း သို့မဟုတ် type ပြောင်းခြင်း၊ documented diagnostic contract ကို ဖယ်ရှားခြင်း၊ သို့မဟုတ် platform guarantee ကို ကျဉ်းမြောင်းခြင်းတို့သည် **major-breaking** change ဖြစ်သည်။ Implementation မလုပ်မီ bilingual compatibility template တွင် change ကို ရေးသားပြီး အတည်ပြုရမည်။

Catalog ၏ `since` value သည် public behavior ကို စတင်ထောက်ပံ့သော release ဖြစ်သည်။ Documentation ပြင်ရုံဖြင့် လက်ရှိ release သို့ မပြောင်းရ။ `deprecation_window` value သည် active stable API အတွက် `none` ဖြစ်ပြီး deprecated API အတွက် တိကျသော migration period ပါရမည်။

## Deprecation နှင့် removal

Deprecation သည် documentation နှင့် tooling event ဖြစ်ပြီး runtime behavior ကို တိတ်တဆိတ် ပြောင်းလဲခြင်း မဟုတ်ပါ။ Deprecated entry သည် အဟောင်း dispatch behavior ကို ထိန်းသိမ်းရမည်၊ replacement ကို ဖော်ပြရမည်၊ deprecation စတင်သည့် release နှင့် removal ပြုလုပ်နိုင်သည့် အနည်းဆုံး release ကို ရေးသားရမည်။ Removal အတွက် major-version decision သို့မဟုတ် အတည်ပြုထားသော compatibility exception၊ ထို့အပြင် language tree နှစ်ခုလုံးတွင် migration example လိုအပ်သည်။

လက်ရှိ public standard-library domain သို့မဟုတ် builtin တစ်ခုမျှ deprecated မဟုတ်ပါ။ Catalog test များသည် metadata ပျောက်ဆုံးမှု၊ duplicate name၊ မသိသော domain၊ non-stable release entry နှင့် limit/error contract မရှိမှုများကို reject လုပ်သည်။

## Platform support နှင့် limits

`linux,windows,macos-arm64` release-target platform value သည် ထောက်ပံ့ထားသော CI နှင့် release target များကို ဆိုလိုသည်။ အနာဂတ် Unix-only သို့မဟုတ် Windows-only API သည် သက်ဆိုင်ရာ catalog value ကို သုံးပြီး target-native regression ပေးရမည်။ Source သည် အခြား target တစ်ခုတွင် compile ဖြစ်ရုံဖြင့် unsupported behavior ကို portable ဟု မဖော်ပြရ။

ဤ policy ၏ limit များသည် admission နှင့် safety boundary များဖြစ်ပြီး performance guarantee မဟုတ်ပါ။ Filesystem နှင့် JSON operation များသည် documented 8 MiB safety boundary ကို ထိန်းသိမ်းသည်။ Network response များသည် 8 MiB boundary ကို ထိန်းသိမ်းပြီး registry transport သည် သီးခြား 16 MiB response bound ကို enforce လုပ်သည်။ Process output သည် 1 MiB အထိ bounded ဖြစ်ပြီး run-owned memory/task/output budget များသည် allocator သို့မဟုတ် tracing-collector measurement မဟုတ်သော logical accounting ဖြစ်သည်။

## Verification နှင့် change checklist

M3-STDLIB-01 acceptance gate တွင် catalog metadata test၊ standard-library security corpus၊ full native test suite၊ strict Clippy၊ documentation consistency၊ specification ownership နှင့် `git diff --check` တို့ ပါဝင်သည်။ Public API change တစ်ခုတိုင်းတွင် သက်ဆိုင်ရာ English/Burmese index entry၊ ဤ policy pair၊ compatibility record နှင့် release roadmap ကိုလည်း update လုပ်ရမည်။

Public surface ကို [English standard-library index](STDLIB_INDEX_EN.md) နှင့် [Burmese standard-library index](STDLIB_INDEX_MM.md) များမှ ရှာဖွေနိုင်သည်။ Catalog သည် deterministic ဖြစ်ပြီး public builtin တစ်ခုချင်းစီကို တစ်ကြိမ်သာ ဖော်ပြကာ domain တစ်ခုတည်းသို့ ပိုင်ဆိုင်စေသည်။

## လက်ရှိ release ဆုံးဖြတ်ချက်

v2.2.0 အတွက် catalog ထဲရှိ standard-library domain နှင့် builtin အားလုံးသည် **stable** ဖြစ်သည်။ Active deprecation window မရှိ၊ default minor-compatible rule ကို လိုက်နာပြီး release-target matrix ကို ထောက်ပံ့ကာ bounded deterministic error behavior ကို ဖော်ပြသည်။ Namespace import နှင့် remote standard-library package များသည် သီးခြား future milestone များအဖြစ် ဆက်ရှိပြီး traits-based composition ကို design-only M4-RFC-01 တွင် မှတ်တမ်းတင်ထားသော်လည်း deferred အဖြစ် ဆက်ရှိသည်။
