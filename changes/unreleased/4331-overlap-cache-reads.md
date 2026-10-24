---
section: Changed
issues: [#4331]
---
- **A proxy cache hit on a filesystem cache reads the body alongside its metadata sidecar instead of after it** (#4331). A first-touch hit used to make three sequential storage round trips: read the sidecar, measure the body, open it. On network storage (NFS) each costs tens of milliseconds, which is most of the latency gap a Maven build sees against local disk. The body is now measured and opened at the same time as the sidecar is read (the streaming path), and the buffered path reads the body alongside the sidecar too; the size and checksum checks against the sidecar are unchanged, and a body opened before a concurrent re-publish of the entry is reopened. S3, GCS and Azure caches keep the sequential reads, since opening a body there starts transferring it. `docs/operations/proxy-cache-ha.md` answers the question in the issue about running remote repositories on several replicas.
