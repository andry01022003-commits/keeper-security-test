---
section: Fixed
issues: [#4308]
---
- **A database failure while authorizing a virtual repository's members now answers a retryable 503 instead of a 404 that Gradle caches** (#4308). The Maven virtual checksum walk and the two shared virtual download resolvers used the infallible member authorization, which turned a failed query (for example a pool timeout under load) into an empty member set and answered `404 Virtual repository has no accessible members`. Gradle recorded the artifact as missing and failed every later build until its cache was cleared. These paths now use the fallible form, so a pool timeout answers `503 POOL_EXHAUSTED` with `Retry-After` (any other database error a 500), while a caller who really has no readable member still gets the same 404.
