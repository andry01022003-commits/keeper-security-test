---
section: Fixed
issues: [#4825]
---
- **A Remote repository created with `upstream_username`/`upstream_password` and no `upstream_auth_type` now stores them as basic auth instead of silently discarding them** (#4825). The create previously answered 201 with `upstream_auth_configured: false`, so the operator believed the repository was authenticated while every upstream fetch went out unauthenticated — and the #4662 public-remote credential guard never fired because nothing was stored. An explicit type still wins, and an explicit `none` is still honoured.
