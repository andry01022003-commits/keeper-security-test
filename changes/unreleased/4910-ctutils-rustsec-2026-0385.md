---
section: Security
issues: [#4910]
---
- **`ctutils` bumped 0.4.0 -> 0.4.5 for RUSTSEC-2026-0385** (#4910). `ctutils` 0.4.0 generated masks and compared `u128` values in non-constant time. It reaches the backend through `crypto-bigint` under the `bergshamra` XML signature crates. Lockfile-only; no behaviour change.
