---
section: Added
issues: [#4319]
---
- **Proxy cache write failures are counted in `ak_proxy_cache_publish_failures_total`** (#4319). A proxy repository serves the upstream body and writes it to the cache at the same time; when the cache write failed, the client still got its bytes and the failure was only logged, so a storage backend that refused every cache write looked healthy while every request missed the cache. The new counter is labelled by `backend` (`filesystem`, `s3`, `gcs`) and `class` (`put_failed`, `copy_rejected`, `publish_mismatch`, `commit_failed`) and is incremented where the failure is logged. Upstream errors, the per-object size ceiling and client disconnects are not counted. See `docs/operations/proxy-cache-metrics.md`.
