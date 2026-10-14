---
section: Fixed
issues: [#4671]
---
- **`POST /api/v1/plugins/install/local` answers 501 `NOT_IMPLEMENTED` instead of 500** (#4671). The route is a known, intentional gap, but it resolved the caller's path on the server and then failed with an alert-generating 500 from a placeholder. It now returns 501 with the stable error code `NOT_IMPLEMENTED` without touching the filesystem, logs at info rather than error, and the OpenAPI document describes the 501.
