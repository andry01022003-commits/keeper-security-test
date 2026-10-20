---
section: Security
issues: [#4842]
---
- **WASM plugin runtime updated to wasmtime 36.0.19** (#4842; RUSTSEC-2026-0379, RUSTSEC-2026-0380). Two Cranelift code-generation defects in 36.0.18 could miscompile plugin code: a large Wasm `br_table` instruction could jump to the wrong target on aarch64, and a bit-packing overflow in the compiler could silently produce wrong machine code. Both are fixed upstream in 36.0.19. The `wasmtime` and `wasmtime-wasi` workspace pins move together as in #4679, and no configuration change is needed.
