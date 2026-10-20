---
section: Fixed
issues: [#4815]
---
- **Per-repository settings reads now require read access to the repository** (#4815). `GET /api/v1/repositories/{key}/cache-ttl`, `/on-upstream-not-found`, `/stale-if-error-grace`, `/routing-rules` and `/pypi-tracks` answered any caller, including an anonymous one, for any repository. They now apply the same visibility check as `GET /api/v1/repositories/{key}`: a repository the caller may not read answers 404, exactly like a key that does not exist.
