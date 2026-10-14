---
section: Security
issues: [#4679]
---
- **WASM plugin runtime updated to wasmtime 36.0.18** (#4679; RUSTSEC-2026-0352, RUSTSEC-2026-0363). A WASM plugin guest could escape its filesystem sandbox by renaming a symlink with a trailing slash, and component-model handle tables were capped only per instance and charged to no limiter, so a guest could grow host memory past its store limits. Both are fixed upstream in 36.0.18; both workspace pins (`wasmtime`, `wasmtime-wasi`) move together as in #4361 and #4332, and no configuration change is needed.
