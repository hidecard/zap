# Zap RustSec Dependency Audit Evidence

**စစ်ဆေးထားသော baseline:** Zap v2.6.0 development line

**ရည်ရွယ်ချက်:** Native runtime ၏ dependency version များ၊ local audit evidence၊ tool limitation နှင့် CI/release control များကို မှတ်တမ်းတင်ရန် ဖြစ်သည်။ ဤစာတမ်းသည် v2.6.0 development line အတွက် evidence record ဖြစ်ပြီး untagged development commit ကို published release ဟု မဆိုလိုပါ။

## လက်ရှိ lockfile dependency graph

Native lockfile တွင် security နှင့်ဆိုင်သော package များကို အောက်ပါအတိုင်း သတ်မှတ်ထားပါသည်။ Repository သည် Rust 1.88.0 ကို pin လုပ်ထားပြီး exact lockfile ကို CI နှင့် release preflight တွင် audit လုပ်ပါသည်။

| Package | Locked version | အသုံးပြုသည့်နေရာ | Audit အခြေအနေ |
|---|---:|---|---|
| `ureq` | `2.12.1` | HTTP client | `2.9.7` မှ update လုပ်ထားသည် |
| `rustls` | `0.23.40` | TLS implementation | လက်ရှိ locked line; ring-only feature သုံးထားသည် |
| `rustls-webpki` | `0.103.15` | Web PKI certificate validation | Current advisory-patched line ဖြစ်သည် |
| `url` | `2.5.8` | URL parsing | `2.4.1` မှ update လုပ်ထားသည် |
| `idna` | `1.1.0` | Internationalized domain processing | RUSTSEC-2024-0421 အတွက် remediated line ဖြစ်သည် |
| `idna_adapter` | `1.2.0` | IDNA backend | Locked dependency ဖြစ်သည် |
| `litemap` | `0.7.4` | IDNA backend support | Locked dependency ဖြစ်သည် |
| `zeroize` | `1.8.2` | Secret-memory cleanup support | Locked dependency ဖြစ်သည် |

Registry TLS tests များသည် `rcgen` ဖြင့် test run တိုင်း certificate generate မလုပ်တော့ဘဲ repository ထဲတွင် သိမ်းထားသော localhost end-entity DER certificate နှင့် key fixture ကို အသုံးပြုပါသည်။ ထို့ကြောင့် မလိုအပ်သော `rcgen`/`time` test dependency graph ကို ဖယ်ရှားနိုင်ပြီး time crate ၏ RFC 2822 advisory ကို native lockfile ထဲသို့ မသယ်ဆောင်တော့ပါ။

## Local audit evidence

Local cargo-audit run သည် repository-compatible Rust toolchain ကို အသုံးပြုနိုင်ပါသည်။ သို့သော် ဤ version ၏ parser သည် current RustSec database ထဲရှိ CVSS 4.0 value ပါသော record များကို မဖတ်နိုင်ဘဲ unsupported CVSS version ဟု ပြန်ပေးပါသည်။

ထို့ကြောင့် local result ကို စစ်ဆေးနိုင်အောင် official current advisory database ၏ temporary copy တစ်ခုကို အသုံးပြုပြီး CVSS 4.0 record များသာ ဖယ်ရှားကာ `native/Cargo.lock` ကို scan ပြုလုပ်ခဲ့ပါသည်။ Parse လုပ်နိုင်သော advisory 1,166 ခုကို load လုပ်ပြီး crate dependency 82 ခုကို စစ်ဆေးရာ finding မတွေ့ဘဲ exit status 0 ရရှိခဲ့ပါသည်။ ထပ်မံ package-name comparison ပြုလုပ်ရာ lockfile ထဲရှိ package နှင့် ကိုက်ညီသော CVSS 4.0 advisory file မတွေ့ပါ။

ဤ workaround သည် old audit parser က CVSS 4.0 record များကို မဖတ်နိုင်ခြင်းကြောင့် complete audit မဟုတ်ကြောင်း ရှင်းလင်းစွာ မှတ်တမ်းတင်ထားပါသည်။ CI နှင့် release workflow များတွင် stable Rust toolchain ပေါ်၌ `cargo-audit 0.22.2` ကို အသုံးပြုပြီး `cargo audit --file native/Cargo.lock --deny warnings` ကို run လုပ်ပါသည်။ ထို့ကြောင့် CVSS 4.0 record များပါဝင်သော live database ကို repository gate မှ စစ်ဆေးနိုင်ပါသည်။

## Advisory remediation mapping

Official RustSec `idna` advisory သည် `idna 1.0.3` သို့မဟုတ် နောက်ပိုင်းကို update လုပ်ရန်၊ သို့မဟုတ် `url` မှတစ်ဆင့် `idna` ရောက်လာပါက `url 2.5.4` သို့မဟုတ် နောက်ပိုင်းကို သုံးရန် အကြံပြုထားပါသည်။[^1] လက်ရှိ lockfile သည် `idna 1.1.0` နှင့် `url 2.5.8` ကို အသုံးပြုပါသည်။

လက်ရှိ `rustls-webpki` advisory record များတွင် issue အလိုက် patched version ကို `0.103.10`၊ `0.103.12` သို့မဟုတ် `0.103.13` နှင့် နောက်ပိုင်းဟု သတ်မှတ်ထားပါသည်။[^2] လက်ရှိ lockfile သည် `rustls-webpki 0.103.15` ကို အသုံးပြုသောကြောင့် ယခင် `0.102.8` package ကို graph မှ ဖယ်ရှားထားပါသည်။

Time crate ၏ current stack-exhaustion advisory သည် `0.3.47` မတိုင်မီ RFC 2822 parsing path များနှင့် သက်ဆိုင်ပါသည်။[^3] Project graph ကို လက်ရှိ lockfile နှင့် pin လုပ်ထားသော Rust 1.88.0 toolchain အပေါ် စစ်ဆေးပါသည်။

## CI နှင့် release controls

CI workflow တွင် သီးခြား RustSec job တစ်ခုရှိပြီး `cargo-audit 0.22.2` ကို install လုပ်ကာ exact native lockfile ကို audit ပြုလုပ်ပါသည်။ Audit job fail ဖြစ်ပါက platform build matrix မဆက်နိုင်အောင် dependency သတ်မှတ်ထားပါသည်။ Tag-release quality job သည်လည်း release validation နှင့် packaging မလုပ်မီ locked audit တစ်ကြိမ် ထပ် run ပါသည်။

Dependency update များကို repository ၏ pinned Rust compatibility policy နှင့်အညီ ဆက်လက်ထိန်းသိမ်းရမည် ဖြစ်သည်။ အနာဂတ်တွင် Rust toolchain update လုပ်ပါက complete audit ကို ပြန် run ပြီး compatibility pin များကို ပြန်လည်စစ်ဆေးရမည်။

## မပြီးသေးသော limitation များ

Filtered snapshot ဖြင့် local audit ပြုလုပ်ထားခြင်းသည် complete live-database scan ၏ အစားထိုးမဟုတ်ပါ။ Complete authoritative scan ကို newer audit binary အသုံးပြုသော CI နှင့် release job များမှ အတည်ပြုမည် ဖြစ်သည်။

Filesystem confinement သည် path ကို canonicalize လုပ်ပြီးနောက် open သို့မဟုတ် rename လုပ်သောကြောင့် portable check/use TOCTOU boundary ရှိနေဆဲ ဖြစ်သည်။ ပြည့်စုံသော fix အတွက် Unix တွင် descriptor-relative no-follow operation၊ Windows တွင် reparse-point-aware handle logic နှင့် dedicated race tests များ လိုအပ်ပါသည်။ ဤ audit document သည် ထို issue ကို fix ပြီးပြီဟု မဆိုပါ။

## References

[^1]: [RUSTSEC-2024-0421: idna accepts Punycode labels that do not produce any non-ASCII when decoded](https://rustsec.org/advisories/RUSTSEC-2024-0421.html)
[^2]: [RustSec advisories for rustls-webpki](https://rustsec.org/packages/rustls-webpki.html)
[^3]: [RUSTSEC-2026-0009: time denial of service via stack exhaustion](https://rustsec.org/advisories/RUSTSEC-2026-0009.html)
