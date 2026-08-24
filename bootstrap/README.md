# Zap Bootstrap Foundation

This directory contains the first self-hosting contract and owned fixtures for Zap. The current Rust implementation remains the B0 reference implementation. The future Zap implementation will be accepted as B1 only after it matches the canonical token, AST, diagnostic, and type-check artifacts described in [`contracts/BOOTSTRAP_CONTRACT_EN.md`](contracts/BOOTSTRAP_CONTRACT_EN.md) and [`contracts/BOOTSTRAP_CONTRACT_MM.md`](contracts/BOOTSTRAP_CONTRACT_MM.md).

## Layout

```text
bootstrap/
├── contracts/
│   ├── BOOTSTRAP_CONTRACT_EN.md
│   └── BOOTSTRAP_CONTRACT_MM.md
└── fixtures/
    ├── lexer/hello.zp
    ├── parser/precedence.zp
    ├── typecheck/list_number.zp
    ├── typecheck/type_error.zp
    └── stdlib/pure_values.zp
```

Run the cargo-independent structure check from the repository root:

```bash
bash scripts/test_bootstrap_contract.sh
```

When a built Zap binary is available, execute the fixtures as well:

```bash
ZAP_BIN=/path/to/zap bash scripts/test_bootstrap_contract.sh
```

The validator intentionally does not claim that a self-hosted compiler already exists. It protects the initial contract and fixture surface so that lexer, parser, type checker, standard library, and later B0/B1 differential runners can be added without changing the acceptance rules.
