---
section: Fixed
issues: [#4837]
---
- **A failed hosted Maven upload no longer leaves its bytes behind** (#4837). A hosted upload stores the file before the transaction that records it, so when that transaction failed (a metadata write error, a quota refusal, a failed POM read) the client got an error while the object stayed in storage. On object storage those orphaned bytes were then downloadable through the repository's storage fallback, serving a deploy the server had rejected. The upload now removes its object when the publish does not commit, on every storage backend, and the object-storage fallback no longer serves an artifact coordinate that has no catalog row (checksum sidecars and `maven-metadata.xml` are still served from it). The cleanup is serialised against an overlapping upload of the same coordinate, so a failed upload can never delete the bytes of a concurrent one that succeeds.
