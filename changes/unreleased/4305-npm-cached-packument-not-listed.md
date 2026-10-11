---
section: Fixed
issues: [#4305]
---
- **An npm Remote's cached packuments no longer appear in its cached-artifact listing as if they were packages** (closes #4305). The proxy cache stores each packument under the bare package name (`@scope/name`, `name`) next to the tarballs (`@scope/name/-/name-1.0.0.tgz`), and the listing showed it as an artifact: about 1.9 MB of `application/json` that could be downloaded and its cache entry invalidated, while deleting it answered `Artifact not found`. For the npm family (npm, yarn, pnpm, bower), a cached path without the registry's `/-/` tarball segment is now treated as metadata, like a PyPI index page: it is left out of the listing and its count and out of the proxy-scans view (where it was a permanent `not_scanned` entry), and stays in the cache and in storage accounting. Tarballs are listed as before.
