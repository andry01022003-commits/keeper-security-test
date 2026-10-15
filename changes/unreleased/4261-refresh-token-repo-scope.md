---
section: Security
issues: [#4261]
---
- **A repository-scoped refresh token no longer widens to an unrestricted grant when rotated** (#4261). `generate_token_pair_capped` stamped the refresh JWT with `allowed_repo_ids: None` while the paired access token carried the real allow-list, so a client holding a scoped credential could present its refresh token and be handed an access token with no repository ceiling. The refresh claims now carry the same allow-list as the access token, so rotation preserves the ceiling instead of silently dropping it.
