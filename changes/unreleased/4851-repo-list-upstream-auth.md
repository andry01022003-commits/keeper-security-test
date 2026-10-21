---
section: Fixed
issues: [#4851]
---
- **The repository list and the repository update response now report a Remote's upstream authentication** (#4851). `GET /api/v1/repositories` and `PATCH /api/v1/repositories/{key}` answered `upstream_auth_type: null` and `upstream_auth_configured: false` for every repository, even one with upstream credentials configured, while `GET /api/v1/repositories/{key}` reported them correctly. The web UI's edit dialog for a Remote reads the list row, so it showed no authentication configured. The list already read the upstream auth rows (for `public_with_upstream_credentials`) but never projected them; the list, create and update responses now set both fields from those rows, exactly as the detail response does. Only the auth type and whether one is configured are reported, never the credentials.
