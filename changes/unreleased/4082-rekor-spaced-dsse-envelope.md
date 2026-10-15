---
section: Fixed
issues: [#4082]
---
- **PyPI attestations published before about May 2026 verify again instead of failing as a signature/certificate-chain error** (#4082). Rekor records a DSSE entry's `envelopeHash` over the envelope exactly as it was submitted, and PyPI's publishing tooling submitted Python's default spaced JSON (`{"payload": ...}`) until it switched to compact JSON around 2026-05. `sigstore-rs` re-serializes the envelope compactly before comparing, so every attestation from the first ~18 months of PEP 740 failed its transparency-log consistency check although the signature, certificate chain and log entry were all genuine. Verification now accepts the recorded `envelopeHash` when it matches either the compact or the spaced serialization of the bundle's own envelope; the Rekor SET and inclusion proof are still checked against the unmodified log entry.

  A Rekor entry that disagrees with its bundle is now reported as `transparency log entry mismatch` (naming the envelopeHash it recorded and both hashes it was compared with) rather than `signature/certificate chain`, which sent operators auditing certificates, keys and clock skew that were never wrong. This covers conda CEP-27 attestations too, which share the same verification core.
