---
section: Fixed
issues: [#4633]
---
- **A Maven package version's size on the Packages page is the total of its files, not one file's** (#4633). Each upload registered the version with its own file's size, and the catalog kept whichever of the jar, pom or classifier files had the smallest checksum, so a version with a 98 MB jar could show 29 KB. A Maven upload, through the Maven endpoint or the generic upload API, now sets the version's size to the sum of the live files in its version directory, which is just as order-independent across replication peers. The value is what the Packages list, package detail and Versions tab show; quotas and storage totals are computed from artifacts and are unaffected. Existing versions are corrected by the next upload of a file into that version, or all at once by running the admin catalog backfill (`POST /api/v1/admin/packages/backfill`), which now records the same total for Maven and Gradle versions.
