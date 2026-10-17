---
section: Added
issues: [#4529]
---
- **`PUT /api/v1/repositories/{key}/upstream-auth` now writes a `REPOSITORY_UPDATED` audit row recording the auth-type change and whether the credentials changed** (#4529). The endpoint changed a Remote's upstream credentials without an audit trail, so credential changes were audited on the `PATCH upstream_url` path (#4467) but not on this one. It now records the actor, the repository, the previous and current auth type and a `credentials_changed` flag — matching the shape #4467 uses — and never the secret values.
