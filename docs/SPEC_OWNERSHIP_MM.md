# P0-02-A Specification Ownership Index

## ရည်ရွယ်ချက်

Machine-readable [`SPEC_OWNERSHIP_INDEX.tsv`](SPEC_OWNERSHIP_INDEX.tsv) သည် canonical Zap specification အတွက် executable ownership index ဖြစ်ပါသည်။ လက်ရှိ row ၃၇ ခုသည် source execution၊ precedence၊ typing၊ first-class callable functions၊ modules၊ memory နှင့် budget boundary၊ deterministic/production async behavior၊ LSP synchronization/interoperability/rename၊ diagnostics၊ registry transport/security၊ lockfile၊ JSON/filesystem limits၊ standard-library determinism၊ benchmark provenance၊ release versioning၊ CLI JSON၊ compatibility policy၊ runtime borrow/equality safety နှင့် CI enforcement များကို လွှမ်းခြုံထားပါသည်။ Public rule တစ်ခုချင်းစီ၏ English section၊ Burmese section၊ implementation သို့မဟုတ် conformance fixture owner၊ implementation status နှင့် compatibility class ကို row တစ်ကြောင်းစီတွင် သတ်မှတ်ထားပါသည်။ ထို့ကြောင့် rule တစ်ခုသည် အဟောင်း guide တွင် ပါနေခြင်း သို့မဟုတ် legacy runtime က လက်ခံနေခြင်းတစ်ခုတည်းကြောင့် normative ဖြစ်သွားခြင်းကို ကာကွယ်ပါသည်။

## Row field များ၏ လိုအပ်ချက်

| Field | Contract |
|---|---|
| `rule_id` | Stable `SPEC-NNN` identifier ဖြစ်ရမည်။ ID ကို rule အဓိပ္ပါယ်ပြောင်းလဲရန်အတွက် တိတ်တဆိတ် ပြန်အသုံးမပြုရပါ |
| `domain` | `values-typing`၊ `diagnostics` သို့မဟုတ် `async-deterministic` ကဲ့သို့ အတိုချုံး semantic domain |
| `canonical_en` | Repository-relative English specification path နှင့် section reference |
| `canonical_mm` | Repository-relative Burmese specification path နှင့် section reference |
| `fixture_owner` | Repository-relative source၊ test၊ matrix သို့မဟုတ် script path ဖြစ်ပြီး လိုအပ်ပါက `#fragment` ပါနိုင်သည် |
| `status` | `implemented` သို့မဟုတ် `deferred` တစ်ခုဖြစ်ရမည် |
| `compatibility` | `normative`၊ `compatibility`၊ `deprecated` သို့မဟုတ် `rejected` တစ်ခုဖြစ်ရမည် |

Validator သည် bilingual document နှစ်ခုနှင့် section reference များ ရှိ/မရှိ၊ fixture owner file နှင့် fragment ရှိ/မရှိ၊ policy value များ မှန်/မမှန်၊ rule ID များ unique ဖြစ်/မဖြစ်နှင့် လိုအပ်သော semantic domain အားလုံး ပါ/မပါကို စစ်ဆေးပါသည်။ Report သည် row တစ်ကြောင်းစီ၏ `PASS` သို့မဟုတ် `FAIL` decision ပါသော deterministic TSV output ဖြစ်ပါသည်။

## Ownership စည်းမျဉ်းများ

Canonical specification သည် semantics ကို ပိုင်ဆိုင်ပါသည်။ Implementation module တစ်ခုချင်းစီသည် ထို semantic boundary အတွင်း executable behavior ကို ပိုင်ဆိုင်ပါသည်။ AST/parser သည် syntax construction၊ evaluator သည် runtime expression နှင့် statement behavior၊ diagnostics သည် structured error field များ၊ registry သည် package transport နှင့် integrity၊ CI သည် enforcement ကို ပိုင်ဆိုင်ပါသည်။ Cross-cutting rule ဖြစ်သော်လည်း canonical bilingual section တစ်ခုနှင့် stable fixture owner policy ရှိရမည်။

Public rule အသစ်တစ်ခုသည် index row၊ bilingual section သို့မဟုတ် normative subcontract သို့ explicit cross-link နှင့် passing သို့မဟုတ် intentionally failing fixture မရှိမချင်း မပြီးစီးသေးပါ။ Deferred rule ကို release documentation တွင် implemented ဟု မရေးဘဲ `deferred` အဖြစ် ဆက်လက် label လုပ်ရမည်။ Compatibility behavior ကို explicit classification လုပ်ရမည်။ Legacy acceptance တစ်ခုတည်းဖြင့် `normative` အဖြစ် မြှင့်တင်ခြင်း မပြုရပါ။ အနာဂတ် behavior change များအတွက် bilingual [`COMPATIBILITY_CHANGE_TEMPLATE_EN.md`](COMPATIBILITY_CHANGE_TEMPLATE_EN.md) နှင့် [`COMPATIBILITY_CHANGE_TEMPLATE_MM.md`](COMPATIBILITY_CHANGE_TEMPLATE_MM.md) records များကို အသုံးပြုရမည်။

## Validation command နှင့် CI artifact

Ownership gate ကို local တွင် အောက်ပါအတိုင်း run နိုင်ပါသည်။

```text
ZAP_SPEC_OWNERSHIP_REPORT=target/spec-ownership-report.tsv scripts/validate_spec_ownership.sh
```

GitHub Actions သည် Rust quality job အတွင်း command တူကို run လုပ်ပြီး `target/spec-ownership-report.tsv` ကို commit-named artifact အဖြစ် upload လုပ်ပါသည်။ Index ထဲတွင် fragmented rule အားလုံးကို ဆက်လက်ချဲ့ထွင်နိုင်ပါသည်။ လက်ရှိ expansion သည် post-review LSP protocol/interoperability contract၊ schema-2 standard-library determinism၊ logical memory budget၊ checked runtime borrow/equality safety၊ registry transport limit၊ benchmark provenance နှင့် release-version validation များကို အတိအလင်း ပိုင်ဆိုင်စေပြီး နောက်ထပ် row များသည် stable ID၊ required-domain coverage နှင့် bilingual ownership field များကို မပြောင်းလဲရပါ။
