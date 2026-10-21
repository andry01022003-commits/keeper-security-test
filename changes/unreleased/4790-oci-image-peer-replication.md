---
section: Fixed
issues: [#4790]
---
- **OCI/Docker images pushed to hosted repositories are now replicated to push/mirror peers** (#4790). A `docker push` only queued the manifest artifact before, but image layers live in `oci_blobs` rather than `artifacts`, so peers received nothing (or had to sync manually). Pushed manifests now queue replication, and the sync worker transfers each image's blob closure (config and layers, and child manifests for a multi-arch index) to the peer before committing the manifest. The peer stores blobs and commits the manifest last, making replicated images indistinguishable from pushed ones; a replicated write is never re-fanned back out.
