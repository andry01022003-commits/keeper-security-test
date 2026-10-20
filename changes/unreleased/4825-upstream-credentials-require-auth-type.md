---
section: Fixed
issues: [#4825]
---
- **Creating a repository with `upstream_username`/`upstream_password` but no `upstream_auth_type` is refused instead of silently dropping the credentials** (#4825). The create used to answer 201 and store nothing, so the Remote fetched upstream unauthenticated while the operator believed it was configured, and the public-remote credential check (#4662) never saw the credentials. It now answers 400 `VALIDATION_ERROR` naming `upstream_auth_type` and creates nothing. The type is not inferred, matching `PUT /api/v1/repositories/{key}/upstream-auth`, which also requires `auth_type`. Empty strings still count as absent.
