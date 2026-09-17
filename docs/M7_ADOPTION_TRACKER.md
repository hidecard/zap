# Zap Community & Adoption Tracker (M7)

**Target:** M7 — Real-world Adoption
**Status:** Early ecosystem
**Created:** 2026-09-17

---

## M7 Definition of Done

| Metric | Target | Current | Status |
|---|---|---|---|
| Real projects | 100+ | TBD | ⬜ |
| Packages | 1000+ | TBD | ⬜ |
| Contributors | 50+ | TBD | ⬜ |
| Companies using Zap | 10+ | TBD | ⬜ |
| Production systems | 5+ | TBD | ⬜ |

---

## 1. Community Platforms

### 1.1 GitHub (Primary)
- **Repository:** [hidecard/zap](https://github.com/hidecard/zap)
- **Stars target:** 1000+
- **Contributors target:** 50+
- **Issues:** Use labels per contributor system below

### 1.2 Discord (Community Chat)
- **Purpose:** Real-time discussion, support, announcements
- **Channels needed:**
  - `#announcements`
  - `#general`
  - `#help`
  - `#showcase`
  - `#compiler-dev`
  - `#runtime-dev`
  - `#stdlib-dev`
  - `#web-dev`
  - `#docs`
  - `#tooling`

### 1.3 Forum (Long-form Discussion)
- **Purpose:** RFC discussions, migration guides, Q&A
- **Alternative:** GitHub Discussions if dedicated forum not viable

### 1.4 Documentation Hub
- Current: `docs/DOCUMENTATION_NAVIGATION_EN.md` (bilingual EN/MM)
- Need: Public-facing learning portal

### 1.5 YouTube (Tutorials)
- Target: 10+ tutorial videos covering Hello World through Web deployment

### 1.6 Blog (Changelog & Announcements)
- Target: Monthly posts highlighting releases, community projects, roadmap updates

---

## 2. Contributor System

### 2.1 Issue Labels

| Label | Purpose |
|---|---|
| `good first issue` | New contributors can start here |
| `help wanted` | Maintainers need help on these |
| `compiler` | Compiler-related work |
| `runtime` | Runtime/VM work |
| `stdlib` | Standard library work |
| `web` | Web framework work |
| `docs` | Documentation work |
| `tooling` | CLI, LSP, formatter, linter |
| `bug` | Bug reports |
| `feature` | Feature requests |
| `RFC` | RFC discussions |

### 2.2 Onboarding Path

```text
New Contributor
      │
      ▼
1. Pick a `good first issue`
      │
      ▼
2. Read CONTRIBUTING.md
      │
      ▼
3. Read relevant RFC (if language change)
      │
      ▼
4. Submit PR with tests
      │
      ▼
5. Code review by maintainer
      │
      ▼
6. Merge → Welcome to community!
```

### 2.3 Maintaining Standards
- All PRs must pass `cargo fmt --check`
- All PRs must pass `cargo clippy`
- All PRs must include tests for new behavior
- Bilingual documentation updates required for public API changes

---

## 3. Real Project Showcase

Target: 7+ project types as roadmap defines:

| # | Project Type | Status | Notes |
|---|---|---|---|
| 1 | Blog | ⬜ | Full-stack (web + DB + auth) |
| 2 | E-commerce | ⬜ | Complex domain, payments |
| 3 | POS | ⬜ | Offline-first, lightweight |
| 4 | School Management | ⬜ | CRUD-heavy, role-based access |
| 5 | REST API | ⬜ | API-only backend |
| 6 | SaaS | ⬜ | Multi-tenant, subscription |
| 7 | Real-time Chat | ⬜ | WebSocket-based |

### 3.1 Each Project Should Include
- [ ] Full source code in `examples/` or dedicated repo
- [ ] `zap.toml` and `zap.lock`
- [ ] README with setup instructions
- [ ] Tests demonstrating real usage
- [ ] Deployment configuration (Docker or platform-specific)
- [ ] Screenshots/demo for non-CLI projects

---

## 4. Package Ecosystem Growth Plan

### 4.1 First 20 Packages (Priority Order)

| # | Package | Category | Complexity |
|---|---|---|---|
| 1 | `http` | Web | Medium |
| 2 | `postgres` | Database | High |
| 3 | `jwt` | Auth | Low |
| 4 | `redis` | Cache/DB | Medium |
| 5 | `uuid` | Utility | Low |
| 6 | `dotenv` | Config | Low |
| 7 | `cors` | Web | Low |
| 8 | `logger` | Stdlib | Medium |
| 9 | `crypto-extra` | Crypto | Medium |
| 10 | `email` | Utility | Medium |
| 11 | `html-template` | Web | Medium |
| 12 | `session` | Auth | Medium |
| 13 | `orm-base` | Database | High |
| 14 | `validate` | Utility | Low |
| 15 | `paginate` | Utility | Low |
| 16 | `cache` | Utility | Medium |
| 17 | `scheduler` | Async | Medium |
| 18 | `filesystem-extra` | Stdlib | Low |
| 19 | `string-utils` | Stdlib | Low |
| 20 | `cli-helpers` | Tooling | Low |

### 4.2 Package Quality Standards

Each package must have:
- [ ] `zap.toml` with name, version, description, dependencies
- [ ] README with installation and usage examples
- [ ] Tests covering all public APIs
- [ ] Documentation in `docs/STDLIB_INDEX_EN.md` (or package-specific docs)
- [ ] Versioned release following SemVer
- [ ] Checksum/signature on publish

---

## 5. Educational Content Plan

### 5.1 Video Tutorials (YouTube)

| # | Title | Duration | Level |
|---|---|---|---|
| 1 | "Zap in 5 Minutes" | 5 min | Beginner |
| 2 | "Your First Zap Web App" | 15 min | Beginner |
| 3 | "Zap Types Explained" | 10 min | Beginner |
| 4 | "Zap Error Handling (Result/Option)" | 10 min | Beginner |
| 5 | "Zap Async Tasks" | 12 min | Intermediate |
| 6 | "Zap Web Framework Deep Dive" | 20 min | Intermediate |
| 7 | "Deploying Zap to the Cloud" | 15 min | Intermediate |
| 8 | "Building a REST API with Zap" | 20 min | Intermediate |
| 9 | "Zap Database with SQLite" | 15 min | Beginner |
| 10 | "Self-Hosting Zap" | 25 min | Advanced |

### 5.2 Blog Content (Monthly)

| Month | Topic | Category |
|---|---|---|
| Month 1 | "Why We Built Zap" | Announcement |
| Month 2 | "Migration from Python to Zap" | Migration Guide |
| Month 3 | "Zap vs Go for Backend Development" | Comparison |
| Month 4 | "First Community Project Showcase" | Community |
| Month 5 | "Understanding Zap's Compiler Pipeline" | Technical |
| Month 6 | "Zap Package Ecosystem Update" | Ecosystem |
| ... | Monthly continuation | Various |

---

## 6. Adoption Metrics Collection

### 6.1 Anonymous Tracking
- Package downloads (similar to npm pypl stats)
- GitHub stars growth
- Issue/PR velocity
- Documentation page views

### 6.2 Community Feedback
- Quarterly survey (features, pain points, suggestions)
- Annual state-of-Zap report
- Contributor spotlights

### 6.3 Success Indicators
- Active Discord with daily messages
- Third-party blog posts/tutorials (not from core team)
- Conference talks (PyCon, GopherCon equivalents)
- Company blog posts using Zap in production
- Package registry with 100+ published packages

---

## 7. Quarterly Review Checklist

### Q1 (Post-M7)
- [ ] GitHub stars: 500+ achieved?
- [ ] Discord active: 100+ members?
- [ ] Packages published: 100+?
- [ ] First company adoption reported?
- [ ] First external contributor merged?

### Q2
- [ ] GitHub stars: 1000+ achieved?
- [ ] Discord active: 300+ members?
- [ ] Packages published: 300+?
- [ ] 5+ real projects in production?
- [ ] First conference talk?

### Q3
- [ ] GitHub stars: 2000+ achieved?
- [ ] Discord active: 500+ members?
- [ ] Packages published: 500+?
- [ ] First book published?
- [ ] Company adoption: 10+?

---

## 8. Release Calendar Integration

| Event | Frequency | Audience |
|---|---|---|
| Release announcement | Monthly | All |
| Beta/RC preview | Bi-weekly (pre-release) | Early adopters |
| Security advisory | As needed | All |
| Roadmap update | Quarterly | All |
| Monthly progress report | Monthly | All |
| Community call | Monthly | Active contributors |

---

## 9. Budget & Resources (if applicable)

### 9.1 Infrastructure Costs (estimated)
| Item | Monthly Cost | Purpose |
|---|---|---|
| Discord server | Free | Community chat |
| GitHub (private repos) | Free (public) | Code hosting |
| CI/CD (GitHub Actions) | Free tier | Build/test |
| Domain (zap.dev) | ~$12/year | Package registry |
| Documentation hosting | Free (GitHub Pages) | Docs portal |
| YouTube (organic) | Free | Tutorials |
| Server for registry | $20-50/mo | Public package registry |

### 9.2 Volunteer Needs
| Role | Count | Commitment |
|---|---|---|
| Core maintainers | 3-5 | 10+ hrs/week |
| Documentation writers | 3-5 | 5+ hrs/week |
| Package maintainers | 20+ | 2+ hrs/week per package |
| Community managers | 1-2 | 5+ hrs/week |
| Video creators | 1-2 | 10+ hrs/week per video |

---

## 10. Milestones Summary

```text
2026 Q4:
  GitHub: 500+ stars
  Discord: 100+ members
  Packages: 50+ published
  First external PR merged

2027 Q1:
  GitHub: 1000+ stars
  Discord: 300+ members
  Packages: 150+ published
  First company adoption announced

2027 Q2:
  GitHub: 2000+ stars
  Packages: 300+ published
  First conference talk

2027 Q3:
  M7 milestone declared complete
  100+ real projects
  1000+ packages
  50+ contributors
  10+ companies
```

---

## References

- [CONTRIBUTING.md](../CONTRIBUTING.md)
- [SECURITY.md](../SECURITY.md)
- [DEPLOYMENT_CLOUD_EN.md](DEPLOYMENT_CLOUD_EN.md)
- [B4_CERTIFICATION_ACTION_PLAN.md](B4_CERTIFICATION_ACTION_PLAN.md)
- [README.md](../README.md)
