# Zap v2.7.0 Release Notes

**Release line:** v2.7.0
**စစ်ဆေးထားသော baseline:** v2.6.0 နောက်ပိုင်း merged `master`
**အခြေအနေ:** Incremental language-server foundation release

## အနှစ်ချုပ်

Zap v2.7.0 တွင် native Language Server Protocol implementation အတွက် bounded incremental document synchronization ကို ထည့်သွင်းထားပါသည်။ Server သည် `textDocumentSync.change = 2` ကို advertise လုပ်ပြီး version monotonicity၊ negotiated UTF-8/UTF-16/UTF-32 character boundary နှင့် 32 MiB workspace byte cap များကို ထိန်းသိမ်းကာ sequential full-document သို့မဟုတ် range edit များကို လုံခြုံစွာ apply လုပ်နိုင်ပါသည်။

Malformed၊ stale၊ oversized၊ out-of-range နှင့် မဖွင့်ထားသော document အတွက် range edit များကို stored text မပြောင်းဘဲ reject လုပ်ပါသည်။ Notification တစ်ခုလျှင် content change အများဆုံး 128 ခုသာ လက်ခံပါသည်။ Symbol update၊ diagnostics၊ sequential edit၊ UTF-16 surrogate-pair boundary နှင့် invalid position များအတွက် regression test များ ထည့်သွင်းထားပါသည်။

## အကောင်အထည်ဖော်ပြီးသော ပြောင်းလဲမှုများ

| နယ်ပယ် | ပြောင်းလဲမှု | အထောက်အထား |
|---|---|---|
| LSP synchronization | Bounded full/range edit application နှင့် deterministic rejection rule များ ထည့်သွင်းထားသည်။ | Native LSP unit tests |
| Position safety | UTF-8၊ UTF-16 နှင့် UTF-32 range-to-byte validation ကို character boundary အတိုင်း ထည့်သွင်းထားသည်။ | UTF-16 regression test |
| Resource safety | Notification တစ်ခုလျှင် edit 128 ခု limit နှင့် edit တစ်ခုချင်းစီပြီးတိုင်း 32 MiB workspace cap ကို enforce လုပ်ထားသည်။ | Workspace-boundary tests |
| Documentation | English/Burmese Language Guide နှင့် Web-native guide များတွင် synchronization contract အသစ်ကို ပြင်ဆင်ထားသည်။ | Bilingual documentation checks |
| Release integrity | v2.7.0 metadata၊ manifest၊ specification၊ policy နှင့် release note များကို ညှိထားသည်။ | Release/version/preflight gates |

## Compatibility နှင့် boundaries

ရှိပြီးသား diagnostics၊ hover၊ completion၊ signature help၊ definition၊ document/workspace symbols၊ formatting နှင့် bounded rename behavior များ ဆက်လက်ရရှိပါသည်။ Range synchronization support လုပ်သော client များအတွက် ဤပြောင်းလဲမှုသည် additive ဖြစ်ပြီး full-document change ဆက်လက်ပေးပို့သော client များလည်း အသုံးပြုနိုင်ပါသည်။

ဤ release သည် complete cross-file semantic refactoring၊ project-wide dependency invalidation၊ incremental compilation၊ debugger/profiler integration၊ provider-neutral production database platform၊ production async I/O reactor၊ complete ORM၊ SSR/template compilation၊ WebSocket/streaming/upload infrastructure၊ built-in admin UI သို့မဟုတ် real mobile/AI/IoT provider adapter များ ပြီးစီးပြီဟု မဆိုထားပါ။ ၎င်းတို့သည် implementation နှင့် platform evidence လိုအပ်သည့် သီးခြား milestone များအဖြစ် ဆက်ရှိပါသည်။

## Verification

Native formatter၊ LSP tests၊ full native test suite၊ host tests၊ release build၊ framework starter checks၊ documentation consistency၊ Markdown link validation၊ VS Code parity၊ deployment checks နှင့် clean-tree release preflight များ pass ဖြစ်ရမည်။ Tag workflow တွင် Linux x86_64၊ macOS ARM64 နှင့် Windows x86_64 packaging၊ checksum/signature၊ provenance၊ installer နှင့် published-asset verification များလည်း pass ဖြစ်ရမည်။

## Upgrade

မိမိ platform နှင့်ကိုက်ညီသော archive ကို v2.7.0 GitHub Release မှ download လုပ်ပြီး checksum နှင့် detached signature ကို verify လုပ်ပါ။ ထို့နောက် `zap --version` ဖြင့် installed binary ကို စစ်ဆေးပါ။ ရှိပြီးသား `.zp` project များသည် manifest နှင့် lockfile workflow ကို ဆက်လက်အသုံးပြုနိုင်ပါသည်။

## References

[1]: ../docs/ZAP_WEB_NATIVE_MM.md
[2]: ../docs/LEARN_ZAP_MM.md
