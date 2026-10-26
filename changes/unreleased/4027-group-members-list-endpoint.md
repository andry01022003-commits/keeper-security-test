---
section: Added
issues: [#4027]
---
- **`GET /api/v1/groups/{id}/members` now lists a group's members instead of returning 405** (#4027). The path served only `POST` and `DELETE`, so a client that had learned the projects API (`GET /api/v1/projects/{id}/members`) got a bare 405 here even though the data was already readable through `GET /api/v1/groups/{id}`. The new endpoint returns the same `members` and `members_total` fields as the group detail endpoint, paginated with `member_limit` / `member_offset` (default 50, max 200) in join order, works for SSO and LDAP-synced groups, and enforces the same visibility rule as the group detail endpoint, so a caller outside the group still gets a 404 rather than the member list.
