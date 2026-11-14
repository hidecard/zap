# Zap Roadmap

> **Redirect:** The full 15-step roadmap, 7 milestones, and contribution tracker lives in [TODO.md](../TODO.md). This file is a convenience entry point.

## AI-native roadmap (2026-11-14)

Zap's implementation order follows seven phases. Phase 0 is mandatory: B4 must be certified before the project claims a fully self-hosted production compiler.

| Phase | Focus | Current boundary |
|---|---|---|
| 0 | Self-hosting and B4 certification | Not certified; bounded Zap-owned backend slices exist, but full-language ownership and seed provenance remain open |
| 1 | Language reliability and production quality | Runtime, async, release, security, parity, and verification foundations exist; current CI portability regression is open |
| 2 | Developer experience | CLI, formatter, linter, LSP, package, installation, and documentation foundations exist |
| 3 | AI-native developer interfaces | Structured diagnostics exist; context, inspection, test-result, `AI_GUIDE.md`, and `AGENTS.md` contracts are next |
| 4 | AI-agent integration | Permission-aware MCP/tool integration is planned; no production agent server is claimed |
| 5 | Real-world application ecosystem | Web, SQLite, package, and example foundations exist; integration validation remains ongoing |
| 6–7 | AI applications, community, and adoption | Provider-neutral AI contracts, examples, tutorials, and transparent adoption evidence remain planned |

**Current evidence:** master `b5b5fa1`; latest overall CI run [42849365359](https://github.com/hidecard/zap/actions/runs/42849365359) passed the three C-backend platform jobs and cross-platform comparison but failed the Rust-quality `run_zap()` portability smoke on a Bash parse error and a leaked `.zap-c3-gate.7fFkJW` artifact.

## Quick Reference

| Resource | Location |
|---|---|
| Full roadmap (15 steps) | [TODO.md](../TODO.md) |
| Repository ownership structure | [docs/REPO_STRUCTURE.md](REPO_STRUCTURE.md) |
| Cloud deployment guide | [docs/DEPLOYMENT_CLOUD_EN.md](DEPLOYMENT_CLOUD_EN.md) |
| B4 certification action plan | [docs/B4_CERTIFICATION_ACTION_PLAN.md](B4_CERTIFICATION_ACTION_PLAN.md) |
| Database provider strategy | [docs/DATABASE_PROVIDER_STRATEGY_EN.md](DATABASE_PROVIDER_STRATEGY_EN.md) |
| Community/adoption tracker (M7) | [docs/M7_ADOPTION_TRACKER.md](M7_ADOPTION_TRACKER.md) |
| Migration guide | [docs/MIGRATION_GUIDE_EN.md](MIGRATION_GUIDE_EN.md) |
| Bootstrap contract | [bootstrap/contracts/BOOTSTRAP_CONTRACT_EN.md](../bootstrap/contracts/BOOTSTRAP_CONTRACT_EN.md) |
| B4 acceptance manifest | [bootstrap/contracts/B4_ACCEPTANCE.tsv](../bootstrap/contracts/B4_ACCEPTANCE.tsv) |

## 7 Milestones

| Milestone | Target | Status |
|---|---|---|
| **M1** — Stable Language | Parser, type checker, IR, diagnostics, tests stable | ✅ |
| **M2** — Native Compiler | Zap → Native production quality | ✅ |
| **M3** — Self-hosted Zap | Zap → Zap compiler → Zap | 🔄 B4 candidate |
| **M4** — Package Ecosystem | `zap install` working | ✅ |
| **M5** — Full-stack Web | Web + DB + API + Auth | ✅ |
| **M6** — Developer Experience | VS Code, LSP, Debugger, Formatter, Linter | ✅ |
| **M7** — Real-world Adoption | 100+ projects, 1000+ packages | 📊 Tracking |

## See Also

- [Documentation Navigation](DOCUMENTATION_NAVIGATION_EN.md)
- [Current Status](CURRENT_STATUS_EN.md)
- [TODO.md](../TODO.md) — full 15-step implementation order
