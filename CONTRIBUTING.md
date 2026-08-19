# Contributing to Zap

Thank you for helping improve the Zap programming language. Contributions are welcome for the native runtime, parser, diagnostics, standard library, documentation, tests, release automation, and examples.

## Before You Start

Please read the [README](README.md), the relevant English or Burmese usage guide, and the [Code of Conduct](CODE_OF_CONDUCT.md). For security vulnerabilities, use the private process in [SECURITY.md](SECURITY.md) instead of opening a public issue.

## Development Workflow

1. Fork the repository and create a focused branch from `master`.
2. Make the smallest complete change that addresses the issue.
3. Add or update regression tests, especially for parser, evaluator, OOP, module, diagnostics, and cross-platform behavior.
4. Update English and Burmese documentation when user-visible behavior changes.
5. Run the local quality gates before opening a pull request.
6. Open a pull request with a clear description, test evidence, compatibility considerations, and any documentation impact.

## Local Quality Gates

Run the following commands from the repository root:

```bash
cargo fmt --manifest-path native/Cargo.toml --all -- --check
cargo clippy --manifest-path native/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --manifest-path native/Cargo.toml --all-targets --all-features
git diff --check
```

For release-related changes, also verify the native package and all supported build targets through the GitHub Actions workflow. Do not commit generated build directories, credentials, local configuration, or large binary artifacts.

## Pull Request Expectations

A pull request should explain the problem, the chosen solution, and the tests that were run. Changes to language syntax must include parser and evaluator coverage. Changes to runtime behavior must document error behavior, resource limits, and cross-platform implications. Changes to public documentation should keep English and Burmese materials synchronized where practical.

Please keep commits focused and use descriptive commit messages such as `feat:`, `fix:`, `docs:`, `test:`, or `ci:`. Reviewers may request changes when a proposal weakens diagnostics, safety limits, portability, or backward compatibility.

## Design Principles

Zap contributions should favor readable syntax, deterministic diagnostics, safe handling of malformed input, bounded resource usage, explicit compatibility behavior, and a clear path for beginners to learn the language.
