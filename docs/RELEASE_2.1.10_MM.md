# Zap v2.1.10 Release Notes

**Release date:** 2026-08-21

Zap v2.1.10 သည် release-engineering နှင့် documentation-quality milestone ဖြစ်ပါသည်။ Bilingual documentation baseline ကို ရှာဖွေရလွယ်ကူပြီး enforce လုပ်နိုင်အောင် ပြင်ဆင်ထားသည့်အပြင် language ၏ deferred trait သို့မဟုတ် broad async scope ကို မပြောင်းလဲဘဲ repeatable p95 benchmark regression protection ကို ထည့်သွင်းထားပါသည်။

## အဓိကပြောင်းလဲမှုများ

- Normative specification၊ runtime contract၊ verification evidence၊ release policy နှင့် contribution path များကို လွှမ်းခြုံသော English/Burmese documentation navigation landing page များကို ထည့်သွင်းထားပါသည်။
- Required-file၊ section-parity၊ code-fence-parity၊ stale-version နှင့် README navigation-link check များပါသော `scripts/validate_documentation_consistency.sh` ကို ထည့်သွင်းထားပါသည်။
- `scripts/test_validate_documentation_consistency.sh` မှတစ်ဆင့် positive နှင့် negative documentation-consistency regression coverage ကို ထည့်သွင်းထားပါသည်။
- Benchmark aggregation တွင် deterministic `p95_seconds` column ကို တိုးချဲ့ပြီး `ZAP_BENCH_WARMUPS` မှတစ်ဆင့် suite တစ်ခုချင်းစီအတွက် configurable warm-up iteration များကို ထည့်သွင်းထားပါသည်။
- Checked-in `benchmark-results/native-summary.csv` baseline နှင့် mean/p95 timing များကို configurable threshold ဖြင့် နှိုင်းယှဉ်သော `scripts/check_benchmark_regression.sh` ကို ထည့်သွင်းထားပါသည်။
- Documentation နှင့် benchmark gate များကို CI နှင့် release preflight တွင် ချိတ်ဆက်ပြီး TSV/log artifact evidence များကို ထည့်သွင်းထားပါသည်။
- English/Burmese syntax၊ language specification၊ async boundary၊ generic-type design၊ P2 progress၊ benchmark၊ README နှင့် changelog documentation များကို v2.1.10 release baseline သို့ update လုပ်ထားပါသည်။

## Contract boundaries

ဤ release သည် trait implementation အသစ်၊ broad language-level async scheduling syntax၊ tracing garbage collection၊ public weak reference သို့မဟုတ် per-run byte accounting အသစ်များကို မဆိုလိုပါ။ ထိုအရာများသည် roadmap တွင် explicit deferred item များအဖြစ် ဆက်လက်ရှိနေပါသည်။ Benchmark threshold သည် deterministic regression signal ဖြစ်ပြီး operating system သို့မဟုတ် hosted runner များအကြား wall-clock timing တူညီမည်ဟု အာမခံချက် မဟုတ်ပါ။

## Verification

Native Rust quality gate များဖြစ်သော rustfmt၊ `-D warnings` ပါ strict Clippy၊ all-target/all-feature test များ (unit test ၁၆၀ ခုနှင့် core integration test ၂၅၄ ခု) နှင့် `git diff --check` တို့ကို local တွင် အောင်မြင်စွာ run လုပ်ထားပါသည်။ GitHub Actions run `32513512535` တွင် documentation နှင့် benchmark quality job အပြင် Linux x86_64၊ Windows x86_64 နှင့် macOS ARM64 build/test job များ အားလုံး အောင်မြင်ပါသည်။ Release workflow run `32513839968` သည် v2.1.10 artifacts များကို validate၊ sign နှင့် publish အောင်မြင်စွာ ပြုလုပ်ထားပါသည်။

ထိန်းသိမ်းထားသော contract များအတွက် [documentation navigation hub](DOCUMENTATION_NAVIGATION_MM.md)၊ [benchmark harness contract](BENCHMARK_HARNESS_MM.md)၊ [release version policy](RELEASE_VERSION_POLICY_MM.md) နှင့် [Burmese language specification](LANGUAGE_SPEC_MM.md) ကို ကြည့်ရှုနိုင်ပါသည်။

## References

[1]: DOCUMENTATION_NAVIGATION_MM.md "Zap Burmese documentation navigation"
[2]: BENCHMARK_HARNESS_MM.md "Zap Burmese benchmark harness contract"
[3]: RELEASE_VERSION_POLICY_MM.md "Zap Burmese release version policy"
[4]: LANGUAGE_SPEC_MM.md "Zap Burmese language specification"
