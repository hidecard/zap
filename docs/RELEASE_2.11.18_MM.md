# Zap v2.11.18 ပြန်လည်ရရှိမှု မှတ်တမ်း

## Release အကျဉ်းချုပ်

- B2 type checker: generic constraints, compound bounds, alias checking & verifiers အားလုံး ပြီးစီး။
- B1 lexer/parser: arbitrary block coverage နှင့် parser diagnostic parity ခြCrossref ခဲ့သည်။
- Typed-IR generalization: arbitrary expression typed-IR, control-flow typed-IR, နှင့် cross-module typed-IR။
- B3 package/build: Zap-side package resolver, dependency graph ownership, နှင့် typed-IR to bytecode lowering။
- B4 self-hosting: deterministic rebuild evidence, second-stage compiler rebuild, နှင့် clean environment verification။
- Bootstrap validation gates: B0, B1, B3, VM platform, non-Rust seed pipeline, နှင့် B4 byte-determinism အားလုံး passed။

## Platform ထောက်ခံမှု

- Linux x86_64: စစ်ဆေးပြီး
- macOS ARM64: build/test pending CI
- Windows x86_64: build/test pending CI

## လုံခြုံရေး

- RustSec cargo audit: advisory database CVSS 4.0 compatibility note (cargo-audit 0.17.0; newer advisory DB format pending toolchain update).
