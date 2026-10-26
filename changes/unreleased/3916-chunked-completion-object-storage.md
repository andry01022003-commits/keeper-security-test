---
section: Changed
issues: [#3916]
---
- **Chunked upload completion no longer needs a full-size local scratch file on object storage** (#3916). On S3-, GCS- and Azure-backed repositories, and for any format that does not need the body as a local file, the staged chunks now stream in order into one `upload-staging/<session-id>/assembled` object, are verified there, and are promoted to the content-addressed key with a server-side copy. Local `.uploads` assembly remains only for filesystem repositories, backends without native streaming, and RPM or Incus/Debian finalizes that parse the body from disk.
