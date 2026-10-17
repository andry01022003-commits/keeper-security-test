---
section: Fixed
issues: [#4758, #4630]
---
- **A proxy-cache entry whose body no longer has the size its sidecar records is refetched on filesystem storage too, instead of being served as `Content-Length: N` over a short body** (#4758, #4630). The #4630 read-side guard recognized a poisoned entry by the storage ETag its sidecar pinned, and the filesystem backend (the default) pins none, so an emptied or truncated `__content__` under an intact `__cache_meta__.json` still answered `200` with the sidecar's length and no body, without contacting upstream, on every request.

  When a sidecar pins no storage ETag, the streaming hit path and the stale-if-error fallback now compare the stored object's size with the size the sidecar records. On a mismatch the entry is evicted (body and sidecar), a warn names the cache key and both sizes, and the request falls through to the upstream fetch, which republishes the entry. Entries that do pin an ETag keep the existing checks and pay no extra storage call.
