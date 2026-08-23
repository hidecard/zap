# Zap Frontend ချိတ်ဆက်အသုံးပြုမှု လမ်းညွှန်

**အခြေအနေ:** Framework branch frontend runtime contract

Zap က browser ကို file နှင့် HTTP boundary အဖြစ် သတ်မှတ်ထားသည်။ Plain HTML/CSS/JavaScript သုံးနိုင်သလို development/build အချိန်တွင် React, Vue, Svelte, Alpine, Solid စသည့် JavaScript framework များကိုလည်း အသုံးပြုနိုင်သည်။ Deploy လုပ်ပြီးနောက် runtime အတွက် Node.js, npm, pnpm, Bun, Python, Java သို့မဟုတ် Rust မလိုပါ။ Install လုပ်ထားသော Zap executable နှင့် Web asset directory ထဲတွင် ရှိပြီးသား build output များသာ လိုအပ်သည်။

## Runtime boundary

| အပိုင်း | Zap က ပိုင်ဆိုင်သည် | Optional frontend toolchain က ပိုင်ဆိုင်သည် |
|---|---|---|
| HTTP process | Project validation, request parsing, route dispatch, limits, response framing | မရှိ |
| Static files | Root confinement, traversal rejection, MIME type, bounded read, text/binary response | Build အချိန် asset import resolution |
| Browser application | HTML/CSS/JS delivery နှင့် JSON API boundary | React, Vue, Svelte, Alpine, Solid သို့မဟုတ် အခြား browser framework |
| SPA navigation | Configured entry document သို့ `web_static_spa` fallback | Client-side router |
| Production artifact | `zap.toml`, Zap source, migration files နှင့် build ပြီး `public/` tree | Generated `dist/` files ကို `public/` ထဲသို့ copy လုပ်ခြင်း |

Zap သည် React, Vue သို့မဟုတ် Svelte compiler ဖြစ်သည်ဟု မဆိုလိုပါ။ ထို build tools များသည် runtime image ထဲမှ လုံးဝ ခွဲထားနိုင်သည်။

## Project တည်ဆောက်ခြင်း

Official Zap binary ကို install လုပ်ပါ။ Install ပြီးနောက် runtime run ရန် language toolchain မလိုပါ။

```bash
curl -fsSL https://raw.githubusercontent.com/hidecard/zap/Framework/install.sh | bash
zap new shop
cd shop
zap check
zap web check
zap build
zap test tests
```

Generated project တွင် plain browser entrypoint, API route, assets route နှင့် နောက်ဆုံး SPA fallback route ပါဝင်သည်။ Generated `[frontend]` section သည် အောက်ပါအတိုင်း ဖြစ်သည်။

```toml
[frontend]
framework = "plain"
output = "public"
spa_fallback = "index.html"
```

`framework` အတွက် `plain`, `react`, `vue`, `svelte`, `other` တန်ဖိုးများကို အသုံးပြုနိုင်သည်။ ဤတန်ဖိုးသည် validation/operations အတွက် metadata သာ ဖြစ်ပြီး Zap က frontend package manager ကို install သို့မဟုတ် execute မလုပ်ပါ။

## Plain HTML/CSS/JavaScript

Entry document ကို `public/index.html`၊ stylesheet နှင့် module များကို `public/assets/` အောက်တွင် ထားပြီး `routes.zp` တွင် route သတ်မှတ်ပါ။

```zap
export fn home(request):
    return web_static("index.html", "public")

export fn asset(request):
    return web_static("assets/" + request["params"]["path"], "public")
```

Development run သည် အောက်ပါအတိုင်း ဖြစ်သည်။

```bash
zap web check
ZAP_WEB_PORT=3000 zap dev
```

Runtime သည် text asset များကို text အဖြစ်၊ image/font/Wasm asset များကို binary HTTP body အဖြစ် serve လုပ်သည်။ Asset root ကို project workspace အတွင်းတွင်သာ ကန့်သတ်ထားသည်။ Absolute path, `..`, encoded traversal, မထောက်ပံ့သော extension နှင့် သတ်မှတ်ထားသော size limit ထက်ကြီးသော file များကို fail-closed reject လုပ်မည်။

## React, Vue နှင့် Svelte

Frontend ကို project team ကြိုက်နှစ်သက်ရာနေရာတွင် build လုပ်နိုင်သည်။ Node သည် build အချိန် dependency သာ ဖြစ်သည်။

```bash
cd frontend
npm ci
npm run build

rm -rf ../shop/public/assets/*
cp -R dist/* ../shop/public/

cd ../shop
zap check
zap web check
zap build
zap test tests
zap dev
```

Vite-based React, Vue သို့မဟုတ် Svelte project များအတွက် entry document ကို `public/index.html` သို့ ရောက်အောင်၊ generated chunk/asset များကို `public/assets/` အောက်သို့ ရောက်အောင် build output သတ်မှတ်ပါ။ `dist/assets` layout ဖြစ်ပါက `dist/index.html` နှင့် `dist/assets` directory နှစ်ခုလုံးကို copy လုပ်ပါ။ JavaScript entry chunk တစ်ခုတည်းကို မကူးပါနှင့်။

Production image ထဲတွင် အောက်ပါ files များသာ ပါနိုင်သည်။

```text
/usr/local/bin/zap
/app/zap.toml
/app/*.zp
/app/migrations/*.zp
/app/public/index.html
/app/public/assets/*
```

Runtime တွင် `node_modules/`, package-manager cache, frontend compiler မလိုပါ။ Public deployment တွင် source map များကို ရည်ရွယ်ချက်ရှိရှိ မထည့်ထားပါက မကူးပါနှင့်။

## SPA fallback

`/dashboard`, `/settings/profile`, `/projects/42` ကဲ့သို့ client-side route များတွင် physical file မရှိသော်လည်း entry document ကို ပြန်ပေးရမည်။ Generated scaffold တွင် final wildcard route နှင့် `web_static_spa` builtin ကို အသုံးပြုထားသည်။

```zap
export fn frontend_spa(request):
    return web_static_spa(request["params"]["path"], "public", "index.html")
```

Fallback route ကို `/assets/*path` နှင့် `/api/*` route များနောက်တွင် ထားသည်။ ထို့ကြောင့် မတွေ့သော JavaScript chunk သို့မဟုတ် API endpoint ကို HTML ဖြင့် အစားထိုးမပေးနိုင်ပါ။ API နှင့် asset route များကို SPA route မတိုင်မီ ထားပါ။

## Production deploy checklist

Clean environment တွင် artifact တင်မီ အောက်ပါ command များ run ပါ။

```bash
zap --version
zap check --json .
zap web check .
zap db check .
zap db migrate --check .
zap test tests --fail-fast
```

Server ကို TLS terminate လုပ်ပေးသော ingress သို့မဟုတ် production-capable host adapter နောက်တွင် run ပါ။ Bind address နှင့် port ကို explicit သတ်မှတ်ပြီး filesystem/network permission ကို ကန့်သတ်ပါ။ Real identity provider, database/repository adapter, migration policy, request timeout, log redaction, readiness probe နှင့် graceful shutdown policy ကို configure လုပ်ရမည်။ `host/zap-host` ထဲက demo repository နှင့် demo authenticator များသည် contract fixture များသာ ဖြစ်ပြီး production identity သို့မဟုတ် persistence မဟုတ်ပါ။

## Compatibility policy

Frontend build တစ်ခု compatible ဖြစ်ရန် အချက်လေးချက် ပြည့်မီရမည်။ Output သည် သတ်မှတ်ထားသော `frontend.output` အတွင်း ရှိရမည်။ Fallback file ရှိရမည်။ Request လုပ်မည့် file တိုင်းတွင် supported MIME type ရှိရမည်။ Browser သည် Node server ရှိသည်ဟု မယူဆဘဲ Zap JSON route များကို ခေါ်နိုင်ရမည်။ `zap web check` က ပထမအချက်နှစ်ချက်ကို စစ်ဆေးပြီး native runtime က path, size နှင့် response constraints များကို request အချိန်တွင် enforce လုပ်သည်။

နောင်တွင် framework-specific adapter များက server-side rendering သို့မဟုတ် streaming ထည့်နိုင်သော်လည်း explicit artifact boundary ကို မဖျက်ရပါ။ ထို adapter များကို version နှင့် test evidence ဖြင့် မထုတ်ပြန်သေးသရွေ့ client-side rendering + static output + Zap API routes ကို အသုံးပြုပါ။

## ကိုးကားချက်များ

[1]: https://react.dev/learn React official learning materials.
[2]: https://vuejs.org/guide/quick-start.html Vue official quick-start guide.
[3]: https://svelte.dev/docs/svelte/getting-started Svelte official getting-started guide.
[4]: https://vite.dev/guide/static-deploy.html Vite official static deployment guidance.
