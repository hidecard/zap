# Zap Version Migration Guide

This guide helps you migrate your Zap projects between major versions. Each section covers breaking changes, new features, and migration steps.

## Table of Contents

- [v2.11.x to v2.12.x](#v211x-to-v212x)
- [v2.10.x to v2.11.x](#v210x-to-v211x)
- [v2.2.x to v2.3.x](#v22x-to-v23x)
- [v2.1.x to v2.2.x](#v21x-to-v22x)
- [v2.0.x to v2.1.x](#v20x-to-v21x)
- [v0.x to v2.0.x](#v0x-to-v20x)

---

## v2.11.x to v2.12.x

**Status:** v2.12 is not yet released. This section will be updated when v2.12 is released.

**Expected changes:**
- B4 self-hosting certification updates
- Additional Zap-owned C backend migration slices
- Enhanced type-checker generic support

**Migration steps:**
- No migration steps available yet

---

## v2.10.x to v2.11.x

**Release:** [v2.11.18](RELEASE_2.11.18_EN.md)

### Breaking Changes

None. v2.11.x is a non-breaking continuation of the v2.10.x series.

### New Features

- B2 type checker: complete generic constraints, compound bounds, alias checking
- B1 lexer/parser: expanded arbitrary block coverage
- B3 package/build: Zap-side package resolver
- B4 self-hosting: deterministic rebuild evidence

### Migration Steps

No migration required. v2.11.x is backward compatible with v2.10.x.

```bash
# Update to v2.11.18
zap update
```

---

## v2.2.x to v2.3.x

**Release:** [v2.3.0](RELEASE_2.3.0_EN.md)

### Breaking Changes

None documented. v2.3.x is a non-breaking continuation.

### New Features

- Web framework foundations
- Database provider strategy
- Enhanced async runtime

### Migration Steps

No migration required.

```bash
# Update to v2.3.0
zap update
```

---

## v2.1.x to v2.2.x

**Release:** [v2.2.0](RELEASE_2.2.0_EN.md) and [v2.2.1](RELEASE_2.2.1_EN.md)

**Important:** See [Post-v2.2.0 Remediation Record](POST_V2.2.0_REMEDIATION_EN.md) for v2.2.1 corrections.

### Breaking Changes

#### LSP Synchronization (v2.2.1 correction)

- **Change:** LSP server now requires standard full-text synchronization
- **Impact:** VS Code extension clients must send full document content on changes
- **Migration:** Update VS Code extension to v2.2.1 or later

#### LSP Rename (v2.2.1 correction)

- **Change:** Rename is now file-local only (no cross-file rename)
- **Impact:** Cross-file rename operations are no longer supported
- **Migration:** Use manual edits for cross-file symbol changes

#### Standard Library Determinism (v2.2.1 correction)

- **Change:** Standard library now reports determinism class instead of boolean
- **Impact:** Code depending on `deterministic: true` boolean may need updates
- **Migration:** Use `determinism_class` field with values: `pure`, `input-deterministic`, `runtime-dependent`, `external-io`

### Migration Steps

```bash
# Update VS Code extension
zap update editors/vscode

# Review LSP synchronization changes
# - Full-text sync is now required
# - Incremental range changes are rejected

# Update standard library checks
# - Replace `deterministic: true` checks with `determinism_class in ["pure", "input-deterministic"]`
```

---

## v2.0.x to v2.1.x

**Release:** [v2.1.0](RELEASE_2.1.0_EN.md)

### Breaking Changes

#### Legacy Runtime Deprecation

- **Change:** Python legacy runtime is now a compatibility reference only
- **Impact:** New features are native-only
- **Migration:** Use native runtime for all new code

See [P0-01 Migration Guidance](P001_MIGRATION_GUIDANCE_EN.md) for detailed native-only fixture handling.

### New Features

- B1 bootstrap: lexer and parser ownership
- B2 bootstrap: type checker and typed-IR
- Package manager foundation
- Standard library expansion

### Migration Steps

```bash
# Update to v2.1.0
zap update

# Review legacy-to-native migration
# - Check conformance/p0-01/matrix.tsv for native-only fixtures
# - Update modern declarations to use native runtime
# - Replace legacy-only APIs with native equivalents

# Run parity checks
scripts/test_p001_parity.sh
```

---

## v0.x to v2.0.x

**Release:** [v2.0.0](RELEASE_2.0.0_EN.md)

### Breaking Changes

#### Complete Language Rewrite

- **Change:** v2.0 is a complete rewrite of the Zap language
- **Impact:** v0.x code is not compatible with v2.0
- **Migration:** Manual code rewrite required

#### Syntax Changes

- **Change:** New syntax for declarations, functions, classes
- **Impact:** All v0.x syntax is invalid in v2.0
- **Migration:** Rewrite code using v2.0 syntax guide

#### Type System

- **Change:** Static type checking is now mandatory
- **Impact:** Untyped v0.x code will not compile
- **Migration:** Add type annotations to all functions and variables

### Migration Steps

v0.x to v2.0 is a major rewrite. There is no automated migration path.

```bash
# Install v2.0
# Follow installation guide in README

# Rewrite code using v2.0 syntax
# - See LANGUAGE_SPEC_EN.md for syntax reference
# - See LEARN_ZAP_EN.md for tutorial

# Add type annotations
# - Every function must have parameter and return types
# - Use type inference where possible

# Test with native runtime
zap test
```

### Migration Example

**v0.x code:**
```python
# v0.x - dynamic typing
def add(a, b):
    return a + b

result = add(1, 2)
print(result)
```

**v2.0 code:**
```zap
# v2.0 - static typing
fn add(a: int, b: int) -> int {
    return a + b
}

let result = add(1, 2)
print(result)
```

---

## General Migration Checklist

For any version transition:

1. **Backup your project**
   ```bash
   git commit -m "Backup before Zap migration"
   ```

2. **Read the release notes**
   - Check for breaking changes
   - Review new features
   - Note deprecation warnings

3. **Update dependencies**
   ```bash
   zap update
   ```

4. **Update your code**
   - Apply breaking changes
   - Replace deprecated APIs
   - Add required type annotations

5. **Run tests**
   ```bash
   zap test
   ```

6. **Check linting**
   ```bash
   zap check
   ```

7. **Build your project**
   ```bash
   zap build
   ```

8. **Test locally**
   ```bash
   zap run
   ```

---

## Getting Help

If you encounter migration issues:

- Check the [Language Specification](LANGUAGE_SPEC_EN.md)
- Review [Release Notes](RELEASE_2.11.18_EN.md)
- Consult [Migration Guidance](P001_MIGRATION_GUIDANCE_EN.md)
- Open an issue on [GitHub](https://github.com/hidecard/zap/issues)

---

## Version Support Policy

| Version | Support Status | End of Life |
|---------|----------------|-------------|
| v2.11.x | Current | TBD |
| v2.10.x | Maintenance | After v2.12 release |
| v2.2.x - v2.9.x | Security only | After v2.12 release |
| v2.0.x - v2.1.x | End of life | v2.10 release |
| v0.x | End of life | v2.0 release |

---

## References

- [Language Specification](LANGUAGE_SPEC_EN.md)
- [Language Specification (Myanmar)](LANGUAGE_SPEC_MM.md)
- [Current Status](CURRENT_STATUS_EN.md)
- [Current Status (Myanmar)](CURRENT_STATUS_MM.md)
- [Release Version Policy](RELEASE_VERSION_POLICY_EN.md)
- [Compatibility Matrix](COMPATIBILITY_MATRIX.md)
