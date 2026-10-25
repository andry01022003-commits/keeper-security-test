---
section: Changed
issues: [#4323]
---
- **Hosted OCI blob pulls verify the stored bytes against their digest** (Part of #4323). A whole-blob pull from a hosted repository, or from a Virtual's hosted member, whose stored object no longer hashes to its digest is refused with `502 STORED_OBJECT_INTEGRITY` and no body when the blob is at most 4 MiB, and cut short before its last chunk when it is larger, as the generic download route already does. Range pulls and blobs streamed from an upstream are served as before. `DOWNLOAD_VERIFY_CHECKSUMS=false` turns the check off.
