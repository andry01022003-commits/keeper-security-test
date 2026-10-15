---
section: Fixed
issues: [#4692]
---
- **A tarball already in a Remote npm repository's proxy cache is now served without reading the package's packument** (#4692). Every tarball GET read the whole raw upstream packument from storage and parsed it into a generic JSON tree just to pick `dist.integrity` and the tarball URL for one file, so a `yarn install` with an empty client cache read several times more packument bytes than it served and left the allocator holding hundreds of MB per run for documents like `@prisma/client` (68 MB) or `next` (31 MB). With scan-on-proxy or a download age gate configured the packument is still read on every GET, and there, on a cache miss and through a Virtual repository the tarball lookup now keeps only `versions.*.dist.{tarball,integrity,shasum}` (plus the name and version fields the GitHub Packages layout from #3785 needs) in memory, and a malformed value in any other field no longer disables the integrity check.
