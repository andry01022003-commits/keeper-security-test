---
section: Security
issues: [#4675]
---
- **Download tickets and other URL-carried credentials are redacted from the `uri` recorded on every request span** (#4675). The `http_request` span already replaced `token`, `key`, `api_key`, `password` and `secret` query values with `[REDACTED]` (#544), but not `ticket`, so every `?ticket=` download put a live single-use ticket into the trace and log pipeline. The list now also covers `ticket`, the OAuth2 / OCI `/v2/token` style `access_token`, `refresh_token`, `id_token` and `client_secret`, the OIDC callback's authorization `code`, and presigned-URL signatures (`X-Amz-Signature`, `X-Amz-Credential`, `X-Amz-Security-Token`, `X-Goog-Signature`, `X-Goog-Credential`, `Signature`, `sig`). Parameter names are matched case-insensitively after percent-decoding, so `%74icket=` is caught too; other parameters are logged unchanged.
