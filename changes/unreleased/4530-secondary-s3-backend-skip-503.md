---
section: Fixed
issues: [#4530]
---
- **A secondary S3/Azure/GCS storage backend skipped at startup is now visible instead of a bare 500** (#4530). When a configured secondary backend fails to initialize (for example a malformed `S3_PUBLIC_ENDPOINT`), the skip is logged at ERROR, repositories on it answer `503 SERVICE_UNAVAILABLE` naming the backend instead of `500 STORAGE_ERROR`, `/health` lists it under `skipped_storage_backends` with its configuration error, and the `ak_storage_backends_skipped` gauge counts such backends (0 when none). Behaviour change: while a configured secondary backend is skipped, `/health` reports `unhealthy` with HTTP 503, so the image's Docker `HEALTHCHECK` (which polls `/health`) marks the container unhealthy; `/livez` and `/readyz`, which the Helm chart probes, are unchanged. A GCS configuration error other than an unset `GCS_BUCKET` now also counts as a skip.
