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

### P0 — Remaining work (still NOT DONE in this session)

- B2 generic constraints: multi-parameter inference, nested/compound bounds, explicit generic call syntax, diagnostic parity with Rust reference — fixtures + verifiers အသစ်များ မရေးသေးပါ။
- B2 alias environment: alias-of-alias, imported aliases, recursive alias detection, alias expansion diagnostics — မပြီးပါ။
- B2 dataflow: short-circuit path sensitivity, mutation/alias invalidation, loop fixpoint, break/continue/return live-path merge — မပြီးပါ။

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
- B2 verifier scripts: **33 scripts pass** (existing, per historical evidence)။
- B3 canonical schema gate နှင့် typed-IR/bytecode gate: pass (existing)။
- Native tests: **272 unit/all-target tests pass** နှင့် **259 integration tests pass** (existing)။
- `git diff --check`: pass (existing)။
- Session-local P0 CI validator PASS: 29/29 with hardened validator + binary path + LF Linux checkout.

## Git update status

လက်ရှိ branch သည် `master` ဖြစ်ပြီး origin ကို fetch/rebase ပြုလုပ်ပြီးနောက် local `master` သည် `origin/master` commit `b74e941` နှင့် up-to-date ဖြစ်နေပါသည်။

Session အတွင်း staged (uncommitted) changes:
- `scripts/validate_release_version.sh`: harden CR/trailing-whitespace + safer `read_cli_version`
- `scripts/test_validate_release_version.sh`: CRLF regression check

Working tree တွင် parser/typecheck/typed-IR/B3 source changes နှငင် new verifier/status files များ uncommitted အဖြစ် ရှိနေပါသည်။ ယခင်သတ်မှတ်ထားသော “အาผုပ်အားလုံးပြီးမှ commit/push” requirement နှင့် remaining B0/B1/B2/B3/B4 work ကြောင့် **ယခုအချိန်တွင် final commit/push မပြုလုပ်သင့်ပါ**။

## အကြံပြုလုပ်ဆောင်ရမည့် အစီအစဉ်

ပထမဦးစွာ P0 B2 alias environment နှင့် generic constraint inference ကို parser AST အပေါ်တွင် တိုက်ရိုက်ချိတ်ဆက်ရမည်။ ထို့နောက် B3 typed-IR producer နှင့် VM တွင် canonical AST schema အားလုံးကို support လုပ်ရမည်။ ထိုအပြီး Rust-reference differential corpus ကို valid/invalid နှစ်မျိုးစလုံး တိုးချဲ့ပြီး full regression နှင့် clean-clone verification ပြုလုပ်ရမည်။ B4 acceptance rows များကို Rust-free full compiler path အပြီးမှ pass သို့ ရွှေ့ပြီး contract status ကို အပြီအပိုင် evidence-backed certified သို့ သာ update လုပ်ရမည်။ အားလုံး pass ဖြစ်မှသာ commit နှင့် GitHub push ပြုလုပ်ရမည်။
