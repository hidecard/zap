# Zap အသုံးပြုနည်း လမ်းညွှန်

**စစ်ဆေးထားသော baseline:** Zap v2.2.6 maintenance branch

**ရည်ရွယ်ချက်:** Installation၊ project development၊ dependency lock၊ testing၊ registry အသုံးပြုမှုနှင့် production boundary များကို ရှင်းပြထားသည်။ v2.2.6 branch သည် release candidate ဖြစ်သောကြောင့် candidate ကို တရားဝင် publish မလုပ်မချင်း [GitHub Releases](https://github.com/hidecard/zap/releases) တွင် နောက်ဆုံး publish လုပ်ထားသော release ကို အသုံးပြုပါ။

## ၁။ Native runtime install လုပ်ခြင်း

Zap သည် standalone native executable အဖြစ် ဖြန့်ချိပါသည်။ မိမိ operating system နှင့် architecture ကိုက်ညီသော archive ကို download လုပ်ပြီး checksum စစ်ဆေးကာ install လုပ်ရမည်။ မတူညီသော OS သို့မဟုတ် CPU architecture အတွက် ထုတ်ထားသော archive ကို မသုံးပါနှင့်။

| Platform | Release asset | Install လုပ်နည်း |
|---|---|---|
| Linux x86_64 | `zap-<version>-linux-x86_64.tar.gz` | Extract ပြီး `bash install.sh` run လုပ်ရန် |
| macOS ARM64 | `zap-<version>-macos-arm64.tar.gz` | Extract၊ `chmod +x install.sh`၊ ထို့နောက် `./install.sh` |
| Windows x86_64 | `zap-<version>-windows-x86_64.zip` | Extract ပြီး Command Prompt မှ `install_windows.bat` run လုပ်ရန် |

Linux/macOS တွင် install မလုပ်မီ checksum စစ်ပါ။

```bash
sha256sum -c zap-<version>-linux-x86_64.tar.gz.sha256
# macOS asset ဖြစ်ပါက archive name ကို အစားထိုးပါ။
```

Extract လုပ်ပြီး user-level binary install လုပ်ပါ။

```bash
cd zap
bash install.sh
zap --version
zap --help
```

Default အားဖြင့် installer သည် binary ကို `~/.local/bin` ထဲတွင် ထည့်ပါသည်။ အခြား user-writable directory သုံးလိုပါက `ZAP_INSTALL_DIR` သတ်မှတ်နိုင်သည်။ Installer အတွက် root privilege မလိုပါ။ Release archive ထဲတွင် `bin/zap` ပါလျှင် Rust/Cargo မလိုပါ။

### Source မှ build လုပ်ခြင်း

Source build သည် developer သို့မဟုတ် operator က ရည်ရွယ်ချက်ရှိရှိ လုပ်သောလုပ်ဆောင်ချက် ဖြစ်သည်။ Repository ၏ `rust-toolchain.toml` သည် Rust 1.75.0 ကို pin လုပ်ထားသောကြောင့် installation အတွင်း dependency graph မပြောင်းလဲစေရန် locked build သုံးပါ။

```bash
ZAP_BUILD_FROM_SOURCE=1 bash install.sh
```

Repository checkout မှ reproducible build လုပ်ရန်—

```bash
cargo build --release --locked --manifest-path native/Cargo.toml
./native/target/release/zap --version
```

## ၂။ Project ဖန်တီးပြီး run လုပ်ခြင်း

အနည်းဆုံး project တစ်ခုတွင် `main.zp` source file ပါရမည်။ Dependency သုံးပါက `zap.toml` manifest နှင့် commit တင်ထားသော `zap.lock` lockfile ကိုပါ ထည့်ထားသင့်သည်။

```bash
mkdir hello-app
cd hello-app
cat > main.zp <<'EOF'
say "Hello from Zap"
EOF
zap check .
zap run main.zp
```

Project manifest ဥပမာ—

```toml
[package]
name = "hello-app"
version = "0.1.0"
main = "main.zp"
```

Standard scaffold အတွက် `zap init <directory>` ကိုသုံးပါ။ Run မလုပ်မီ `zap check` လုပ်ပြီး editor/CI အတွက် structured diagnostic လိုပါက `zap check --json .` သုံးပါ။ Local module များကို main-file directory နှင့် supported project module directory များတွင် ရှာဖွေပြီး module cycle နှင့် unsafe path များကို reject လုပ်သည်။

## ၃။ CLI workflow

| Command | အသုံးပြုရန် |
|---|---|
| `zap <file.zp>` | Canonical native AST runtime ဖြင့် source file run လုပ်ရန် |
| `zap run <file.zp>` | Source file ကို explicit run လုပ်ရန် |
| `zap init <dir>` | Project scaffold ဖန်တီးရန် |
| `zap fmt <file.zp>` | Source format ပြင်ရန် |
| `zap lint <file.zp>` | Source formatting နှင့် style စစ်ရန် |
| `zap check [dir]` | Manifest၊ module၊ type နှင့် project structure စစ်ရန် |
| `zap check --json [dir]` | CI/editor အတွက် structured diagnostic ထုတ်ရန် |
| `zap test [dir]` | `*_test.zp` files များကို deterministic path order ဖြင့် runရန် |
| `zap test --fail-fast [dir]` | ပထမ test failure တွင် ရပ်ရန် |
| `zap lock [dir]` | Canonical `zap.lock` generate လုပ်ရန် |
| `zap add <name> <version> [dir]` | Dependency ထည့်ပြီး lockfile အဟောင်းကို invalidate လုပ်ရန် |
| `zap install [dir]` | Project နှင့် lockfile ကို validate/install လုပ်ရန် |
| `zap install --locked [dir]` | ရှိပြီးသား lockfile ကိုသာ အသုံးပြုပြီး graph မပြောင်းရန် |
| `zap update [dir]` | Manifest မှ lockfile ပြန် generate လုပ်ရန် |
| `zap registry gc [--dry-run] [dir]` | မသုံးတော့သော cache artifact ဖယ်ရန် သို့မဟုတ် preview လုပ်ရန် |
| `zap lsp` | Editor integration အတွက် stdio language server runရန် |
| `zap async-check` | Deterministic async runtime foundation စစ်ရန် |

ပုံမှန် development loop သည်—

```bash
zap fmt main.zp
zap lint main.zp
zap check .
zap test .
zap build --locked .
zap install --locked .
```

Native runtime သည် source၊ execution depth၊ loop၊ output၊ memory၊ task နှင့် collection limit များကို ကန့်သတ်ထားသည်။ ထို runtime limit များသည် OS-level isolation ၏ အစားထိုးမဟုတ်ပါ။

## ၄။ Language ဥပမာ

Zap သည် indentation ဖြင့် block သတ်မှတ်ပြီး ဖတ်ရလွယ်သော expression များကို အသုံးပြုသည်။

```zap
fn greet(name):
    return "Hello, " + name

for item in ["language", "runtime", "tooling"]:
    say greet(item)
```

Typed result ကို စစ်ဆေးပြီး `?` ဖြင့် propagate လုပ်နိုင်သည်။

```zap
fn load_name(value: text) -> result<text>:
    if value == "":
        return err("name is empty")
    return ok(value)
```

Functions၊ closures၊ classes၊ modules၊ collections၊ JSON၊ `Result`/`Option`၊ async task handle နှင့် deterministic tests များ ပါဝင်သည်။ Normative behavior အတွက် အဟောင်း example များထက် [language specification](LANGUAGE_SPEC_MM.md) ကို အဓိကကိုးကားပါ။

## ၅။ Files၊ JSON နှင့် environment

Standard library တွင် bounded text/line file helper၊ JSON encode/decode၊ path helper၊ time helper၊ logging နှင့် environment access ပါဝင်သည်။

```zap
let lines = ["one", "two"]
write_lines("notes.txt", lines)
let loaded = read_lines("notes.txt")
say json({"lines": loaded})
```

`ExecutionContext` ပိုင် run အတွင်း relative file operation များကို ထို run ၏ workspace အတွင်း ကန့်သတ်ထားသည်။ Symlink/canonicalization checks များသည် defensive control များသာဖြစ်ပြီး process ကို kernel sandbox မဖြစ်စေပါ။ မယုံကြည်ရသော program များအတွက် isolated worker၊ read-only source tree၊ သီးခြား writable directory၊ အနည်းဆုံး environment variable၊ quota နှင့် network egress restriction များကို ထပ်မံသုံးပါ။

## ၆။ Dependency နှင့် registry workflow

Dependency ပါသော project တွင် lockfile generate ပြီး commit တင်ပါ။

```bash
zap add utility 1.2.0 .
zap lock .
zap check .
zap install --locked .
```

`zap install --locked` သည် manifest၊ lockfile၊ registry metadata၊ selected version၊ yanked policy နှင့် SHA-256 cache artifact များ ကိုက်ညီမှုကို စစ်ဆေးသည်။ `ZAP_OFFLINE=1 zap install --locked .` သည် cache ထဲတွင် ရှိပြီး checksum မှန်သော artifact များကိုသာ သုံးပြီး network retrieval မလုပ်ပါ။

Remote registry ကို explicit trust လုပ်ပြီးမှ အသုံးပြုပါ။ HTTP ကို controlled local fixture အတွက်သာ ရည်ရွယ်ထားပြီး ပုံမှန်အားဖြင့် ပိတ်ထားသည်။

```bash
zap registry trust add https://registry.example/team
export ZAP_REGISTRY_TOKEN_CI='secret-manager မှ ဖတ်ထားသော read token'
zap registry credential set https://registry.example/team --token-env ZAP_REGISTRY_TOKEN_CI
zap install --locked .
```

Credential list command သည် origin များကိုသာ ပြပြီး token value မပြပါ။ Credential များကို secret manager သို့မဟုတ် protected environment variable ထဲတွင်သာ ထားပါ။ `zap.toml`၊ `zap.lock`၊ source code၊ logs သို့မဟုတ် CI output ထဲသို့ မထည့်ပါနှင့်။

Package publish လုပ်ရန် local checksum တွက်ပြီး HTTPS endpoint မှ ပို့ပါ။

```bash
checksum="$(sha256sum ./demo.pkg | awk '{print $1}')"
export ZAP_REGISTRY_TOKEN='secret-manager မှ ဖတ်ထားသော publish token'
zap registry publish https://registry.example/team/publish ./demo.pkg demo 1.0.0 "$checksum"
```

Client သည် body မပို့မီ package checksum ကို စစ်သည်။ Registry fetch/publish path များသည် automatic redirect ပိတ်ထားပြီး untrusted mode တွင် registry host ကို တစ်ကြိမ် resolve လုပ်ကာ special/private destination များကို reject ပြုလုပ်ပြီး validated address set သို့ connection ကို pin လုပ်သည်။ TLS certificate validation သည် ပုံမှန် platform trust configuration ကို ဆက်သုံးသည်။

## ၇။ Testing နှင့် CI

Application test များကို `tests/` သို့မဟုတ် ရွေးချယ်ထားသော directory အောက်တွင် `*_test.zp` နာမည်ဖြင့် ထားပါ။

```bash
zap test --fail-fast .
```

Runtime contributor များအတွက် complete locked native gate များမှာ—

```bash
cargo fmt --manifest-path native/Cargo.toml --all -- --check
cargo check --manifest-path native/Cargo.toml --all-targets --all-features --locked
cargo clippy --manifest-path native/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path native/Cargo.toml --all-targets --all-features --locked
scripts/validate_registry_deployment.sh
```

CI နှင့် release workflow များတွင် RustSec `cargo-audit`၊ deployment-policy validation၊ deterministic replay၊ native/legacy parity၊ archive checks နှင့် release provenance checks များလည်း run ပါသည်။ Dependency evidence အတွက် [`RUSTSEC_AUDIT_MM.md`](RUSTSEC_AUDIT_MM.md) ကို ဖတ်ပါ။

## ၈။ Production security boundary

`ZAP_UNTRUSTED=1` သည် filesystem၊ environment၊ process၊ outbound network နှင့် local-registry capability များကို runtime boundary မှ deny လုပ်သည်။

```bash
ZAP_UNTRUSTED=1 zap check --json .
ZAP_UNTRUSTED=1 zap run main.zp
```

Native process ကို Internet သို့ တိုက်ရိုက်မဖွင့်ပါနှင့်။ Production registry reference deployment သည် service ကို loopback တွင် bind လုပ်ပြီး ingress proxy တွင် TLS terminate လုပ်သည်၊ dedicated service identity သုံးသည်၊ memory/CPU/tasks/open-files limit သတ်မှတ်သည်၊ backend egress ပိတ်ထားသည်၊ credential များကို repository အပြင်တွင် ထားသည်။ Complete systemd/nginx runbook အတွက် [production operations guide](PRODUCTION_OPERATIONS_MM.md) ကို လိုက်နာပါ။

Runtime သည် universal OS sandbox မဟုတ်ပါ၊ kernel-enforced multi-tenant isolation မပေးပါ၊ built-in metrics သို့မဟုတ် durable backup system မပါပါ။ Operator ဘက်မှ isolation၊ monitoring၊ alerting၊ backup၊ restore drill၊ key rotation၊ firewall၊ certificate renewal နှင့် incident response များကို စီမံရမည်။

## ၉။ VS Code နှင့် LSP

Published extension ရှိပါက—

```bash
code --install-extension ArkarYan.zap-language-support
```

Extension သည် `zap lsp` ကို အသုံးပြုသည်။ `zap` ကို `PATH` ထဲတွင် ထည့်ထားပါ သို့မဟုတ် VS Code setting တွင် `zap.executable` သတ်မှတ်ပါ။ LSP တွင် full document synchronization၊ diagnostics၊ hover၊ completion၊ formatting၊ definition၊ workspace symbols နှင့် file-local rename ပါဝင်သည်။ Cross-file rename မပါဝင်သေးသောကြောင့် automated refactor မလုပ်မီ result ကို ပြန်စစ်ပါ။

## ၁၀။ Uninstall

Unix installer သည် user-level directory ကို အသုံးပြုသည်။ `uninstall.sh` run လုပ်ပါ သို့မဟုတ် installed binary နှင့် shell profile ထဲရှိ Zap PATH line ကို ဖယ်ရှားပါ။ Windows တွင် `uninstall_windows.bat` run လုပ်ပါ သို့မဟုတ် `%USERPROFILE%\.zap\bin\zap.exe` နှင့် PATH entry ကို ဖယ်ရှားပါ။ Uninstall လုပ်ခြင်းသည် project files၊ registry data သို့မဟုတ် credential များကို မဖျက်ပါ။

## ကိုးကားရန်

Normative reference များမှာ [language specification](LANGUAGE_SPEC_MM.md)၊ [package guide](PACKAGE.md)၊ [registry authentication contract](REGISTRY_AUTH_MM.md)၊ [deployment boundaries](DEPLOYMENT_MM.md)၊ [production operations guide](PRODUCTION_OPERATIONS_MM.md)၊ [security policy](../SECURITY.md) နှင့် [RustSec audit evidence](RUSTSEC_AUDIT_MM.md) တို့ ဖြစ်သည်။
