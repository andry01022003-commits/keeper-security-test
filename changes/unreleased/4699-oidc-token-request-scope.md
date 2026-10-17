---
section: Fixed
issues: [#4699]
---
- **OIDC sign-in with Microsoft Entra ID no longer fails with `Token response missing id_token`** (#4699). The authorization-code exchange now sends the provider's configured scopes (`scope`, default `openid profile email`) on the token request as well as on the login redirect; Entra's v2 token endpoint only returns an `id_token` when the token request itself carries `openid`, and RFC 6749 lets every other IdP accept it. When an IdP still answers without an `id_token`, the callback now returns `502` with code `OIDC_NO_ID_TOKEN` and a message naming the provider (and the IdP's OAuth `error` code, when it sent one) instead of an opaque `500 INTERNAL_ERROR`.
