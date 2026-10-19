---
section: Fixed
issues: [#4740]
---
- **A migrated Debian package is filed under the component the source records, not one guessed from its path** (#4740). A `.deb` in the `pool/<letter>/<name>/` layout (a Nexus apt repository, or an Artifactory mirror of Ubuntu's pool) kept its path, so the letter became its component and the package was missing from `dists/<dist>/main/`. The migration now reads the item's Artifactory `deb.component` property; without one, a path with no component segment files the package under `main`, and a path that already has one (`pool/universe/...`) keeps it. The duplicate check looks the package up at that destination, so re-running a job moves a package an earlier run filed under the wrong component and retires the old row.
