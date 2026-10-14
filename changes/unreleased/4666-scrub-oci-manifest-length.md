---
section: Fixed
issues: [#4666]
---
- **`storage-scrub` no longer flags every OCI manifest as corrupt** (#4666). Since #3601 an OCI manifest's `artifacts.size_bytes` holds the image size (config plus layers), not the length of the stored manifest body, and the scrub compared that against the object it read, so every manifest reported a size mismatch and real corruption was buried in the noise. The scrub now checks a manifest's length against `oci_manifests.size_bytes`, the recorded body length, and verifies a manifest with no such record yet by its SHA-256 alone; a finding's expected size is that body length. Blobs and every other artifact are checked as before, and a truncated or altered manifest is still reported corrupt.
