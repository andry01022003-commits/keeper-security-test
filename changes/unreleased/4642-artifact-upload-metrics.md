---
section: Fixed
issues: [#4642]
---
- **`ak_artifact_uploads_total` and `ak_artifact_upload_size_bytes` show up in `/metrics` again** (#4642). `record_artifact_upload` was defined but never called outside its own tests, so a scrape returned the `ak_http_*` series and none of the artifact ones, and any dashboard built on them stayed empty. The call now happens in `finalize_upload`, which both the buffered and the streaming upload path go through, so each upload is counted once and a failed one isn't counted at all.

  `ak_artifact_downloads_total` is still missing a producer. The shared download recorder only gets an artifact id, and resolving the repo from it would put a query on the serving path, so that half needs a separate decision.
