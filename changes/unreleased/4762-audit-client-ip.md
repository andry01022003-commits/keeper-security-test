---
section: Fixed
issues: [#4762]
---
- **Every audit row written while handling a request now records the client IP, including `REPOSITORY_CREATED` and `USER_CREATED`** (#4762). Only login, the other authentication events and the #4710 rows asked for the client IP, so repository and user creation, update and deletion, role changes, password and TOTP changes, token mint and revoke, settings and policy changes, curation, trash restore, SBOM, storage integrity and the other older writers stored an empty `ip_address` (and a `null` `source_ip` in the audit stream). An audit entry now takes the request's verified client IP by default, the same address login records: the TCP peer, or the forwarded address only behind a trusted proxy with `XFF_TRUSTED_FOR_AUTHZ=true`. Rows written by background jobs, or behind a trusted proxy without that setting, still record no IP rather than an unverified one. `docs/audit-events.md` describes the column.
