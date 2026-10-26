---
section: Security
issues: [#4941, #4940]
---
- **`crypto-bigint` bumped 0.7.5 -> 0.7.6 for RUSTSEC-2026-0387** (#4941, #4940). `crypto-bigint` 0.7.5 could take secret-dependent branches in some operations intended to be constant-time. It reaches the backend through the `bergshamra` XML signature crates. A second copy, 0.5.5, stays in the tree through `elliptic-curve` 0.13 (`pgp` and `bergshamra`) because the 0.5 line has no patched release; the advisory is ignored in `.cargo/audit.toml` until those crates move off it (#4940). Lockfile-only; no behaviour change.
