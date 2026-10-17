---
section: Fixed
issues: [#4642]
---
- **`/metrics` now exports `ak_artifact_uploads_total`, `ak_artifact_upload_size_bytes` and `ak_artifact_downloads_total`** (#4642). The three metrics were defined but nothing recorded them, so dashboards built on them stayed empty. Uploads are counted at the post-commit step every upload path already reaches (the one that starts scan-on-upload), for every format's native client and the upload API, whether or not scanning is configured. Downloads are counted when the download-statistics row is written, so the counter matches the download counts the UI shows and a `HEAD` is never counted. Labels are the repository key and format only. The metrics are documented in `docs/operations/artifact-metrics.md`.
