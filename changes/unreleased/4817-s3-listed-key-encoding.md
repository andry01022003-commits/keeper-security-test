---
section: Fixed
issues: [#4817]
---
- **S3 storage reindex no longer reports artifacts with non-ASCII or reserved characters in their path as missing** (#4817). On S3 storage, object_store writes such a path under a percent-encoded object key (`uni-1.0-café.jar` is stored as `uni-1.0-caf%C3%A9.jar`), and listing returned that stored key unchanged, so `POST /api/v1/admin/repositories/{key}/reindex-storage` reported the row's object as missing and counted the object as an unregistered ghost that a non-dry run would register a second time under the encoded path. S3 listings, including the `S3_PROVIDER=oss` REST fallback, now return keys in the form reads and writes use, which also lets proxy-cache purges and RPM version retention delete such objects. Existing object keys are not rewritten. An object written by another tool under a key that Artifact Keeper can neither read nor write (for example a raw `~`) is left out of the listing with a warning.
