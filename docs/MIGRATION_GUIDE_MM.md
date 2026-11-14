# Zap ဗားရှင်် ရွှေ့ပြောင်းရန် လမ်းညွှန်

ဒီလမ်းညွှန်က Zap ပရောဂျက်များကို မေဂျာ ဗားရှင်းများကြား ရွှေ့ပြောင်းရာတွင် ကူညီပေးပါသည်။ အပိုင်းတစ်ခုစီတွင် breaking changes၊ လက်ရှိ features နှင့် ရွှေ့ပြောင်းရန် အဆင့်များ ပါဝင်သည်။

## အကြောင်းအရာများ မျက်နှာစာ

- [v2.11.x မှ v2.12.x](#v211x-မှ-v212x)
- [v2.10.x မှ v2.11.x](#v210x-မှ-v211x)
- [v2.2.x မှ v2.3.x](#v22x-မှ-v23x)
- [v2.1.x မှ v2.2.x](#v21x-မှ-v22x)
- [v2.0.x မှ v2.1.x](#v20x-မှ-v21x)
- [v0.x မှ v2.0.x](#v0x-မှ-v20x)

---

## v2.11.x မှ v2.12.x

**အခြေအနေ:** v2.12 ကို မထုတ်ပြီးသေးပါ။ v2.12 ထုတ်သည့်အခါ ဒီအပိုင်းကို အပ်ဒိတ်လုပ်ပေးပါမည်။

**မျှော်လင့်ထားသော ပြောင်းလဲမှုများ:**
- B4 self-hosting certification အပ်ဒိတ်များ
- နောက်ထပ် Zap-owned C backend migration slices များ
- တိုးတက်သော type-checker generic support

**ရွှေ့ပြောင်းရန် အဆင့်များ:**
- ရွှေ့ပြောင်းရန် အဆင့်များ မရှိသေးပါ

---

## v2.10.x မှ v2.11.x

**ထုတ်ဝေမှု:** [v2.11.18](RELEASE_2.11.18_MM.md)

### Breaking Changes

မရှိပါ။ v2.11.x သည် v2.10.x စီးရီး၏ non-breaking continuation ဖြစ်သည်။

### လက်ရှိ Features

- B2 type checker: complete generic constraints, compound bounds, alias checking
- B1 lexer/parser: expanded arbitrary block coverage
- B3 package/build: Zap-side package resolver
- B4 self-hosting: deterministic rebuild evidence

### ရွှေ့ပြောင်းရန် အဆင့်များ

ရွှေ့ပြောင်းရန် မလိုအပ်ပါ။ v2.11.x သည် v2.10.x နှင့် backward compatible ဖြစ်သည်။

```bash
# v2.11.18 သို့ update လုပ်ပါ
zap update
```

---

## v2.2.x မှ v2.3.x

**ထုတ်ဝေမှု:** [v2.3.0](RELEASE_2.3.0_MM.md)

### Breaking Changes

မှတ်တမ်းတင်ထားသည် မရှိပါ။ v2.3.x သည် non-breaking continuation ဖြစ်သည်။

### လက်ရှိ Features

- Web framework foundations
- Database provider strategy
- တိုးတက်သော async runtime

### ရွှေ့ပြောင်းရန် အဆင့်များ

ရွှေ့ပြောင်းရန် မလိုအပ်ပါ။

```bash
# v2.3.0 သို့ update လုပ်ပါ
zap update
```

---

## v2.1.x မှ v2.2.x

**ထုတ်ဝေမှု:** [v2.2.0](RELEASE_2.2.0_MM.md) နှင့် [v2.2.1](RELEASE_2.2.1_MM.md)

**အရေးကြီးသည်:** v2.2.1 ပြုပြင်မှုများအတွက် [Post-v2.2.0 Remediation Record](POST_V2.2.0_REMEDIATION_MM.md) ကို ကြည့်ပါ။

### Breaking Changes

#### LSP Synchronization (v2.2.1 ပြုပြင်မှု)

- **ပြောင်းလဲမှု:** LSP server သည် ယခု standard full-text synchronization ကို လိုအပ်သည်
- **သက်ရောက်မှု:** VS Code extension clients သည် changes တွင် full document content ကို ပို့ရမည်
- **ရွှေ့ပြောင်းမှု:** VS Code extension ကို v2.2.1 သို့မဟုတ် ထို့ထက်ကြီးသော version သို့ update လုပ်ပါ

#### LSP Rename (v2.2.1 ပြုပြင်မှု)

- **ပြောင်းလဲမှု:** Rename သည် ယခု file-local only (cross-file rename မရှိ)
- **သက်ရောက်မှု:** Cross-file rename operations များကို ယခု မပံ့ပိုးတော့ပါ
- **ရွှေ့ပြောင်းမှု:** Cross-file symbol changes အတွက် manual edits ကို သုံးပါ

#### Standard Library Determinism (v2.2.1 ပြုပြင်မှု)

- **ပြောင်းလဲမှု:** Standard library သည် ယခု boolean အစား determinism class ကို ပြန်ပေးသည်
- **သက်ရောက်မှု:** `deterministic: true` boolean ပေါ်မူတည်သော code များသည် update လိုနိုင်သည်
- **ရွှေ့ပြောင်းမှု:** `determinism_class` field ကို အသုံးပြုပါ: `pure`, `input-deterministic`, `runtime-dependent`, `external-io`

### ရွှေ့ပြောင်းရန် အဆင့်များ

```bash
# VS Code extension ကို update လုပ်ပါ
zap update editors/vscode

# LSP synchronization changes ကို စစ်ဆေးပါ
# - Full-text sync ကို ယခု လိုအပ်သည်
# - Incremental range changes ကို ပယ်ချသည်

# Standard library checks ကို update လုပ်ပါ
# - `deterministic: true` checks ကို `determinism_class in ["pure", "input-deterministic"]` ဖြင့် အစားထိုးပါ
```

---

## v2.0.x မှ v2.1.x

**ထုတ်ဝေမှု:** [v2.1.0](RELEASE_2.1.0_MM.md)

### Breaking Changes

#### Legacy Runtime Deprecation

- **ပြောင်းလဲမှု:** Python legacy runtime သည် ယခု compatibility reference only ဖြစ်သည်
- **သက်ရောက်မှု:** လက်ရှိ features များသည် native-only ဖြစ်သည်
- **ရွှေ့ပြောင်းမှု:** အသစ် code အားလုံးအတွက် native runtime ကို သုံးပါ

Detailed native-only fixture handling အတွက် [P0-01 Migration Guidance](P001_MIGRATION_GUIDANCE_MM.md) ကို ကြည့်ပါ။

### လက်ရှိ Features

- B1 bootstrap: lexer နှင့် parser ownership
- B2 bootstrap: type checker နှင့် typed-IR
- Package manager foundation
- Standard library expansion

### ရွှေ့ပြောင်းရန် အဆင့်များ

```bash
# v2.1.0 သို့ update လုပ်ပါ
zap update

# Legacy-to-native migration ကို စစ်ဆေးပါ
# - conformance/p0-01/matrix.tsv တွင် native-only fixtures ကို စစ်ဆေးပါ
# - Modern declarations ကို native runtime သုံးအောင် update လုပ်ပါ
# - Legacy-only APIs ကို native equivalents ဖြင့် အစားထိုးပါ

# Parity checks ကို run လုပ်ပါ
scripts/test_p001_parity.sh
```

---

## v0.x မှ v2.0.x

**ထုတ်ဝေမှု:** [v2.0.0](https://github.com/hidecard/zap/releases/tag/v2.0.0)

### Breaking Changes

#### Complete Language Rewrite

- **ပြောင်းလဲမှု:** v2.0 သည် Zap language ၏ complete rewrite ဖြစ်သည်
- **သက်ရောက်မှု:** v0.x code သည် v2.0 နှင့် compatible မဖြစ်ပါ
- **ရွှေ့ပြောင်းမှု:** Manual code rewrite လိုအပ်သည်

#### Syntax Changes

- **ပြောင်းလဲမှု:** declarations, functions, classes အတွက် လက်ရှိ syntax
- **သက်ရောက်မှု:** v0.x syntax အားလုံးသည် v2.0 တွင် invalid ဖြစ်သည်
- **ရွှေ့ပြောင်းမှု:** v2.0 syntax guide သုံးပြီး code ကို ပြန်ရေးပါ

#### Type System

- **ပြောင်းလဲမှု:** Static type checking သည် ယခု mandatory ဖြစ်သည်
- **သက်ရောက်မှု:** Untyped v0.x code သည် compile မဖြစ်နိုင်ပါ
- **ရွှေ့ပြောင်းမှု:** ခင်းပြီး function နှင့် variable များတွင် type annotations ထည့်ပါ

### ရွှေ့ပြောင်းရန် အဆင့်များ

v0.x မှ v2.0 သည် major rewrite ဖြစ်သည်။ Automated migration path မရှိပါ။

```bash
# v2.0 ကို install လုပ်ပါ
# README တွင် installation guide ကို လိုက်နာပါ

# v2.0 syntax သုံးပြီး code ကို ပြန်ရေးပါ
# - Syntax reference အတွက် LANGUAGE_SPEC_MM.md ကို ကြည့်ပါ
# - Tutorial အတွက် LEARN_ZAP_MM.md ကို ကြည့်ပါ

# Type annotations ထည့်ပါ
# - ခင်းပြီး function သည် parameter နှင့် return types ရှိရမည်
# - Type inference ကို ဖြစ်နိုင်သလောက် သုံးပါ

# Native runtime ဖြင့် test လုပ်ပါ
zap test
```

### ရွှေ့ပြောင်းမှု ဥပမာ

**v0.x code:**
```python
# v0.x - dynamic typing
def add(a, b):
    return a + b

result = add(1, 2)
print(result)
```

**v2.0 code:**
```zap
# v2.0 - static typing
fn add(a: int, b: int) -> int {
    return a + b
}

let result = add(1, 2)
print(result)
```

---

## ယေဘုယျ ရွှေ့ပြောင်းရန် Checklist

မည်သည့် version transition အတွက်မဆို:

1. **သင့် project ကို backup လုပ်ပါ**
   ```bash
   git commit -m "Backup before Zap migration"
   ```

2. **Release notes ကို ဖတ်ပါ**
   - Breaking changes များကို စစ်ဆေးပါ
   - လက်ရှိ features များကို စစ်ဆေးပါ
   - Deprecation warnings များကို မှတ်ထားပါ

3. **Dependencies ကို update လုပ်ပါ**
   ```bash
   zap update
   ```

4. **သင့် code ကို update လုပ်ပါ**
   - Breaking changes များကို အသုံးပြုပါ
   - Deprecated APIs များကို အစားထိုးပါ
   - လိုအပ်သော type annotations များကို ထည့်ပါ

5. **Tests ကို run လုပ်ပါ**
   ```bash
   zap test
   ```

6. **Linting ကို စစ်ဆေးပါ**
   ```bash
   zap check
   ```

7. **သင့် project ကို build လုပ်ပါ**
   ```bash
   zap build
   ```

8. **Local တွင် test လုပ်ပါ**
   ```bash
   zap run
   ```

---

## အကူအညီ ရယူရန်

ရွှေ့ပြောင်းရာတွင် ပြဿနာတက်ပါက:

- [Language Specification](LANGUAGE_SPEC_MM.md) ကို စစ်ဆေးပါ
- [Release Notes](RELEASE_2.11.18_MM.md) ကို စစ်ဆေးပါ
- [Migration Guidance](P001_MIGRATION_GUIDANCE_MM.md) ကို တိုင်းပါ
- [GitHub](https://github.com/hidecard/zap/issues) တွင် issue ဖွင့်ပါ

---

## Version Support Policy

| Version | Support Status | End of Life |
|---------|----------------|-------------|
| v2.11.x | Current | TBD |
| v2.10.x | Maintenance | v2.12 release ပြီးနောက် |
| v2.2.x - v2.9.x | Security only | v2.12 release ပြီးနောက် |
| v2.0.x - v2.1.x | End of life | v2.10 release |
| v0.x | End of life | v2.0 release |

---

## ကိုးကားချက်များ

- [Language Specification](LANGUAGE_SPEC_MM.md)
- [Language Specification (English)](LANGUAGE_SPEC_EN.md)
- [Current Status](CURRENT_STATUS_MM.md)
- [Current Status (English)](CURRENT_STATUS_EN.md)
- [Release Version Policy](RELEASE_VERSION_POLICY_MM.md)
- [Compatibility Matrix](COMPATIBILITY_MATRIX_MM.md)
