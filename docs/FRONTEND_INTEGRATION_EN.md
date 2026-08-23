# Zap Frontend Integration Guide

**Status:** Framework branch frontend runtime contract

Zap treats the browser as a file and HTTP boundary. A project may use plain HTML/CSS/JavaScript or any JavaScript framework during development and build time. The deployed application does not execute Node.js, npm, pnpm, Bun, Python, Java, or Rust. It needs the installed Zap executable and the already-built files under the declared Web asset directory.

## Runtime contract

| Concern | Zap owns | Optional frontend toolchain owns |
|---|---|---|
| HTTP process | Project validation, request parsing, route dispatch, limits, response framing | None |
| Static files | Safe root confinement, traversal rejection, MIME type, bounded reads, text and binary response encoding | Asset import resolution during build |
| Browser application | HTML/CSS/JS delivery and JSON API boundary | React, Vue, Svelte, Alpine, Solid, or another browser framework |
| SPA navigation | `web_static_spa` fallback to the configured entry document | Client-side router |
| Production artifact | `zap.toml`, Zap source, migration files, and built `public/` tree | Generated `dist/` files copied into `public/` |

The boundary is deliberately explicit. Zap does not pretend to be a React, Vue, or Svelte compiler. Those tools can remain entirely outside the runtime image.

## Create a project

Install the official Zap binary or build it explicitly from source:

```bash
# Release installation; no language toolchain is required after installation.
curl -fsSL https://raw.githubusercontent.com/hidecard/zap/Framework/install.sh | bash

# Create and validate a Web project.
zap new shop
cd shop
zap check
zap web check
zap build
zap test tests
```

The generated project contains a plain browser entrypoint, an API route, an assets route, and a final SPA fallback route. The generated `[frontend]` section is:

```toml
[frontend]
framework = "plain"
output = "public"
spa_fallback = "index.html"
```

The supported `framework` values are `plain`, `react`, `vue`, `svelte`, and `other`. This value is metadata for validation and operations; it never causes Zap to install or execute a frontend package manager.

## Plain HTML/CSS/JavaScript

Put the entry document at `public/index.html`, stylesheets and modules under `public/assets/`, and declare routes in `routes/routes.zp`:

```zap
export fn home(request):
    return web_static("index.html", "public")

export fn asset(request):
    return web_static("assets/" + request["params"]["path"], "public")
```

A normal development run is:

```bash
zap web check
ZAP_WEB_PORT=3000 zap dev
```

The runtime serves text assets as text and image/font/Wasm assets as binary HTTP bodies. The asset root is confined to the project workspace. Absolute paths, `..`, encoded traversal, unsupported extensions, and files above the configured asset limit fail closed.

## React, Vue, and Svelte

Build the frontend wherever the project team prefers. Node is a build-time dependency only:

```bash
# Example: frontend is maintained in a separate directory.
cd frontend
npm ci
npm run build

# Copy the generated browser artifact into the Zap project.
rm -rf ../shop/public/assets/*
cp -R dist/* ../shop/public/

cd ../shop
zap check
zap web check
zap build
zap test tests
zap dev
```

For Vite-based React, Vue, or Svelte applications, configure the build output so that the entry document is copied to `public/index.html` and generated chunks/assets are copied below `public/assets/`. If the build uses a `dist/assets` layout, copy both the `dist/index.html` file and the complete `dist/assets` directory; do not copy only the JavaScript entry chunk.

The production image can now contain only:

```text
/usr/local/bin/zap
/app/zap.toml
/app/*.zp
/app/migrations/*.zp
/app/public/index.html
/app/public/assets/*
```

No `node_modules/`, package manager cache, source maps, or frontend compiler is required at runtime. Keep source maps out of a public deployment unless they are intentionally protected.

## SPA fallback

Client-side routes such as `/dashboard`, `/settings/profile`, and `/projects/42` must return the entry document when no physical asset exists. The generated scaffold uses a final wildcard route and the `web_static_spa` builtin:

```zap
export fn frontend_spa(request):
    return web_static_spa(request["params"]["path"], "public", "index.html")
```

The fallback is intentionally after `/assets/*path` and `/api/*` routes. This prevents an unavailable JavaScript chunk or API endpoint from being silently replaced with HTML. Keep API routes and asset routes before the SPA route.

## Deployment checklist

Before promoting an artifact, run the following checks in a clean environment:

```bash
zap --version
zap check --json .
zap web check .
zap db check .
zap db migrate --check .
zap test tests --fail-fast
```

Run the server behind a TLS-terminating ingress or a production-capable host adapter, set an explicit bind address and port, and restrict the process filesystem and network permissions. Configure a real identity provider, repository/database adapter, migration policy, request timeout, log redaction, readiness probe, and graceful shutdown policy. The demo repository and demo authenticator in `host/zap-host` are contract fixtures, not production identity or persistence.

## Compatibility policy

A frontend build is compatible when it satisfies four conditions: the output is inside the declared `frontend.output` directory; the fallback file exists; every requested file uses a supported and declared MIME type; and the browser can call Zap JSON routes without assuming a Node server. `zap web check` enforces the first two conditions and the native runtime enforces path, size, and response constraints at request time.

A future framework-specific adapter may add server-side rendering or streaming, but it must preserve the same explicit artifact boundary. Until such an adapter is versioned and tested, use client-side rendering with static output and Zap API routes.

## References

[1]: https://react.dev/learn React documentation — official learning materials.
[2]: https://vuejs.org/guide/quick-start.html Vue documentation — official quick-start guide.
[3]: https://svelte.dev/docs/svelte/getting-started Svelte documentation — official getting-started guide.
[4]: https://vite.dev/guide/static-deploy.html Vite documentation — static deployment guidance.
