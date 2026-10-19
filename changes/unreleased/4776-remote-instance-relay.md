---
section: Fixed
issues: [#4776]
---
- **The remote-instance relay now forwards query strings and content types, relays useful response headers, and refuses to chain to another relay** (#4776). `/api/v1/instances/{id}/proxy/{*path}` sent only the path to the remote, so pagination, search and filter parameters were silently dropped; it relabelled every request body as `application/json`, so multipart or octet-stream bodies were misparsed; and it returned only `content-type`/`content-length`, losing `content-disposition`, `etag`, `last-modified`, `link` and `x-total-count`. The relay now appends the caller's query string, passes the caller's `content-type` through (still defaulting to JSON when none is sent), and forwards an allow-list of response headers. Paths under `api/v1/instances` are now rejected with 400 instead of being forwarded to the remote's own relay or instance registry. A PATCH relay route (needed for chunked uploads through the relay) follows in 1.12.0.
