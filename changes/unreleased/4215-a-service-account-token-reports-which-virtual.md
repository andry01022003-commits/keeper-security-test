---
section: Added
issues: [#4215]
---
- **Service-account token diagnostics name the virtual-repository members a token cannot read, and flag repository restrictions that deny everything** (#4215). `GET /api/v1/service-accounts/{id}/tokens/{token_id}/scope-analysis` lists, per virtual repository in the token's scope, the leaf members the service account cannot read through it and why (`no_grant`), and the token list gains `unreachable_member_count` per token. `POST /api/v1/service-accounts/repo-selector/preview` accepts an optional `service_account_id` and reports the same before a token is minted. The analysis uses the read paths' own helpers (recursive member walk, parent visibility, owner grant and read-action checks, and the #4559 rule that a token naming the virtual reaches its members through it), so it cannot drift from what a download does.

  Token rows and the analysis also carry an optional `scope_error`: `invalid_selector` (a stored selector that no longer parses), `selector_matches_nothing`, or `deleted_pinned_repositories` (every pinned repository was deleted). All three already deny every repository at authentication; authentication and the diagnostics now share one stored-scope resolver. The preview rejects a selector with unknown keys instead of ignoring them. All of these routes are admin-only, and nothing here changes what a token may access.
