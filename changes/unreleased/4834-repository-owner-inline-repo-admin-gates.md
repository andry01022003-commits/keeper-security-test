---
section: Fixed
issues: [#4834]
---
- **A `repository-owner` role member can once again use every inline repository-administration route** (#4834). The repository update/delete, cache-TTL, upstream-not-found, stale-if-error grace, npm scope policy, proxy cache invalidation and virtual-member mutation handlers still gated on `PermissionService::check_permission`, which reads fine-grained `permissions` rows only; a member holding the `admin` action through a role assignment (the `repository-owner` role auto-granted to a repository's creator) was therefore refused 403 — the same gap `require_repo_admin` had in #4818. Each site now resolves the `admin` action through `check_repository_action`, so role-derived repository admin is honoured, while a write-only member is still refused.
