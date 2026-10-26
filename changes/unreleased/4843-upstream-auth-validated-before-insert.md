---
section: Fixed
issues: [#4843]
---
- **An invalid `upstream_auth_type` on repository create is refused before the repository row is written** (#4843). An unknown auth type, `basic` without a password, or an AWS auth type without an `aws` block returned 400 `VALIDATION_ERROR` only after the repository had been inserted, leaving a half-configured Remote behind (and a retry failing with 409 on the key). The credentials are now built and validated up front, alongside the `upstream_username`/`upstream_password` check (#4825), so a refused create leaves nothing behind.
