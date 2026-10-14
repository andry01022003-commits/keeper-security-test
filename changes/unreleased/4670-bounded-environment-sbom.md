---
section: Fixed
issues: [#4670]
---
- **`POST /api/v1/sbom/environment` has a bounded response and can render one platform at a time** (#4670). The route built one document per (environment, platform) graph whole in memory, so a 31 MiB lockfile produced a 162 MB buffered response. The body is now serialized graph by graph into a buffer capped at 64 MiB; a response past the cap is refused with 422 `UNPROCESSABLE_ENTITY` naming the lockfile's graphs, and the new `platform` and `environment` query parameters select which graphs to render. A filter that matches no graph is a 400 that lists the graphs there are.
