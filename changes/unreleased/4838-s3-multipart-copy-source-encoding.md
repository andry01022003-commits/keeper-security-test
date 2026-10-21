---
section: Fixed
issues: [#4838]
---
- **Server-side S3 copy of a large object now names the source in the encoding it was stored under, so promotion and copy of a path with a non-ASCII or reserved character no longer fail with `NoSuchKey`** (#4838). object_store percent-encodes each key segment on write, leaving a space and `&` literal and encoding `~`; the multipart `UploadPartCopy` path, which every object above `S3_MAX_SINGLE_COPY_BYTES` takes, named its source with a different encoder (`urlencoding`) that does the opposite for those fourteen characters, so S3 could not resolve the copy source. The copy source is now built with the same segment encoding the write uses, matching the single-request `CopyObject` an object below the threshold already took.
