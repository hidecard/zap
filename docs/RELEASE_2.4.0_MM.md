# Zap v2.4.0 Release Notes

**Release date:** 2026-08-23  
**Release line:** v2.4.0  
**Theme:** သင်ယူလမ်းကြောင်း ပြည့်စုံခြင်းနှင့် project entry point ရှင်းလင်းခြင်း။

## အဓိကပြောင်းလဲမှုများ

Zap v2.4.0 တွင် language ကို လေ့လာရလွယ်ကူစေရန်နှင့် repository ကို ရှာဖွေရလွယ်ကူစေရန် documentation ကို ပြန်လည်တည်ဆောက်ထားပါသည်။ Bilingual learner material ကို **Zap Language Guide** အဖြစ် ပြန်ရေးထားပြီး install လုပ်ခြင်း၊ ပထမဆုံး `.zp` file၊ value၊ type၊ control flow၊ function၊ closure၊ class၊ module၊ Result/Option၊ diagnostic၊ standard library၊ testing၊ package၊ Web၊ database၊ async၊ LSP၊ runtime safety နှင့် advanced practice များကို အစမှအဆုံး ဖော်ပြထားပါသည်။

Root README ကို implementation detail များ ထပ်နေသော စာတမ်းအဖြစ် မထားတော့ဘဲ focused landing page အဖြစ် ပြန်စီထားပါသည်။ Install၊ command တစ်ကြောင်း project generator၊ CLI workflow၊ documentation link၊ frontend integration၊ development command နှင့် stable/deferred boundary များကိုသာ ဦးစားပေးဖော်ပြထားပါသည်။

## Command တစ်ကြောင်း project workflow

```bash
zap new my_app
cd my_app
zap check
zap build --locked
zap test tests
zap dev
```

Generate လုပ်သော project ထဲတွင် `zap.toml`၊ `zap.lock`၊ `main.zp`၊ `web.zp`၊ `server.zp`၊ `models/`၊ `functions/`၊ `ui/`၊ `routes/`၊ `middleware/`၊ `migrations/`၊ `admin/`၊ `public/` နှင့် `tests/` ပါဝင်ပါသည်။ Directory များသည် ordinary user-managed file များဖြစ်ပြီး Django-style `startapp` command သို့မဟုတ် hidden app registry မထည့်ထားပါ။

## Documentation cleanup

Stale ဖြစ်နေသော duplicate `docs/PACKAGES.md` ကို ဖယ်ရှားပြီး ထိန်းသိမ်းထားသော bilingual package guide များဖြစ်သည့် `docs/PACKAGE_EN.md` နှင့် `docs/PACKAGE.md` ကို canonical အဖြစ် အသုံးပြုထားပါသည်။ Historical release note၊ mandatory contract document၊ framework README နှင့် security/release evidence များကို traceability နှင့် release surface အတွက် လိုအပ်သောကြောင့် ဆက်လက်ထားရှိပါသည်။

English နှင့် Burmese documentation hub များတွင် Language Guide ကို ပထမဆုံး learning entry point အဖြစ် ပြောင်းထားပြီး လက်ရှိ release အတွက် v2.4.0 release note၊ language specification နှင့် release-version policy များကို ညွှန်ပြထားပါသည်။

## Validation

Release candidate သည် pinned Rust formatting/test suite၊ Framework starter validation၊ documentation consistency၊ release-version consistency၊ VS Code asset parity၊ LSP semantic parity၊ native/host check၊ security check နှင့် Linux၊ Windows၊ macOS ARM64 cross-platform build workflow များကို pass ရမည်ဖြစ်ပါသည်။

## မပါဝင်သေးသော boundary များ

ဤ release တွင် complete ORM၊ provider-neutral production migration platform၊ user-defined trait syntax၊ production asynchronous I/O reactor၊ cross-file semantic rename၊ template compiler သို့မဟုတ် hidden application registry များကို ပြည့်စုံပြီဟု မဆိုထားပါ။ ၎င်းတို့သည် deferred ဖြစ်နေသေးပြီး language specification နှင့် ဆက်စပ် contract များတွင် ဖော်ပြထားပါသည်။

## Upgrade လမ်းညွှန်

မိမိစက်၏ platform နှင့် architecture ကိုက်ညီသော v2.4.0 standalone executable ကို install လုပ်ပါ။ ရှိပြီးသား `.zp` project များသည် လက်ရှိ manifest/lockfile workflow အတိုင်း ဆက်လက်အသုံးပြုနိုင်ပါသည်။ Executable သို့မဟုတ် project dependency ပြောင်းပြီးနောက် `zap check`၊ `zap test` နှင့် `zap build --locked` ကို run ပါ။
