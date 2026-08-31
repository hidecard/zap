# B2 Milestone TODO Plan

**Created:** 2026-08-31  
**Current stage:** B1 lexer milestone reached; B2 remains candidate/provisional  
**Target:** B2 Zap-owned type checker and validated typed IR  
**Source plan:** `pasted_content.txt` နှင့် `SECTION_A_NEXT10_QUEUE.md`

## ရည်ရွယ်ချက်

B2 milestone ၏ အဓိကရည်ရွယ်ချက်မှာ parser မှရရှိသော AST ကို Zap type checker ဖြင့် စစ်ဆေးပြီး deterministic typed IR အဖြစ် ထုတ်ပေးနိုင်ရန် ဖြစ်သည်။ Native Rust implementation သည် differential/reference oracle အဖြစ်သာ ဆက်လက်အသုံးပြုမည်ဖြစ်ပြီး B2 candidate သည် Rust သို့ fallback မလုပ်ရပါ။

## Acceptance gates

| Gate | ဖြည့်ဆည်းရမည့်အချက် | Evidence |
|---|---|---|
| B2-TYPE-01 | Primitive, collection, option/result, function-call နှင့် return inference | `verify_b2_typecheck.sh`, `verify_b2_typecheck_candidate.sh` |
| B2-TYPE-02 | Branch/loop flow-sensitive narrowing၊ reassignment invalidation နှင့် scope restoration | `verify_b2_flow_sensitive_10.sh`, `verify_b2_recursive_cfg_loop_convergence_12.sh` |
| B2-TYPE-03 | Generic substitution၊ bounds နှင့် cross-module signature checks | `verify_b2_generic_end_to_end_10.sh`, `verify_b2_generic_bounds_10.sh` |
| B2-IR-01 | Typed-IR node shape၊ deterministic output နှင့် repeated-run equality | `verify_b2_typed_ir_candidate.sh`, `verify_b2_typed_ir_owned_program_38.sh` |
| B2-IR-02 | Every emitted node has valid span/type metadata; malformed input returns stable diagnostics | typed-IR contract checks and diagnostic fixtures |
| B2-OWN-01 | B2 type checker/typed IR ownership is recorded without claiming B2 completion prematurely | `bootstrap/contracts/OWNERS.tsv`, `bootstrap/contracts/VERSIONS.toml` |
| B2-CI-01 | Consolidated B2 gate runs in CI and emits a durable TSV report | `verify_b2_milestone.sh`, `target/b2-milestone-report.tsv` |

## Immediate TODO queue

- [ ] Run and record all B2 candidate suites on the current toolchain.
- [ ] Fix the reproducible `index out of range` failure in the B2 type-checker candidate runner.
- [ ] Add typed-IR validation as an explicit pass, not only fixture-level shape assertions.
- [ ] Add deterministic B2 aggregate report with one row per gate and failure output retained.
- [ ] Transfer only verified B2 rules to Zap ownership; retain Rust as reference owner for unverified rules.
- [ ] Keep `bootstrap.stage = "B0"` until the full B2 acceptance gate, including no-Rust execution, passes.
- [ ] Run the complete CI-equivalent local matrix and rerun GitHub Actions after push.

## B2 definition of done

B2 သည် အောက်ပါအချက်အားလုံး ပြည့်စုံမှသာ complete ဟု သတ်မှတ်မည်။ Zap source သည် parser/type checker/typed-IR pipeline ကို Rust fallback မပါဘဲ run နိုင်ရမည်။ Repeated runs များ၏ typed-IR bytes သည် တူညီရမည်။ Valid AST construct တိုင်းတွင် type metadata နှင့် source span ပါရမည်။ Invalid program များသည် stable diagnostic code နှင့် deterministic location ပြန်ပေးရမည်။ Reference differential corpus နှင့် Zap-owned candidate corpus နှစ်မျိုးစလုံးသည် CI တွင် အောင်မြင်ရမည်။

## လက်ရှိအခြေအနေ

B2 fixture suites အများစုသည် အောင်မြင်နေသော်လည်း candidate type-checker runner တွင် reproducible `ProjectError: index out of range` ရှိနေပြီး full B2 completion မဟုတ်သေးပါ။ ထို့အပြင် latest GitHub Actions run သည် B1 parser candidate failure နှင့် missing evidence artifact uploads ကြောင့် failed ဖြစ်နေသည်။ ထို CI failures များကို B2 milestone push မတိုင်မီ သီးခြား blocker အဖြစ် ဆက်လက်မှတ်တမ်းတင်မည်။
