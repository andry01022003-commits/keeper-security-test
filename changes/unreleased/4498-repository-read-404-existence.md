---
section: Security
issues: [#4498]
---
- **A repository the caller cannot read now answers exactly like one that does not exist** (Closes #4498). The REST read endpoints that hide a private repository's existence (`GET /api/v1/repositories/{key}`, its `artifacts`, `artifacts/{path}`, `download/{path}`, `versions/{path}` and `storage` sub-resources, `GET /api/v1/promotion/repositories/{key}/promotion-history` and `release-target`, and the other reads gated the same way) refused an unreadable repository with `Repository '<key>' not found`, while a key that names no repository answered `Repository not found`. The two 404s had the same status but different bodies, so an anonymous caller or a user without a grant could tell that a private repository exists. Both now return the same body, `{"code":"NOT_FOUND","message":"Repository not found"}`.
