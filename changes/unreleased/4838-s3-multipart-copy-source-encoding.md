---
section: Fixed
issues: [#4838]
---
- **Server-side S3 copy of a large object with a non-ASCII or reserved character in its path no longer fails with `NoSuchKey`** (#4838). object_store stores each key segment percent-encoded (`café` is stored as `caf%C3%A9`), and S3 URL-decodes the `x-amz-copy-source` header once before looking the source up. The multipart `UploadPartCopy` path, which every object above `S3_MAX_SINGLE_COPY_BYTES` takes, encoded the caller's key instead of the stored one, so the header decoded to a key that does not exist. The header is now the URL encoding of the stored key, the same value object_store's single-request `CopyObject` sends for a small object.
