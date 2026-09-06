# Contributing to Zap

Thank you for your interest in contributing to Zap! This guide will help you get started.

## Quick Start

1. Fork and clone the repository
2. Build the project: `cargo build --release --locked --manifest-path native/Cargo.toml`
3. Run tests: `bash scripts/bootstrap/verify_b0_artifacts.sh --release`
4. Make your changes
5. Ensure CI passes: `git push origin <branch>`
6. Open a pull request

## Development Workflow

### Bootstrap Stage
Zap is currently in bootstrap phase. The compiler is implemented in Rust and we are building toward self-hosting.

- B0: Artifact contracts and metadata
- B1: Lexer and parser ownership
- B2: Type checker and flow analysis
- B3: Canonical AST bridge and typed-IR producer
- B4: Rust-free full-language contract

### Running Verifiers
```bash
# B0 artifacts
bash scripts/bootstrap/verify_b0_artifacts.sh --release

# B1 lexer
bash scripts/bootstrap/verify_b1_lexer.sh

# B1 parser
bash scripts/bootstrap/verify_b1_parser.sh

# B2 type checker
bash scripts/bootstrap/verify_b2_typecheck.sh

# All B4 gates
bash scripts/bootstrap/verify_b4_evidence.sh --run-gates
```

### Code Style
- Follow existing code conventions in the file you are editing
- Run `cargo fmt -- --check` before committing
- Ensure `cargo clippy` passes without warnings

## Reporting Bugs

Please use the bug report issue template when reporting bugs. Include:
- Zap version (run `zap --version`)
- Platform (OS, architecture)
- Minimal reproduction case
- Expected vs actual behavior

## Suggesting Features

For language changes, use the RFC process (see `RFC_TEMPLATE.md`).
For standard library additions, open a feature request issue.

## Security Issues

See `SECURITY.md` for how to report security vulnerabilities.

## Release Calendar

Zap follows a monthly release calendar:
- Feature freeze: 15th of each month
- Release: last day of each month
- Security patches: as needed
