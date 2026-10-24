---
section: Fixed
issues: [#4921]
---
- **Protobuf commits now replicate to push peers** (#4921). A protobuf (BSR) commit row recorded the commit digest as its `checksum_sha256`, so the receiving peer, which verifies the pushed bundle against that value, refused every commit push with 409 "checksum mismatch". New commit rows record the sha256 of the stored bundle (the commit digest stays the row's version and is still what every lookup uses). Rows written before this release need no operator step: the sync worker recomputes the sha256 from the stored bundle before pushing such a row and stores the corrected value. A push that the peer refuses for that one artifact (a checksum mismatch, or an immutable path already occupied) now fails only that task and no longer puts the whole peer into retry backoff; connection errors and 5xx responses still back the peer off as before.
