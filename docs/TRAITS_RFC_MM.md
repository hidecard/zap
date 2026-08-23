# Zap Traits နှင့် Composition RFC

**RFC အခြေအနေ:** Design-only proposal ဖြစ်ပြီး parser သို့မဟုတ် runtime implementation မပါဝင်ပါ။
**စစ်ဆေးထားသော baseline:** Zap v2.2.7
**ဆုံးဖြတ်မည့် version:** Future post-v2.2 language version အတွက် review ပြုလုပ်ရန်။ v2.2.0 တွင် traits၊ interfaces သို့မဟုတ် inheritance semantics အသစ်များကို enable မလုပ်ပါ။
**ဖတ်ရှုသင့်သူများ:** Language designer၊ runtime maintainer၊ package author နှင့် အနာဂတ် compatibility change reviewer များ။
**လမ်းညွှန်:** [Documentation hub](DOCUMENTATION_NAVIGATION_MM.md) · [လေ့လာရေး guide](LEARN_ZAP_MM.md) · [Syntax guide](SYNTAX_GUIDE.md) · [Language specification](LANGUAGE_SPEC_MM.md) · [Package guide](PACKAGE.md) · [English RFC](TRAITS_RFC_EN.md)

## အနှစ်ချုပ်

ဤ RFC သည် Zap တွင် reusable behavior များကို composition-first ပုံစံဖြင့် ဒီဇိုင်းဆွဲရန် အဆိုပြုချက်ဖြစ်သည်။ အဆိုပြုချက်သည် named behavioral contract များနှင့် explicit composition ကို ထည့်သွင်းစဉ်းစားသော်လည်း v2.2.0 တွင် လက်ရှိ single-inheritance model ကို မဖယ်ရှားပါ။ Implementation မစမီ conceptual model၊ surface syntax၊ method lookup၊ visibility၊ diagnostic၊ migration rule၊ dispatch choice၊ rejected alternative နှင့် compatibility boundary များကို သတ်မှတ်ထားရန် ရည်ရွယ်သည်။

> **ဆုံးဖြတ်ချက်:** `extends` ကို လက်ရှိ inheritance mechanism အဖြစ် ဆက်လက်ထားရှိမည်။ Traits/interfaces ကို deferred ထားမည်။ ဤ RFC ကို review ပြုလုပ်ပြီး နောက် version တစ်ခုတွင် explicit approval ရရှိသည်အထိ parser သို့မဟုတ် evaluator behavior မပြောင်းလဲရ။

## ၁။ ပြဿနာသတ်မှတ်ချက်

Zap တွင် လက်ရှိ `extends` ဖြင့် class၊ method နှင့် single inheritance ကို support လုပ်ထားသည်။ ထို model သည် အမျိုးအစားတူ object များအတွက် သင့်တော်သော်လည်း behavior reuse ကို nominal parent တစ်ခုတည်းနှင့် ချိတ်ဆက်ထားသည်။ သီးခြား domain နှစ်ခုမှ capability များကို class တစ်ခုက လိုအပ်သောအခါ method များကို ထပ်ရေးခြင်း၊ မသင့်လျော်သော parent hierarchy တည်ဆောက်ခြင်း သို့မဟုတ် method contract တွင် မပါဝင်သည့် helper function များကို အသုံးပြုခြင်းတို့ ဖြစ်တတ်သည်။ ထိုရွေးချယ်မှုများကြောင့် diagnostic၊ package evolution နှင့် method ownership များ မရှင်းလင်းနိုင်ပါ။

ဤအဆိုပြုချက်သည် composition ကို သီးခြား design problem အဖြစ် ဖြေရှင်းသည်။ လက်ရှိ runtime တွင် traits၊ interfaces၊ conflict resolution သို့မဟုတ် multiple inheritance ရှိပြီးသားဟု မဆိုလိုပါ။ လက်ရှိ specification သည် class နှင့် runtime ownership boundary ကို သတ်မှတ်ထားပြီး ဤ RFC သည် အနာဂတ် design record သာဖြစ်သည် [1] [2]။

## ၂။ ရည်မှန်းချက်နှင့် မပါဝင်သည့်အရာများ

| အပိုင်း | ရည်မှန်းချက် | ဤ RFC တွင် မပါဝင်သည့်အရာ |
|---|---|---|
| Reuse | အမည်ပေးထားသော behavior unit များကို class ထဲ compose လုပ်ရန် | v2.2.0 တွင် feature implementation လုပ်ရန် |
| Contract | Required method နှင့် optional provided method များ သတ်မှတ်ရန် | Function အားလုံးကို default structural ဖြစ်စေရန် |
| Lookup | Precedence နှင့် conflict handling ကို deterministic ဖြစ်စေရန် | Source order ဖြင့် method ကို ခန့်မှန်းရွေးရန် |
| Visibility | Composed unit များအတွင်း public/private boundary ကို ထိန်းရန် | Private implementation detail ကို consumer ထံ ဖော်ထုတ်ရန် |
| Diagnostics | Stable missing/conflict diagnostic နှင့် source span ပေးရန် | မတည်ငြိမ်သော string-only error ကို ပြန်သုံးရန် |
| Migration | `extends` အသုံးပြုသူများအတွက် mechanical migration လမ်းကြောင်းပေးရန် | Single inheritance ကို အလိုအလျောက်ဖယ်ရှားရန် |
| Dispatch | Static/dynamic boundary ကို explicit ရွေးရန် | လက်ရှိ check ထက်ကျော်သော production type system ကို ကတိပြုရန် |
| Compatibility | Accepted၊ deprecated၊ rejected နှင့် future syntax ကို ခွဲရန် | Review မပြီးမီ parser/runtime ပြောင်းရန် |

## ၃။ လက်ရှိ baseline

လက်ရှိ Zap baseline တွင် class declaration၊ method၊ constructor နှင့် `extends` ဖြင့် single inheritance ပါဝင်သည်။ လက်ရှိ specification သည် syntax နှင့် runtime semantics ကို ပိုင်ဆိုင်ပြီး structured diagnostic တွင် severity၊ stable code၊ message နှင့် ရနိုင်သည့် source location များ ပါရမည် [1]။ လက်ရှိ release line သည် v2.2.7 ဖြစ်ပြီး semantics change တစ်ခုအတွက် specification update၊ bilingual documentation၊ conformance fixture၊ changelog entry နှင့် explicit version decision လိုအပ်သည် [1]။

ထို့ကြောင့် ဤ RFC ကြောင့် လက်ရှိ baseline မပြောင်းလဲပါ။

```zap
class Animal:
    fn speak(self):
        return "sound"

class Dog extends Animal:
    fn speak(self):
        return "woof"
```

ဤဥပမာသည် v2.2.0 တွင်လည်း single inheritance အဖြစ်သာ အဓိပ္ပာယ်ရမည်။ နောက်ပိုင်း approved implementation milestone မရမချင်း `trait`၊ `interface`၊ `with` သို့မဟုတ် conflict-resolution syntax များကို parser က လက်မခံရ။

## ၄။ ဝေါဟာရ

| Term | အဆိုပြုအဓိပ္ပာယ် |
|---|---|
| Trait | Required method၊ provided method နှင့် visibility metadata ပါနိုင်သော named behavior declaration အစု။ Instantiable class မဟုတ်ပါ။ |
| Interface | Required callable signature နှင့် visibility rule ပါသော contract။ ပထမအဆိုပြုချက်တွင် method body မပါပါ။ |
| Composition | Explicit syntax ဖြင့် trait/interface တစ်ခု သို့မဟုတ် အများကို class နှင့် ချိတ်ဆက်ခြင်း။ |
| Required method | Composing class က implement လုပ်ရမည့် method။ မပြီးသေးလျှင် class သည် concrete မဖြစ်ပါ။ |
| Provided method | Trait ကပေးသော default method body။ Conflict ဖြစ်ပါက explicit resolution လိုသည်။ |
| Conformance | Class တစ်ခုသည် interface သို့မဟုတ် trait requirement ကို ပြည့်မီသည်ဟု checker/runtime တွင် မြင်နိုင်သောအချက်။ |
| Conflict | Composed unit နှစ်ခု သို့မဟုတ် အများက method name တူတူပေးပြီး explicit selection မရှိခြင်း။ |
| Linearization | Class၊ composed unit နှင့် parent method များကို ရှာဖွေမည့် deterministic order။ |
| Static dispatch | Receiver contract သို့မဟုတ် selected implementation ကို statically သိရှိ၍ call ဖြေရှင်းခြင်း။ |
| Dynamic dispatch | Receiver ၏ concrete class နှင့် conformance table ကို runtime တွင် အသုံးပြု၍ call ဖြေရှင်းခြင်း။ |

## ၅။ Composition နှင့် single inheritance နှိုင်းယှဉ်ချက်

Composition နှင့် inheritance သည် ပြဿနာမတူသောကြောင့် language model တွင် သီးခြားထားရမည်။

| မေးခွန်း | Single inheritance | အဆိုပြု composition |
|---|---|---|
| အဓိကဆက်နွယ်မှု | “Is a specialized form of” | “Has these capabilities” |
| Parent အရေအတွက် | Nominal parent အများဆုံးတစ်ခု | Explicit trait/interface အများအပြား |
| Reuse unit | Parent class ၏ state နှင့် method | Named behavior contract နှင့် selected method |
| State ownership | Parent က instance state ပေးနိုင်သည် | ပထမအဆိုပြုချက်တွင် trait သည် instance field ကို implicit မထည့်ပါ |
| Override rule | Child method က inherited method ကို override လုပ်သည် | Class implementation က provided trait method ကို override လုပ်သည်။ Provider နှစ်ခု မရွေးထားလျှင် conflict ဖြစ်သည် |
| Constructor behavior | Parent constructor rule များကို explicit ထားသည် | Trait/interface သည် constructor မ run ပါ |
| Compatibility | လက်ရှိ `extends` ကို ဆက် support လုပ်သည် | Syntax အသစ်ကို နောက် version gate ဖြင့်သာ ထည့်မည် |
| သင့်တော်သည့်အသုံး | Object taxonomy နှင့် stateful specialization | Printable၊ iterable၊ comparable သို့မဟုတ် serializable ကဲ့သို့ cross-cutting capability |

အဆိုပြုချက်သည် multiple inheritance ကို တမင်ရှောင်ထားသည်။ Class တစ်ခုတွင် class parent တစ်ခုသာ ရှိနိုင်ပြီး stateless behavior unit များကို အများအပြား compose လုပ်နိုင်သည်။ ထိုသို့ object layout၊ constructor order နှင့် `super` behavior ကို capability reuse မှ သီးခြားထားနိုင်သည်။

## ၆။ အဆိုပြု surface syntax

အောက်ပါ syntax သည် ရှင်းလင်းပြရန်အတွက်သာဖြစ်ပြီး v2.2.0 parser က လက်မခံပါ။

### ၆.၁ Provided method ပါသော trait

```zap
trait Printable:
    fn format(self) -> text:
        return "<value>"

class Report with Printable:
    fn format(self) -> text:
        return self.title
```

Class implementation သည် provided trait method ထက် precedence မြင့်သည်။ Required method အားလုံးကို ပြည့်မီစေရန် class တွင် တာဝန်ရှိသည်။

### ၆.၂ Required method ပါသော interface

```zap
interface Identifiable:
    fn id(self) -> text

class User implements Identifiable:
    fn id(self) -> text:
        return self.name
```

Interface သည် contract ကို ကြေညာပြီး implementation မပေးပါ။ `id` မပါသော class ကို instantiate မလုပ်နိုင်သကဲ့သို့ `Identifiable` လိုအပ်သည့်နေရာတွင်လည်း သုံးမရပါ။ နောက်ဆုံး static/dynamic dispatch ဆုံးဖြတ်ချက်နှင့်အညီ အသေးစိတ်သတ်မှတ်ရမည်။

### ၆.၃ Explicit conflict selection

```zap
trait JsonView:
    fn render(self) -> text:
        return json(self.data)

trait TableView:
    fn render(self) -> text:
        return join(self.columns, " | ")

class Report with JsonView, TableView:
    use JsonView.render as render
```

`use Trait.method as name` သည် အဆိုပြု explicit selection form ဖြစ်သည်။ နောက်ဆုံး syntax spelling ပြောင်းလဲနိုင်သော်လည်း semantic requirement သည် မပြောင်းပါ။ Conflict ကို source order ဖြင့် မခန့်မှန်းဘဲ declaration site တွင် explicit ဖြေရှင်းရမည်။

## ၇။ Method lookup နှင့် linearization

ကနဦး lookup rule သည် deterministic ဖြစ်ပြီး diagnostic တွင် ရှင်းပြရလွယ်ကူရမည်။

၁။ တောင်းဆိုထားသော method ကို concrete class တွင် ရှာပါ။
၂။ Class က explicit ရွေးထားသော trait method များကို ရှာပါ။
၃။ Candidate တစ်ခုတည်းသာ ကျန်လျှင် declaration order အတိုင်း composed trait များမှ ရှာပါ။
၄။ လက်ရှိ `extends` semantics အတိုင်း single class parent တွင် ရှာပါ။
၅။ Candidate မရှိလျှင် missing-method diagnostic ပြပါ။
၆။ Explicit selection မရှိဘဲ provided candidate များစွာကျန်လျှင် conflict diagnostic ပြပါ။

Explicit class method သည် provided trait method ထက် အမြဲအနိုင်ရမည်။ Explicit selection သည် မရွေးထားသော provided method များထက် အနိုင်ရမည်။ Parent class သည် class-level conflict ကို တိတ်တဆိတ် override မလုပ်ရ။ Implicit diamond linearization သို့မဟုတ် C3-style multiple-parent order ကို မအဆိုပြုပါ၊ အကြောင်းမှာ class parent တစ်ခုသာရှိပြီး composed unit များသည် stateless ဖြစ်သောကြောင့် ဖြစ်သည်။

### ၇.၁ Lookup ဥပမာ

```zap
trait Printable:
    fn format(self):
        return "trait"

class Invoice with Printable:
    fn format(self):
        return "invoice"
```

`Invoice.format` သည် class method သို့ resolve ဖြစ်သည်။ Class method ကို ဖယ်ပြီး `Printable` တစ်ခုတည်း compose လုပ်လျှင် provided trait method ကို ရွေးသည်။ `format` ကို provided လုပ်သော trait နှစ်ခု compose လုပ်လျှင် explicit selection မရှိသရွေ့ declaration ကို reject လုပ်ရမည်။

### ၇.၂ `super` နှင့် explicit trait call

`super` သည် single parent-class path ကိုသာ ဆက်လက်ဆိုလိုရမည်။ Trait method သည် မတော်တဆ second parent မဖြစ်ရ။ အနာဂတ် design တွင် selected trait implementation ကို ခေါ်ခွင့်ပေးပါက `JsonView.render(self)` ကဲ့သို့ trait name ဖြင့် explicit ခေါ်ရမည်။ Checker သည် class က ထို trait ကို compose လုပ်ထားကြောင်း စစ်ရမည်။ ထို rule သည် hidden lookup path များကို ကာကွယ်ပြီး refactoring ကို မြင်သာစေသည်။

## ၈။ Visibility နှင့် ownership

အဆိုပြုချက်သည် public/private member ခွဲခြားမှုကို ထိန်းသိမ်းသည်။ Composed unit သည် public method များကို contract အဖြစ် ဖော်ပြနိုင်သော်လည်း private helper များကို ထို unit အတွင်းတွင်သာ အသုံးပြုနိုင်ရမည်။ Public method တစ်ခုက private helper ကို ခေါ်သောကြောင့် ထို helper သည် public class method မဖြစ်ရ။

| Member | Declared in | Composing class ထံ မြင်နိုင်မှု | External caller ထံ မြင်နိုင်မှု |
|---|---|---:|---:|
| Public required method | Interface/trait contract | ရှိ | Receiver contract ခွင့်ပြုလျှင် ရှိ |
| Public provided method | Trait | ရှိ | Composed နှင့် exported ဖြစ်လျှင် ရှိ |
| Private trait helper | Trait | ပုံမှန်အားဖြင့် မရှိ | မရှိ |
| Class-private method | Class | Class rule အတွင်း ရှိ | မရှိ |
| Parent protected/public method | Parent class | လက်ရှိ inheritance rule အတိုင်း | လက်ရှိ inheritance rule အတိုင်း |

Trait သည် field ကြေညာခွင့်ရှိ/မရှိကို နောက်ဆုံး implementation တွင် သတ်မှတ်ရမည်။ ဤ RFC သည် feature v1 တွင် **implicit instance field မရှိစေရန်** အကြံပြုသည်။ State ကို class က ပိုင်ဆိုင်ပြီး trait သည် `name(self)` သို့မဟုတ် `data(self)` ကဲ့သို့ accessor method ကိုသာ လိုအပ်စေသင့်သည်။

## ၉။ Missing နှင့် conflicting implementation diagnostic

Diagnostic များသည် နောက်ဆက်တွဲအရာမဟုတ်ဘဲ language contract ၏ အစိတ်အပိုင်းဖြစ်သည်။ Parser/runtime မစမီ stable code များကို သတ်မှတ်ရမည်။

| အခြေအနေ | အဆိုပြု code | Diagnostic ထဲ လိုအပ်သည့်အကြောင်းအရာ |
|---|---|---|
| Required method မရှိခြင်း | `ZAP-TRAIT-001` | Composing class၊ required method၊ declaring trait/interface၊ source span နှင့် implementation signature အကြံပြုချက် |
| Provided method conflict | `ZAP-TRAIT-002` | Class၊ method၊ provider name/span အားလုံးနှင့် explicit-selection အကြံပြုချက် |
| Trait/interface target မမှန်ခြင်း | `ZAP-TRAIT-003` | Target name၊ မျှော်မှန်းထားသော contract kind၊ source span နှင့် ရရှိနိုင်သော declaration များ |
| Private member access | `ZAP-TRAIT-004` | Member၊ declaring unit၊ caller context နှင့် visibility ရှင်းလင်းချက် |
| Explicit selection မမှန်ခြင်း | `ZAP-TRAIT-005` | Selected unit/method၊ composition declaration နှင့် valid candidate များ |
| Feature version မထောက်ပံ့ခြင်း | `ZAP-TRAIT-006` | Syntax၊ current version၊ approved ဖြစ်ပါက first supported version နှင့် migration hint |

CLI check၊ runtime boundary နှင့် LSP consumer များတွင် diagnostic ကို တစ်ပြေးညီ ထုတ်ရမည်။ Code တစ်ခုချင်းစီအတွက် English/Burmese documentation နှင့် durable conformance fixture ရှိပြီးမှသာ feature enable လုပ်ရမည်။

## ၁၀။ Inheritance မှ migration

Migration သည် opt-in ဖြစ်ပြီး structure မကောင်းမွန်မီ behavior ကို ထိန်းသိမ်းရမည်။ အကြံပြုအဆင့်များမှာ အောက်ပါအတိုင်းဖြစ်သည်။

၁။ လက်ရှိ parent class ကို ထားပြီး capability ဖြစ်လာမည့် behavior အတွက် test ရေးပါ။
၂။ Method name နှင့် visibility မပြောင်းဘဲ stateless method များကို trait အဖြစ် ခွဲထုတ်ပါ။
၃။ လိုအပ်ပါက inherited call များကို class-owned state accessor များဖြင့် explicit ပြောင်းပါ။
၄။ Trait contract နှင့် conflict check pass ပြီးမှ `with TraitName` ကို ထည့်ပါ။
၅။ Relationship သည် အမှန်တကယ် subtype ဖြစ်နေသေးလျှင် `extends Parent` ကို ထားပါ။
၆။ Parity fixture pass ပြီးမှ duplicate method များကို ဖယ်ရှားပါ။

### ၁၀.၁ Migration ဥပမာ

မပြောင်းမီ —

```zap
class PrintableReport extends Report:
    fn format(self):
        return json(self.data)
```

Feature ကို approve လုပ်ပြီးနောက် —

```zap
trait JsonPrintable:
    fn format(self):
        return json(self.data)

class PrintableReport extends Report with JsonPrintable:
    pass
```

v2.2.0 တွင် ဤ migration သည် source-compatible မဟုတ်ပါ၊ အကြောင်းမှာ ဤပုံစံရှိ `trait`၊ `with` နှင့် `pass` များသည် proposed syntax သာဖြစ်သောကြောင့် ဖြစ်သည်။ နောက်ပိုင်း implementation သည် versioned migration tool သို့မဟုတ် ရှင်းလင်းသော diagnostic ပေးရမည်၊ older runtime တွင် syntax ကို တိတ်တဆိတ် လက်မခံရ။

## ၁၁။ Static နှင့် dynamic dispatch

ဤ RFC သည် **hybrid boundary** ကို အကြံပြုသည်။

| Call site | ဦးစားပေး dispatch | အကြောင်းရင်း |
|---|---|---|
| Statically checked interface-typed parameter | Static conformance check ပြီးနောက် direct selected method | Missing/conflict diagnostic ကို စောစီးစွာ ရပြီး hot path ကို ခန့်မှန်းလွယ်စေသည် |
| Concrete class method call | လက်ရှိ class/parent lookup | လက်ရှိ behavior နှင့် compatibility ကို ထိန်းသည် |
| `any` ဖြင့် annotate လုပ်ထားသော value သို့မဟုတ် dynamically loaded value | Conformance table မှ dynamic dispatch | Zap ၏ dynamic boundary ကို ထိန်းပြီး statically သိရှိထားသည်ဟု မဟန်ဆောင်ပါ |
| LSP completion/hover | Canonical catalog/AST မှ contract metadata | Editor behavior သည် parser-owned နှင့် deterministic ဖြစ်ရမည် |

ဤနေရာတွင် static dispatch ဆိုသည်မှာ whole-program compilation ကတိမဟုတ်ဘဲ contract validation ကို ဆိုလိုသည်။ Dynamic dispatch သည် လက်ရှိ runtime boundary တွင် stable `NameError`၊ `TypeError` သို့မဟုတ် trait-specific diagnostic များကို ထိန်းရမည်။ v2.2.0 တွင် type-system phase အသစ်ထည့်ရန် မလိုအပ်ပါ။

## ၁၂။ Package နှင့် version compatibility

Traits နှင့် interfaces သည် public API၊ method lookup၊ diagnostics၊ package metadata နှင့် editor tooling များကို သက်ရောက်စေသည်။ ထို့ကြောင့် approved implementation တစ်ခုအတွက် language specification၊ bilingual syntax guide၊ package-author guidance၊ standard-library stability record၊ LSP metadata၊ conformance ownership နှင့် changelog များကို တစ်ပြိုင်နက် update လုပ်ရမည်။

| Change | Compatibility classification |
|---|---|
| လက်ရှိ `class` နှင့် single `extends` behavior | v2.2.0 တွင် normative ဖြစ်ပြီး မပြောင်းပါ |
| Approval မရမီ `trait`/`interface`/`with` syntax | v2.2.0 parser က reject လုပ်ရမည် |
| နောက်ပိုင်း minor release တွင် approved additive syntax | Explicit capability/version metadata ပါသော feature အသစ် |
| လက်ရှိ method lookup ကို ပြောင်းခြင်း | Breaking semantics ဖြစ်၍ major-version သို့မဟုတ် compatibility layer ဆုံးဖြတ်ချက် လိုသည် |
| `extends` ကို ဖယ်ရှားခြင်း သို့မဟုတ် ပြောင်းခြင်း | Breaking change ဖြစ်၍ migration plan နှင့် explicit major-version decision လိုသည် |
| Enablement ပြီးနောက် conflict diagnostic ပြောင်းခြင်း | Compatibility-sensitive ဖြစ်၍ stable code နှင့် migration note လိုသည် |

Standard-library stability policy အရ public API များတွင် stability၊ introduction release၊ deprecation၊ platform၊ limits၊ timeout/error နှင့် determinism metadata များ မှတ်တမ်းတင်ရသည် [3]။ Trait-backed stdlib API အနာဂတ်တွင် ထို record များကို release မတိုင်မီ update လုပ်ရမည်။

## ၁၃။ ပယ်ချထားသော အခြားရွေးချယ်စရာများ

### ၁၃.၁ Multiple inheritance

ကနဦး design အတွက် ပယ်ချသည်။ Object layout၊ constructor order၊ `super` semantics၊ diamond lookup နှင့် diagnostic များ ပိုမိုရှုပ်ထွေးစေသောကြောင့် ဖြစ်သည်။ Nominal parent တစ်ခုနှင့် stateless composition သည် semantic surface ပိုသေးစေသည်။

### ၁၃.၂ Object အားလုံးအတွက် implicit structural typing

လက်ရှိ annotation ၏ အဓိပ္ပာယ်ကို ပြောင်းလဲပြီး accidental conformance ကို ရှာဖွေရှင်းလင်းရန် ခက်ခဲစေသောကြောင့် ပယ်ချသည်။ Explicit interface အတွက် opt-in structural check ကို နောက်ပိုင်းတွင် စဉ်းစားနိုင်သော်လည်း default မဖြစ်ရ။

### ၁၃.၃ Source order ဖြင့် trait method ရွေးခြင်း

Declaration reorder လုပ်ခြင်းဖြင့် behavior တိတ်တဆိတ်ပြောင်းသွားနိုင်သောကြောင့် ပယ်ချသည်။ Conflict သည် explicit နှင့် reviewable ဖြစ်ရမည်။

### ၁၃.၄ Trait က field များကို တိတ်တဆိတ် ထည့်ခြင်း

Field ownership၊ initialization order၊ serialization နှင့် memory lifecycle များ implicit ဖြစ်သွားနိုင်သောကြောင့် ကနဦး version အတွက် ပယ်ချသည်။ Accessor requirement များသည် audit လုပ်ရလွယ်သည်။

### ၁၃.၅ Runtime-only conflict error

Missing/conflicting implementation များကို ရရှိနိုင်သည့် information အတိုင်း စောစီးစွာ report လုပ်သင့်သောကြောင့် ပယ်ချသည်။ CLI နှင့် LSP တွင် structured contract တူညီရမည်။

### ၁၃.၆ RFC review မပြီးမီ traits implementation လုပ်ခြင်း

Project policy အရ ပယ်ချသည်။ Conformance၊ specification ownership နှင့် bilingual documentation gate များ မပြီးမချင်း parser/runtime တွင် broad language syntax မထည့်ရ။

## ၁၄။ Approval ပြီးနောက် implementation gate များ

ဤ RFC သည် design milestone အဖြစ် ပြီးစီးရန် implementation မစမီ အောက်ပါ gate များ လိုအပ်သည်။

| Gate | လိုအပ်သည့် evidence |
|---|---|
| Specification | Ownership ID ပါသော canonical English/Burmese rule section များ |
| Syntax | Proposed form တစ်ခုချင်းစီအတွက် parser accept/reject fixture |
| Lookup | Class၊ parent၊ single-provider၊ conflict နှင့် explicit-selection test များ |
| Visibility | Public/private နှင့် external-access fixture များ |
| Diagnostics | Stable `ZAP-TRAIT-*` code၊ JSON field နှင့် CLI/LSP parity test များ |
| Migration | Before/after example နှင့် compatibility/deprecation note များ |
| Dispatch | Static/dynamic boundary test နှင့် documented limitation များ |
| Packages | Manifest နှင့် registry consumer အတွက် trait/interface metadata rule များ |
| Tooling | Completion၊ hover၊ definition၊ rename နှင့် formatting parity fixture များ |
| Platforms | Runtime state သက်ရောက်ပါက Linux၊ Windows နှင့် macOS native behavior |
| Release | Changelog၊ bilingual docs၊ version decision နှင့် full quality gate များ |

ဤ RFC တစ်ခုတည်းဖြင့် implementation commit ကို လက်မခံရ။ နောက်ပိုင်း implementation milestone သည် ဤစာတမ်းကို ရည်ညွှန်းပြီး specification ownership index ကို update လုပ်ရမည်။

## ၁၅။ Explicit version decision

**v2.2.0 အတွက် ဆုံးဖြတ်ချက်:** Traits၊ interfaces၊ composition syntax၊ conflict-resolution syntax အသစ်နှင့် ဆက်စပ် parser/runtime behavior များသည် **deferred** ဖြစ်သည်။ v2.2.0 release တွင် ဤ RFC ကို reviewed design record အဖြစ် ထည့်သွင်းနိုင်သော်လည်း proposed syntax ကို supported ဟု မကြေညာရ။

**အနာဂတ်ဆုံးဖြတ်မည့်အချိန်:** နောက် release proposal တစ်ခုသည် RFC review ပြီး၊ diagnostic နှင့် lookup rule များ freeze လုပ်ပြီး၊ bilingual contract update နှင့် conformance fixture များ supported target အားလုံးတွင် pass ပြီးမှ additive subset တစ်ခုကို enable လုပ်ရန် စဉ်းစားနိုင်သည်။ လက်ရှိ inheritance semantics ပြောင်းလဲမှုတိုင်းအတွက် သီးခြား compatibility နှင့် major-version decision လိုအပ်သည်။

## References

[1]: LANGUAGE_SPEC_MM.md — Zap canonical language specification နှင့် ownership boundary များ။
[2]: MEMORY_MODEL_MM.md — Zap ownership နှင့် single-threaded object-field boundary များ။
[3]: STDLIB_POLICY_MM.md — Zap public API stability နှင့် release policy။
[4]: COMPATIBILITY_CHANGE_TEMPLATE_MM.md — လိုအပ်သော compatibility/deprecation change record။
[5]: SPEC_OWNERSHIP_MM.md — Rule-to-section-to-fixture ownership contract။
