---
section: Fixed
issues: [#4669]
---
- **The dependency scanner asks OSV about the scanned package itself, not only its dependencies** (#4669). A leaf package with no dependencies, such as `lodash@4.17.20`, put nothing in the advisory batch and graded `complete` with 0 findings and `live:osv` provenance without a single OSV request. The package's own pinned identity (npm, PyPI sdist, RubyGems, Cargo, NuGet and Maven coordinates, the same pin the CVE engines grade) now joins the batch, so its own advisories are reported; it is queried once even when the manifest also names it, and is not added to the dependency inventory.
