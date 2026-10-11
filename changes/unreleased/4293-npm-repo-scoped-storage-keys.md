---
section: Fixed
issues: [#4293]
---
- **The same npm package version can be published to more than one npm repository on S3, GCS or Azure storage** (#4293). npm tarballs were written to the flat key `npm/{package}/{version}/{file}`, which every repository on a shared object-storage namespace shares, so the second repository's publish was refused with `409 ... already owned by another repository; refusing cross-repository overwrite`. New tarballs on a cloud backend are written under the repository-scoped key `npm/{repository_id}/{package}/{version}/{file}` (the #2624 scheme Maven already uses), so each repository keeps its own copy. Existing tarballs stay where they are and keep serving from the key their artifact row records; filesystem storage and `STORAGE_KEY_SCHEME=flat` keep the flat key.
