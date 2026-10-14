---
section: Fixed
issues: [#4664]
---
- **A login refused with 429 now reports the login limiter's `X-RateLimit-Limit` and `X-RateLimit-Remaining`, not the general API bucket's** (#4664). The login limiter (10 attempts per 900 s per username and source IP) sits inside the general API limiter, and the outer limiter, which had allowed the request, overwrote the headers of the inner limiter's 429 with its own (`X-RateLimit-Limit: 10000`) while `Retry-After` still came from the login bucket. Clients and dashboards reading the headers concluded the API limiter had fired. An outer limiter now leaves a 429 that already describes its limiter untouched, so the headers and `Retry-After` describe the same bucket. The same applies to any limiter nested inside the API limiter.
