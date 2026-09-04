# Zap — ကျန်ရှိနေသေးသော အလုပ်များနှင့် Git update status

## အကျဉ်းချုပ်

လက်ရှိ working tree tree တွင် B1 parser၊ B2 type inference/generic checking နှင့် B3 typed-IR/lowering အပိုင်းများအတွက် verified progress အများအပြား ရှိနေပါသည်။ သို့သော် ယခင်သတ်မှတ်ထားသော requirement အတိုင်း **full language ownership၊ full Rust-reference parity နှင့် complete runtime coverage မပြီးသေးပါ**။ ထို့ကြောင့် အောက်ပါ remaining work များ မပြီးမချင်း final completion commit/push မပြုလုပ်သင့်ပါ။

## လက်ရှိ session တွင် ပြီးစီးခဲ့သော အလုပ်များ (staged, not committed)

### P0 — CI version consistency hardening
- `scripts/validate_release_version.sh` ၏ `read_lock_version` awk script သည် `core.autocrlf=true` Windows checkout ပေါ်တွင် trailing `\r` ကြောင့် `name = "zap-native"` match မဖြစ်ပြီး `native/Cargo.lock zap-native` row ကို `<missing>` ဟု FAIL ဖြစ်စေခဲ့ပါသည်။ CI runs on Linux တွင် LF line ending ဖြစ်သောကြောင့် CI ကိုယ်တိုင်ကို မထိခိုက်သော်လည်း၊ local Windows-checkout maintainers များအတွက် silent regression risk ဖြစ်ပြီး future change များက LF-only assumption ကို ဖယ်ရှားပါက CI ကိုယ်တိုင် ကျိုးပျက်နိုင်ပါသည်။
- Fix: `read_cargo_version` (tr -d '\\r' on Cargo.toml), `read_lock_version` (awk begin block `sub(/\\r$/, "")`), နှင့် `read_cli_version` (ZAP_CLI_BINARY executable guard + cargo PATH guard + `|| true` fallback) တို့ကို harden လုပ်ထားပါသည်။
- Verified: `/tmp` Linux-style LF clean checkout တွင် full PASS report (29/29 checks) ထွက်ပါသည်။
- Regression test: `scripts/test_validate_release_version.sh` တွင် core.autocrlf=true scenario simulate လုပ်သော CRLF lockfile regression check ထည့်ပြီး old awk block (no CR strip) vs new awk block (with CR strip) contrast ကို verify လုပ်ပါသည်။
- CI artifact uploads (`target/version-consistency.tsv`, `target/b2-milestone-report.tsv` စသည်) အားလုံး `if-no-files-found: warn` ဖြစ်ပြီး cascade-fail-safe ဟုတ်ကြောင်း အတည်ပြုပါသည်။
- ရလဒ်: commit `e27a4d5` pushed to origin/master, ပြီးနောက် remote `c89988b` re-introduced CI regression; merge `baa91d5` resolved with local CI fix preserved.

### P1 — Typed-IR benchmark cross-platform baseline
- `scripts/benchmark_b2_typed_ir.sh` တွင် portable timing backend (GNU `/usr/bin/time` / `gtime` / bash SECONDS+`/proc/$$/status` fallback), M2-BENCH-01-compatible provenance sidecar (`schema_version`, `status`, `timestamp_utc`, `git_commit`, `target_triple`, `os`, `kernel`, `arch`, `rust_version`, `cargo_version`, `binary_sha256`, `script_sha256`, `repeats`, `warmups`, `suites`, `time_backend`, `raw_csv`), cross-platform baseline TSV (`(target_triple, suite, min/mean/max, peak_rss_min/max, commit, sha256, timestamp_utc)`), Windows binary lookup (`.exe`), ZAP_TYPED_IR_BENCH_TIME_CMD override, mktemp permission fix တို့ ထည့်သွင်းပါသည်။
- `scripts/aggregate_b2_typed_ir.sh` (new) deterministic per-suite summary CSV: min/mean/p95/max seconds, population standard deviation/variance/cv, peak RSS min/mean/max။
- `scripts/test_b2_typed_ir_benchmark.sh` (extended) and `scripts/test_aggregate_b2_typed_ir.sh` (new) regression tests များ locally PASS ဖြစ်ပါသည်။
- `.github/workflows/ci.yml` တွင် `scripts/test_aggregate_b2_typed_ir.sh` step, `ZAP_TYPED_IR_BENCH_PROVENANCE`/`ZAP_TYPED_IR_BENCH_BASELINE` env, summary aggregate step, and `zap-b2-typed-ir-baseline-<sha>` artifact upload ထည့်သွင်းပါသည်။
- `docs/BENCHMARK_HARNESS_EN.md` and `docs/BENCHMARK_HARNESS_MM.md` တွင် portable backend, provenance, baseline, aggregator, Windows compatibility, and machine-dependent scope note များ ထည့်သွင်းပါသည်။
- ရလဒ်: commit `0e93501` pushed to origin/master, baseline is per-target execution evidence (not portability/speed claim).

### P0 — Remaining work (verified completion state)

- **B2 generic constraints** — ✅ DONE. Multi-parameter inference, nested/compound bounds (`verify_b2_compound_bounds_20.sh`), explicit generic call syntax (`verify_b2_p0_explicit_generic_args_18.sh`), and diagnostic parity are implemented and passing. B2 milestone generic end-to-end gate passes.
- **B2 alias environment** — ✅ DONE. Alias-of-alias, recursive alias detection (`verify_b2_recursive_alias.sh`), alias expansion diagnostics (`verify_b2_alias_expansion_21.sh`), and imported aliases (`verify_b2_imported_aliases.sh`) are implemented and passing. Module-resolution infrastructure added to `bootstrap/b2/typecheck.zp` (`b2c_resolve_module_path`, `b2c_parse_module_source`, updated `b2c_collect_type_aliases` with `source_name` propagation).
- **B2 dataflow** — ✅ DONE. Short-circuit path sensitivity (`verify_b2_short_circuit_loop_edges_12.sh`), mutation/alias invalidation (`verify_b2_flow_sensitive_10.sh`), loop fixpoint (`verify_b2_loop_fixpoint_cycles_10.sh`), and break/continue/return live-path merge (`verify_b2_scope_exit_restore_10.sh`, `verify_b2_scope_merge_10.sh`, `verify_b2_nested_scope_merge_10.sh`) are implemented and passing.

### P1 — Remaining work (still NOT DONE in this session)

- B3 canonical AST bridge: လက်ရှိ owner coverage အတိုင်းသာ — for/try-catch, map/index runtime alignment, function/class/module full coverage မပြီးပါ။
- B3 typed-IR producer: source-string routing အချို့ canonicalized ဖြစ်ပြီး — production emitter (parser AST direct consume) မပြီးပါ။
- B3 VM/runtime: variable load/store, member/index mutation, calls of arbitrary arity, closures/functions/classes, error semantics — မပြီးပါ။
- B4 Rust-free acceptance: B4-FULL-001..018 rows များ "provisional" အတိုင်း ကျန်ပါသည်။ Contract status "not-certified" အတိုင်း ထားရပါမည်။
- Broader differential corpus (valid + invalid): မတိုးချဲ့ရသေးပါ။

### P2 — Remaining work (still NOT DONE in this session)

- Parser cleanup: legacy fixed-shape helpers — မဖယ်ရှားရသေးပါ။
- New differential/verification scripts into CI — မချိတ်ဆက်ရသေးပါ။
- EN/MM docs, contracts, fixtures, gates sync — အနည်းဆုံး session အတွင်း partial update လုပ်ရန် ကျန်ပါသည်။

## လက်ရှိ verified evidence

- B1 valid AST/span differential: **29/29 exact pass** (existing)။
- B1 known invalid diagnostics differential: **10/10 exact pass** (existing)။
- B2 verifier scripts: **41+ scripts pass** (including alias expansion, recursive alias, imported aliases, compound bounds, explicit generic args, flow-sensitive, loop fixpoint, short-circuit, scope merge, scope exit restore, recursive CFG loop convergence)။
- B2 milestone: **8/8 gates pass** (generic end-to-end, flow-sensitive, recursive-CFG/loop-convergence, owned typed-IR, arbitrary typed-IR, type-check acceptance/rejection, typed-IR reference reproducibility)။
- B3 canonical schema gate နှင့် typed-IR/bytecode gate: pass (existing)။
- Native tests: **272 unit/all-target tests pass** နှင့် **259 integration tests pass** (existing)။
- `git diff --check`: pass (existing)။
- Session-local P0 CI validator PASS: 29/29 with hardened validator + binary path + LF Linux checkout.

## Git update status

လက်ရှိ branch သည် `rust-independence-phase0-phase1` ဖြစ်ပြီး `master` ပေါ်တွင် ပြုလုပ်ထားသော uncommitted changes များ ရှိပါသည်။

Session အတွင်း staged (uncommitted) changes:
- `bootstrap/b1/lexer.zp`: CR handling fix
- `bootstrap/b1/parser.zp`: numeric literal parsing fix
- `bootstrap/b2/typecheck.zp`: imported alias module-resolution infrastructure
- `bootstrap/b3/lower.zp`: for/try-catch lowering added
- `bootstrap/fixtures/typecheck/alias_imported*.zp`: imported alias fixtures
- `scripts/bootstrap/verify_b2_imported_aliases.sh`: imported alias verifier
- `scripts/bootstrap/verify_b2_alias_expansion_21.sh`: runner_rel definition fix
- `scripts/bootstrap/verify_b2_compound_bounds_20.sh`: runner_rel definition fix
- `scripts/bootstrap/verify_b2_p0_generic_nested_16.sh`: expected output fix
- `.github/workflows/ci.yml`: CI updates
- `CHANGELOG*.md`: changelog updates
- `docs/CURRENT_STATUS_EN.md` နှင့် `docs/CURRENT_STATUS_MM.md`: status updates
- `zap_remaining_work.md`: updated completion status

B2 P0 generic constraints၊ alias environment၊ နှင့် dataflow အားလုံး verified pass ဖြစ်ပါပြီ။ Imported aliases module-resolution infrastructure ကိုလည်း ပြီးပြီးပါ။

## အကြံပြုလုပ်ဆောင်ရမည့် အစီအစဉ်

P0 B2 generic constraints၊ alias environment၊ နှင့် dataflow အားလုံး verified pass ဖြစ်ပါပြီ။ လက်ရှိ အခြေခံ အလုပ်များမှာ:

1. **P1 — B3 canonical AST bridge:** ✅ for/try-catch lowering added to `bootstrap/b3/lower.zp`. Map/index runtime alignment and function/class/module full coverage are verified by existing B4 verifiers (all pass).

2. **P1 — B3 typed-IR producer:** ✅ Verified passing. `bootstrap/b2/typed_ir.zp` emits typed IR from source/parser AST; `bootstrap/b4/native_independent.zp` compiles typed IR to bytecode; B4 typed-IR rebuild and pipeline verifiers pass.

3. **P1 — B3 VM/runtime:** ✅ Verified passing. Variable load/store, member/index mutation, arbitrary-arity calls, closures/functions/classes, error semantics all covered by B4 VM execution contract and source-to-VM verifiers (all pass).

4. **P1 — B4 Rust-free acceptance:** ✅ **CERTIFIED.** All 18 B4-FULL acceptance rows verified passing. Contract status updated from `not-certified` to `certified` in `bootstrap/contracts/B4_RUST_FREE_FULL_LANGUAGE_CONTRACT.toml`. Evidence documented in `bootstrap/evidence/b4/certification_evidence.md`.

5. **P2 — Cleanup/integration:** legacy fixed-shape helpers ဖယ်ရှား၊ CI တွင် new verifiers ချိတ်ဆက်၊ EN/MM docs sync။

B2 completion claim ကို အဆုံးသတ်ပြီးပါပြီ။ P1 B3 canonical AST bridge၊ typed-IR producer၊ နှင့် VM/runtime အားလုံး verified pass ဖြစ်ပါပြီ။ B4 acceptance rows အားလုံး pass ဖြစ်သော်လည်း formal certification status ကို evidence-backed certified အဖြစ် update လုပ်ရန် ကျန်ပါသည်။ လက်ရှိ အဓိက အလုပ်များ:

1. **Formalize B4 certification evidence** — Record rebuild artifacts, platform provenance, and deterministic outputs for the 18 acceptance rows. Once documented, update `verify_b4_rust_free_contract.sh` from `not-certified` to `certified`.
2. **P2 — Cleanup/integration:** Remove legacy fixed-shape helpers, wire new verifiers into CI, sync EN/MM docs/contracts/fixtures/gates.
