---
section: Fixed
issues: [#4648]
---
- **Every login response now says how long the session lasts, and the short login lifetime is documented** (#4648). `POST /api/v1/auth/login` already returned `expires_in` and `token_type`, but LDAP login (`POST /api/v1/auth/sso/ldap/{id}/login`) and the SSO exchange (`POST /api/v1/auth/sso/exchange`) returned no `expires_in` and set the access cookie for one hour while the token itself expires after `JWT_ACCESS_TOKEN_EXPIRY_MINUTES` (30 by default). Both now return `expires_in` in seconds and size the cookie to the token's real lifetime. `.env.example` and `docs/api-token-expiration-policy.md` now state that login sessions are short-lived, that `JWT_ACCESS_TOKEN_EXPIRY_MINUTES` sets the lifetime (the uncommented `JWT_EXPIRATION_SECS=86400` there suggested a 24-hour session but no token lifetime reads it), and that scripts and CI should mint an API token with `POST /api/v1/auth/tokens` instead of logging in. The default lifetime is unchanged.
