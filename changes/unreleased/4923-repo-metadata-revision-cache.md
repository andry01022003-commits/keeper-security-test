---
section: Added
issues: [#4923, #4174]
---
- **Repository metadata renders are now observable: `ak_repo_metadata_regenerations_total`, `ak_repo_metadata_regeneration_seconds` and `ak_repo_metadata_cache_requests_total`** (#4923, #4174). The RPM repodata cache, which renders `repomd.xml` and its indexes once per repository state, moved into a format-agnostic metadata revision cache that conda repodata will also use. Its behaviour for RPM is unchanged; it now reports each render with `format` and `scope` labels and each request as a `hit`, `coalesced` or `miss`. See `docs/operations/repo-metadata-cache-metrics.md`.
