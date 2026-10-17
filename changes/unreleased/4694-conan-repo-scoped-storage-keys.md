---
section: Fixed
issues: [#4694]
---
- **The same Conan recipe or package revision can be uploaded to more than one Conan repository on S3, GCS or Azure storage** (#4694). Conan files were written to the flat key `conan/{name}/{version}/{user}/{channel}/...`, which every repository on a shared object-storage namespace shares, so uploading a revision another Conan repository already held was refused with `409 ... already owned by another repository; refusing cross-repository overwrite`, on every retry. New recipe and package files on a cloud backend are written under the repository-scoped key `conan/{repository_id}/...` (the #2624 scheme npm and Maven already use), so each repository keeps its own copy. Existing files stay where they are and keep serving from the key their artifact row records; filesystem storage and `STORAGE_KEY_SCHEME=flat` keep the flat key.
