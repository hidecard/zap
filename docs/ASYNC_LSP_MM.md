# Zap Async Runtime နှင့် LSP Foundation

## လက်ရှိအခြေအနေ

Zap တွင် P2 အတွက် **foundation** နှစ်မျိုးကို ထည့်သွင်းထားပါသည်။ Async runtime သည် deterministic single-thread executor ဖြစ်ပြီး နောက်ပိုင်း async language syntax အတွက် အခြေခံအဖြစ် အသင့်ရှိပါသည်။ Editor integration သည် stdio JSON-RPC server ဖြစ်ပြီး လက်ရှိတွင် initialize၊ shutdown၊ text synchronization နှင့် lint diagnostics များကို support လုပ်ပါသည်။

ဤ foundation များကြောင့် လက်ရှိ synchronous execution အပြုအမူ မပြောင်းလဲပါ။ Full async syntax၊ timers၊ cancellation၊ completion၊ hover၊ formatting နှင့် workspace indexing များမှာ နောက်ထပ် roadmap ဖြစ်ပါသည်။

## Async Runtime Foundation

Native runtime တွင် executor operation သုံးမျိုးရှိပါသည်။

| Operation | ရည်ရွယ်ချက် |
|---|---|
| `spawn(future)` | Async task တစ်ခုကို deterministic task queue ထဲ ထည့်သည်။ |
| `run_until_idle()` | Task များကို spawn order အတိုင်း poll လုပ်ပြီး အားလုံးပြီးသည်အထိ run သည်။ |
| `block_on(future)` | Future တစ်ခုကို ပြီးဆုံးသည်အထိ synchronous အနေဖြင့် drive လုပ်သည်။ |

လက်ရှိ implementation သည် worker thread များနှင့် external runtime dependency များကို မသုံးသေးပါ။ Parser နှင့် evaluator ထဲသို့ async syntax မထည့်မီ တည်ငြိမ်သော semantic base ရရှိရန် ဤပုံစံကို ရွေးထားပါသည်။

CLI smoke check ကို အောက်ပါအတိုင်း run နိုင်ပါသည်။

```bash
zap async-check
```

အောင်မြင်ပါက အောက်ပါ output ရပါမည်။

```text
async runtime foundation ready
```

## LSP Foundation

Editor server ကို အောက်ပါ command ဖြင့် စတင်နိုင်ပါသည်။

```bash
zap lsp
```

Server သည် standard input/output မှတစ်ဆင့် `Content-Length` header ဖြင့် frame လုပ်ထားသော JSON-RPC message များကို ဆက်သွယ်ပါသည်။ လက်ရှိ support လုပ်ထားသော message များမှာ—

| Message | အပြုအမူ |
|---|---|
| `initialize` | Zap server information နှင့် text synchronization capability ကို ပြန်ပေးသည်။ |
| `shutdown` | အောင်မြင်သော null result ကို ပြန်ပေးသည်။ |
| `textDocument/didOpen` | ဖွင့်ထားသော source text အတွက် diagnostics ထုတ်ပေးသည်။ |
| `textDocument/didChange` | ပြောင်းလဲထားသော source text အတွက် diagnostics ထုတ်ပေးသည်။ |

Diagnostics များကို Zap ၏ လက်ရှိ lint implementation မှ ထုတ်ယူပါသည်။ ထို့ကြောင့် CLI နှင့် editor diagnostics များသည် rule နှစ်မျိုးခွဲမသွားဘဲ တူညီနေပါသည်။

Editor client များသည် standard LSP transport framing ကို အသုံးပြုသင့်ပြီး completion သို့မဟုတ် hover capability များ လက်ရှိပါပြီးသားဟု မယူဆသင့်ပါ။

## နောက်ထပ် Roadmap

Async ပိုင်းတွင် language-level `async`/`await` semantics၊ suspension points၊ error propagation၊ timers၊ cancellation နှင့် resource limits များကို သတ်မှတ်ရမည်ဖြစ်ပါသည်။ LSP ပိုင်းတွင် parser source ranges၊ တိကျသော diagnostic severity၊ document synchronization အသေးစိတ်၊ formatting၊ hover၊ completion၊ go-to-definition နှင့် workspace-aware package/module indexing များကို ဆက်လက်ထည့်သွင်းရမည်ဖြစ်ပါသည်။
