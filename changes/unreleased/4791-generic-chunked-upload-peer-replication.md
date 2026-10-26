---
section: Fixed
issues: [#4791]
---
- **Chunked uploads through `/api/v1/uploads` now replicate to subscribed push peers** (#4791). The generic finalize path wrote its own `artifacts` row and never queued a push sync task, so a completed chunked upload only reached a peer after a manual sync; it now fans out like every direct and format-native publish, without re-queueing peer replication writes.
