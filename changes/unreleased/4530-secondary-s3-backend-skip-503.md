---
section: Fixed
issues: [#4530]
---
- **A skipped secondary S3/Azure/GCS backend now returns 503 with the configuration error and is reported in health** (#4530). When a secondary storage backend fails configuration at startup (for example a malformed `S3_PUBLIC_ENDPOINT`), repositories on it now answer `503 SERVICE_UNAVAILABLE` naming the backend and the configuration error instead of `500 STORAGE_ERROR`, and `/health` reports the skipped backends as degraded.
