---
section: Security
issues: [#4663]
---
- **An expired CI OIDC assertion no longer buys a session, and Artifact Keeper's own tokens get no expiry leeway** (#4663). `POST /api/v1/auth/ci/token` accepts an assertion up to 60 s past its `exp` for clock skew, and capped the minted token at that already-passed `exp`, so it answered `expires_in: 0` with a token that session validation then accepted for its own 60 s leeway. The exchange now refuses with 401 when less than one second of the assertion's validity remains, and never returns `expires_in: 0`. Tokens this server signs (sessions, refresh tokens, TOTP tickets, on HTTP and gRPC) are now validated without leeway, since issuer and verifier share a clock; they expire exactly at their `exp`. The assertion-side leeway is unchanged.
