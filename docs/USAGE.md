# Zap အသုံးပြုနည်း လမ်းညွှန်

**စစ်ဆေးထားသော baseline:** Zap v2.11.4 development line

**ရည်ရွယ်ချက်:** ဤဖိုင်သည် command နှင့် operations အတွက် အတိုချုံး reference ဖြစ်သည်။ Installation မှ advanced အထိ အပြည့်အစုံလေ့လာရန် [မြန်မာ Language Guide](LEARN_ZAP_MM.md) ကို အသုံးပြုပါ။ Normative behavior သည် [language specification](LANGUAGE_SPEC_MM.md) တွင် သတ်မှတ်ထားပါသည်။

## Native runtime install လုပ်ခြင်း

Zap သည် standalone native executable အဖြစ် ဖြန့်ချိပါသည်။ မိမိ operating system နှင့် CPU architecture ကိုက်ညီသော archive ကို download လုပ်ပြီး checksum နှင့် signature စစ်ဆေးကာ extract လုပ်ပြီး executable ကို `PATH` ထဲ ထည့်ပါ။ v2.11.4 release တွင် release page ၌ ဖော်ပြထားသော target များကိုသာ support လုပ်မည်ဖြစ်သောကြောင့် မဖော်ပြထားသော target ကို မခန့်မှန်းပါနှင့်။

| Platform | Archive pattern | Install လုပ်နည်း |
|---|---|---|
| Linux x86_64 | `zap-<version>-linux-x86_64.tar.gz` | Extract ပြီး `bash install.sh` run လုပ်ရန် |
| macOS ARM64 | `zap-<version>-macos-arm64.tar.gz` | Extract၊ `chmod +x install.sh`၊ ထို့နောက် `./install.sh` |
| Windows x86_64 | `zap-<version>-windows-x86_64.zip` | Extract ပြီး Command Prompt မှ `install_windows.bat` run လုပ်ရန် |

`bin/zap` ပါသော release archive ကို application host တွင် Rust၊ Cargo၊ Python၊ Node.js၊ Java သို့မဟုတ် အခြား language runtime မလိုဘဲ run နိုင်ပါသည်။ Installer သည် user-writable directory ကို default သုံးပြီး အခြားနေရာလိုပါက `ZAP_INSTALL_DIR` သတ်မှတ်နိုင်သည်။

## Project ဆောက်ခြင်း

One-command user-managed Web scaffold ကို အသုံးပြုပါ။ Zap တွင် Django-style `startapp` command နှင့် hidden application registry မရှိပါ။

```bash
zap new my_app
cd my_app
zap check
zap build --locked
zap test tests
zap dev
```

Generator က `models/`, `functions/`, `ui/`, `routes/`, `middleware/`, `migrations/`, `admin/`, `public/` နှင့် `tests/` ကို ordinary user-owned files အဖြစ် ထုတ်ပေးပါသည်။ Module များကို မိမိလိုသလို ထည့်၊ ဖျက်၊ အမည်ပြောင်း၊ စီမံနိုင်ပါသည်။ Scaffold သည် development/reference Web slice ဖြစ်ပြီး production server၊ ORM၊ admin UI၊ authentication system သို့မဟုတ် deployment supervisor အဖြစ် အလိုအလျောက် မဖြစ်ပါ။

Web မဟုတ်သော project အသေးများအတွက် `zap.toml` နှင့် `main.zp` ပါသော directory ကို ကိုယ်တိုင်ဖန်တီးနိုင်ပါသည်။ Compatibility အတွက် `zap init <directory>` ရှိသော်လည်း project အသစ်နှင့် user-managed workflow အတွက် `zap new` ကို ဦးစားပေးပါ။

## CLI workflow

| Command | အသုံးပြုရန် |
|---|---|
| `zap <file.zp>` | Source file run လုပ်ရန် |
| `zap run <file.zp>` | Source file ကို explicit run လုပ်ရန် |
| `zap new <dir>` | User-managed Web scaffold အပြည့် ဖန်တီးရန် |
| `zap check [dir]` | Zap project directory၊ manifest၊ module နှင့် သိရှိနိုင်သော type များကို စစ်ရန် |
| `zap check --json [dir]` | Structured project diagnostic ထုတ်ရန် |
| `zap build [dir]` | Project ကို validate/prepare လုပ်ရန် |
| `zap build --locked [dir]` | ရှိပြီးသား lockfile ကိုသာ အသုံးပြုရန် |
| `zap test [dir]` | `*_test.zp` file များကို deterministic order ဖြင့် runရန် |
| `zap test --filter <value> [dir]` | ကိုက်ညီသော test များသာ runရန် |
| `zap test --fail-fast [dir]` | ပထမ failure တွင် ရပ်ရန် |
| `zap test --json [dir]` | Support ပြုထားသောနေရာတွင် machine-readable result ထုတ်ရန် |
| `zap fmt <file.zp>` | Zap source format ပြင်ရန် |
| `zap lint <file.zp>` | Formatting နှင့် style issue စစ်ရန် |
| `zap lock [dir]` | Canonical `zap.lock` generate လုပ်ရန် |
| `zap add <name> <version> [dir]` | Dependency ထည့်ပြီး lockfile အဟောင်းကို invalidate လုပ်ရန် |
| `zap install [dir]` | Lockfile/cache မှ validate/install လုပ်ရန် |
| `zap install --locked [dir]` | Valid lockfile ရှိမှသာ လက်ခံရန် |
| `zap update [dir]` | Manifest ပြောင်းပြီးနောက် lock data ပြန်ထုတ်ရန် |
| `zap web check [dir]` | Web configuration နှင့် project structure စစ်ရန် |
| `zap db check [dir]` | Migration layout နှင့် database plan စစ်ရန် |
| `zap db plan [dir] --json` | Read-only SQLite migration plan ပြရန် |
| `zap db migrate [dir] --dry-run` | Migration မသုံးဘဲ ကြိုတင်ကြည့်ရန် |
| `zap db migrate [dir] --check` | Pending migration ရှိပါက fail လုပ်ရန် |
| `zap dev [dir]` | Bounded native development server runရန် |
| `zap lsp` | Stdio language server runရန် |

ပုံမှန် development loop သည်—

```bash
zap fmt main.zp
zap lint main.zp
zap check .
zap test tests
zap build --locked .
zap db check .
```

## Package နှင့် lockfile

`zap.toml` တွင် package identity နှင့် dependency များကို ကြေညာပြီး canonical `zap.lock` ကို commit တင်ပါ။ Reproducible environment တွင် `zap install --locked` သုံးပါ။ Local path dependency များကို recursive validate လုပ်ပြီး cycle များကို reject လုပ်ပါသည်။ Registry operation များသည် transport၊ checksum၊ signature၊ cache limit နှင့် credential policy ကို enforce လုပ်သော်လည်း registry foundation သည် npm၊ PyPI၊ crates.io သို့မဟုတ် Go modules တို့၏ package volume/governance အဆင့် မရောက်သေးပါ။

## Web နှင့် frontend boundary

HTML၊ CSS နှင့် JavaScript များသည် `public/` အောက်ရှိ ordinary files ဖြစ်ပါသည်။ React၊ Vue၊ Svelte၊ Alpine သို့မဟုတ် အခြား browser framework ကို build-time toolchain အဖြစ် သီးခြားသုံးနိုင်ပါသည်။ Build output ကို သတ်မှတ်ထားသော asset directory ထဲ copy လုပ်ပြီး deployed Zap executable ဖြင့် serve လုပ်ပါ။ Deployment အချိန်တွင် npm သို့မဟုတ် Node.js မလိုပါ။ [Frontend integration guide](FRONTEND_INTEGRATION_MM.md) နှင့် [Zap Web guide](ZAP_WEB_NATIVE_MM.md) ကို ဖတ်ပါ။

လက်ရှိ Web runtime သည် bounded request/response နှင့် static/SPA development behavior ကို ပေးပါသည်။ TLS termination၊ production concurrency၊ WebSocket၊ streaming upload၊ session persistence၊ provider-neutral database driver၊ ORM၊ SSR/template compiler၊ cache invalidation၊ observability နှင့် process supervision များသည် host/deployment အလုပ်များအဖြစ် သီးခြားကျန်ရှိပါသည်။

## Safety boundary

Source size၊ execution depth၊ loop၊ value၊ collection၊ file၊ process output၊ HTTP request နှင့် task limit များသည် reliability control များ ဖြစ်ပါသည်။ ၎င်းတို့သည် OS sandbox မဟုတ်ပါ။ Untrusted Zap code ကို filesystem၊ process၊ network၊ identity နှင့် secret policy ပါဝင်သော operating-system isolation profile အတွင်းတွင်သာ run ပါ။

## Source development

Repository သည် `rust-toolchain.toml` တွင် Rust toolchain ကို pin လုပ်ထားပါသည်။ Source contributor များသည် locked dependency graph ကို အသုံးပြုပြီး native test၊ formatting၊ strict Clippy၊ documentation consistency၊ release-version၊ VS Code asset၊ LSP parity၊ framework နှင့် release-preflight gate များကို [မြန်မာ documentation navigation](DOCUMENTATION_NAVIGATION_MM.md) တွင် ဖော်ပြထားသည့်အတိုင်း run ရမည်။
