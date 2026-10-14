---
section: Security
issues: [#4679]
---
- **`quinn-proto` bumped 0.11.15 -> 0.11.19 for RUSTSEC-2026-0338, -0339, -0340, -0341 and -0342** (#4679). The QUIC implementation under `reqwest`'s HTTP/3 support could be driven into unbounded memory growth through stream reassembly, pending connection-ID retirements and zero-length DATAGRAM frames. Lockfile-only; the registry does not negotiate HTTP/3 today, so there is no behaviour change.
