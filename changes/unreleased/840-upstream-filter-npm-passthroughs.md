---
section: Changed
issues: [#840]
---
- **A remote repository's upstream filter now also covers the npm `/-/` passthroughs and npm audit** (#840). The 1.11.0 upstream filter (`include_patterns` / `exclude_patterns`) gated every fetch made through the proxy cache, but the npm `/-/` namespace (search, attestations, ping and the rest) and `npm audit` called the upstream directly. They now match the filter against the path they would send (`-/v1/search?text=...`, `-/npm/v1/security/advisories/bulk`): a refused `/-/` request answers 404 and a virtual repository skips that member, and a refused audit answers the empty advisories body, all without contacting the upstream. The curation upstream sync and the admin `test-upstream` probe are not filtered yet.
