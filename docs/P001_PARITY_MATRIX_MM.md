# P0-01-A Native/Legacy Parity Matrix

## Scope နှင့် ownership

ဤ matrix သည် native Zap behavior ကို ထိန်းသိမ်းထားသော Python reference runtime နှင့် executable အဖြစ် နှိုင်းယှဉ်စေပါသည်။ Versioned source of truth သည် [`conformance/p0-01/matrix.tsv`](../conformance/p0-01/matrix.tsv) ဖြစ်ပြီး durable source fixture များကို ထို directory အောက်တွင် ထားရှိပါသည်။ Native runtime သည် canonical implementation ဖြစ်ပြီး legacy runtime သည် compatibility reference သာ ဖြစ်ပါသည်။ ဒုတိယ normative specification အဖြစ် မသတ်မှတ်ရပါ။

## Policy အမျိုးအစားများ

| Policy | Native မှ လိုအပ်သောရလဒ် | Legacy မှ လိုအပ်သောရလဒ် | အဓိပ္ပါယ် |
|---|---|---|---|
| `common` | Exit `0` နှင့် normalized stdout digest သည် legacy နှင့် ကိုက်ညီရမည် | Exit `0` နှင့် normalized stdout digest ကိုက်ညီရမည် | Runtime နှစ်ခုကြား behavior compatibility ရှိနေသည် |
| `native-only` | Exit `0` | Non-zero exit | Native language contract သည် ထိန်းသိမ်းထားသော legacy translator ထက် ပိုကျယ်သည်။ Silent drift မဟုတ်ဘဲ migration documentation လိုအပ်သည် |
| `rejected` | Non-zero exit | Non-zero exit | Malformed သို့မဟုတ် unsupported input ကို implementation နှစ်ခုလုံးက fail-closed reject လုပ်ရမည် |
| `compatibility` | Inventory တွင် တွေ့ရသည် | Inventory တွင် တွေ့ရသည် | Runtime နှစ်ခုလုံး အောင်မြင်သော်လည်း output ကွဲသည်; explicit compatibility decision လိုသည် |
| `deprecated` | Inventory တွင် တွေ့ရသည် | Inventory တွင် တွေ့ရသည် | Legacy က လက်ခံပြီး native က ပယ်သည်; migration သို့မဟုတ် removal decision လိုသည် |

Executable matrix တွင် လက်ရှိ `common`၊ `native-only` နှင့် `rejected` rows များကို gate လုပ်သည်။ Inventory command က observed class အားလုံးကို မှတ်တမ်းတင်ထားသဖြင့် compatibility နှင့် deprecated behavior များ review မှ မပျောက်စေရပါ။

Runtime နှစ်ခု၏ diagnostic surface မတူသောကြောင့် raw error wording ကို မနှိုင်းယှဉ်ပါ။ Rejection အတွက် exit status နှင့် common success behavior အတွက် normalized stdout ၏ SHA-256 digest ကို နှိုင်းယှဉ်ပါသည်။ Normalization သည် blank line များဖယ်ရှားပြီး CRLF ကို LF သို့ ပြောင်းသော်လည်း user-visible output content ကို မဖျက်ပါ။

## Versioned case များ

| Fixture ID | Policy | Fixture | ရည်ရွယ်ချက် |
|---|---|---|---|
| `P001-COMMON-HELLO` | `common` | `common/hello.zp` | တည်ငြိမ်သော `say` output |
| `P001-COMMON-CONDITIONAL` | `common` | `common/conditional.zp` | အခြေခံ indentation နှင့် conditional execution |
| `P001-COMMON-FUNCTION` | `common` | `common/function_body.zp` | Function declaration၊ return၊ call နှင့် numeric output |
| `P001-NATIVE-ARITHMETIC` | `native-only` | `native-only/arithmetic.zp` | Modern declaration boundary နှင့် arithmetic |
| `P001-NATIVE-WHILE-LOOP` | `native-only` | `native-only/while_loop.zp` | Modern declaration boundary နှင့် loop |
| `P001-NATIVE-VARIABLES` | `native-only` | `native-only/variables.zp` | Modern declaration နှင့် assignment boundary |
| `P001-NATIVE-LET` | `native-only` | `native-only/let_binding.zp` | Native declaration semantics ကို legacy runtime က မ translate လုပ်နိုင်ခြင်း |
| `P001-NATIVE-APPEND` | `native-only` | `native-only/append_function.zp` | Modern declaration boundary ရှိ native list builtin |
| `P001-NATIVE-ASSERT` | `native-only` | `native-only/assert_statement.zp` | Modern declaration boundary ရှိ native assertion builtin |
| `P001-NATIVE-JOIN` | `native-only` | `native-only/join_function.zp` | Modern declaration boundary ရှိ native string builtin |
| `P001-REJECT-GROUP` | `rejected` | `rejected/unclosed_group.zp` | မပိတ်ရသေးသော expression delimiter |
| `P001-REJECT-STRING` | `rejected` | `rejected/unterminated_string.zp` | မပြီးဆုံးသော text literal |
| `P001-REJECT-INVALID-INDENTATION` | `rejected` | `rejected/invalid_indentation.zp` | မှားယွင်းသော block indentation |

## Executable gate နှင့် inventory

Curated matrix ကို local တွင် အောက်ပါအတိုင်း run ပါ။

```text
ZAP_PARITY_REPORT=target/p001-parity-report.tsv scripts/test_p001_parity.sh
```

Tracked fixture inventory ကို အောက်ပါအတိုင်း run ပါ။

```text
ZAP_LEGACY_PARITY_INVENTORY=target/legacy-parity-inventory.tsv scripts/inventory_legacy_parity.sh
```

Runner သည် လိုအပ်ပါက native binary ကို build သို့မဟုတ် select လုပ်ပြီး engine နှစ်ခုလုံးကို fixture တစ်ခုတည်းဖြင့် invoke လုပ်ပါသည်။ သတ်မှတ်ထားသော normalization ကို အသုံးပြီး fixture ID၊ policy၊ exit status၊ output digest၊ decision နှင့် classification ပါသော deterministic tab-separated report ကို ရေးပါသည်။ Inventory သည် tracked `.zp` source များကိုသာ အသုံးပြုပြီး fixture၊ conformance، example၊ test၊ corpus နှင့် framework trees များကို ဖတ်သည်။ File-writing fixture များ repository ကို မပြင်းစေရန် isolated temporary working directory တွင် run သည်။

GitHub Actions သည် Rust quality job အတွင်း curated gate ကို run လုပ်ပြီး `target/p001-parity-report.tsv` ကို commit-named artifact အဖြစ် upload လုပ်ပါသည်။ CI သည် prose comparison ပေါ်တွင်သာ မမှီခိုဘဲ executable parity report ကို ထိန်းသိမ်းပါသည်။ Inventory report သည် release evidence နှင့် review အတွက် သင့်တော်ပြီး second normative specification မဟုတ်ပါ။

## Migration စည်းမျဉ်းများ

Native behavior အသစ်တစ်ခု ထည့်သွင်းပါက matrix row အသစ်တစ်ခုနှင့် executable policy သုံးမျိုးထဲမှ တစ်မျိုးကို အရင်သတ်မှတ်ရမည်။ `common` mismatch သည် parity regression ဖြစ်ပြီး ပြင်ဆင်ရမည် သို့မဟုတ် reviewed matrix change ဖြင့်သာ reclassify လုပ်ရမည်။ `native-only` row သည် bilingual migration guidance သို့ link ချိတ်ထားရမည်၊ intentional ဖြစ်ကြောင်း ဆက်လက်ရှင်းလင်းရမည်။ `rejected` row သည် panic မဖြစ်ဘဲ ဆက်လက် reject ဖြစ်ရမည်။ `compatibility` နှင့် `deprecated` inventory rows များသည် curated matrix ထဲမတင်မီ owner နှင့် release decision လိုပါသည်။ Fixture တစ်ခုမျှ network access၊ wall-clock time၊ host-specific absolute path သို့မဟုတ် secret value ကို မမှီခိုရပါ။

Legacy line-based representation ကို older/internal declaration များအတွက် compatibility format အဖြစ် ဆက်လက်ထားရှိပါသည်။ ဤ matrix သည် broad syntax expansion၊ traits implementation သို့မဟုတ် fallback ဖယ်ရှားခြင်းကို ခွင့်မပြုပါ။ ထိုပြောင်းလဲမှုများသည် သီးခြား compatibility decision နှင့် release note လိုအပ်ပါသည်။
