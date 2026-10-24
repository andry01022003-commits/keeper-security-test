---
section: Security
issues: [#4883]
---
- **Access tokens are invalidated at logout** (#4883). Signing out now ends the session's access token as well as its refresh token, and so does refresh-token replay detection. Before, the access token stayed valid until it expired (30 minutes by default). The replica that handled the logout refuses the token at once; other replicas refuse it within `SESSION_REVOCATION_CACHE_TTL_SECS` (default 5 seconds, `0` to 300). Other sessions of the same user are not affected. Adds migration 294 (`session_family_revocations`, additive). See `docs/session-sign-out.md`.
