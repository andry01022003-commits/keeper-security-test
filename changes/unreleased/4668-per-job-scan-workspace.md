---
section: Fixed
issues: [#4668]
---
- **Concurrent scans of the same artifact no longer share one scan workspace** (#4668). The Trivy and Grype filesystem scanners, OpenSCAP and Grype's OCI layout all extracted into a directory named only by the artifact id, so two scans of one artifact running at once (a re-scan during a scan, a repository rescan, a scan flood) deleted or overwrote each other's tree and one of them failed. Each scan job now gets its own directory (`<artifact_id>-<job>`), which its guard still removes on completion, failure and cancellation (#3565).
