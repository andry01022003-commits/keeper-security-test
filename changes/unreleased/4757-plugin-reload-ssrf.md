---
section: Security
issues: [#4757]
---
- **Reloading a git-sourced WASM plugin now runs the same outbound-URL check that install does before re-cloning its stored source** (#4757, GHSA-r8fc-5fvw-rvh4). `POST /api/v1/plugins/{id}/reload` re-cloned the plugin's stored `source_url` with no SSRF validation, so a source whose host had since moved to, or started resolving to, a private or internal address was fetched from anyway. Every git re-fetch now validates the stored URL first and refuses a blocked source with the same `400` install returns, naming `AK_SSRF_ALLOW_PRIVATE_CIDRS` for an address an operator can admit, before any clone is attempted. Reload is admin-only. Thanks to @vadim-belous for the report.
