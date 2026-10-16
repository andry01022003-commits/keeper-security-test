---
section: Added
issues: [#4028]
---
- **New guide explaining remote proxy cache freshness and the yank asymmetry: `docs/proxy-cache-freshness.md`** (#4028). The guide documents how a Remote or Virtual repository caches a package manager's metadata and its artifacts under two different schedules — mutable indexes/packuments/tag manifests (5 minutes) versus immutable artifacts (effectively forever) — and why an upstream yank therefore surfaces as a *resolution* failure with the artifact still served. It covers conditional revalidation, the stale-if-error window, negative caching of a definitive 404/410 (which deletes the cached body), the `cache_ttl_seconds` override on mutable paths only, and the `POST /api/v1/repositories/{key}/cache/invalidate` escape hatch.
