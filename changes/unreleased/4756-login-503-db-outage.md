---
section: Fixed
issues: [#4756]
---
- **Login and other API requests now return a retryable 503 during a database outage, not a 500** (#4756). With the database unreachable, `POST /api/v1/auth/login` returned `500 DATABASE_ERROR` for correct and wrong credentials alike, because the user lookup cannot run. The same happened to any handler whose own query hit the outage, for example `GET /api/v1/auth/me` with an API token served from the token cache. Clients that treat a 5xx other than 503 as a server bug gave up instead of retrying. A lost, refused or closed database connection, a statement or lock timeout, and a server that is shutting down or recovering now return `503` with code `DATABASE_UNAVAILABLE` and `Retry-After: 1`, using the same classification as #4750. A pool-acquire timeout still returns `503 POOL_EXHAUSTED`, and query or data errors still return `500 DATABASE_ERROR`.
