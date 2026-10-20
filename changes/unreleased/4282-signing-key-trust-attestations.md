---
section: Added
issues: [#4282]
---
- **Signing keys can store an out-of-band OpenPGP trust attestation** (#4282). An admin fetches a challenge from `GET /api/v1/signing/keys/{id}/trust-attestation/challenge`, signs it with an external issuer key (for example a root key on a hardware token), and submits the detached signature to `POST .../trust-attestation/verify`; `GET .../trust-attestation` returns the stored record. A valid signature is recorded as `signature_valid`, a cryptographic check only and not a trust-anchor decision (#2462): nothing in signing or verification reads it yet. Replacing an existing issuer is refused (409). SHA-1/MD5 signatures, non-binary signatures, RSA issuer keys under 2048 bits and revoked or expired issuer keys are rejected, and each recorded attestation writes a `SIGNING_KEY_TRUST_ATTESTED` audit event.
