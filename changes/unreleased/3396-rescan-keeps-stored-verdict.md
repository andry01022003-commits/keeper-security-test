---
section: Fixed
issues: [#3396]
---
- **A proxy rescan that cannot grade a package archive leaves the stored verdict unchanged** (#3396). `POST /api/v1/repositories/{key}/security/proxy-scans/rescan` has no request coordinate to pin, so for the objects the download gate grades only through a pin or a staged image (npm tarballs, PyPI sdists, gems, crates, NuGet packages and OCI image manifests) a rescan whose scanner cataloged no components is now inconclusive: it answers 503 `RESCAN_SCAN_FAILED` and keeps the stored verdict, instead of recording `clean`. For every format, a rescan that finds no vulnerabilities no longer replaces a stored `vulnerable` verdict when it cataloged fewer components than that verdict was graded on; that is inconclusive too.
