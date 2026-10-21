---
section: Security
issues: [#4861]
---
- **TLS certificate verification updated to rustls-webpki 0.103.16 and webpki-roots / webpki-root-certs 1.0.10** (#4861; RUSTSEC-2026-0381, RUSTSEC-2026-0382, RUSTSEC-2026-0383). The previous versions could accept a certificate chain through a distrusted intermediate, and the bundled root set still trusted the distrusted emSign intermediates. No configuration change is needed.
