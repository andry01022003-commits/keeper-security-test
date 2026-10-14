---
section: Security
issues: [#4679]
---
- **`rustls` 0.23.45 -> 0.23.46 for RUSTSEC-2026-0346, -0347, -0348 and -0349** (#4679). `rustls` is the TLS stack under every outbound connection the registry makes (`reqwest`, `hyper-rustls`, `redis`, `sqlx`'s `tls-rustls`, the Prometheus exporter and the backend's own dependency). 0.23.45 could leak TLS secrets through `Debug` output, send buffered application data to the outer server after an ECH rejection, allocate up to 1 GiB decompressing a Brotli certificate before authenticating the peer, and let an unauthenticated ServerHello downgrade later connections to classical key exchange. Lockfile-only; `backend/Cargo.toml` already allowed `0.23` with a caret.
