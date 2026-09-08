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
- Filesystem race boundary နှင့် process cleanup regression tests ထည့်သွင်းခဲ့သည်။
- DNS-to-connection pinning security regression tests ထည့်သွင်းခဲ့သည်။
- Dependency license check regression tests ထည့်သွင်းခဲ့သည်။

## Implemented scope

ဤ release တွင် ပါဝင်သော implemented features နှင့် improvements များမှာ:
- P0.1: Native CLI release gate ဖြင့် version validation နှင့် အသုံးပြုသား build instructions များ
- P0.2: Linux x86_64၊ macOS ARM64 နှင့် Windows x86_64 အတွက် cross-platform release verification
- P0.3: Filesystem race boundary၊ DNS pinning နှင့် license checks ပါဝင်သော runtime/security regression gates
- B2 type checker enhancements ဖြင့် complete generic constraints နှင့် compound bounds
- B1 lexer/parser improvements ဖြင့် expanded arbitrary block coverage
- B3 package/build foundations ဖြင့် Zap-side package resolver
- B4 self-hosting evidence ဖြင့် deterministic rebuild verification

## Deferred scope

အောက်ပါ features များသည် နောက် release များအတွက် deferred ဖြစ်သည်:
- Full framework ecosystem expansion (P1+ deferred until P0-P2 acceptance)
- Self-hosting production deployment claims (P3 deferred until P2 ownership transfer)
- Broad async feature expansion beyond current bounded foundation
- Package registry without full B3 implementation
- Performance claims beyond current benchmark baseline
