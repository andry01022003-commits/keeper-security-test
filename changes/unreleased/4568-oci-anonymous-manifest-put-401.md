---
section: Fixed
issues: [#4568]
---
- **An anonymous OCI manifest PUT now gets 401 with the push challenge instead of a 500** (#4568). The manifest `PUT` read the whole request body before it checked any credentials, so the retry `crane tag` / `crane copy` sends after a token exchange (it reuses the spent request: `Content-Length` set, 0 bytes written) failed the body read and answered `500 BLOB_UPLOAD_UNKNOWN` (502 behind Caddy) instead of the `401` and `WWW-Authenticate` challenge. The handler now authenticates, checks the push scope, repository write access and the repository-type and promotion-only gates first, and reads the body only after them, the order the blob upload handlers already used. An unauthenticated client can also no longer make the server buffer up to `max_upload_size_bytes` of manifest before it is refused.
