---
section: Fixed
issues: [#1683]
---
- **The `oci_manifests` reconcile no longer removes the rows of manifests that still pull by digest** (#1683). The stale predicate added earlier in this release removed the row of the leaf of an index of indexes, of the child of a migrated index kept only by checksum, and of the old digest after a tag moved while its blob refs remained, all of which a 1.11 replica still serves. It now keeps them, adds rows for such manifests that have none, and sweeps again in the same pass when a removed index leaves its children stale. The storage GC's blob-ref prune retires the row of a manifest whose refs it prunes, so a 1.11 and a 1.12 replica stop serving it together.
