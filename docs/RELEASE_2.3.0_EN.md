# Zap v2.3.0 Release Notes

**Status:** Source integration baseline and release-contract record
**Date:** 2026-08-23

## Summary

Zap v2.3.0 integrates the Zap-native Framework branch with the current master runtime and the reviewed security-maintenance branches. The result keeps the native executable as the project execution boundary while making the Web scaffold clearer for projects that separate models, business functions, browser UI, routes, middleware, migrations, administration, and tests.

## Included changes

- Integrated the Framework runtime, frontend interoperability boundary, authentication and deployment contracts, production-operation documentation, and host-adapter updates.
- Added JSON-cycle protection, bounded collection-producing builtins, restricted HTTP connection pinning after DNS validation, registry-operation hardening, RustSec evidence, and the macOS native web-request test fix.
- Updated `zap new` to generate an explicit `ui/ui.zp` module. The generated module records the browser entrypoint, asset root, frontend mode, and the fact that Node.js is not required at runtime.
- Updated the English and Burmese Web guides to describe the Model/Function/UI separation and the optional build-time relationship with React, Vue, Svelte, Alpine, or plain HTML/CSS/JavaScript.

## Runtime and deployment boundary

A deployed Zap project requires the installed Zap executable and its declared project assets. Python, Node.js, Rust, Java, and another application runtime are not required on the deployment host. JavaScript framework toolchains remain optional build-time tools; their emitted static files are served from the declared public asset root.

The Web server and database/authentication integrations remain subject to the documented development/reference and adapter boundaries. Before distributing release assets, maintainers must complete the repository release preflight, checksum, signature, provenance, deployment, and security gates.

## Validation evidence

The integrated branch passed Rust formatting, the native test suite with 258 tests, release compilation, the Framework starter validator with 193 checks, and generated Web scaffold checks for `zap check`, `zap run`, and `zap test tests` in the development environment.
