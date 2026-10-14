---
section: Security
issues: [#4550]
---
- **Repository-scoped admin tokens are refused on `/api/v1/admin/downloads*` and `/api/v1/admin/stats`** (#4550). An admin's API token or JWT restricted to some repositories could list download attribution (client IPs, users, paths) and instance-wide statistics for every repository, while `/api/v1/admin/holds/*` already refused it. These endpoints now apply the same rule, that token scope binds ahead of admin, and return 403 for such tokens. Unrestricted admin tokens are unaffected.
