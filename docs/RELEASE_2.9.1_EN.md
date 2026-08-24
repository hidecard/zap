# Zap v2.9.1 Release Notes

**Release line:** v2.9.1
**Focus:** scaffold correctness, installer reliability, bounded host behavior, and release hardening.

## Highlights

Zap v2.9.1 is a maintenance release that fixes the first-run developer experience and tightens production-facing defaults. Fresh Web projects now use the requested project name, generate linter-compatible source, and pass the documented validation workflow.

The Unix installer now persists a configured `ZAP_INSTALL_DIR` instead of always writing the default `~/.local/bin` path. The Makefile now points to the actual legacy test suite and exposes locked native, host, legacy, and aggregate test targets.

Standalone release archives now carry the Markdown documentation set needed by the README navigation. The release workflow also uses least-privilege job permissions and reviewed immutable action references.

## Safety and reliability

The host adapter adds an explicit production-mode guard so missing JWT configuration does not silently select the demo authenticator. User-list responses are bounded through a hard maximum and pagination contract. The demo repository remains local-development-only; a real persistent repository adapter is still required before a deployment can be called production-ready.

## Upgrade notes

Existing Zap source files and lockfiles remain compatible with the v2.7.0 language/runtime baseline. Re-run `zap check`, `zap build --locked`, and `zap test` after upgrading. If you use a custom Unix installation directory, re-run `install.sh` so the corrected PATH entry is written to your shell profile.

## Verification

The release is intended to pass version consistency, formatting, Clippy, native and host tests, documentation/link checks, scaffold smoke tests, release preflight, deterministic packaging, checksum/signature verification, and installer verification on the supported release targets.
