---
section: Added
issues: [#3954]
---
- **The repository upstream check, the curation sync and the curation popularity lookups now trace their upstream requests** (#3954). `POST /api/v1/repositories/{key}/test-upstream`, the curation sync's RPM and Debian index fetches and PyPI provenance lookups, and the pypistats / npm download-count lookups now send through the same path as the proxy, so each request exports an OpenTelemetry `CLIENT` span with the HTTP client attributes (`url.full` without credentials or query, `http.response.status_code`, `error.type`) and forwards `traceparent`. The scheduled curation sync runs outside any request, so each of its fetches starts a trace of its own. See `docs/operations/logging.md`.
