---
section: Fixed
issues: [#4505]
---
- **Creating a virtual repository whose members already hold content reports their total in the create response** (Closes #4505). `POST /api/v1/repositories` for a virtual repository with `member_repos` returned `member_storage_used_bytes: 0` even when the members held data, while the next `GET` and any `PATCH` reported the real total. The create response now computes the member total after attaching the members, scoped to the members the caller can see, as `GET` does.
