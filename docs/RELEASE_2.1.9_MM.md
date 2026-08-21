# Zap v2.1.9 Release Notes

**Release date:** 2026-08-21

Zap v2.1.9 သည် runtime-safety ကို အာရုံစိုက်ထားသော patch release ဖြစ်ပါသည်။ Single-threaded `Rc<RefCell>` boundary အတွင်း object-field borrow conflict များကို ထိန်းချုပ်ပြီး မထိန်းချုပ်နိုင်သော panic အစား deterministic structured error အဖြစ် ပြန်ပေးနိုင်အောင် ပြင်ဆင်ထားပါသည်။

## အဓိကပြောင်းလဲမှုများ

- Object field များအတွက် checked `try_borrow` နှင့် `try_borrow_mut` accessor များ ထည့်သွင်းထားပါသည်။
- Compatibility code `ZAP-BORROW-001` ပါသော stable `BorrowError` diagnostic၊ deterministic note၊ help text၊ source-location support နှင့် JSON rendering ကို ထည့်သွင်းထားပါသည်။
- Object-field boundary တွင် `clear_object_fields()` နှင့် `object_field_count()` ကို fallible ပြုလုပ်ပြီး conflict ဖြစ်ပါက panic မဖြစ်ဘဲ error ပြန်ပေးနိုင်ပါသည်။
- Recursive JSON conversion၊ object-field initialization၊ property assignment၊ property lookup နှင့် memory validation လမ်းကြောင်းများတွင် checked borrow failure ကို ဆက်လက် propagate လုပ်ထားပါသည်။
- Conflicting object borrow၊ JSON conversion failure propagation နှင့် stable BorrowError metadata များအတွက် regression tests များ ထည့်သွင်းထားပါသည်။
- English/Burmese memory model၊ structured diagnostic model၊ roadmap၊ release policy၊ README onboarding၊ SECURITY release reference နှင့် type-check conformance baseline များကို update လုပ်ထားပါသည်။

## Contract boundaries

ဤ patch သည် tracing garbage collector၊ public weak-reference API၊ process-wide heap telemetry၊ per-run byte accounting သို့မဟုတ် arbitrary object cycle များကို အလိုအလျောက် reclaim လုပ်ခြင်း ရှိသည်ဟု မဆိုလိုပါ။ Closure `RefCell` ownership၊ real async scheduling၊ host I/O isolation နှင့် OS-level sandboxing တို့သည် သီးခြား roadmap milestone များအဖြစ် ဆက်လက်ရှိနေပါသည်။

## Verification

Publication မပြုမီ native Rust suite၊ strict formatting နှင့် Clippy gate များ၊ version consistency validator၊ positive/negative version regression harness နှင့် cross-platform GitHub Actions matrix တို့ကို အောင်မြင်ရမည်။ Release workflow သည် tagged source ကို စစ်ဆေးပြီး Linux x86_64၊ Windows x86_64 နှင့် macOS ARM64 artifacts များကို build လုပ်ကာ checksum/signature ကို verify ပြီး provenance ကို publish လုပ်ရမည်။

Normative contract များအတွက် [Burmese memory model](MEMORY_MODEL_MM.md)၊ [Burmese diagnostic model](DIAGNOSTIC_MODEL_MM.md) နှင့် [release version policy](RELEASE_VERSION_POLICY_MM.md) ကို ကြည့်ရှုနိုင်ပါသည်။
