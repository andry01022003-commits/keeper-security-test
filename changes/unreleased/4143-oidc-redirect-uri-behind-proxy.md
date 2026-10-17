---
section: Fixed
issues: [#4143]
---
- **The OIDC `redirect_uri` behind a reverse proxy now follows only trusted forwarding headers, and an `http://` one is called out in the log** (#4143). When `AK_EXTERNAL_URL` is unset, the backend builds the `redirect_uri` it sends to the IdP from `X-Forwarded-Proto` / `X-Forwarded-Host`; it believed those headers from any client. Once `RATE_LIMIT_TRUSTED_PROXY_CIDRS` is set they are now honoured only when the TCP peer is one of the listed proxies, the same rule #4681 applies to `X-Forwarded-For`, so a client cannot steer the `redirect_uri`; with the list empty the previous behaviour is kept. A TLS-terminating proxy that sends no `X-Forwarded-Proto: https` produces an `http://` `redirect_uri` that Microsoft Entra rejects with AADSTS50011; the login now logs a warning saying so and naming `AK_EXTERNAL_URL`, the setting that fixes it, and `.env.example` documents it for SSO.
